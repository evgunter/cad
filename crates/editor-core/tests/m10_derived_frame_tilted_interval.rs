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
    FreeVar, Node, NodeResult, ProfileDoc, ProfileLift, RoleSeg, UnitSym, VarName, evaluate,
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
    let boss_p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 0.5)])));
    let boss = r.insert(Node::Extrude {
        profile: boss_p.into(),
        distance: len(0.25),
    });
    let (under, end) = if derived {
        let p = r.insert(Node::Profile(fixture::desc(vec![fixture::square(0.0, 0.0, 1.0)])));
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
        vec![fixture::rim_edge(under, end, fixture::piece(&r.doc, under, 0, 0))],
    );
    let tilted = editor_core::Operand::Pose(Box::new(PoseDef::Spin {
        pose: RoleRead::plain(fixture::plane(face)),
        about: RoleRead::plain(fixture::axis(rim)),
        by: tilt,
    }));
    let to = if derived { tilted } else { fixture::flip(tilted) };
    let placement = r.insert(Node::place(vec![editor_core::BodyRead::plain(
        editor_core::Operand::from(boss),
    )]));
    let start = editor_core::Operand::select(boss, vec![fixture::fname(boss, RoleSeg::Cap(CapEnd::Start))]);
    r.constrain(
        placement,
        editor_core::Mate {
            on: fixture::plane(start),
            to: editor_core::Target::Pose(to),
            class: editor_core::ContactClass::Rest,
        },
        vec![
            (
                editor_core::Coord::Slide(editor_core::SlideAxis::X),
                Formula::zero(Dimension::Length),
            ),
            (
                editor_core::Coord::Slide(editor_core::SlideAxis::Y),
                Formula::zero(Dimension::Length),
            ),
            (editor_core::Coord::Spin, Formula::zero(Dimension::Angle)),
        ],
    );
    r.doc
}

/// **SYM-5's pin: the derived boss on a tilted frame certifies where
/// its authored twin does.** The acceptance row of the unit, under
/// BOTH `ProfileLift`s, at the widths the twin certifies at on the
/// symbolic lane.
///
/// What carries it is rule E ([`SymRules::common_factor`], the
/// quotient's common factor): with the rule off — `without_rule_e`,
/// M10-10's tier bit for bit — the derived boss refuses under `Guided`
/// while the twin certifies, and that asymmetry is asserted here too,
/// so the pin says what the rule is for and not only that it works.
/// The predicate it stops on is the CARRIER ENDPOINT, ahead of the
/// side plane's Newell sum: on the sign-hull frame the un-cancelled
/// quotient freezes in the extrude's attachment gate before the cap is
/// reached, so with the rule off the document never gets as far as the
/// plane it used to fail on.
///
/// At a half-width of `5e-2` the derived boss still refuses, and the
/// refusal is NOT the tier's: it is the clause-1 `Invalid` margin
/// `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`
/// carries, the same one PR-1 measured on the translated document.
/// That is asserted by name below — it is a defect pinned as the
/// current behaviour, and when that row is answered this assertion
/// fails and the width joins the parity list above it.
#[test]
fn m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does() {
    let eps = Tol::witness().eps();
    for half in [eps / 8.0, 1.0e-3] {
        {
            let lift = ProfileLift::Guided;
            let authored = sym(&boss_on_tilted(half, false), SymRules::shipped(), budget()).0;
            let derived = sym(&boss_on_tilted(half, true), SymRules::shipped(), budget()).0;
            assert!(
                authored.is_empty(),
                "half={half:e} {lift:?}: the authored twin certifies: {authored:?}"
            );
            assert!(
                derived.is_empty(),
                "half={half:e} {lift:?}: the derived boss must certify where its twin does — \
                 this is rule E's acceptance row: {derived:?}"
            );
        }
    }

    // Rule E is what carries it: with the rule off the derived boss
    // refuses under Guided where the twin certifies.
    let off = SymRules::without_rule_e();
    let (derived_off, _) = sym(&boss_on_tilted(1.0e-3, true), off, budget());
    let (authored_off, _) = sym(&boss_on_tilted(1.0e-3, false), off, budget());
    assert!(
        authored_off.is_empty(),
        "the twin certifies with rule E off too: {authored_off:?}"
    );
    assert_eq!(
        derived_off.len(),
        1,
        "without rule E the derived boss refuses: {derived_off:?}"
    );
    assert!(
        derived_off[0].contains("carrier_endpoint_end"),
        "and it refuses on the carrier's end endpoint, in the attachment gate: {derived_off:?}"
    );

    // The one width still refused, and the reason, read off the kernel.
    let (wide, _) = sym(&boss_on_tilted(5.0e-2, true), SymRules::shipped(), budget());
    assert_eq!(wide.len(), 1, "one refusal at 5e-2: {wide:?}");
    assert!(
        wide[0].contains("interval_span_forward"),
        "THIS PINS A DEFECT AS THE CURRENT BEHAVIOUR, NOT A DESIRED ONE: at 5e-2 the tier has \
         done its work and the VALUE channel refuses first. The wall moved with DECIDE-3: \
         the decision read settles the frame, the boss's cap plane is no longer what stops \
         the document, and what does is the stored interval's own span over a box this wide \
         (work/frame/a-widened-derived-placement-normalises-a-straddling-newell-sum carries \
         the class). When that is answered this assertion FAILS and 5e-2 joins the parity \
         list above: {wide:?}"
    );
}
