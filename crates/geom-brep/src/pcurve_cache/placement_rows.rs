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

/// **A placement's margin is one expression per name across a frame
/// flip.** A direction pointing down the chart (`y = −1`), read at
/// `Sym<f64>` with its `x` a parameter: at `x = 0` the frame lane reads
/// `atan2(y, x)`, at `x` ten band-widths negative `atan2(−y, −x) +
/// π`, the same angle a turn up, so the placement's gap is a different
/// expression on each side. Each side decides under its own formula's
/// name (`Frame::placement`), and within a name every margin re-values
/// at a nearby `x` on the same side to the bits that build decides
/// there. (`editor-core`'s `revalue_declared_tangency` is the corpus
/// witness: one name over both formulas re-valued a turn away.)
#[test]
#[allow(clippy::expect_used)]
fn a_placements_margin_is_one_expression_per_name_across_a_frame_flip() {
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
    // One build at `x`: each placement decision's name and value, and its
    // margin re-valued at `to`.
    let build = |x: f64, to: f64| {
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
            let env = move |p: ParamSymbol| (p == x_sym).then_some(to);
            let (_, decisions) = record(|| {
                place
                    .phase(Along::Azimuth, dir, |_| S::zero(), band())
                    .expect("a unit direction places")
            });
            decisions
                .iter()
                .filter(|d| d.predicate.starts_with("pcurve_chart_placement"))
                .map(|d| (d.predicate, d.value, revalue(d.node, &env)))
                .collect::<Vec<_>>()
        })
        .0
    };
    // Past the band on the turned side at every ε row, so the frame
    // decides Negative there.
    let past = 10.0 * band().escalate();
    for (side, x, near, name) in [
        ("direct", 0.0, past, "pcurve_chart_placement"),
        (
            "turned",
            -past,
            -2.0 * past,
            "pcurve_chart_placement_turned",
        ),
    ] {
        let here = build(x, near);
        let there = build(near, near);
        assert!(
            !here.is_empty() && here.len() == there.len(),
            "{side}: both builds decide the same marks: {here:?} / {there:?}"
        );
        for (k, ((n0, _, re), (n1, v1, _))) in here.iter().zip(&there).enumerate() {
            assert!(
                *n0 == name && *n1 == name,
                "{side}, margin {k}: decided under its formula's name {name}: {n0} / {n1}"
            );
            assert_eq!(
                re.map(f64::to_bits),
                v1.map(f64::to_bits),
                "{side}, margin {k}: one expression under {name}"
            );
        }
    }
}
