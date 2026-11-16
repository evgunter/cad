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
/// across the knot and inside one span; and its length term is no less
/// than an inscribed polygon's of 256 chords, which the arc's one chord
/// is.
#[test]
fn a_polynomial_arcs_bulge_is_its_closed_form_area() {
    let h = 0.375;
    let carrier = parabola(h, 0.0);
    assert_eq!(segment_kind(&carrier), SegmentKind::Closed);
    for (t0, t1) in [(0.0, 1.0), (0.2, 0.9), (0.55, 0.8), (0.0, 0.5)] {
        for forward in [true, false] {
            let SegmentTerm { bulge, length } =
                closed_segment_term((&carrier, (t0, t1)), forward).expect("a closed form");
            let dt: f64 = t1 - t0;
            let want = (4.0 / 3.0) * h * dt.powi(3) * if forward { -1.0 } else { 1.0 };
            assert!(
                (bulge.z - want).abs() <= 1e-15 && bulge.x == 0.0 && bulge.y == 0.0,
                "{t0}..{t1} forward {forward}: {bulge:?} against {want}"
            );
            let at = |k: u32| carrier.eval(t0 + dt * f64::from(k) / 256.0);
            let polygon: f64 = (0..256).map(|k| (at(k + 1) - at(k)).norm()).sum();
            assert!(
                length >= polygon,
                "{t0}..{t1}: the length bound {length} is under an inscribed polygon's {polygon}"
            );
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
            let SegmentTerm { bulge, length } = term(ENCLOSURE_PIECES[0]);
            // The finer count is read one way round: it is the cost.
            let finer = forward.then(|| (1, term(ENCLOSURE_PIECES[1]).bulge.z, 1e-9));
            for (count, got, width) in [(0, bulge.z, 1e-5)].into_iter().chain(finer) {
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
    let bulge = SegmentQ::of((&c, span), true)
        .expect("the enclosure builds")
        .term(ENCLOSURE_PIECES[0])
        .unwrap()
        .expect("a spline has a term")
        .bulge;
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
        let (on, off) = (term(&flat).bulge, term(&raised).bulge);
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

/// The cubic whose first span `[0, ½]` is a shallow arch of height
/// about `h` over the unit chord, and whose second runs out to `x = far`:
/// a quiet stretch of a carrier that is fast elsewhere.
fn quiet_span(h: f64, far: f64) -> Curve3<f64> {
    spline(
        vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0],
        3,
        &[
            [0.0, 0.0, 0.0],
            [0.33, h, 0.0],
            [0.67, h, 0.0],
            [1.0, 0.0, 0.0],
            [far, 0.0, 0.0],
        ],
        vec![1.0; 5],
    )
}

/// The rational cubic of seven control points `scale` times an arch over
/// `x ∈ [1, 6]` from a first point at `x = x0`, its weights spread over
/// three decades.
fn wide_weights(scale: f64, x0: f64) -> Curve3<f64> {
    let ys = [0.0, 1.0, 1.5, 2.0, 1.5, 1.0, 0.0];
    let pts: Vec<[f64; 3]> = (0..7u8)
        .map(|i| {
            let x = if i == 0 { x0 } else { f64::from(i) };
            [scale * x, scale * ys[usize::from(i)], 0.0]
        })
        .collect();
    spline(
        vec![0.0, 0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0, 1.0],
        3,
        &pts,
        vec![1.0, 50.0, 0.02, 3.0, 1.0, 0.05, 1.0],
    )
}

/// The inscribed polygon of `n` chords of `carrier` over `(t0, t1)`:
/// twice its area about `+z` closed by the one chord (the arc run
/// forward), and its length.
fn polygon(carrier: &Curve3<f64>, (t0, t1): (f64, f64), n: u32) -> (f64, f64) {
    let at = |k: u32| carrier.eval(t0 + (t1 - t0) * f64::from(k) / f64::from(n));
    let origin = at(0);
    (0..n).fold((0.0, 0.0), |(area, length), k| {
        let (p, q) = (at(k), at(k + 1));
        (
            area + (p - origin).cross(q - origin).z,
            length + (q - p).norm(),
        )
    })
}

/// The winding about `+z` at `band` of the open run forward along
/// `carrier` over `span`, closed by its straight chord.
fn run_winding_at(carrier: &Curve3<f64>, span: (f64, f64), band: geom_core::Band) -> Winding {
    Body::<f64>::winding_of_steps(
        &[(carrier.eval(span.0), Some(((carrier, span), true)))],
        carrier.eval(span.1),
        None,
        true,
        Vec3::unit_z(),
        band,
    )
}

/// The band at `ε = 10⁻⁶` and the run's `K`.
fn micron_band() -> geom_core::Band {
    geom_core::Band::linear_at(geom_core::Tol::witness(), 1e-6).unwrap()
}

/// **An edge's length term is its own stretch's, on the closed lane**:
/// the edge on the slow first span of a cubic whose second span runs out
/// to `x = 50` is charged within a quarter of an inscribed polygon's
/// length, where the carrier's speed sup times the span is over two
/// hundred times it. At `ε = 10⁻⁶` the run it closes with its chord, a
/// region of mean width between 20 and 30 ε, winds clockwise, where the
/// loose lever read it as no area.
#[test]
fn a_quiet_span_of_a_fast_carrier_is_not_wound_zero() {
    let span = (0.0, 0.5);
    for far in [20.0, 50.0] {
        let carrier = quiet_span(1e-4, far);
        assert_eq!(segment_kind(&carrier), SegmentKind::Closed);
        let lever = closed_segment_term((&carrier, span), true).unwrap().length;
        let (area, length) = polygon(&carrier, span, 4096);
        assert!(
            length <= lever && lever < 1.25 * length,
            "far {far}: the lever {lever} against an inscribed polygon's {length}"
        );
        let chord = (carrier.eval(span.1) - carrier.eval(span.0)).norm();
        let width = area.abs() / (length + chord);
        assert!(
            (20e-6..30e-6).contains(&width),
            "far {far}: the premise, a region 20 to 30 ε wide: {width}"
        );
        let wound = run_winding_at(&carrier, span, micron_band());
        assert!(
            matches!(
                wound,
                Ok(Ok(Decided {
                    sign: Sign::Negative,
                    ..
                }))
            ),
            "far {far}: {wound:?}"
        );
    }
}

/// **An edge's length term is its own stretch's, on the enclosure
/// lane**: the arc `(0.12, 0.9)` of a rational cubic with weights over
/// three decades, whose first control point stands 0.3 m or 1 m off the
/// rest, runs past the fast start. Its lever is within a quarter of an
/// inscribed polygon's length, where the carrier's speed sup times the
/// span is hundreds to thousands of times it; and at `ε = 10⁻⁶` the run
/// it closes with its chord, about 5 mm wide, winds clockwise at the
/// first piece count, where the loose lever escalated at the finest or
/// read no area.
#[test]
fn a_wide_weight_arc_past_a_fast_start_is_decided() {
    let span = (0.12, 0.9);
    for start in [-30.0, -100.0] {
        let carrier = wide_weights(0.01, start);
        assert_eq!(segment_kind(&carrier), SegmentKind::Enclosed);
        let (c, lifted_span) = lifted(&carrier, span);
        let lever = SegmentQ::of((&c, lifted_span), true)
            .expect("the enclosure builds")
            .term(ENCLOSURE_PIECES[0])
            .unwrap()
            .expect("a spline has a term")
            .length;
        let (area, length) = polygon(&carrier, span, 1 << 14);
        assert!(
            length <= lever.lo() && lever.hi() < 1.25 * length,
            "start {start}: the lever {lever:?} against an inscribed polygon's {length}"
        );
        assert!(area < 0.0, "start {start}: the premise, a clockwise run");
        let wound = run_winding_at(&carrier, span, micron_band());
        assert!(
            matches!(
                wound,
                Ok(Ok(Decided {
                    sign: Sign::Negative,
                    ..
                }))
            ),
            "start {start}: {wound:?}"
        );
    }
}

/// The quarter-circle lens: [`quarter`] of radius 2 out, and back along
/// the conic through the same ends and tangents whose middle weight is
/// `delta` more.
fn weight_lens(delta: f64) -> [Curve3<f64>; 2] {
    let fatter = spline(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        2,
        &[[2.0, 0.0, 0.0], [2.0, 2.0, 0.0], [0.0, 2.0, 0.0]],
        vec![1.0, FRAC_1_SQRT_2 + delta, 1.0],
    );
    [quarter(2.0), fatter]
}

/// The lens's margin at `pieces` per span, at the interval scalar.
fn lens_margin([out, back]: &[Curve3<f64>; 2], pieces: usize) -> Margin<Interval> {
    let span = (0.0, 1.0);
    let point = |p: Point3<f64>| p.map(Interval::from_certified);
    let term = |c: &Curve3<f64>, forward| {
        let (c, span) = lifted(c, span);
        SegmentQ::of((&c, span), forward)
            .expect("the enclosure builds")
            .term(pieces)
            .unwrap()
    };
    let (p, q) = (point(out.eval(0.0)), point(out.eval(1.0)));
    let points = [(p, term(out, true)), (q, term(back, false))];
    let (area, perimeter) = winding_sum(
        &points,
        p,
        None,
        false,
        Vec3::<f64>::unit_z().map(Interval::from_certified),
    );
    Margin::over_lever(area, perimeter)
}

/// **The refinement ladder reads past its first count on a spline**: a
/// lens between two rational arcs a millionth apart in their middle
/// weight has a margin of about `2.5·10⁻⁷` m, which the first count's
/// enclosure straddles zero on and the second decides at `ε = 10⁻⁹`.
#[test]
fn a_thin_rational_lens_is_decided_at_a_finer_count() {
    let band = geom_core::Band::linear_at(geom_core::Tol::witness(), 1e-9).unwrap();
    let lens = weight_lens(1e-6);
    let decided = |pieces| {
        crate::validate::decide(
            "planar_winding_enclosure_refine",
            lens_margin(&lens, pieces),
            band,
        )
    };
    assert!(
        decided(ENCLOSURE_PIECES[0]).is_err(),
        "the premise, a first count that does not decide: {:?}",
        lens_margin(&lens, ENCLOSURE_PIECES[0])
    );
    assert_eq!(decided(ENCLOSURE_PIECES[1]), Ok(Sign::Negative));
    let [out, back] = &lens;
    let (span, p) = ((0.0, 1.0), out.eval(0.0));
    let wound = Body::<f64>::winding_of_steps(
        &[
            (p, Some(((out, span), true))),
            (out.eval(1.0), Some(((back, span), false))),
        ],
        p,
        None,
        false,
        Vec3::unit_z(),
        band,
    );
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

/// A clamped non-rational carrier of `degree` over `interior` knots,
/// its control points a wavy arch `scale` times the unit one and
/// `offset` along each axis, standing off the plane `z = offset`.
fn wavy(degree: usize, interior: &[f64], scale: f64, offset: f64) -> Curve3<f64> {
    let n = degree + 1 + interior.len();
    let mut knots = vec![0.0; degree + 1];
    knots.extend_from_slice(interior);
    knots.extend(vec![1.0; degree + 1]);
    #[allow(clippy::cast_precision_loss)]
    let pts: Vec<[f64; 3]> = (0..n)
        .map(|i| {
            let x = i as f64 / (n - 1) as f64;
            let y = (7.0 * x).sin() + 2.0 * x * (1.0 - x);
            let z = 0.05 * (3.0 * x).cos();
            [x, y, z].map(|c| offset + scale * c)
        })
        .collect();
    spline(knots, degree, &pts, vec![1.0; n])
}

/// **The closed form is the area a dense polygon converges to, and the
/// enclosure holds it, at every scale**: over a multi-span carrier of
/// `degree` with the non-uniform `interior` knots, on arcs that cross
/// knots, lie inside a span and are a hundredth of the domain, at 1 m at
/// the origin and at 1 cm and 100 m a thousand sizes off it. The closed
/// bulge along the normal is the Richardson limit of two inscribed
/// polygons' to `10⁻⁹` of the scale squared, negated for the reversed
/// traversal to the bit; the enclosure at the first count holds it and is
/// under `10⁻⁵` of the scale squared wide; and neither length term is
/// under the polygon's.
fn closed_form_and_enclosure_agree_with_a_polygon(degree: usize, interior: &[f64]) {
    let spans = [(0.0, 1.0), (0.1, 0.52), (0.6, 0.75), (0.5, 0.51)];
    for (scale, offset) in [(0.01, 10.0), (1.0, 0.0), (100.0, 1e5)] {
        let carrier = wavy(degree, interior, scale, offset);
        assert_eq!(segment_kind(&carrier), SegmentKind::Closed);
        for span in spans {
            let name = format!("degree {degree} at {scale} m off {offset}, {span:?}");
            let SegmentTerm { bulge, length } =
                closed_segment_term((&carrier, span), true).unwrap();
            let back = closed_segment_term((&carrier, span), false).unwrap().bulge;
            assert_eq!(back.z.to_bits(), (-bulge.z).to_bits(), "{name}: reversed");
            let ((coarse, _), (fine, polygon)) =
                (polygon(&carrier, span, 512), polygon(&carrier, span, 1024));
            let limit = (4.0 * fine - coarse) / 3.0;
            let unit = scale * scale;
            assert!(
                (bulge.z - limit).abs() <= 1e-9 * unit,
                "{name}: the closed form {} against the polygons' limit {limit}",
                bulge.z
            );
            let (c, lifted_span) = lifted(&carrier, span);
            let SegmentTerm {
                bulge: enclosed,
                length: bound,
            } = SegmentQ::of((&c, lifted_span), true)
                .expect("the enclosure builds")
                .term(ENCLOSURE_PIECES[0])
                .unwrap()
                .expect("a spline has a term");
            let (lo, hi) = (enclosed.z.lo(), enclosed.z.hi());
            // The closed form is rounded: a part in 10¹¹ of the area's
            // scale covers it a thousand sizes off the origin.
            let rounding = 1e-11 * unit;
            assert!(
                lo - rounding <= bulge.z && bulge.z <= hi + rounding && hi - lo < 1e-5 * unit,
                "{name}: the enclosure [{lo}, {hi}] against the closed form {}",
                bulge.z
            );
            assert!(
                polygon <= length && polygon <= bound.lo(),
                "{name}: the length terms {length} and {} against the polygon's {polygon}",
                bound.lo()
            );
        }
    }
}

#[test]
fn a_cubics_closed_form_and_enclosure_agree_with_a_polygon_at_every_scale() {
    closed_form_and_enclosure_agree_with_a_polygon(3, &[0.2, 0.5, 0.55]);
}

#[test]
fn a_quintics_closed_form_and_enclosure_agree_with_a_polygon_at_every_scale() {
    closed_form_and_enclosure_agree_with_a_polygon(5, &[0.3, 0.8]);
}
