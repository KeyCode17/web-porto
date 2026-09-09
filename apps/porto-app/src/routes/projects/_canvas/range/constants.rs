// The bipod is planted: it is the pivot, not cargo. The weapon swings about the
// leg mount and the legs stay where they stand.
pub(super) const BIPOD_X_RATIO: f64 = 0.32;
pub(super) const BIPOD_Y_RATIO: f64 = 0.62;
pub(super) const BIPOD_LOCAL_X: f64 = -46.0;
pub(super) const BIPOD_LOCAL_Y: f64 = 6.0;
// Aiming left throws the butt right, into the readout, because the pivot is the
// bipod - so the box is bounded by where the BODY lands, not where the plate does.
pub(super) const PLATE_X_MIN: f64 = 0.15;
pub(super) const PLATE_X_MAX: f64 = 0.62;
pub(super) const PLATE_Y_MIN: f64 = 0.36;
pub(super) const PLATE_Y_MAX: f64 = 0.56;
pub(super) const PLATE_TRAVEL: f64 = 0.14;
pub(super) const PLATE_HISTORY_MAX: usize = 64;
pub(super) const PLATE_MIN_RANGE: f64 = 0.14;
pub(super) const PLATE_RELOCATE_TRIES: usize = 8;
pub(super) const AIM_TRACKING: f64 = 0.14;
pub(super) const REWIND_AIM_TRACKING: f64 = 0.42;
pub(super) const AIM_SETTLED: f64 = 0.06;
pub(super) const PLATE_RADIUS: f64 = 52.0;
pub(super) const GUN_SCALE: f64 = 0.95;

// The port, not the muzzle. Given in the gun's own frame so it rides the same
// rotation, scale and recoil the body is drawn with.
pub(super) const PORT_LOCAL_X: f64 = -196.0;
pub(super) const PORT_LOCAL_Y: f64 = -20.0;
pub(super) const EJECT_LOCAL_DX: f64 = -0.35;
pub(super) const EJECT_LOCAL_DY: f64 = -1.0;

// Queued, not fired on the spot: one scroll event can span several rounds, and
// firing once per event collapsed a jump of seven into a single shot.
pub(super) const FIRE_INTERVAL_FRAMES: f64 = 4.0;
pub(super) const MAX_QUEUED_SHOTS: usize = 24;
pub(super) const REWIND_QUEUE_MAX: usize = 8;

pub(super) const RECOIL_KICK: f64 = 14.0;
pub(super) const RECOIL_RECOVER: f64 = 0.82;

pub(super) const GUN_BODY: &str = "#AEB9C4";
pub(super) const GUN_WOOD: &str = "#8C7A63";

pub(super) const FLASH_DECAY: f64 = 0.16;
pub(super) const TRACER_DURATION: f64 = 5.0;
pub(super) const TRACER_TAIL: f64 = 0.22;
pub(super) const PLATE_SPRING: f64 = 0.055;
pub(super) const PLATE_DAMPING: f64 = 0.88;
pub(super) const PLATE_KICK: f64 = 0.9;
pub(super) const GRAVITY: f64 = 0.55;

pub(super) const SHARD_MIN: usize = 6;
pub(super) const SHARD_MAX: usize = 12;
pub(super) const SHARD_RADIUS_JITTER: f64 = 0.28;
pub(super) const SHARD_SPEED: f64 = 3.2;
pub(super) const SHARD_SPREAD: f64 = 2.6;
pub(super) const SHARD_DURATION: f64 = 40.0;
pub(super) const SHARD_REWIND_RATE: f64 = 3.5;
pub(super) const SHARD_GRAVITY: f64 = 0.07;
pub(super) const CASING_DURATION: f64 = 46.0;
pub(super) const CASING_GRAVITY: f64 = 0.09;
pub(super) const PLATE_REGROW: f64 = 0.11;
pub(super) const PLATE_INTACT_SCALE: f64 = 0.35;

pub(super) const SPARK_COUNT: usize = 22;
pub(super) const SPARK_LIFE: f64 = 1.0;
pub(super) const SPARK_FADE: f64 = 0.035;
pub(super) const SPARK_GRAVITY_SCALE: f64 = 0.35;

pub(super) const CASING_COLOR: &str = "#C8A951";
pub(super) const PLATE_COLOR: &str = "#8C949B";
pub(super) const PLATE_RIM: &str = "#E5E5E5";
pub(super) const RING_COLOR: &str = "#D65108";
pub(super) const FLASH_COLOR: &str = "#FFD9A0";
