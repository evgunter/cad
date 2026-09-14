//! DOCM-9 review lane R2 — probes against `editor_core::range` at
//! `691a26e2`. Measurements print with `--nocapture`; the assertions
//! are the PR's own claims restated so a red row is a falsified claim.
#![cfg(feature = "interval")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout,
    clippy::cast_precision_loss,
    dead_code
)]

use crate::{corpus, fixture};

use std::time::Instant;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, RefusalReason, drive};
use editor_core::range::{RangeField, RangeSeed, RangeSide, certified_range, derive};
use editor_core::{
    CancelToken, Datum, Dimension, DocEdit, DocParam, DocParamValue, Evaluation, EvalOptions,
    Expr, LoopProgram, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId,
    SlotId, evaluate,
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
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}
fn scalar(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar literal")
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
fn f64_run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol())
}
fn failing_names(ev: &Evaluation<f64>) -> Vec<String> {
    ev.nodes
        .iter()
        .filter_map(|(id, r)| match r {
            NodeResult::Failed(e) => Some(format!("{}:{}", id.0, e.kind)),
            NodeResult::Poisoned { .. } => Some(format!("{}:poisoned", id.0)),
            NodeResult::Ok(_) => None,
        })
        .collect()
}
/// The document with parameter `p` set to `value`, at f64: which nodes fail.
fn failing_at(doc: &ProfileDoc, p: &str, value: f64) -> Vec<String> {
    let moved = editor_core::apply(
        doc,
        &DocEdit::SetDocParamValue {
            name: name(p),
            value: DocParamValue::Continuous(value),
        },
        tol(),
    )
    .expect("a value edit applies")
    .doc;
    failing_names(&f64_run(&moved))
}

// ------------------------------------------------------------ fixtures

/// The PR's slab: a unit square extruded by a parameter.
fn slab(depth: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "depth", depth);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap(),
        ],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: param("depth"),
    });
    r.doc
}

/// A `w` x 2 plate with one circular hole of radius `r` whose centre x
/// is the parameter `cx`.
fn plate_hole_cx(w: f64, cx: f64, r: f64) -> ProfileDoc {
    let mut rec = Recorder::new();
    declare(&mut rec, "cx", cx);
    let f = frame(&mut rec);
    let p = rec.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (w, 0.0), (w, 2.0), (0.0, 2.0)]).unwrap(),
            LoopProgram::Circle {
                centre: [param("cx"), lit(1.0)],
                radius: lit(r),
            },
        ],
    }));
    rec.insert(Node::Extrude {
        profile: p,
        distance: lit(1.0),
    });
    rec.doc
}

/// A `w` x 2 plate with one circular hole whose RADIUS is the
/// parameter `r` (the hole outgrows the plate at r = 1).
fn plate_hole_r(w: f64, r: f64) -> ProfileDoc {
    let mut rec = Recorder::new();
    declare(&mut rec, "r", r);
    let f = frame(&mut rec);
    let p = rec.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (w, 0.0), (w, 2.0), (0.0, 2.0)]).unwrap(),
            LoopProgram::Circle {
                centre: [lit(w / 2.0), lit(1.0)],
                radius: param("r"),
            },
        ],
    }));
    rec.insert(Node::Extrude {
        profile: p,
        distance: lit(1.0),
    });
    rec.doc
}

/// A quadrilateral (0,0),(1,0),(x,1),(0,1) whose fourth-vertex x is a
/// parameter: for x < 0 the polygon is self-intersecting.
fn bowtie(x: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "x", x);
    let f = frame(&mut r);
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: f,
        loops: vec![LoopProgram::polygon_expr([
            [lit(0.0), lit(0.0)],
            [lit(1.0), lit(0.0)],
            [param("x"), lit(1.0)],
            [lit(0.0), lit(1.0)],
        ])],
    }));
    r.insert(Node::Extrude {
        profile: p,
        distance: lit(0.5),
    });
    r.doc
}

// ------------------------------------------------------------ ledgers

fn span(b: &ParamBox, axis: &ParamName) -> (f64, f64) {
    let v: Vec<_> = b.varying().collect();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].0, axis);
    (v[0].1, v[0].2)
}

fn class(reason: Option<&RefusalReason>) -> String {
    match reason {
        None => "Certified".into(),
        Some(RefusalReason::FlipCrossing { flipped }) => {
            let standing = flipped
                .verdicts
                .nodes
                .values()
                .any(|d| d.old_status != d.new_status);
            format!(
                "Flip{}{}",
                if standing { "+STANDING" } else { "" },
                if flipped.structure.is_empty() {
                    ""
                } else {
                    "+structure"
                }
            )
        }
        Some(RefusalReason::Budget(k)) => format!("Budget({k:?})"),
        Some(RefusalReason::SliverTerminal { predicate }) => format!("Sliver({predicate})"),
        Some(RefusalReason::MeasureRefused { class, .. }) => format!("MeasureRefused({class})"),
        Some(other) => format!("{other:?}"),
    }
}

/// The sorted one-axis leaves of a drive: (lo, hi, class).
fn ledger(
    doc: &ProfileDoc,
    field: &RangeField,
    seed: RangeSeed,
    cfg: &DriveConfig,
) -> Vec<(f64, f64, String)> {
    let d = derive(doc, field, seed, tol()).expect("derives");
    let an = analyzed_box(&d.doc, &AnalysisPolicy::default());
    let v = drive(&d.doc, &an, cfg, tol()).expect("drives");
    let mut rows: Vec<(f64, f64, String)> = v
        .certified()
        .iter()
        .map(|l| {
            let (lo, hi) = span(&l.box_, &d.axis);
            (lo, hi, class(None))
        })
        .chain(v.refused().iter().map(|l| {
            let (lo, hi) = span(&l.box_, &d.axis);
            (lo, hi, class(Some(&l.reason)))
        }))
        .collect();
    rows.sort_by(|a, b| a.0.total_cmp(&b.0));
    rows
}

/// Consecutive leaves of one class folded into runs.
fn runs(rows: &[(f64, f64, String)]) -> Vec<(f64, f64, String, usize)> {
    let mut out: Vec<(f64, f64, String, usize)> = Vec::new();
    for (lo, hi, c) in rows {
        match out.last_mut() {
            Some(last) if last.2 == *c && last.1 == *lo => {
                last.1 = *hi;
                last.3 += 1;
            }
            _ => out.push((*lo, *hi, c.clone(), 1)),
        }
    }
    out
}

fn print_runs(label: &str, rows: &[(f64, f64, String)]) {
    println!("--- {label}: {} leaves", rows.len());
    for (lo, hi, c, n) in runs(rows) {
        println!("    [{lo:+.6e}, {hi:+.6e}] w={:.3e} {c} x{n}", hi - lo);
    }
}

/// Pairs of neighbouring leaves where one is certified and the other a
/// flip crossing — the pair D1's argument says cannot exist.
fn cert_flip_neighbours(rows: &[(f64, f64, String)]) -> usize {
    rows.windows(2)
        .filter(|p| {
            (p[0].2 == "Certified" && p[1].2.starts_with("Flip"))
                || (p[1].2 == "Certified" && p[0].2.starts_with("Flip"))
        })
        .count()
}

fn arm(s: &RangeSide) -> String {
    match s {
        RangeSide::Certified { to } => format!("Certified {{ to: {to:+.6e} }}"),
        RangeSide::NewFailure {
            certified_to,
            within,
            ..
        } => format!(
            "NewFailure {{ certified_to: {certified_to:+.6e}, within: [{:+.6e}, {:+.6e}] }}",
            within.0, within.1
        ),
        RangeSide::DecisionFlip {
            certified_to,
            within,
            ..
        } => format!(
            "DecisionFlip {{ certified_to: {certified_to:+.6e}, within: [{:+.6e}, {:+.6e}] (w={:.3e}) }}",
            within.0,
            within.1,
            within.1 - within.0
        ),
        RangeSide::Indeterminate {
            certified_to,
            within,
            reason,
        } => format!(
            "Indeterminate {{ certified_to: {certified_to:+.6e}, within: [{:+.6e}, {:+.6e}] (w={:.3e}), reason: {} }}",
            within.0,
            within.1,
            within.1 - within.0,
            class(Some(reason))
        ),
    }
}

fn report(label: &str, doc: &ProfileDoc, field: &RangeField, seed: RangeSeed, cfg: &DriveConfig) {
    let t = Instant::now();
    let r = certified_range(doc, field, seed, cfg, tol());
    let dt = t.elapsed();
    match r {
        Ok(r) => {
            println!(
                "=== {label} seed [{:+.3e}, {:+.3e}] nominal {} in {dt:.2?}\n    lo: {}\n    hi: {}",
                seed.lo,
                seed.hi,
                r.nominal(),
                arm(r.lo()),
                arm(r.hi())
            );
        }
        Err(e) => println!("=== {label}: REFUSED in {dt:.2?}: {e}"),
    }
}

// ------------------------------------------------------------ C1 / D1

/// D1's structural argument, measured: on every ledger below, no
/// certified leaf touches a flip-crossing leaf. Goes red if any does.
#[test]
fn r2_no_certified_leaf_neighbours_a_flip_leaf() {
    let cases: Vec<(&str, ProfileDoc, &str, RangeSeed, DriveConfig)> = vec![
        (
            "slab depth (PR A2 seed)",
            slab(1.0),
            "depth",
            RangeSeed { lo: -1.05, hi: 0.5 },
            budget(24, 2048),
        ),
        (
            "slab depth (PR D1 measurement, depth 40)",
            slab(1.0),
            "depth",
            RangeSeed { lo: -2.0, hi: 0.5 },
            budget(40, 4096),
        ),
        (
            "plate 10x2, hole r0.2 at cx=5 (PR round_escape)",
            plate_hole_cx(10.0, 5.0, 0.2),
            "cx",
            RangeSeed { lo: -1.0, hi: 5.5 },
            budget(24, 256),
        ),
        (
            "plate 10x2, hole r0.2 at cx=9.7 (hole leaves the plate at +0.1)",
            plate_hole_cx(10.0, 9.7, 0.2),
            "cx",
            RangeSeed { lo: -0.05, hi: 0.25 },
            budget(24, 256),
        ),
        (
            "plate 4x2, hole radius r=0.5 (outgrows the plate at r=1)",
            plate_hole_r(4.0, 0.5),
            "r",
            RangeSeed { lo: -0.45, hi: 0.6 },
            budget(24, 256),
        ),
        (
            "bowtie x=0.5 (self-intersecting below 0)",
            bowtie(0.5),
            "x",
            RangeSeed { lo: -0.6, hi: 0.3 },
            budget(24, 256),
        ),
    ];
    let mut total_pairs = 0;
    for (label, doc, p, seed, cfg) in &cases {
        let field = RangeField::Param(name(p));
        let t = Instant::now();
        let rows = ledger(doc, &field, *seed, cfg);
        println!("({:.2?})", t.elapsed());
        print_runs(label, &rows);
        let pairs = cert_flip_neighbours(&rows);
        println!("    certified/flip neighbour pairs: {pairs}");
        total_pairs += pairs;
        report(label, doc, &field, *seed, cfg);
    }
    assert_eq!(total_pairs, 0, "D1's neighbour argument is falsified");
}

/// The corpus plate (`plate_param`, hole radius shared by two holes):
/// the holes overlap each other at r = 0.6 and the radius is refused at
/// r <= 0. Both are NEW FAILURES the probe reports. What does the query say?
#[test]
fn r2_new_failure_hunt_on_the_corpus_plate() {
    let cd = corpus::plate_param::document();
    let doc = cd.doc;
    let p = corpus::plate_param::HOLE_R;
    println!(
        "plate_param failing at r=0.61: {:?}",
        failing_at(&doc, p, 0.61)
    );
    println!(
        "plate_param failing at r=-0.01: {:?}",
        failing_at(&doc, p, -0.01)
    );
    println!("plate_param failing at r=0.0: {:?}", failing_at(&doc, p, 0.0));
    let field = RangeField::Param(name(p));
    let seed = RangeSeed { lo: -0.26, hi: 0.4 };
    let cfg = budget(24, 96);
    let t = Instant::now();
    let rows = ledger(&doc, &field, seed, &cfg);
    println!("({:.2?})", t.elapsed());
    print_runs("plate_param hole_r", &rows);
    println!(
        "    certified/flip neighbour pairs: {}",
        cert_flip_neighbours(&rows)
    );
    report("plate_param hole_r", &doc, &field, seed, &cfg);
    let r = certified_range(&doc, &field, seed, &cfg, tol()).unwrap();
    // The PR's D3 claim: NewFailure is unreachable. Red if it is reached.
    assert!(
        !matches!(r.lo(), RangeSide::NewFailure { .. })
            && !matches!(r.hi(), RangeSide::NewFailure { .. }),
        "NewFailure WAS reached: lo {} hi {}",
        arm(r.lo()),
        arm(r.hi())
    );
}

/// **Does `within` of a `DecisionFlip` hold only values at which every
/// node still builds?** The arm's doc says so. On the PR's own A2
/// fixture the coincidence zone (depth in `[-ε, ε]`, a degenerate
/// extrusion that FAILS) lies between the proof frontier and the first
/// flip leaf — i.e. inside `within`.
#[test]
fn r2_decision_flip_within_holds_a_failing_value() {
    let doc = slab(1.0);
    let seed = RangeSeed { lo: -1.05, hi: 0.5 };
    let r = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        seed,
        &budget(24, 2048),
        tol(),
    )
    .unwrap();
    let RangeSide::DecisionFlip { within, .. } = r.lo() else {
        panic!("{}", arm(r.lo()));
    };
    println!("A2 lo: {}", arm(r.lo()));
    let zero_offset = -1.0;
    assert!(within.0 <= zero_offset && zero_offset <= within.1);
    let at_zero = failing_at(&doc, "depth", r.absolute(zero_offset));
    println!("nodes failing at depth = nominal + ({zero_offset}) = 0: {at_zero:?}");
    assert!(
        at_zero.is_empty(),
        "a value inside a DecisionFlip's `within` FAILS a node — the arm's \
         \"every node still builds\" is false here: {at_zero:?}"
    );
}

// ------------------------------------------------------------ cost

/// A leaf-budget ladder on one field: wall clock per budget, and the
/// leaf classes each drive ended with.
fn ladder(label: &str, doc: &ProfileDoc, field: &RangeField, seed: RangeSeed, leaves: &[usize]) {
    for &n in leaves {
        let cfg = budget(24, n);
        let t = Instant::now();
        let d = derive(doc, field, seed, tol()).expect("derives");
        let an = analyzed_box(&d.doc, &AnalysisPolicy::default());
        let v = drive(&d.doc, &an, &cfg, tol()).expect("drives");
        let dt = t.elapsed();
        let mut by: std::collections::BTreeMap<String, usize> = Default::default();
        for l in v.refused() {
            *by.entry(class(Some(&l.reason))).or_default() += 1;
        }
        println!(
            "LADDER {label} seed [{:+.2e},{:+.2e}] leaves<={n}: {dt:.2?}, certified {} refused {:?}",
            seed.lo,
            seed.hi,
            v.certified().len(),
            by
        );
        let rows = ledger(doc, field, seed, &cfg);
        print_runs(&format!("{label} leaves<={n}"), &rows);
    }
}

/// The corpus plate (`plate_param`, two holes sharing a radius) on a
/// leaf-budget ladder; the default budget is 65 536 leaves.
#[test]
fn r2_cost_plate_param_ladder() {
    let cd = corpus::plate_param::document();
    let t = Instant::now();
    let _ = f64_run(&cd.doc);
    println!("plate_param one f64 evaluation: {:.2?}", t.elapsed());
    let field = RangeField::Param(name(corpus::plate_param::HOLE_R));
    ladder("plate_param hole_r", &cd.doc, &field, RangeSeed::symmetric(0.01), &[4, 16, 64]);
}

/// The tour's die (77 nodes, 21 declared subtracts): its +z pip's
/// extrusion distance slot on a leaf-budget ladder.
#[test]
fn r2_cost_die_ladder() {
    let cd = corpus::die::document();
    let field = RangeField::Slot {
        node: cd.bump_root,
        slot: SlotId::Distance,
    };
    let t = Instant::now();
    let d = derive(&cd.doc, &field, RangeSeed::symmetric(0.01), tol()).unwrap();
    println!("die derive: {:.2?}, nominal {}", t.elapsed(), d.nominal);
    let t = Instant::now();
    let _ = f64_run(&cd.doc);
    println!("die one f64 evaluation: {:.2?}", t.elapsed());
    ladder("die pz Distance", &cd.doc, &field, RangeSeed::symmetric(0.01), &[2, 8, 32]);
}

// ------------------------------------------------------------ misc

/// The parametric-polygon finding, reproduced on the bowtie (a
/// parameter in one vertex) at a 100 ε seed.
#[test]
fn r2_parametric_polygon_vertex_at_100_eps() {
    let doc = bowtie(0.5);
    let field = RangeField::Param(name("x"));
    for w in [1e-7, 1e-3, 1e-1] {
        let rows = ledger(&doc, &field, RangeSeed::symmetric(w), &budget(24, 64));
        print_runs(&format!("bowtie ±{w:e}"), &rows);
    }
}

/// A document whose OTHER parameter carries a distribution: what the
/// derived document keeps of it, and whether the certificate covers
/// that parameter's spread at all.
#[test]
fn r2_other_parameter_spread_is_dropped() {
    let mut r = Recorder::new();
    declare(&mut r, "depth", 1.0);
    r.push(DocEdit::SetDocParam {
        name: name("side"),
        value: DocParam::continuous_with(
            Dimension::Length,
            1.0,
            editor_core::Distribution::Uniform { lo: -0.3, hi: 0.3 },
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
    let d = derive(&doc, &RangeField::Param(name("depth")), RangeSeed::symmetric(0.25), tol())
        .unwrap();
    println!("derived params: {:?}", d.doc.params());
    let e = certified_range(
        &doc,
        &RangeField::Param(name("depth")),
        RangeSeed::symmetric(0.25),
        &budget(24, 256),
        tol(),
    );
    match e {
        Ok(r) => println!("lo {} hi {}", arm(r.lo()), arm(r.hi())),
        Err(e) => println!("refused: {e}"),
    }
}
