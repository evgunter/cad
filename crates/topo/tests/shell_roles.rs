//! The per-shell orientation door (`topo::classify_shells`): signed
//! volume and outer/void role per shell, sharing tier-3 check 7's flux
//! machinery restricted to one shell's faces.
//!
//! Claims pinned here:
//! - a one-shell cube classifies as exactly one `Outer`;
//! - a voided body (`A ∖ B`, `B` strictly inside) classifies as one
//!   `Outer` plus one `Void`, the void's signed volume NEGATIVE;
//! - per-shell signed volumes sum to the whole body's signed volume
//!   (the structural-sharing claim made measurable against
//!   `mass_properties`);
//! - the sign read goes through the named funnel site
//!   (`chk_shell_volume_sign`), one verdict per closed-form shell.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{
    BooleanResult, BooleanResultKind, ShellRole, classify_shells, mass_properties, subtract,
};

/// `A ∖ B` with `B` strictly inside `A`: the void birth — one solid,
/// two shells (outer + reverted interior).
fn voided() -> topo::AtRestBody<f64> {
    let tol = Tol::witness();
    let a = finished(
        "a",
        brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol),
        tol,
    );
    let b = finished(
        "b",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol),
        tol,
    );
    let r = subtract(&a, &b, Tol::witness()).unwrap();
    let BooleanResult::Body(bb) = r else {
        panic!("the strict-containment subtract yields a voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    bb.body
}

#[test]
fn cube_is_one_outer_shell() {
    let body = finished(
        "a cube",
        brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    assert_eq!(classes.len(), 1);
    let c = &classes[0];
    assert_eq!(c.role, ShellRole::Outer);
    assert_eq!(c.enclosure.volume_pad, 0.0);
    // The one shell IS the body: its flux is the whole-body flux,
    // bit-identical (same per-face contributions, same order).
    let props = mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    assert_eq!(
        c.enclosure.volume.to_bits(),
        props.enclosure.volume.to_bits()
    );
    assert_eq!(
        c.enclosure.surface_area.to_bits(),
        props.enclosure.surface_area.to_bits()
    );
}

#[test]
fn voided_body_is_outer_plus_void() {
    let body = voided();
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    assert_eq!(classes.len(), 2);
    let outers: Vec<_> = classes
        .iter()
        .filter(|c| c.role == ShellRole::Outer)
        .collect();
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == ShellRole::Void)
        .collect();
    assert_eq!(outers.len(), 1);
    assert_eq!(voids.len(), 1);
    // Both shells of one solid (a cavity is not a component).
    assert_eq!(outers[0].solid, voids[0].solid);
    // The outer boundary encloses the uncarved brick; the cavity wall
    // integrates NEGATIVE (its loops wind about the outward — i.e.
    // into-the-cavity — normal).
    assert!((outers[0].enclosure.volume - 27.0).abs() < 1e-12);
    assert!(voids[0].enclosure.volume < 0.0);
    assert!((voids[0].enclosure.volume - (-1.0)).abs() < 1e-12);
}

#[test]
fn per_shell_volumes_sum_to_the_body_volume() {
    let body = voided();
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    let props = mass_properties(
        &body,
        Tol::witness(),
        topo::test_support::pinned_resolution(Tol::witness()),
    )
    .unwrap();
    let sum: f64 = classes.iter().map(|c| c.enclosure.volume).sum();
    // 27 − 1: the whole-body walk visits the same faces with the same
    // closed forms; only the grouping differs.
    assert!((sum - props.enclosure.volume).abs() < 1e-12);
    assert!((props.enclosure.volume - 26.0).abs() < 1e-12);
    let area_sum: f64 = classes.iter().map(|c| c.enclosure.surface_area).sum();
    assert!((area_sum - props.enclosure.surface_area).abs() < 1e-12);
}

/// The role read is the NAMED funnel site: each shell's role is read
/// off a ray aimed at one of its faces, its closest crossing decided by
/// `side_at_infinity_order`, and no shell's volume sign is read.
#[test]
fn role_read_is_the_aimed_ray() {
    // Finished before the bracket opens, so it holds the classification's
    // reads alone.
    let body = voided();
    let bracket = geom_core::k_stats::Bracket::open();
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    let read: Vec<&str> = bracket
        .finish()
        .verdicts
        .iter()
        .map(|v| v.predicate)
        .collect();
    assert_eq!(classes.len(), 2);
    assert!(
        read.contains(&"side_at_infinity_order"),
        "the roles are read by aimed rays: {read:?}"
    );
    assert!(
        !read
            .iter()
            .any(|p| p.starts_with("chk_shell_volume_sign") || p.starts_with("positive_volume")),
        "no shell's volume sign is read: {read:?}"
    );
}
