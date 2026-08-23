//! VERBS-RING review probes (reviewer lane r1): adversarial holed
//! full revolves through the public API, beside the PR's own
//! acceptance suite (`revolve_ring.rs`).
//!
//! Rows:
//! - the ratified DEFINITION checked as an oracle: the one-call holed
//!   revolve's mass properties equal revolve(outer) minus
//!   revolve(hole-as-outer), each built separately;
//! - a hole hugging the outer boundary at a gap scaled to THIS run's
//!   committed eps (the tangent-close attack — the gap is decidable,
//!   so the build must succeed and match closed forms at every eps
//!   the matrix draws, 1e-6 and 1e-12 alike);
//! - a hole hugging the axis: below the decidable band the PROFILE
//!   refuses (which is what makes `HoleTouchesAxis` defensive-only
//!   in-process), above it the ring builds and matches closed forms;
//! - the degenerate-arm pin on a fixture the boolean operand gate
//!   would ACCEPT (planes + cylinders): a reroute of holed revolves
//!   through the crossing pipeline turns this red even where the
//!   torus-wall structural argument does not apply;
//! - a wire outer (axis-touching) with a CIRCULAR hole — the wire ×
//!   torus-cavity composition the PR suite does not cover.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::Tol;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use revolve_common::*;
use sweep::{Revolution, revolve};

const PI: f64 = core::f64::consts::PI;

/// The ratified definition as an oracle: one-call holed revolve ==
/// revolve(outer) − revolve(hole-as-outer), in mass properties.
/// An inside-out cavity (the winding inversion done wrong) adds the
/// hole volume instead of subtracting it and turns this red.
#[test]
fn one_call_equals_the_defining_composition() {
    let tol = Tol::witness();
    let outer = || ProfileLoop::polygon([p2(1.0, 0.0), p2(3.0, 0.0), p2(3.0, 2.0), p2(1.0, 2.0)]);
    let hole = || ProfileLoop::polygon([p2(1.5, 0.5), p2(2.5, 0.5), p2(2.5, 1.5), p2(1.5, 1.5)]);

    let holed = revolve(
        &validated(vec![outer(), hole()]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap();
    let solid_outer = revolve(&validated(vec![outer()]), axis_y(), Revolution::Full, tol).unwrap();
    let hole_as_outer = revolve(&validated(vec![hole()]), axis_y(), Revolution::Full, tol).unwrap();

    assert_all_tiers(&holed.body);
    let p = topo::mass_properties(&holed.body, tol).unwrap();
    let po = topo::mass_properties(&solid_outer.body, tol).unwrap();
    let ph = topo::mass_properties(&hole_as_outer.body, tol).unwrap();
    // Volume: exactly the definition. Area: both boundaries count.
    assert!(((p.volume - (po.volume - ph.volume)) / p.volume).abs() < 1e-12);
    assert!(
        ((p.surface_area - (po.surface_area + ph.surface_area)) / p.surface_area).abs() < 1e-12
    );
}

/// A hole whose boundary hugs the outer boundary at 1000·eps on all
/// four sides — decidably clear at every eps (K = 10), so the build
/// must succeed and hit the Pappus forms whether the run draws
/// eps = 1e-6 or eps = 1e-12.
#[test]
fn hole_hugging_the_outer_boundary_at_this_eps() {
    let tol = Tol::witness();
    let g = 1000.0 * eps();
    let (x0, x1, y0, y1) = (1.0, 2.0, 0.0, 1.0);
    let outer = ProfileLoop::polygon([p2(x0, y0), p2(x1, y0), p2(x1, y1), p2(x0, y1)]);
    let hole = ProfileLoop::polygon([
        p2(x0 + g, y0 + g),
        p2(x1 - g, y0 + g),
        p2(x1 - g, y1 - g),
        p2(x0 + g, y1 - g),
    ]);
    let t = revolve(
        &validated(vec![outer, hole]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);
    // Pappus, cancellation-free algebra: 1 − (1−2g)² = 4g(1−g), all
    // at r̄ 1.5. The kernel's flux sum carries ~1e-16 absolute error
    // from the near-cancelling outer/cavity contributions, so the
    // relative gate is 1e-6 of the (tiny) result — still red many
    // orders of magnitude before any winding/containment mistake
    // (an inside-out cavity lands near 2·2π·1.5 ≈ 18.8).
    let v_expect = 2.0 * PI * 1.5 * (4.0 * g * (1.0 - g));
    let p = topo::mass_properties(&t.body, tol).unwrap();
    assert!(
        ((p.volume - v_expect) / v_expect).abs() < 1e-6,
        "{}",
        p.volume
    );
}

/// A hole hugging the AXIS. Below the decidable band the profile
/// itself refuses (hole-vs-outer clearance is in-band or touching) —
/// this is the guard that makes `HoleTouchesAxis` unreachable
/// in-process, pinned here so a validation loosening shows up as a
/// red row in THIS suite. Above the band the ring builds and hits
/// the closed forms.
#[test]
fn near_axis_hole_refuses_below_the_band_and_builds_above_it() {
    let tol = Tol::witness();
    let e = eps();
    // Outer's left edge ON the axis; the hole 5·eps away from it:
    // inside the ambiguity band (K = 10) — the profile must refuse
    // typed (touching or escalated), never validate.
    let outer = || ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let tight = ProfileLoop::polygon([
        p2(5.0 * e, 0.25),
        p2(0.5, 0.25),
        p2(0.5, 0.75),
        p2(5.0 * e, 0.75),
    ]);
    assert!(
        Profile::new(SketchPlane::xy(), vec![outer(), tight])
            .validate(tol)
            .is_err(),
        "a hole 5·eps off the axis-touching outer edge must not validate"
    );
    // 1000·eps away: decidable — the wire-case ring builds, with the
    // cavity's inner wall a cylinder of radius 1000·eps.
    let g = 1000.0 * e;
    let hole = ProfileLoop::polygon([p2(g, 0.25), p2(0.5, 0.25), p2(0.5, 0.75), p2(g, 0.75)]);
    let t = revolve(
        &validated(vec![outer(), hole]),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.cavities.len(), 1);
    // Pappus by hand: outer A=1 at r̄=0.5 → 2π·0.5; hole A=(0.5−g)·0.5
    // at r̄=(g+0.5)/2.
    let v_hole = 2.0 * PI * ((g + 0.5) / 2.0) * ((0.5 - g) * 0.5);
    let v_expect = 2.0 * PI * 0.5 - v_hole;
    let p = topo::mass_properties(&t.body, tol).unwrap();
    assert!(
        ((p.volume - v_expect) / v_expect).abs() < 1e-9,
        "{}",
        p.volume
    );
}

/// The degenerate-arm pin on a fixture the boolean operand gate would
/// ACCEPT (planes + cylinders only): unlike the annulus/torus fixture,
/// a reroute of THIS build through `subtract` would actually run — and
/// would log `bool_`-prefixed predicates, turning the row red.
#[test]
fn boolean_admissible_fixture_still_runs_no_crossing_machinery() {
    let tol = Tol::witness();
    let outer = ProfileLoop::polygon([p2(1.0, 0.0), p2(3.0, 0.0), p2(3.0, 3.0), p2(1.0, 3.0)]);
    let h1 = ProfileLoop::polygon([p2(1.25, 0.5), p2(2.75, 0.5), p2(2.75, 1.0), p2(1.25, 1.0)]);
    let h2 = ProfileLoop::polygon([p2(1.25, 1.5), p2(2.0, 1.5), p2(2.0, 2.5), p2(1.25, 2.5)]);
    let vp = validated(vec![outer, h1, h2]);
    geom_core::k_stats::start_verdict_log();
    let t = revolve(&vp, axis_y(), Revolution::Full, tol).unwrap();
    let verdicts = geom_core::k_stats::take_verdict_log();
    assert!(!verdicts.is_empty());
    let crossing: Vec<_> = verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("bool_"))
        .collect();
    assert!(crossing.is_empty(), "{crossing:?}");
    assert_eq!(t.cavities.len(), 2);
    assert_all_tiers(&t.body);
}

/// Wire outer (axis contact) × circular hole: the torus cavity rides
/// inside a two-π-band wire build — the composition the PR suite's
/// wire row (square hole) does not cover.
#[test]
fn wire_outer_with_circular_hole_torus_cavity() {
    let tol = Tol::witness();
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 3.0), p2(0.0, 3.0)]);
    let hole = profile::circle(p2(1.0, 1.5), 0.4, tol).unwrap();
    let vp = validated(vec![outer, hole.into()]);
    let t = revolve(&vp, axis_y(), Revolution::Full, tol).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);
    // V = cylinder (r=2, h=3) − torus (Rc=1, r=0.4)
    //   = 12π − 2π²·1·0.16.
    let v_expect = 12.0 * PI - 2.0 * PI * PI * 0.16;
    let a_torus = 4.0 * PI * PI * 1.0 * 0.4;
    let a_cyl = 2.0 * PI * 2.0 * 3.0 + 2.0 * PI * 4.0;
    let p = topo::mass_properties(&t.body, tol).unwrap();
    assert!(
        ((p.volume - v_expect) / v_expect).abs() < 1e-12,
        "{}",
        p.volume
    );
    assert!(
        ((p.surface_area - (a_cyl + a_torus)) / p.surface_area).abs() < 1e-12,
        "{}",
        p.surface_area
    );
}
