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
use topo::{
    Body, BooleanOp, PlantedDegradation, SweepStrategy, SweepTrace, boolean_op_with, sweep_traces,
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
        let (r_ab, r_ba) = sweep_traces(&a, &b, SweepStrategy::Realized, None).unwrap();
        let (i_ab, i_ba) = sweep_traces(&a, &b, SweepStrategy::Idealized, None).unwrap();
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
    let (r_ab, r_ba) = sweep_traces(&a, &b, SweepStrategy::Realized, None).unwrap();
    let (i_ab, i_ba) = sweep_traces(&a, &b, SweepStrategy::Idealized, None).unwrap();
    assert_eq!(r_ab.examined.len() + r_ba.examined.len(), 0);
    assert!(i_ab.examined.len() + i_ba.examined.len() > 0);
}

/// Pin 2: full boolean results bit-equal through either strategy —
/// same bodies, same bytes (`Debug` form, the D9 pin), for every op
/// and scenario, declared flush contacts included.
#[test]
fn results_bit_equal_realized_vs_idealized() {
    for (name, a, b) in scenarios() {
        for op in [BooleanOp::Union, BooleanOp::Intersect, BooleanOp::Subtract] {
            let decls = common::flush_declarations(&a, &b);
            let real = boolean_op_with(op, &a, &b, &decls, SweepStrategy::Realized);
            let ideal = boolean_op_with(op, &a, &b, &decls, SweepStrategy::Idealized);
            assert_eq!(
                format!("{real:?}"),
                format!("{ideal:?}"),
                "{name} / {op:?}: realized and idealized results diverged"
            );
        }
    }
}

/// Pin 3: plant one face box empty and the superset comparator MUST
/// report the loss — proof the suite can fail.
#[test]
fn planted_degradation_is_caught() {
    let a: Body<f64> = brick((0.0, 4.0), (0.0, 2.0), (0.0, 2.0));
    let b: Body<f64> = brick((1.0, 3.0), (-1.0, 3.0), (1.0, 3.0));
    let (i_ab, _) = sweep_traces(&a, &b, SweepStrategy::Idealized, None).unwrap();
    let &(_, face) = i_ab
        .accepted
        .first()
        .expect("crossing bricks accept A→B events");
    let plant = PlantedDegradation { face };
    let (r_ab, _) = sweep_traces(&a, &b, SweepStrategy::Realized, Some(plant)).unwrap();
    let missing = missing_pairs(&r_ab, &i_ab);
    assert!(
        missing.iter().any(|&(_, f)| f == face),
        "planted-empty face box must lose its accepted pairs (got {missing:?})"
    );
}
