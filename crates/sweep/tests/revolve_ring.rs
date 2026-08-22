//! VERBS-RING acceptance: the one-call hollow ring — a full revolve of
//! a holed profile, DEFINED as `revolve(outer) − revolve(hole-as-outer)`
//! (OFFSET-DESIGN O4) and executed as the degenerate no-crossing arm:
//! each hole revolves as its own solid of revolution and its reversed
//! boundary is inserted as a cavity through the shared void-insertion
//! door (`topo::insert_void`), with containment carried from the
//! profile's own validated 2-D margins.
//!
//! The suite pins:
//! - the two-shell result (outer + one cavity per hole), tier-3 valid,
//!   with the census and shell count stated;
//! - mass properties against independently derived closed forms
//!   (Pappus for the polygon fixtures, the torus forms for the
//!   annulus) — outer minus hole, which is also the orientation
//!   oracle for the reversed cavity boundary;
//! - **the degenerate-arm claim, structurally**: the construction
//!   fires not one `bool_`-named predicate (every crossing-pipeline
//!   decision — reduction sweep, sector classification, containment
//!   probes — reports under that prefix), pinned through the verdict
//!   log. Rerouting the ring through the boolean pipeline turns this
//!   RED twice over: the probes would log `bool_point_in_solid_*`,
//!   and the annulus ring's torus walls cannot even pass the boolean
//!   operand gate.

mod revolve_common;

use geom_core::Tol;
use profile::ProfileLoop;
use revolve_common::*;
use sweep::{Revolution, RevolvedKind, revolve};

fn washer() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)])
}

/// The square-holed washer, fully revolved: a rectangular-section ring
/// with a rectangular-section toroidal cavity.
#[test]
fn one_call_hollow_ring() {
    let hole = ProfileLoop::polygon([
        p2(1.25, 0.25),
        p2(1.75, 0.25),
        p2(1.75, 0.75),
        p2(1.25, 0.75),
    ]);
    let vp = validated(vec![washer(), hole]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);

    // Shell census: exactly two shells in one solid — the outer
    // boundary and the cavity, and the handle bundle names both.
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);
    assert_ne!(t.cavities[0], t.shell);
    let cavity = t.body.get_shell(t.cavities[0]).unwrap();
    let outer = t.body.get_shell(t.shell).unwrap();
    assert_eq!(cavity.solid, t.solid);
    assert_eq!(outer.solid, t.solid);
    assert_eq!(cavity.faces.len(), 4);
    assert_eq!(outer.faces.len(), 4);

    // Entity census: each shell is a 4-segment lamina ring — 4 walls,
    // 4 meridians, 4 full-period rim self-loops, 4 vertices, no rings.
    assert_eq!(counts(&t.body), (8, 16, 8, 0));

    // The handle bundle covers the hole loop with result-body keys.
    assert!(t.walls[1].iter().all(Option::is_some));
    assert!(t.rims[1].iter().all(Option::is_some));
    assert!(t.poles[1].iter().all(Option::is_none));
    let RevolvedKind::Full { meridians, .. } = &t.kind else {
        panic!("full revolve")
    };
    assert_eq!(meridians.len(), 2);
    assert!(meridians[1].iter().all(Option::is_some));
    // The revolve's seam conventions hold inside the cavity exactly as
    // on the outer shell: meridians of periodic walls (the two
    // cylinders) re-describe as `Seam`; meridians of plane annuli
    // honestly keep `MappedCurve` (a plane chart is not periodic).
    let hole_seams = meridians[1]
        .iter()
        .filter(|m| {
            matches!(
                description(&t.body, m.unwrap()),
                geom_brep::EdgeGeometry::Seam { .. }
            )
        })
        .count();
    assert_eq!(hole_seams, 2);

    // Mass properties = outer minus hole, both derived independently
    // by Pappus (2π·r̄·A for volumes, 2π·r̄·L per wall for areas):
    //   outer: A = 1·1 at r̄ = 1.5           → V₊ = 2π·1.5
    //   hole:  A = 0.5·0.5 at r̄ = 1.5       → V₋ = 2π·0.375
    //   outer walls: 2π(1.5 + 2 + 1.5 + 1)  = 2π·6
    //   hole walls:  2π·0.5·(1.5+1.75+1.5+1.25) = 2π·3
    let pi = core::f64::consts::PI;
    let props = topo::mass_properties(&t.body, Tol::witness()).unwrap();
    let v_expect = 2.0 * pi * (1.5 - 0.375);
    let a_expect = 2.0 * pi * 9.0;
    assert!(((props.volume - v_expect) / v_expect).abs() < 1e-12);
    assert!(((props.surface_area - a_expect) / a_expect).abs() < 1e-12);
    assert_eq!(props.volume_pad, 0.0);
    assert_eq!(props.area_pad, 0.0);
}

/// The annulus ring (klein wall 6's shape): two concentric circles
/// fully revolved — a torus with a toroidal cavity — pinned as the
/// degenerate no-crossing arm THROUGH THE VERDICT LOG: not one
/// `bool_`-named predicate fires. This fixture doubles the structural
/// claim: its walls are tori, which the boolean operand gate refuses
/// outright, so no reroute through the crossing pipeline could even
/// start.
#[test]
fn hollow_torus_runs_no_crossing_machinery() {
    let tol = Tol::witness();
    let (rc, ro, ri) = (5.0, 0.5, 0.35);
    let outer = profile::circle(p2(rc, 0.0), ro, tol).unwrap();
    let inner = profile::circle(p2(rc, 0.0), ri, tol).unwrap();
    let vp = validated(vec![outer.into(), inner.into()]);

    geom_core::k_stats::start_verdict_log();
    let t = revolve(&vp, axis_y(), Revolution::Full, tol).unwrap();
    let verdicts = geom_core::k_stats::take_verdict_log();
    assert!(
        !verdicts.is_empty(),
        "the construction decides through the funnel"
    );
    let crossing: Vec<_> = verdicts
        .iter()
        .filter(|v| v.predicate.starts_with("bool_"))
        .collect();
    assert!(
        crossing.is_empty(),
        "the ring is the degenerate no-crossing arm: no crossing-pipeline \
         predicate may fire, got {crossing:?}"
    );

    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);

    // Torus closed forms (independent): V = 2π²·Rc·(ro² − ri²),
    // A = 4π²·Rc·(ro + ri).
    let pi = core::f64::consts::PI;
    let props = topo::mass_properties(&t.body, tol).unwrap();
    let v_expect = 2.0 * pi * pi * rc * (ro * ro - ri * ri);
    let a_expect = 4.0 * pi * pi * rc * (ro + ri);
    assert!(((props.volume - v_expect) / v_expect).abs() < 1e-12);
    assert!(((props.surface_area - a_expect) / a_expect).abs() < 1e-12);
}

/// A multi-hole profile: one cavity per hole, each with its own
/// closed-form contribution.
#[test]
fn two_holes_two_cavities() {
    let outer = ProfileLoop::polygon([p2(1.0, 0.0), p2(3.0, 0.0), p2(3.0, 3.0), p2(1.0, 3.0)]);
    let h1 = ProfileLoop::polygon([p2(1.5, 0.5), p2(2.5, 0.5), p2(2.5, 1.0), p2(1.5, 1.0)]);
    let h2 = ProfileLoop::polygon([p2(1.5, 1.5), p2(2.0, 1.5), p2(2.0, 2.5), p2(1.5, 2.5)]);
    let vp = validated(vec![outer, h1, h2]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 3);
    assert_eq!(t.cavities.len(), 2);
    assert_ne!(t.cavities[0], t.cavities[1]);
    assert_eq!(t.walls.len(), 3);

    // Pappus, hole by hole: outer 2·3 at r̄ 2; h1 1·0.5 at r̄ 2;
    // h2 0.5·1 at r̄ 1.75.
    let pi = core::f64::consts::PI;
    let props = topo::mass_properties(&t.body, Tol::witness()).unwrap();
    let v_expect = 2.0 * pi * (6.0 * 2.0 - 0.5 * 2.0 - 0.5 * 1.75);
    assert!(((props.volume - v_expect) / v_expect).abs() < 1e-12);
}

/// An axis-touching OUTER with a hole: the outer builds as the
/// two-π-band wire (poles at the run's ends), the hole still inserts
/// as a lamina cavity — the case split composes.
#[test]
fn wire_outer_with_hole_cavity() {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 3.0), p2(0.0, 3.0)]);
    let hole = ProfileLoop::polygon([p2(0.5, 1.0), p2(1.5, 1.0), p2(1.5, 2.0), p2(0.5, 2.0)]);
    let vp = validated(vec![outer, hole]);
    let t = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
    assert_eq!(t.body.shells().count(), 2);
    assert_eq!(t.cavities.len(), 1);
    let RevolvedKind::Full {
        pi_walls,
        meridians,
        ..
    } = &t.kind
    else {
        panic!("full revolve")
    };
    // The outer is the wire case (π-band walls exist for its off-axis
    // segments); the hole is lamina (its meridians all present).
    assert!(pi_walls.iter().any(Option::is_some));
    assert!(meridians[1].iter().all(Option::is_some));

    // Pappus: outer cylinder solid 2·3 at r̄ 1 minus hole 1·1 at r̄ 1.
    let pi = core::f64::consts::PI;
    let props = topo::mass_properties(&t.body, Tol::witness()).unwrap();
    let v_expect = 2.0 * pi * (6.0 * 1.0 - 1.0 * 1.0);
    assert!(((props.volume - v_expect) / v_expect).abs() < 1e-12);
}

/// D9 determinism: the holed full revolve replays byte-identically
/// (two builds of the same input agree entity-for-entity on counts
/// and mass properties bit-for-bit).
#[test]
fn holed_full_revolve_replays() {
    let hole = ProfileLoop::polygon([
        p2(1.25, 0.25),
        p2(1.75, 0.25),
        p2(1.75, 0.75),
        p2(1.25, 0.75),
    ]);
    let build = || {
        let vp = validated(vec![washer(), hole.clone()]);
        revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap()
    };
    let (a, b) = (build(), build());
    assert_eq!(counts(&a.body), counts(&b.body));
    let pa = topo::mass_properties(&a.body, Tol::witness()).unwrap();
    let pb = topo::mass_properties(&b.body, Tol::witness()).unwrap();
    assert_eq!(pa.volume.to_bits(), pb.volume.to_bits());
    assert_eq!(pa.surface_area.to_bits(), pb.surface_area.to_bits());
}
