#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Rewind,
}

pub(super) struct Clock {
    value: f64,
    step: f64,
}

impl Clock {
    pub(super) fn new(direction: Direction, frames: f64) -> Self {
        match direction {
            Direction::Forward => Self {
                value: 0.0,
                step: 1.0 / frames,
            },
            Direction::Rewind => Self {
                value: 1.0,
                step: -1.0 / frames,
            },
        }
    }

    pub(super) fn value(&self) -> f64 {
        self.value
    }

    pub(super) fn direction(&self) -> Direction {
        match self.step < 0.0 {
            true => Direction::Rewind,
            false => Direction::Forward,
        }
    }

    pub(super) fn advance(&mut self) {
        self.value += self.step;
    }

    pub(super) fn running(&self) -> bool {
        self.value > 0.0 && self.value < 1.0
    }

    pub(super) fn landed(&self) -> bool {
        self.direction() == Direction::Rewind && self.value <= 0.0
    }

    pub(super) fn fade(&self) -> f64 {
        match self.direction() {
            Direction::Rewind => 1.0,
            Direction::Forward => (1.0 - self.value).clamp(0.0, 1.0),
        }
    }
}
