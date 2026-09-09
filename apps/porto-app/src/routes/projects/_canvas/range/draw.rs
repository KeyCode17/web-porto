use super::constants::*;
use super::state::RangeState;
use std::f64::consts::TAU;
use web_sys::CanvasRenderingContext2d;

pub(super) fn poly(ctx: &CanvasRenderingContext2d, points: &[(f64, f64)]) {
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

pub(super) fn draw_bipod(ctx: &CanvasRenderingContext2d, pivot_x: f64, pivot_y: f64) {
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

fn draw_barrel(ctx: &CanvasRenderingContext2d) {
    ctx.fill_rect(-150.0, -5.0, 150.0, 11.0);
    ctx.fill_rect(-146.0, -22.0, 108.0, 9.0);
    poly(
        ctx,
        &[(-14.0, -5.0), (-4.0, -5.0), (-6.0, -30.0), (-12.0, -30.0)],
    );
}

fn draw_receiver(ctx: &CanvasRenderingContext2d) {
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
}

fn draw_magazine(ctx: &CanvasRenderingContext2d) {
    poly(
        ctx,
        &[
            (-186.0, 20.0),
            (-236.0, 24.0),
            (-224.0, 92.0),
            (-172.0, 84.0),
        ],
    );
}

fn draw_trigger_guard(ctx: &CanvasRenderingContext2d) {
    ctx.set_line_width(6.0);
    ctx.begin_path();
    let _ = ctx.arc(-274.0, 40.0, 17.0, -0.35, 3.3);
    ctx.stroke();
}

fn draw_grip(ctx: &CanvasRenderingContext2d) {
    poly(
        ctx,
        &[
            (-262.0, 22.0),
            (-292.0, 22.0),
            (-306.0, 84.0),
            (-280.0, 88.0),
        ],
    );
}

fn draw_stock(ctx: &CanvasRenderingContext2d) {
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
}

// Traced in the weapon's own frame: muzzle at the origin, body back along -x,
// underside at +y - so aiming is one rotation and recoil one slide along -x.
pub(super) fn draw_gun(
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
    draw_barrel(ctx);
    draw_receiver(ctx);
    draw_magazine(ctx);
    draw_trigger_guard(ctx);
    draw_grip(ctx);
    draw_stock(ctx);

    ctx.restore();
}

pub(super) fn draw_muzzle_flash(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    aim: f64,
    flash: f64,
) {
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

pub(super) fn draw_shards(ctx: &CanvasRenderingContext2d, state: &RangeState) {
    for shard in state.shards.iter() {
        ctx.save();
        let (x, y) = shard.position();
        let _ = ctx.translate(x, y);
        let _ = ctx.rotate(shard.spin * shard.frame());
        ctx.set_global_alpha(shard.clock.fade());

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

pub(super) fn draw_plate(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    angle: f64,
    flash: f64,
    scale: f64,
) {
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

pub(super) fn draw(ctx: &CanvasRenderingContext2d, state: &RangeState, width: f64, height: f64) {
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
        ctx.set_global_alpha(casing.clock.fade());
        ctx.set_fill_style_str(CASING_COLOR);
        ctx.fill_rect(-3.0, -7.0, 6.0, 14.0);
        ctx.set_global_alpha(1.0);
        ctx.restore();
    }
}
