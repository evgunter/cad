//! R1 review probes for DOCM-9 (`editor_core::range`). Not part of the
//! unit: written by the review lane to falsify the PR's claims by
//! execution. Every row here is a measurement, and several are
//! deliberately red.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeSet;

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use editor_core::range::{RangeField, RangeSeed, RangeSide, certified_range, derive};
use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node,
    NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, evaluate,
};
use geom_core::Tol;

use fixture::Recorder;

fn tol() -> Tol {
    Tol::witness()
}
fn name(n: &str) -> ParamName {
    ParamName::new(n)
}
fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite")
}
fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite")
}
fn param(n: &str) -> Expr {
    Expr::param(name(n), Dimension::Length)
}
fn budget(max_depth: u32, max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_depth,
        max_leaves,
        ..DriveConfig::default()
    }
}
fn frame(r: &mut Recorder) -> RecipeNodeId {
    r.insert(Node::Datum(Datum::Frame {
        origin: [lit(0.0), lit(0.0), lit(0.0)],
        u: [scalar(1.0), scalar(0.0), scalar(0.0)],
        v: [scalar(0.0), scalar(1.0), scalar(0.0)],
    }))
}
fn declare(r: &mut Recorder, n: &str, value: f64) {
    r.push(DocEdit::SetDocParam {
        name: name(n),
        value: DocParam::continuous(Dimension::Length, value),
    });
}
fn square(side: f64) -> LoopProgram {
    LoopProgram::polygon([(0.0, 0.0), (side, 0.0), (side, side), (0.0, side)]).expect("finite")
}

/// A slab whose extrusion distance is a document parameter.
fn slab(depth: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", depth);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square(1.0)],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// **The tour's die**: a 2x2 plate with a circular hole, the hole's
/// radius a document parameter. The realistic document the cost
/// question is about.
fn plate_with_hole(radius: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "r", radius);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![
            square(2.0),
            LoopProgram::Circle {
                centre: [lit(1.0), lit(1.0)],
                radius: param("r"),
            },
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: lit(0.5),
    });
    r.doc
}

fn leaves_of(
    doc: &ProfileDoc,
    field: &str,
    seed: RangeSeed,
    config: &DriveConfig,
) -> Vec<(f64, f64, String)> {
    let d = derive(doc, &RangeField::Param(name(field)), seed, tol()).expect("derives");
    let analyzed = analyzed_box(&d.doc, &AnalysisPolicy::default());
    let v = drive(&d.doc, &analyzed, config, tol()).expect("drives");
    let mut out: Vec<(f64, f64, String)> = Vec::new();
    for l in v.certified() {
        let (_, lo, hi) = l.box_.varying().next().expect("one axis");
        out.push((lo, hi, "certified".to_string()));
    }
    for l in v.refused() {
        let (_, lo, hi) = l.box_.varying().next().expect("one axis");
        out.push((lo, hi, format!("{:?}", l.reason.class())));
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out
}

fn arm(s: &RangeSide) -> String {
    match s {
        RangeSide::Certified { to } => format!("Certified{{to:{to}}}"),
        RangeSide::NewFailure {
            certified_to,
            within,
            ..
        } => format!("NewFailure{{certified_to:{certified_to}, within:{within:?}}}"),
        RangeSide::DecisionFlip {
            certified_to,
            within,
            ..
        } => format!("DecisionFlip{{certified_to:{certified_to}, within:{within:?}}}"),
        RangeSide::Indeterminate {
            certified_to,
            within,
            reason,
        } => format!(
            "Indeterminate{{certified_to:{certified_to}, within:{within:?}, reason:{:?}}}",
            reason.class()
        ),
    }
}

// ---------------------------------------------------------------- P1

/// **D1's structural argument, by execution.** If a certified leaf and
/// a `FlipCrossing` leaf are ever neighbours, the PR's reason for
/// re-reading the spec's arm is false.
#[test]
fn p1_certified_and_flipcrossing_are_never_neighbours() {
    let mut adjacencies = 0usize;
    for (label, doc, field, seed, cfg) in [
        (
            "slab depth",
            slab(1.0),
            "depth",
            RangeSeed { lo: -2.0, hi: 0.5 },
            budget(24, 4096),
        ),
        (
            "slab depth deep",
            slab(1.0),
            "depth",
            RangeSeed { lo: -2.0, hi: 0.5 },
            budget(34, 8192),
        ),
        (
            "plate hole r",
            plate_with_hole(0.5),
            "r",
            RangeSeed { lo: -0.4, hi: 0.8 },
            budget(20, 2048),
        ),
    ] {
        let leaves = leaves_of(&doc, field, seed, &cfg);
        let certified = leaves.iter().filter(|l| l.2 == "certified").count();
        let flips = leaves.iter().filter(|l| l.2 == "FlipCrossing").count();
        for w in leaves.windows(2) {
            let pair = (w[0].2.as_str(), w[1].2.as_str());
            if pair == ("certified", "FlipCrossing") || pair == ("FlipCrossing", "certified") {
                adjacencies += 1;
                println!("ADJACENT on {label}: {:?} then {:?}", w[0], w[1]);
            }
        }
        println!("{label}: {} leaves, {certified} certified, {flips} flip-crossing", leaves.len());
        let mut classes: Vec<&str> = leaves.iter().map(|l| l.2.as_str()).collect();
        classes.dedup();
        println!("  class run: {classes:?}");
    }
    println!("total certified/flip adjacencies: {adjacencies}");
    assert_eq!(adjacencies, 0, "D1's argument says this is structural");
}

// ---------------------------------------------------------------- P2

/// **Is `NewFailure` reachable through a document that already fails?**
#[test]
fn p2_a_failing_witness_is_refused_by_the_driver_not_reported() {
    let doc = slab(0.0);
    let ev: editor_core::Evaluation<f64> =
        evaluate(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol());
    let failed: BTreeSet<RecipeNodeId> = ev
        .nodes
        .iter()
        .filter(|(_, r)| matches!(r, NodeResult::Failed(_)))
        .map(|(id, _)| *id)
        .collect();
    println!("slab(0.0) failing nodes at f64: {failed:?}");
    let r = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.5),
        &budget(20, 2048),
        tol(),
    );
    println!("certified_range on a failing witness: {r:?}");
    assert!(r.is_err(), "a failing witness has no certificate");
}

// ---------------------------------------------------------------- P3

/// **The hole that outgrows its plate** — a document the implementer
/// did not choose, aimed straight at `NewFailure`.
#[test]
fn p3_a_hole_outgrowing_its_plate() {
    for (label, r0, seed) in [
        ("r=0.5, seed [-0.4,+0.8]", 0.5, RangeSeed { lo: -0.4, hi: 0.8 }),
        ("r=0.9, seed [-0.5,+0.5]", 0.9, RangeSeed { lo: -0.5, hi: 0.5 }),
        ("r=0.5, seed [-0.49,+1.4]", 0.5, RangeSeed { lo: -0.49, hi: 1.4 }),
    ] {
        let doc = plate_with_hole(r0);
        let out = certified_range(
            &doc,
            &RangeField::Param(name("r")),
            seed,
            &budget(22, 4096),
            tol(),
        );
        match out {
            Ok(cr) => println!("{label}: lo={} hi={}", arm(cr.lo()), arm(cr.hi())),
            Err(e) => println!("{label}: refused {e}"),
        }
    }
}

// --------------------------------------------------------------- P11

/// **The door's read order.** `derive` tests `slot.is_structural()`
/// BEFORE asking whether the node carries the slot at all, so a
/// structural slot named on a node that has none is reported as
/// `StructuralSlot` — "the count slot of node N is structural" — for a
/// node with no count slot.
#[test]
fn p11_a_structural_slot_on_a_node_that_has_none() {
    let mut r = Recorder::new();
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square(1.0)],
    }));
    let e = r.insert(Node::Extrude {
        profile: p,
        distance: lit(1.0),
    });
    let doc = r.doc;
    let has_count = doc
        .node(e)
        .and_then(|n| n.expr(editor_core::SlotId::Count))
        .is_some();
    let out = derive(
        &doc,
        &RangeField::Slot {
            node: e,
            slot: editor_core::SlotId::Count,
        },
        RangeSeed::symmetric(0.25),
        tol(),
    );
    println!("the extrude carries a Count slot: {has_count}");
    match &out {
        Ok(_) => println!("derived (unexpected)"),
        Err(e) => println!("refusal: {e:?} / rendered: {e}"),
    }
    assert!(has_count || !matches!(out, Err(editor_core::range::RangeRefusal::StructuralSlot { .. })),
        "a node with no Count slot is told its Count slot is structural");
}

// --------------------------------------------------------------- P10

/// **The filed `parametric-polygon-loop-certifies-nothing` row**,
/// reproduced independently: a parameter inside a `polygon_expr`
/// vertex, seeded at 100 ε.
#[test]
fn p10_a_parameter_in_a_polygon_vertex() {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    declare(&mut r, "side", 1.0);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![LoopProgram::polygon_expr([
            [lit(0.0), lit(0.0)],
            [param("side"), lit(0.0)],
            [param("side"), param("side")],
            [lit(0.0), param("side")],
        ])],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    let doc = r.doc;
    for w in [1e-7_f64, 1e-9, 1e-3] {
        let t = std::time::Instant::now();
        let d = derive(
            &doc,
            &RangeField::Param(name("side")),
            RangeSeed::symmetric(w),
            tol(),
        )
        .expect("derives");
        let analyzed = analyzed_box(&d.doc, &AnalysisPolicy::default());
        let v = drive(&d.doc, &analyzed, &budget(24, 64), tol()).expect("drives");
        println!(
            "side seed +/-{w:e}: {} certified, {} refused, {:?}",
            v.certified().len(),
            v.refused().len(),
            t.elapsed()
        );
    }
    // And the same document ranged over `depth`, which is not in a vertex.
    let t = std::time::Instant::now();
    let cr = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.1),
        &budget(24, 64),
        tol(),
    )
    .unwrap();
    println!(
        "depth seed +/-0.1: lo={} hi={} in {:?}",
        arm(cr.lo()),
        arm(cr.hi()),
        t.elapsed()
    );
}

// ---------------------------------------------------------------- P9

/// **RED.** `RangeSide::DecisionFlip`'s own documentation says "a
/// recorded predicate decides differently inside `within` WHILE EVERY
/// NODE STILL BUILDS", and "the probe would call such values valid".
/// On the unit's own A2 fixture the bracket contains a value at which
/// the extrude does not build at all.
#[test]
fn p9_decisionflip_within_contains_a_value_that_does_not_build() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -1.05, hi: 0.5 };
    let cr = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        seed,
        &budget(24, 2048),
        tol(),
    )
    .unwrap();
    let RangeSide::DecisionFlip { within, .. } = cr.lo() else {
        panic!("expected a decision flip, got {:?}", cr.lo());
    };
    println!("within (offsets) = {within:?}");
    let mut broken = Vec::new();
    for k in 0..=10 {
        let off = within.0 + (within.1 - within.0) * f64::from(k) / 10.0;
        let value = cr.absolute(off);
        let moved = editor_core::apply(
            &doc,
            &DocEdit::SetDocParamValue {
                name: name("depth"),
                value: editor_core::DocParamValue::Continuous(value),
            },
            tol(),
        )
        .expect("a value edit applies")
        .doc;
        let ev: editor_core::Evaluation<f64> = evaluate(
            &moved,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol(),
        );
        let failed: Vec<RecipeNodeId> = ev
            .nodes
            .iter()
            .filter(|(_, r)| matches!(r, NodeResult::Failed(_)))
            .map(|(id, _)| *id)
            .collect();
        if !failed.is_empty() {
            broken.push((off, value, failed));
        }
    }
    for b in &broken {
        println!("NOT A VALID BUILD inside `within`: offset {} (depth {}) fails {:?}", b.0, b.1, b.2);
    }
    assert!(
        broken.is_empty(),
        "DecisionFlip says every node still builds inside `within`; {} sampled values do not",
        broken.len()
    );
}

// ---------------------------------------------------------------- P8

/// **Does ANY flip-crossing leaf carry a node standing change?** The
/// arm's reachability read at the driver rather than at the reported
/// side: a `NewFailure` needs one such leaf to be the first one
/// outward, and this counts them over whole drives.
#[test]
fn p8_standing_changes_among_flip_leaves() {
    for (label, doc, field, seed, cfg) in [
        (
            "slab depth [-2,0.5]",
            slab(1.0),
            "depth",
            RangeSeed { lo: -2.0, hi: 0.5 },
            budget(24, 4096),
        ),
        (
            "plate hole r [-0.4,0.8]",
            plate_with_hole(0.5),
            "r",
            RangeSeed { lo: -0.4, hi: 0.8 },
            budget(18, 1024),
        ),
        (
            "plate hole r=0.9 [-0.5,0.5]",
            plate_with_hole(0.9),
            "r",
            RangeSeed { lo: -0.5, hi: 0.5 },
            budget(18, 1024),
        ),
    ] {
        let d = derive(&doc, &RangeField::Param(name(field)), seed, tol()).expect("derives");
        let analyzed = analyzed_box(&d.doc, &AnalysisPolicy::default());
        let v = drive(&d.doc, &analyzed, &cfg, tol()).expect("drives");
        let mut flips = 0usize;
        let mut standing = 0usize;
        let mut structure = 0usize;
        for l in v.refused() {
            if let editor_core::drive::RefusalReason::FlipCrossing { flipped } = &l.reason {
                flips += 1;
                if flipped
                    .verdicts
                    .nodes
                    .values()
                    .any(|x| x.old_status != x.new_status)
                {
                    standing += 1;
                }
                if !flipped.structure.is_empty() {
                    structure += 1;
                }
            }
        }
        println!(
            "{label}: {flips} flip leaves, {standing} with a node STANDING change, \
             {structure} with a structure flip"
        );
    }
}

// ---------------------------------------------------------------- P4

/// **Cost at the spec's on-demand posture**, on the tour's die.
#[test]
fn p4_cost_of_a_realistic_document() {
    for (depth, leaves) in [(12u32, 512usize), (20, 2048), (24, 4096)] {
        let doc = plate_with_hole(0.5);
        let t = std::time::Instant::now();
        let out = certified_range(
            &doc,
            &RangeField::Param(name("r")),
            RangeSeed::symmetric(0.2),
            &budget(depth, leaves),
            tol(),
        );
        let dt = t.elapsed();
        match out {
            Ok(cr) => println!(
                "plate+hole depth={depth} leaves={leaves}: {dt:?} lo={} hi={}",
                arm(cr.lo()),
                arm(cr.hi())
            ),
            Err(e) => println!("plate+hole depth={depth} leaves={leaves}: {dt:?} refused {e}"),
        }
    }
}

// ---------------------------------------------------------------- P5

/// **A document whose OTHER parameters carry distributions.**
#[test]
fn p5_other_distributions_are_cleared() {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    r.push(DocEdit::SetDocParam {
        name: name("side"),
        value: DocParam::continuous_with(
            Dimension::Length,
            1.0,
            editor_core::Distribution::Uniform { lo: -0.1, hi: 0.1 },
        ),
    });
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![LoopProgram::polygon_expr([
            [lit(0.0), lit(0.0)],
            [param("side"), lit(0.0)],
            [param("side"), param("side")],
            [lit(0.0), param("side")],
        ])],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    let doc = r.doc;
    let before = analyzed_box(&doc, &AnalysisPolicy::default());
    println!(
        "input analyzed axes: {:?}",
        before.varying().map(|(n, _)| n.0.clone()).collect::<Vec<_>>()
    );
    let d = derive(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.25),
        tol(),
    )
    .unwrap();
    let after = analyzed_box(&d.doc, &AnalysisPolicy::default());
    println!(
        "derived analyzed axes: {:?}",
        after.varying().map(|(n, _)| n.0.clone()).collect::<Vec<_>>()
    );
    println!(
        "side in the derived doc: {:?}",
        d.doc.params().get(&name("side"))
    );
    let cr = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.25),
        &budget(20, 2048),
        tol(),
    )
    .unwrap();
    println!(
        "range over depth with side pinned: lo={} hi={}",
        arm(cr.lo()),
        arm(cr.hi())
    );
}

// ---------------------------------------------------------------- P6

/// **What `within` claims, read off the leaves.**
#[test]
fn p6_within_is_bracketed_by_a_flip_leaf() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -2.0, hi: 0.5 };
    let cfg = budget(24, 4096);
    let leaves = leaves_of(&doc, "depth", seed, &cfg);
    let cr = certified_range(&doc, &RangeField::Param(name("depth")), seed, &cfg, tol()).unwrap();
    println!("lo={} hi={}", arm(cr.lo()), arm(cr.hi()));
    let lo = cr.lo();
    if let RangeSide::DecisionFlip {
        certified_to,
        within,
        ..
    }
    | RangeSide::NewFailure {
        certified_to,
        within,
        ..
    } = lo
    {
        let far = within.0;
        let abutting: Vec<_> = leaves
            .iter()
            .filter(|(_, hi, k)| k == "FlipCrossing" && *hi == far)
            .collect();
        println!("certified_to={certified_to} within={within:?}");
        println!("flip leaves whose FAR-side edge equals within.0: {abutting:?}");
        let between: Vec<_> = leaves
            .iter()
            .filter(|(lo_, hi_, _)| *lo_ >= within.0 && *hi_ <= within.1)
            .collect();
        println!("leaves inside within ({} of them):", between.len());
        for b in between.iter().take(10) {
            println!("  {b:?}");
        }
    }
}

// ---------------------------------------------------------------- P7

/// **A degenerate slot dimension.** `SlotId::dimension()` is what the
/// synthetic parameter is declared with; a slot whose dimension is not
/// Length is the case to look at.
#[test]
fn p7_a_scalar_or_angle_slot_widens() {
    let mut r = Recorder::new();
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![square(1.0)],
    }));
    let e = r.insert(Node::Extrude {
        profile: p,
        distance: lit(1.0),
    });
    let doc = r.doc;
    let _ = e;
    let out = certified_range(
        &doc,
        &RangeField::Slot {
            node: e,
            slot: editor_core::SlotId::Distance,
        },
        RangeSeed::symmetric(0.25),
        &budget(20, 2048),
        tol(),
    );
    match out {
        Ok(cr) => println!(
            "slot field: nominal={} lo={} hi={}",
            cr.nominal(),
            arm(cr.lo()),
            arm(cr.hi())
        ),
        Err(err) => println!("slot field refused: {err}"),
    }
}
