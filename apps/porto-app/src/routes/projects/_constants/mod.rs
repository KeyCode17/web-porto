pub const RANGE_CANVAS_ID: &str = "projects-range";
pub const RANGE_CONTAINER_ID: &str = "projects-range-scroll";

// Scroll distance per project. A full screen each turned sixteen projects into
// seventeen screens of scrolling, so rounds come in bursts instead: the whole
// belt now runs inside roughly three screens.
pub const SCROLL_PER_ROUND_VH: usize = 13;
pub const RANGE_TAIL_VH: usize = 60;

pub const LOAD_DELAY_MS: i32 = 180;
pub const BELT_STAGGER_MS: usize = 45;

// Round width plus the gap between rounds. The strip is centred and shifted by
// whole multiples of this, so the chambered round always lands under the marker
// without measuring anything in the DOM.
pub const BELT_PITCH_PX: f64 = 54.0;
