use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{self, BufRead, BufReader, Read, Seek},
    path::Path,
    sync::{Arc, Mutex},
};

use super::writer::ErrorWriter;

pub struct ErrorContext<R: Read + Seek> {
    pub(crate) source: BufReader<R>,
}

pub trait ErrorProvider: Debug {
    fn write_errors<R: Read + Seek>(&self, writer: &mut ErrorWriter<R>) -> std::fmt::Result;
}

pub struct PackagedError<R: Read + Seek, E: ErrorProvider> {
    context: Arc<Mutex<ErrorContext<R>>>,
    provider: E,
}

impl ErrorContext<File> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Arc<Mutex<Self>>> {
        Ok(Arc::new(Mutex::new(Self {
            source: BufReader::new(File::open(path)?),
        })))
    }
}

impl<R: Read + Seek> ErrorContext<R> {
    pub fn line(&mut self, line: usize) -> io::Result<String> {
        self.source.rewind()?;
        let mut buffer = String::new();

        for _ in 0..=line {
            buffer.clear();
            self.source.read_line(&mut buffer)?;
        }

        Ok(buffer.trim_end().to_owned())
    }
}

impl<R: Read + Seek, E: ErrorProvider> PackagedError<R, E> {
    pub fn new(context: Arc<Mutex<ErrorContext<R>>>, provider: E) -> Self {
        Self { context, provider }
    }
}

impl<R: Read + Seek, E: ErrorProvider> std::error::Error for PackagedError<R, E> {}

impl<R: Read + Seek, E: ErrorProvider> Display for PackagedError<R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut context = self
            .context
            .lock()
            .map_err(|_| std::fmt::Error::default())?;
        let mut writer = ErrorWriter::new(&mut context, f);
        self.provider.write_errors(&mut writer)
    }
}

impl<R: Read + Seek, E: ErrorProvider> Debug for PackagedError<R, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackagedError")
            .field("provider", &self.provider)
            .finish()
    }
}
