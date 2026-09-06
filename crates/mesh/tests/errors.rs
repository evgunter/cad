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
