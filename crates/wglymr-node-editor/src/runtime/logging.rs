pub trait RuntimeLogger {
    fn log(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn debug(&self, message: &str);
}

/// Global logger instance.
///
/// SAFETY: This uses mutable static which is normally unsafe.
/// However, this is safe in WASM context because:
/// - WASM is single-threaded (no data races possible)
/// - set_logger is called exactly once during init_engine()
/// - All subsequent access is read-only
/// - The logger has 'static lifetime
static mut LOGGER: Option<&'static dyn RuntimeLogger> = None;

/// Set the global logger for the runtime.
///
/// SAFETY: This must be called exactly once during initialization,
/// before any other runtime operations. The logger must have a 'static
/// lifetime and remain valid for the entire program execution.
/// Typically called from wasm::init_engine().
pub fn set_logger(logger: &'static dyn RuntimeLogger) {
    unsafe {
        LOGGER = Some(logger);
    }
}

pub fn log(message: &str) {
    unsafe {
        if let Some(logger) = LOGGER {
            logger.log(message);
        }
    }
}

pub fn error(message: &str) {
    unsafe {
        if let Some(logger) = LOGGER {
            logger.error(message);
        }
    }
}

pub fn warn(message: &str) {
    unsafe {
        if let Some(logger) = LOGGER {
            logger.warn(message);
        }
    }
}

pub fn debug(message: &str) {
    unsafe {
        if let Some(logger) = LOGGER {
            logger.debug(message);
        }
    }
}
