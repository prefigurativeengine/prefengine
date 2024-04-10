use simple_logger;
use time::macros::format_description;

pub fn init_styled_logger() {
    simple_logger::SimpleLogger::new()
    .env()
    .with_colors(true)
    .with_timestamp_format(format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"))
    .init()
    .unwrap();
}

