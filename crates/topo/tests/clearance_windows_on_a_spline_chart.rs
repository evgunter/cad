//! **Which NURBS faces the clearance engine admits a window for**, on
//! the bilinear identity chart `(u, v) ↦ (u, v, 0)` over `[0, 1]²`:
//!
//! - the square lamina whose four edges are the chart's four boundary
//!   isos is the whole patch, and both its faces are admitted;
//! - the triangle lamina, whose third edge is the diagonal, refuses
//!   naming the edge that is not an iso;
//! - the same square on a chart twice as wide refuses: its isos run
//!   round half the domain, one of them an interior column;
//! - the bored block's top cap re-charted onto a patch refuses naming
//!   its inner loops.
//!
//! Every coordinate is dyadic, so each scalar lifts the fixture exactly.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Interval, Point3, Real, Tol};
use topo::clearance::{EngineRefusal, Side, Which};
use topo::construction::{Construct, FaceSurface, MefSite, MevSite};
use topo::{AtRestPolicy, Body, FaceKey};

/// The lamina through `corners` on the identity chart over
/// `[0, width] × [0, 1]` at `T`: `mvfs`, a `mev_line` to each later
/// corner, and a `mef_chord` back to the first, every edge's pcurve
/// minted. Returns the body and its faces.
fn lamina<T: AtRestPolicy>(width: f64, corners: &[(f64, f64)]) -> (Body<T>, Vec<FaceKey>) {
    let tol = Tol::witness();
    let pt = |(x, y): (f64, f64)| Point3::new(T::from_f64(x), T::from_f64(y), T::zero());
    let mut body = Body::<T>::new();
    let seed = body.mvfs(pt(corners[0]), true).unwrap();
    let span = |w: f64| KnotVector::clamped(vec![0.0, 0.0, w, w], 1).unwrap();
    let net = NurbsSurface::new(
        Knots::from(span(width)).lift(Real::from_f64),
        Knots::from(span(1.0)).lift(Real::from_f64),
        [(0.0, 0.0), (0.0, 1.0), (width, 0.0), (width, 1.0)]
            .into_iter()
            .map(pt)
            .collect(),
        vec![Real::one(); 4],
    )
    .unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(net)),
            sense: true,
        },
    )
    .unwrap();
    let mut prev = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(corners[1]),
            tol,
        )
        .unwrap();
    let first = prev.he_plus;
    let mut before = prev.vertex;
    for &corner in &corners[2..] {
        before = prev.vertex;
        prev = body
            .mev_line(
                MevSite::Fan {
                    he1: prev.he_minus,
                    he2: prev.he_minus,
                },
                pt(corner),
                tol,
            )
            .unwrap();
    }
    let back = body.find_half_edge(seed.face, prev.vertex, before).unwrap();
    let closed = body
        .mef_chord(
            MefSite::Chords {
                he1: back,
                he2: first,
            },
            tol,
        )
        .unwrap();
    let faces = vec![seed.face, closed.face];
    topo::mint_pcurves_of(&mut body, &faces, tol).unwrap();
    (body, faces)
}

fn windows<T: AtRestPolicy + geom_core::CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<(), EngineRefusal> {
    Side {
        body,
        faces: vec![face],
    }
    .windows(Which::First)
    .map(|_| ())
}

fn refused_naming<T: AtRestPolicy + geom_core::CertifiedBounds>(
    body: &Body<T>,
    face: FaceKey,
    what: &str,
) {
    match windows(body, face) {
        Err(EngineRefusal::Unsupported {
            carrier,
            face: at,
            side,
        }) => {
            assert_eq!(at, face, "the refusal names the face");
            assert_eq!(
                side,
                Which::First,
                "the refusal names the side it was read as"
            );
            assert!(carrier.contains(what), "{carrier:?} names {what:?}");
        }
        other => panic!("expected a refusal naming {what:?}, got {other:?}"),
    }
}

/// The square's four edges are the chart's four boundary isos: both
/// faces of the lamina are the whole patch.
fn the_iso_square_is_admitted<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    for face in faces {
        assert_eq!(
            windows(&body, face),
            Ok(()),
            "{face:?} at {} has a window",
            T::NAME
        );
    }
}

/// The triangle's diagonal is no iso of the chart.
fn the_triangle_refuses_its_diagonal<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(1.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)]);
    for face in faces {
        refused_naming(&body, face, "not an iso of its chart");
    }
}

/// The square `[0, 1]²` on the chart over `[0, 2] × [0, 1]`: its edges
/// are all isos of the chart, and its right edge, `u = 1`, is an
/// interior column, so the loop runs round half the domain.
fn the_half_square_refuses_as_not_covering<T: AtRestPolicy + geom_core::CertifiedBounds>() {
    let (body, faces) = lamina::<T>(2.0, &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]);
    for face in faces {
        refused_naming(&body, face, "does not run once around its knot domain");
    }
}

#[test]
fn an_iso_square_lamina_is_admitted_at_f64_and_interval() {
    the_iso_square_is_admitted::<f64>();
    the_iso_square_is_admitted::<Interval>();
}

#[test]
fn a_triangle_lamina_refuses_its_diagonal_at_f64_and_interval() {
    the_triangle_refuses_its_diagonal::<f64>();
    the_triangle_refuses_its_diagonal::<Interval>();
}

/// **A trimmed NURBS face refuses, typed**: the bored block's top cap,
/// re-charted onto the patch `(u, v) ↦ (3u, 2v, 2)`, keeps its holes as
/// inner loops.
#[test]
fn a_cap_with_holes_on_a_patch_refuses_as_trimmed() {
    let tol = Tol::witness();
    let mut body = topo::test_support::holed_block::<f64>(3.0, &[0.75, 2.25], tol);
    topo::test_support::describe_as_intersections(&mut body, tol);
    let top = body
        .faces()
        .find(|(_, f)| {
            !f.rings.is_empty()
                && matches!(
                    body.get_surface(f.surface),
                    Some(Surface::Plane { normal, .. }) if normal.z > 0.5
                )
        })
        .map(|(k, _)| k)
        .expect("the top cap holds the holes");
    let unit = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let patch = NurbsSurface::new(
        unit(),
        unit(),
        vec![
            Point3::new(0.0, 0.0, 2.0),
            Point3::new(0.0, 2.0, 2.0),
            Point3::new(3.0, 0.0, 2.0),
            Point3::new(3.0, 2.0, 2.0),
        ],
        vec![1.0; 4],
    )
    .unwrap();
    body.set_face_surface_unvouched_for_tests(
        top,
        FaceSurface::New {
            surface: Surface::Nurbs(Arc::new(patch)),
            sense: true,
        },
    )
    .unwrap();
    refused_naming(&body, top, "trimmed free-form face");
}

#[test]
fn a_square_on_half_its_chart_refuses_as_not_covering_at_f64_and_interval() {
    the_half_square_refuses_as_not_covering::<f64>();
    the_half_square_refuses_as_not_covering::<Interval>();
}
