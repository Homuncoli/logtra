use sink::Sink;

pub mod msg;
pub mod sink;

static mut SINKS: Vec<Box<dyn Sink>> = Vec::new();

/// Registers a new [crate::sink::Sink]
macro_rules! sink {
    ($sink: tt) => {{
        unsafe {
            crate::SINKS.push(Box::new($sink));
        }
    }};
}

/// Creates a new [crate::msg::LogMessage]
macro_rules! msg {
    ($severity: tt, $color: tt, $($arg:tt)*) => {
        crate::msg::LogMessage {
            line: line!(),
            file: file!(),
            time: chrono::Utc::now().into(),
            module: module_path!(),
            msg: &format_args!($($arg)*).to_string(),
            severity: crate::msg::LogSeverity::$severity,
            color: crate::msg::Color::$color,
        }
    };
}
/// Takes a [crate::msg::LogMessage] and tries to log it on every registered [crate::sink::Sink]
macro_rules! publish {
    ($msg: expr) => {
        unsafe {
            for i in 0..crate::SINKS.len() {
                crate::SINKS.get_mut(i).unwrap().log_filtered($msg);
            }
        }
    };
}

macro_rules! trace {
    ($($arg:tt)*) => {{
        let msg = msg!(Trace, Grey, $($arg)*);
        publish!(&msg);
    }};
}
macro_rules! debug {
    ($($arg:tt)*) => {{
        let msg = msg!(Debug, Blue, $($arg)*);
        publish!(&msg);
    }};
}
macro_rules! info {
    ($($arg:tt)*) => {{
        let msg = msg!(Info, Default, $($arg)*);
        publish!(&msg);
    }};
}
macro_rules! warn {
    ($($arg:tt)*) => {{
        let msg = msg!(Warn, Orange, $($arg)*);
        publish!(&msg);
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = msg!(Error, Red, $($arg)*);
        publish!(&msg);
    }};
}
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let msg = msg!(Fatal, DarkRed, $($arg)*);
        publish!(&msg);
    }};
}
#[doc(hidden)]
/// Use log! instead
pub fn log<T: std::fmt::Debug + ?Sized>(severity: crate::msg::LogSeverity, name: &str, obj: &T) {
    match severity {
        msg::LogSeverity::Trace => trace!("{}: {:?}", name, obj),
        msg::LogSeverity::Debug => debug!("{}: {:?}", name, obj),
        msg::LogSeverity::Info => info!("{}: {:?}", name, obj),
        msg::LogSeverity::Warn => warn!("{}: {:?}", name, obj),
        msg::LogSeverity::Error => error!("{}: {:?}", name, obj),
        msg::LogSeverity::Fatal => fatal!("{}: {:?}", name, obj),
    }
}
macro_rules! log {
    ($obj: expr) => {
        log!(Debug, $obj)
    };
    ($severity: tt, $obj: expr) => {
        crate::log(crate::msg::LogSeverity::$severity, stringify!($obj), $obj)
    };
}
macro_rules! fatal_assert {
    ($val: expr) => {
        match $val {
            true => log!(Info, $val),
            false => log!(Fatal, $val),
        }
    };
}
macro_rules! error_assert {
    ($val: expr) => {
        match $val {
            true => log!(Info, $val),
            false => log!(Error, $val),
        }
    };
}

#[cfg(test)]
mod test {
    use std::{time::SystemTime};

    use chrono::Utc;

    use crate::{
        msg::{LogSeverity},
        sink::{ConsoleSink, SinkDeclaration},
    };

    #[test]
    fn log_macros() {
        let sink = ConsoleSink::new(SinkDeclaration {
            name: "console".to_string(),
            severity: LogSeverity::Trace,
            module: "".to_string(),
            template: "[%t][%c][%[%i%]][%s][%f:%l]: %m\n".to_string(),
        });
        sink!(sink);

        let now: SystemTime = Utc::now().into();

        trace!("Hello World: Trace!");
        debug!("Hello World: Debug!");
        info!("Hello World: Info!");
        warn!("Hello World: Warn!");
        error!("Hello World: Error!");
        fatal!("Hello World: Fatal!");
        log!(Info, &now);
        log!(Info, &Some(now));
        log!(Info, &None as &Option<String>);
        log!(Info, &Ok::<&str, &str>("error"));
        log!(Error, &Err::<&str, &str>("error"));
        error_assert!(&(now != now));
        error_assert!(&(now == now));
        fatal_assert!(&(now != now));
        fatal_assert!(&(now == now));
    }
}

#[cfg(test)]
mod performance {
    use std::time::SystemTime;

    use chrono::Utc;

    use crate::{
        msg::LogSeverity,
        sink::{SinkDeclaration, VoidSink},
    };

    #[test]
    fn log_performance() {
        let sink = VoidSink::new(SinkDeclaration {
            name: "void".to_string(),
            severity: LogSeverity::Trace,
            module: "".to_string(),
            template: "[%t][%[%i%]][%s][%f:%l]: %m\n".to_string(),
        });
        sink!(sink);

        for _ in 0..10 {
            let start: SystemTime = Utc::now().into();
            let mut counter = 0;
            while start.elapsed().unwrap().as_millis() < 1000 {
                info!("Hello World: Info!");
                counter += 1;
            }

            println!(
                "Processed {} infos in {}ms",
                counter,
                start.elapsed().unwrap().as_millis()
            );
            assert!(counter > 100000);
        }
    }
}
