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

/// Whether a reading refused at the whole reading's accounting cell
/// budget, the cause the value carries being that refusal's own text: a
/// gap to the partner the exclusion's zero test cannot resolve within
/// the budget, and no tube holds.
fn unaccounted_at_the_budget(got: &Result<Components<f64>, SectionError>) -> bool {
    let budget = crate::ssi::SsiError::CellBudget {
        budget: crate::ssi::exhaust::SSI_MAX_CELLS,
    }
    .to_string();
    matches!(got, Err(SectionError::Unaccounted { cause }) if *cause == budget)
}

/// **An exact sphere patch on its sphere lies on it.** The hull's
/// ceiling reads the composite `|S|² − r²` within the zero band over
/// every cell: [`SectionError::CoincidentSurfaces`] (D10), in either
/// order. Red against the hull skipped.
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
/// Red against the hull skipped: the pass and the sweep then read a wall
/// with no slope off its partner, cell by cell, and spend the cell
/// budget.
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

/// **A loop round an excluded top is read whole and listed.** The dome
/// `z = −(x − ½)² − (y − ½)²` against the unit sphere centred `0.95`
/// above its top: the top lies inside the sphere, which cuts the dome
/// in a closed loop about it. The top's cells are excluded inside the
/// sphere but joined to no side through cells excluded inside, so the
/// sweep declines, and the pose is read whole on the 1×2 trace: one
/// loop, its witness on the dome within its pad of the sphere. Red
/// against the same-side join dropped (the sweep then reads it
/// loop-free, with no end: no component), and against a declined curved
/// pose left to the general rung.
#[test]
fn a_loop_round_an_excluded_top_is_listed() {
    let g = [-0.25, 0.25, -0.25];
    #[allow(clippy::cast_precision_loss)]
    let dome = patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, g[i] + g[j]), 1.0)
    });
    let ball = sphere(pt(0.5, 0.5, 0.95), 1.0);
    let got = read(&dome, &ball);
    let Ok(Components::Counted(listed)) = &got else {
        panic!("a sphere swallowing the dome's top reads {got:?}");
    };
    let [loop_] = listed.as_slice() else {
        panic!("one component: {listed:?}");
    };
    assert!(
        matches!(loop_.shape, crate::pair_section::ComponentShape::Loop),
        "a loop: {loop_:?}"
    );
    let q = loop_.witness.expect("the loop's witness");
    let off = crate::implicit_residual(&ball, q).abs();
    assert!(
        off <= loop_.pad + band().zero(),
        "the witness {off:e} m off the sphere, its pad {:e}",
        loop_.pad
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
/// with its apex at the centre passes the door, and the unit circle
/// runs across it from one ruled side to the other. Red against the door
/// skipped (the on-partner fan then reads as the pass decides it), and
/// against the door refusing a pole clear of the partner.
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
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("a fan whose apex is the sphere's centre reads {got:?}");
    };
    assert_eq!(
        reached.rims()[0],
        side(ChartAxis::V, ChartEnd::Low).or(side(ChartAxis::V, ChartEnd::High)),
        "the unit circle runs across the fan from one ruled side to the other"
    );
}

/// **A spout through a belly is loop-free, ending on the two sides it
/// crosses.** A quarter tube of radius `0.2` about the `z` axis, rising
/// from `z = 0.5` to `1.5` (its `u` the quarter circle, its `v` the
/// height), against the unit sphere: `φ = 0.04 + z² − 1` along the
/// wall, whose gradient never vanishes, so the section is the arc at
/// `z = √0.96` from the `u = 0` side to the `u = 1` side.
/// [`Components::Reaching`] names those two sides on the wall and none
/// on the sphere, in the order the pair was asked. Red against the pass's
/// crossings dropped.
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
///
/// **And at every scale**: the pair scaled by `10⁻²` and `10⁻³` reads the
/// same. The form's gradient scales as the cube of the length on a torus,
/// so a slope that is not over `‖∇f‖` reads a hundredth or a millionth of
/// itself there: red against the sweep's regular slope and the side
/// roots' slope each read without `sup‖∇f‖`.
#[test]
fn a_leaf_through_a_stem_is_loop_free() {
    for scale in [1.0, 1e-2, 1e-3] {
        let xs = [0.7, 1.0, 1.3];
        let leaf = patch(2, 1, &|i, j| {
            let bow = if i == 1 { 0.05 } else { 0.0 };
            let z = if j == 0 { 0.05 } else { 0.6 };
            (pt(scale * xs[i], scale * bow, scale * z), 1.0)
        });
        let got = read(&torus(scale, 0.2 * scale), &leaf);
        let Ok(Components::Reaching(reached)) = &got else {
            panic!("the leaf through the stem at scale {scale:e} reads {got:?}");
        };
        assert_eq!(
            reached.rims()[1],
            side(ChartAxis::V, ChartEnd::Low),
            "the leaf's ends at scale {scale:e}"
        );
    }
}

/// **With no march door a curved partner is R-lane**: its hull is
/// formed in certification arithmetic through the door, so the value
/// asked with none refuses [`SectionError::NoMarch`], whatever the
/// pose.
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

/// The wall standing on the unit sphere's equator along its `v = 0`
/// side, a quarter arc in `u`, rising to `z = 1` in `v` while it leaves
/// the sphere radially by `slope·v + 0.1·v²`: its slope off the sphere
/// across that side is `slope` per metre risen.
fn standing(slope: f64) -> Surface<f64> {
    let w = std::f64::consts::FRAC_1_SQRT_2;
    let arc = [(1.0, 0.0, 1.0), (1.0, 1.0, w), (0.0, 1.0, 1.0)];
    let g = [1.0, 1.0 + slope / 2.0, 1.0 + slope + 0.1];
    let z = [0.0, 0.5, 1.0];
    patch(2, 2, &|i, j| {
        let (x, y, wt) = arc[i];
        (pt(x * g[j], y * g[j], z[j]), wt)
    })
}

/// **A side lying on the sphere, the wall leaving it steeply, is a
/// `Side` region**: the pass reads the slope across the side one-signed
/// over a strip beside it, and the sweep the rest regular, so the
/// section reaches the wall's `v = 0` side and no other. Red against the
/// curved partial read over a whole strip at once, where `∇f`'s box and
/// the derivative's part nothing of their correlation, no slope across
/// reads one-signed, and the side falls to its roots, every cell of
/// which holds a zero (the cell budget).
#[test]
fn a_side_lying_on_the_sphere_is_a_side_region() {
    let got = read(&sphere(pt(0.0, 0.0, 0.0), 1.0), &standing(0.1));
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("the wall standing on the equator reads {got:?}");
    };
    assert_eq!(
        reached.rims()[1],
        side(ChartAxis::V, ChartEnd::Low),
        "the wall's ends"
    );
}

/// **A wall of revolution about the `z` axis**: the meridian `m(i)`, its
/// `(ρ, z, w)` control points over the knot vector `ku`, revolved through
/// the longitudes `lon` (under a right angle) as one rational quadratic
/// span in `v`.
fn revolved(ku: KnotVector, meridian: &[(f64, f64, f64)], lon: (f64, f64)) -> Surface<f64> {
    let h = 0.5 * (lon.1 - lon.0);
    let parallel = [
        (lon.0, 1.0, 1.0),
        (lon.0 + h, 1.0 / h.cos(), h.cos()),
        (lon.1, 1.0, 1.0),
    ];
    let (control, weights): (Vec<_>, Vec<_>) = meridian
        .iter()
        .flat_map(|&(rho, z, w)| {
            parallel.iter().map(move |&(theta, stretch, wp)| {
                (
                    pt(rho * stretch * theta.cos(), rho * stretch * theta.sin(), z),
                    w * wp,
                )
            })
        })
        .unzip();
    wall(NurbsSurface::new(ku, span(2), control, weights).unwrap())
}

/// **A circle's arc from `a` to `b` cut into `n` exact rational quadratic
/// spans**, about `(c0, c1)` of radius `r`: its control points as
/// `(first, second, w)` and its knot vector, each interior knot doubled.
fn arcs(
    n: usize,
    (a, b): (f64, f64),
    (c0, c1): (f64, f64),
    r: f64,
) -> (KnotVector, Vec<(f64, f64, f64)>) {
    #[allow(clippy::cast_precision_loss)]
    let step = (b - a) / n as f64;
    let half = 0.5 * step;
    let mut control = Vec::with_capacity(2 * n + 1);
    for k in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let t = a + step * k as f64;
        control.push((c0 + r * t.cos(), c1 + r * t.sin(), 1.0));
        let m = t + half;
        control.push((
            c0 + r / half.cos() * m.cos(),
            c1 + r / half.cos() * m.sin(),
            half.cos(),
        ));
    }
    control.push((c0 + r * b.cos(), c1 + r * b.sin(), 1.0));
    let mut knots = vec![0.0; 3];
    for k in 1..n {
        #[allow(clippy::cast_precision_loss)]
        let x = k as f64 / n as f64;
        knots.extend([x, x]);
    }
    knots.extend([1.0; 3]);
    (KnotVector::clamped(knots, 2).unwrap(), control)
}

/// **A sphere zone half the band off a sphere of radius 100 lies on it.**
/// The composite there is `|S|² − R² ≈ 2R·δ`, a hundred times the
/// band at `δ = ε/2`, and only its reading in metres, through the
/// divisor `|p| + R ≈ 200`, is within the band. Red against the hull
/// deciding apart on the raw form (it reads the wall a hundred bands
/// inside the sphere, disjoint).
#[test]
fn a_zone_half_the_band_off_a_large_sphere_lies_on_it() {
    let off = 0.5 * band().zero();
    let got = read(&sphere(pt(0.0, 0.0, 0.0), 100.0), &the_zone(100.0 + off));
    assert!(
        matches!(got, Err(SectionError::CoincidentSurfaces)),
        "a zone {off:e} off a sphere of radius 100 reads {got:?}"
    );
}

/// The exact wall of the ring torus `(major, minor)` about the `z` axis:
/// its whole meridian circle in `n` spans, revolved through 60°.
fn torus_wall(n: usize, major: f64, minor: f64) -> Surface<f64> {
    let (ku, meridian) = arcs(n, (0.0, std::f64::consts::TAU), (major, 0.0), minor);
    revolved(ku, &meridian, (0.0, std::f64::consts::FRAC_PI_3))
}

/// **A torus wall up to three bands off its torus is not on it, by the
/// hull.** The wall of `(R + δ, r − δ)` against the torus `(R, r)`,
/// `R = 1`, `r = 0.9`, `δ = 1.5ε`, its meridian in eight spans: it meets
/// the torus on the outer equator and stands `2δ` off it on the inner,
/// where the form's divisor is a fifth of the outer's. The largest `|f|`
/// read over the net's largest divisor is under the band's zero, so a
/// coincidence read on the floor reads the wall on the torus; read on
/// the ceiling it is not. The row reads stage 1 alone: past a hull that
/// does not decide it, the pass and the sweep over a wall this near its
/// partner spend the cell budget. Red against the coincidence read on
/// the floor, and against the ceiling divided by the floor's divisor.
#[test]
fn a_torus_wall_three_bands_off_its_torus_is_not_on_it() {
    let d = 1.5 * band().zero();
    let curved = crate::ssi::residual::ImplicitForm::of(&torus(1.0, 0.9)).unwrap();
    let Surface::Nurbs(net) = torus_wall(8, 1.0 + d, 0.9 - d) else {
        unreachable!("a NURBS wall")
    };
    let got = super::hull(&curved, &net, band());
    assert!(
        !matches!(got, Err(SectionError::CoincidentSurfaces)),
        "a wall up to {:e} off the torus reads {got:?}",
        2.0 * d
    );
}

/// **A refined quarter cylinder just past the band's zero escalates.**
/// Sixteen exact arcs of radius `1 + δ`, `δ = 1.05ε`, ruled up the `z`
/// axis, against the unit cylinder: the net's hull is a square box whose
/// corner stands `√2` out, so the divisor's upper bound there makes a
/// floor of `0.87ε`, decided zero, where the ceiling reads `1.05ε`, in
/// band. Red against the coincidence read on the floor.
#[test]
fn a_quarter_cylinder_just_past_the_zero_escalates() {
    let d = 1.05 * band().zero();
    let (ku, ring) = arcs(16, (0.0, std::f64::consts::FRAC_PI_2), (0.0, 0.0), 1.0 + d);
    let (control, weights): (Vec<_>, Vec<_>) = ring
        .iter()
        .flat_map(|&(x, y, w)| [(pt(x, y, 0.0), w), (pt(x, y, 1.0), w)])
        .unzip();
    let quarter = wall(NurbsSurface::new(ku, span(1), control, weights).unwrap());
    let got = read(&cylinder(pt(0.0, 0.0, 0.0), 1.0), &quarter);
    assert!(
        matches!(got, Err(SectionError::Escalated(_))),
        "a quarter cylinder {d:e} off reads {got:?}"
    );
}

/// **A wall leaving a large cylinder along a ruling at a slope in the
/// band is held tangent.** A flat wall whose `v = 0` side is a ruling of
/// the cylinder of radius 100, leaving it at `θ = 0.3ε` per metre: the
/// strip's slope across the side, `∇f·S_v` over `sup‖∇f‖·sup‖S_v‖`, is a
/// sine of about `θ`, levered by the feature extent well within the
/// band, and the pass refuses the strip as the pinch. `‖∇f‖ = 2R = 200`
/// on the cylinder, so a slope not over it reads two hundred times the
/// sine, past the band at every ε. Monotone in `R`. Red against the
/// strip's sine read without `sup‖∇f‖`.
#[test]
fn a_wall_leaving_a_large_cylinder_in_the_band_is_held_tangent() {
    let theta = 0.3 * band().zero();
    let away = pt(100.0 + theta.sin(), theta.cos(), 0.0);
    let w = flat([
        [pt(100.0, 0.0, 0.0), away],
        [pt(100.0, 0.0, 1.0), pt(away.x, away.y, 1.0)],
    ]);
    let got = read(&cylinder(pt(0.0, 0.0, 0.0), 100.0), &w);
    assert!(
        matches!(
            got,
            Err(SectionError::Pinched {
                predicate: "ssi_boundary_strip"
            })
        ),
        "a wall leaving the cylinder at {theta:e} reads {got:?}"
    );
}

/// **A touch with a large sphere at the wall's side is no exclusion.**
/// The half dome `z = −(x − ½)² − y²` over the unit square, its top on
/// the `y = 0` side, under the sphere of radius 100 whose lowest point
/// stands `4ε` above that top. The side never meets the sphere, so the
/// pass finds no end. The top is a critical point of `φ` within the band:
/// its cells at the accounting floor read in band in metres, neither
/// regular nor excluded, and the sweep declines there. The raw form there
/// is `2R·4ε`, eight hundred bands. A sweep excluding on the raw form
/// reads the top's cells excluded, and since they lie on the rectangle's
/// side, they join it and the pose clears with no component.
///
/// The touch stands on the side because, inside the rectangle, the
/// excluded top would be ringed by cells read regular, which the join
/// does not walk (SPLICE's
/// `the-plane-spline-sweep-declines-a-loop-free-dome-beside-a-saddle`), so
/// the pose would decline either way. Read whole, the section is empty,
/// but the top stands only `4ε` off the sphere: the accounting's zero
/// test, on boxes of the wall that close on the top only as fast as
/// their cells shrink, cannot clear the cells about it before its cell
/// budget runs out, and no tube holds them, so it refuses there,
/// unaccounted. Red against the sweep's exclusion read on the raw form
/// rather than in metres.
#[test]
fn a_touch_with_a_large_sphere_at_a_side_is_no_exclusion() {
    let (gx, gy) = ([-0.25, 0.25, -0.25], [0.0, 0.0, -1.0]);
    #[allow(clippy::cast_precision_loss)]
    let half_dome = patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, gx[i] + gy[j]), 1.0)
    });
    let above = 100.0 + 4.0 * band().zero();
    let got = read(&half_dome, &sphere(pt(0.5, 0.0, above), 100.0));
    assert!(
        unaccounted_at_the_budget(&got),
        "a half dome touching a large sphere within the band reads {got:?}"
    );
}

/// **A torus corner in the band is no clear corner.** At the point of the
/// torus `(1, 0.9)` at meridian angle 120°, `0.85` of the band's
/// escalation inside it, the pass's corner reading is a floor on the
/// distance, which reads in band, so the corner is classified; the
/// form's ceiling there over-reads the distance by about `1.38`, past
/// the escalation, which would skip the corner as clear. Red against the
/// corner read on a ceiling.
#[test]
fn a_torus_corner_in_the_band_is_no_clear_corner() {
    use crate::ssi::boundary::Partner;
    use geom_core::Sign;
    use geom_core::k_stats::decide;
    let b = band();
    let curved = crate::ssi::residual::ImplicitForm::of(&torus(1.0, 0.9)).unwrap();
    let a = 2.0 * std::f64::consts::FRAC_PI_3;
    let rho = 0.9 - 0.85 * b.escalate();
    let p = pt(1.0 + rho * a.cos(), 0.0, rho * a.sin());
    let (_, metres) = (&curved).at_point(p);
    let read = decide("ssi_boundary_corner", geom_core::Margin::of(metres), b);
    assert!(
        !matches!(read, Ok(Sign::Positive)),
        "a corner {:e} inside the torus reads {metres:e}: {read:?}",
        0.85 * b.escalate()
    );
}

/// The section of `a` and `b` through the march door, whole.
fn section(a: &Surface<f64>, b: &Surface<f64>) -> Result<crate::PairSection<f64>, SectionError> {
    match pair_section(a, b, None, Some(SsiLane::marched()), band())? {
        SectionRead::Built(s) => Ok(s),
        SectionRead::Elsewhere(route) => panic!("the value builds the pair: {route:?}"),
    }
}

/// The dome `z = −(x − ½)² − (y − ½)²` over the unit square, its top
/// `z = 0` at the centre.
fn dome() -> Surface<f64> {
    let g = [-0.25, 0.25, -0.25];
    #[allow(clippy::cast_precision_loss)]
    patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, g[i] + g[j]), 1.0)
    })
}

/// **A dome touching a sphere at its top is one touch.** The unit sphere
/// standing on the dome's top: the top is `∇φ`'s one zero there, its
/// Hessian definite and its value zero, so the sweep decides a touch and
/// the pass finds no end. The section is that touch, one point
/// component witnessed at the top, a tangency decided Zero. Red against
/// the Hessian's sign read the other way (a node pinches), and against a
/// decided touch left to the general rung.
#[test]
fn a_dome_touching_a_sphere_is_one_touch() {
    let got = section(&dome(), &sphere(pt(0.5, 0.5, 1.0), 1.0));
    let Ok(s) = &got else {
        panic!("the dome under the sphere reads {got:?}");
    };
    let Components::Counted(c) = s.components() else {
        panic!("a counted touch: {:?}", s.components());
    };
    assert!(
        matches!(c.as_slice(), [p] if p.shape == crate::ComponentShape::Point
            && p.witness.is_some_and(|w| w.distance(pt(0.5, 0.5, 0.0)) <= 1e-6)),
        "one touch at the top: {c:?}"
    );
    assert!(
        s.tangency()
            .is_some_and(|z| z.decision.name() == "spline_critical_value"),
        "the touch is a tangency decided Zero: {:?}",
        s.tangency()
    );
}

/// **A top only in band of a sphere is not decided.** The same dome
/// under the unit sphere raised half way between the band's zero and its
/// escalation: the top is a critical point whose value is in band, so it
/// is no touch, and the sweep declines there. Read whole, the section is
/// empty, but the top stands within the band of the sphere: the
/// accounting's zero test cannot clear the cells about it before its
/// cell budget runs out, and no tube holds them, so it refuses there,
/// unaccounted. Red against the
/// value read past the band's zero rather than decided zero (it would
/// read a touch).
#[test]
fn a_top_in_band_of_a_sphere_is_not_decided() {
    let b = band();
    let lift = 0.5 * (b.zero() + b.escalate());
    let got = read(&dome(), &sphere(pt(0.5, 0.5, 1.0 + lift), 1.0));
    assert!(
        unaccounted_at_the_budget(&got),
        "a top {lift:e} under the sphere reads {got:?}"
    );
}

/// **A saddle at the zero level of a sphere pinches its arcs.** The
/// saddle `z = (x − ½)² − (y − ½)²` under the sphere of radius 100 resting
/// on its centre: the centre is a node decided Zero, the section's arcs
/// crossing there and running out to the sides, and the march that would
/// list them does not start at a node, so the value refuses them as the
/// pinch named by the decision. Red against a node's arcs read as the
/// pass's reach (`Reaching`, which the guard would clear).
#[test]
fn a_saddle_under_a_sphere_pinches_at_its_node() {
    let g = [0.25, -0.25, 0.25];
    #[allow(clippy::cast_precision_loss)]
    let saddle = patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, g[i] - g[j]), 1.0)
    });
    let got = read(&saddle, &sphere(pt(0.5, 0.5, 100.0), 100.0));
    assert!(
        matches!(
            got,
            Err(SectionError::Pinched {
                predicate: "spline_critical_value"
            })
        ),
        "the saddle under the sphere reads {got:?}"
    );
}

/// **Klein's half-tube: a creased wall through a sphere is loop-free.**
/// A half tube of radius `0.2` about the `z` axis, rising from `z = 0.5`
/// to `1.5`, its half circle two rational arcs of 60° and 120° on the two
/// halves of `u`, so a knot of multiplicity the degree sits at `u = ½`
/// and the wall's `u` derivative jumps there (a crease by its knots, as
/// klein's half-tube's arc joint is). Against the unit sphere `φ`
/// depends on the height alone; the crease pass reads the crease line
/// regular along itself, each half of the rectangle is swept, and the
/// section is the arc from the `u = 0` side to the `u = 1` side. Red
/// against the rectangle swept whole (it declines at the crease).
#[test]
fn klein_half_tube_through_a_sphere_is_loop_free() {
    let arc = |a: f64, b: f64| {
        let h = 0.5 * (b - a);
        [
            (0.2 * a.cos(), 0.2 * a.sin(), 1.0),
            (
                0.2 / h.cos() * (a + h).cos(),
                0.2 / h.cos() * (a + h).sin(),
                h.cos(),
            ),
        ]
    };
    let d = std::f64::consts::PI / 180.0;
    let mut ring: Vec<(f64, f64, f64)> = arc(0.0, 60.0 * d).to_vec();
    ring.extend(arc(60.0 * d, 180.0 * d));
    ring.push((-0.2, 0.0, 1.0));
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let (control, weights): (Vec<_>, Vec<_>) = ring
        .iter()
        .flat_map(|&(x, y, w)| [(pt(x, y, 0.5), w), (pt(x, y, 1.5), w)])
        .unzip();
    let half_tube = wall(NurbsSurface::new(ku, span(1), control, weights).unwrap());
    let got = read(&sphere(pt(0.0, 0.0, 0.0), 1.0), &half_tube);
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("the half tube through the sphere reads {got:?}");
    };
    assert_eq!(
        reached.rims()[1],
        side(ChartAxis::U, ChartEnd::Low).or(side(ChartAxis::U, ChartEnd::High)),
        "the half tube's ends"
    );
}

/// **A loop round a ridge's top declines, and its march refuses at the
/// crease.** A tent creased along `x = ½`, its ridge rising to `z = 0.3`
/// at the middle of its run, under the unit sphere sunk 0.02 below the
/// ridge's top: the sphere cuts the tent in a loop about the ridge's
/// top. On each flank `φ`'s gradient is off zero, so every cell there
/// reads regular or excluded; the top is an extremum of `φ` along the
/// crease line, which the crease pass reads excluded inside the sphere,
/// not regular along the line, and joined to no side, so the sweep
/// declines. Read whole, the loop's march meets the crease, where the
/// wall's chart turns a corner, and its step falls in the band there:
/// the march's escalation on its step. Red against the crease pass
/// skipped (the loop is proved away: no component).
#[test]
fn a_loop_round_a_ridges_top_declines() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let ridge = [0.2, 0.4, 0.2];
    let (control, weights): (Vec<_>, Vec<_>) = (0..3)
        .flat_map(|i| (0..3).map(move |j| (i, j)))
        .map(|(i, j)| {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = (0.5 * i as f64, 0.5 * j as f64);
            (pt(x, y, if i == 1 { ridge[j] } else { 0.0 }), 1.0)
        })
        .unzip();
    let tent = wall(NurbsSurface::new(ku, span(2), control, weights).unwrap());
    let got = read(&tent, &sphere(pt(0.5, 0.5, 0.3 + 1.0 - 0.02), 1.0));
    assert!(
        matches!(
            &got,
            Err(SectionError::Escalated(cause))
                if cause.predicate == Some("ssi_step_progress")
        ),
        "the sphere cutting the ridge's top reads {got:?}"
    );
}

/// **A pose the sweep decides needs no march tolerance.** The flat unit
/// plate against the sphere of radius 10⁶ centred at `(0.3 − 10⁶, ½, 0)`:
/// the sphere's equator crosses the plate as a near-straight arc from
/// its `v = 0` side to its `v = 1` side, and the sweep reads it
/// loop-free, so the section is `Reaching` those two sides, with no
/// march. The march's tolerance, read about the sphere's centre a
/// million metres off, settles no finer than `2·10⁻¹⁰` m, too coarse for
/// ε = 1e-12, and is minted only where a march runs. Red against the
/// tolerance minted with the section, before the classification's pass
/// (it refuses `SettlingUnresolvable` at ε = 1e-12, unaccounted).
#[test]
fn a_pose_the_sweep_decides_needs_no_march_tolerance() {
    let r = 1.0e6;
    let plate = flat([
        [pt(0.0, 0.0, 0.0), pt(0.0, 1.0, 0.0)],
        [pt(1.0, 0.0, 0.0), pt(1.0, 1.0, 0.0)],
    ]);
    let got = read(&plate, &sphere(pt(0.3 - r, 0.5, 0.0), r));
    let Ok(Components::Reaching(reached)) = &got else {
        panic!("a plate the sphere's equator crosses reads {got:?}");
    };
    assert_eq!(
        reached.rims()[0],
        side(ChartAxis::V, ChartEnd::Low).or(side(ChartAxis::V, ChartEnd::High)),
        "the arc's ends, on the plate (the first operand)"
    );
}

/// The saddle `z = (x − ½)² − (y − ½)²` over the unit square.
fn saddle() -> Surface<f64> {
    let g = [0.25, -0.25, 0.25];
    #[allow(clippy::cast_precision_loss)]
    patch(2, 2, &|i, j| {
        (pt(0.5 * i as f64, 0.5 * j as f64, g[i] - g[j]), 1.0)
    })
}

/// The one touch a section reads, at its witness, or the reason it is
/// not one.
fn touch(a: &Surface<f64>, b: &Surface<f64>) -> Result<Point3<f64>, String> {
    let s = section(a, b).map_err(|e| format!("{e:?}"))?;
    match s.components() {
        Components::Counted(c) => match c.as_slice() {
            [p] if p.shape == crate::ComponentShape::Point && s.tangency().is_some() => {
                p.witness.ok_or_else(|| "no witness".to_owned())
            }
            other => Err(format!("{other:?}")),
        },
        other => Err(format!("{other:?}")),
    }
}

/// **A small partner resting on a saddle's centre touches it.** The
/// saddle's curvature is `±2` along its axes; a sphere of radius `0.15`
/// resting on its centre curves `1/0.15` every way, so `φ`'s Hessian
/// there, the partner's own Hessian `∇²f` read along the saddle's
/// tangents plus `∇f` against the saddle's second partials, is definite:
/// a touch. Without the partner's `∇²f` only the saddle's curvature is
/// read, the Hessian is indefinite, and the point reads a node, which
/// pinches. The same holds for a cylinder of that radius lying along `y`,
/// and for a ring torus about the `x` axis whose tube, of that radius,
/// rests on the centre at its outer equator, its core circle of radius 1
/// standing up in the plane `x = ½`. Red against each form's
/// `frame_hessian` read as zero.
#[test]
fn a_small_partner_resting_on_a_saddle_touches_it() {
    let r = 0.15;
    let partners = [
        ("sphere", sphere(pt(0.5, 0.5, r), r)),
        (
            "cylinder",
            Surface::Cylinder {
                origin: pt(0.5, 0.0, r),
                axis: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
                radius: r,
            },
        ),
        (
            "torus",
            Surface::Torus {
                center: pt(0.5, 0.5, 1.0 + r),
                axis: Vec3::unit_x(),
                u_ref: Vec3::unit_y(),
                major_radius: 1.0,
                minor_radius: r,
            },
        ),
    ];
    for (what, partner) in partners {
        let got = touch(&saddle(), &partner);
        assert!(
            got.as_ref()
                .is_ok_and(|w| w.distance(pt(0.5, 0.5, 0.0)) <= 1e-6),
            "the {what} on the saddle reads {got:?}"
        );
    }
}
