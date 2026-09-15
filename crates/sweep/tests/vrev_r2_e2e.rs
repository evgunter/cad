//! **VREV R2 end-to-end exercise.** A reviewer's program written the way
//! a user of PR 2627's door would write one: loft a body, take a face's
//! NURBS chart off it, reverse `v`, put it back with `set_face_surface`,
//! and validate the body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::RawLoop;
use std::sync::Arc;
use topo::{Body, FaceSurface};

fn prism() -> Body<f64> {
    let square = || -> sweep::Section {
        let v = |x: f64, y: f64| profile::ProfileVertex::new(Point2::new(x, y), 0.0);
        vec![profile::ProfileLoop::new(vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ])]
    };
    let sections = vec![square(), square(), square()];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.5, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    sweep::loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("the offset square prism builds")
        .body
}

/// E2E-1. Reverse every NURBS chart on a lofted body in `v`, put each
/// back, and validate. Reports what the door refuses on real lofted
/// geometry and what validation says afterwards.
#[test]
fn e2e_reverse_every_nurbs_chart_and_validate() {
    let mut body = prism();
    assert!(
        topo::validate(&body).is_ok(),
        "the lofted body validates before surgery"
    );

    let faces: Vec<_> = body.faces().map(|(fk, f)| (fk, f.surface)).collect();
    let mut reversed = 0usize;
    let mut refused = 0usize;
    let mut not_nurbs = 0usize;
    let mut placeholder = 0usize;
    for (fk, sk) in faces {
        let n = match body.get_surface(sk) {
            Some(Surface::Nurbs(n)) => (**n).clone(),
            Some(_) => {
                not_nurbs += 1;
                continue;
            }
            None => continue,
        };
        if n.is_placeholder() {
            placeholder += 1;
        }
        match n.reversed_v() {
            Ok(r) => {
                reversed += 1;
                body.set_face_surface(fk, FaceSurface::New(Surface::Nurbs(Arc::new(r))))
                    .expect("a v-reversed chart re-attaches");
            }
            Err(e) => {
                refused += 1;
                println!("E2E-1: face {fk:?} refused: {e}");
            }
        }
    }
    println!(
        "E2E-1: {reversed} charts reversed, {refused} refused, {not_nurbs} non-NURBS, \
         {placeholder} placeholder"
    );
    match topo::validate(&body) {
        Ok(()) => println!("E2E-1: the body still validates structurally"),
        Err(errs) => {
            println!("E2E-1: {} structural validation error(s):", errs.len());
            for e in errs.iter().take(6) {
                println!("  {e:?}");
            }
            panic!("E2E-1: v-reversing charts broke structural validation");
        }
    }
    assert!(reversed > 0, "E2E-1 actually reversed something");
}

/// E2E-2. The geometric half: `set_face_surface` runs no residual
/// certification (its own docs say tier 3 reports it). Does reversing a
/// chart under its face leave the body geometrically valid, or does the
/// door hand the user a body that only LOOKS fine?
#[test]
fn e2e_geometric_validation_after_a_chart_reversal() {
    let tol = Tol::witness();
    let mut body = prism();
    let before = topo::validate_geometric_structural(&body, tol);
    println!(
        "E2E-2: geometric-structural before surgery: {}",
        match &before {
            Ok(()) => "ok".to_string(),
            Err(e) => format!("{} error(s)", e.len()),
        }
    );

    let faces: Vec<_> = body.faces().map(|(fk, f)| (fk, f.surface)).collect();
    let mut done = 0usize;
    for (fk, sk) in faces {
        let Some(Surface::Nurbs(n)) = body.get_surface(sk) else {
            continue;
        };
        let n = (**n).clone();
        if let Ok(r) = n.reversed_v() {
            body.set_face_surface(fk, FaceSurface::New(Surface::Nurbs(Arc::new(r))))
                .expect("re-attaches");
            done += 1;
        }
    }
    let after = topo::validate_geometric_structural(&body, tol);
    match &after {
        Ok(()) => println!("E2E-2: {done} charts reversed; geometric-structural still ok"),
        Err(errs) => {
            println!(
                "E2E-2: {done} charts reversed; geometric-structural now {} error(s):",
                errs.len()
            );
            for e in errs.iter().take(8) {
                println!("  {e:?}");
            }
        }
    }
    println!(
        "E2E-2: before ok = {}, after ok = {}",
        before.is_ok(),
        after.is_ok()
    );
}
