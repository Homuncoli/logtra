use sink::Sink;

pub mod msg;
pub mod sink;

static mut SINKS: Vec<Box<dyn Sink>> = Vec::new();

macro_rules! sink {
    ($sink: tt) => {{
        unsafe {
            crate::SINKS.push(Box::new($sink));
        }
    }};
}

macro_rules! msg {
    ($intensity: tt, $color: tt, $time: expr, $line: expr, $($arg:tt)*) => {
        crate::msg::LogMessage { 
            line: $line,
            file: file!(),
            time: $time,
            scope: module_path!(),
            msg: &format_args!($($arg)*).to_string(),
            intensity: crate::msg::LogIntensity::$intensity,
            color: crate::msg::Color::$color,
        };
    };
}
macro_rules! msg_now {
    ($intensity: tt, $color: ident, $($arg:tt)*) => {
        msg!($intensity, $color, chrono::Utc::now().into(), line!(), $($arg)*)
    };
}
macro_rules! log {
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
        let msg = msg_now!(Trace, Grey, $($arg)*);
        log!(&msg);
    }};
}
macro_rules! debug {
    ($($arg:tt)*) => {{
        let msg = msg_now!(Debug, Blue, $($arg)*);
        log!(&msg);
    }};
}
macro_rules! info {
    ($($arg:tt)*) => {{
        let msg = msg_now!(Info, Default, $($arg)*);
        log!(&msg);
    }};
}
macro_rules! warn {
    ($($arg:tt)*) => {{
        let msg = msg_now!(Warn, Orange, $($arg)*);
        log!(&msg);
    }};
}
macro_rules! error {
    ($($arg:tt)*) => {{
        let msg = msg_now!(Error, Red, $($arg)*);
        log!(&msg);
    }};
}
macro_rules! fatal {
    ($($arg:tt)*) => {{
        let msg = msg_now!(Fatal, DarkRed, $($arg)*);
        log!(&msg);
    }};
}


#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use chrono::Utc;

    use crate::{msg::{LogIntensity, LogMessage}, sink::{ConsoleSink, SinkDeclaration, VoidSink}};

    #[test]
    fn msg_macro() {
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

        let result = msg!(Info, Red, now, line, "Hello World!");
        
        assert_eq!(expected, result);
    }

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