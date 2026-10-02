//! DELTA review probes (ordinal 104 verification pass, PR #1131),
//! sweep side: a real axis-touching revolve output with stored pcurve
//! caches present, and the retire-note's "usable as a boolean operand"
//! condition measured rather than inferred. The full revolve builds its
//! base disc as ONE face (`crates/sweep/README.md`, "Walls: one per
//! run"), so there is no pole-split cap left to repair; the merge's own
//! kef→kev rows are `topo`'s.
//!
//! **ADOPTED** from the delta review's `verbs/f7d-probes`,
//! authorship-preserving; `topo`'s `f7d_delta_probes` carries the
//! hand-built rows (D1–D4).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use geom_core::{Band, Point2, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{Body, BooleanOp, boolean_reduce, mint_pcurves, validate_closed, validate_geometric};

fn cone() -> Body<f64> {
    let vp = validated(vec![ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// D5 — the cone with STORED PCURVE CACHES: its base disc is built
/// whole, with no pole vertex at its centre, so the structural rung
/// finds nothing to merge and the caches stay valid through the call.
#[test]
fn d5_whole_cap_with_stored_pcurves_stays_cache_clean() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut c = cone();
    mint_pcurves(&mut c, tol).expect("the cone's caches mint");
    let rows_before = c
        .half_edges()
        .filter(|(k, _)| c.pcurve(*k).is_some())
        .count();
    assert!(rows_before > 0, "the probe needs stored caches");
    assert_eq!(
        (c.vertices().count(), c.faces().count()),
        (3, 3),
        "apex, two rim vertices; cone halves and one base disc"
    );
    let out = c.merge_coplanar_faces(tol).expect("nothing to repair");
    assert!(out.groups.is_empty(), "{:?}", out.groups);
    let findings = topo::pcurves::validate_pcurves(&c, band);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(validate_closed(&c), Ok(()), "tier 2");
    assert_eq!(validate_geometric(&c, tol), Ok(()), "tier 3");
}

/// D6 — the retire note's first half, measured on the simplest
/// axis-touching revolve as built: is the body actually USABLE as a
/// boolean operand (does some boolean accept it), or does it merely
/// fail one door later? Either answer is recorded; what the probe pins
/// is that the F7/maximal-faces door itself does not answer.
#[test]
fn d6_built_cone_operand_door_measured() {
    let tol = Tol::witness();
    let c = cone();
    assert_all_tiers(&c);
    let b = {
        use profile::{Profile, SketchPlane};
        use sweep::{Extrusion, extrude};
        let loop_ = ProfileLoop::polygon([
            Point2::new(-0.5, -0.5),
            Point2::new(0.5, -0.5),
            Point2::new(0.5, 0.5),
            Point2::new(-0.5, 0.5),
        ]);
        let vp = Profile::new(SketchPlane::xy(), vec![loop_])
            .validate(tol)
            .unwrap();
        extrude(&vp, Extrusion::Distance(0.4), tol).unwrap().body
    };
    let res = boolean_reduce(BooleanOp::Union, &c, &b, tol);
    match &res {
        Ok(_) => println!("[d6] union(cone, brick) => Ok — operand fully usable"),
        Err(e) => {
            println!("[d6] union(cone, brick) => {e:?}");
            assert!(
                !matches!(e, topo::BooleanError::NonMaximalFaces { .. }),
                "the F7 door must not answer on a built cone — {e:?}"
            );
        }
    }
}
