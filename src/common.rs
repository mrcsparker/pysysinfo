macro_rules! common_methods {
    ($struct:ident) => {
        #[pymethods]
        impl $struct {
            fn __repr__(&self) -> String {
                format!("{self:?}")
            }

            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap_or_default()
            }
        }
    };
}
