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
use crate::fixture::resolver::{PartStore, in_part};
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
        fresh: Vec::new(),
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
            input: b.into(),
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
            input: b.into(),
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
            fresh: Vec::new(),
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
            input: b.into(),
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

/// **The slot forms a door lowers that can name nothing the document
/// holds** (INTENT-LITERALS spec §4): a name, a variable's id, and an
/// entry of the edit's fresh table, each unheld.
#[derive(Clone, Copy, Debug)]
enum Unheld {
    /// A name no variable holds.
    Name,
    /// An id the document never minted.
    Id,
    /// An entry the edit's (empty) fresh table does not hold.
    Fresh,
}

impl Unheld {
    const ALL: [Self; 3] = [Self::Name, Self::Id, Self::Fresh];

    /// This form, read at `dim`.
    fn formula(self, dim: Dimension) -> Formula {
        match self {
            Self::Name => Formula::named(name("nope"), dim),
            Self::Id => Formula::var(
                editor_core::VarId::new(0, test_utils::refusal::tagged(9)),
                dim,
            ),
            Self::Fresh => Formula::fresh(0, dim),
        }
    }

    /// Whether `refused` is this form's refusal at a slot or the
    /// payload, or — for a node's count no slot addresses — the
    /// placement rule's own (p1's edge).
    fn refuses(self, refused: &EditError) -> bool {
        matches!(refused, EditError::PlacementRuleMismatch { .. })
            || self.refuses_at_a_slot(refused)
    }

    /// Whether `refused` is this form's refusal where every formula has
    /// an address (a program's argument, an offset's).
    fn refuses_at_a_slot(self, refused: &EditError) -> bool {
        match self {
            Self::Name => matches!(
                refused,
                EditError::SlotUnknownVarName { .. } | EditError::PayloadUnknownVarName { .. }
            ),
            Self::Id => matches!(
                refused,
                EditError::SlotUnresolvedVar { .. } | EditError::PayloadUnresolvedVar { .. }
            ),
            Self::Fresh => matches!(refused, EditError::FreshUnheld { index: 0, .. }),
        }
    }
}

/// The formula a walk visits `at`th replaced by `form` read at its
/// dimension, every other kept: one walk, the door's own.
fn unheld_at<T>(
    form: Unheld,
    at: usize,
    map: impl FnOnce(&mut dyn FnMut(&Formula) -> Result<Formula, ()>) -> Result<T, ()>,
) -> (T, usize) {
    let mut seen = 0;
    let mapped = map(&mut |formula| {
        let here = seen;
        seen += 1;
        Ok(if here == at {
            form.formula(formula.dim())
        } else {
            formula.clone()
        })
    })
    .expect("the walk refuses nothing");
    (mapped, seen)
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

/// **Every formula field the door's walk reaches, written in each slot
/// form as something the document does not hold — a name, an id, a
/// fresh entry — refuses typed.** Over every node of every corpus document plus
/// p1/p2's listed-rule nodes: each node re-authored, and for each
/// formula `try_map_slots` visits — slots, payload leaves, a profile's
/// program, a placement's steps, a mate's frame offsets, the count no
/// slot addresses — that one formula unheld, inserted. Then the
/// same over every profile's program at `SetProgram`. A slot type the
/// walk reaches and the refusal does not is a red row, not a panic in
/// a caller.
#[test]
fn every_formula_the_door_walks_refuses_typed_as_an_unheld_name() {
    // The block placed in its world, as a part stored for instancing
    // is: the mate rows name its caps through that placement.
    let mut store = PartStore::default();
    let (part, b) = store.insert_part(body(), Tol::witness());
    let edge_doc = store.doc(part.id);
    // The block's own sketch frame and profile, for the slots that read
    // those kinds.
    let held = |is: fn(&Node<ProfileProgram>) -> bool| {
        edge_doc
            .ids()
            .into_iter()
            .find(|&id| edge_doc.node(id).is_some_and(is))
            .expect("the block holds it")
    };
    let frame = held(|n| matches!(n, Node::Datum(editor_core::Datum::Frame { .. })));
    let profile = held(|n| matches!(n, Node::Profile(_)));
    // The listed-rule counts no slot addresses (p1/p2), and the slot
    // types no corpus document holds: a shell, a sweep, a windowed
    // tube, a gauge's and an instance's placements, a mate's frame
    // offsets. Each operand reads a live output of the kind its slot
    // admits, so the refusal is the formula's.
    let edges: Vec<AuthoredNode> = vec![
        Node::Pattern {
            input: b.into(),
            count: Formula::count(2),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        },
        Node::PlacedUnion {
            input: b.into(),
            count: Some(Formula::count(2)),
            kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
        },
        Node::shell(b, len(0.01), Vec::new()),
        Node::Sweep {
            profile: profile.into(),
            path: profile.into(),
            stations: Formula::count(4),
            v_degree: Formula::count(3),
        },
        Node::Tube {
            frame: frame.into(),
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
            a: head(in_part(b, b, CapEnd::End)).into(),
            b: head(in_part(b, b, CapEnd::Start)).into(),
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
        for id in d.doc.ids() {
            let node = d
                .doc
                .node(id)
                .expect("an ordered node is held")
                .authored(&d.doc);
            cases.push((d.name.to_owned(), d.doc.clone(), node));
        }
    }

    let mut kinds = BTreeSet::new();
    let mut formulas = 0usize;
    let mut programs = 0usize;
    for (doc_name, doc, node) in &cases {
        kinds.insert(kind_of(node));
        for form in Unheld::ALL {
            let mut at = 0;
            loop {
                let (mutated, seen) = unheld_at(form, at, |f| {
                    node.try_map_slots(
                        |p, g, r| p.try_map_slots(&mut |e| g(e), &mut |at, read| r(at, read)),
                        &mut |e| f(e),
                        &mut |_, read| Ok(read.clone()),
                    )
                });
                if at >= seen {
                    break;
                }
                formulas += 1;
                let refused = edit(doc, &insert(mutated)).expect_err("an unheld read refuses");
                assert!(
                    form.refuses(&refused),
                    "{doc_name}: {} formula {at} as {form:?}: {refused:?}",
                    kind_of(node)
                );
                at += 1;
            }
        }
        // The same formulas reached through `SetProgram`, where the node
        // is a standing profile.
        let Node::Profile(program) = node else {
            continue;
        };
        let Some(target) = doc
            .ids()
            .iter()
            .copied()
            .find(|&id| doc.node(id).is_some_and(|held| held.authored(doc) == *node))
        else {
            continue;
        };
        let loops: &[LoopProgram<Formula>] = &program.loops;
        for form in Unheld::ALL {
            let mut at = 0;
            loop {
                let (mutated, seen) = unheld_at(form, at, |f| {
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
                        fresh: Vec::new(),
                    },
                )
                .expect_err("an unheld read refuses");
                assert!(
                    form.refuses_at_a_slot(&refused),
                    "{doc_name}: program argument {at} as {form:?}: {refused:?}"
                );
                at += 1;
            }
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
    for form in Unheld::ALL {
        let mut at = 0;
        loop {
            let (mutated, seen) = unheld_at(form, at, |f| offset.try_map_slots(&mut |e| f(e)));
            if at >= seen {
                break;
            }
            offsets += 1;
            let refused = edit(
                &instance.doc,
                &DocEdit::SetOffset {
                    instance: target,
                    offset: Some(mutated),
                    fresh: Vec::new(),
                },
            )
            .expect_err("an unheld read refuses");
            assert!(
                form.refuses_at_a_slot(&refused),
                "offset step argument {at} as {form:?}: {refused:?}"
            );
            at += 1;
        }
    }
    println!(
        "{} nodes, {formulas} node formulas, {programs} program arguments, {offsets} offset \
         arguments; kinds {kinds:?}",
        cases.len()
    );
    assert!(formulas > 0 && programs > 0 && offsets == 3 * 7);
    // Every node kind is in the sweep: a kind added to the vocabulary
    // that no corpus document holds owes an edge above.
    assert_eq!(
        kinds.len(),
        24,
        "a node kind the sweep does not reach: {kinds:?}"
    );
}

/// A payload whose walk maps a formula its slot table does not list:
/// the shape a future payload could take by forgetting a row.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
struct Unlisted<S> {
    hidden: S,
}
impl editor_core::SlotPayload<editor_core::VarId> for Unlisted<editor_core::VarId> {}
impl editor_core::SlotPayload<Formula> for Unlisted<Formula> {}
impl editor_core::ProfilePayload for Unlisted<editor_core::VarId> {
    type Authored = Unlisted<Formula>;
    fn lower<E>(
        authored: &Unlisted<Formula>,
        f: &mut dyn FnMut(&Formula) -> Result<editor_core::VarId, E>,
        _read: &mut dyn FnMut(
            editor_core::OperandSlot,
            &editor_core::Operand,
        ) -> Result<editor_core::VarId, E>,
    ) -> Result<Self, E> {
        Ok(Unlisted {
            hidden: f(&authored.hidden)?,
        })
    }
    fn authored_with(
        &self,
        reader: &mut dyn FnMut(editor_core::VarId, Dimension) -> Formula,
    ) -> Unlisted<Formula> {
        Unlisted {
            hidden: reader(self.hidden, Dimension::Length),
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
    let doc: editor_core::Doc<Unlisted<editor_core::VarId>> =
        editor_core::Doc::empty_derived("intent-literals-b-unlisted", Tol::witness());
    let refused = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(Node::Profile(Unlisted {
                    hidden: Formula::named(name("nope"), Dimension::Length),
                })),
                fresh: Vec::new(),
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
