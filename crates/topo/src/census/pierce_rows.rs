//! The pierce lane's pinned rows: a grazing edge across a thin notch,
//! strip, slot, hole or disc of a cap face, or through a boundary
//! vertex; an edge in a far-origined plane; and the span's height.
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

/// The half-disc cap `x² + y² ≤ 1`, `y ≥ 0` of the plane `z = 1`: the
/// unit circle's arc from `(1, 0, 1)` through `+y` to `(−1, 0, 1)`, an
/// intersection of a cylinder with the plane, closed by its chord, built
/// through the public operators ([`crate::test_support_fixtures::arc_cap`]
/// at `T`). Returns the body and the cap face.
fn half_disc<T: Decide + crate::props::AtRestPolicy>() -> (Body<T>, FaceKey) {
    use crate::construction::{FaceSurface, MefSite, MevSite};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    let tol = Tol::witness();
    let lift = |x: f64, y: f64, z: f64| Point3::new(x, y, z).map(T::from_f64);
    let (centre, axis, u_ref) = (
        lift(0.0, 0.0, 1.0),
        Vec3::new(T::zero(), T::zero(), T::one()),
        Vec3::new(T::one(), T::zero(), T::zero()),
    );
    let carrier = geom::Curve3::Circle {
        center: centre,
        axis,
        radius: T::one(),
        u_ref,
    };
    let (p0, p1) = (lift(1.0, 0.0, 1.0), lift(-1.0, 0.0, 1.0));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body.add_surface(geom::Surface::Plane {
        origin: centre,
        normal: axis,
        u_ref,
    });
    let wall = body.add_surface(geom::Surface::Cylinder {
        origin: centre,
        axis,
        radius: T::one(),
        u_ref,
    });
    let arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(wall, plane),
                    witness: lift(0.0, 1.0, 1.0),
                },
                carrier,
                param_start: T::zero(),
                param_end: T::pi(),
            },
            tol,
        )
        .unwrap();
    let cap = body
        .mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    (body, cap.face)
}

/// The [`half_disc`] cap at `T`, its stored origin moved to
/// `(30, 30, 1)`, read against the grazing edge [`level_ends`] `(x, y)`
/// at ε = 1e-12: what the pair found, and the predicates its
/// escalations name. The cap is planar and bounded by an arc, so it is
/// no polygon.
fn half_disc_read<T: Decide + crate::props::AtRestPolicy>(
    x: (f64, f64),
    y: f64,
) -> (Found, Vec<&'static str>) {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut body, cap) = half_disc::<T>();
    move_origin(
        &mut body,
        cap,
        Point3::new(30.0, 30.0, 1.0).map(T::from_f64),
    );
    let pair = Pair::new(&body, cap, level_ends(x, y).map(|p| p.map(T::from_f64)));
    assert!(!pair.f().polygon, "the cap is no polygon");
    let errors = pair.read(band);
    let rows = errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusEscalated { cause } => cause.predicate,
            _ => None,
        })
        .collect();
    (found(&errors), rows)
}

/// **A crossing far from a face's curved edge is read a pierce at both
/// scalars**: the [`half_disc`] cap crossed at `(0, 0.5)`, half a metre
/// from its arc and from its chord. Both ends of the millimetre span
/// read `In`, and the arc's clearance from the span's middle passes the
/// span's half-length and its reach.
#[test]
fn a_crossing_far_from_a_curved_edge_is_a_pierce() {
    for (scalar, (got, rows)) in [
        ("f64", half_disc_read::<f64>((-0.25, 0.25), 0.5)),
        ("Interval", half_disc_read::<Interval>((-0.25, 0.25), 0.5)),
    ] {
        assert_eq!(got, Found::Pierce, "{scalar}: refused by {rows:?}");
    }
}

/// **A span grazing a face's curved edge, or crossing it, is never read
/// clear**: the [`half_disc`] cap crossed under its arc's top `(0, 1)`,
/// `5e-12` inside it (in the band), and `4e-7` inside, where the
/// millimetre span crosses the arc twice: its ends stand `Out` past the
/// arc, which falls `8e-7` within a millimetre of the top. Neither span
/// clears the arc, so the half-span's reading, which straddles the band,
/// refuses each (never a containment refusal, which would leave the
/// clearance unread).
#[test]
fn a_span_grazing_or_crossing_a_curved_edge_is_not_read_clear() {
    for (what, y) in [("grazing", 1.0 - 5e-12), ("crossing", 1.0 - 4e-7)] {
        for (scalar, (got, rows)) in [
            ("f64", half_disc_read::<f64>((-0.25, 0.25), y)),
            ("Interval", half_disc_read::<Interval>((-0.25, 0.25), y)),
        ] {
            assert!(
                got == Found::Refused
                    && !rows.is_empty()
                    && rows
                        .iter()
                        .all(|r| *r == "pm_census_ef_pierce_span_straddle"),
                "{what}, {scalar}: read {got:?}, refused by {rows:?}"
            );
        }
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
        let rows: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::CensusEscalated { cause } => cause.predicate,
                _ => None,
            })
            .collect();
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
        let rows: Vec<_> = errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::CensusEscalated { cause } => cause.predicate,
                _ => None,
            })
            .collect();
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

// ---- reviewer r1 probes (PR 5900) ----

/// A cap on plane (normal n, u_ref u) bounded by `carrier` over
/// `[t0, t1]` and its chord, the arc certified as wall ∩ plane.
fn r1_cap(
    carrier: geom::Curve3<f64>,
    (t0, t1): (f64, f64),
    o: Point3<f64>,
    n: Vec3<f64>,
    u: Vec3<f64>,
) -> (Body<f64>, FaceKey) {
    use crate::construction::{FaceSurface, MefSite, MevSite};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    let tol = Tol::witness();
    let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body.add_surface(geom::Surface::Plane {
        origin: o,
        normal: n,
        u_ref: u,
    });
    let wall = body.add_surface(geom::Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    });
    let arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(wall, plane),
                    witness: carrier.eval(0.5 * (t0 + t1)),
                },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            tol,
        )
        .unwrap();
    let cap = body
        .mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    (body, cap.face)
}

fn r1_read(
    body: &Body<f64>,
    face: FaceKey,
    ends: [Point3<f64>; 2],
) -> (Found, Vec<&'static str>) {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let pair = Pair::new(body, face, ends);
    assert!(!pair.f().polygon);
    let errors = pair.read(band);
    let rows = errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::CensusEscalated { cause } => cause.predicate,
            _ => None,
        })
        .collect();
    (found(&errors), rows)
}

#[test]
fn r1_probe_ellipse_equal_to_circle() {
    let c = geom::Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        semi_u: 1.0,
        semi_v: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let (body, face) = r1_cap(
        c,
        (0.0, core::f64::consts::PI),
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        Vec3::unit_x(),
    );
    for y in [0.5, 1.0 - 4e-7, 1.0 - 5e-12] {
        eprintln!(
            "R1 ellipse-circle y={y}: {:?}",
            r1_read(&body, face, level_ends((-0.25, 0.25), y))
        );
    }
}

#[test]
fn r1_probe_tilted_ellipse() {
    let phi: f64 = 0.5;
    let (s, c) = phi.sin_cos();
    let n = Vec3::new(-s, 0.0, c);
    let u = Vec3::new(c, 0.0, s);
    let v = n.cross(u);
    let carrier = geom::Curve3::Ellipse {
        center: Point3::origin(),
        axis: n,
        semi_u: 1.0 / c,
        semi_v: 1.0,
        u_ref: u,
    };
    let (body, face) = r1_cap(carrier, (0.0, core::f64::consts::PI), Point3::origin(), n, u);
    let off = 2f64.powi(-36);
    for y in [0.5, 1.0 - 4e-7, 1.0 - 5e-12] {
        let at = |x: f64, o: f64| Point3::origin() + u * x + v * y + n * o;
        let ends = [at(-0.25, off), at(0.25, -off)];
        eprintln!("R1 tilted ellipse y={y}: {:?}", r1_read(&body, face, ends));
    }
}

#[test]
fn r1_probe_nurbs_half_circle() {
    let h = core::f64::consts::FRAC_1_SQRT_2;
    let knots =
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2)
            .unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(-1.0, 1.0, 1.0),
        Point3::new(-1.0, 0.0, 1.0),
    ];
    let c = geom::Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(knots, control, vec![1.0, h, 1.0, h, 1.0]).unwrap(),
    ));
    let (body, face) = r1_cap(
        c,
        (0.0, 1.0),
        Point3::new(0.0, 0.0, 1.0),
        Vec3::unit_z(),
        Vec3::unit_x(),
    );
    for y in [0.5, 1.0 - 4e-7] {
        eprintln!(
            "R1 nurbs y={y}: {:?}",
            r1_read(&body, face, level_ends((-0.25, 0.25), y))
        );
    }
}

/// Threshold: y chosen so bound − L/2 − reach is k·Kε near the arc top.
#[test]
fn r1_probe_threshold() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (body, cap) = half_disc::<f64>();
    let ends = level_ends((-0.25, 0.25), 1.0 - 1e-3);
    let pair = Pair::new(&body, cap, ends);
    let PierceSpan { t: [lo, hi], lever } = pair.span(band).unwrap();
    let reach = span_reach(&pair.e, [lo, hi], lever, band);
    let half = (hi - lo) * pair.e.len * 0.5;
    eprintln!("R1 threshold: half={half:e} reach={reach:e} lo={lo} hi={hi}");
    for k in [3.0, 2.0, 1.2, 0.8, 0.5, -1.0] {
        let y = 1.0 - (3.145_982_733_45e-5 + reach + k * 1e-11 + 0.0 * half);
        let got = r1_read(&body, cap, level_ends((-0.25, 0.25), y));
        eprintln!("R1 threshold k={k} y={y}: {got:?}");
    }
}

/// A half-disc cap of radius `r` about `(0, 0, 1)` at `T`.
fn r1_small_cap<T: Decide + crate::props::AtRestPolicy>(r: f64) -> (Body<T>, FaceKey) {
    use crate::construction::{FaceSurface, MefSite, MevSite};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    let tol = Tol::witness();
    let lift = |x: f64, y: f64, z: f64| Point3::new(x, y, z).map(T::from_f64);
    let (centre, axis, u_ref) = (
        lift(0.0, 0.0, 1.0),
        Vec3::new(T::zero(), T::zero(), T::one()),
        Vec3::new(T::one(), T::zero(), T::zero()),
    );
    let carrier = geom::Curve3::Circle {
        center: centre,
        axis,
        radius: T::from_f64(r),
        u_ref,
    };
    let (p0, p1) = (carrier.eval(T::zero()), carrier.eval(T::pi()));
    let mut body = Body::<T>::new();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body.add_surface(geom::Surface::Plane {
        origin: centre,
        normal: axis,
        u_ref,
    });
    let wall = body.add_surface(geom::Surface::Cylinder {
        origin: centre,
        axis,
        radius: T::from_f64(r),
        u_ref,
    });
    let arc = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(wall, plane),
                    witness: lift(0.0, r, 1.0),
                },
                carrier,
                param_start: T::zero(),
                param_end: T::pi(),
            },
            tol,
        )
        .unwrap();
    let cap = body
        .mef(
            MefSite::Chords {
                he1: arc.he_minus,
                he2: arc.he_plus,
            },
            EdgeCurveSpec::line_between(p1, p0),
            FaceSurface::Shared {
                key: plane,
                sense: true,
            },
            tol,
        )
        .unwrap();
    (body, cap.face)
}

fn r1_grid<T: Decide + crate::props::AtRestPolicy>(scalar: &str, r: f64, far: bool) {
    let band = Band::new(1e-12, 1e-11).unwrap();
    let (mut body, cap) = r1_small_cap::<T>(r);
    if far {
        move_origin(&mut body, cap, Point3::new(30.0, 30.0, 1.0).map(T::from_f64));
    }
    let (mut pierce, mut nothing, mut refused, mut wrong) = (0, 0, 0, 0);
    let n = 41;
    for i in 0..n {
        for j in 0..n {
            let xc = -1.5 * r + 3.0 * r * (i as f64) / ((n - 1) as f64);
            let y = -0.5 * r + 2.0 * r * (j as f64) / ((n - 1) as f64);
            let ends = level_ends((xc - 0.25, xc + 0.25), y);
            let mid = 0.5 * (ends[0].x + ends[1].x);
            let rho = (mid * mid + y * y).sqrt();
            let depth = (r - rho).min(y); // > 0 inside
            let pair = Pair::new(&body, cap, ends.map(|p| p.map(T::from_f64)));
            let got = found(&pair.read(band));
            match got {
                Found::Pierce => pierce += 1,
                Found::Nothing => nothing += 1,
                Found::Refused => refused += 1,
            }
            let bad = (got == Found::Pierce && depth < -1e-10)
                || (got == Found::Nothing && depth > 1e-10);
            if bad {
                wrong += 1;
                eprintln!("R1 WRONG {scalar} r={r:e} far={far} xc={xc:e} y={y:e} depth={depth:e} got={got:?}");
            }
        }
    }
    eprintln!("R1 grid {scalar} r={r:e} far={far}: pierce={pierce} nothing={nothing} refused={refused} wrong={wrong}");
}

#[test]
fn r1_probe_small_arc_grid() {
    for r in [1e-5, 3e-5, 6e-5, 2e-4] {
        for far in [false, true] {
            r1_grid::<f64>("f64", r, far);
            r1_grid::<Interval>("Interval", r, far);
        }
    }
}
