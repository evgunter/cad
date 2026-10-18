//! C4's placement: a periodic chart's image is placed over its edge's
//! span in the chart's principal window ([`super::ANGLE_WINDOW_START`];
//! a sphere's latitude in its own, [`super::Window::Latitude`]), so where
//! a carrier's parameter starts moves no stored image.

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ANGLE_WINDOW_START, PcurveCache, chart_pcurve_over};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    }
}

/// A named arm: its chart, its carrier, and an edge's span on it.
type Arm = (&'static str, Surface<f64>, Curve3<f64>, (f64, f64));

/// A ring torus `R = 2`, `r = 0.7` about `z`, and its Villarceau circle
/// through azimuth `phi` on `family` (`±1`), started at `psi` (PR 5225's
/// second review).
fn villarceau(phi: f64, family: f64, psi: f64) -> (Surface<f64>, Curve3<f64>) {
    let (major, minor) = (2.0, 0.7);
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref,
    };
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
    let carrier = circle(
        Point3::new(0.0, 0.0, 0.0) + d * minor,
        d.cross(lean),
        major,
        d * psi.cos() + lean * psi.sin(),
    );
    (torus, carrier)
}

/// A cone of half-angle 0.4 about `z` and its section by a plane tilted
/// 0.2 rad off the axis at height 1.5: an ellipse, the focal arm's.
fn cone_ellipse() -> (Surface<f64>, Curve3<f64>) {
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle: 0.4,
        u_ref: Vec3::unit_x(),
    };
    let tilt: f64 = 0.2;
    let plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 1.5),
        normal: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    };
    let reach = crate::Reach::Measured {
        at: Point3::new(0.0, 0.0, 0.0),
        lever: 4.0,
    };
    match crate::intersect::plane_cone_section(&plane, &cone, &reach, band()) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(e)) => (cone, e),
        other => panic!("a tilted section of a cone is an ellipse: {other:?}"),
    }
}

/// Whether a chart's second coordinate is an angle, and in which window.
fn second_window(surface: &Surface<f64>) -> Option<fn(f64) -> bool> {
    match surface {
        Surface::Torus { .. } => Some(in_window),
        Surface::Sphere { .. } => Some(in_latitude),
        _ => None,
    }
}

/// One carrier per periodic arm of the derivation, with an edge's span.
fn arms() -> Vec<Arm> {
    let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
    let o = Point3::new(0.0, 0.0, 0.0);
    let cylinder = Surface::Cylinder {
        origin: o,
        axis: z,
        radius: 1.0,
        u_ref: x,
    };
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: 0.4,
        u_ref: x,
    };
    let sphere = Surface::Sphere {
        center: o,
        radius: 1.0,
        axis: z,
        u_ref: x,
    };
    let torus = Surface::Torus {
        center: o,
        axis: z,
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: x,
    };
    let lat: f64 = 0.5;
    let tube: f64 = 0.3;
    let (cone_e, ellipse) = cone_ellipse();
    let (torus_v, villarceau_circle) = villarceau(0.7, 1.0, 0.3);
    let (wall_r, wall_rr, wall_d) = (0.09375, 0.0703125, 0.0078125);
    let wall_torus = Surface::Torus {
        center: o,
        axis: y,
        major_radius: wall_r,
        minor_radius: wall_rr,
        u_ref: x,
    };
    let spiric = Curve3::Spiric {
        center: o,
        axis: y,
        u_ref: x,
        major_radius: wall_r,
        minor_radius: wall_rr,
        offset: wall_d,
    };
    let wall_t = 0.8911225078866529;
    let general = circle(o, Vec3::new(0.6_f64.sin(), 0.0, 0.6_f64.cos()), 1.0, {
        Vec3::new(0.6_f64.cos(), 0.0, -0.6_f64.sin())
    });
    vec![
        ("cone focal section", cone_e, ellipse, (0.3, 2.1)),
        ("torus Villarceau", torus_v, villarceau_circle, (0.4, 1.6)),
        ("torus spiric wall", wall_torus, spiric, (-wall_t, wall_t)),
        (
            "sphere general circle (projected)",
            sphere.clone(),
            general,
            (0.3, 0.3 + FRAC_PI_2),
        ),
        (
            "sphere meridian, below latitude −1",
            sphere.clone(),
            circle(o, -y, 1.0, x),
            (-1.4, -1.1),
        ),
        (
            "cylinder rim",
            cylinder.clone(),
            circle(Point3::new(0.0, 0.0, 0.5), z, 1.0, x),
            (0.3, 1.9),
        ),
        (
            "cylinder rim, reversed",
            cylinder,
            circle(Point3::new(0.0, 0.0, 0.5), -z, 1.0, x),
            (4.0, 5.9),
        ),
        (
            "cone rim",
            cone,
            circle(Point3::new(0.0, 0.0, 1.0), z, 0.4_f64.tan(), x),
            (2.0, 3.5),
        ),
        (
            "sphere parallel",
            sphere.clone(),
            circle(Point3::new(0.0, 0.0, lat.sin()), z, lat.cos(), x),
            (5.0, 6.2),
        ),
        ("sphere meridian", sphere, circle(o, -y, 1.0, x), (0.2, 1.0)),
        (
            "torus parallel",
            torus.clone(),
            circle(
                Point3::new(0.0, 0.0, 0.5 * tube.sin()),
                z,
                2.0 + 0.5 * tube.cos(),
                x,
            ),
            (1.0, 4.0),
        ),
        (
            "torus meridian",
            torus,
            circle(Point3::new(2.0, 0.0, 0.0), -y, 0.5, x),
            (0.1, 2.0),
        ),
    ]
}

fn in_window(u: f64) -> bool {
    ANGLE_WINDOW_START < u && u <= ANGLE_WINDOW_START + TAU
}

/// A sphere's latitude window `(−π, π]`, and within it the chart's own
/// latitudes `[−π/2, π/2]` an edge's midpoint on the sphere reads.
fn in_latitude(v: f64) -> bool {
    -PI < v && v <= PI && v.abs() <= FRAC_PI_2 + 1e-12
}

/// **A carrier restated whole periods off derives one placed image.**
/// On every periodic arm (the harmonic ones, the focal section on a cone
/// and a torus, the spiric wall and the projected image), the edge's
/// span shifted by `k ∈ {−10, −1, 1, 10}` whole turns of the carrier's
/// parameter derives an image that meets the unshifted one at every
/// point of the span, both its angular channels in their windows at the
/// span's midpoint, and certifies with the same envelope: check 4
/// re-derives the image the mint placed.
#[test]
fn a_carrier_restated_whole_periods_off_derives_one_placed_image() {
    let band = band();
    for (name, surface, carrier, (t0, t1)) in arms() {
        let derive = |k: f64| {
            let (a, b) = (t0 + k * TAU, t1 + k * TAU);
            let image = chart_pcurve_over(&carrier, a, b, &surface, band)
                .unwrap_or_else(|e| panic!("{name}, {k} turns: derives: {e:?}"));
            let cache = PcurveCache::certify(image.clone(), a, b, &carrier, &surface, band)
                .unwrap_or_else(|e| panic!("{name}, {k} turns: certifies: {e:?}"));
            (image, cache.certificate().envelope, (a, b))
        };
        let (base, envelope, _) = derive(0.0);
        for k in [-10.0, -1.0, 1.0, 10.0] {
            let (image, env, (a, b)) = derive(k);
            for s in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let (p, q) = (image.eval(a + s * (b - a)), base.eval(t0 + s * (t1 - t0)));
                assert!(
                    (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9,
                    "{name}, {k} turns, at {s} of the span: {p:?} against {q:?}"
                );
            }
            let mid = image.eval(0.5 * (a + b));
            assert!(
                in_window(mid.x),
                "{name}, {k} turns: the azimuth at the midpoint is in the window: {mid:?}"
            );
            if let Some(second) = second_window(&surface) {
                assert!(
                    second(mid.y),
                    "{name}, {k} turns: the second angle at the midpoint is in its window: \
                     {mid:?}"
                );
            }
            assert!(
                (env - envelope).abs() < 1e-12,
                "{name}, {k} turns: the envelope {env:e} against {envelope:e}"
            );
        }
    }
}

/// **The window holds an image's midpoint, wherever the edge sits.** A
/// cylinder rim's edge of a fixed length slid around the circle in
/// steps of a tenth of a turn reads its azimuth at the span's midpoint
/// in `(−1, τ − 1]` every time.
#[test]
fn every_edge_of_a_rim_is_placed_in_the_window_at_its_midpoint() {
    let band = band();
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let rim = circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_z(),
        1.0,
        Vec3::unit_x(),
    );
    for step in 0..30 {
        let a = -10.0 + 0.1 * TAU * f64::from(step);
        let image = chart_pcurve_over(&rim, a, a + 1.3, &cylinder, band).unwrap();
        let u = image.eval(a + 0.65).x;
        assert!(in_window(u), "an edge from {a}: its midpoint azimuth {u}");
    }
}

/// One line per line arm of the derivation, with an edge's span: a
/// cylinder's meridian, a cone's ruling, and a ruling whose span is
/// centred on the apex (its midpoint on the axis, so its azimuth's lever
/// is the reach of its ends, [`super::Levers`]).
fn line_arms() -> Vec<Arm> {
    let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
    let o = Point3::new(0.0, 0.0, 0.0);
    let phi: f64 = 2.3;
    let alpha: f64 = 0.4;
    let ruling = Vec3::new(
        alpha.sin() * phi.cos(),
        alpha.sin() * phi.sin(),
        alpha.cos(),
    );
    let cone = Surface::Cone {
        apex: o,
        axis: z,
        half_angle: alpha,
        u_ref: x,
    };
    vec![
        (
            "cylinder meridian",
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            Curve3::Line {
                origin: Point3::new(phi.cos(), phi.sin(), 0.0),
                dir: z,
            },
            (0.3, 2.0),
        ),
        (
            "cone ruling",
            cone.clone(),
            Curve3::Line {
                origin: o + ruling * 0.5,
                dir: ruling,
            },
            (0.2, 1.5),
        ),
        (
            "cone ruling across the apex",
            cone,
            Curve3::Line {
                origin: o,
                dir: ruling,
            },
            (-1.0, 1.0),
        ),
    ]
}

/// `carrier` started `by` further along: a circle's frame turned by `by`
/// about its axis, a line's origin moved `by` along it. The restarted
/// carrier at `t − by` is the first at `t`.
fn restarted(carrier: &Curve3<f64>, by: f64) -> Curve3<f64> {
    match *carrier {
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => circle(
            center,
            axis,
            radius,
            u_ref * by.cos() + axis.cross(u_ref) * by.sin(),
        ),
        Curve3::Line { origin, dir } => Curve3::Line {
            origin: origin + dir * by,
            dir,
        },
        _ => unreachable!("only circles and lines are restarted here"),
    }
}

/// **A carrier restarted derives one placed image** (C4: where a
/// carrier's parameter happens to start moves neither the stored image
/// nor its certificate). Every circle arm's carrier with its frame
/// turned, and every line arm's with its origin moved, by
/// `by ∈ {0.7, −2.1, π, 5.5}`, the edge's span moved back by as much,
/// derives an image that meets the first at every point of the span,
/// both its angular channels in their windows at the midpoint, and
/// certifies with the same envelope. A sphere meridian started on the
/// far half of its great circle is among them (`by = π`): it reads its
/// half at the span's midpoint, not at its `t = 0` point.
#[test]
fn a_carrier_restarted_derives_one_placed_image() {
    let band = band();
    let restartable = arms()
        .into_iter()
        .filter(|(_, _, c, _)| matches!(c, Curve3::Circle { .. }))
        .chain(line_arms());
    for (name, surface, carrier, (t0, t1)) in restartable {
        let derive = |carrier: &Curve3<f64>, (a, b): (f64, f64)| {
            let image = chart_pcurve_over(carrier, a, b, &surface, band)
                .unwrap_or_else(|e| panic!("{name}: derives over [{a}, {b}]: {e:?}"));
            let cache = PcurveCache::certify(image.clone(), a, b, carrier, &surface, band)
                .unwrap_or_else(|e| panic!("{name}: certifies over [{a}, {b}]: {e:?}"));
            (image, cache.certificate().envelope)
        };
        let (base, envelope) = derive(&carrier, (t0, t1));
        for by in [0.7, -2.1, PI, 5.5] {
            let moved = restarted(&carrier, by);
            let (a, b) = (t0 - by, t1 - by);
            let (image, env) = derive(&moved, (a, b));
            for s in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let (p, q) = (image.eval(a + s * (b - a)), base.eval(t0 + s * (t1 - t0)));
                assert!(
                    (p.x - q.x).abs() < 1e-9 && (p.y - q.y).abs() < 1e-9,
                    "{name}, restarted by {by}, at {s} of the span: {p:?} against {q:?}"
                );
            }
            let mid = image.eval(0.5 * (a + b));
            assert!(in_window(mid.x), "{name}, restarted by {by}: {mid:?}");
            if let Some(second) = second_window(&surface) {
                assert!(second(mid.y), "{name}, restarted by {by}: {mid:?}");
            }
            assert!(
                (env - envelope).abs() < 1e-12,
                "{name}, restarted by {by}: the envelope {env:e} against {envelope:e}"
            );
        }
    }
}

/// **A projected torus image names both its angles in the window.** A
/// rational arc of a torus's meridian circle (a spline carrier, so the
/// projected image) whose tube angle runs across the window's cut at
/// `τ − 1`, from `5.0` to `5.8`, is placed with its deck origin naming
/// the azimuth and the tube angle in the window at the span's midpoint
/// (`v_off`).
#[test]
fn a_projected_torus_image_names_both_its_angles_in_the_window() {
    use geom::NurbsCurve3;
    use geom_core::spline::KnotVector;
    use std::sync::Arc;
    let band = band();
    let (major, minor, phi) = (2.0_f64, 0.5_f64, 1.1_f64);
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        major_radius: major,
        minor_radius: minor,
        u_ref: Vec3::unit_x(),
    };
    let radial = Vec3::new(phi.cos(), phi.sin(), 0.0);
    let centre = Point3::new(0.0, 0.0, 0.0) + radial * major;
    let at = |v: f64| centre + (radial * v.cos() + Vec3::unit_z() * v.sin()) * minor;
    let (a, span) = (5.0_f64, 0.8_f64);
    let h = 0.5 * span;
    let peak =
        centre + (radial * (a + h).cos() + Vec3::unit_z() * (a + h).sin()) * (minor / h.cos());
    let arc = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![at(a), peak, at(a + span)],
            vec![1.0, h.cos(), 1.0],
        )
        .unwrap(),
    ));
    let image = chart_pcurve_over(&arc, 0.0, 1.0, &torus, band).unwrap();
    assert!(
        matches!(image, super::Pcurve::Projected(_)),
        "a spline images projected: {image:?}"
    );
    let mid = image.eval(0.5);
    assert!(in_window(mid.x), "the azimuth at the midpoint: {mid:?}");
    assert!(in_window(mid.y), "the tube angle at the midpoint: {mid:?}");
    assert!(
        (mid.x - phi).abs() < 1e-9 && (mid.y - (a + h - TAU)).abs() < 1e-9,
        "the midpoint names the meridian's azimuth and its tube angle a turn down: {mid:?}"
    );
}

/// **A span past every turn an `i32` counts refuses typed** (it panicked
/// as an `unreachable!`, PR 5225's delta review, g6): a cylinder rim's
/// edge at `m = 1.3493037703522018e10` sits on a mark of the window more
/// than `2³¹` turns out, and derives `PlacementTurnsExceeded`.
#[test]
fn a_span_past_every_turn_an_i32_counts_refuses_typed() {
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let rim = circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_z(),
        1.0,
        Vec3::unit_x(),
    );
    let m = 1.3493037703522018e10;
    let out = chart_pcurve_over(&rim, m - 0.5, m + 0.5, &cylinder, band());
    assert!(
        matches!(out, Err(super::PcurveCertifyError::PlacementTurnsExceeded)),
        "{out:?}"
    );
}

/// **A pole's zero-radius parallel keeps the angle it reads, at every
/// scalar.** A ball's poles carry a parallel of radius zero: its azimuth
/// moves nothing, every copy is the same point, and its placement's lever
/// (its reach from the axis, over its span and over its own turn) is zero.
/// The placement names no other copy (zero turns), and the parallel's
/// derived image is the one read, at `f64` and at `Interval` alike: its
/// azimuth `0` along the span (the parallel's `u_ref` is the chart's, and
/// a radius of zero turns no orientation), its latitude
/// `π/2`, never deciding marks at a zero lever (PR 5225's delta review, G1
/// and S2).
#[test]
fn a_poles_zero_radius_parallel_keeps_the_angle_it_reads() {
    use geom_core::{Bounds, Interval, Real};
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let pole = circle(
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        0.0,
        Vec3::unit_x(),
    );
    let place = super::Placement::over((2.0, 2.0), &pole, &sphere);
    assert_eq!(
        place.turns(super::Along::Azimuth, 7.0, band()).unwrap(),
        0,
        "f64"
    );
    let at_f64 = chart_pcurve_over(&pole, 1.0, 3.0, &sphere, band())
        .unwrap()
        .eval(2.0);
    assert_eq!((at_f64.x, at_f64.y), (0.0, FRAC_PI_2), "f64: {at_f64:?}");
    let f = Interval::from_f64;
    let (pole, sphere) = (pole.map_scalar(f), sphere.map_scalar(f));
    let place = super::Placement::over((f(2.0), f(2.0)), &pole, &sphere);
    assert_eq!(
        place.turns(super::Along::Azimuth, f(7.0), band()).unwrap(),
        0,
        "Interval"
    );
    let at = chart_pcurve_over(&pole, f(1.0), f(3.0), &sphere, band())
        .unwrap()
        .eval(f(2.0));
    assert!(
        at.x.lo() <= at_f64.x
            && at_f64.x <= at.x.hi()
            && at.y.lo() <= FRAC_PI_2
            && FRAC_PI_2 <= at.y.hi()
            && at.x.hi() - at.x.lo() < 1e-12,
        "Interval: the image is f64's, tightly: {at:?}"
    );
}

/// **A meridian whose midpoint is at its pole names one image wherever
/// its carrier starts.** A meridian edge whose span's midpoint is at a
/// pole, or `10⁻¹³` to `10⁻³` rad past it, derives the same image when its
/// carrier is started on the far half of its great circle (the frame
/// negated, the span moved by `π`). The half is read against `d̂` turned
/// to the chart's own side, so both starts read one number; where the
/// midpoint's half is in band, the half is the chart's, never the one the
/// carrier's `t = 0` point is on (PR 5225's delta review, g8). An offset
/// exactly on the band's edge (`10⁻⁸` at the witness ε) is left out: the
/// two parameterizations evaluate the midpoint `1e-16` apart, so one
/// decides and the other does not, as any decided quantity does at its
/// edge; there the two images are twins, naming the same points.
#[test]
fn a_meridian_with_its_midpoint_at_the_pole_names_one_image_wherever_it_starts() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let o = Point3::new(0.0, 0.0, 0.0);
    let near = circle(o, -Vec3::unit_y(), 1.0, Vec3::unit_x());
    let far = circle(o, -Vec3::unit_y(), 1.0, -Vec3::unit_x());
    for off in [0.0, 1e-13, 1e-10, 3e-9, 1e-6, 1e-3] {
        for pole in [FRAC_PI_2, -FRAC_PI_2] {
            let mid = pole + off;
            let (a, b) = (mid - 0.3, mid + 0.3);
            let p = chart_pcurve_over(&near, a, b, &sphere, band()).unwrap();
            let q = chart_pcurve_over(&far, a - PI, b - PI, &sphere, band()).unwrap();
            for s in [0.0, 0.5, 1.0] {
                let (u, v) = (p.eval(a + s * 0.6), q.eval(a - PI + s * 0.6));
                assert!(
                    (u.x - v.x).abs() < 1e-9 && (u.y - v.y).abs() < 1e-9,
                    "pole {pole}, off {off:e}, at {s} of the span: {u:?} against {v:?}"
                );
            }
        }
    }
}

/// **`x = 0` is no decision boundary of the azimuth read.** A direction
/// pointing down the chart (`y = −1`), read at `Sym<f64>` with its `x` a
/// parameter: the frame is decided across the window's cut line, so `x`
/// crossing zero flips nothing, and every `pcurve_chart_placement` margin
/// the `x = 0` build decides re-values at `x` ten band-widths negative to
/// the bits that build decides there, under the one name. (Under a frame
/// decided on `x`'s sign the downward direction sat on a second cut, a
/// landmark a quarter arc's radial lands on; `editor-core`'s
/// `revalue_declared_tangency` and `boss_union` are the corpus witnesses.)
#[test]
#[allow(clippy::expect_used)]
fn a_downward_direction_reads_one_expression_either_side_of_x_zero() {
    use geom_core::sym::revalue::{record, revalue};
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Real, Sym, SymBudget};

    use super::{Along, Azimuth, Levers, Placement};

    type S = Sym<f64>;
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let x_sym = ParamSymbol::new(1);
    let (x0, x1) = (0.0, -10.0 * band().escalate());
    let build = |x: f64| {
        with_session(budget, || {
            let place = Placement::<S> {
                mid: S::from_f64(0.5),
                levers: Levers {
                    azimuth: S::one(),
                    tube: S::one(),
                    latitude: S::one(),
                },
            };
            let dir = Azimuth::of(S::from_f64(-1.0), S::param(x_sym, x));
            let env = move |p: ParamSymbol| (p == x_sym).then_some(x1);
            let (_, decisions) = record(|| {
                place
                    .phase(Along::Azimuth, dir, |_| S::zero(), band())
                    .expect("a unit direction places")
            });
            decisions
                .iter()
                .filter(|d| d.predicate.starts_with("pcurve_chart_"))
                .map(|d| (d.predicate, d.value, revalue(d.node, &env)))
                .collect::<Vec<_>>()
        })
        .0
    };
    let (at0, at1) = (build(x0), build(x1));
    assert!(
        !at0.is_empty() && at0.len() == at1.len(),
        "both builds decide the same decisions: {at0:?} / {at1:?}"
    );
    for (k, ((n0, _, re), (n1, v1, _))) in at0.iter().zip(&at1).enumerate() {
        assert_eq!(n0, n1, "decision {k}: one name either side");
        assert_eq!(
            re.map(f64::to_bits),
            v1.map(f64::to_bits),
            "decision {k} ({n0}): one expression either side of x = 0"
        );
    }
}

/// **A direction on the window's cut reads the closed end, at every
/// scalar.** The direction at angle `ANGLE_WINDOW_START`, the window's
/// cut, lies on the frame decision's line: its margin is zero, the read
/// takes `atan2(−y, −x) + π`, and names it `τ − 1`, the window's closed
/// end, as placement's tie-break does; at `Interval` the enclosure is the
/// angle's own, not a turn wide.
#[test]
fn a_direction_on_the_windows_cut_reads_its_closed_end_at_every_scalar() {
    use geom_core::{Bounds, Interval, Real};

    use super::Azimuth;

    let r = 0.75;
    let (s, c) = ANGLE_WINDOW_START.sin_cos();
    let closed = ANGLE_WINDOW_START + TAU;
    let at_f64 = Azimuth::of(s * r, c * r).principal(band());
    assert!(
        (at_f64 - closed).abs() < 1e-12,
        "f64: the cut reads the closed end {closed}: {at_f64}"
    );
    let f = Interval::from_f64;
    let at_iv = Azimuth::of(f(s * r), f(c * r)).principal(band());
    assert!(
        at_iv.lo() <= closed + 1e-12
            && at_iv.hi() >= closed - 1e-12
            && at_iv.hi() - at_iv.lo() < 1e-12,
        "Interval: the cut reads the closed end, a narrow enclosure: {at_iv:?}"
    );
}

/// **A southern latitude places as the latitude it reads, at every
/// scalar.** The latitude window `(−π, π]` is `atan2`'s own range, so the
/// latitude is read as plain `atan2` (`Azimuth::latitude`): a latitude
/// below `−1`, which the angle window's read would name a turn up and
/// placement take back down again, keeps `k = 0` and the read bit for bit
/// (PR 5225's round-4 delta, K1).
#[test]
fn a_southern_latitude_places_bit_equal_to_its_atan2() {
    use geom_core::{Bounds, Interval, Real};

    use super::{Along, Azimuth, Placement};

    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let meridian = circle(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        1.0,
        Vec3::unit_x(),
    );
    for lat in [-1.3, -1.000_000_1, -FRAC_PI_2] {
        let (y, x) = lat.sin_cos();
        let place = Placement::over((0.5, 0.5), &meridian, &sphere);
        let placed = place
            .phase(Along::Latitude, Azimuth::of(y, x), |_| 0.0, band())
            .unwrap();
        assert_eq!(
            placed.to_bits(),
            y.atan2(x).to_bits(),
            "f64, latitude {lat}: {placed}"
        );
        let f = Interval::from_f64;
        let (meridian, sphere) = (meridian.map_scalar(f), sphere.map_scalar(f));
        let place = Placement::over((f(0.5), f(0.5)), &meridian, &sphere);
        let placed = place
            .phase(Along::Latitude, Azimuth::of(f(y), f(x)), |_| f(0.0), band())
            .unwrap();
        let read = f(y).atan2(f(x));
        assert_eq!(
            (placed.lo().to_bits(), placed.hi().to_bits()),
            (read.lo().to_bits(), read.hi().to_bits()),
            "Interval, latitude {lat}: {placed:?}"
        );
    }
}

/// **A direction whose cut-side margin is undecided reads the closed
/// end.** At `Interval`, a direction on the window's cut whose enclosure
/// is wide enough that the margin straddles the escalation gap leaves the
/// frame decision undecided; the read then takes the turned formula,
/// `atan2(−y, −x) + π`, whose value there is the closed end `τ − 1`,
/// never the direct formula's `−1` (PR 5225's round-4 delta, K5).
#[test]
fn an_undecided_cut_side_reads_the_turned_formula() {
    use geom_core::{Bounds, Interval};

    use super::Azimuth;

    let r = 0.75;
    let (s, c) = ANGLE_WINDOW_START.sin_cos();
    let w = 4.0 * band().escalate();
    let wide = |v: f64| Interval::from_bounds(v - w, v + w);
    let closed = ANGLE_WINDOW_START + TAU;
    let at = Azimuth::of(wide(s * r), wide(c * r)).principal(band());
    assert!(
        at.lo() > PI && at.lo() <= closed && at.hi() >= closed,
        "the undecided cut side reads the turned formula, about the closed end {closed}: {at:?}"
    );
}
// ---- review 5225 r2 probes ----

fn r2_villarceau(phi: f64, family: f64, psi: f64) -> (Surface<f64>, Curve3<f64>) {
    let (major, minor) = (2.0, 0.7);
    let axis = Vec3::unit_z();
    let u_ref = Vec3::unit_x();
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis,
        major_radius: major,
        minor_radius: minor,
        u_ref,
    };
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (family * tilt.sin());
    let c = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0) + d * minor,
        axis: d.cross(lean),
        radius: major,
        u_ref: d * psi.cos() + lean * psi.sin(),
    };
    (torus, c)
}

fn report(name: &str, surface: &Surface<f64>, carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> Vec<String> {
    let band = band();
    let mut bad = vec![];
    let base = chart_pcurve_over(carrier, t0, t1, surface, band).unwrap();
    for k in [-3.0, -1.0, 0.0, 1.0, 3.0] {
        let (a, b) = (t0 + k * TAU, t1 + k * TAU);
        let image = chart_pcurve_over(carrier, a, b, surface, band).unwrap();
        let m = image.eval(0.5 * (a + b));
        let q = base.eval(0.5 * (t0 + t1));
        eprintln!("{name} k={k}: mid {m:?} (base {q:?})");
        if !(in_window(m.x)) { bad.push(format!("{name} k={k}: u {} out of window", m.x)); }
        if (m.y - q.y).abs() > 1e-9 { bad.push(format!("{name} k={k}: v {} vs base {}", m.y, q.y)); }
        if (m.x - q.x).abs() > 1e-9 { bad.push(format!("{name} k={k}: u {} vs base {}", m.x, q.x)); }
    }
    bad
}

#[test]
fn r2_villarceau_v_channel() {
    let (torus, c) = r2_villarceau(0.7, 1.0, 0.3);
    let bad = report("villarceau", &torus, &c, (0.4, 1.6));
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn r2_spiric_wall_v_channel() {
    let (r, rr, d) = (0.09375, 0.0703125, 0.0078125);
    let c = Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
        major_radius: r,
        minor_radius: rr,
        offset: d,
    };
    let torus = Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_y(),
        major_radius: r,
        minor_radius: rr,
        u_ref: Vec3::unit_x(),
    };
    let t = 0.8911225078866529;
    let bad = report("spiric wall", &torus, &c, (-t, t));
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn r2_sphere_meridian_south() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    // meridian in the xz plane; circle(o, -y, 1, x): t=0 at +x, t=π/2 at ?
    let c = circle(Point3::new(0.0, 0.0, 0.0), -Vec3::unit_y(), 1.0, Vec3::unit_x());
    let band = band();
    for (t0, t1) in [(0.2, 1.0), (-1.5, -1.1), (-1.4, -0.9), (4.9, 5.2)] {
        let image = chart_pcurve_over(&c, t0, t1, &sphere, band).unwrap();
        let old = super::chart_pcurve(&c, &sphere, band).unwrap();
        eprintln!("sphere meridian [{t0},{t1}]: mid {:?}, ends {:?} {:?}; spanless mid {:?}", image.eval(0.5*(t0+t1)), image.eval(t0), image.eval(t1), old.eval(0.5*(t0+t1)));
        let cert = PcurveCache::certify(image.clone(), t0, t1, &c, &sphere, band);
        eprintln!("  certifies: {:?}", cert.as_ref().map(|c| c.certificate().envelope));
    }
}

#[test]
fn r2_cone_focal_u() {
    let cone = Surface::Cone {
        apex: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::unit_z(),
        half_angle: 0.4,
        u_ref: Vec3::unit_x(),
    };
    // an oblique plane z = 1 + 0.2 x cuts an ellipse
    let n = Vec3::new(-0.2, 0.0, 1.0).normalize();
    // find ellipse via intersect? build directly: param the section numerically is hard; use geom's plane-cone section if any
    let _ = (cone, n);
}

/// delta4 probe: a direction moved across the read's cut line (the line
/// through `ANGLE_WINDOW_START`), with the gap placed somewhere other than
/// the window's cut (an offset, or the latitude window). Prints, per case,
/// whether every `pcurve_chart_` decision of the `t0` build re-values at
/// `t1` to the bits the `t1` build decides.
#[test]
#[allow(clippy::expect_used)]
fn delta4_read_cut_off_the_windows_cut() {
    use geom_core::sym::revalue::{record, revalue};
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Real, Sym, SymBudget};

    use super::{Along, Azimuth, Levers, Placement};

    type S = Sym<f64>;
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let t_sym = ParamSymbol::new(1);
    let e = 10.0 * band().escalate();
    let mut report = Vec::new();
    for (case, along, off, ang, t0, t1) in [
        ("azimuth, offset 0, dir at -1 (window cut)", Along::Azimuth, 0.0, ANGLE_WINDOW_START, -e, e),
        ("azimuth, offset 2.0, dir at -1 (gap at 1.0)", Along::Azimuth, 2.0, ANGLE_WINDOW_START, -e, e),
        ("azimuth, offset 2.0, dir at pi-1 (gap at pi+1)", Along::Azimuth, 2.0, PI + ANGLE_WINDOW_START, -e, e),
        ("latitude, offset 0, dir at -1", Along::Latitude, 0.0, ANGLE_WINDOW_START, -e, e),
        ("latitude, offset 0, dir at pi-1", Along::Latitude, 0.0, PI + ANGLE_WINDOW_START, -e, e),
        ("tube, offset 0.5, dir at -1", Along::Tube, 0.5, ANGLE_WINDOW_START, -e, e),
        ("control: azimuth, offset 2.0, dir at 0.3", Along::Azimuth, 2.0, 0.3, -e, e),
    ] {
        let (s, c) = ang.sin_cos();
        let build = |t: f64| {
            with_session(budget, || {
                let place = Placement::<S> {
                    mid: S::from_f64(0.5),
                    levers: Levers {
                        azimuth: S::one(),
                        tube: S::one(),
                        latitude: S::one(),
                    },
                };
                let tt = S::param(t_sym, t);
                let dir = Azimuth::of(S::from_f64(s) + tt * S::from_f64(c), S::from_f64(c) - tt * S::from_f64(s));
                let env = move |p: ParamSymbol| (p == t_sym).then_some(t1);
                let (placed, decisions) = record(|| {
                    place
                        .phase(along, dir, |_| S::from_f64(off), band())
                        .expect("a unit direction places")
                });
                (
                    geom_core::Bounds::lo(placed),
                    decisions
                        .iter()
                        .filter(|d| d.predicate.starts_with("pcurve_chart_"))
                        .map(|d| (d.predicate, d.value, revalue(d.node, &env)))
                        .collect::<Vec<_>>(),
                )
            })
            .0
        };
        let ((p0, a0), (p1, a1)) = (build(t0), build(t1));
        let same_len = a0.len() == a1.len();
        let mut bad = Vec::new();
        for (k, ((n0, _, re), (n1, v1, _))) in a0.iter().zip(&a1).enumerate() {
            if n0 != n1 || re.map(f64::to_bits) != v1.map(f64::to_bits) {
                bad.push(format!("#{k} {n0}: re-valued {re:?} vs decided {v1:?}"));
            }
        }
        report.push(format!(
            "{case}: placed {p0:?} -> {p1:?}; decisions {} / {}; mismatches {bad:?}",
            a0.len(),
            a1.len()
        ));
        let _ = same_len;
    }
    for r in &report {
        eprintln!("DELTA4 {r}");
    }
}

/// delta4 probe: a latitude below the read's cut (−1) reads a turn up and
/// is placed back by a whole turn; is the placed value atan2's own bits?
#[test]
fn delta4_latitude_below_the_reads_cut() {
    use super::{Along, Azimuth, Levers, Placement};
    let place = Placement::<f64> {
        mid: 0.0,
        levers: Levers {
            azimuth: 1.0,
            tube: 1.0,
            latitude: 1.0,
        },
    };
    for lat in [-FRAC_PI_2, -1.3, -1.0000001, -0.9, 0.5] {
        let (y, x) = (lat.sin(), lat.cos());
        let read = Azimuth::of(y, x).principal(band());
        let turns = place.turns(Along::Latitude, read, band()).unwrap();
        let placed = place.phase(Along::Latitude, Azimuth::of(y, x), |_| 0.0, band()).unwrap();
        eprintln!(
            "DELTA4LAT lat {lat:?}: read {read:?} turns {turns} placed {placed:?} atan2 {:?} bit-equal {}",
            y.atan2(x),
            placed.to_bits() == y.atan2(x).to_bits()
        );
    }
}

// ---- review 5225 delta5 probes ----

/// delta5: a meridian placed on the far half reads its polar angle as
/// `atan2(aa, −‖a_r‖)`: at an equator start (`aa ≈ 0`) that is `π`, on
/// atan2's own cut. At Interval with `aa` straddling zero, what is read?
#[test]
fn delta5_far_meridian_polar_angle_at_the_equator_phase() {
    use geom_core::{Bounds, Interval, Real};

    use super::{Along, Azimuth, Levers, Placement};
    let f = Interval::from_f64;
    for (lo, hi) in [(0.0, 0.0), (-1e-17, 1e-17), (-1e-12, 1e-12)] {
        let place = Placement::<Interval> {
            mid: f(3.0),
            levers: Levers {
                azimuth: f(1.0),
                tube: f(1.0),
                latitude: f(1.0),
            },
        };
        let dir = Azimuth::of(Interval::from_bounds(lo, hi), f(-1.0));
        let lat = dir.latitude();
        let pr = dir.principal(band());
        let placed = place.phase(Along::Latitude, dir, |m| m, band());
        eprintln!(
            "DELTA5PHASE aa [{lo:e},{hi:e}]: latitude read [{:.17},{:.17}] principal read [{:.17},{:.17}] placed {:?}",
            lat.lo(),
            lat.hi(),
            pr.lo(),
            pr.hi(),
            placed.map(|p| (p.lo(), p.hi(), p.hi() - p.lo()))
        );
    }
}

/// delta5: end to end, a meridian of a sphere with a generic axis, its
/// carrier started on the equator, its edge on the far half (midpoint at
/// `t = 3`). Interval image width of the polar channel, against the same
/// edge on the carrier started elsewhere (near half).
#[test]
fn delta5_far_meridian_on_a_tilted_sphere_end_to_end() {
    use geom_core::{Bounds, Interval, Real};
    let axis = Vec3::new(0.0, 0.6, 0.8);
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis,
        u_ref: Vec3::unit_x(),
    };
    let a = Vec3::new(0.0, 0.8, -0.6);
    let o = Point3::new(0.0, 0.0, 0.0);
    let start_eq = circle(o, Vec3::new(-1.0, 0.0, 0.0), 1.0, a);
    // The same great circle started a quarter turn on (at -axis).
    let start_q = circle(o, Vec3::new(-1.0, 0.0, 0.0), 1.0, axis * -1.0);
    let f = Interval::from_f64;
    for (name, c, (t0, t1)) in [
        ("equator start, far half", &start_eq, (2.5, 3.5)),
        ("equator start, near half", &start_eq, (-0.5, 0.5)),
        ("quarter start, same edge as far", &start_q, (2.5 - FRAC_PI_2, 3.5 - FRAC_PI_2)),
    ] {
        let p = chart_pcurve_over(c, t0, t1, &sphere, band());
        let at_f64 = p.as_ref().map(|p| p.eval(0.5 * (t0 + t1)));
        let (ci, si) = (c.map_scalar(f), sphere.map_scalar(f));
        let q = chart_pcurve_over(&ci, f(t0), f(t1), &si, band());
        let at_iv = q.as_ref().map(|q| {
            let e = q.eval(f(0.5 * (t0 + t1)));
            ((e.x.lo(), e.x.hi()), (e.y.lo(), e.y.hi()), e.y.hi() - e.y.lo())
        });
        eprintln!("DELTA5E2E {name}: f64 mid {at_f64:?}; Interval mid {at_iv:?}");
        let cert = q.map(|img| {
            PcurveCache::certify(img, f(t0), f(t1), &ci, &si, band())
                .map(|c| format!("{:?}", c.certificate().envelope))
        });
        eprintln!("DELTA5CERT {name}: Interval certify {cert:?}");
    }
}

/// delta5: latitude exactly at ±π/2 and an Interval enclosure straddling a
/// pole (x straddling zero).
#[test]
fn delta5_latitude_at_and_astride_the_pole() {
    use geom_core::{Bounds, Interval, Real};

    use super::Azimuth;
    for (y, x) in [(1.0, 0.0), (-1.0, 0.0), (1.0, -0.0), (-1.0, -0.0)] {
        eprintln!(
            "DELTA5POLE f64 ({y},{x}): latitude {:?} principal {:?}",
            Azimuth::of(y, x).latitude(),
            Azimuth::of(y, x).principal(band())
        );
    }
    let f = Interval::from_f64;
    for (y, xlo, xhi) in [(1.0, -1e-12, 1e-12), (-1.0, -1e-12, 1e-12), (1.0, -0.0, 0.0)] {
        let d = Azimuth::of(f(y), Interval::from_bounds(xlo, xhi));
        let l = d.latitude();
        let p = d.principal(band());
        eprintln!(
            "DELTA5POLE Interval y {y} x [{xlo:e},{xhi:e}]: latitude [{:.17},{:.17}] principal [{:.17},{:.17}]",
            l.lo(),
            l.hi(),
            p.lo(),
            p.hi()
        );
    }
}
