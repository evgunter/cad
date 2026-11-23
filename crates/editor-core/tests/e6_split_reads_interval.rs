//! **The E6 split follows the deciding node** (ERROR-DESIGN E6's split
//! rule). A leaf that is undecided is split on a variable its deciding
//! node reads — the first undecided node in evaluation order — and on
//! no other: of those with depth left, the relatively widest, ties to
//! the earliest declared. A deciding node that reads no variable of the
//! box refuses the leaf, naming the node; a read set at its depth
//! budget is `Budget`.
//!
//! Each row names the change to `drive::bisect`, `drive::classify_replay`
//! or `Doc::free_reads` that turns it red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::fixture;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    BudgetKind, DriveConfig, ParamBoxVerdict, ReasonClass, Receipt, RefusalReason, Unsettled, drive,
};
use editor_core::{
    Dimension, Distribution, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileDoc,
    ProfileProgram, RecipeNodeId, UnitSym, VarId, VarName,
};
use fixture::{Recorder, len};
use geom_core::Tol;

fn eps() -> f64 {
    Tol::witness().eps()
}

fn read(name: &'static str) -> Formula {
    Formula::named(VarName::from_static(name), Dimension::Length)
}

/// Declares the length `name` at `nominal`, uniform over `± half`.
fn declare(r: &mut Recorder, name: &'static str, nominal: f64, half: f64) {
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static(name),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: nominal,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
}

/// The rectangle `[0, width] × [0, 1]`.
fn rect(width: Formula) -> LoopProgram<Formula> {
    let at = |x: Formula, y: f64| {
        editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([x, len(y)]))
    };
    LoopProgram::Chain(vec![
        editor_core::ProgramStep::At([len(0.0), len(0.0)]),
        at(width.clone(), 0.0),
        at(width, 1.0),
        at(len(0.0), 1.0),
        editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
    ])
}

/// A rectangle `width` wide extruded by `depth`: the profile, then the
/// extrude.
fn slab(r: &mut Recorder, width: Formula, depth: Formula) -> (RecipeNodeId, RecipeNodeId) {
    let profile = r.insert(Node::Profile(ProfileProgram {
        loops: vec![rect(width)],
        ids: Vec::new(),
        on: None,
    }));
    let extrude = r.insert(Node::Extrude {
        profile: profile.into(),
        distance: depth,
    });
    (profile, extrude)
}

/// **A depth no leaf holding zero decides**: `20ε ± 40ε`. The witness
/// builds; a box reaching zero holds the extrude's coincidence zone, so
/// its extrude is undecided at the root and in every leaf that still
/// touches zero.
const UNDECIDED: (f64, f64) = (20.0, 40.0);

/// A depth every leaf decides: `1 ± ε/16`.
const DECIDED: (f64, f64) = (1.0, 1.0 / 16.0);

fn declare_in_eps(r: &mut Recorder, name: &'static str, (nominal, half): (f64, f64)) {
    let scale = if nominal == 1.0 { 1.0 } else { eps() };
    declare(r, name, nominal * scale, half * eps());
}

/// A loft of two unit circles one unit apart: its arc sections refuse
/// at `Interval` whatever the box
/// (`work/inlane/an-enclosure-carrier-holds-point-weights-and-knots.md`),
/// and it reads no variable.
fn arc_loft(r: &mut Recorder) -> RecipeNodeId {
    let mut sections = Vec::new();
    for _ in 0..2 {
        sections.push(r.insert(Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(1.0),
            }],
            ids: Vec::new(),
            on: None,
        })));
    }
    r.insert(Node::Loft {
        first: sections[0].into(),
        then: vec![editor_core::LoftSection {
            id: None,
            profile: sections[1].into(),
            station: vec![editor_core::StationMove::Slide {
                along: editor_core::Axis3::Z,
                by: len(1.0),
            }],
        }],
        v_degree: Formula::count(1),
        first_id: None,
    })
}

fn run(doc: &ProfileDoc, config: DriveConfig) -> ParamBoxVerdict {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &config,
        &editor_core::Stop::default(),
        Tol::witness(),
    )
    .expect("the nominal builds")
}

fn leaves(max_leaves: usize) -> DriveConfig {
    DriveConfig {
        max_leaves,
        ..DriveConfig::default()
    }
}

fn var(doc: &ProfileDoc, name: &str) -> VarId {
    doc.var_named(name).expect("declared")
}

/// Every leaf's box, certified then refused.
fn boxes(v: &ParamBoxVerdict) -> Vec<&ParamBox> {
    v.certified()
        .iter()
        .map(|l| &l.box_)
        .chain(v.refused().iter().map(|l| &l.box_))
        .collect()
}

/// The distinct spans the leaves hold on `var`, as bits.
fn spans(v: &ParamBoxVerdict, var: VarId) -> BTreeSet<(u64, u64)> {
    boxes(v)
        .into_iter()
        .map(|b| {
            let (lo, hi) = b.get(var).expect("an axis of the box").span();
            (lo.to_bits(), hi.to_bits())
        })
        .collect()
}

fn whole(v: &ParamBoxVerdict, var: VarId) -> BTreeSet<(u64, u64)> {
    let (lo, hi) = v.root().get(var).expect("an axis of the box").span();
    BTreeSet::from([(lo.to_bits(), hi.to_bits())])
}

/// **A variable the deciding node does not read is never split.** The
/// first slab's extrude is undecided wherever `depth` reaches zero and
/// reads `depth` alone; `other`, declared first and as wide, feeds a
/// slab every leaf decides. Four leaves hold the whole of `other`.
///
/// Red when `drive::bisect` admits every varying axis (its `read(var)`
/// filter dropped): the root's tie then goes to `other`.
#[test]
fn a_variable_the_deciding_node_does_not_read_is_never_split() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "other", DECIDED);
    declare_in_eps(&mut r, "depth", UNDECIDED);
    slab(&mut r, len(1.0), read("depth"));
    slab(&mut r, len(1.0), read("other"));
    let v = run(&r.doc, leaves(4));
    assert!(
        v.receipt().holds() && v.receipt().splits >= 3,
        "{:?}",
        v.receipt()
    );
    assert_eq!(
        spans(&v, var(&r.doc, "other")),
        whole(&v, var(&r.doc, "other")),
        "a leaf was split on the variable its deciding node does not read"
    );
    assert!(
        spans(&v, var(&r.doc, "depth")).len() > 3,
        "the splits are all on the variable it reads: {:?}",
        v.receipt()
    );
}

/// **Ties go to the earliest-declared variable of the read set, and the
/// read set holds what the node reads through its inputs.** The extrude
/// is the deciding node: it reads `depth` itself and `width` through
/// its profile, and `unread`, declared before both, through nothing.
/// All three are at full relative width at the root, so the one split
/// two leaves allow is `width`'s; with four, each half then splits
/// `depth`, the relatively wider of the two it reads.
///
/// Red when `Doc::free_reads` keeps a node's own reads alone (the
/// operations it reads dropped): the root then splits `depth`. Red when
/// `ParamBox::split_axis` breaks a tie to the later variable (`>=`):
/// `depth` again. Red when `drive::bisect` admits every varying axis:
/// `unread`.
#[test]
fn a_tie_goes_to_the_earliest_variable_read_and_a_read_through_an_input_counts() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "unread", DECIDED);
    declare_in_eps(&mut r, "width", DECIDED);
    declare_in_eps(&mut r, "depth", UNDECIDED);
    slab(&mut r, read("width"), read("depth"));
    slab(&mut r, len(1.0), read("unread"));
    let (unread, width, depth) = (
        var(&r.doc, "unread"),
        var(&r.doc, "width"),
        var(&r.doc, "depth"),
    );

    let two = run(&r.doc, leaves(2));
    assert_eq!(two.receipt().splits, 1, "{:?}", two.receipt());
    assert_eq!(spans(&two, width).len(), 2, "the root's split is width's");
    assert_eq!(spans(&two, depth), whole(&two, depth));
    assert_eq!(spans(&two, unread), whole(&two, unread));

    let four = run(&r.doc, leaves(4));
    assert_eq!(four.receipt().splits, 3, "{:?}", four.receipt());
    assert_eq!(spans(&four, width).len(), 2, "each half of width, once");
    assert_eq!(spans(&four, depth).len(), 2, "then depth, the wider read");
    assert_eq!(spans(&four, unread), whole(&four, unread));
}

/// **A read through a definition counts.** The extrude's distance is
/// `throw`, defined as `depth`; `unread` is declared before both. The
/// one split two leaves allow is `depth`'s.
///
/// Red when `Doc::free_reads` stops at a defined variable instead of
/// following its definition: the extrude then reads nothing and the
/// root refuses `ReadsNoVariable`.
#[test]
fn a_read_through_a_definition_counts() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "unread", DECIDED);
    declare_in_eps(&mut r, "depth", UNDECIDED);
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("throw"),
        def: editor_core::VarDecl::Defined(read("depth")),
    });
    slab(&mut r, len(1.0), read("throw"));
    slab(&mut r, len(1.0), read("unread"));
    let v = run(&r.doc, leaves(2));
    assert_eq!(v.receipt().splits, 1, "{:?}", v.refused()[0].reason);
    assert_eq!(spans(&v, var(&r.doc, "depth")).len(), 2);
    assert_eq!(
        spans(&v, var(&r.doc, "unread")),
        whole(&v, var(&r.doc, "unread"))
    );
}

/// The human form says the node failed, with its cause, on the line
/// under its own class's count, the node mentioned in the driven
/// document `doc`. (The loft's cause names no other node; a cause that
/// does is `drive::tests::a_failed_leafs_cause_mentions_the_node_it_names`.)
fn assert_spoken_under_its_class(
    lines_said: &editor_core::Lines,
    leaves: usize,
    node: &editor_core::SpokenNode,
    doc: editor_core::DocumentId,
    cause: &editor_core::Sentence,
) {
    let said = lines_said.to_string();
    let lines: Vec<&str> = said.lines().collect();
    let (node, cause_text) = (
        editor_core::test_support::node_text(node),
        cause.to_string(),
    );
    let at = lines
        .iter()
        .position(|l| *l == format!("  {leaves} leaf/leaves refused reads_no_variable"))
        .unwrap_or_else(|| panic!("no count line for the class:\n{said}"));
    assert_eq!(
        lines.get(at + 1).copied(),
        Some(
            format!(
                "    {node} fails and reads no variable of the box: no split changes it \
                 ({cause_text})"
            )
            .as_str()
        ),
        "the node's line follows its class's count:\n{said}"
    );
    let line: Vec<_> = lines_said[at + 1].mentions().collect();
    assert!(
        line.first()
            .is_some_and(|m| m.node().is_some() && m.document() == Some(doc)),
        "the line mentions its node in the driven document: {line:?}"
    );
}

/// **A deciding node that reads no variable of the box refuses the
/// leaf, naming the node**: at the root, and at a child. The arc loft
/// is undecided at every box and reads nothing.
///
/// Alone under a box elsewhere it is the root's deciding node: one
/// leaf, refused `ReadsNoVariable` as a node that failed (the carrier's
/// refusal of an arc's weight, a `Skin` failure), priced under its own
/// class and written in both forms, the spoken line under its class. Behind a slab whose depth straddles zero the
/// slab's extrude speaks first at the root, which splits `depth`; a
/// child clear of zero decides the slab, the loft speaks, and that
/// child refuses for the loft.
///
/// Red when `drive()` prices `Unsplit::Unread` as a budget
/// (`BudgetKind::Resolution`), and when `drive::bisect` admits every
/// varying axis (the leaves then bisect to the leaf budget).
#[test]
fn a_deciding_node_that_reads_nothing_refuses_naming_it_at_the_root_and_at_a_child() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "w", DECIDED);
    let loft = arc_loft(&mut r);
    slab(&mut r, read("w"), len(1.0));
    let v = run(&r.doc, leaves(256));
    assert_eq!(
        v.receipt(),
        Receipt {
            certified: 0,
            refused: 1,
            splits: 0
        }
    );
    // The loft did not come back undecided: it failed, and the reason
    // says which failure.
    let RefusalReason::ReadsNoVariable {
        node,
        unsettled: Unsettled::Failed { class, cause },
    } = &v.refused()[0].reason
    else {
        panic!(
            "the root refuses for a node that failed: {:?}",
            v.refused()[0].reason
        );
    };
    assert_eq!((*node, *class), (loft, editor_core::NodeErrorClass::Skin));
    assert_eq!(v.refused()[0].reason.class(), ReasonClass::ReadsNoVariable);
    assert_eq!(
        v.accounting().refused.get(&ReasonClass::ReadsNoVariable),
        Some(&Ok(1.0)),
        "the whole box is priced under the reason's own class"
    );
    let line = format!("reads_no_variable {} failed:Skin", loft.full());
    assert!(
        v.serialize().contains(&line),
        "the goldening form names the node:\n{}",
        v.serialize()
    );
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    assert_spoken_under_its_class(
        &v.render(&r.doc, &analyzed),
        1,
        &r.doc.spoken(loft),
        r.doc.id(),
        cause,
    );

    let mut r = Recorder::new();
    declare_in_eps(&mut r, "depth", UNDECIDED);
    slab(&mut r, len(1.0), read("depth"));
    let loft = arc_loft(&mut r);
    let v = run(&r.doc, leaves(8));
    let depth = var(&r.doc, "depth");
    assert!(
        v.receipt().holds() && v.receipt().splits >= 1,
        "{:?}",
        v.receipt()
    );
    let named: Vec<_> = v
        .refused()
        .iter()
        .filter(|l| {
            matches!(&l.reason, RefusalReason::ReadsNoVariable { node, unsettled: Unsettled::Failed { .. } } if *node == loft)
        })
        .collect();
    assert!(
        !named.is_empty(),
        "no child refused for the loft: {:?}",
        v.refused().iter().map(|l| &l.reason).collect::<Vec<_>>()
    );
    let analyzed = analyzed_box(&r.doc, &AnalysisPolicy::default());
    let RefusalReason::ReadsNoVariable {
        unsettled: Unsettled::Failed { cause, .. },
        ..
    } = &named[0].reason
    else {
        unreachable!("filtered above");
    };
    assert_spoken_under_its_class(
        &v.render(&r.doc, &analyzed),
        named.len(),
        &r.doc.spoken(loft),
        r.doc.id(),
        cause,
    );
    for leaf in named {
        assert_ne!(
            leaf.box_.get(depth).map(BoxAxis::span),
            v.root().get(depth).map(BoxAxis::span),
            "the root's deciding node is the slab's extrude, which reads depth"
        );
    }
}

/// **A read set at its depth budget is `Budget`, as it was, and the
/// variables outside it are still not split.** One halving is allowed
/// per axis. The root splits `depth`; both halves touch zero, so the
/// extrude is undecided in each with `depth` spent, and each refuses
/// `Budget(Depth)` although `other` has its halving left.
///
/// Red when `drive::bisect` asks for depth before it asks whether the
/// node reads anything (a spent read set then reads as
/// `ReadsNoVariable`), and when it admits every varying axis (`other`
/// is then split).
#[test]
fn a_read_set_at_its_depth_budget_refuses_budget_and_splits_nothing_else() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "other", DECIDED);
    declare_in_eps(&mut r, "depth", UNDECIDED);
    slab(&mut r, len(1.0), read("depth"));
    slab(&mut r, len(1.0), read("other"));
    let v = run(
        &r.doc,
        DriveConfig {
            max_depth: 1,
            ..leaves(64)
        },
    );
    assert_eq!(v.receipt().splits, 1, "{:?}", v.receipt());
    assert_eq!(
        spans(&v, var(&r.doc, "other")),
        whole(&v, var(&r.doc, "other"))
    );
    let reasons: Vec<_> = v.refused().iter().map(|l| &l.reason).collect();
    assert!(
        reasons.contains(&&RefusalReason::Budget(BudgetKind::Depth { max_depth: 1 })),
        "{reasons:?}"
    );
    assert!(
        reasons
            .iter()
            .all(|r| !matches!(r, RefusalReason::ReadsNoVariable { .. })),
        "a spent read set is a budget, not a node that reads nothing: {reasons:?}"
    );
}

/// **The first undecided node in evaluation order chooses the axis.**
/// Two slabs, each undecided at the root: the first inserted reads
/// `second`, the later-declared variable, and the second reads `first`.
/// The one split two leaves allow is on `second`.
///
/// Red when `drive::classify_replay` lets the last undecided node speak
/// (its walk reversed), and when `drive::bisect` admits every varying
/// axis: both split `first`.
#[test]
fn the_first_undecided_node_in_evaluation_order_chooses_the_axis() {
    let mut r = Recorder::new();
    declare_in_eps(&mut r, "first", UNDECIDED);
    declare_in_eps(&mut r, "second", UNDECIDED);
    slab(&mut r, len(1.0), read("second"));
    slab(&mut r, len(1.0), read("first"));
    let v = run(&r.doc, leaves(2));
    assert_eq!(v.receipt().splits, 1, "{:?}", v.receipt());
    assert_eq!(spans(&v, var(&r.doc, "second")).len(), 2);
    assert_eq!(
        spans(&v, var(&r.doc, "first")),
        whole(&v, var(&r.doc, "first"))
    );
}

/// **The read set is complete over the corpus**: moving one free
/// variable moves the content key of no node whose read set lacks it.
/// Each continuous free variable of each corpus document is moved
/// alone, by a part in `2^20` of its size, and every node that still
/// builds is compared with the nominal run (the moved run's prior, so a
/// node whose key holds is served and only a moved one is built). At
/// `f64`: the read set is the document's, whatever scalar replays it,
/// and a key hashes the scalar's bits.
///
/// A node that fails at the moved value has no key to compare and is
/// passed over, so the row speaks for nodes that build.
///
/// Red when `Doc::free_reads` keeps a node's own reads alone, or stops
/// following a read at its definition.
#[test]
fn moving_a_variable_moves_no_key_outside_its_readers() {
    fn run(
        doc: &ProfileDoc,
        prior: Option<&editor_core::Evaluation<f64>>,
    ) -> editor_core::Evaluation<f64> {
        editor_core::evaluate::<f64>(
            doc,
            prior,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            Tol::witness(),
        )
    }
    let mut compared = 0usize;
    let mut moved = 0usize;
    for d in crate::corpus::documents() {
        let reads = d.doc.free_reads();
        let nominal = run(&d.doc, None);
        let continuous: Vec<(VarId, f64)> = d
            .doc
            .free_vars()
            .filter_map(|(id, free)| match free {
                FreeVar::Continuous { value, .. } => Some((id, *value)),
                _ => None,
            })
            .collect();
        for (var, value) in continuous {
            let to = value + value.abs().max(1.0) / f64::from(1u32 << 20);
            let edit = DocEdit::SetVarValue {
                var: editor_core::VarRef::Id(var),
                value: editor_core::FreeValue::Continuous(to),
            };
            let there = editor_core::apply(&d.doc, &edit, Tol::witness())
                .unwrap_or_else(|e| panic!("{}: a value edit applies: {e:?}", d.name))
                .doc;
            let at = run(&there, Some(&nominal));
            for &node in &nominal.order {
                let (Some(a), Some(b)) = (nominal.value(node), at.value(node)) else {
                    continue;
                };
                compared += 1;
                if a.content_key == b.content_key {
                    continue;
                }
                moved += 1;
                assert!(
                    reads.get(&node).is_some_and(|set| set.contains(&var)),
                    "{}: moving {} moved the key of {}, whose read set lacks it",
                    d.name,
                    editor_core::test_support::var_text(&d.doc.spoken_var(var)),
                    editor_core::test_support::node_text(&d.doc.spoken(node))
                );
            }
        }
    }
    // The witness is static: the corpus holds read variables, so keys move.
    assert!(
        moved > 0 && compared > moved,
        "{moved} of {compared} keys moved"
    );
}
