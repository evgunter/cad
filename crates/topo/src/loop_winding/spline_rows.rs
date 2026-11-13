//! **A spline arc's term in the winding sum**: the closed form against
//! areas known in closed form, the certified enclosure of a rational
//! arc, and the windings of loops whose area is their bulges'.

use super::*;
use core::f64::consts::FRAC_1_SQRT_2;
use geom::NurbsCurve3;
use geom_core::interval::Interval;
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Bounds, Dual64};

fn band() -> geom_core::Band {
    geom_core::Band::linear(geom_core::Tol::witness()).unwrap()
}

/// A clamped spline carrier at `f64`.
fn spline(knots: Vec<f64>, degree: usize, pts: &[[f64; 3]], weights: Vec<f64>) -> Curve3<f64> {
    Curve3::Nurbs(std::sync::Arc::new(
        NurbsCurve3::new(
            Knots::from(KnotVector::clamped(knots, degree).unwrap()),
            pts.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect(),
            weights,
        )
        .unwrap(),
    ))
}

/// The parabola `(t, h·4t(1 − t), z)` over `[0, 1]`, from `(0, 0)` to
/// `(1, 0)` with its apex `h` above the chord, as a quadratic with a
/// knot at `½` it does not need; `lift` raises its middle control points
/// off the plane.
fn parabola(h: f64, lift: f64) -> Curve3<f64> {
    spline(
        vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
        2,
        &[
            [0.0, 0.0, 0.0],
            [0.25, h, lift],
            [0.75, h, lift],
            [1.0, 0.0, 0.0],
        ],
        vec![1.0; 4],
    )
}

/// The quarter of the circle of radius `r` about the origin from
/// `(r, 0)` to `(0, r)`, as the rational quadratic.
fn quarter(r: f64) -> Curve3<f64> {
    spline(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[[r, 0.0, 0.0], [r, r, 0.0], [0.0, r, 0.0]],
        vec![1.0, FRAC_1_SQRT_2, 1.0],
    )
}

/// `carrier`'s arc `(t0, t1)` lifted to the interval scalar.
fn lifted(carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> (Curve3<Interval>, (Interval, Interval)) {
    (
        carrier.map_scalar(Interval::from_certified),
        (Interval::from_certified(t0), Interval::from_certified(t1)),
    )
}

/// The winding about `+z` of the open run along `carrier` over `span`,
/// closed by its straight chord.
fn run_winding(carrier: &Curve3<f64>, (t0, t1): (f64, f64), forward: bool) -> Winding {
    let (ta, tb) = if forward { (t0, t1) } else { (t1, t0) };
    Body::<f64>::winding_of_steps(
        &[(carrier.eval(ta), Some(((carrier, (t0, t1)), forward)))],
        carrier.eval(tb),
        None,
        true,
        Vec3::unit_z(),
        band(),
    )
}

/// **A non-rational arc's bulge is the area it cuts off its chord, to
/// rounding**: the parabola `y = 4h·t(1 − t)` over `(t0, t1)` cuts
/// `(2/3)·h·(t1 − t0)³` off its chord, so its bulge is twice that about
/// `−z` run forward (the arc is to the left of its chord run backward),
/// across the knot and inside one span; and its length term is at least
/// its chord's.
#[test]
fn a_polynomial_arcs_bulge_is_its_closed_form_area() {
    let h = 0.375;
    let carrier = parabola(h, 0.0);
    assert_eq!(segment_kind(&carrier), SegmentKind::Closed);
    for (t0, t1) in [(0.0, 1.0), (0.2, 0.9), (0.55, 0.8), (0.0, 0.5)] {
        for forward in [true, false] {
            let (bulge, length) =
                closed_segment_term((&carrier, (t0, t1)), forward).expect("a closed form");
            let dt: f64 = t1 - t0;
            let want = (4.0 / 3.0) * h * dt.powi(3) * if forward { -1.0 } else { 1.0 };
            assert!(
                (bulge.z - want).abs() <= 1e-15 && bulge.x == 0.0 && bulge.y == 0.0,
                "{t0}..{t1} forward {forward}: {bulge:?} against {want}"
            );
            let chord = (carrier.eval(t1) - carrier.eval(t0)).norm();
            assert!(length >= chord, "{t0}..{t1}: length {length} under {chord}");
        }
    }
}

/// **A rational arc's bulge is enclosed, and decided at a certifying
/// scalar**: the quarter circle of radius 2 over `(t0, t1)` cuts
/// `R²(Δ − sin Δ)/2` off its chord, `Δ` the angle between its ends. The
/// enclosure holds twice that at the first piece count, under `1e-5` m²
/// wide, and at the second, under `1e-9`; the run it closes with its chord winds with the arc's
/// traversal at `f64`, and has no winding at a dual, which holds no
/// certified lane.
#[test]
fn a_rational_arc_is_enclosed_and_winds_at_a_certifying_scalar() {
    let r = 2.0;
    let carrier = quarter(r);
    assert_eq!(segment_kind(&carrier), SegmentKind::Enclosed);
    assert!(
        closed_segment_term((&carrier, (0.0, 1.0)), true).is_none(),
        "a rational arc has no closed form"
    );
    for (t0, t1) in [(0.0, 1.0), (0.1, 0.7)] {
        let angle = |t: f64| {
            let p = carrier.eval(t);
            p.y.atan2(p.x)
        };
        let delta = angle(t1) - angle(t0);
        let want = r * r * (delta - delta.sin());
        for forward in [true, false] {
            let want = if forward { want } else { -want };
            let (c, span) = lifted(&carrier, (t0, t1));
            let edge = SegmentQ::of((&c, span), forward).expect("the enclosure builds");
            let term = |pieces| edge.term(pieces).unwrap().expect("a spline has a term");
            let (bulge, length) = term(ENCLOSURE_PIECES[0]);
            let (finer, _) = term(ENCLOSURE_PIECES[1]);
            for (count, got, width) in [(0, bulge.z, 1e-5), (1, finer.z, 1e-9)] {
                assert!(
                    got.lo() <= want && want <= got.hi() && got.hi() - got.lo() < width,
                    "{t0}..{t1} forward {forward}, count {count}: [{}, {}] against {want}",
                    got.lo(),
                    got.hi()
                );
            }
            assert!(
                length.lo() >= r * delta * (1.0 - 1e-12),
                "{t0}..{t1}: the length bound {} is under the arc's {}",
                length.lo(),
                r * delta
            );
            let sign = if forward {
                Sign::Positive
            } else {
                Sign::Negative
            };
            let wound = run_winding(&carrier, (t0, t1), forward);
            assert!(
                matches!(wound, Ok(Ok(Decided { sign: s, .. })) if s == sign),
                "{t0}..{t1} forward {forward}: {wound:?}"
            );
        }
    }
    let dual = carrier.map_scalar(Dual64::constant);
    let span = (Dual64::constant(0.0), Dual64::constant(1.0));
    let wound = Body::<Dual64>::winding_of_steps(
        &[(dual.eval(span.0), Some(((&dual, span), true)))],
        dual.eval(span.1),
        None,
        true,
        Vec3::unit_z().map(Dual64::constant),
        band(),
    );
    assert_eq!(
        wound.map(|w| w.map(|d| d.sign)),
        Err(Unwound::Uncertified),
        "a dual holds no certified lane"
    );
}

/// **A polynomial arc past the exact rule's degrees is read by its
/// enclosure**: the parabola `y = 4h·t(1 − t)` raised to degree 6 (its
/// Bézier points, `yᵢ = 4h·i(6 − i)/30`) has no closed-form term, and its
/// enclosure holds the parabola's area.
#[test]
fn a_degree_past_the_exact_rule_is_read_by_its_enclosure() {
    let h = 0.375;
    let pts: Vec<[f64; 3]> = (0..=6)
        .map(|i| {
            let i = f64::from(i);
            [i / 6.0, 4.0 * h * i * (6.0 - i) / 30.0, 0.0]
        })
        .collect();
    let mut knots = vec![0.0; 7];
    knots.extend([1.0; 7]);
    let carrier = spline(knots, 6, &pts, vec![1.0; 7]);
    assert_eq!(segment_kind(&carrier), SegmentKind::Enclosed);
    let (c, span) = lifted(&carrier, (0.0, 1.0));
    let (bulge, _) = SegmentQ::of((&c, span), true)
        .expect("the enclosure builds")
        .term(ENCLOSURE_PIECES[0])
        .unwrap()
        .expect("a spline has a term");
    let want = -(4.0 / 3.0) * h;
    assert!(
        bulge.z.lo() - 1e-13 <= want && want <= bulge.z.hi() + 1e-13,
        "[{}, {}] against {want}",
        bulge.z.lo(),
        bulge.z.hi()
    );
    let wound = run_winding(&carrier, (0.0, 1.0), true);
    assert!(
        matches!(
            wound,
            Ok(Ok(Decided {
                sign: Sign::Negative,
                ..
            }))
        ),
        "{wound:?}"
    );
}

/// **A lens is wound by its bulges alone**: two arcs between the same
/// two vertices, whose chord polygon has no area. A parabola below the
/// chord run out and one above it run back wind counter-clockwise about
/// `+z` with the area `(2/3)(h₁ + h₂)` between them, over a lever no
/// shorter than their chords; the same cycle traversed the other way
/// winds clockwise; and with a rational arc for the upper side it is
/// decided the same, at the interval scalar.
#[test]
fn a_lens_of_two_arcs_winds_by_its_bulges() {
    let (a, b) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let (below, above) = (parabola(-0.25, 0.0), parabola(0.5, 0.0));
    let span = (0.0, 1.0);
    let ccw = [
        (a, Some(((&below, span), true))),
        (b, Some(((&above, span), false))),
    ];
    let cw = [
        (a, Some(((&above, span), true))),
        (b, Some(((&below, span), false))),
    ];
    let wind = |steps: &[Step<'_, f64>]| {
        Body::<f64>::winding_of_steps(steps, a, None, false, Vec3::unit_z(), band())
    };
    let terms: Vec<_> = ccw
        .iter()
        .map(|&(p, c)| (p, c.and_then(|(c, fwd)| closed_segment_term(c, fwd))))
        .collect();
    let (area, perimeter) = winding_sum(&terms, a, None, false, Vec3::unit_z());
    assert!(
        (area - (4.0 / 3.0) * 0.75).abs() <= 1e-15,
        "twice the lens's area: {area}"
    );
    assert!(perimeter >= 2.0, "the lever holds both chords: {perimeter}");
    for (name, steps, want) in [
        ("counter-clockwise", &ccw, Sign::Positive),
        ("clockwise", &cw, Sign::Negative),
    ] {
        let wound = wind(steps);
        assert!(
            matches!(wound, Ok(Ok(Decided { sign, .. })) if sign == want),
            "{name}: {wound:?}"
        );
    }
    // The quarter circle from `(2, 0)` to `(0, 2)` and a parabola
    // sagging toward the origin from its chord.
    let arc = quarter(2.0);
    let sag = spline(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[[2.0, 0.0, 0.0], [0.5, 0.5, 0.0], [0.0, 2.0, 0.0]],
        vec![1.0; 3],
    );
    let (p, q) = (Point3::new(2.0, 0.0, 0.0), Point3::new(0.0, 2.0, 0.0));
    let wound = Body::<f64>::winding_of_steps(
        &[
            (p, Some(((&arc, span), true))),
            (q, Some(((&sag, span), false))),
        ],
        p,
        None,
        false,
        Vec3::unit_z(),
        band(),
    );
    assert!(
        matches!(
            wound,
            Ok(Ok(Decided {
                sign: Sign::Positive,
                ..
            }))
        ),
        "the rational lens: {wound:?}"
    );
}

/// **An arc standing off the plane is wound as its projection**: the
/// parabola with its middle raised `10⁻³` off `z = 0` has the same bulge
/// along the normal, to the bit, and the run it closes winds the same.
#[test]
fn an_arc_standing_off_the_plane_winds_as_its_projection() {
    let (flat, raised) = (parabola(0.375, 0.0), parabola(0.375, 1e-3));
    for span in [(0.0, 1.0), (0.2, 0.9)] {
        let term = |c: &Curve3<f64>| closed_segment_term((c, span), true).expect("a closed form");
        let ((on, _), (off, _)) = (term(&flat), term(&raised));
        assert_eq!(on.z, off.z, "{span:?}: the bulge along the normal");
        assert!(
            off.x != 0.0 || off.y != 0.0,
            "{span:?}: the premise, an arc off the plane: {off:?}"
        );
        let sign = |c| run_winding(c, span, true).map(|w| w.map(|d| d.sign));
        assert_eq!(sign(&flat), Ok(Ok(Sign::Negative)), "{span:?}");
        assert_eq!(sign(&raised), sign(&flat), "{span:?}");
    }
}
