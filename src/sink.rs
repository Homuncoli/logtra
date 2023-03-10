use std::{fs::File, io::{self, Write}};

use contra::{Deserialize, Serialize};

use crate::msg::{LogIntensity, LogMessage};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct SinkDeclaration {
    pub(crate) name: String,
    pub(crate) intensity: LogIntensity,
    pub(crate) scope: String,
    pub(crate) template: String,
}

pub trait Sink: Sync + 'static {
    fn log(&mut self, msg: &LogMessage);
    fn log_filtered(&mut self, msg: &LogMessage) {
        if self.intensity() > msg.intensity {
            return;
        }
        if msg.scope.contains(self.scope()) {
            return;
        }

        self.log(msg);
    }

    fn intensity(&self) -> LogIntensity; 
    fn scope(&self) -> &str; 
}

pub struct ConsoleSink { 
    decl: SinkDeclaration
}

impl ConsoleSink {
    pub fn new(decl: SinkDeclaration) -> Self {
        ConsoleSink {  
            decl
        }
    }
}

impl Sink for ConsoleSink {
    fn log(&mut self, msg: &LogMessage) {
        print!("{}", msg.parse(&self.decl.template));
    }

    fn intensity(&self) -> LogIntensity {
        self.decl.intensity
    }

    fn scope(&self) -> &str {
        &self.decl.scope
    }
}


const FILE_SINK_BUFFER_SIZE: usize = 100;
pub struct FileSink {
    decl: SinkDeclaration,
    buffer: [String; FILE_SINK_BUFFER_SIZE],
    index: usize,
}

impl FileSink {
    fn new(decl: SinkDeclaration) -> Self {
        const empty: String = String::new();
        FileSink { 
            decl,
            buffer: [empty; FILE_SINK_BUFFER_SIZE],
            index: 0,
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut file = File::create(&self.decl.name)?;
        for i in 0..self.index {
            file.write_all((self.buffer.get(i).unwrap()).as_bytes())?;
        }

        const empty: String = String::new();
        self.buffer = [empty; FILE_SINK_BUFFER_SIZE];
        Ok(())
    }
}

impl Sink for FileSink {
    fn log(&mut self, msg: &LogMessage) {
        self.buffer[self.index] = msg.parse(&self.decl.template);
        if self.index + 1 >= FILE_SINK_BUFFER_SIZE {
            if let Err(err) = self.flush() {
                panic!("{}", err);
            }
        }
        self.index = self.index + 1 % FILE_SINK_BUFFER_SIZE;
    }

    fn intensity(&self) -> LogIntensity {
        self.decl.intensity
    }

    fn scope(&self) -> &str {
        &self.decl.scope
    }
}

impl Drop for FileSink {
    fn drop(&mut self) {
        if let Err(err) = self.flush() {
            panic!("{}", err);
        }
    }
}

pub struct VoidSink {
    decl: SinkDeclaration
}

impl VoidSink {
    pub fn new(decl: SinkDeclaration) -> Self {
        Self {
            decl
        }
    }
}

impl Sink for VoidSink {
    fn log(&mut self, _msg: &LogMessage) {
        // do nothing
    }

    fn intensity(&self) -> LogIntensity {
        self.decl.intensity
    }

    fn scope(&self) -> &str {
        &self.decl.scope
    }
}

#[cfg(test)]
mod test {
    use std::{fs::{remove_file}, path::Path};

    use chrono::{DateTime, Utc};

    use crate::{sink::{SinkDeclaration, ConsoleSink, Sink}, msg::{LogIntensity, LogMessage, Color}};

    use super::FileSink;

    #[test]
    fn console_sink_works() {
        let decl = SinkDeclaration { 
            name: "Default".to_string(), 
            intensity: LogIntensity::Info, 
            scope: "*".to_string(), 
            template: "[%t][%c%s%c][%f:%l]: %m\n".to_string() 
        };
        let msg = LogMessage {
            time: DateTime::<Utc>::default().into(),
            scope: "logtra",
            file: file!(),
            line: line!(),
            msg: "Hello world!",
            intensity: LogIntensity::Info,
            color: Color::Red,
        };

        let mut sink = ConsoleSink::new(decl);
        sink.log(&msg);
    }

    #[test]
    fn file_sink_works() {
        let decl = SinkDeclaration { 
            name: "example.log".to_string(), 
            intensity: LogIntensity::Info, 
            scope: "*".to_string(), 
            template: "[%t][%s][%f:%l]: %m\n".to_string() 
        };
        let msg = LogMessage {
            time: DateTime::<Utc>::default().into(),
            scope: "logtra",
            file: file!(),
            line: line!(),
            msg: "Hello world!",
            intensity: LogIntensity::Info,
            color: Color::Red,
        };

        {
            let mut sink = FileSink::new(decl);
            sink.log(&msg);
            sink.log(&msg);
            sink.log(&msg);
            sink.log(&msg);
        }

        assert!(remove_file(Path::new("example.log")).is_ok());
    }
}