//! The pierce lane's pinned rows: a grazing edge across a thin notch,
//! strip, slot or hole of a cap face, near or across a half-disc cap's
//! arc, or through a boundary vertex; an edge in a far-origined plane;
//! and the span's height and clearance.
use super::*;
use geom_core::Interval;

/// The edge grazes the cap plane `z = 1` and crosses it at `x = 0.5`,
/// `y = 0.45`, inside a V notch of half-width `0.1·w` there (outside
/// the face), its ends at `x = 0.2` and `0.8`.
pub(super) fn notch_read<T: Decide + crate::props::AtRestPolicy>(
    w: f64,
    far: (f64, f64),
    off: f64,
    band: Band,
) -> Result<(Found, Vec<String>), String> {
    let profile = [
        (0.0, 0.0),
        (0.5 - w, 0.0),
        (0.5, 0.5),
        (0.5 + w, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
    ];
    cap_read::<T>(&profile, far, off, band)
}

/// The same edge over a cap strip `|x − 0.5| < h`: the crossing at its
/// middle, `h` inside it.
pub(super) fn strip_read<T: Decide + crate::props::AtRestPolicy>(
    h: f64,
    far: (f64, f64),
    off: f64,
    band: Band,
) -> Result<(Found, Vec<String>), String> {
    let profile = [
        (0.5 - h, 0.0),
        (0.5 + h, 0.0),
        (0.5 + h, 1.0),
        (0.5 - h, 1.0),
    ];
    cap_read::<T>(&profile, far, off, band)
}

fn cap_read<T: Decide + crate::props::AtRestPolicy>(
    profile: &[(f64, f64)],
    far: (f64, f64),
    off: f64,
    band: Band,
) -> Result<(Found, Vec<String>), String> {
    std::panic::catch_unwind(|| {
        let mut prism =
            crate::test_support_fixtures::prism_z::<T>(profile, 0.0, 1.0, Tol::witness());
        let face = prism.top_face;
        move_origin(
            &mut prism.body,
            face,
            Point3::new(far.0, far.1, 1.0).map(T::from_f64),
        );
        let ends = [
            Point3::new(0.2, 0.45, 1.0 + off),
            Point3::new(0.8, 0.45, 1.0 - off),
        ];
        let errors = Pair::new(&prism.body, face, ends.map(|p| p.map(T::from_f64))).read(band);
        (
            found(&errors),
            errors
                .iter()
                .map(|e| format!("{e:?}").chars().take(200).collect())
                .collect(),
        )
    })
    .map_err(|_| "panicked (its message is on stderr)".to_string())
}

/// **A grazing edge crossing a face's plane inside a thin notch is no
/// pierce**: the crossing stands `1e-5` (`2e-4` with the origin 42 m
/// off) inside the notch, far past `2Kε` from its sides; both ends of
/// the placement's span read `In` beyond the notch, so they answer
/// only once the notch's sides are read clear of the span.
#[test]
fn a_crossing_inside_a_thin_notch_is_no_pierce() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    for (w, far) in [(1e-4, (0.0, 0.0)), (2e-3, (30.0, 30.0))] {
        for (scalar, got) in [
            ("f64", notch_read::<f64>(w, far, 1.5e-11, band).map(|x| x.0)),
            (
                "Interval",
                notch_read::<Interval>(w, far, 1.5e-11, band).map(|x| x.0),
            ),
        ] {
            assert!(
                matches!(got, Ok(Found::Nothing | Found::Refused)),
                "notch half-width {:e}, origin {far:?}, {scalar}: {got:?}",
                0.1 * w
            );
        }
    }
}

/// **A grazing edge crossing a thin strip face's plane at the strip's
/// middle pierces it**: `2e-4` (`1e-5` near the origin) inside each
/// side; both ends of the placement's span read `Out` past the strip,
/// so they answer only once its sides are read clear of the span.
#[test]
fn a_crossing_inside_a_thin_strip_is_a_pierce() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    for (h, far, off) in [(2e-4, (30.0, 30.0), 1.5e-11), (1e-5, (0.0, 0.0), 1.5e-11)] {
        for (scalar, got) in [
            ("f64", strip_read::<f64>(h, far, off, band).map(|x| x.0)),
            (
                "Interval",
                strip_read::<Interval>(h, far, off, band).map(|x| x.0),
            ),
        ] {
            assert!(
                matches!(got, Ok(Found::Pierce | Found::Refused)),
                "strip half-width {h:e}, origin {far:?}, {scalar}: {got:?}"
            );
        }
    }
}

/// **An edge lying in a face's plane, the plane's origin 950 m off,
/// does not panic at `Interval`**: both residuals' enclosures straddle
/// zero past the band's zero, so the `[P, N]` branch is read, and a
/// lever that reads at most ε there rules that branch out.
#[test]
fn an_edge_in_a_far_origined_plane_does_not_panic_at_interval() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let pose = Pose {
        far: (-947.176_348_895_219_8, 88.292_226_138_031_99),
        depth: 0.174_060_838_769_796_22,
        off: -3.667_677_545_687_027e-13,
        across: 0.234_866_643_068_815_6,
    };
    let (o, n) = plane(&pose);
    let ends = tilted_ends(&pose, n, -0.063_050_604_584_334_09);
    let got = read_ends::<Interval>(o, ends, band);
    assert!(got.is_ok(), "{got:?}");
}

/// The ends `x₀`, `x₁` of a grazing edge along `x` at height `y` over
/// the cap `z = 1`, `2⁻³⁶` either side of it (both exact), so the
/// crossing is at `(x₀ + x₁)/2` exactly.
fn level_ends(x: (f64, f64), y: f64) -> [Point3<f64>; 2] {
    let off = 2f64.powi(-36);
    [
        Point3::new(x.0, y, 1.0 + off),
        Point3::new(x.1, y, 1.0 - off),
    ]
}

/// The pair read on `face` of `body`, its plane's stored origin first
/// moved to `(30, 30, 1)`, at ε = 1e-12.
fn far_read(mut body: Body<f64>, face: FaceKey, ends: [Point3<f64>; 2]) -> Found {
    let band = Band::new(1e-12, 1e-11).unwrap();
    move_origin(&mut body, face, Point3::new(30.0, 30.0, 1.0));
    found(&Pair::new(&body, face, ends).read(band))
}

/// **A crossing on a boundary vertex the span passes through is no
/// pierce**: a V notch hangs from the cap's far edge to its apex at
/// `(0.5, 0.45)`, and the edge crosses the plane exactly there. Both
/// ends of the millimetre span read `In` and no boundary edge's
/// interior meets the span's, so only the apex's own reading on the
/// span keeps the ends from answering for the crossing.
#[test]
fn a_crossing_on_a_boundary_vertex_inside_the_span_is_no_pierce() {
    let profile = [
        (0.0, 0.0),
        (1.0, 0.0),
        (1.0, 1.0),
        (0.6, 1.0),
        (0.5, 0.45),
        (0.4, 1.0),
        (0.0, 1.0),
    ];
    let prism = crate::test_support_fixtures::prism_z::<f64>(&profile, 0.0, 1.0, Tol::witness());
    let got = far_read(prism.body, prism.top_face, level_ends((0.25, 0.75), 0.45));
    assert!(
        matches!(got, Found::Nothing | Found::Refused),
        "a pierce read at the notch's apex: {got:?}"
    );
}

/// **A strip whose two sides hold the span's two ends is pierced or
/// refused, never read clear**: the strip's sides are placed at the
/// ends of the span the crossing is read on, so both ends read on the
/// boundary, on two edges, while the crossing stands half the span
/// (a millimetre) inside.
#[test]
fn a_strip_whose_sides_hold_the_spans_ends_is_not_read_clear() {
    let ends = level_ends((0.25, 0.75), 0.45);
    let strip = |a: f64, b: f64| [(a, 0.0), (b, 0.0), (b, 1.0), (a, 1.0)];
    let mut probe =
        crate::test_support_fixtures::prism_z::<f64>(&strip(0.4, 0.6), 0.0, 1.0, Tol::witness());
    move_origin(
        &mut probe.body,
        probe.top_face,
        Point3::new(30.0, 30.0, 1.0),
    );
    let band = Band::new(1e-12, 1e-11).unwrap();
    let PierceSpan { t: [lo, hi], .. } = Pair::new(&probe.body, probe.top_face, ends)
        .span(band)
        .expect("a placed crossing");
    let x = |t: f64| (ends[0] + (ends[1] - ends[0]) * t).x;
    assert!(x(hi) - x(lo) > 1e-4, "a span of {:e}", x(hi) - x(lo));
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &strip(x(lo), x(hi)),
        0.0,
        1.0,
        Tol::witness(),
    );
    let got = far_read(prism.body, prism.top_face, ends);
    assert!(
        matches!(got, Found::Pierce | Found::Refused),
        "the crossing half a span inside the strip read clear: {got:?}"
    );
}

/// The body's level planar face whose stored origin stands at `z`.
fn top_at(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z == z)
        })
        .map(|(k, _)| k)
        .expect("a level face at z")
}

/// The predicates `errors`' escalations name.
fn escalations(errors: &[ValidationError]) -> Vec<&'static str> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusEscalated { cause } => cause.predicate,
            _ => None,
        })
        .collect()
}

/// The half-disc cap of `radius` about `(cx, cy, 1)` in the plane
/// `z = 1`, on the `+y` side of its chord: the circle's arc there, an
/// intersection of a cylinder with the plane, closed by its chord
/// ([`crate::test_support_fixtures::arc_cap`] at `T`). Its plane's
/// stored origin is then moved to `(ox, oy, 1)`.
fn half_disc<T: Decide + crate::props::AtRestPolicy>(
    (cx, cy): (f64, f64),
    radius: f64,
    (ox, oy): (f64, f64),
) -> (Body<T>, FaceKey) {
    let carrier = geom::Curve3::Circle {
        center: Point3::new(cx, cy, 1.0).map(T::from_f64),
        axis: Vec3::new(T::zero(), T::zero(), T::one()),
        radius: T::from_f64(radius),
        u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
    };
    let (mut body, cap) = crate::test_support_fixtures::arc_cap(
        carrier,
        (T::zero(), T::pi()),
        Vec3::new(T::zero(), T::zero(), T::zero()),
        Tol::witness(),
    );
    move_origin(&mut body, cap, Point3::new(ox, oy, 1.0).map(T::from_f64));
    (body, cap)
}

/// What the [`half_disc`] cap at `T` and the grazing edge [`level_ends`]
/// `(x, y)` find at ε = 1e-12, and the predicates its escalations name.
/// The cap is planar and bounded by an arc, so it is no polygon.
fn half_disc_read<T: Decide + crate::props::AtRestPolicy>(
    centre: (f64, f64),
    radius: f64,
    origin: (f64, f64),
    (x, y): ((f64, f64), f64),
) -> (Found, Vec<&'static str>) {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (body, cap) = half_disc::<T>(centre, radius, origin);
    let pair = Pair::new(&body, cap, level_ends(x, y).map(|p| p.map(T::from_f64)));
    assert!(!pair.f().polygon, "the cap is no polygon");
    let errors = pair.read(band);
    (found(&errors), escalations(&errors))
}

/// Whether a pair refused through the half-span's reading alone, which
/// straddles the band wherever the span is not read clear.
fn refused_by_the_span(got: Found, rows: &[&str]) -> bool {
    got == Found::Refused
        && !rows.is_empty()
        && rows
            .iter()
            .all(|r| *r == "pm_census_ef_pierce_span_straddle")
}

/// **A crossing far from a face's curved edge is read a pierce at both
/// scalars**: the unit [`half_disc`] cap crossed half a metre from its
/// arc and from its chord, by a millimetre span whose ends both read
/// `In`, so the arc's clearance from the span's middle passes its
/// half-length and its reach. The cap stands about its centre `(0, 0)`
/// and at `(1000, 0)`, its plane's stored origin 42 m from it each time.
#[test]
fn a_crossing_far_from_a_curved_edge_is_a_pierce() {
    for (cx, ox) in [(0.0, 30.0), (1000.0, 1030.0)] {
        let x = (cx - 0.25, cx + 0.25);
        for (scalar, (got, rows)) in [
            (
                "f64",
                half_disc_read::<f64>((cx, 0.0), 1.0, (ox, 30.0), (x, 0.5)),
            ),
            (
                "Interval",
                half_disc_read::<Interval>((cx, 0.0), 1.0, (ox, 30.0), (x, 0.5)),
            ),
        ] {
            assert_eq!(
                got,
                Found::Pierce,
                "at {cx} m, {scalar}: refused by {rows:?}"
            );
        }
    }
}

/// **A span grazing a face's curved edge, or crossing it, is never read
/// clear**: the unit [`half_disc`] cap crossed under its arc's top
/// `(0, 1)`, `5e-12` inside it (in the band), and `4e-7` inside, where
/// the millimetre span crosses the arc twice: its ends stand `Out` past
/// the arc, which falls `8e-7` within a millimetre of the top. Neither
/// span clears the arc, so the half-span's reading refuses each (never a
/// containment refusal, which would leave the clearance unread).
#[test]
fn a_span_grazing_or_crossing_a_curved_edge_is_not_read_clear() {
    for (what, y) in [("grazing", 1.0 - 5e-12), ("crossing", 1.0 - 4e-7)] {
        let at = ((-0.25, 0.25), y);
        for (scalar, (got, rows)) in [
            (
                "f64",
                half_disc_read::<f64>((0.0, 0.0), 1.0, (30.0, 30.0), at),
            ),
            (
                "Interval",
                half_disc_read::<Interval>((0.0, 0.0), 1.0, (30.0, 30.0), at),
            ),
        ] {
            assert!(
                refused_by_the_span(got, &rows),
                "{what}, {scalar}: read {got:?}, refused by {rows:?}"
            );
        }
    }
}

/// The span `[lo, hi]` of the chord [`pierce_span`] places for the
/// grazing edge [`level_ends`] `(x, y)` over the plane `z = 1` whose
/// stored origin is `(ox, oy, 1)`, as the `x` its ends stand at, beside
/// the pair's [`PierceSpan`] and edge.
fn span_over(
    (x, y): ((f64, f64), f64),
    origin: (f64, f64),
) -> ((f64, f64), PierceSpan<f64>, EdgeGeo<f64>) {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (body, cap) = half_disc::<f64>((x.0, y - 10.0), 1.0, origin);
    let ends = level_ends(x, y);
    let pair = Pair::new(&body, cap, ends);
    let span = pair.span(band).expect("a placed crossing");
    let at = |t: f64| ends[0].x + (ends[1].x - ends[0].x) * t;
    ((at(span.t[0]), at(span.t[1])), span, pair.e)
}

/// **A curved edge is read from the span's middle**: a half-disc of
/// radius `1e-4` whose arc the millimetre span crosses twice, `r/2` over
/// its chord, centred a quarter span past the span's middle towards
/// either end. The crossing, at the middle, is outside the disc, and both
/// ends read `Out`. The arc stands `L/4 − r` from the middle, short of
/// `L/2`, so the span is not read clear; from the far end it would stand
/// `3L/4 − r`, past `L/2`, which a bound read there would clear.
#[test]
fn a_curved_edge_meeting_one_half_of_the_span_is_not_read_clear() {
    let (x, y, origin) = ((-0.25, 0.25), 1.0, (30.0, 30.0));
    let ((lo, hi), ..) = span_over((x, y), origin);
    let (mid, length, radius) = ((lo + hi) / 2.0, hi - lo, 1e-4);
    assert!(length > 10.0 * radius, "a span of {length:e}");
    for side in [-1.0, 1.0] {
        let centre = (mid + side * length / 4.0, y - radius / 2.0);
        for (scalar, (got, rows)) in [
            ("f64", half_disc_read::<f64>(centre, radius, origin, (x, y))),
            (
                "Interval",
                half_disc_read::<Interval>(centre, radius, origin, (x, y)),
            ),
        ] {
            assert!(
                refused_by_the_span(got, &rows),
                "the disc towards {side}, {scalar}: read {got:?}, refused by {rows:?}"
            );
        }
    }
}

/// **A curved edge clears the span only past its reach**: a half-disc
/// of radius 0.5 about `(1000, 0)`, its plane's stored origin 42 m off,
/// crossed by a millimetre span whose middle stands `L/2 + Kε + reach/2`
/// under the arc's top, at ε = 1e-12 (K = 10). A metre from the origin
/// the span's rounding charge `c` passes ε, so the reach is a few ε, and
/// the arc's bound from the middle passes `L/2` by more than `Kε` but
/// `L/2 + reach` by less: the span is not read clear. Every point of the
/// arc stands past `L/2 − 2e-6` from the span, so the crossing pierces
/// the cap; the refusal is the price of a bound read from one point.
#[test]
fn a_curved_edge_clears_the_span_only_past_its_reach() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (centre, radius, origin) = ((1000.0, 0.0), 0.5, (1030.0, 30.0));
    let x = (999.75, 1000.25);
    let at = |y: f64| {
        let ((lo, hi), PierceSpan { t, lever }, e) = span_over((x, y), origin);
        let reach = span_reach(&e, t, lever, band);
        let half = (t[1] - t[0]) * e.len / 2.0;
        ((lo + hi) / 2.0, half, reach)
    };
    let mut y = radius / 2.0;
    for _ in 0..4 {
        let (_, half, reach) = at(y);
        y = centre.1 + radius - (half + band.escalate() + reach / 2.0);
    }
    let (mid, half, reach) = at(y);
    let carrier = geom::Curve3::Circle {
        center: Point3::new(centre.0, centre.1, 1.0),
        axis: Vec3::unit_z(),
        radius,
        u_ref: Vec3::unit_x(),
    };
    let bound = carrier.clearance(Point3::new(mid, y, 1.0)).unwrap();
    let past = (bound - half - band.escalate()) / reach;
    assert!(
        reach > 2.0 * band.zero() && (0.25..0.75).contains(&past),
        "the bound passes L/2 + Kε by {past} of the reach {:.2}ε",
        reach / band.zero()
    );
    let (got, rows) = half_disc_read::<f64>(centre, radius, origin, (x, y));
    assert!(
        refused_by_the_span(got, &rows),
        "read {got:?}, refused by {rows:?}"
    );
}

/// **A side with no closed-form clearance is not read clear**: the
/// clearance read on the unit square cap `z = 1` over the middle of a
/// level edge across it, as at [`a_spans_reach_counts_the_band_and_the_slope`],
/// once its side `x = 1` is restated as a degree-1 spline along itself
/// and once as null scaffolding (a carrier no clearance reads). Both
/// read `Some(false)`; with the line side the same span reads clear.
#[test]
fn a_side_with_no_closed_form_clearance_is_not_read_clear() {
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::spline::{KnotVector, Knots};
    let band = Band::new(1e-12, 1e-11).unwrap();
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let ends = [Point3::new(0.25, 0.5, 1.0), Point3::new(0.75, 0.5, 1.0)];
    let clear = |body: &Body<f64>, face: FaceKey| {
        let pair = Pair::new(body, face, ends);
        let lever = 0.01 * pair.e.len;
        let half = 5.0 * band.zero() / lever / 2.0;
        let mut errors = Vec::new();
        let read = span_clear(
            pair.body,
            &pair.geo,
            &pair.e,
            pair.f(),
            [0.5 - half, 0.5 + half],
            lever,
            band,
            &mut errors,
        );
        assert!(errors.is_empty(), "refused {errors:?}");
        read
    };
    let restated = |null: bool| {
        let mut prism =
            crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, Tol::witness());
        let body = &mut prism.body;
        let (key, plus, minus, q0, q1) = body
            .edges()
            .find_map(|(k, e)| {
                let v0 = body.get_half_edge(e.he_plus)?.start;
                let v1 = body.half_edge_end(e.he_plus)?;
                let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                let (p0, p1): (Point3<f64>, Point3<f64>) = (p(v0), p(v1));
                (p0.x == 1.0 && p1.x == 1.0 && p0.z == 1.0 && p1.z == 1.0)
                    .then_some((k, e.he_plus, e.he_minus, p0, p1))
            })
            .expect("the cap's side at x = 1");
        if null {
            let (v0, v1) = (
                body.get_half_edge(plus).unwrap().start,
                body.half_edge_end(plus).unwrap(),
            );
            let curve = body.edges.get(key).unwrap().curve;
            *body.curves.get_mut(curve).unwrap() =
                crate::null::CurveGeom::NullScaffold(crate::null::NullEdge {
                    below_end: v0,
                    above_end: v1,
                });
        } else {
            let surface = |he| {
                let f = body.face_of_half_edge(he).unwrap();
                body.get_face(f).unwrap().surface
            };
            let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let spline = geom::NurbsCurve3::new(Knots::from(knots), vec![q0, q1], vec![1.0; 2])
                .expect("the side as a degree-1 spline");
            let spec = EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(surface(plus), surface(minus)),
                    selector: geom_brep::CurveSelector::Witness(q0.lerp(q1, 0.5)),
                },
                carrier: geom::Curve3::Nurbs(std::sync::Arc::new(spline)),
                param_start: 0.0,
                param_end: 1.0,
            };
            body.set_edge_curve(key, spec, Tol::witness())
                .expect("a degree-1 spline along the side certifies");
        }
        prism
    };
    let prism = crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, Tol::witness());
    assert_eq!(
        clear(&prism.body, prism.top_face),
        Some(true),
        "the line side"
    );
    for (what, null) in [("a spline side", false), ("a null side", true)] {
        let prism = restated(null);
        assert_eq!(clear(&prism.body, prism.top_face), Some(false), "{what}");
    }
}

/// **A span standing off the plane past the band is not read clear**: a
/// slot `|x − 0.5| < 0.03` cut into the cap from its far edge, the edge
/// crossing the plane exactly at the slot's middle, the plane's stored
/// origin moved to `(1000, −1000, 1)`, at ε = 2.5e-13 and K = 4. The
/// residuals' charge, about `2.5e-12`, lifts the span's ends that far
/// off the plane, so the slot's sides stand past `Kε` from the span in
/// 3D, though their projections cross it. Both ends read `In`; only the
/// span's height keeps them from answering for the crossing.
#[test]
fn a_span_standing_off_the_plane_past_the_band_is_not_read_clear() {
    fn read<T: Decide + crate::props::AtRestPolicy>() -> Found {
        let band = Band::new(2.5e-13, 1e-12).unwrap();
        let slot = [
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (0.53, 1.0),
            (0.53, 0.3),
            (0.47, 0.3),
            (0.47, 1.0),
            (0.0, 1.0),
        ];
        let mut prism = crate::test_support_fixtures::prism_z::<T>(&slot, 0.0, 1.0, Tol::witness());
        let face = prism.top_face;
        move_origin(
            &mut prism.body,
            face,
            Point3::new(1000.0, -1000.0, 1.0).map(T::from_f64),
        );
        let ends = level_ends((0.25, 0.75), 0.45).map(|p| p.map(T::from_f64));
        found(&Pair::new(&prism.body, face, ends).read(band))
    }
    for (scalar, got) in [("f64", read::<f64>()), ("Interval", read::<Interval>())] {
        assert!(
            matches!(got, Found::Nothing | Found::Refused),
            "{scalar}: a pierce read at the slot's middle: {got:?}"
        );
    }
}

/// The edge's ends, tilted along the plane by `tilt` of its length.
pub(super) fn tilted_ends(pose: &Pose, normal: Vec3<f64>, tilt: f64) -> [Point3<f64>; 2] {
    let [(x0, y0), (x1, y1)] = [PROFILE[0], PROFILE[1]];
    let along = normal.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
    let aim = Point3::new(
        x0 + (x1 - x0) * pose.across,
        y0 + (y1 - y0) * pose.across,
        1.0 - pose.depth,
    );
    let half = Vec3::new(0.0, 0.0, 0.5) + along * (0.5 * tilt);
    [
        aim - half + normal * pose.off,
        aim + half - normal * pose.off,
    ]
}

/// The pair read at `T` on side face 0, its stored origin at `o`, a
/// panic caught.
pub(super) fn read_ends<T: Decide + crate::props::AtRestPolicy>(
    o: Point3<f64>,
    ends: [Point3<f64>; 2],
    band: Band,
) -> Result<Found, String> {
    std::panic::catch_unwind(|| {
        let mut prism =
            crate::test_support_fixtures::prism_z::<T>(&PROFILE, 0.0, 1.0, Tol::witness());
        let face = prism.side_faces[0];
        let lift = |p: Point3<f64>| p.map(T::from_f64);
        move_origin(&mut prism.body, face, lift(o));
        found(&Pair::new(&prism.body, face, ends.map(lift)).read(band))
    })
    .map_err(|_| "panicked (its message is on stderr)".to_string())
}

/// **A hole narrower than the span is no pierce**: the block
/// `[0, 3] × [0, 2] × [0, 2]` holed at `x ∈ [1, 2]`, `y ∈ [0.5, 1.5]`,
/// its top's stored origin 140 m off, an 80 m edge grazing the top
/// along `y = 1` and crossing it at the hole's centre. The span holds
/// the hole, both its ends on material (`In`), and stands within ε of
/// the plane; only the hole's ring keeps the ends from answering for a
/// crossing that is in the hole.
#[test]
fn a_hole_narrower_than_the_span_is_no_pierce() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let top = top_at(
        &crate::test_support_fixtures::holed_block::<f64>(3.0, &[1.5], Tol::witness()),
        2.0,
    );
    fn go<T: Decide + crate::props::AtRestPolicy>(
        top: FaceKey,
        off: f64,
        band: Band,
    ) -> (Found, Option<PierceSpan<T>>, [Point3<T>; 2]) {
        let mut body = crate::test_support_fixtures::holed_block::<T>(3.0, &[1.5], Tol::witness());
        move_origin(
            &mut body,
            top,
            Point3::new(100.0, -100.0, 2.0).map(T::from_f64),
        );
        let ends = [
            Point3::new(-38.5, 1.0, 2.0 + off).map(T::from_f64),
            Point3::new(41.5, 1.0, 2.0 - off).map(T::from_f64),
        ];
        let pair = Pair::new(&body, top, ends);
        let f = pair.f();
        assert!(f.polygon && f.loops.len() == 2, "a polygon with one ring");
        (found(&pair.read(band)), pair.span(band), ends)
    }
    for off in [1.2e-11, 1.5e-11] {
        let (f64_read, span, [p0, p1]) = go::<f64>(top, off, band);
        let PierceSpan { t: [lo, hi], lever } = span.expect("a placed crossing");
        let x = |t: f64| p0.x + (p1.x - p0.x) * t;
        assert!(
            (0.1..0.9).contains(&x(lo)) && (2.1..2.9).contains(&x(hi)),
            "off {off:e}: the span [{:.3}, {:.3}] holds the hole, its ends on material",
            x(lo),
            x(hi)
        );
        let height = lever * (hi - lo);
        assert!(
            height < 0.8 * band.zero(),
            "off {off:e}: the span stands {height:e} off"
        );
        let (iv_read, ..) = go::<Interval>(top, off, band);
        for (scalar, got) in [("f64", f64_read), ("Interval", iv_read)] {
            assert_ne!(
                got,
                Found::Pierce,
                "off {off:e}, {scalar}: a pierce in the hole"
            );
        }
    }
}

/// **A span whose reach to the boundary under it passes `(K − 1)ε`, by
/// under twice that, is refused by its height; one well within it is
/// read**: an edge grazing the unit square cap `z = 1` at its middle,
/// its plane's stored origin moved off along the plane, at ε = 1e-12.
/// At 2.1 km the residuals' charges lift the span's ends about `13ε`
/// off the plane, so a boundary point under it could stand about `14ε`
/// away, and the reading refuses; at 30 m that reach is under `1.5ε`,
/// and the crossing, half a metre inside, is read a pierce.
#[test]
fn a_span_standing_past_its_reach_off_the_plane_is_refused_by_its_height() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let read = |far: f64| {
        let mut prism =
            crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, Tol::witness());
        let face = prism.top_face;
        move_origin(&mut prism.body, face, Point3::new(far, -far, 1.0));
        let pair = Pair::new(&prism.body, face, level_ends((0.25, 0.75), 0.5));
        let PierceSpan { t, lever } = pair.span(band).expect("a placed crossing");
        let reach = span_reach(&pair.e, t, lever, band);
        let errors = pair.read(band);
        let rows = escalations(&errors);
        (reach / band.zero(), found(&errors), rows)
    };
    let (reach, got, rows) = read(1500.0);
    assert!(
        (10.0..17.0).contains(&reach),
        "the far span reaches {reach}ε, not between 10ε and 17ε"
    );
    assert!(
        got == Found::Refused
            && !rows.is_empty()
            && rows
                .iter()
                .all(|r| r.starts_with("pm_census_ef_pierce_height")),
        "a span reaching {reach}ε read {got:?}, refused by {rows:?}"
    );
    let (reach, got, _) = read(21.0);
    assert!(reach < 1.5, "the near span reaches {reach}ε");
    assert_eq!(got, Found::Pierce, "a span reaching {reach}ε");
}

/// **A span's reach counts the ε a boundary point stands off the plane
/// and the slope that moves a vertex's foot along it**: the clearance
/// read on the unit square cap `z = 1` over the middle of a level edge
/// across it, the boundary a quarter metre off, at ε = 1e-12 (K = 10),
/// the lever chosen to give the span a height `h` and the edge a slope
/// `σ`. With `(h + ε)(1 + σ)` within `9ε` it reads clear; past it the
/// height rows refuse, at `h = 8.5ε, σ = 0.01` only through the ε and
/// at `h = 7.5ε, σ = 0.5` only through the slope.
#[test]
fn a_spans_reach_counts_the_band_and_the_slope() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let prism = crate::test_support_fixtures::prism_z::<f64>(&square, 0.0, 1.0, Tol::witness());
    let ends = [Point3::new(0.25, 0.5, 1.0), Point3::new(0.75, 0.5, 1.0)];
    let pair = Pair::new(&prism.body, prism.top_face, ends);
    let clear = |height: f64, slope: f64| {
        let lever = slope * pair.e.len;
        let half = height * band.zero() / lever / 2.0;
        let mut errors = Vec::new();
        let read = span_clear(
            pair.body,
            &pair.geo,
            &pair.e,
            pair.f(),
            [0.5 - half, 0.5 + half],
            lever,
            band,
            &mut errors,
        );
        let rows = escalations(&errors);
        (read, rows)
    };
    assert_eq!(clear(5.0, 0.01), (Some(true), vec![]), "h = 5ε, σ = 0.01");
    for (height, slope) in [(8.5, 0.01), (7.5, 0.5)] {
        let (read, rows) = clear(height, slope);
        assert!(
            read.is_none()
                && !rows.is_empty()
                && rows
                    .iter()
                    .all(|r| r.starts_with("pm_census_ef_pierce_height")),
            "h = {height}ε, σ = {slope}: read {read:?}, refused by {rows:?}"
        );
    }
}

/// **A span overhanging its edge's start is read over the overhang**:
/// the clearance read on the unit square cap `z = 1` at both scalars,
/// over a window `[lo, hi]` of an edge's chord with `lo < 0`, a boundary
/// meeting the window only at `s < 0`. Across the level edge
/// `(0.25, 0.5) → (0.75, 0.5)`, the window `[−0.6, −0.4]` holds the
/// side `x = 0` at `t = −0.5`; along the diagonal `(0.25, 0.25) →
/// (0.75, 0.75)`, the window `[−0.6, 0.2]` holds the corner `(0, 0)`
/// at `t = −0.5`. Each is a meeting, so neither reads clear.
#[test]
fn a_span_overhanging_its_edges_start_is_read_over_the_overhang() {
    fn clear<T: Decide + crate::props::AtRestPolicy>(
        ends: [(f64, f64); 2],
        window: [f64; 2],
    ) -> Option<bool> {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let square = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let prism = crate::test_support_fixtures::prism_z::<T>(&square, 0.0, 1.0, Tol::witness());
        let ends = ends.map(|(x, y)| Point3::new(x, y, 1.0).map(T::from_f64));
        let pair = Pair::new(&prism.body, prism.top_face, ends);
        let lever = T::from_f64(1e-13);
        let mut errors = Vec::new();
        let read = span_clear(
            pair.body,
            &pair.geo,
            &pair.e,
            pair.f(),
            window.map(T::from_f64),
            lever,
            band,
            &mut errors,
        );
        assert!(
            errors.is_empty(),
            "{ends:?} over {window:?}: refused {errors:?}"
        );
        read
    }
    for (what, ends, window) in [
        ("the side x = 0", [(0.25, 0.5), (0.75, 0.5)], [-0.6, -0.4]),
        (
            "the corner (0, 0)",
            [(0.25, 0.25), (0.75, 0.75)],
            [-0.6, 0.2],
        ),
    ] {
        for (scalar, read) in [
            ("f64", clear::<f64>(ends, window)),
            ("Interval", clear::<Interval>(ends, window)),
        ] {
            assert_eq!(read, Some(false), "{scalar}: {what} in the overhang");
        }
    }
}

/// **A grazing edge far inside a far-origined face is read a pierce at
/// both scalars**: side face 0's edge crossed 0.3 m below its top, its
/// stored origin 1, 30 and 300 m off along its plane, the edge's slope
/// off it from `0.2` down to `2e-5`, at ε = 1e-12. The placement's span
/// is read on the edge's own line, so at [`Interval`] its rounded ends
/// do not turn it.
#[test]
fn a_grazing_edge_far_inside_a_far_origined_face_is_read_pierced_at_both_scalars() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    for far in [1.0, 30.0, 300.0] {
        for off in [1e-1, 3e-2, 1e-2, 3e-3, 1e-3, 1e-4, 1e-5] {
            let pose = Pose {
                far: (far, far),
                depth: 0.3,
                off,
                across: 0.5,
            };
            for (scalar, got) in [
                ("f64", read::<f64>(&pose, band)),
                ("Interval", read::<Interval>(&pose, band)),
            ] {
                assert_eq!(got, Found::Pierce, "{scalar}, {pose:?}");
            }
        }
    }
}
