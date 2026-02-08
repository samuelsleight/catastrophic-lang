use std::time::Instant;

struct TimeEntry {
    label: String,
    start: Instant,
    end: Option<Instant>,
}

pub struct TimeKeeper {
    entries: Vec<TimeEntry>,
}

#[must_use]
pub struct TimeScope<'keeper> {
    entries: &'keeper mut Vec<TimeEntry>,
    start: usize,
    indent: usize,
}

impl TimeKeeper {
    pub fn new<S: ToString>(label: &S) -> Self {
        let entry = TimeEntry {
            label: label.to_string(),
            start: Instant::now(),
            end: None,
        };

        Self { entries: vec![entry] }
    }

    pub fn scope<S: ToString>(&mut self, label: &S) -> TimeScope<'_> {
        TimeScope::make(&mut self.entries, 1, label.to_string())
    }

    pub fn finish(mut self) {
        TimeScope::finish(&mut self.entries, 0);

        for entry in self.entries {
            println!(
                "{}: \t\t{:0<11}s",
                entry.label,
                (entry.end.unwrap_or_else(Instant::now) - entry.start).as_secs_f64()
            );
        }
    }
}

impl<'keeper> TimeScope<'keeper> {
    pub fn scope<S: ToString>(&mut self, label: &S) -> TimeScope<'_> {
        Self::make(self.entries, self.indent + 1, label.to_string())
    }

    fn make(entries: &mut Vec<TimeEntry>, indent: usize, mut label: String) -> TimeScope<'_> {
        for _ in 0..indent {
            label.insert(0, ' ');
        }

        let entry = TimeEntry {
            label,
            start: Instant::now(),
            end: None,
        };

        let start = entries.len();
        entries.push(entry);

        TimeScope { entries, start, indent }
    }

    fn finish(entries: &mut [TimeEntry], index: usize) {
        entries.get_mut(index).unwrap().end = Some(Instant::now());
    }
}

impl<'keeper> Drop for TimeScope<'keeper> {
    fn drop(&mut self) {
        Self::finish(self.entries, self.start);
    }
}
