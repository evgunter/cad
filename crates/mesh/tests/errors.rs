//! Error paths: absurd δ (zero/negative/poisoned/infinite), refused
//! `Nurbs` surfaces, and the resolution-overflow guard.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use geom_core::Tol;
use mesh::{TessellateError, tessellate};

#[test]
fn zero_delta_is_refused() {
    let body = l_prism();
    match tessellate(&body, 0.0, Tol::witness()) {
        Err(TessellateError::InvalidChordalTolerance { value }) => assert_eq!(value, 0.0),
        other => panic!(
            "expected InvalidChordalTolerance, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn negative_delta_is_refused() {
    let body = l_prism();
    match tessellate(&body, -0.5, Tol::witness()) {
        Err(TessellateError::InvalidChordalTolerance { value }) => assert_eq!(value, -0.5),
        other => panic!(
            "expected InvalidChordalTolerance, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn poisoned_delta_is_refused() {
    let body = l_prism();
    match tessellate(&body, f64::NAN, Tol::witness()) {
        Err(TessellateError::InvalidChordalTolerance { value }) => assert!(value.is_nan()),
        other => panic!(
            "expected InvalidChordalTolerance, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn infinite_delta_is_refused() {
    let body = l_prism();
    match tessellate(&body, f64::INFINITY, Tol::witness()) {
        Err(TessellateError::InvalidChordalTolerance { value }) => {
            assert_eq!(value, f64::INFINITY);
        }
        other => panic!(
            "expected InvalidChordalTolerance, got {:?}",
            other.map(|_| ())
        ),
    }
}

#[test]
fn nurbs_surface_is_refused() {
    // The mvfs seed face carries `Surface::Nurbs` (the honest
    // no-description placeholder) — tessellation refuses it typed.
    let mut body = topo::Body::<f64>::new();
    body.mvfs(geom_core::Point3::new(0.0, 0.0, 0.0)).unwrap();
    match tessellate(&body, 0.1, Tol::witness()) {
        Err(TessellateError::UnsupportedSurface { .. }) => {}
        other => panic!("expected UnsupportedSurface, got {:?}", other.map(|_| ())),
    }
}

#[test]
fn absurdly_fine_delta_overflows_typed() {
    // A denormal δ is finite and positive but demands ~10^162 chords
    // per circle — refused before allocating.
    let body = ball();
    match tessellate(&body, 5e-324, Tol::witness()) {
        Err(TessellateError::ResolutionOverflow { count }) => assert!(count > 16_777_216.0),
        other => panic!("expected ResolutionOverflow, got {:?}", other.map(|_| ())),
    }
}

/// The Display contract (#1111): a façade consumer renders a
/// `TessellateError` through the tessellator's own words, so every arm
/// must state what happened in prose — δ, the chart, the walk, the
/// unbuilt lane's note — and must never read as the `Debug` struct
/// dump the Python bindings were reduced to printing. The variant
/// identifier and the field-name punctuation are the dump's
/// fingerprints; asserting their ABSENCE is what keeps a future
/// `write!(f, "{self:?}")` from passing this test.
#[test]
fn tessellate_error_display_names_its_content_not_its_struct() {
    let face = topo::FaceKey::default();
    let edge = topo::EdgeKey::default();
    let cases = [
        (
            TessellateError::InvalidChordalTolerance { value: 0.0 },
            vec!["δ", "positive"],
        ),
        (
            TessellateError::UnsupportedSurface { face },
            vec!["placeholder", "surface"],
        ),
        (
            TessellateError::UnsupportedNurbsFace {
                face,
                note: "a C⁰-creased direction",
            },
            vec!["NURBS face", "C⁰-creased direction"],
        ),
        (
            TessellateError::UnsupportedCurve {
                edge,
                note: "an illegal-rational carrier",
            },
            vec!["carrier", "illegal-rational carrier"],
        ),
        (
            TessellateError::NullScaffoldEdge { edge },
            vec!["scaffolding", "at-rest"],
        ),
        (
            TessellateError::RingOnCurvedFace { face },
            vec!["interior ring", "kernel bug"],
        ),
        (
            TessellateError::EmptyLoop { face },
            vec!["empty loop", "at-rest"],
        ),
        (
            TessellateError::MissingEntity {
                what: "a loop's face back-reference",
            },
            vec!["a loop's face back-reference", "corrupt"],
        ),
        (
            TessellateError::ResolutionOverflow { count: 1e9 },
            vec!["1e9", "coarser"],
        ),
        (
            TessellateError::CertificateExceeded {
                face,
                bound: 2.0,
                requested: 1.0,
            },
            vec!["2e0", "1e0", "uncertified"],
        ),
        (
            TessellateError::Triangulation { face },
            vec!["CDT", "corrupt"],
        ),
        (
            TessellateError::SelfTouchingTrimLoop { face },
            vec!["trim loop", "T-junction"],
        ),
        (
            TessellateError::UnsupportedCurvedDomain {
                face,
                off_bbox: 3,
                first_uv: (0.25, 0.5),
                max_distance: 1e-9,
            },
            vec!["3 walk entries", "2.5e-1", "1e-9", "re-author"],
        ),
        (
            TessellateError::UnsupportedCurvedShape {
                face,
                source: geom_brep::props::PropsError::NotIsoRectangle {
                    what: "props_rim_level",
                },
            },
            vec!["props_rim_level", "iso-parameter rectangle", "quadrature"],
        ),
        (
            TessellateError::Band {
                error: geom_core::BandError::Empty {
                    zero: 1.0,
                    escalate: 1.0,
                },
            },
            vec!["band", "tolerance"],
        ),
    ];
    // The variant identifiers, spelled out: a rendering that leaks one
    // is a struct dump wearing a sentence's clothes.
    let dumps = [
        "InvalidChordalTolerance",
        "UnsupportedSurface",
        "UnsupportedNurbsFace",
        "UnsupportedCurve",
        "NullScaffoldEdge",
        "RingOnCurvedFace",
        "EmptyLoop",
        "MissingEntity",
        "ResolutionOverflow",
        "CertificateExceeded",
        "Triangulation",
        "SelfTouchingTrimLoop",
        "UnsupportedCurvedDomain",
        "UnsupportedCurvedShape",
    ];
    for (err, wants) in cases {
        let shown = err.to_string();
        for want in wants {
            assert!(
                shown.contains(want),
                "{err:?} renders as {shown:?}, missing {want:?}"
            );
        }
        for dump in dumps {
            assert!(
                !shown.contains(dump),
                "{err:?} renders as {shown:?} — that is the variant name, i.e. a struct dump"
            );
        }
        assert!(
            !shown.contains('{') && !shown.contains("face:") && !shown.contains("note:"),
            "{err:?} renders as {shown:?} — that is Debug punctuation, not a sentence"
        );
        assert_ne!(shown, format!("{err:?}"));
    }
}

/// **The failure path's order is ARENA order, not the map's.**
///
/// `tessellate`'s per-face dispatch is D9 idiom 1, so every face's lane
/// runs whatever the first one answers, and the refusal a caller sees
/// is chosen by the fold rather than by whichever worker finished
/// first. The serial loop got that for free by stopping at the first
/// refusing face; here it is a property of the fold and so it is
/// checked.
///
/// The body is the washer with TWO faces poisoned into refusing —
/// differently, so the two refusals are distinguishable — one at the
/// head of the face arena and one at its tail. The answer must be the
/// head's, at every thread count.
#[test]
fn two_faces_refusing_differently_report_the_first_in_arena_order() {
    use geom::Surface;
    use geom_core::{Point3, Vec3};
    use topo::{Body, FaceKey, FaceSurface};

    fn poison(body: &mut Body<f64>, which: usize, surface: Surface<f64>) -> FaceKey {
        let fk = body.faces().nth(which).expect("a face at that index").0;
        body.set_face_surface(fk, FaceSurface::New(surface))
            .expect("the surface swap is accepted");
        fk
    }
    // Two surfaces a washer's face cannot be. Both refuse INSIDE a
    // lane, which is the point: a poison the chord pass refuses (a
    // placeholder NURBS, say) never reaches the map and would test the
    // order of a pass that is still serial.
    let sphere = || Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let cone = || Surface::Cone {
        apex: Point3::new(0.0, 0.0, 4.0),
        axis: Vec3::new(0.0, 0.0, -1.0),
        half_angle: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };

    let clean = washer();
    let n = clean.faces().count();
    assert!(n >= 2, "the row needs a head and a tail face");

    let mut head_only = clean.clone();
    poison(&mut head_only, 0, sphere());
    let head =
        tessellate(&head_only, 0.05, Tol::witness()).expect_err("the poisoned head face refuses");

    let mut tail_only = clean.clone();
    poison(&mut tail_only, n - 1, cone());
    let tail =
        tessellate(&tail_only, 0.05, Tol::witness()).expect_err("the poisoned tail face refuses");
    assert_ne!(
        format!("{head:?}"),
        format!("{tail:?}"),
        "the two poisons must refuse differently, or this row proves nothing"
    );

    let mut both = clean;
    poison(&mut both, 0, sphere());
    poison(&mut both, n - 1, cone());
    for threads in [1usize, 4] {
        let got = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("a rayon pool of the requested width")
            .install(|| tessellate(&both, 0.05, Tol::witness()).expect_err("both faces refuse"));
        assert_eq!(
            format!("{got:?}"),
            format!("{head:?}"),
            "at {threads} thread(s) the reported refusal is the LAST face's, not the \
             first in arena order — the fold is taking the map's order"
        );
    }
}
