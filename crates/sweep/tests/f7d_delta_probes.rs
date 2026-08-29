//! DELTA review probes (ordinal 104 verification pass, PR #1131),
//! sweep side: the repair on a real revolve output, with stored pcurve
//! caches present, and the retire-note's "usable as a boolean operand"
//! condition measured rather than inferred.
//!
//! **ADOPTED** from the delta review's `verbs/f7d-probes`,
//! authorship-preserving. They were written as review-lane probes;
//! they ship because they are the mechanism's differential rows —
//! D1 makes the merge-side comparison RED-CAPABLE, where the
//! shipped `verbs_f7_collinear_seam` row only printed it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Band, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{Body, BooleanOp, boolean_reduce, mint_pcurves, validate_closed, validate_geometric};

fn cone() -> Body<f64> {
    let vp = validated(vec![ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(0.0, 1.0),
    ])]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// D5 — the kev's vertex with STORED PCURVE CACHES beside it. The cone
/// carries a curved wall whose chart mints; mint the caches, then run
/// the repair, then re-validate the caches on the committed result.
/// The op's contract says a cache-carrying input re-mints on the staged
/// clone before commit — this row measures that the promise covers the
/// kef→kev path.
#[test]
fn d5_repair_with_stored_pcurves_stays_cache_clean() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut c = cone();
    mint_pcurves(&mut c, tol).expect("the cone's caches mint");
    let cached = |b: &Body<f64>| {
        b.half_edges()
            .filter(|(k, _)| b.pcurve(*k).is_some())
            .count()
    };
    let rows_before = cached(&c);
    assert!(
        rows_before > 0,
        "the probe needs stored caches to attack with"
    );
    let out = c
        .merge_coplanar_faces(tol)
        .expect("the pole-split cap repairs with caches present");
    assert_eq!(out.groups.len(), 1);
    let findings = topo::pcurves::validate_pcurves(&c, band);
    println!(
        "[d5] pcurve rows {rows_before} -> {}; findings after repair = {findings:?}",
        cached(&c)
    );
    assert!(
        findings.is_empty(),
        "stored caches must re-validate after the kev repair — {findings:?}"
    );
    assert_eq!(validate_closed(&c), Ok(()), "tier 2");
    assert_eq!(validate_geometric(&c, tol), Ok(()), "tier 3");
}

/// D6 — the retire note's first half, measured on the simplest
/// axis-touching revolve: after the authored repair, is the body
/// actually USABLE as a boolean operand (does some boolean accept it),
/// or does it merely fail one door later? Either answer is recorded;
/// what the probe pins is that the F7/maximal-faces door itself no
/// longer answers.
#[test]
fn d6_repaired_cone_operand_door_measured() {
    let tol = Tol::witness();
    let mut c = cone();
    c.merge_coplanar_faces(tol).expect("the cap repairs");
    assert_all_tiers(&c);
    let b = {
        use profile::{Profile, SketchPlane};
        use sweep::{Extrusion, extrude};
        let loop_ =
            ProfileLoop::polygon([p2(-0.5, -0.5), p2(0.5, -0.5), p2(0.5, 0.5), p2(-0.5, 0.5)]);
        let vp = Profile::new(SketchPlane::xy(), vec![loop_])
            .validate(tol)
            .unwrap();
        extrude(&vp, Extrusion::Distance(0.4), tol).unwrap().body
    };
    let res = boolean_reduce(BooleanOp::Union, &c, &b, tol);
    match &res {
        Ok(_) => println!("[d6] union(repaired cone, brick) => Ok — operand fully usable"),
        Err(e) => {
            println!("[d6] union(repaired cone, brick) => {e:?}");
            assert!(
                !matches!(e, topo::BooleanError::NonMaximalFaces { .. }),
                "the F7 door must not answer on a repaired body — {e:?}"
            );
        }
    }
}
