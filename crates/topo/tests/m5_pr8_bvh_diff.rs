//! M5 PR 8 — the idealized/realized differential suite for the
//! BVH-backed reduction sweep (PERF-PLAN §4.4, C10: the pattern is
//! only permitted WITH this suite). Three pins over the boolean demo
//! bodies (brick scenarios from the M3 acceptance heritage; the Band 4
//! corpus rides the editor-core twin of this suite):
//!
//! 1. **Superset**: realized candidates ⊇ idealized ACCEPTED pairs,
//!    per sweep direction (supersets allowed, misses fatal).
//! 2. **Bit-equal results**: full ops through `Realized` and
//!    `Idealized` produce byte-identical `Debug` forms (D9).
//! 3. **Planted degradation**: one face box planted empty loses that
//!    face's accepted pairs and the superset comparator CATCHES it —
//!    the suite is able to fail.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::BTreeSet;

use common::prism_z;
use geom_core::Decide;
use geom_core::Tol;
use topo::{
    Body, BooleanOp, BooleanResult, PlantedDegradation, SweepStrategy, SweepTrace, boolean_op_with,
    sweep_traces, sweep_traces_with_pad,
};

type Pair = (topo::EdgeKey, topo::FaceKey);

fn brick<T: Decide + geom_core::Bounds>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// The suite's superset comparator (pins 1 and 3): idealized-accepted
/// pairs missing from the realized candidate set. Any entry is a lost
/// event — fatal.
fn missing_pairs(realized: &SweepTrace, idealized: &SweepTrace) -> Vec<Pair> {
    let examined: BTreeSet<Pair> = realized.examined.iter().copied().collect();
    idealized
        .accepted
        .iter()
        .copied()
        .filter(|p| !examined.contains(p))
        .collect()
}

/// The demo-body scenarios: (name, A, B) brick pairs from the M3
/// acceptance heritage — crossing, flush-stacked, corner kiss,
/// disjoint, nested.
fn scenarios() -> Vec<(&'static str, Body<f64>, Body<f64>)> {
    vec![
        (
            "crossing bricks",
            brick((0.0, 4.0), (0.0, 2.0), (0.0, 2.0)),
            brick((1.0, 3.0), (-1.0, 3.0), (1.0, 3.0)),
        ),
        (
            "flush-stacked bricks",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
            brick((0.0, 2.0), (0.0, 2.0), (1.0, 2.0)),
        ),
        (
            "corner kiss",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
        ),
        (
            "disjoint bricks",
            brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0)),
        ),
        (
            "nested bricks",
            brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0)),
            brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
        ),
        (
            "skew edge cross",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((1.0, 3.0), (1.0, 3.0), (-0.5, 2.5)),
        ),
    ]
}

/// Pin 1: realized candidate set ⊇ idealized accepted set, both
/// directions, every scenario.
#[test]
fn realized_candidates_superset_of_idealized_accepted() {
    for (name, a, b) in scenarios() {
        let (r_ab, r_ba) =
            sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
        let (i_ab, i_ba) =
            sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
        assert_eq!(
            missing_pairs(&r_ab, &i_ab),
            vec![],
            "{name}: A→B lost accepted pairs"
        );
        assert_eq!(
            missing_pairs(&r_ba, &i_ba),
            vec![],
            "{name}: B→A lost accepted pairs"
        );
        // The realized examined set is itself a subsequence of the
        // idealized one (same exact path, fewer candidates).
        let i_ex: BTreeSet<Pair> = i_ab.examined.iter().copied().collect();
        assert!(
            r_ab.examined.iter().all(|p| i_ex.contains(p)),
            "{name}: realized examined a pair the idealized scan never did"
        );
    }
}

/// The tree must actually PRUNE somewhere (evidence the realized path
/// engages): disjoint operands examine strictly fewer pairs.
#[test]
fn disjoint_bodies_are_pruned() {
    let a: Body<f64> = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b: Body<f64> = brick((5.0, 6.0), (5.0, 6.0), (5.0, 6.0));
    let (r_ab, r_ba) = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
    let (i_ab, i_ba) =
        sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
    assert_eq!(r_ab.examined.len() + r_ba.examined.len(), 0);
    assert!(i_ab.examined.len() + i_ba.examined.len() > 0);
}

/// Pin 2: full boolean results bit-equal through either strategy —
/// same bodies, same bytes (`Debug` form, the D9 pin), for every op
/// and scenario, declared flush contacts included. Split per the fix
/// pass (item 4): known-good rows must be `Ok` and their PAYLOADS
/// byte-equal — a both-`Err` row can no longer pass silently; the
/// explicitly-listed expected refusals must be `Err` on BOTH sides
/// with byte-equal refusals.
#[test]
fn results_bit_equal_realized_vs_idealized() {
    // Typed refusals, listed by hand and re-derived if a scenario
    // changes: (scenario, op) rows whose result is an expected
    // refusal in BOTH strategies (probed; each is a genuine kernel
    // refusal, not a suite artifact).
    let expected_refusals: &[(&str, BooleanOp)] = &[
        // (Empty since M5 S1: the flush-stacked UNION — formerly the
        // Join(UnpairedLooseEnds) row — now BUILDS through the
        // declared-REST zip, identically in both strategies; its
        // Ok payloads ride the byte-equality assertion below like
        // every green row. Intersect/Subtract on the same pair were
        // always green.)
    ];
    for (name, a, b) in scenarios() {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let decls = common::flush_declarations(&a, &b);
            let real = boolean_op_with(op, &a, &b, &decls, SweepStrategy::Realized, Tol::witness());
            let ideal =
                boolean_op_with(op, &a, &b, &decls, SweepStrategy::Idealized, Tol::witness());
            let refusal_row = expected_refusals.contains(&(name, op));
            match (&real, &ideal) {
                (Ok(r), Ok(i)) => {
                    assert!(!refusal_row, "{name} / {op:?}: expected refusal, got Ok");
                    assert_eq!(
                        format!("{r:?}"),
                        format!("{i:?}"),
                        "{name} / {op:?}: Ok payloads diverged"
                    );
                    // The vacuity guard's other half: Empty is only
                    // legitimate where a regularized result IS empty
                    // (measure-zero contact / disjoint intersects).
                    let empty_ok = matches!(
                        (name, op),
                        ("disjoint bricks", BooleanOp::Intersect)
                            | ("flush-stacked bricks", BooleanOp::Intersect)
                            | ("corner kiss", BooleanOp::Intersect)
                    );
                    assert!(
                        matches!(r, BooleanResult::Body(_)) || empty_ok,
                        "{name} / {op:?}: unexpectedly empty result"
                    );
                }
                (Err(re), Err(ie)) => {
                    assert!(
                        refusal_row,
                        "{name} / {op:?}: BOTH strategies refused but the row is not \
                         listed as an expected refusal — pin 2 must not pass vacuously \
                         (realized: {re:?})"
                    );
                    assert_eq!(
                        format!("{re:?}"),
                        format!("{ie:?}"),
                        "{name} / {op:?}: refusals diverged"
                    );
                }
                _ => panic!(
                    "{name} / {op:?}: strategies disagree about success: \
                     realized {real:?} vs idealized {ideal:?}"
                ),
            }
        }
    }
}

/// Pin 3: plant one face box empty and the superset comparator MUST
/// report the loss — proof the suite can fail.
#[test]
fn planted_degradation_is_caught() {
    let a: Body<f64> = brick((0.0, 4.0), (0.0, 2.0), (0.0, 2.0));
    let b: Body<f64> = brick((1.0, 3.0), (-1.0, 3.0), (1.0, 3.0));
    let (i_ab, _) = sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
    let &(_, face) = i_ab
        .accepted
        .first()
        .expect("crossing bricks accept A→B events");
    let plant = PlantedDegradation { face };
    let (r_ab, _) =
        sweep_traces(&a, &b, SweepStrategy::Realized, Some(plant), Tol::witness()).unwrap();
    let missing = missing_pairs(&r_ab, &i_ab);
    assert!(
        missing.iter().any(|&(_, f)| f == face),
        "planted-empty face box must lose its accepted pairs (got {missing:?})"
    );
}

// ---- fix-pass item 1: the sweep_pad derivation, PINNED ----

/// The flush-tower fixture: A = [0,1]³, B = laterally-shrunk brick
/// hovering `gap` above A's top face — the ONLY near-contact is B's
/// bottom vertices against A's top face plane (side planes are ≥0.25
/// apart, definite at every ε row).
fn tower(gap: f64) -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        brick((0.25, 0.75), (0.25, 0.75), (1.0 + gap, 2.0 + gap)),
    )
}

/// Pin 1(a): accepted / escalated / examined / pruned transitions land
/// exactly where `boxes::sweep_pad`'s derivation says. Gaps are in
/// units of the run band (zero = ε, escalate = K·ε, pad = escalate +
/// 2·zero), so the pin holds at every ε row.
#[test]
fn sweep_pad_derivation_transitions() {
    let tol = geom_core::Tol::witness().get();
    let (zero, escalate) = (tol.eps, tol.k * tol.eps);
    let pad = escalate + 2.0 * zero;

    // Accepted: gap ≤ zero (the decide's inclusive coincidence zone).
    // Interior points only: an exactly-threshold gap lands on either
    // side after the construction arithmetic's own rounding (the
    // margin reaches the decide through profile/extrude evaluation),
    // so the derivation's INEQUALITIES are pinned via interior gaps.
    for gap in [0.0, 0.25 * zero, 0.75 * zero] {
        let (a, b) = tower(gap);
        let (r_ab, r_ba) =
            sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
        let (i_ab, i_ba) =
            sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
        assert!(
            !r_ab.accepted.is_empty() || !r_ba.accepted.is_empty(),
            "gap {gap:e}: flush contact must be accepted"
        );
        assert_eq!(missing_pairs(&r_ab, &i_ab), vec![], "gap {gap:e}");
        assert_eq!(missing_pairs(&r_ba, &i_ba), vec![], "gap {gap:e}");
    }

    // Escalated: gap strictly inside (zero, escalate) — BOTH
    // strategies refuse (the boxes overlap: escalate < pad, so the
    // tree never hides an in-band margin here).
    let mid = 0.5 * (zero + escalate);
    let (a, b) = tower(mid);
    assert!(
        sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).is_err(),
        "in-band gap must escalate (realized)"
    );
    assert!(
        sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).is_err(),
        "in-band gap must escalate (idealized)"
    );

    // Examined-not-accepted: escalate ≤ gap ≤ 2·pad (each side is
    // padded by `pad`, so box reach is 2·pad plus outward ulps).
    // Interior points again (2·escalate, not escalate — see above).
    for gap in [2.0 * escalate, pad, 2.0 * pad] {
        let (a, b) = tower(gap);
        let (r_ab, r_ba) =
            sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
        assert!(
            !r_ab.examined.is_empty() || !r_ba.examined.is_empty(),
            "gap {gap:e}: still a candidate inside 2·pad"
        );
        assert!(
            r_ab.accepted.is_empty() && r_ba.accepted.is_empty(),
            "gap {gap:e}: definitely-separate gap must accept nothing"
        );
    }

    // Pruned: beyond 2·pad the pair space empties under the tree
    // while the idealized scan still walks it.
    let (a, b) = tower(4.0 * pad);
    let (r_ab, r_ba) = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness()).unwrap();
    let (i_ab, i_ba) =
        sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
    assert_eq!(
        r_ab.examined.len() + r_ba.examined.len(),
        0,
        "beyond 2·pad the tree prunes everything"
    );
    assert!(i_ab.examined.len() + i_ba.examined.len() > 0);
    assert!(i_ab.accepted.is_empty() && i_ba.accepted.is_empty());
}

/// Pin 1(b): the suite CATCHES a pad regression — at pad = 0 the
/// tolerance-zone contact (gap = 0.5·zero, accepted by the exact
/// predicates) is pruned, and the superset comparator reports it.
#[test]
fn pad_zero_regression_is_caught() {
    let gap = 0.5 * geom_core::Tol::witness().get().eps;
    let (a, b) = tower(gap);
    let (i_ab, i_ba) =
        sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness()).unwrap();
    let (r_ab, r_ba) = sweep_traces_with_pad(
        &a,
        &b,
        SweepStrategy::Realized,
        None,
        Some(0.0),
        Tol::witness(),
    )
    .unwrap();
    let lost = missing_pairs(&r_ab, &i_ab).len() + missing_pairs(&r_ba, &i_ba).len();
    assert!(
        lost > 0,
        "pad = 0 must LOSE the tolerance-zone contact and the comparator must say so"
    );
}

// ---- fix-pass item 8: the documented error-channel divergence, PINNED ----

/// Bricks grazing an INFINITE face plane far from the face itself:
/// A sits laterally 4 m from B, with A's bottom exactly `k·zero`
/// above the plane of B's top face. The reduce module docs promise:
/// pruning can drop only spurious escalations, never a value — so
///
/// - k < 1 (coincidence zone): idealized examines the pair, classifies
///   the endpoint against the face, gets a definite lateral `Out` —
///   both strategies agree green; results bit-equal.
/// - 1 < k < K (the ambiguity band): idealized ESCALATES on the
///   grazing side margin; realized prunes the pair and completes —
///   EXACTLY the documented error-channel divergence, pinned here.
/// - k > K (definite): both green, results bit-equal.
#[test]
fn grazing_infinite_plane_divergence_is_exactly_as_documented() {
    let tol = geom_core::Tol::witness().get();
    let zero = tol.eps;
    // Multipliers straddling the band edges (K = tol.k, default 10);
    // interior points only (threshold-exact gaps are fp-fragile
    // through the construction arithmetic — see the pad pin above).
    assert!(
        tol.k > 5.0 && tol.k < 20.0,
        "band pin assumes default-ish K"
    );
    for k in [0.5, 2.0, 5.0, 9.0, 20.0] {
        let d = k * zero;
        let a: Body<f64> = brick((5.0, 6.0), (0.0, 1.0), (1.0 + d, 2.0 + d));
        let b: Body<f64> = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
        let realized = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness());
        let idealized = sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness());
        let (r_ab, r_ba) = realized.expect("realized never examines the remote pair");
        assert_eq!(
            r_ab.examined.len() + r_ba.examined.len(),
            0,
            "k = {k}: laterally-remote pair must be pruned"
        );
        if k > 1.0 && k < tol.k {
            let err = idealized.expect_err("in-band grazing margin must escalate idealized");
            assert!(
                format!("{err:?}").contains("bool_vertex_face_side"),
                "k = {k}: the escalation is the grazing side test, got {err:?}"
            );
        } else {
            let (i_ab, i_ba) = idealized.expect("out-of-band grazing is green idealized");
            assert!(i_ab.examined.len() + i_ba.examined.len() > 0);
            assert!(
                i_ab.accepted.is_empty() && i_ba.accepted.is_empty(),
                "k = {k}: a remote graze must accept nothing"
            );
        }
        // The VALUE-channel pin: out of band the full boolean is
        // byte-equal through either strategy. IN band, BOTH refuse —
        // the in-band margin that the realized sweep prunes is caught
        // again by the disjoint-operands containment stage
        // (`bool_point_in_solid_plane`), so no strategy ever returns
        // a silent value; the divergence is confined to the REFUSAL
        // SITE (sweep side test vs solid containment), pinned here
        // predicate-by-predicate.
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let decls = topo::BooleanDeclarations::none();
            let real = boolean_op_with(op, &a, &b, &decls, SweepStrategy::Realized, Tol::witness());
            let ideal =
                boolean_op_with(op, &a, &b, &decls, SweepStrategy::Idealized, Tol::witness());
            if k > 1.0 && k < tol.k {
                let re = real.expect_err("in-band realized refuses at containment");
                let ie = ideal.expect_err("in-band idealized refuses at the sweep");
                assert!(
                    format!("{re:?}").contains("bool_point_in_solid_plane"),
                    "k = {k} / {op:?}: realized refusal site, got {re:?}"
                );
                assert!(
                    format!("{ie:?}").contains("bool_vertex_face_side"),
                    "k = {k} / {op:?}: idealized refusal site, got {ie:?}"
                );
            } else {
                assert_eq!(
                    format!("{:?}", real.unwrap()),
                    format!("{:?}", ideal.unwrap()),
                    "k = {k} / {op:?}: value channel diverged"
                );
            }
        }
    }
}
