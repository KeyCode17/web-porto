use super::clock::{Clock, Direction};
use super::constants::*;

pub(super) struct Spark {
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) vx: f64,
    pub(super) vy: f64,
    pub(super) life: f64,
}

// A wedge's position is a pure function of its own clock rather than of
// accumulated velocity, so running that clock backwards retraces the exact arc
// it flew out on. Mirroring gravity instead only launches the debris away.
pub(super) struct Shard {
    pub(super) origin_x: f64,
    pub(super) origin_y: f64,
    pub(super) dir_x: f64,
    pub(super) dir_y: f64,
    pub(super) speed: f64,
    pub(super) spin: f64,
    pub(super) from: f64,
    pub(super) to: f64,
    pub(super) radius: f64,
    pub(super) clock: Clock,
}

impl Shard {
    pub(super) fn frame(&self) -> f64 {
        self.clock.value() * SHARD_DURATION
    }

    pub(super) fn position(&self) -> (f64, f64) {
        let f = self.frame();
        (
            self.origin_x + self.dir_x * self.speed * f,
            self.origin_y + self.dir_y * self.speed * f + 0.5 * SHARD_GRAVITY * f * f,
        )
    }
}

pub(super) struct Tracer {
    pub(super) plate_x: f64,
    pub(super) plate_y: f64,
    pub(super) clock: Clock,
}

impl Tracer {
    fn at(&self, t: f64, muzzle: (f64, f64)) -> (f64, f64) {
        let t = t.clamp(0.0, 1.0);
        (
            muzzle.0 + (self.plate_x - muzzle.0) * t,
            muzzle.1 + (self.plate_y - muzzle.1) * t,
        )
    }

    pub(super) fn head(&self, muzzle: (f64, f64)) -> (f64, f64) {
        self.at(self.clock.value(), muzzle)
    }

    pub(super) fn tail(&self, muzzle: (f64, f64)) -> (f64, f64) {
        let sign = match self.clock.direction() {
            Direction::Rewind => -1.0,
            Direction::Forward => 1.0,
        };
        self.at(self.clock.value() - TRACER_TAIL * sign, muzzle)
    }
}

pub(super) struct Casing {
    pub(super) dir_x: f64,
    pub(super) dir_y: f64,
    pub(super) speed: f64,
    pub(super) spin: f64,
    pub(super) clock: Clock,
}

impl Casing {
    pub(super) fn frame(&self) -> f64 {
        self.clock.value() * CASING_DURATION
    }

    pub(super) fn position(&self, port: (f64, f64)) -> (f64, f64) {
        let f = self.frame();
        (
            port.0 + self.dir_x * self.speed * f,
            port.1 + self.dir_y * self.speed * f + 0.5 * CASING_GRAVITY * f * f,
        )
    }
}
