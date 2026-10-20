//! Review probe for PR 5421: a notched cap face (not part of the PR).
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
    .map_err(|p| p.downcast_ref::<String>().cloned().unwrap_or_default())
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
/// the placement's span read `In` beyond the notch. Red at PR 5421's
/// head (a pierce, witnessed inside the notch); main reads nothing.
#[test]
fn review_5421_a_crossing_inside_a_thin_notch_is_no_pierce() {
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
/// side; both ends of the placement's span read `Out` past the strip.
/// Red at PR 5421's head (no pierce); main reads the pierce.
#[test]
fn review_5421_a_crossing_inside_a_thin_strip_is_a_pierce() {
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
/// zero past the band's zero, so the `[P, N]` branch is read, and the
/// chord-form lever, intersected in, reads at most ε. Red at PR 5421's
/// head (`pierce_span`'s `unreachable!`); main refuses.
#[test]
fn review_5421_an_edge_in_a_far_origined_plane_does_not_panic_at_interval() {
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
