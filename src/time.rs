#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    pub start: f32,
    pub end: f32,
}

impl Time {
    pub fn new(start: f32, end: f32) -> Self {
        Self { start, end }
    }

    pub fn from_exposure(exposure: f32) -> Self {
        Self {
            start: 0.0,
            end: exposure,
        }
    }

    pub fn duration(&self) -> f32 {
        self.end - self.start
    }
}
