pub fn report_error(message: &str) {
    sentry::capture_message(message, sentry::Level::Error);
}

pub fn format_err<T: std::fmt::Debug>(context: &str, err: T) -> String {
    format!("{}: {:?}", context, err)
}
