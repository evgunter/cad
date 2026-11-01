//! **A body placed on a pose whose AXES carry the parameter** — SYM-5's
//! second document class, and the one the item
//! `derived-frame-placement-freezes-on-the-symbolic-lane` was filed
//! on. `m10_derived_frame_interval`'s document is a pure
//! z-TRANSLATION: every normalised quantity in it is constant in the
//! parameter. Tilt the pose a boss rests on, so its AXES carry the
//! parameter, and the boss's placement carries a turn that is a
//! function of it.
//!
//! The rows that measured a profile lifted onto a tilted datum frame
//! and a face frame derived from it went with the datum; the
//! acceptance row stays, on the restated document.
//!
//! The preamble this file shares with its siblings — `failures`, the
//! `budget()` constants — is copied across the `m10_*` family; the
//! class is `work/sym/interval-test-preamble-is-copied-across-the-m10-files`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture::{self, Recorder, len, scl};

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{
    CancelToken, CapEnd, Dimension, Distribution, DocEdit, EvalOptions, Evaluation, Formula,
    FreeVar, Node, NodeResult, ProfileDoc, RoleSeg, UnitSym, VarName, evaluate,
};
use geom_core::{Interval, SymBudget, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

fn failures<T: geom_core::Decide>(ev: &Evaluation<T>) -> Vec<String> {
    ev.order
        .iter()
        .filter_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => {
                Some(format!("node {} — {} — {:?}", id.0, e.kind, e.kind))
            }
            Some(NodeResult::Poisoned { through }) => {
                Some(format!("node {} poisoned through {}", id.0, through.0))
            }
            _ => None,
        })
        .collect()
}

fn opts(doc: &editor_core::ProfileDoc) -> EvalOptions {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    }
}

fn sym(
    doc: &ProfileDoc,
    rules: SymRules,
    budget: SymBudget,
) -> (Vec<String>, geom_core::SymCounts) {
    let o = opts(doc);
    geom_core::sym::with_session_rules(budget, rules, || {
        let ev: Evaluation<geom_core::Sym<Interval>> =
            evaluate(doc, None, &CancelToken::new(), &o, Tol::witness());
        failures(&ev)
    })
}

/// **The tilted boss**: a boss whose copy rests on a pose turned
/// `atan2(t, 1)` about its own x — the turn that takes `y` to
/// `(0, 1, t)` — with `t` the widened parameter. `derived`: the pose is
/// a unit cube's top cap, turned about its front rim (the stored unit
/// vectors read off the cube). Otherwise it is the boss's own start
/// cap, turned about its own front rim and faced up.
pub(crate) fn boss_on_tilted(half: f64, derived: bool) -> ProfileDoc {
    use editor_core::pose::{PoseDef, RoleRead};
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("t"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Scalar,
            value: 0.25,
            display_unit: UnitSym::canonical_for(Dimension::Scalar),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
    let t = Formula::named(VarName::from_static("t"), Dimension::Scalar);
    let tilt = Formula::atan2(t, scl(1.0)).expect("an angle");
    let boss_p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
        0.0, 0.0, 0.5,
    )])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(0.25),
    });
    let (under, end) = if derived {
        let p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(
            0.0, 0.0, 1.0,
        )])));
        let cube = r.insert(Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        });
        (cube, CapEnd::End)
    } else {
        (boss, CapEnd::Start)
    };
    let face = editor_core::Operand::select(under, vec![fixture::fname(under, RoleSeg::Cap(end))]);
    let rim = editor_core::Operand::select(
        under,
        vec![fixture::rim_edge(
            under,
            end,
            fixture::piece(&r.doc, under, 0, 0),
        )],
    );
    let tilted = editor_core::Operand::Pose(Box::new(PoseDef::Spin {
        pose: RoleRead::plain(fixture::plane(face)),
        about: RoleRead::plain(fixture::axis(rim)),
        by: tilt,
    }));
    let to = if derived {
        tilted
    } else {
        fixture::flip(tilted)
    };
    let placement = r.insert(crate::fixture::stood(editor_core::BodyRead::plain(
        editor_core::Operand::from(boss),
    )));
    let start = editor_core::Operand::select(
        boss,
        vec![fixture::fname(boss, RoleSeg::Cap(CapEnd::Start))],
    );
    r.constrain(
        placement,
        editor_core::Mate {
            on: fixture::plane(start),
            to,
            // The start cap faces the plane's normal unflipped: the copy
            // stands upended, its `z` along `−z`.
            orient: editor_core::Orientation::new(
                editor_core::SignedAxis::PlusX,
                editor_core::SignedAxis::MinusZ,
            )
            .expect("x and z lie along two axes"),
        },
        vec![
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                editor_core::Coord::Slide(editor_core::TargetAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (editor_core::Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    r.doc
}

/// **SYM-5's pin: the derived boss on a tilted frame certifies where
/// its authored twin does.** The acceptance row of the unit, at the
/// widths the twin certifies at on the symbolic lane.
///
/// What carried it was rule E ([`SymRules::common_factor`], the
/// quotient's common factor): with the rule off — `without_rule_e`,
/// M10-10's tier bit for bit — the derived boss refused under `Guided`
/// on the carrier's end endpoint while the twin certified. Under D's
/// placement solve the tilted copy's map is composed from its charts'
/// frames, and the derived boss certifies with the rule off too, which
/// is asserted here so the row says what the rule no longer has to do.
///
/// At a half-width of `5e-2` the derived boss used to refuse on the
/// stored interval's own span (`interval_span_forward`, the class
/// `work/frame/a-widened-derived-placement-normalises-a-straddling-newell-sum`
/// carries); it certifies there now, and the width joins the parity
/// list.
#[test]
fn m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does() {
    let eps = Tol::witness().eps();
    for half in [eps / 8.0, 1.0e-3, 5.0e-2] {
        let authored = sym(&boss_on_tilted(half, false), SymRules::shipped(), budget()).0;
        let derived = sym(&boss_on_tilted(half, true), SymRules::shipped(), budget()).0;
        assert!(
            authored.is_empty(),
            "half={half:e}: the authored twin certifies: {authored:?}"
        );
        assert!(
            derived.is_empty(),
            "half={half:e}: the derived boss must certify where its twin does — this is rule \
             E's acceptance row: {derived:?}"
        );
    }

    // Rule E no longer carries it: under D's placement solve the tilted
    // copy's map is composed from its charts' frames, and with the rule
    // off the derived boss certifies where the twin does too.
    let off = SymRules::without_rule_e();
    let (derived_off, _) = sym(&boss_on_tilted(1.0e-3, true), off, budget());
    let (authored_off, _) = sym(&boss_on_tilted(1.0e-3, false), off, budget());
    assert!(
        authored_off.is_empty(),
        "the twin certifies with rule E off too: {authored_off:?}"
    );
    assert!(
        derived_off.is_empty(),
        "the derived boss certifies with rule E off too: {derived_off:?}"
    );
}
