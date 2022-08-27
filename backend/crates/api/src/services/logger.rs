use crate::services::utils_service::UtilsService;
use std::env;
use std::str::FromStr;

pub(crate) fn setup_logger_panic_on_fail() {
    let log_filter = env::var("LOG_FILTER_LEVEL").expect("No log filter level in environment!");
    let filter = log::LevelFilter::from_str(&log_filter)
        .expect("Valid log level filter expected, valid values are: [\"OFF\", \"ERROR\", \"WARN\", \"INFO\", \"DEBUG\", \"TRACE\"]");

    // let log_file_name = format!("./logs/log-{}.log", UtilsService::naive_now().format("%Y-%m-%d-%H-%M-%S"));

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(filter)
        .chain(std::io::stdout())
        // .chain(fern::log_file(log_file_name).expect("Setting up log file failed!"))
        .apply()
        .expect("Setting up logging failed!");
}
