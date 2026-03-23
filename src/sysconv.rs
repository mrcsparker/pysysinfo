//! Small conversion helpers that keep the binding code readable.

use std::ffi::{OsStr, OsString};
use std::path::Path;

use pyo3::PyResult;
use pyo3::exceptions::PyValueError;

const ALL_SIGNALS: [sysinfo::Signal; 32] = [
    sysinfo::Signal::Hangup,
    sysinfo::Signal::Interrupt,
    sysinfo::Signal::Quit,
    sysinfo::Signal::Illegal,
    sysinfo::Signal::Trap,
    sysinfo::Signal::Abort,
    sysinfo::Signal::IOT,
    sysinfo::Signal::Bus,
    sysinfo::Signal::FloatingPointException,
    sysinfo::Signal::Kill,
    sysinfo::Signal::User1,
    sysinfo::Signal::Segv,
    sysinfo::Signal::User2,
    sysinfo::Signal::Pipe,
    sysinfo::Signal::Alarm,
    sysinfo::Signal::Term,
    sysinfo::Signal::Child,
    sysinfo::Signal::Continue,
    sysinfo::Signal::Stop,
    sysinfo::Signal::TSTP,
    sysinfo::Signal::TTIN,
    sysinfo::Signal::TTOU,
    sysinfo::Signal::Urgent,
    sysinfo::Signal::XCPU,
    sysinfo::Signal::XFSZ,
    sysinfo::Signal::VirtualAlarm,
    sysinfo::Signal::Profiling,
    sysinfo::Signal::Winch,
    sysinfo::Signal::IO,
    sysinfo::Signal::Poll,
    sysinfo::Signal::Power,
    sysinfo::Signal::Sys,
];

/// Lossily convert an `OsStr` into a Python-friendly `String`.
pub(crate) fn os_str_to_string(value: &OsStr) -> String {
    value.to_string_lossy().into_owned()
}

/// Convert an `OsString` slice into owned Python-friendly strings.
pub(crate) fn os_string_slice_to_strings(values: &[OsString]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect()
}

/// Lossily convert a filesystem path into a Python-friendly `String`.
pub(crate) fn path_to_string(value: &Path) -> String {
    value.to_string_lossy().into_owned()
}

/// Convert an optional path into an optional string.
pub(crate) fn optional_path_to_string(value: Option<&Path>) -> Option<String> {
    value.map(path_to_string)
}

/// Normalize `DiskKind` into the stable display strings exposed by the Python API.
pub(crate) fn disk_kind_to_string(kind: sysinfo::DiskKind) -> String {
    match kind {
        sysinfo::DiskKind::HDD => "HDD".to_string(),
        sysinfo::DiskKind::SSD => "SSD".to_string(),
        sysinfo::DiskKind::Unknown(value) => format!("Unknown({value})"),
    }
}

/// Normalize `ProcessStatus` into snake_case strings for Python consumers.
pub(crate) fn process_status_to_string(status: sysinfo::ProcessStatus) -> String {
    match status {
        sysinfo::ProcessStatus::Idle => "idle".to_string(),
        sysinfo::ProcessStatus::Run => "run".to_string(),
        sysinfo::ProcessStatus::Sleep => "sleep".to_string(),
        sysinfo::ProcessStatus::Stop => "stop".to_string(),
        sysinfo::ProcessStatus::Zombie => "zombie".to_string(),
        sysinfo::ProcessStatus::Tracing => "tracing".to_string(),
        sysinfo::ProcessStatus::Dead => "dead".to_string(),
        sysinfo::ProcessStatus::Wakekill => "wakekill".to_string(),
        sysinfo::ProcessStatus::Waking => "waking".to_string(),
        sysinfo::ProcessStatus::Parked => "parked".to_string(),
        sysinfo::ProcessStatus::LockBlocked => "lock_blocked".to_string(),
        sysinfo::ProcessStatus::UninterruptibleDiskSleep => {
            "uninterruptible_disk_sleep".to_string()
        }
        sysinfo::ProcessStatus::Suspended => "suspended".to_string(),
        sysinfo::ProcessStatus::Unknown(value) => format!("unknown({value})"),
    }
}

/// Normalize `ThreadKind` into snake_case strings for Python consumers.
pub(crate) fn thread_kind_to_string(kind: sysinfo::ThreadKind) -> String {
    match kind {
        sysinfo::ThreadKind::Kernel => "kernel".to_string(),
        sysinfo::ThreadKind::Userland => "userland".to_string(),
    }
}

/// Parse the Python-facing update-kind strings used by the refresh APIs.
pub(crate) fn parse_update_kind(value: &str) -> PyResult<sysinfo::UpdateKind> {
    match normalize_identifier(value).as_str() {
        "never" => Ok(sysinfo::UpdateKind::Never),
        "always" => Ok(sysinfo::UpdateKind::Always),
        "only_if_not_set" => Ok(sysinfo::UpdateKind::OnlyIfNotSet),
        other => Err(PyValueError::new_err(format!(
            "invalid update kind {other:?}; expected one of: never, always, only_if_not_set"
        ))),
    }
}

/// Convert a `Signal` into the lowercase identifiers used by the Python API.
pub(crate) fn signal_to_string(signal: sysinfo::Signal) -> &'static str {
    match signal {
        sysinfo::Signal::Hangup => "hangup",
        sysinfo::Signal::Interrupt => "interrupt",
        sysinfo::Signal::Quit => "quit",
        sysinfo::Signal::Illegal => "illegal",
        sysinfo::Signal::Trap => "trap",
        sysinfo::Signal::Abort => "abort",
        sysinfo::Signal::IOT => "iot",
        sysinfo::Signal::Bus => "bus",
        sysinfo::Signal::FloatingPointException => "floating_point_exception",
        sysinfo::Signal::Kill => "kill",
        sysinfo::Signal::User1 => "user1",
        sysinfo::Signal::Segv => "segv",
        sysinfo::Signal::User2 => "user2",
        sysinfo::Signal::Pipe => "pipe",
        sysinfo::Signal::Alarm => "alarm",
        sysinfo::Signal::Term => "term",
        sysinfo::Signal::Child => "child",
        sysinfo::Signal::Continue => "continue",
        sysinfo::Signal::Stop => "stop",
        sysinfo::Signal::TSTP => "tstp",
        sysinfo::Signal::TTIN => "ttin",
        sysinfo::Signal::TTOU => "ttou",
        sysinfo::Signal::Urgent => "urgent",
        sysinfo::Signal::XCPU => "xcpu",
        sysinfo::Signal::XFSZ => "xfsz",
        sysinfo::Signal::VirtualAlarm => "virtual_alarm",
        sysinfo::Signal::Profiling => "profiling",
        sysinfo::Signal::Winch => "winch",
        sysinfo::Signal::IO => "io",
        sysinfo::Signal::Poll => "poll",
        sysinfo::Signal::Power => "power",
        sysinfo::Signal::Sys => "sys",
    }
}

/// Parse a Python-facing signal name into a concrete `sysinfo::Signal`.
pub(crate) fn parse_signal(value: &str) -> PyResult<sysinfo::Signal> {
    let normalized = normalize_identifier(value);
    let normalized = normalized
        .strip_prefix("sig")
        .map_or(normalized.as_str(), |value| value);

    ALL_SIGNALS
        .into_iter()
        .find(|signal| signal_to_string(*signal) == normalized)
        .ok_or_else(|| {
            PyValueError::new_err(format!(
                "invalid signal {value:?}; expected one of: {}",
                supported_signals().join(", ")
            ))
        })
}

/// Return the supported platform signals as lowercase Python-facing identifiers.
pub(crate) fn supported_signals() -> Vec<String> {
    sysinfo::SUPPORTED_SIGNALS
        .iter()
        .copied()
        .map(signal_to_string)
        .map(str::to_string)
        .collect()
}

/// Normalize a flexible identifier string into the canonical snake_case form.
fn normalize_identifier(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|character| match character {
            '-' | ' ' => '_',
            _ => character.to_ascii_lowercase(),
        })
        .collect()
}
