use std::fmt::Display;

use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyTuple};
use serde::Serialize;
use serde_json::{Map, Number, Value};

/// Convert a serialization failure into a Python runtime error with context.
fn serialization_error(context: &str, err: impl Display) -> PyErr {
    PyRuntimeError::new_err(format!("{context}: {err}"))
}

/// Serialize a Rust value into a JSON string for Python callers.
pub(crate) fn serialize_to_json<T: Serialize>(value: &T) -> PyResult<String> {
    serde_json::to_string(value)
        .map_err(|err| serialization_error("failed to serialize value to JSON", err))
}

/// Serialize a Rust value into a Python `dict`.
pub(crate) fn serialize_to_py_dict<'py, T: Serialize>(
    py: Python<'py>,
    value: &T,
) -> PyResult<Bound<'py, PyDict>> {
    let serialized = serde_json::to_value(value).map_err(|err| {
        serialization_error("failed to serialize value to a Python dictionary", err)
    })?;
    let Value::Object(entries) = serialized else {
        return Err(PyRuntimeError::new_err(
            "only JSON objects can be converted to Python dictionaries",
        ));
    };
    json_object_to_py_dict(py, entries)
}

/// Build a readable Python-style `repr` from a serializable value.
pub(crate) fn repr_from_serialize<T: Serialize>(class_name: &str, value: &T) -> PyResult<String> {
    let serialized = serde_json::to_value(value)
        .map_err(|err| serialization_error("failed to build __repr__", err))?;
    match serialized {
        Value::Object(fields) => {
            let rendered = fields
                .into_iter()
                .map(|(key, value)| format!("{key}={}", repr_value(&value)))
                .collect::<Vec<_>>()
                .join(", ");
            Ok(format!("{class_name}({rendered})"))
        }
        other => Ok(format!("{class_name}({})", repr_value(&other))),
    }
}

/// Convert a Rust collection into a Python tuple.
pub(crate) fn tuple_from_vec<'py, T>(
    py: Python<'py>,
    values: Vec<T>,
) -> PyResult<Bound<'py, PyTuple>>
where
    T: IntoPyObject<'py>,
{
    PyTuple::new(py, values)
}

/// Convert a Rust collection into a Python tuple by first mapping each item into a Python object.
pub(crate) fn tuple_from_mapped_vec<'py, T, F>(
    py: Python<'py>,
    values: Vec<T>,
    mut convert: F,
) -> PyResult<Bound<'py, PyTuple>>
where
    F: FnMut(Python<'py>, T) -> PyResult<Py<PyAny>>,
{
    let objects = values
        .into_iter()
        .map(|value| convert(py, value))
        .collect::<PyResult<Vec<_>>>()?;
    PyTuple::new(py, objects)
}

fn json_object_to_py_dict<'py>(
    py: Python<'py>,
    entries: Map<String, Value>,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (key, value) in entries {
        dict.set_item(key, json_value_to_py(py, value)?)?;
    }
    Ok(dict)
}

fn json_value_to_py(py: Python<'_>, value: Value) -> PyResult<Py<PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(value) => value.into_py_any(py),
        Value::Number(number) => json_number_to_py(py, number),
        Value::String(value) => value.into_py_any(py),
        Value::Array(values) => {
            let list = PyList::empty(py);
            for value in values {
                list.append(json_value_to_py(py, value)?)?;
            }
            Ok(list.into_any().unbind())
        }
        Value::Object(entries) => Ok(json_object_to_py_dict(py, entries)?.into_any().unbind()),
    }
}

fn json_number_to_py(py: Python<'_>, value: Number) -> PyResult<Py<PyAny>> {
    if let Some(value) = value.as_u64() {
        value.into_py_any(py)
    } else if let Some(value) = value.as_i64() {
        value.into_py_any(py)
    } else if let Some(value) = value.as_f64() {
        value.into_py_any(py)
    } else {
        Err(PyRuntimeError::new_err(
            "encountered a JSON number that could not be represented in Python",
        ))
    }
}

fn repr_value(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(value) => {
            if *value {
                "True".to_string()
            } else {
                "False".to_string()
            }
        }
        Value::Number(value) => value.to_string(),
        Value::String(value) => format!("{value:?}"),
        Value::Array(values) => {
            let values = values.iter().map(repr_value).collect::<Vec<_>>().join(", ");
            format!("[{values}]")
        }
        Value::Object(values) => {
            let values = values
                .iter()
                .map(|(key, value)| format!("{key:?}: {}", repr_value(value)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{values}}}")
        }
    }
}
