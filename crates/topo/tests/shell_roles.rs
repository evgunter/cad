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

use common::prism_z;
use geom_core::{Sign, Tol};
use topo::{
    Body, BooleanResult, BooleanResultKind, ShellRole, classify_shells, mass_properties, subtract,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// `A ∖ B` with `B` strictly inside `A`: the void birth — one solid,
/// two shells (outer + reverted interior).
fn voided() -> Body<f64> {
    let a = brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r = subtract(&a, &b, Tol::witness()).unwrap();
    let BooleanResult::Body(bb) = r else {
        panic!("the strict-containment subtract yields a voided body")
    };
    assert_eq!(bb.kind, BooleanResultKind::Voided);
    bb.body
}

#[test]
fn cube_is_one_outer_shell() {
    let body = brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    assert_eq!(classes.len(), 1);
    let c = &classes[0];
    assert_eq!(c.role, ShellRole::Outer);
    assert_eq!(c.volume_pad, 0.0);
    // The one shell IS the body: its flux is the whole-body flux,
    // bit-identical (same per-face contributions, same order).
    let props = mass_properties(&body, Tol::witness()).unwrap();
    assert_eq!(c.volume.to_bits(), props.volume.to_bits());
    assert_eq!(c.surface_area.to_bits(), props.surface_area.to_bits());
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
    assert!((outers[0].volume - 27.0).abs() < 1e-12);
    assert!(voids[0].volume < 0.0);
    assert!((voids[0].volume - (-1.0)).abs() < 1e-12);
}

#[test]
fn per_shell_volumes_sum_to_the_body_volume() {
    let body = voided();
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    let props = mass_properties(&body, Tol::witness()).unwrap();
    let sum: f64 = classes.iter().map(|c| c.volume).sum();
    // 27 − 1: the whole-body walk visits the same faces with the same
    // closed forms; only the grouping differs.
    assert!((sum - props.volume).abs() < 1e-12);
    assert!((props.volume - 26.0).abs() < 1e-12);
    let area_sum: f64 = classes.iter().map(|c| c.surface_area).sum();
    assert!((area_sum - props.surface_area).abs() < 1e-12);
}

/// The sign read is the NAMED funnel site: one
/// `chk_shell_volume_sign` verdict per closed-form shell (pad = 0
/// reuses the low-end verdict), signs matching the roles.
#[test]
fn sign_read_is_the_named_decide_site() {
    let body = voided();
    geom_core::k_stats::start_verdict_log();
    let classes = classify_shells(&body, Tol::witness()).unwrap();
    let verdicts = geom_core::k_stats::take_verdict_log();
    let signs: Vec<Sign> = verdicts
        .iter()
        .filter(|v| v.predicate == "chk_shell_volume_sign")
        .map(|v| v.sign)
        .collect();
    assert_eq!(signs.len(), classes.len());
    assert_eq!(signs, vec![Sign::Positive, Sign::Negative]);
}
