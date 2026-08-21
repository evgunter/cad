//! Error paths: absurd δ (zero/negative/poisoned/infinite), refused
//! `Nurbs` surfaces, and the resolution-overflow guard.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::*;
use mesh::{TessellateError, tessellate};
use geom_core::Tol;

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
