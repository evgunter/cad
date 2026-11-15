//! PROBE (never merged): each arc verb extruded over a box on its
//! defining quantity, replayed at `Sym<Interval>`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget, UnitSym, VarName,
};
use geom_core::sym::report::ShapeOutcome;
use geom_core::{SymRules, Tol};

use crate::fixture::{Recorder, len, scl};
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{blocked, head};

fn named(n: &'static str, dim: Dimension) -> Formula {
    Formula::named(VarName::from_static(n), dim)
}

fn pt(x: Formula, y: Formula) -> ProgramTarget<Formula> {
    ProgramTarget::Point([x, y])
}

/// A 4 × 2 mm blank whose right side is one arc from `(4, 0)` to
/// `(4, 2)` mm, authored by `arc`, extruded 1 mm; `var` the arc's
/// defining quantity over `nominal ± half`.
pub(crate) fn one_arc(
    var: &'static str,
    dim: Dimension,
    nominal: f64,
    half: f64,
    arc: Vec<ProgramStep<Formula>>,
    top: bool,
) -> ProfileDoc {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(var),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim,
            value: nominal,
            display_unit: UnitSym::canonical_for(dim),
            distribution: Some(Distribution::Uniform { lo: -half, hi: half }),
        }),
    });
    let mut steps = vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::LineTo(pt(len(4.0e-3), len(0.0))),
    ];
    steps.extend(arc);
    if top {
        steps.push(ProgramStep::LineTo(pt(len(0.0), len(2.0e-3))));
    }
    steps.push(ProgramStep::LineTo(ProgramTarget::Start));
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(steps)],
        ids: Vec::new(),
        on: None,
    }));
    r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0e-3),
    });
    r.doc
}

pub(crate) fn verb_doc(name: &str) -> ProfileDoc {
    let top = || pt(len(4.0e-3), len(2.0e-3));
    let all: Vec<(&'static str, Box<dyn Fn() -> ProfileDoc>)> = vec![
        (
            // Tangent to +x at (4, 0), ending straight above at height
            // `h`: a half turn of radius h/2. The top edge follows `h`.
            "tangent",
            Box::new(move || {
                let mut r = Recorder::new();
                r.push(DocEdit::DeclareVar {
                    name: VarName::from_static("h"),
                    def: editor_core::VarDecl::Free(FreeVar::Continuous {
                        dim: Dimension::Length,
                        value: 2.0e-3,
                        display_unit: UnitSym::canonical_for(Dimension::Length),
                        distribution: Some(Distribution::Uniform {
                            lo: -5.0e-5,
                            hi: 5.0e-5,
                        }),
                    }),
                });
                let h = || named("h", Dimension::Length);
                let profile = r.insert(Node::Profile(ProfileProgram {
                    loops: vec![LoopProgram::Chain(vec![
                        ProgramStep::At([len(0.0), len(0.0)]),
                        ProgramStep::LineTo(pt(len(4.0e-3), len(0.0))),
                        ProgramStep::Tangent,
                        ProgramStep::TangentArcTo(pt(len(4.0e-3), h())),
                        ProgramStep::Tangent,
                        ProgramStep::Line(len(4.0e-3)),
                        ProgramStep::LineTo(ProgramTarget::Start),
                    ])],
                    ids: Vec::new(),
                    on: None,
                }));
                r.insert(Node::Extrude {
                    profile: profile.into(),
                    distance: len(1.0e-3),
                });
                r.doc
            }),
        ),
        (
            "via",
            Box::new(move || one_arc(
                "s",
                Dimension::Length,
                4.6e-3,
                5.0e-5,
                vec![ProgramStep::ArcTo(ProgramArcData::Via {
                    q: [named("s", Dimension::Length), len(1.0e-3)],
                    target: top(),
                })],
                true,
            )),
        ),
        (
            "via_cw",
            Box::new(move || one_arc(
                "s",
                Dimension::Length,
                3.4e-3,
                5.0e-5,
                vec![ProgramStep::ArcTo(ProgramArcData::Via {
                    q: [named("s", Dimension::Length), len(1.0e-3)],
                    target: top(),
                })],
                true,
            )),
        ),
        (
            "bulge",
            Box::new(move || one_arc(
                "b",
                Dimension::Scalar,
                0.5,
                0.02,
                vec![ProgramStep::ArcTo(ProgramArcData::Bulge {
                    target: top(),
                    b: named("b", Dimension::Scalar),
                })],
                true,
            )),
        ),
        (
            "bulge_cw",
            Box::new(move || one_arc(
                "b",
                Dimension::Scalar,
                -0.3,
                0.02,
                vec![ProgramStep::ArcTo(ProgramArcData::Bulge {
                    target: top(),
                    b: named("b", Dimension::Scalar),
                })],
                true,
            )),
        ),
        (
            "fillet",
            Box::new(move || one_arc(
                "r",
                Dimension::Length,
                0.5e-3,
                2.0e-5,
                vec![
                    ProgramStep::Toward {
                        dx: scl(0.0),
                        dy: scl(1.0),
                    },
                    ProgramStep::Fillet(named("r", Dimension::Length)),
                    ProgramStep::Toward {
                        dx: scl(-1.0),
                        dy: scl(0.0),
                    },
                    ProgramStep::FarEndTo([len(0.0), len(2.0e-3)]),
                ],
                false,
            )),
        ),
    ];
    let (_, build) = all
        .into_iter()
        .find(|(n, _)| *n == name)
        .expect("a named verb");
    build()
}

pub(crate) const VERBS: [&str; 6] = ["tangent", "via", "via_cw", "bulge", "bulge_cw", "fillet"];

#[test]
#[ignore = "probe"]
fn probe_each_verb_over_a_box() {
    let tol = Tol::witness();
    let only = std::env::var("P_VERB").ok();
    for name in VERBS {
        if only.as_deref().is_some_and(|o| !o.is_empty() && o != name) {
            continue;
        }
        let Ok(doc) = std::panic::catch_unwind(|| verb_doc(name)) else {
            println!("== {name}: THE FIXTURE DID NOT BUILD");
            continue;
        };
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, box_) in [
            ("nominal", ParamBox::of(&analyzed).at_nominal()),
            ("box", ParamBox::of(&analyzed)),
        ] {
            let (shapes, refusal, counts) = replay(&doc, &box_, SymRules::shipped(), tol);
            println!("== {name} / {label}: {counts:?}");
            println!(
                "   first refusal: {}",
                refusal.as_deref().map_or("none".to_owned(), |r| head(r, 1500))
            );
            for (pred, (invalid, indeterminate, all)) in blocked(&shapes) {
                println!("   blocked {pred}: invalid {invalid} indeterminate {indeterminate} of {all}");
            }
            for s in shapes
                .iter()
                .filter(|s| matches!(s.outcome, ShapeOutcome::Indeterminate))
                .take(2)
            {
                println!("   shape {}: {}", s.predicate, head(&format!("{s:?}"), 1200));
            }
        }
    }
}
