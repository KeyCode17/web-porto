use crate::libs::{AnimationFrameLoop, start_animation_loop};
use std::cell::RefCell;
use std::f64::consts::TAU;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// The weapon owns the lower left, the readout the right, and the target sits
// between them: the tracer then crosses open space instead of the copy.
// The bipod is planted: it is the pivot, not cargo. The weapon swings about the
// leg mount and the legs stay where they stand.
const BIPOD_X_RATIO: f64 = 0.32;
const BIPOD_Y_RATIO: f64 = 0.62;
const BIPOD_LOCAL_X: f64 = -46.0;
const BIPOD_LOCAL_Y: f64 = 6.0;
// The target roams most of the stage. Two things bound it rather than taste:
// the HUD block sits top left, and swinging past the left edge throws the butt
// of the weapon right, into the readout - the pivot is the bipod, so aiming one
// way pushes the body the other.
const PLATE_X_MIN: f64 = 0.15;
const PLATE_X_MAX: f64 = 0.62;
const PLATE_Y_MIN: f64 = 0.36;
const PLATE_Y_MAX: f64 = 0.56;
const PLATE_TRAVEL: f64 = 0.14;
const PLATE_HISTORY_MAX: usize = 64;
const PLATE_MIN_RANGE: f64 = 0.14;
const PLATE_RELOCATE_TRIES: usize = 8;
const AIM_TRACKING: f64 = 0.14;
const REWIND_AIM_TRACKING: f64 = 0.42;
const AIM_SETTLED: f64 = 0.06;
const PLATE_RADIUS: f64 = 52.0;
const GUN_SCALE: f64 = 0.95;

// Where the brass actually leaves the weapon: the ejection port on the upper
// right of the receiver, not the muzzle. Given in the gun's own frame, then
// carried through the same rotation, scale and recoil the body is drawn with.
const PORT_LOCAL_X: f64 = -196.0;
const PORT_LOCAL_Y: f64 = -20.0;
const EJECT_LOCAL_DX: f64 = -0.35;
const EJECT_LOCAL_DY: f64 = -1.0;

// Shots are queued, not fired on the spot: a smooth scroll can skip several
// rounds between two scroll events, and firing once per change turned a jump of
// seven rounds into a single shot. Queueing the difference gives the burst.
const FIRE_INTERVAL_FRAMES: f64 = 4.0;
const MAX_QUEUED_SHOTS: usize = 24;
const REWIND_QUEUE_MAX: usize = 8;

const RECOIL_KICK: f64 = 14.0;
const RECOIL_RECOVER: f64 = 0.82;

const GUN_BODY: &str = "#AEB9C4";
const GUN_WOOD: &str = "#8C7A63";

const FLASH_DECAY: f64 = 0.16;
const TRACER_DURATION: f64 = 5.0;
const TRACER_TAIL: f64 = 0.22;
const PLATE_SPRING: f64 = 0.055;
const PLATE_DAMPING: f64 = 0.88;
const PLATE_KICK: f64 = 0.9;
const GRAVITY: f64 = 0.55;

const SHARD_MIN: usize = 6;
const SHARD_MAX: usize = 12;
const SHARD_RADIUS_JITTER: f64 = 0.28;
const SHARD_SPEED: f64 = 3.2;
const SHARD_SPREAD: f64 = 2.6;
const SHARD_DURATION: f64 = 40.0;
const SHARD_REWIND_RATE: f64 = 3.5;
const SHARD_GRAVITY: f64 = 0.07;
const CASING_DURATION: f64 = 46.0;
const CASING_GRAVITY: f64 = 0.09;
const PLATE_REGROW: f64 = 0.11;

const SPARK_COUNT: usize = 22;
const SPARK_LIFE: f64 = 1.0;
const SPARK_FADE: f64 = 0.035;

const CASING_COLOR: &str = "#C8A951";
const PLATE_COLOR: &str = "#8C949B";
const PLATE_RIM: &str = "#E5E5E5";
const RING_COLOR: &str = "#D65108";
const FLASH_COLOR: &str = "#FFD9A0";

struct Spark {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    life: f64,
}

// One wedge of the plate. Position is a pure function of its own clock rather
// than accumulated velocity, so running the clock backwards retraces the exact
// arc it flew out on - flipping gravity instead would just launch it away.
struct Shard {
    origin_x: f64,
    origin_y: f64,
    dir_x: f64,
    dir_y: f64,
    speed: f64,
    spin: f64,
    from: f64,
    to: f64,
    radius: f64,
    clock: f64,
    step: f64,
}

impl Shard {
    fn frame(&self) -> f64 {
        self.clock * SHARD_DURATION
    }

    fn position(&self) -> (f64, f64) {
        let f = self.frame();
        (
            self.origin_x + self.dir_x * self.speed * f,
            self.origin_y + self.dir_y * self.speed * f + 0.5 * SHARD_GRAVITY * f * f,
        )
    }

    fn alpha(&self) -> f64 {
        match self.step < 0.0 {
            true => 1.0,
            false => (1.0 - self.clock).clamp(0.0, 1.0),
        }
    }

    fn landed(&self) -> bool {
        self.step < 0.0 && self.clock <= 0.0
    }
}

enum Rewind {
    Idle,
    Aiming,
    Rebuilding,
}

struct Tracer {
    plate_x: f64,
    plate_y: f64,
    clock: f64,
    step: f64,
}

impl Tracer {
    fn at(&self, t: f64, muzzle: (f64, f64)) -> (f64, f64) {
        let t = t.clamp(0.0, 1.0);
        (
            muzzle.0 + (self.plate_x - muzzle.0) * t,
            muzzle.1 + (self.plate_y - muzzle.1) * t,
        )
    }

    fn head(&self, muzzle: (f64, f64)) -> (f64, f64) {
        self.at(self.clock, muzzle)
    }

    fn tail(&self, muzzle: (f64, f64)) -> (f64, f64) {
        let sign = match self.step < 0.0 {
            true => -1.0,
            false => 1.0,
        };
        self.at(self.clock - TRACER_TAIL * sign, muzzle)
    }

    fn landed(&self) -> bool {
        self.step < 0.0 && self.clock <= 0.0
    }
}

struct Casing {
    dir_x: f64,
    dir_y: f64,
    speed: f64,
    spin: f64,
    clock: f64,
    step: f64,
}

impl Casing {
    fn frame(&self) -> f64 {
        self.clock * CASING_DURATION
    }

    fn alpha(&self) -> f64 {
        match self.step < 0.0 {
            true => 1.0,
            false => (1.0 - self.clock).clamp(0.0, 1.0),
        }
    }

    fn position(&self, port: (f64, f64)) -> (f64, f64) {
        let f = self.frame();
        (
            port.0 + self.dir_x * self.speed * f,
            port.1 + self.dir_y * self.speed * f + 0.5 * CASING_GRAVITY * f * f,
        )
    }
}

struct RangeState {
    plate_x: f64,
    plate_y: f64,
    plate_to_x: f64,
    plate_to_y: f64,
    history: Vec<(f64, f64)>,
    rewind: Rewind,
    rewind_at: (f64, f64),
    aim: f64,
    pending: usize,
    cooldown: f64,
    forward: bool,
    shards: Vec<Shard>,
    plate_scale: f64,
    flash: f64,
    tracers: Vec<Tracer>,
    plate_angle: f64,
    plate_velocity: f64,
    plate_flash: f64,
    recoil: f64,
    sparks: Vec<Spark>,
    casings: Vec<Casing>,
    seed: f64,
}

impl RangeState {
    fn new() -> Self {
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
            forward: true,
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

    fn random(&mut self) -> f64 {
        self.seed = (self.seed * 9301.0 + 49297.0) % 233_280.0;
        self.seed / 233_280.0
    }

    // Rolls inside the same box, but keeps the target off the weapon's lap: a
    // plate that lands next to the muzzle leaves no flight to watch. Retrying
    // rather than shoving the point outward keeps it clear of the dossier text.
    fn relocate(&mut self, width: f64, height: f64) {
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

    // The plate standing here is undone, so it leaves rather than shatters; the
    // one that breaks apart - and therefore reassembles - is the previous spot.
    // Snapping, not travelling: sliding across would read as the target moving.
    fn rewind_spot(&mut self) -> (f64, f64) {
        self.history
            .pop()
            .unwrap_or((self.plate_to_x, self.plate_to_y))
    }

    // Maps a point from the weapon's frame into canvas space.
    // Past vertical the sprite would hang upside down - magazine and trigger on
    // top - so the body is mirrored across the barrel instead of rotated through.
    // The barrel still points at the target either way.
    fn flip(&self) -> f64 {
        match self.aim.cos() < 0.0 {
            true => -1.0,
            false => 1.0,
        }
    }

    fn gun_point(&self, width: f64, height: f64, local_x: f64, local_y: f64) -> (f64, f64) {
        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let x = GUN_SCALE * (local_x - BIPOD_LOCAL_X - self.recoil);
        let y = GUN_SCALE * (local_y - BIPOD_LOCAL_Y) * self.flip();
        let (sin, cos) = self.aim.sin_cos();
        (pivot_x + x * cos - y * sin, pivot_y + x * sin + y * cos)
    }

    fn queue(&mut self, shots: usize, forward: bool) {
        if self.forward != forward {
            self.forward = forward;
            self.pending = 0;
            self.rewind = Rewind::Idle;
        }
        let cap = match forward {
            true => MAX_QUEUED_SHOTS,
            false => REWIND_QUEUE_MAX,
        };
        self.pending = (self.pending + shots).min(cap);
    }

    fn burst_sparks(&mut self, x: f64, y: f64) {
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

    fn fire_round(&mut self, width: f64, height: f64, spot: (f64, f64)) {
        let forward = self.forward;

        if forward {
            self.flash = 1.0;
            self.recoil = RECOIL_KICK;
        }

        let (tracer_clock, tracer_step) = match forward {
            true => (0.0, 1.0 / TRACER_DURATION),
            false => (1.0, -1.0 / TRACER_DURATION),
        };

        self.tracers.push(Tracer {
            plate_x: width * spot.0,
            plate_y: height * spot.1,
            clock: tracer_clock,
            step: tracer_step,
        });

        let spin = (self.random() - 0.5) * 0.06;
        let speed = (4.6 + self.random() * 2.2) * 0.5;
        let scatter = (self.random() - 0.5) * 0.3;
        let (sin, cos) = (self.aim + scatter).sin_cos();
        let (casing_clock, casing_step) = match forward {
            true => (0.0, 1.0 / CASING_DURATION),
            false => (1.0, -1.0 / CASING_DURATION),
        };

        self.casings.push(Casing {
            dir_x: EJECT_LOCAL_DX * cos - EJECT_LOCAL_DY * sin,
            dir_y: EJECT_LOCAL_DX * sin + EJECT_LOCAL_DY * cos,
            speed,
            spin,
            clock: casing_clock,
            step: casing_step,
        });
    }

    fn shatter(&mut self, width: f64, height: f64, spot: (f64, f64)) {
        let forward = self.forward;
        if forward && self.plate_scale <= 0.35 {
            return;
        }

        let plate_x = width * spot.0;
        let plate_y = height * spot.1;
        let (clock, step) = match forward {
            true => (0.0, 1.0 / SHARD_DURATION),
            false => (1.0, -SHARD_REWIND_RATE / SHARD_DURATION),
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
                clock,
                step,
            });
        }
        self.plate_scale = 0.0;

        if forward {
            self.plate_velocity += PLATE_KICK;
            self.plate_flash = 1.0;
            self.burst_sparks(plate_x, plate_y);
        }
    }

    fn aim_settled(&self, width: f64, height: f64) -> bool {
        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let wanted =
            (height * self.rewind_at.1 - pivot_y).atan2(width * self.rewind_at.0 - pivot_x);
        (wanted - self.aim).abs() < AIM_SETTLED
    }

    // A shooter swings onto the next target only after firing, so undoing that
    // runs the beats in the same order with each one played backwards: the
    // weapon comes back on aim first, and the instant the round leaves the plate
    // to fly home the plate starts closing up behind it.
    fn rewind_step(&mut self, width: f64, height: f64) {
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
                if !self.shards.iter().any(|shard| shard.step < 0.0) {
                    self.rewind = Rewind::Idle;
                }
            }
        }
    }

    fn step(&mut self, width: f64, height: f64) {
        self.cooldown -= 1.0;
        match self.forward {
            true => {
                if self.pending > 0 && self.cooldown <= 0.0 {
                    self.pending -= 1;
                    self.cooldown = FIRE_INTERVAL_FRAMES;
                    let spot = (self.plate_x, self.plate_y);
                    self.fire_round(width, height, spot);
                    self.shatter(width, height, spot);
                    self.relocate(width, height);
                }
            }
            false => self.rewind_step(width, height),
        }

        self.plate_x += (self.plate_to_x - self.plate_x) * PLATE_TRAVEL;
        self.plate_y += (self.plate_to_y - self.plate_y) * PLATE_TRAVEL;

        let pivot_x = width * BIPOD_X_RATIO;
        let pivot_y = height * BIPOD_Y_RATIO;
        let wanted = (height * self.plate_y - pivot_y).atan2(width * self.plate_x - pivot_x);
        let tracking = match self.forward {
            true => AIM_TRACKING,
            false => REWIND_AIM_TRACKING,
        };
        self.aim += (wanted - self.aim) * tracking;

        self.flash = (self.flash - FLASH_DECAY).max(0.0);
        self.plate_flash = (self.plate_flash - 0.06).max(0.0);
        self.recoil *= RECOIL_RECOVER;

        self.plate_velocity -= self.plate_angle * PLATE_SPRING;
        self.plate_velocity *= PLATE_DAMPING;
        self.plate_angle += self.plate_velocity * 0.12;

        for tracer in self.tracers.iter_mut() {
            tracer.clock += tracer.step;
        }
        if self.tracers.iter().any(Tracer::landed) {
            self.flash = 1.0;
            self.recoil = RECOIL_KICK;
        }
        self.tracers.retain(|t| t.clock > 0.0 && t.clock < 1.0);

        for shard in self.shards.iter_mut() {
            shard.clock += shard.step;
        }
        let reassembled = self
            .shards
            .iter()
            .find(|shard| shard.landed())
            .map(|shard| (shard.origin_x, shard.origin_y));
        self.shards.retain(|s| s.clock > 0.0 && s.clock < 1.0);

        // Rewinding, the wedges themselves are the plate coming back together, so
        // no disc swells underneath them - it is simply absent until the first
        // batch closes the circle, then stands whole while later ones keep
        // streaming home into it. Going forward they leave a spot the plate has
        // already left, so there it rises on its own at the next target.
        match (reassembled, self.forward) {
            (Some((x, y)), _) => {
                self.plate_scale = 1.0;
                self.plate_flash = 1.0;
                self.plate_velocity += PLATE_KICK;
                self.burst_sparks(x, y);
            }
            (None, true) => self.plate_scale += (1.0 - self.plate_scale) * PLATE_REGROW,
            (None, false) => {}
        }

        for spark in self.sparks.iter_mut() {
            spark.x += spark.vx;
            spark.y += spark.vy;
            spark.vy += GRAVITY * 0.35;
            spark.life -= SPARK_FADE;
        }
        self.sparks.retain(|s| s.life > 0.0);

        for casing in self.casings.iter_mut() {
            casing.clock += casing.step;
        }
        self.casings.retain(|c| c.clock > 0.0 && c.clock < 1.0);
    }
}

// The weapon is drawn in its own frame: the muzzle sits at the origin and the
// body runs back along -x, so aiming is one rotation and recoil is one offset
// along the same axis.
fn poly(ctx: &CanvasRenderingContext2d, points: &[(f64, f64)]) {
    let Some((first_x, first_y)) = points.first().copied() else {
        return;
    };
    ctx.begin_path();
    ctx.move_to(first_x, first_y);
    for (x, y) in points.iter().skip(1) {
        ctx.line_to(*x, *y);
    }
    ctx.close_path();
    ctx.fill();
}

// Drawn as a flat silhouette in the weapon's own frame: muzzle at the origin,
// body running back along -x, underside at +y. Traced from a light machine gun
// profile - bipod at the front, gas tube over the barrel, angled box magazine,
// trigger guard and a curved wooden butt.
fn draw_bipod(ctx: &CanvasRenderingContext2d, pivot_x: f64, pivot_y: f64) {
    ctx.save();
    let _ = ctx.translate(pivot_x, pivot_y);
    ctx.set_stroke_style_str(GUN_BODY);
    ctx.set_line_cap("round");

    ctx.set_line_width(7.0);
    ctx.begin_path();
    ctx.move_to(0.0, 0.0);
    ctx.line_to(34.0, 66.0);
    ctx.move_to(0.0, 0.0);
    ctx.line_to(-34.0, 66.0);
    ctx.stroke();

    ctx.set_line_width(5.0);
    ctx.begin_path();
    ctx.move_to(22.0, 68.0);
    ctx.line_to(46.0, 62.0);
    ctx.move_to(-22.0, 68.0);
    ctx.line_to(-46.0, 62.0);
    ctx.stroke();

    ctx.restore();
}

fn draw_gun(
    ctx: &CanvasRenderingContext2d,
    pivot_x: f64,
    pivot_y: f64,
    aim: f64,
    recoil: f64,
    flip: f64,
) {
    ctx.save();
    let _ = ctx.translate(pivot_x, pivot_y);
    let _ = ctx.rotate(aim);
    let _ = ctx.scale(GUN_SCALE, GUN_SCALE * flip);
    let _ = ctx.translate(-BIPOD_LOCAL_X - recoil, -BIPOD_LOCAL_Y);

    ctx.set_fill_style_str(GUN_BODY);

    // barrel, gas tube and front sight
    ctx.fill_rect(-150.0, -5.0, 150.0, 11.0);
    ctx.fill_rect(-146.0, -22.0, 108.0, 9.0);
    poly(
        ctx,
        &[(-14.0, -5.0), (-4.0, -5.0), (-6.0, -30.0), (-12.0, -30.0)],
    );

    // handguard and receiver
    poly(
        ctx,
        &[
            (-150.0, -14.0),
            (-196.0, -20.0),
            (-262.0, -18.0),
            (-262.0, 24.0),
            (-206.0, 26.0),
            (-150.0, 14.0),
        ],
    );

    // angled box magazine under the receiver
    poly(
        ctx,
        &[
            (-186.0, 20.0),
            (-236.0, 24.0),
            (-224.0, 92.0),
            (-172.0, 84.0),
        ],
    );

    // trigger guard
    ctx.set_line_width(6.0);
    ctx.begin_path();
    let _ = ctx.arc(-274.0, 40.0, 17.0, -0.35, 3.3);
    ctx.stroke();

    // pistol grip
    poly(
        ctx,
        &[
            (-262.0, 22.0),
            (-292.0, 22.0),
            (-306.0, 84.0),
            (-280.0, 88.0),
        ],
    );

    // wooden butt, curved along the underside
    ctx.set_fill_style_str(GUN_WOOD);
    poly(
        ctx,
        &[
            (-262.0, -14.0),
            (-408.0, 6.0),
            (-412.0, 46.0),
            (-372.0, 52.0),
            (-330.0, 34.0),
            (-292.0, 24.0),
            (-262.0, 20.0),
        ],
    );

    ctx.restore();
}

fn draw_muzzle_flash(ctx: &CanvasRenderingContext2d, x: f64, y: f64, aim: f64, flash: f64) {
    if flash <= 0.0 {
        return;
    }

    ctx.save();
    let _ = ctx.translate(x, y);
    let _ = ctx.rotate(aim);
    ctx.set_global_alpha(flash);

    let reach = 26.0 + flash * 46.0;
    let spread = 9.0 + flash * 17.0;

    ctx.set_fill_style_str(FLASH_COLOR);
    ctx.begin_path();
    ctx.move_to(0.0, 0.0);
    ctx.line_to(reach, -spread);
    ctx.line_to(reach * 1.28, 0.0);
    ctx.line_to(reach, spread);
    ctx.close_path();
    ctx.fill();

    ctx.set_fill_style_str("#FFFFFF");
    ctx.begin_path();
    let _ = ctx.arc(4.0, 0.0, 4.0 + flash * 9.0, 0.0, TAU);
    ctx.fill();

    ctx.set_global_alpha(1.0);
    ctx.restore();
}

fn draw_shards(ctx: &CanvasRenderingContext2d, state: &RangeState) {
    for shard in state.shards.iter() {
        ctx.save();
        let (x, y) = shard.position();
        let _ = ctx.translate(x, y);
        let _ = ctx.rotate(shard.spin * shard.frame());
        ctx.set_global_alpha(shard.alpha());

        ctx.set_fill_style_str(PLATE_COLOR);
        ctx.begin_path();
        ctx.move_to(0.0, 0.0);
        let _ = ctx.arc(0.0, 0.0, shard.radius, shard.from, shard.to);
        ctx.close_path();
        ctx.fill();

        ctx.set_stroke_style_str(PLATE_RIM);
        ctx.set_line_width(2.0);
        ctx.stroke();

        ctx.set_global_alpha(1.0);
        ctx.restore();
    }
}

fn draw_plate(ctx: &CanvasRenderingContext2d, x: f64, y: f64, angle: f64, flash: f64, scale: f64) {
    if scale < 0.03 {
        return;
    }

    ctx.save();
    let _ = ctx.translate(x, y);
    let _ = ctx.rotate(angle);
    let _ = ctx.scale(scale, scale);

    ctx.set_fill_style_str(PLATE_COLOR);
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.0, PLATE_RADIUS, 0.0, TAU);
    ctx.fill();

    ctx.set_stroke_style_str(PLATE_RIM);
    ctx.set_line_width(4.0);
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.0, PLATE_RADIUS, 0.0, TAU);
    ctx.stroke();

    ctx.set_stroke_style_str(RING_COLOR);
    ctx.set_line_width(3.0);
    for ring in 1..=2 {
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, PLATE_RADIUS * (ring as f64) / 3.0, 0.0, TAU);
        ctx.stroke();
    }

    ctx.set_fill_style_str(RING_COLOR);
    ctx.begin_path();
    let _ = ctx.arc(0.0, 0.0, 5.0, 0.0, TAU);
    ctx.fill();

    if flash > 0.0 {
        ctx.set_global_alpha(flash * 0.8);
        ctx.set_fill_style_str(FLASH_COLOR);
        ctx.begin_path();
        let _ = ctx.arc(0.0, 0.0, PLATE_RADIUS * 0.9, 0.0, TAU);
        ctx.fill();
        ctx.set_global_alpha(1.0);
    }

    ctx.restore();
}

fn draw(ctx: &CanvasRenderingContext2d, state: &RangeState, width: f64, height: f64) {
    ctx.clear_rect(0.0, 0.0, width, height);

    let pivot_x = width * BIPOD_X_RATIO;
    let pivot_y = height * BIPOD_Y_RATIO;
    let (muzzle_x, muzzle_y) = state.gun_point(width, height, 0.0, 0.0);
    let plate_x = width * state.plate_x;
    let plate_y = height * state.plate_y;

    let aim = state.aim;
    draw_gun(ctx, pivot_x, pivot_y, aim, state.recoil, state.flip());
    draw_bipod(ctx, pivot_x, pivot_y);

    ctx.set_stroke_style_str(FLASH_COLOR);
    ctx.set_line_width(2.0);
    for tracer in state.tracers.iter() {
        let (head_x, head_y) = tracer.head((muzzle_x, muzzle_y));
        let (tail_x, tail_y) = tracer.tail((muzzle_x, muzzle_y));
        ctx.begin_path();
        ctx.move_to(tail_x, tail_y);
        ctx.line_to(head_x, head_y);
        ctx.stroke();
    }

    draw_shards(ctx, state);
    draw_plate(
        ctx,
        plate_x,
        plate_y,
        state.plate_angle,
        state.plate_flash,
        state.plate_scale,
    );

    draw_muzzle_flash(ctx, muzzle_x, muzzle_y, aim, state.flash);

    ctx.set_fill_style_str(RING_COLOR);
    for spark in state.sparks.iter() {
        ctx.set_global_alpha(spark.life.clamp(0.0, 1.0));
        ctx.begin_path();
        let _ = ctx.arc(spark.x, spark.y, 2.0, 0.0, TAU);
        ctx.fill();
    }
    ctx.set_global_alpha(1.0);

    let port = state.gun_point(width, height, PORT_LOCAL_X, PORT_LOCAL_Y);
    for casing in state.casings.iter() {
        ctx.save();
        let (x, y) = casing.position(port);
        let _ = ctx.translate(x, y);
        let _ = ctx.rotate(casing.spin * casing.frame());
        ctx.set_global_alpha(casing.alpha());
        ctx.set_fill_style_str(CASING_COLOR);
        ctx.fill_rect(-3.0, -7.0, 6.0, 14.0);
        ctx.set_global_alpha(1.0);
        ctx.restore();
    }
}

pub struct ShootingRange {
    state: Rc<RefCell<RangeState>>,
    _frames: AnimationFrameLoop,
}

impl ShootingRange {
    pub fn queue_shots(&self, shots: usize, forward: bool) {
        self.state.borrow_mut().queue(shots.max(1), forward);
    }
}

pub fn start_range(canvas_id: &str) -> Option<ShootingRange> {
    let document = web_sys::window()?.document()?;
    let canvas = document
        .get_element_by_id(canvas_id)?
        .dyn_into::<HtmlCanvasElement>()
        .ok()?;

    let rect = canvas.get_bounding_client_rect();
    if rect.width() < 1.0 || rect.height() < 1.0 {
        return None;
    }
    canvas.set_width(rect.width() as u32);
    canvas.set_height(rect.height() as u32);

    let ctx = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;

    let state = Rc::new(RefCell::new(RangeState::new()));
    let canvas_rc = Rc::new(canvas);

    let loop_state = state.clone();
    let loop_canvas = canvas_rc.clone();
    let frames = start_animation_loop(move || {
        let width = loop_canvas.width() as f64;
        let height = loop_canvas.height() as f64;
        let mut current = loop_state.borrow_mut();
        current.step(width, height);
        draw(&ctx, &current, width, height);
    })?;

    Some(ShootingRange {
        state,
        _frames: frames,
    })
}
