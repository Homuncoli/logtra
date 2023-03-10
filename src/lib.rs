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
    ($intensity: tt, $color: tt, $($arg:tt)*) => {
        crate::msg::LogMessage { 
            line: line!(),
            file: file!(),
            time: chrono::Utc::now().into(),
            scope: module_path!(),
            msg: &format_args!($($arg)*).to_string(),
            intensity: crate::msg::LogIntensity::$intensity,
            color: crate::msg::Color::$color,
        };
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
pub fn log<T: std::fmt::Debug>(intensity: crate::msg::LogIntensity, name: &str, obj: &T) {
    match intensity {
        msg::LogIntensity::Trace => trace!("{}: {:?}", name, obj),
        msg::LogIntensity::Debug => debug!("{}: {:?}", name, obj),
        msg::LogIntensity::Info => info!("{}: {:?}", name, obj),
        msg::LogIntensity::Warn => warn!("{}: {:?}", name, obj),
        msg::LogIntensity::Error => error!("{}: {:?}", name, obj),
        msg::LogIntensity::Fatal => fatal!("{}: {:?}", name, obj),
    }
}
macro_rules! log {
    ($obj: expr) => {
        log!(Debug, $obj)
    };
    ($intensity: tt, $obj: expr) => {
        crate::log(crate::msg::LogIntensity::$intensity, stringify!($obj), $obj)
    };
}
macro_rules! fatal_assert {
    ($val: expr) => {
        match $val {
            true => log!(Info, $val),
            false => log!(Fatal, $val)
        }
    };
}
macro_rules! error_assert {
    ($val: expr) => {
        match $val {
            true => log!(Info, $val),
            false => log!(Error, $val)
        }
    };
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use crate::{msg::{LogIntensity, LogMessage}, sink::{ConsoleSink, SinkDeclaration}};

    #[test]
    fn log_macros() {
        let sink = ConsoleSink::new(
            SinkDeclaration {
            name: "console".to_string(),
            intensity: LogIntensity::Trace,
            scope: "*".to_string(),
            template: "[%t][%c][%[%i%]][%s][%f:%l]: %m\n".to_string(),
            }
        );
        sink!(sink);

        let now = Utc::now().into();
        let line = line!();

        let expected = LogMessage {
            time: now,
            scope: module_path!(),
            file: file!(),
            line: line,
            msg: "Hello World!",
            intensity: LogIntensity::Info,
            color: crate::msg::Color::Red,
        };

        trace!("Hello World: Trace!");
        debug!("Hello World: Debug!");
        info!("Hello World: Info!");
        warn!("Hello World: Warn!");
        error!("Hello World: Error!");
        fatal!("Hello World: Fatal!");
        log!(Info, &expected);
        error_assert!(&(expected != expected));
        error_assert!(&(expected == expected));
        fatal_assert!(&(expected != expected));
        fatal_assert!(&(expected == expected));
    }
}

#[cfg(test)]
mod performance {
    use std::time::SystemTime;

    use chrono::Utc;

    use crate::{sink::{VoidSink, SinkDeclaration}, msg::LogIntensity};

    #[test]
    fn log_performance() {
        let sink = VoidSink::new(
            SinkDeclaration {
                name: "void".to_string(),
                intensity: LogIntensity::Trace,
                scope: "*".to_string(),
                template: "[%t][%[%i%]][%s][%f:%l]: %m\n".to_string(),
            }
        );
        sink!(sink);

        for i in 0..10 {
            let start: SystemTime = Utc::now().into();
            let mut counter = 0;
            while start.elapsed().unwrap().as_millis() < 1000 {
                info!("Hello World: Info!");
                counter += 1;
            }

            println!("Processed {} infos in {}ms", counter, start.elapsed().unwrap().as_millis());
            assert!(counter > 0);
        }
    }
}