#[derive(Default, Debug, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Span<Data> {
    pub start: Location,
    pub end: Location,
    pub data: Data,
}

impl Location {
    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    pub fn advance(&mut self) {
        self.col += 1;
    }
}

impl<Data> Span<Data> {
    pub fn new(start: Location, end: Location, data: Data) -> Self {
        Self { start, end, data }
    }

    pub fn swap<New>(&self, new: New) -> Span<New> {
        Span {
            start: self.start,
            end: self.end,
            data: new,
        }
    }
}
