
#[macro_export]
macro_rules! green {
    ($($arg:tt)*) => (format!("\x1b[32m{}\x1b[0m", format_args!($($arg)*)))
}

#[macro_export]
macro_rules! red {
    ($($arg:tt)*) => (format!("\x1b[31m{}\x1b[0m", format_args!($($arg)*)))
}
