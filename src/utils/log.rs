use leptos::logging;

pub fn log_with_file_path(file: &str, module: &str, line: u32, msg: &str) {
    logging::log!(
        "File: {}, Module: {}, Line: {}, Message: {}",
        file,
        module,
        line,
        msg,
    );
}

pub fn simple_log(msg: &str) {
    logging::log!("{}", msg);
}
