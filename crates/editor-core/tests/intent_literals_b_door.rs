//! INTENT-LITERALS PR B: the edit door lowers an authored node, program
//! or constraint in one walk, and every formula that walk reaches refuses
//! typed where it does not lower — never a panic.
//!
//! The review rows pin the base's refusals: a definition of another kind
//! refuses its kind before its names, and a refused insert speaks one
//! id whether its held names were written by name or by id.
#![allow(clippy::expect_used, clippy::panic)]

use crate::blocks::block;
use crate::corpus;
use crate::fixture::resolver::PartStore;
use crate::fixture::{len, scl};
use editor_core::{
    AuthoredNode, CapEnd, Dimension, DocEdit, DocRef, DocumentId, EditError, Formula, FreeVar,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, TubeWindow, VarDecl,
    VarName, apply, content_pin,
};
use geom_core::Tol;
use std::collections::BTreeSet;

fn name(text: &'static str) -> VarName {
    VarName::from_static(text)
}

fn edit(doc: &ProfileDoc, e: &DocEdit<ProfileProgram>) -> Result<ProfileDoc, EditError> {
    apply(doc, e, Tol::witness()).map(|a| a.doc)
}

fn body() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("intent-literals-b-door", Tol::witness());
    block(doc, (0.0, 1.0), (0.0, 1.0), 1.0)
}

fn insert(node: AuthoredNode) -> DocEdit<ProfileProgram> {
    DocEdit::InsertNode {
        node: Box::new(node),
        fresh: Vec::new(),
    }
}

/// The plane of `body`'s end cap slid along its normal by `by`: a mate's
/// pose whose one number is a formula.
fn moved(body: RecipeNodeId, by: Formula) -> editor_core::Operand {
    let cap = crate::fixture::plane(crate::fixture::head(crate::fixture::fname(
        body,
        RoleSeg::Cap(CapEnd::End),
    )));
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(cap.clone()),
        along: editor_core::pose::RoleRead::through(cap, editor_core::pose::PoseRole::Normal),
        by,
    }))
}

/// A mate of the plane of `body`'s start cap with its end cap's plane
/// slid by `0.3` ([`moved`]).
fn on_the_world(body: RecipeNodeId) -> editor_core::Mate<Formula> {
    editor_core::Mate {
        on: crate::fixture::plane(crate::fixture::head(crate::fixture::fname(
            body,
            RoleSeg::Cap(CapEnd::Start),
        ))),
        to: moved(body, len(0.3)),
        orient: editor_core::Orientation::IDENTITY,
    }
}

/// A reference to `doc`, pinned at its content.
fn reference(doc: &ProfileDoc) -> DocRef {
    DocRef {
        id: doc.id(),
        pin: content_pin(doc, Tol::witness()).expect("the pin computes"),
    }
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
    )
    .expect("w declares");
    let w = applied.record.minted_var.expect("a declare mints");
    let doc = applied.doc;
    let target = crate::fixture::space(&doc, b);
    let node = |first: Formula| -> AuthoredNode {
        let by = Formula::add(first, Formula::named(name("nope"), Dimension::Length))
            .expect("two lengths add");
        let placed = editor_core::Placed::of(
            &doc,
            Vec::new(),
            vec![(editor_core::Coord::Slide(editor_core::TargetAxis::X), by)],
        );
        Node::place(
            editor_core::BodyRead::plain(editor_core::Operand::Node(b)),
            editor_core::Operand::Var(target),
            placed.bundle(),
        )
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

    /// Whether `refused` is this form's refusal where every formula has
    /// an address (a program's argument, a world read's number).
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
/// fresh entry — refuses typed.** Over every node of every corpus
/// document plus the slot types no corpus document holds: each node
/// re-authored, and for each formula `try_map_slots` visits — slots,
/// payload leaves, a profile's program, a placement's steps, a mate's
/// pose, a value's number — that one formula unheld, inserted. Then the
/// same over every profile's program at `SetProgram`. A slot type the
/// walk reaches and the refusal does not is a red row, not a panic in
/// a caller.
#[test]
fn every_formula_the_door_walks_refuses_typed_as_an_unheld_name() {
    // The block placed in its world, as a part stored for instancing
    // is: the placement rows copy its body.
    let mut store = PartStore::default();
    let (part, b) = store.insert_part(body(), Tol::witness());
    let edge_doc = store.doc(part.id);
    // The block's own profile, for the slots that read that kind.
    let held = |is: fn(&Node<ProfileProgram>) -> bool| {
        edge_doc
            .ids()
            .into_iter()
            .find(|&id| edge_doc.node(id).is_some_and(is))
            .expect("the block holds it")
    };
    let profile = held(|n| matches!(n, Node::Profile(_)));
    // The slot types no corpus document holds: a shell, a sweep, a windowed
    // tube, an instance's mate, a placement's mate and a value beside
    // it. Each operand reads a live output of the kind its slot
    // admits, so the refusal is the formula's.
    let edges: Vec<AuthoredNode> = vec![
        Node::shell(b, len(0.01), Vec::new()),
        Node::Sweep {
            profile: profile.into(),
            path: profile.into(),
            stations: Formula::count(4),
            v_degree: Formula::count(3),
        },
        Node::Tube {
            major_radius: len(0.5),
            minor_radius: len(0.1),
            window: TubeWindow::Arc {
                t0: scl(0.0),
                t1: scl(0.5),
            },
        },
        Node::InstantiatePart {
            doc_ref: reference(&edge_doc),
            interface: editor_core::InterfaceRecord::default(),
            target: editor_core::Operand::Var(editor_core::VarId::WORLD),
            constraints: vec![editor_core::Constraint {
                id: editor_core::ConstraintId(editor_core::MintId::new(0, 0)),
                kind: editor_core::ConstraintKind::Mate(on_the_world(b)),
            }],
        },
        Node::Place {
            source: b.into(),
            target: editor_core::Operand::Var(editor_core::VarId::WORLD),
            constraints: vec![
                editor_core::Constraint {
                    id: editor_core::ConstraintId(editor_core::MintId::new(0, 0)),
                    kind: editor_core::ConstraintKind::Mate(on_the_world(b)),
                },
                editor_core::Constraint {
                    id: editor_core::ConstraintId(editor_core::MintId::new(0, 1)),
                    kind: editor_core::ConstraintKind::Value(editor_core::Value {
                        coord: editor_core::Coord::Slide(editor_core::TargetAxis::X),
                        by: len(0.2),
                    }),
                },
            ],
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
                        |p, g, r| p.try_map_slots(&mut |e| g(e), r),
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
                    form.refuses_at_a_slot(&refused),
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
    // The same over a number a mate's pose reads, at `AddConstraint`.
    let instance = apply(
        &edge_doc,
        &insert(crate::fixture::mated_instance(reference(&edge_doc))),
        Tol::witness(),
    )
    .expect("an instance inserts");
    let target = instance.record.minted.expect("an insert mints");
    let mut offsets = 0usize;
    for form in Unheld::ALL {
        offsets += 1;
        let refused = edit(
            &instance.doc,
            &DocEdit::AddConstraint {
                placement: target,
                mate: Box::new(editor_core::Mate {
                    to: moved(b, form.formula(editor_core::Dimension::Length)),
                    ..on_the_world(b)
                }),
                fresh: Vec::new(),
            },
        )
        .expect_err("an unheld read refuses");
        assert!(
            form.refuses_at_a_slot(&refused),
            "a mate pose's number as {form:?}: {refused:?}"
        );
    }
    println!(
        "{} nodes, {formulas} node formulas, {programs} program arguments, {offsets} mate pose \
         numbers; kinds {kinds:?}",
        cases.len()
    );
    assert!(formulas > 0 && programs > 0 && offsets == 3);
    // Every node kind is in the sweep but the world, which no edit
    // inserts and no document writes: a kind added to the vocabulary
    // that no corpus document holds owes an edge above. D′3's `⊔`
    // joined it: every assembly's product reads one.
    // A pattern is a `Place` over an index, its member an indexed read.
    assert_eq!(
        kinds.len(),
        20,
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
        _: &mut dyn FnMut(
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
        _: &mut dyn FnMut(editor_core::VarId) -> editor_core::Operand,
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
        )
        .expect_err("nope is unheld");
    assert!(
        matches!(refused, EditError::PayloadUnknownVarName { .. }),
        "{refused:?}"
    );
}
