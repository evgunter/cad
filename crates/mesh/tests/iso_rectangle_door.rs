//! **The iso-rectangle SHAPE door in front of the curved lane, and the
//! two questions it separates from the walk-consistency check
//! (issues 727 and 726).**
//!
//! Two bodies reach `tessellate` here that no public construction
//! mints — the boolean refuses `CurvedPierceUnsupported` and
//! `import_step`'s tier 3 refuses `NotIsoRectangle` first — so they are
//! assembled through the Euler doors, the one route the walk's own docs
//! name as fronted by no certification:
//!
//! * **the keyway**: a cylinder wall whose iso domain is a U (a notch
//!   cut into the top rim) — iso-bounded, every edge a rim or a
//!   generator, and not a rectangle;
//! * **the oblique lens**: a sphere face bounded by two plane sections
//!   that are neither coaxial rims nor great circles, meeting off the
//!   axis — the `walk::iso_side_starts` qualification's own case.
//!
//! Per direction: a notched iso domain refuses at the SHAPE door with
//! props' predicate name; the spatial check, asked directly about the
//! same walk, also refuses it (the two derivations agree on a real
//! notch, by a feature-sized distance); the lens refuses at the shape
//! door, and the walk it would otherwise have received collapses onto
//! one rim level and IS its own bounding box — the spatial check admits
//! it, which is the defeat the qualification recorded and the door now
//! closes. A rimless lune (the partial sphere wedge) keeps meshing:
//! the door is the shape predicate, not the flux lane's `Δu = π`
//! premise.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{keyway, oblique_lens};
use common::*;
use geom::Surface;
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;

/// Direction 1 — **a notched iso domain refuses at the SHAPE door, with
/// props' own predicate name**, before any walk runs.
#[test]
fn the_keyway_refuses_at_the_shape_door_by_props_rim_level() {
    let (body, face) = keyway();
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert_eq!(
        got,
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: PropsError::NotIsoRectangle {
                what: "props_rim_level"
            },
        }),
        "a U-shaped iso domain is refused by the ONE named predicate"
    );
}

/// Direction 2 — **the oblique lens refuses at the SHAPE door**: props'
/// sphere classification admits a circle only as a coaxial rim or a
/// great circle, and a tilted section is neither. This is the face the
/// `walk::iso_side_starts` qualification named as the one that could
/// collapse past the spatial check; it never reaches the walk now.
#[test]
fn the_oblique_lens_refuses_at_the_shape_door() {
    let (body, face) = oblique_lens();
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert_eq!(
        got,
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: PropsError::NotIsoRectangle {
                what: "props_rim_axis_parallel"
            },
        }),
        "a tilted plane section of a sphere is not an iso curve; the door refuses it typed"
    );
}

/// The divergence between the SHAPE door and the flux lane, pinned in
/// both directions: the partial sphere wedge is a rimless lune — a
/// chart rectangle `[0, θ] × [−π/2, π/2]` — so the door admits it and
/// it meshes exactly as before, while `mass_properties` refuses the
/// same body for the flux lane's own reason (`Δu = π`,
/// `props_band_coplanar`), which is a closed-form premise and not a
/// statement about the shape.
#[test]
fn a_rimless_lune_meshes_through_the_door_that_the_flux_lane_refuses() {
    let body = sphere_wedge(2.0);
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("the lune meshes");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let sphere_face = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(fk, f)| (fk, f.outer))
        .expect("the wedge has a sphere face");
    let (outer, _) = topo::props::loop_edges(&body, sphere_face.1).unwrap();
    let surface = body
        .get_surface(body.get_face(sphere_face.0).unwrap().surface)
        .unwrap();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        geom_brep::props::require_iso_rectangle(surface, &outer, band),
        Ok(()),
        "a rimless lune is a chart rectangle"
    );
    assert!(
        matches!(
            topo::mass_properties(&body, Tol::witness()),
            Err(topo::MassPropsError::Face {
                source: PropsError::NotIsoRectangle {
                    what: "props_band_coplanar"
                },
                ..
            })
        ),
        "the flux lane refuses the same face for its own Δu = π premise"
    );
}
