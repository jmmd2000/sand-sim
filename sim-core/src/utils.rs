use wasm_bindgen::JsValue;

/// Console logging helper
pub fn log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s));
}

/// println!-style macro that uses the log function
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        $crate::utils::log(&format!($($t)*))
    }
}
