//! A grazing edge across a thin notch or strip of a cap face: its rows,
//! and a printing probe (ignored).
use super::*;

fn classify(errors: &[ValidationError]) -> Found {
    if errors.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusEscalated { .. } | ValidationError::CensusUnsupported { .. }
        )
    }) {
        Found::Refused
    } else if errors.is_empty() {
        Found::Nothing
    } else {
        Found::Pierce
    }
}

/// The edge grazes the cap plane `z = 1` and crosses it at `x = 0.5`,
/// `y = 0.45`, inside a V notch of half-width `0.1·w` there (outside
/// the face), its ends at `x = 0.2` and `0.8`.
fn notch_read<T: Decide + crate::props::AtRestPolicy>(
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
fn strip_read<T: Decide + crate::props::AtRestPolicy>(
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
        let s = prism.body.get_face(face).unwrap().surface;
        match prism.body.surfaces.get_mut(s).unwrap() {
            geom::Surface::Plane { origin, .. } => {
                *origin = Point3::new(T::from_f64(far.0), T::from_f64(far.1), T::from_f64(1.0));
            }
            _ => panic!(),
        }
        let geo = snapshot(&prism.body);
        let f = geo.faces.iter().find(|f| f.key == face).unwrap();
        let e = segment((
            Point3::new(0.2, 0.45, 1.0 + off).map(T::from_f64),
            Point3::new(0.8, 0.45, 1.0 - off).map(T::from_f64),
        ));
        let declared = Declared::index(&geo, &ContactRecords::default());
        let mut errors = Vec::new();
        pair_edge_face(&prism.body, &geo, &declared, band, None, &e, f, &mut errors);
        (
            classify(&errors),
            errors
                .iter()
                .map(|e| format!("{e:?}").chars().take(200).collect())
                .collect(),
        )
    })
    .map_err(|_| "panicked (its message is on stderr)".to_string())
}

#[test]
#[ignore = "review probe, prints only: cargo nextest run -p topo --run-ignored all --no-capture -E 'test(/probe_/)'"]
fn probe_notch() {
    let band = Band::new(1e-12, 1e-11).unwrap();
    for (w, far) in [
        (2e-2, (30.0, 30.0)),
        (2e-3, (30.0, 30.0)),
        (2e-4, (30.0, 30.0)),
        (2e-5, (30.0, 30.0)),
        (1e-4, (0.0, 0.0)),
        (1e-5, (0.0, 0.0)),
    ] {
        for off in [1.5e-11, 1e-10] {
            let f = notch_read::<f64>(w, far, off, band);
            let i = notch_read::<Interval>(w, far, off, band);
            println!(
                "[probe] notch w {w:e} (half-width {:e}) far {far:?} off {off:e}: f64 {:?}  Interval {:?}",
                0.1 * w,
                f.as_ref().map(|x| x.0),
                i.as_ref().map(|x| x.0)
            );
            if let Ok((_, errs)) = &f {
                for e in errs {
                    println!("     {e}");
                }
            }
        }
    }
    for (h, far) in [
        (2e-2, (30.0, 30.0)),
        (2e-4, (30.0, 30.0)),
        (2e-5, (30.0, 30.0)),
        (1e-5, (0.0, 0.0)),
        (1e-6, (0.0, 0.0)),
    ] {
        for off in [1.5e-11, 1e-10] {
            let f = strip_read::<f64>(h, far, off, band);
            let i = strip_read::<Interval>(h, far, off, band);
            println!(
                "[probe] strip h {h:e} far {far:?} off {off:e}: f64 {:?}  Interval {:?}",
                f.as_ref().map(|x| x.0),
                i.as_ref().map(|x| x.0)
            );
        }
    }
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
    let ends = super::probe::tilted_ends(&pose, n, -0.063_050_604_584_334_09);
    let got = super::probe::read_ends::<Interval>(o, ends, band);
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
    let s = body.get_face(face).unwrap().surface;
    match body.surfaces.get_mut(s).unwrap() {
        geom::Surface::Plane { origin, .. } => *origin = Point3::new(30.0, 30.0, 1.0),
        _ => panic!("a planar face"),
    }
    let geo = snapshot(&body);
    let f = geo.faces.iter().find(|f| f.key == face).unwrap();
    let e = segment((ends[0], ends[1]));
    let declared = Declared::index(&geo, &ContactRecords::default());
    let mut errors = Vec::new();
    pair_edge_face(&body, &geo, &declared, band, None, &e, f, &mut errors);
    classify(&errors)
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
    let s = probe.body.get_face(probe.top_face).unwrap().surface;
    if let geom::Surface::Plane { origin, .. } = probe.body.surfaces.get_mut(s).unwrap() {
        *origin = Point3::new(30.0, 30.0, 1.0);
    }
    let geo = snapshot(&probe.body);
    let f = geo.faces.iter().find(|f| f.key == probe.top_face).unwrap();
    let e = segment((ends[0], ends[1]));
    let r = [e.p0, e.p1].map(|p| (p - f.origin).dot(f.normal));
    let band = Band::new(1e-12, 1e-11).unwrap();
    let PierceSpan { t: [lo, hi], .. } =
        pierce_span(&e, f, r, band, &mut Vec::new()).expect("a placed crossing");
    let x = |t: f64| (e.p0 + (e.p1 - e.p0) * t).x;
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

/// **A disc narrower than the span is pierced or refused, never read
/// clear**: a disc of radius `2e-4` planted in a brick's top, its
/// stored origin 42 m off, crossed at its centre by an edge whose span
/// is millimetres long. Both ends read `Out` past the disc, and its
/// boundary is a circle, which the span's clearance is not read
/// against.
#[test]
fn a_disc_narrower_than_the_span_is_not_read_clear() {
    use crate::test_support_fixtures::{brick, plant_disc_face};
    let tol = Tol::witness();
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let host = body
        .faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z == 1.0)
        })
        .map(|(k, _)| k)
        .expect("the top");
    let at = body
        .half_edges()
        .find(|(_, h)| body.get_loop(h.parent_loop).unwrap().face == host)
        .map(|(k, _)| k)
        .unwrap();
    let radius = 2e-4;
    let disc = plant_disc_face(&mut body, at, Point3::new(0.5, 0.5, 1.0), radius, tol);
    let got = far_read(
        body,
        disc.face,
        level_ends((0.25, 0.75), 0.5 + 0.5 * radius),
    );
    assert!(
        matches!(got, Found::Pierce | Found::Refused),
        "the crossing inside the disc read clear: {got:?}"
    );
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
        let s = prism.body.get_face(prism.top_face).unwrap().surface;
        match prism.body.surfaces.get_mut(s).unwrap() {
            geom::Surface::Plane { origin, .. } => {
                *origin = Point3::new(1000.0, -1000.0, 1.0).map(T::from_f64);
            }
            _ => panic!("a planar face"),
        }
        let geo = snapshot(&prism.body);
        let f = geo.faces.iter().find(|f| f.key == prism.top_face).unwrap();
        let [p0, p1] = level_ends((0.25, 0.75), 0.45).map(|p| p.map(T::from_f64));
        let e = segment((p0, p1));
        let declared = Declared::index(&geo, &ContactRecords::default());
        let mut errors = Vec::new();
        pair_edge_face(&prism.body, &geo, &declared, band, None, &e, f, &mut errors);
        classify(&errors)
    }
    for (scalar, got) in [("f64", read::<f64>()), ("Interval", read::<Interval>())] {
        assert!(
            matches!(got, Found::Nothing | Found::Refused),
            "{scalar}: a pierce read at the slot's middle: {got:?}"
        );
    }
}
