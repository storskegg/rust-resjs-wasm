/// Log a message using the active logger (native or WASM).
pub fn log_message(message: &str) {
    #[cfg(feature = "native-logger")]
    {
        // Use `log` crate for native logging
        log::info!("Native logger: {}", message);
    }

    #[cfg(feature = "wasm-logger")]
    {
        // Use `web-sys` to log to browser console
        web_sys::console::log_1(&format!("WASM logger: {}", message).into());
    }
}
