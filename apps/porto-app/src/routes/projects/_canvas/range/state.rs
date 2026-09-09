use super::clock::{Clock, Direction};
use super::constants::*;
use super::particles::*;
use std::f64::consts::TAU;

pub(super) enum Rewind {
    Idle,
    Aiming,
    Rebuilding,
}

pub(super) struct RangeState {
    pub(super) plate_x: f64,
    pub(super) plate_y: f64,
    pub(super) plate_to_x: f64,
    pub(super) plate_to_y: f64,
    pub(super) history: Vec<(f64, f64)>,
    pub(super) rewind: Rewind,
    pub(super) rewind_at: (f64, f64),
    pub(super) aim: f64,
    pub(super) pending: usize,
    pub(super) cooldown: f64,
    pub(super) direction: Direction,
    pub(super) shards: Vec<Shard>,
    pub(super) plate_scale: f64,
    pub(super) flash: f64,
    pub(super) tracers: Vec<Tracer>,
    pub(super) plate_angle: f64,
    pub(super) plate_velocity: f64,
    pub(super) plate_flash: f64,
    pub(super) recoil: f64,
    pub(super) sparks: Vec<Spark>,
    pub(super) casings: Vec<Casing>,
    pub(super) seed: f64,
}

impl RangeState {
    pub(super) fn new() -> Self {
        Self {
            plate_x: 0.5,
            plate_y: 0.3,
            plate_to_x: 0.5,
            plate_to_y: 0.3,
            history: Vec::new(),
            rewind: Rewind::Idle,
            rewind_at: (0.5, 0.3),
            aim: 0.0,
            pending: 0,
            cooldown: 0.0,
            direction: Direction::Forward,
            shards: Vec::new(),
            plate_scale: 1.0,
            flash: 0.0,
            tracers: Vec::new(),
            plate_angle: 0.0,
            plate_velocity: 0.0,
            plate_flash: 0.0,
            recoil: 0.0,
            sparks: Vec::new(),
            casings: Vec::new(),
            seed: 1.0,
        }
    }

    pub(super) fn random(&mut self) -> f64 {
        self.seed = (self.seed * 9301.0 + 49297.0) % 233_280.0;
        self.seed / 233_280.0
    }

    // Retrying rather than shoving the point outward: a pushed point leaves the
    // box and lands under the dossier text.
    pub(super) fn relocate(&mut self, width: f64, height: f64) {
        if self.history.len() >= PLATE_HISTORY_MAX {
            self.history.remove(0);
        }
        self.history.push((self.plate_to_x, self.plate_to_y));

        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let min_range = width * PLATE_MIN_RANGE;

        let mut best = (self.plate_to_x, self.plate_to_y);
        let mut best_range = -1.0;
        for _ in 0..PLATE_RELOCATE_TRIES {
            let x = PLATE_X_MIN + self.random() * (PLATE_X_MAX - PLATE_X_MIN);
            let y = PLATE_Y_MIN + self.random() * (PLATE_Y_MAX - PLATE_Y_MIN);
            let range = (width * x - pivot_x).hypot(height * y - pivot_y);
            if range > best_range {
                best_range = range;
                best = (x, y);
            }
            if range >= min_range {
                break;
            }
        }

        self.plate_to_x = best.0;
        self.plate_to_y = best.1;
    }

    // Snapping, not travelling: sliding across would read as the target moving
    // rather than as this one being undone.
    pub(super) fn rewind_spot(&mut self) -> (f64, f64) {
        self.history
            .pop()
            .unwrap_or((self.plate_to_x, self.plate_to_y))
    }

    // Past vertical the sprite would hang upside down, so the body is mirrored
    // across the barrel rather than rotated through it.
    pub(super) fn flip(&self) -> f64 {
        match self.aim.cos() < 0.0 {
            true => -1.0,
            false => 1.0,
        }
    }

    pub(super) fn gun_point(
        &self,
        width: f64,
        height: f64,
        local_x: f64,
        local_y: f64,
    ) -> (f64, f64) {
        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let x = GUN_SCALE * (local_x - BIPOD_LOCAL_X - self.recoil);
        let y = GUN_SCALE * (local_y - BIPOD_LOCAL_Y) * self.flip();
        let (sin, cos) = self.aim.sin_cos();
        (pivot_x + x * cos - y * sin, pivot_y + x * sin + y * cos)
    }

    pub(super) fn inbound(shard: &Shard) -> bool {
        shard.clock.direction() == Direction::Rewind
    }

    pub(super) fn queue(&mut self, shots: usize, direction: Direction) {
        if self.direction != direction {
            self.direction = direction;
            self.pending = 0;
            self.rewind = Rewind::Idle;
        }
        let cap = match direction {
            Direction::Forward => MAX_QUEUED_SHOTS,
            Direction::Rewind => REWIND_QUEUE_MAX,
        };
        self.pending = (self.pending + shots).min(cap);
    }

    pub(super) fn burst_sparks(&mut self, x: f64, y: f64) {
        for _ in 0..SPARK_COUNT {
            let angle = self.random() * TAU;
            let speed = 1.5 + self.random() * 4.5;
            self.sparks.push(Spark {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed - 1.0,
                life: SPARK_LIFE,
            });
        }
    }

    pub(super) fn fire_round(&mut self, width: f64, height: f64, spot: (f64, f64)) {
        let direction = self.direction;

        if direction == Direction::Forward {
            self.flash = 1.0;
            self.recoil = RECOIL_KICK;
        }

        self.tracers.push(Tracer {
            plate_x: width * spot.0,
            plate_y: height * spot.1,
            clock: Clock::new(direction, TRACER_DURATION),
        });

        let spin = (self.random() - 0.5) * 0.06;
        let speed = (4.6 + self.random() * 2.2) * 0.5;
        let scatter = (self.random() - 0.5) * 0.3;
        let (sin, cos) = (self.aim + scatter).sin_cos();
        self.casings.push(Casing {
            dir_x: EJECT_LOCAL_DX * cos - EJECT_LOCAL_DY * sin,
            dir_y: EJECT_LOCAL_DX * sin + EJECT_LOCAL_DY * cos,
            speed,
            spin,
            clock: Clock::new(direction, CASING_DURATION),
        });
    }

    pub(super) fn shatter(&mut self, width: f64, height: f64, spot: (f64, f64)) {
        let direction = self.direction;
        if direction == Direction::Forward && self.plate_scale <= PLATE_INTACT_SCALE {
            return;
        }

        let plate_x = width * spot.0;
        let plate_y = height * spot.1;
        let shard_frames = match direction {
            Direction::Forward => SHARD_DURATION,
            Direction::Rewind => SHARD_DURATION / SHARD_REWIND_RATE,
        };

        let span = (SHARD_MAX - SHARD_MIN + 1) as f64;
        let count = (SHARD_MIN + (self.random() * span) as usize).min(SHARD_MAX);

        let mut cuts: Vec<f64> = Vec::with_capacity(count);
        for _ in 0..count {
            cuts.push(self.random() * TAU);
        }
        cuts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let mut rolls: Vec<(f64, f64, f64)> = Vec::with_capacity(count);
        for _ in 0..count {
            rolls.push((self.random(), self.random(), self.random()));
        }

        for (i, (speed_roll, spin_roll, radius_roll)) in rolls.into_iter().enumerate() {
            let from = cuts[i];
            let to = match i + 1 < count {
                true => cuts[i + 1],
                false => cuts[0] + TAU,
            };
            let mid = (from + to) / 2.0;
            let radius = PLATE_RADIUS
                * (1.0 - SHARD_RADIUS_JITTER / 2.0 + radius_roll * SHARD_RADIUS_JITTER);

            self.shards.push(Shard {
                origin_x: plate_x,
                origin_y: plate_y,
                dir_x: mid.cos(),
                dir_y: mid.sin(),
                speed: (SHARD_SPEED + speed_roll * SHARD_SPREAD) * 0.5,
                spin: (spin_roll - 0.5) * 0.02,
                from,
                to,
                radius,
                clock: Clock::new(direction, shard_frames),
            });
        }
        self.plate_scale = 0.0;

        if direction == Direction::Forward {
            self.plate_velocity += PLATE_KICK;
            self.plate_flash = 1.0;
            self.burst_sparks(plate_x, plate_y);
        }
    }

    pub(super) fn aim_settled(&self, width: f64, height: f64) -> bool {
        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let wanted =
            (height * self.rewind_at.1 - pivot_y).atan2(width * self.rewind_at.0 - pivot_x);
        (wanted - self.aim).abs() < AIM_SETTLED
    }

    // A shooter swings onto the next target only after firing, so undoing that
    // replays the beats in order, each one backwards.
    pub(super) fn rewind_step(&mut self, width: f64, height: f64) {
        match self.rewind {
            Rewind::Idle => {
                if self.pending == 0 {
                    return;
                }
                self.pending -= 1;
                self.rewind_at = self.rewind_spot();
                self.plate_to_x = self.rewind_at.0;
                self.plate_to_y = self.rewind_at.1;
                self.plate_x = self.rewind_at.0;
                self.plate_y = self.rewind_at.1;
                self.plate_scale = 0.0;
                self.rewind = Rewind::Aiming;
            }
            Rewind::Aiming => {
                if self.aim_settled(width, height) {
                    let spot = self.rewind_at;
                    self.fire_round(width, height, spot);
                    self.shatter(width, height, spot);
                    self.rewind = Rewind::Rebuilding;
                }
            }
            Rewind::Rebuilding => {
                if !self.shards.iter().any(Self::inbound) {
                    self.rewind = Rewind::Idle;
                }
            }
        }
    }

    pub(super) fn step(&mut self, width: f64, height: f64) {
        self.cooldown -= 1.0;
        match self.direction {
            Direction::Forward => {
                if self.pending > 0 && self.cooldown <= 0.0 {
                    self.pending -= 1;
                    self.cooldown = FIRE_INTERVAL_FRAMES;
                    let spot = (self.plate_x, self.plate_y);
                    self.fire_round(width, height, spot);
                    self.shatter(width, height, spot);
                    self.relocate(width, height);
                }
            }
            Direction::Rewind => self.rewind_step(width, height),
        }

        self.plate_x += (self.plate_to_x - self.plate_x) * PLATE_TRAVEL;
        self.plate_y += (self.plate_to_y - self.plate_y) * PLATE_TRAVEL;

        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let wanted = (height * self.plate_y - pivot_y).atan2(width * self.plate_x - pivot_x);
        let tracking = match self.direction {
            Direction::Forward => AIM_TRACKING,
            Direction::Rewind => REWIND_AIM_TRACKING,
        };
        self.aim += (wanted - self.aim) * tracking;

        self.flash = (self.flash - FLASH_DECAY).max(0.0);
        self.plate_flash = (self.plate_flash - 0.06).max(0.0);
        self.recoil *= RECOIL_RECOVER;

        self.plate_velocity -= self.plate_angle * PLATE_SPRING;
        self.plate_velocity *= PLATE_DAMPING;
        self.plate_angle += self.plate_velocity * 0.12;

        for tracer in self.tracers.iter_mut() {
            tracer.clock.advance();
        }
        if self.tracers.iter().any(|tracer| tracer.clock.landed()) {
            self.flash = 1.0;
            self.recoil = RECOIL_KICK;
        }
        self.tracers.retain(|tracer| tracer.clock.running());

        for shard in self.shards.iter_mut() {
            shard.clock.advance();
        }
        let reassembled = self
            .shards
            .iter()
            .find(|shard| shard.clock.landed())
            .map(|shard| (shard.origin_x, shard.origin_y));
        self.shards.retain(|shard| shard.clock.running());

        // Rewinding, the wedges ARE the plate coming back together, so no disc
        // may swell underneath them. Forward they leave a spot the plate has
        // already left, so there it rises on its own at the next target.
        match (reassembled, self.direction) {
            (Some((x, y)), _) => {
                self.plate_scale = 1.0;
                self.plate_flash = 1.0;
                self.plate_velocity += PLATE_KICK;
                self.burst_sparks(x, y);
            }
            (None, Direction::Forward) => {
                self.plate_scale += (1.0 - self.plate_scale) * PLATE_REGROW
            }
            (None, Direction::Rewind) => {}
        }

        for spark in self.sparks.iter_mut() {
            spark.x += spark.vx;
            spark.y += spark.vy;
            spark.vy += GRAVITY * SPARK_GRAVITY_SCALE;
            spark.life -= SPARK_FADE;
        }
        self.sparks.retain(|s| s.life > 0.0);

        for casing in self.casings.iter_mut() {
            casing.clock.advance();
        }
        self.casings.retain(|casing| casing.clock.running());
    }
}
