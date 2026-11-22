//! Reviewer probes for PR 7369 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, certifying_vector, witness_options};
use editor_core::range::{
    CertifiedRange, RangeField, RangeSearch, RangeSeed, Step, certified_range, derive,
};
use editor_core::{
    Dimension, DocEdit, Evaluation, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, VarName, evaluate,
};
use fixture::{Recorder, len, scl, tol};

fn name(n: &'static str) -> VarName {
    VarName::from_static(n)
}
fn param(n: &'static str) -> Formula {
    Formula::named(name(n), Dimension::Length)
}
fn declare(r: &mut Recorder, n: &'static str, value: f64) {
    r.push(DocEdit::DeclareVar {
        name: name(n),
        def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
    });
}
fn budget(max_depth: u32, max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_depth,
        max_leaves,
        ..DriveConfig::default()
    }
}

/// docm9's notch: bottom vertex (0.5, h); crossing at h = 0.
fn notch(h: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "h", h);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [len(0.5), param("h")],
            [len(1.0), len(0.0)],
            [len(1.0), len(1.0)],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

/// Two boundaries, one each side: bottom vertex (0.5, h) crosses at
/// h = 0, top vertex (0.5, 1.3 + 2h) crosses at h = -0.15.
fn two_sided(h: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "h", h);
    let top = Formula::add(
        len(1.3),
        Formula::mul(param("h"), scl(2.0)).expect("len*scalar"),
    )
    .expect("len+len");
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [len(0.0), len(0.0)],
            [len(0.5), param("h")],
            [len(1.0), len(0.0)],
            [len(1.0), len(1.0)],
            [len(0.5), top],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

/// A field at ZERO: the first corner's x. At x = 1 the corner meets
/// (1, 0); past it the loop turns over.
fn corner_x(x: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    declare(&mut r, "h", x);
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::polygon_expr([
            [param("h"), len(0.0)],
            [len(1.0), len(0.0)],
            [len(1.0), len(1.0)],
            [len(0.0), len(1.0)],
        ])],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(1.0),
    });
    r.doc
}

fn set(doc: &ProfileDoc, v: f64) -> ProfileDoc {
    editor_core::apply(
        doc,
        &DocEdit::SetVarValue {
            var: name("h").into(),
            value: editor_core::FreeValue::Continuous(v),
        },
        tol(),
    )
    .expect("value edit")
    .doc
}

/// The certifying vector of the f64 build at absolute value `v`, built
/// exactly as the witness is (derived document, witness options).
fn vector_at(doc: &ProfileDoc, v: f64) -> editor_core::VerdictVector {
    let at = set(doc, v);
    let field = RangeField::Param(at.var_named("h").unwrap());
    let d = derive(&at, &field, RangeSeed::symmetric(1e-3), tol()).expect("derive");
    let analyzed = analyzed_box(&d.doc, &AnalysisPolicy::default());
    let stop = editor_core::Stop::default();
    let ev: Evaluation<f64> = evaluate(
        &d.doc,
        None,
        &stop.cancel,
        &witness_options(&analyzed, &stop),
        tol(),
    );
    let mut v = certifying_vector(&d.doc, &ev);
    if std::env::var("PROBE_STRICT").is_err() {
        for row in &mut v.rows {
            row.verdicts.retain(|x| !x.predicate.starts_with("canonical_order_"));
        }
    }
    v
}

fn next_up(x: f64) -> f64 {
    if x >= 0.0 {
        f64::from_bits(x.to_bits() + 1)
    } else {
        f64::from_bits(x.to_bits() - 1)
    }
}
fn next_down(x: f64) -> f64 {
    -next_up(-x)
}

/// Sample the proven interval (absolute values) densely, its exact
/// ends, a few ulps inside, and the point proven different on each side.
fn audit(label: &str, doc: &ProfileDoc, r: &CertifiedRange) -> Vec<String> {
    let mut bad = Vec::new();
    let w = vector_at(doc, r.nominal());
    let (lo_off, hi_off) = r.proven_offsets();
    let mut pts: Vec<f64> = Vec::new();
    for i in 0..=64 {
        let o = lo_off + (hi_off - lo_off) * f64::from(i) / 64.0;
        pts.push(r.absolute(o));
    }
    let (a, b) = r.proven_interval();
    let mut x = a;
    let mut y = b;
    for _ in 0..8 {
        pts.push(x);
        pts.push(y);
        x = next_up(x);
        y = next_down(y);
    }
    for v in pts {
        if v < a || v > b {
            continue;
        }
        if vector_at(doc, v) != w {
            bad.push(format!(
                "{label}: proven interval [{a:e}, {b:e}] covers {v:e} whose f64 build differs"
            ));
        }
    }
    for (side, s) in [("lo", r.lo()), ("hi", r.hi())] {
        if let Some(d) = s.different() {
            let v = r.absolute(d.at());
            if vector_at(doc, v) == w {
                bad.push(format!(
                    "{label}/{side}: proven different at {v:e} (offset {:e}) but f64 build equals the witness's",
                    d.at()
                ));
            }
        }
    }
    eprintln!(
        "{label}: nominal {} seed {:?} steps {} proven {:?} lo {:?} hi {:?}",
        r.nominal(),
        r.seed(),
        r.steps(),
        r.proven_offsets(),
        r.lo().end().map(|e| matches!(e, editor_core::range::SideEnd::Seed)),
        r.hi().end().map(|e| matches!(e, editor_core::range::SideEnd::Seed)),
    );
    eprintln!(
        "  lo nd {:?} diff {:?} | hi nd {:?} diff {:?}",
        r.lo().not_decided().map(|n| (n.past(), n.more_time_helps())),
        r.lo().different().map(|d| (d.at(), d.kind())),
        r.hi().not_decided().map(|n| (n.past(), n.more_time_helps())),
        r.hi().different().map(|d| (d.at(), d.kind())),
    );
    bad
}

fn run(doc: &ProfileDoc, seed: RangeSeed, config: &DriveConfig) -> CertifiedRange {
    certified_range(
        doc,
        &RangeField::Param(doc.var_named("h").unwrap()),
        seed,
        config,
        None,
        &editor_core::Stop::default(),
        tol(),
    )
    .expect("admitted")
}

#[test]
fn p1_soundness_sweep() {
    let mut bad = Vec::new();
    let cases: Vec<(&str, ProfileDoc, Option<RangeSeed>)> = vec![
        ("notch -0.125", notch(-0.125), Some(RangeSeed { lo: -0.0625, hi: 0.15 })),
        ("notch -0.125 of", notch(-0.125), None),
        ("notch -1e-7", notch(-1e-7), Some(RangeSeed::symmetric(3e-7))),
        ("notch -0.25 (gap)", notch(-0.25), Some(RangeSeed { lo: -0.0625, hi: 0.4 })),
        ("notch 0.2 island", notch(0.2), None),
        ("two_sided -0.125 of", two_sided(-0.125), None),
        ("two_sided -0.125 wide", two_sided(-0.125), Some(RangeSeed::symmetric(0.5))),
        ("corner_x 0 of", corner_x(0.0), None),
        ("corner_x 0.999999 of", corner_x(0.999_999), None),
    ];
    for (label, doc, seed) in cases {
        let seed = seed.unwrap_or_else(|| {
            RangeSeed::of(&doc, &RangeField::Param(doc.var_named("h").unwrap())).unwrap()
        });
        let r = run(&doc, seed, &budget(48, 400));
        bad.extend(audit(label, &doc, &r));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Determinism across mid-replay stops: cancel after t ms, resume, and
/// compare the answer after n steps with an uninterrupted search.
#[test]
fn p3_determinism_across_mid_replay_stops() {
    let doc = two_sided(-0.125);
    let field = RangeField::Param(doc.var_named("h").unwrap());
    let seed = RangeSeed::symmetric(0.125);
    let config = budget(48, 400);
    let new = || {
        RangeSearch::new(
            &doc,
            &field,
            seed,
            &config,
            None,
            &editor_core::Stop::default(),
            tol(),
        )
        .unwrap()
    };
    let n = 40;
    let mut clean = new();
    let mut reference = Vec::new();
    for _ in 0..n {
        assert_eq!(clean.step(&editor_core::Stop::default()), Step::Advanced);
        reference.push(clean.answer());
    }
    for delay_us in [50u64, 300, 1000, 3000, 10_000] {
        let mut s = new();
        let mut stops = 0;
        while s.steps() < n {
            let stop = editor_core::Stop::default();
            let c = stop.cancel.clone();
            let h = std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_micros(delay_us));
                c.cancel();
            });
            let before = s.answer();
            match s.step(&stop) {
                Step::Advanced => {
                    assert_eq!(
                        s.answer(),
                        reference[s.steps() - 1],
                        "delay {delay_us}us: after {} steps (with {stops} stops) the answer differs",
                        s.steps()
                    );
                }
                Step::Stopped(_) => {
                    stops += 1;
                    assert_eq!(s.answer(), before, "a stop changed the answer");
                }
                Step::Finished => panic!("finished early"),
            }
            h.join().unwrap();
            if stops > 2000 {
                break;
            }
        }
        eprintln!("delay {delay_us}us: {stops} mid-replay stops, steps {}", s.steps());
    }
}

/// How many replays the search spends after its answer last moved.
#[test]
fn p4_replays_spent_after_the_answer_stops_moving() {
    for (label, doc, seed, depth) in [
        ("notch", notch(-0.125), RangeSeed { lo: -0.0625, hi: 0.15 }, 24u32),
        ("notch d48", notch(-0.125), RangeSeed { lo: -0.0625, hi: 0.15 }, 48),
        ("two_sided", two_sided(-0.125), RangeSeed::symmetric(0.5), 24),
        ("corner_x", corner_x(0.0), RangeSeed::symmetric(2.0), 24),
    ] {
        let field = RangeField::Param(doc.var_named("h").unwrap());
        let config = budget(depth, 5000);
        let mut s = RangeSearch::new(
            &doc,
            &field,
            seed,
            &config,
            None,
            &editor_core::Stop::default(),
            tol(),
        )
        .unwrap();
        let key = |r: &CertifiedRange| {
            let side = |s: &editor_core::range::RangeSide| {
                (
                    s.proven_to(),
                    s.reached_seed_end(),
                    s.different().map(|d| d.at()),
                    s.not_decided().map(|n| (n.past(), n.more_time_helps())),
                )
            };
            (side(r.lo()), side(r.hi()))
        };
        let mut last = key(&s.answer());
        let mut moved_at = 0;
        let t = std::time::Instant::now();
        while s.step(&editor_core::Stop::default()) == Step::Advanced {
            let k = key(&s.answer());
            if k != last {
                moved_at = s.steps();
                last = k;
            }
        }
        eprintln!(
            "{label} depth {depth}: finished after {} replays, answer last moved at replay {moved_at} ({:?})",
            s.steps(),
            t.elapsed()
        );
    }
}

#[test]
fn p5_corner_x_vector_diff() {
    let doc = corner_x(0.0);
    let w = vector_at(&doc, 0.0);
    for v in [1e-12, 1e-6, 0.25, 0.5, 0.98, -1e-12, -0.5, -0.98] {
        let at = vector_at(&doc, v);
        eprintln!("v = {v:e}: equal = {}", at == w);
        for (a, b) in w.rows.iter().zip(at.rows.iter()) {
            if a != b {
                eprintln!("  node {:?}: witness {:?} {} verdicts", a.node, a.outcome, a.verdicts.len());
                eprintln!("  node {:?}: at v    {:?} {} verdicts", b.node, b.outcome, b.verdicts.len());
                for (i, (x, y)) in a.verdicts.iter().zip(b.verdicts.iter()).enumerate() {
                    if x != y {
                        eprintln!("    first differing verdict #{i}:\n      w: {x:?}\n      v: {y:?}");
                        break;
                    }
                }
            }
        }
    }
}

/// Does `drive` itself certify leaves either side of the corner's zero?
#[test]
fn p6_drive_on_corner_x() {
    for (nominal, w) in [(0.0, 0.5), (0.3, 0.6)] {
        let doc = corner_x(nominal);
        let field = RangeField::Param(doc.var_named("h").unwrap());
        let d = derive(&doc, &field, RangeSeed::symmetric(w), tol()).unwrap();
        let analyzed = analyzed_box(&d.doc, &AnalysisPolicy::default());
        let v = editor_core::drive::drive(
            &d.doc,
            &analyzed,
            &budget(6, 256),
            &editor_core::Stop::default(),
            tol(),
        )
        .unwrap();
        eprintln!("drive nominal {nominal} band ±{w}: {} certified, {} refused", v.certified().len(), v.refused().len());
        for c in v.certified().iter().take(80) {
            eprintln!("  certified {:?}", c.box_.axes().values().collect::<Vec<_>>());
        }
        let r = run(&doc, RangeSeed::symmetric(w), &budget(24, 200));
        eprintln!("  search: proven {:?}", r.proven_offsets());
        bad_probe(&doc, &r);
    }
}

fn bad_probe(doc: &ProfileDoc, r: &CertifiedRange) {
    for e in audit("corner", doc, r).iter().take(3) {
        eprintln!("  RED {e}");
    }
}
