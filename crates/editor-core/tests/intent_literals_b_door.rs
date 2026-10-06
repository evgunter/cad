//! INTENT-LITERALS PR B: the edit door lowers an authored node, program
//! or offset in one walk, and every formula that walk reaches refuses
//! typed where it does not lower — never a panic.
//!
//! The review rows (p1–p4) pin the base's refusals: a count beside a
//! listed placement rule refuses the rule's own words, a definition of
//! another kind refuses its kind before its names, and a refused insert
//! speaks one id whether its held names were written by name or by id.
#![allow(clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::docm7_union_declare::block;
use crate::fixture::resolver::in_part;
use crate::fixture::{ang, head, len, scl};
use editor_core::{
    Alignment, AuthoredNode, AxisSense, CapEnd, ContactClass, CountMismatch, Dimension, DocEdit,
    DocRef, DocumentId, EditError, Formula, Frame, FreeVar, LoopProgram, MateFrame, MatePrimitive,
    Node, PatternKind, Placement, ProfileDoc, ProfileProgram, RecipeNodeId, RefusingReach, Step,
    TubeWindow, VarDecl, VarName, apply, content_pin,
};
use geom_core::Tol;
use std::collections::BTreeSet;

fn name(text: &'static str) -> VarName {
    VarName::from_static(text)
}

fn edit(doc: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> Result<ProfileDoc, EditError> {
    apply(doc, e, Tol::witness(), &RefusingReach).map(|a| a.doc)
}

fn body() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("intent-literals-b-door", Tol::witness());
    block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

fn insert(node: AuthoredNode) -> DocEdit<ProfileProgram> {
    DocEdit::InsertNode {
        node: Box::new(node),
    }
}

/// A placement of one rigid step, every component a formula.
fn rigid() -> Placement<Formula> {
    Placement {
        steps: vec![Step::Rigid {
            translation: [len(0.1), len(0.2), len(0.3)],
            axis: [scl(0.0), scl(0.0), scl(1.0)],
            angle: ang(0.5),
        }],
    }
}

/// A reference to `doc`, pinned at its content.
fn reference(doc: &ProfileDoc) -> DocRef {
    DocRef {
        id: doc.id(),
        pin: content_pin(doc, Tol::witness()).expect("the pin computes"),
    }
}

/// p1: a pattern listing its placements and also spelling a count, the
/// count an unheld name. The rule spells its count twice, and that is
/// the refusal, as it is whatever the count holds.
#[test]
fn a_listed_pattern_with_an_unheld_count_refuses_the_rule_not_a_panic() {
    let (doc, b) = body();
    let refused = edit(
        &doc,
        &insert(Node::Pattern {
            input: b,
            count: Formula::named(name("nope"), Dimension::Count),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        }),
    )
    .expect_err("the rule spells its count twice");
    assert!(
        matches!(
            refused,
            EditError::PlacementRuleMismatch {
                shape: CountMismatch::ListedOnPattern,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// p2: the placed-union twin of p1.
#[test]
fn a_listed_union_with_an_unheld_count_refuses_the_rule_not_a_panic() {
    let (doc, b) = body();
    let refused = edit(
        &doc,
        &insert(Node::PlacedUnion {
            input: b,
            count: Some(Formula::named(name("nope"), Dimension::Count)),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        }),
    )
    .expect_err("the rule spells its count twice");
    assert!(
        matches!(
            refused,
            EditError::PlacementRuleMismatch {
                shape: CountMismatch::ListedWithCount,
                ..
            }
        ),
        "{refused:?}"
    );
}

/// p3: a definition of another kind that also reads an unheld name
/// refuses its kind first, as the base did.
#[test]
fn a_definition_of_another_kind_refuses_its_kind_before_its_names() {
    let doc = ProfileDoc::empty(DocumentId::derive("intent-literals-b-p3"), Tol::witness());
    let doc = edit(
        &doc,
        &DocEdit::DeclareVar {
            name: name("n"),
            def: VarDecl::Free(FreeVar::Count { value: 4 }),
        },
    )
    .expect("a count declares");
    let refused = edit(
        &doc,
        &DocEdit::DefineVar {
            var: name("n").into(),
            def: VarDecl::Defined(Formula::named(name("nope"), Dimension::Length)),
        },
    )
    .expect_err("a count is not redefined as a length");
    assert!(
        matches!(refused, EditError::VarKindFixed { .. }),
        "{refused:?}"
    );
}

/// p4: a refused insert holding one held name and one unheld speaks one
/// id whether the held name was written by name or by id: the id is
/// drawn from the node with every held name lowered.
#[test]
fn a_refused_insert_speaks_one_id_by_name_or_by_id() {
    let (doc, b) = body();
    let applied = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: name("w"),
            def: VarDecl::Free(FreeVar::written_length(quantity::WrittenLength::in_unit(
                5.0,
                quantity::MM,
            ))),
        },
        Tol::witness(),
        &RefusingReach,
    )
    .expect("w declares");
    let w = applied.record.minted_var.expect("a declare mints");
    let doc = applied.doc;
    let node = |first: Formula| -> AuthoredNode {
        Node::Pattern {
            input: b,
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: Formula::add(first, Formula::named(name("nope"), Dimension::Length))
                    .expect("two lengths add"),
            },
        }
    };
    let by_name = edit(
        &doc,
        &insert(node(Formula::named(name("w"), Dimension::Length))),
    )
    .expect_err("nope is unheld");
    let by_id =
        edit(&doc, &insert(node(Formula::var(w, Dimension::Length)))).expect_err("nope is unheld");
    assert_eq!(by_name.to_string(), by_id.to_string());
}

/// The formula a walk visits `at`th replaced by an unheld name read at
/// its dimension, every other kept: one walk, the door's own.
fn named_at<T>(
    at: usize,
    map: impl FnOnce(&mut dyn FnMut(&Formula) -> Result<Formula, ()>) -> Result<T, ()>,
) -> (T, usize) {
    let mut seen = 0;
    let mapped = map(&mut |formula| {
        let here = seen;
        seen += 1;
        Ok(if here == at {
            Formula::named(name("nope"), formula.dim())
        } else {
            formula.clone()
        })
    })
    .expect("the walk refuses nothing");
    (mapped, seen)
}

/// Whether `refused` is a name refusal, at a slot or the payload, or
/// the placement rule's own (p1's edge).
fn names_the_name(refused: &EditError) -> bool {
    matches!(
        refused,
        EditError::SlotUnknownVarName { .. }
            | EditError::PayloadUnknownVarName { .. }
            | EditError::PlacementRuleMismatch { .. }
    )
}

/// The variant a node is, for the coverage count.
fn kind_of(node: &AuthoredNode) -> String {
    let debug = format!("{node:?}");
    debug
        .split(|c: char| !c.is_alphanumeric())
        .next()
        .unwrap_or_default()
        .to_owned()
}

/// **Every formula field the door's walk reaches, written as an unheld
/// name, refuses typed.** Over every node of every corpus document plus
/// p1/p2's listed-rule nodes: each node re-authored, and for each
/// formula `try_map_slots` visits — slots, payload leaves, a profile's
/// program, a placement's steps, a mate's frame offsets, the count no
/// slot addresses — that one formula an unheld name, inserted. Then the
/// same over every profile's program at `SetProgram`. A slot type the
/// walk reaches and the refusal does not is a red row, not a panic in
/// a caller.
#[test]
fn every_formula_the_door_walks_refuses_typed_as_an_unheld_name() {
    let (edge_doc, b) = body();
    // The listed-rule counts no slot addresses (p1/p2), and the slot
    // types no corpus document holds: a shell, a sweep, a windowed
    // tube, a gauge's and an instance's placements, a mate's frame
    // offsets. The door lowers before it reads an input, so the inputs
    // here need only be ids.
    let edges: Vec<AuthoredNode> = vec![
        Node::Pattern {
            input: b,
            count: Formula::count(2),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        },
        Node::PlacedUnion {
            input: b,
            count: Some(Formula::count(2)),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        },
        Node::shell(b, len(0.01), Vec::new()),
        Node::Sweep {
            profile: b,
            path: b,
            stations: Formula::count(4),
            v_degree: Formula::count(3),
        },
        Node::Tube {
            spine: b,
            u_ref: [scl(1.0), scl(0.0), scl(0.0)],
            major_radius: len(0.5),
            minor_radius: len(0.1),
            window: TubeWindow::Arc {
                t0: scl(0.0),
                t1: scl(0.5),
            },
        },
        Node::gauge(None, rigid()),
        Node::instantiate_part_with(
            reference(&edge_doc),
            editor_core::InterfaceRecord::default(),
            None,
            Some(rigid()),
        ),
        Node::Mate {
            a: head(in_part(b, b, CapEnd::End)),
            b: head(in_part(b, b, CapEnd::Start)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: MateFrame::on_part(rigid()),
                b: MateFrame::on_part(rigid()),
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    ];
    let mut cases: Vec<(String, ProfileDoc, AuthoredNode)> = edges
        .into_iter()
        .map(|node| ("edge".to_owned(), edge_doc.clone(), node))
        .collect();
    for d in corpus::documents() {
        for &id in d.doc.order() {
            let node = d.doc.node(id).expect("an ordered node is held").authored();
            cases.push((d.name.to_owned(), d.doc.clone(), node));
        }
    }

    let mut kinds = BTreeSet::new();
    let mut formulas = 0usize;
    let mut programs = 0usize;
    for (doc_name, doc, node) in &cases {
        kinds.insert(kind_of(node));
        let mut at = 0;
        loop {
            let (mutated, seen) = named_at(at, |f| {
                node.try_map_slots(|p, g| p.try_map_slots(&mut |e| g(e)), &mut |e| f(e))
            });
            if at >= seen {
                break;
            }
            formulas += 1;
            let refused = edit(doc, &insert(mutated)).expect_err("an unheld name refuses");
            assert!(
                names_the_name(&refused),
                "{doc_name}: {} formula {at}: {refused:?}",
                kind_of(node)
            );
            at += 1;
        }
        // The same formulas reached through `SetProgram`, where the node
        // is a standing profile.
        let Node::Profile(program) = node else {
            continue;
        };
        let Some(target) = doc
            .order()
            .iter()
            .copied()
            .find(|&id| doc.node(id).is_some_and(|held| held.authored() == *node))
        else {
            continue;
        };
        let loops: &[LoopProgram<Formula>] = &program.loops;
        let mut at = 0;
        loop {
            let (mutated, seen) = named_at(at, |f| {
                loops
                    .iter()
                    .map(|lp| lp.try_map_slots(&mut |e| f(e)))
                    .collect::<Result<Vec<_>, _>>()
            });
            if at >= seen {
                break;
            }
            programs += 1;
            let refused = edit(
                doc,
                &DocEdit::SetProgram {
                    node: target,
                    loops: mutated,
                    ids: program
                        .ids
                        .iter()
                        .map(|steps| steps.iter().copied().map(Some).collect())
                        .collect(),
                },
            )
            .expect_err("an unheld name refuses");
            assert!(
                matches!(refused, EditError::SlotUnknownVarName { .. }),
                "{doc_name}: program argument {at}: {refused:?}"
            );
            at += 1;
        }
    }
    // The same over an instance's offset, at `SetOffset`.
    let instance = apply(
        &edge_doc,
        &insert(Node::instantiate_part(reference(&edge_doc))),
        Tol::witness(),
        &RefusingReach,
    )
    .expect("an instance inserts");
    let target = instance.record.minted.expect("an insert mints");
    let offset = rigid();
    let mut offsets = 0usize;
    let mut at = 0;
    loop {
        let (mutated, seen) = named_at(at, |f| offset.try_map_slots(&mut |e| f(e)));
        if at >= seen {
            break;
        }
        offsets += 1;
        let refused = edit(
            &instance.doc,
            &DocEdit::SetOffset {
                instance: target,
                offset: Some(mutated),
            },
        )
        .expect_err("an unheld name refuses");
        assert!(
            matches!(refused, EditError::SlotUnknownVarName { .. }),
            "offset step argument {at}: {refused:?}"
        );
        at += 1;
    }
    println!(
        "{} nodes, {formulas} node formulas, {programs} program arguments, {offsets} offset \
         arguments; kinds {kinds:?}",
        cases.len()
    );
    assert!(formulas > 0 && programs > 0 && offsets == 7);
    // Every node kind is in the sweep: a kind added to the vocabulary
    // that no corpus document holds owes an edge above.
    assert_eq!(
        kinds.len(),
        23,
        "a node kind the sweep does not reach: {kinds:?}"
    );
}

/// A payload whose walk maps a formula its slot table does not list:
/// the shape a future payload could take by forgetting a row.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct Unlisted<S> {
    hidden: S,
}
impl editor_core::SlotPayload<editor_core::Expr> for Unlisted<editor_core::Expr> {}
impl editor_core::SlotPayload<Formula> for Unlisted<Formula> {}
impl editor_core::ProfilePayload for Unlisted<editor_core::Expr> {
    type Authored = Unlisted<Formula>;
    fn lower<E>(
        authored: &Unlisted<Formula>,
        f: &mut dyn FnMut(&Formula) -> Result<editor_core::Expr, E>,
    ) -> Result<Self, E> {
        Ok(Unlisted {
            hidden: f(&authored.hidden)?,
        })
    }
    fn authored(&self) -> Unlisted<Formula> {
        Unlisted {
            hidden: Formula::from(&self.hidden),
        }
    }
    fn drawn_pieces(
        &self,
        _env: &editor_core::VarEnv<f64>,
        _tol: Tol,
    ) -> Result<BTreeSet<editor_core::ProfileEdgeRef>, editor_core::ProgramRefusal> {
        Ok(BTreeSet::new())
    }
}

/// **A formula the walk reaches and no row addresses refuses typed**,
/// at the node's payload: the door's last word for an address it cannot
/// name, where the node's placement rule is not the reason.
#[test]
fn a_formula_no_row_addresses_refuses_at_the_payload() {
    let doc: editor_core::Doc<Unlisted<editor_core::Expr>> =
        editor_core::Doc::empty_derived("intent-literals-b-unlisted", Tol::witness());
    let refused = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Profile(Unlisted {
                    hidden: Formula::named(name("nope"), Dimension::Length),
                })),
            },
            Tol::witness(),
            &RefusingReach,
        )
        .expect_err("nope is unheld");
    assert!(
        matches!(refused, EditError::PayloadUnknownVarName { .. }),
        "{refused:?}"
    );
}
