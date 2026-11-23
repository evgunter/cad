//! Reviewer probes for the split-reads change (PR 7458). Every row is
//! `#[ignore]`d: each writes what it measured to `$REVIEW_OUT` for a
//! comparison taken outside the suite. Run with
//! `REVIEW_OUT=<dir> cargo nextest run -p editor-core --run-ignored only -E 'test(/review_split_reads_probes/)'`.
//! The `base_*` rows compile on the merge base too.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use crate::fixture;
use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, certifying_vector, drive};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, UnitSym, VarId, VarName,
};
use fixture::{Recorder, len};
use geom_core::Tol;

fn out(name: &str, body: &str) {
    let dir = std::env::var("REVIEW_OUT").expect("REVIEW_OUT names the output directory");
    std::fs::write(format!("{dir}/{name}"), body).expect("writable");
}

fn param(n: &'static str) -> Formula {
    Formula::named(VarName::from_static(n), Dimension::Length)
}

fn plate(w: f64) -> ProfileDoc {
    let mut r = Recorder::new();
    for (n, v) in [("hole_r", 0.25), ("depth", 0.5)] {
        r.push(DocEdit::DeclareVar {
            name: VarName::from_static(n),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: v,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(Distribution::Uniform { lo: -w, hi: w }),
            }),
        });
    }
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![
            LoopProgram::polygon([(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]).unwrap(),
            LoopProgram::Circle {
                centre: [len(1.0), len(1.0)],
                radius: param("hole_r"),
            },
        ],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: p.into(),
        distance: param("depth"),
    });
    r.doc
}

fn run(doc: &ProfileDoc, max_leaves: usize) -> Option<ParamBoxVerdict> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_leaves,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        Tol::witness(),
    )
    .ok()
}

fn eval_f64(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    editor_core::evaluate::<f64>(
        doc,
        None,
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    )
}

fn at(doc: &ProfileDoc, var: VarId, to: f64) -> Option<ProfileDoc> {
    editor_core::apply(
        doc,
        &DocEdit::SetVarValue {
            var: editor_core::VarRef::Id(var),
            value: editor_core::FreeValue::Continuous(to),
        },
        Tol::witness(),
    )
    .ok()
    .map(|o| o.doc)
}

/// One line per leaf: class, then each axis's span as bits; and for a
/// flip crossing, how many of a 9x9 grid of f64 points in its box give a
/// certifying vector other than the witness's.
fn leaves(doc: &ProfileDoc, v: &ParamBoxVerdict, sample_flips: bool) -> String {
    let mut s = String::new();
    let vars: Vec<VarId> = v.root().varying().map(|(id, _, _)| id).collect();
    let line = |s: &mut String, class: &str, b: &editor_core::analysis::ParamBox| {
        let _ = write!(s, "{class}");
        for &id in &vars {
            let (lo, hi) = b.get(id).unwrap().span();
            let _ = write!(s, " {:016x} {:016x}", lo.to_bits(), hi.to_bits());
        }
    };
    let witness = v.witness_vector().clone();
    let all: Vec<(String, &editor_core::analysis::ParamBox)> = v
        .certified()
        .iter()
        .map(|l| ("certified".to_owned(), &l.box_))
        .chain(v.refused().iter().map(|l| (format!("{:?}", l.reason.class()), &l.box_)))
        .collect();
    for (class, b) in all {
        line(&mut s, &class, b);
        if sample_flips && vars.len() == 2 {
            let (a0, a1) = b.get(vars[0]).unwrap().span();
            let (b0, b1) = b.get(vars[1]).unwrap().span();
            let nominal = |id: VarId| match doc.free_vars().find(|(v, _)| *v == id).unwrap().1 {
                FreeVar::Continuous { value, .. } => *value,
                FreeVar::Count { .. } => unreachable!(),
            };
            let (n0, n1) = (nominal(vars[0]), nominal(vars[1]));
            let mut differ = 0;
            for i in 0..9 {
                for j in 0..9 {
                    let x = n0 + a0 + (a1 - a0) * f64::from(i) / 8.0;
                    let y = n1 + b0 + (b1 - b0) * f64::from(j) / 8.0;
                    let d = at(doc, vars[0], x).and_then(|d| at(&d, vars[1], y)).unwrap();
                    if certifying_vector(&d, &eval_f64(&d)) != witness {
                        differ += 1;
                    }
                }
            }
            let _ = write!(s, " differ={differ}/81");
        }
        s.push('\n');
    }
    s
}

#[test]
#[ignore = "reviewer probe: writes to REVIEW_OUT"]
fn base_plate_overlay() {
    let doc = plate(0.16);
    for n in [16, 64] {
        let v = run(&doc, n).unwrap();
        out(
            &format!("plate-{n}.txt"),
            &format!("{:?}\n{}", v.receipt(), leaves(&doc, &v, true)),
        );
    }
}

/// Every corpus document with every continuous free variable widened
/// to a uniform `± rel·max(|v|, 1)`, driven to 4 leaves, for at most 25 minutes.
#[test]
#[ignore = "reviewer probe: writes to REVIEW_OUT"]
fn base_corpus_drives() {
    let mut all = String::new();
    let t0 = std::time::Instant::now();
    for d in crate::corpus::documents() {
        for rel in [2e-2] {
            if t0.elapsed().as_secs() > 1500 {
                break;
            }
            let mut doc = d.doc.clone();
            let vars: Vec<(VarId, f64)> = doc
                .free_vars()
                .filter_map(|(id, f)| match f {
                    FreeVar::Continuous { value, .. } => Some((id, *value)),
                    FreeVar::Count { .. } => None,
                })
                .collect();
            for (id, value) in vars {
                let w = rel * value.abs().max(1.0);
                if let Ok(o) = editor_core::apply(
                    &doc,
                    &DocEdit::SetVarDistribution {
                        var: editor_core::VarRef::Id(id),
                        distribution: Some(Distribution::Uniform { lo: -w, hi: w }),
                    },
                    Tol::witness(),
                ) {
                    doc = o.doc;
                }
            }
            let t = std::time::Instant::now();
            let body = match run(&doc, 4) {
                Some(v) => format!("{:?}\n{}", v.receipt(), leaves(&doc, &v, false)),
                None => "drive refused up front\n".to_owned(),
            };
            let _ = writeln!(all, "== {} rel={rel:e} {:.1}s", d.name, t.elapsed().as_secs_f64());
            all.push_str(&body);
            out("corpus.txt", &all);
        }
    }
}
