use sentry;
use std::error::Error;

pub fn report_error(message: &str) {
    sentry::capture_message(message, sentry::Level::Error);
}

pub fn report_error_with_cause<E: Error + ?Sized>(message: &str, error: &E) {
    sentry::capture_error(error);
    sentry::capture_message(&format!("{message}: {error}"), sentry::Level::Error);
}

pub fn format_err<T: std::fmt::Debug>(context: &str, err: T) -> String {
    format!("{context}: {err:?}")
}
