use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use catastrophic_ast::span::{Location, Span};

use super::error::Error;

pub struct CharReader<R: BufRead> {
    input: R,
    path: PathBuf,
}

impl CharReader<BufReader<File>> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let file = File::open(&path).map_err(|err| Error::file_open(path.to_owned(), err))?;
        let reader = BufReader::new(file);

        Ok(Self {
            input: reader,
            path: path.to_owned(),
        })
    }
}

pub enum Continuation {
    Consume,
    Peek,
}

impl<R: BufRead> CharReader<R> {
    pub fn read<Callback: FnMut(Span<char>) -> Continuation>(mut self, mut callback: Callback) -> Result<(), Error> {
        let mut buffer = String::new();
        let mut location = Location::default();

        loop {
            buffer.clear();

            match self.input.read_line(&mut buffer) {
                Ok(0) => return Ok(()),
                Err(err) => return Err(Error::file_read(self.path, err)),
                _ => (),
            }

            let mut handle_char = |char| {
                let start = location;
                location.advance();
                while let Continuation::Peek = callback(Span::new(start, location, char)) {}
            };

            for char in buffer.trim_end().chars() {
                handle_char(char);
            }

            handle_char('\n');

            location.next_line();
        }
    }
}
