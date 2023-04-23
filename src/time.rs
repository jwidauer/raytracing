#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    pub start: f64,
    pub end: f64,
}

impl Time {
    pub fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    pub fn from_exposure(exposure: f64) -> Self {
        Self {
            start: 0.0,
            end: exposure,
        }
    }

    pub fn duration(&self) -> f64 {
        self.end - self.start
    }
}
