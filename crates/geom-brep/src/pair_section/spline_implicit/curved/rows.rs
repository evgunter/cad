//! **A spline wall against a cylinder, sphere or torus: the
//! classification's rows.** Each pair is built where its answer is
//! known by construction (an exact rational sphere patch, a wall whose
//! side the partner's trace touches, a dome whose top the partner
//! swallows), read through the value's door with the march door, and
//! checked for the verdict the stage that owns it gives. Every row names
//! what turns it red; each runs at the run's ε (the suite runs 1e-9,
//! 1e-6 and 1e-12), its offsets stated in the band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::intersect::SectionError;
use crate::pair_section::{ChartReach, Components, SectionRead, SsiLane, pair_section};
use crate::ssi::{ChartAxis, ChartEnd, ChartSide};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

fn wall(n: NurbsSurface<f64>) -> Surface<f64> {
    Surface::Nurbs(std::sync::Arc::new(n))
}

fn sphere(center: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The cylinder of `radius` about the `z`-parallel line through `origin`.
fn cylinder(origin: Point3<f64>, radius: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        radius,
    }
}

/// The ring torus about the `z` axis through the origin.
fn torus(major: f64, minor: f64) -> Surface<f64> {
    Surface::Torus {
        center: pt(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
        major_radius: major,
        minor_radius: minor,
    }
}

/// A clamped single-span knot vector of degree `p` over `[0, 1]`.
fn span(p: usize) -> KnotVector {
    let mut k = vec![0.0; p + 1];
    k.extend(vec![1.0; p + 1]);
    KnotVector::clamped(k, p).unwrap()
}

/// The single-span wall of degrees `(p, q)` whose control point and
/// weight `(i, j)` are `c(i, j)`.
fn patch(p: usize, q: usize, c: &dyn Fn(usize, usize) -> (Point3<f64>, f64)) -> Surface<f64> {
    let (control, weights): (Vec<_>, Vec<_>) = (0..=p)
        .flat_map(|i| (0..=q).map(move |j| (i, j)))
        .map(|(i, j)| c(i, j))
        .unzip();
    wall(NurbsSurface::new(span(p), span(q), control, weights).unwrap())
}

/// The bilinear wall with corners `c[i][j]` at `(u, v) = (i, j)`.
fn flat(c: [[Point3<f64>; 2]; 2]) -> Surface<f64> {
    patch(1, 1, &|i, j| (c[i][j], 1.0))
}

/// **The zone of the sphere of `radius` about the origin** between the
/// latitudes `lat` and the longitudes `lon` (radians, each span under a
/// right angle): the exact rational biquadratic, a meridian arc in `u`
/// revolved through `lon` in `v`, with no collapsed side.
fn zone(radius: f64, lat: (f64, f64), lon: (f64, f64)) -> Surface<f64> {
    let arc = |(a, b): (f64, f64)| {
        let h = 0.5 * (b - a);
        let m = 0.5 * (a + b);
        [(a, 1.0, 1.0), (m, 1.0 / h.cos(), h.cos()), (b, 1.0, 1.0)]
    };
    let (meridian, parallel) = (arc(lat), arc(lon));
    patch(2, 2, &|i, j| {
        let (phi, stretch_m, w_m) = meridian[i];
        let (theta, stretch_p, w_p) = parallel[j];
        let rho = radius * stretch_m * phi.cos() * stretch_p;
        (
            pt(
                rho * theta.cos(),
                rho * theta.sin(),
                radius * stretch_m * phi.sin(),
            ),
            w_m * w_p,
        )
    })
}

/// The sphere zone the coincidence rows read.
fn the_zone(radius: f64) -> Surface<f64> {
    let d = std::f64::consts::PI / 180.0;
    zone(radius, (-30.0 * d, 30.0 * d), (0.0, 60.0 * d))
}

fn side(fixed: ChartAxis, end: ChartEnd) -> ChartReach {
    ChartReach::side(ChartSide { fixed, end })
}

/// The section of `a` and `b` through the march door.
fn read(a: &Surface<f64>, b: &Surface<f64>) -> Result<Components<f64>, SectionError> {
    match pair_section(a, b, None, Some(SsiLane::marched()), band())? {
        SectionRead::Built(s) => Ok(s.components().clone()),
        SectionRead::Elsewhere(route) => panic!("the value builds the pair: {route:?}"),
    }
}

/// **An exact sphere patch on its sphere lies on it.** The hull's
/// ceiling reads the composite `|S|² − r²` within the zero band over
/// every cell: [`SectionError::CoincidentSurfaces`] (D10), in either
/// order. Red against the hull skipped, and against the coincidence read
/// on a floor rather than a ceiling where the ceiling's divisor bound is
/// dropped.
#[test]
fn an_exact_sphere_patch_lies_on_its_sphere() {
    let s = sphere(pt(0.0, 0.0, 0.0), 1.0);
    let z = the_zone(1.0);
    for (a, b) in [(&s, &z), (&z, &s)] {
        let got = read(a, b);
        assert!(
            matches!(got, Err(SectionError::CoincidentSurfaces)),
            "an exact sphere patch reads {got:?}"
        );
    }
}

/// **The same patch off its sphere past the band is apart**: the hull's
/// floor reads every cell definitely inside, no component and no march.
/// Red against the floor's divisor read at its lower end (the floor then
/// overstates the distance, which is no failure here) — and against the
/// apart verdict dropped, where the pass and the sweep run and the side
/// arithmetic decides it instead (the row checks only the answer; the
/// mutant that skips the hull reds the coincidence row).
#[test]
fn a_sphere_patch_past_the_band_is_apart() {
    let off = 4.0 * band().escalate();
    let got = read(&sphere(pt(0.0, 0.0, 0.0), 1.0 + off), &the_zone(1.0));
    assert!(
        matches!(&got, Ok(Components::Counted(c)) if c.is_empty()),
        "a patch {off:e} inside its sphere reads {got:?}"
    );
}

/// **A wall within the band of its sphere everywhere escalates at the
/// hull**: the zone of radius `r + δ`, `δ` between the band's zero and
/// its escalation, against the sphere of radius `r`. Every cell of the
/// rectangle can read regular to the sweep, and no side crosses, so a
/// classification without its hull reads it loop-free with no
/// component. Red against the hull skipped.
#[test]
fn a_wall_in_the_band_everywhere_escalates_at_the_hull() {
    let b = band();
    let off = 0.5 * (b.zero() + b.escalate());
    let got = read(&sphere(pt(0.0, 0.0, 0.0), 1.0), &the_zone(1.0 + off));
    assert!(
        matches!(got, Err(SectionError::Escalated(_))),
        "a patch {off:e} off its sphere reads {got:?}"
    );
}

/// **A side the partner's trace touches refuses at the pass, before the
/// sweep.** The unit square in `z = 0` against the cylinder about the
/// vertical line through `(½, −0.3)` of radius `0.3`: their section in
/// the plane is the circle that touches the square's `v = 0` side at
/// `(½, 0)` from outside. The sweep reads every cell regular or
/// excluded (the circle's normal at the touch is across the side), so a
/// sweep run ahead of the pass clears the pose with no end; the pass
/// refuses the touch as a graze, a pinch. Red against the sweep run
/// before the pass (the W5 class).
#[test]
fn a_touched_side_refuses_at_the_pass_before_the_sweep() {
    let square = flat([
        [pt(0.0, 0.0, 0.0), pt(0.0, 1.0, 0.0)],
        [pt(1.0, 0.0, 0.0), pt(1.0, 1.0, 0.0)],
    ]);
    let got = read(&cylinder(pt(0.5, -0.3, 0.0), 0.3), &square);
    assert!(
        matches!(got, Err(SectionError::Pinched { .. })),
        "a cylinder touching the wall's side reads {got:?}"
    );
}

/// **A loop round an excluded top declines.** The dome
/// `z = −(x − ½)² − (y − ½)²` against the unit sphere centred `0.95`
/// above its top: the top lies inside the sphere, which cuts the dome
/// in a closed loop about it. The top's cells are excluded inside the
/// sphere but joined to no side through cells excluded inside, so the
/// sweep declines, and a curved partner's declined pose is the general
/// rung's. Red against the same-side join dropped (the sweep then reads
/// it loop-free, with no end: no component).
#[test]
fn a_loop_round_an_excluded_top_declines() {
    let g = [-0.25, 0.25, -0.25];
    #[allow(clippy::cast_precision_loss)]
    let dome = patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, g[i] + g[j]), 1.0)
    });
    let got = read(&dome, &sphere(pt(0.5, 0.5, 0.95), 1.0));
    assert!(
        matches!(got, Err(SectionError::RoutesToGeneralRung { .. })),
        "a sphere swallowing the dome's top reads {got:?}"
    );
}

/// The fan whose `u = 0` side collapses to `apex`, its `u = 1` side the
/// segment from `a` to `b`.
fn fan(apex: Point3<f64>, a: Point3<f64>, b: Point3<f64>) -> Surface<f64> {
    flat([[apex, apex], [a, b]])
}

/// **A collapsed side on the partner refuses at the door; one clear of
/// it passes.** A fan whose apex lies on the unit sphere and whose far
/// side lies outside it refuses naming the collapsed side; the same fan
/// with its apex at the centre passes the door to the pass and the
/// sweep. Red against the door skipped (the on-partner fan then reads
/// as the pass decides it), and against the door refusing a pole clear
/// of the partner.
#[test]
fn a_collapsed_side_on_the_partner_refuses_at_the_door() {
    let s = sphere(pt(0.0, 0.0, 0.0), 1.0);
    let on = fan(pt(1.0, 0.0, 0.0), pt(2.0, -0.5, 0.0), pt(2.0, 0.5, 0.0));
    let got = read(&on, &s);
    assert!(
        matches!(&got, Err(SectionError::Unaccounted { cause }) if cause.contains("collapses")),
        "a fan whose apex lies on the sphere reads {got:?}"
    );
    let clear = fan(pt(0.0, 0.0, 0.0), pt(2.0, -0.5, 0.0), pt(2.0, 0.5, 0.0));
    let got = read(&clear, &s);
    assert!(
        !matches!(&got, Err(SectionError::Unaccounted { cause }) if cause.contains("collapses")),
        "a fan whose apex is the sphere's centre reads {got:?}"
    );
}

/// **A spout through a belly is loop-free, ending on the two sides it
/// crosses.** A quarter tube of radius `0.2` about the `z` axis, rising
/// from `z = 0.5` to `1.5` (its `u` the quarter circle, its `v` the
/// height), against the unit sphere: `φ = 0.04 + z² − 1` along the
/// wall, whose gradient never vanishes, so the section is the arc at
/// `z = √0.96` from the `u = 0` side to the `u = 1` side.
/// [`Components::Reaching`] names those two sides on the wall and none
/// on the sphere, in the order the pair was asked. Red against the reach
/// read off the wrong operand, and against the pass's crossings dropped.
#[test]
fn a_spout_through_a_belly_reaches_the_sides_it_crosses() {
    let w = std::f64::consts::FRAC_1_SQRT_2;
    let ring = [(0.2, 0.0, 1.0), (0.2, 0.2, w), (0.0, 0.2, 1.0)];
    let spout = patch(2, 1, &|i, j| {
        let (x, y, wt) = ring[i];
        (pt(x, y, if j == 0 { 0.5 } else { 1.5 }), wt)
    });
    let belly = sphere(pt(0.0, 0.0, 0.0), 1.0);
    let ends = side(ChartAxis::U, ChartEnd::Low).or(side(ChartAxis::U, ChartEnd::High));
    for (a, b, on_wall) in [(&belly, &spout, 1), (&spout, &belly, 0)] {
        let got = read(a, b);
        let Ok(Components::Reaching(reached)) = &got else {
            panic!("the spout through the belly reads {got:?}");
        };
        let rims = reached.rims();
        assert_eq!(rims[on_wall], ends, "the wall's ends: {rims:?}");
        assert!(
            !rims[1 - on_wall].any(),
            "the sphere has no sides: {rims:?}"
        );
    }
}

/// **A cylinder cutting a wall's side twice is loop-free on that side.**
/// The unit square in `z = 0` against the vertical cylinder about
/// `(½, −0.1)` of radius `0.3`: their section is an arc of the circle
/// leaving and re-entering the `v = 0` side. Red against the pass's
/// crossings dropped.
#[test]
fn a_cylinder_cutting_one_side_twice_reaches_it() {
    let square = flat([
        [pt(0.0, 0.0, 0.0), pt(0.0, 1.0, 0.0)],
        [pt(1.0, 0.0, 0.0), pt(1.0, 1.0, 0.0)],
    ]);
    let got = read(&square, &cylinder(pt(0.5, -0.1, 0.0), 0.3));
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("the cylinder across the side reads {got:?}");
    };
    assert_eq!(
        reached.rims()[0],
        side(ChartAxis::V, ChartEnd::Low),
        "the wall's ends"
    );
}

/// **A leaf through a stem is loop-free.** A gently bowed wall standing
/// across the ring torus (`R = 1`, `r = 0.2`) in `y ≈ 0`, from `x = 0.7`
/// to `1.3` and `z = 0.05` to `0.6`: it cuts the tube in an arc leaving
/// and re-entering its `v = 0` side above the core, where the inside of
/// the tube reaches that side, so the sweep proves no loop. Red against
/// the pass's crossings dropped.
#[test]
fn a_leaf_through_a_stem_is_loop_free() {
    let xs = [0.7, 1.0, 1.3];
    let leaf = patch(2, 1, &|i, j| {
        let bow = if i == 1 { 0.05 } else { 0.0 };
        (pt(xs[i], bow, if j == 0 { 0.05 } else { 0.6 }), 1.0)
    });
    let got = read(&torus(1.0, 0.2), &leaf);
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("the leaf through the stem reads {got:?}");
    };
    assert_eq!(
        reached.rims()[1],
        side(ChartAxis::V, ChartEnd::Low),
        "the leaf's ends"
    );
}

/// **With no march door a curved partner is R-lane**: its hull is
/// formed in certification arithmetic through the door, so the value
/// asked with none refuses [`SectionError::NoMarch`], whatever the
/// pose. Red against a curved partner read at the scalar with no lane.
#[test]
fn a_curved_partner_with_no_march_door_is_r_lane() {
    let got = pair_section(
        &sphere(pt(0.0, 0.0, 0.0), 1.0),
        &the_zone(1.0),
        None,
        None,
        band(),
    );
    assert!(matches!(got, Err(SectionError::NoMarch { .. })), "{got:?}");
}
