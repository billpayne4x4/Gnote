use std::fmt::Arguments;

#[macro_export]
macro_rules! log_test {
    ($($arg:tt)*) => {
        #[cfg(test)]
        {
            $crate::tools::logging::log("Test", format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::tools::logging::log("Info", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::tools::logging::log("Warning", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::tools::logging::log("Error", format_args!($($arg)*));
    };
}

#[allow(dead_code)]
pub fn log(log_level: &str, args: Arguments<'_>) {
    println!("{}: {}", log_level, args);
}
