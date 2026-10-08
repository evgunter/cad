//! **Operands are reads** (D10, INTENT stage 2 unit B): every operand
//! field holds a variable read of an output, typed by the kind its slot
//! admits; one slot door writes every operand; a delete leaves its
//! readers unresolved and says so; and the load door, the schedule and
//! the cascade read one dependency relation (`Doc::upstream`), so a
//! member re-pointed forward saves, loads and deletes.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, on_frame_keeping, scl, xform};
use crate::wire::up_to_ids;
use editor_core::{
    DocEdit, EditError, Maintenance, Node, NodeErrorKind, Operand, OperandKind, OperandSlot,
    PatternKind, ProfileDoc, RecipeNodeId, Took, VarKind, apply, load, save,
};
use geom_core::Tol;

/// A frame, a square on it and an extrude of the square, `dx` along x.
fn block(doc: ProfileDoc, dx: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, _, profile) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5 + dx, 0.5, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    (doc, profile, extrude)
}

fn applied(
    doc: &ProfileDoc,
    edit: DocEdit<editor_core::ProfileProgram>,
) -> editor_core::Applied<editor_core::ProfileProgram> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap_or_else(|refusal| panic!("the edit applies: {refusal}"))
}

fn refused(doc: &ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>) -> EditError {
    match apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach) {
        Ok(_) => panic!("the door accepted an edit this row expects it to refuse"),
        Err(refusal) => refusal,
    }
}

/// **(B, test 3) Kind at the door.** A boolean whose operand reads a
/// profile's output refuses by kind, naming the slot and both kinds,
/// and leaves the document as it was — where an untyped operand would
/// insert and refuse `WrongOperand` at evaluation.
#[test]
fn a_read_of_the_wrong_kind_refuses_at_the_door() {
    let (doc, profile, extrude) = block(ProfileDoc::empty_derived("s2b-kind", Tol::witness()), 0.0);
    let read = doc
        .output(profile, 0)
        .expect("a profile defines its profile");
    let before = doc.clone();
    let refusal = refused(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Boolean {
                op: editor_core::BooleanOp::Union,
                a: Operand::Var(read),
                b: extrude.into(),
                declare: Vec::new(),
            }),
            fresh: Vec::new(),
        },
    );
    assert!(
        matches!(
            &refusal,
            EditError::OperandVarKind {
                slot: OperandSlot::A,
                found: VarKind::Profile,
                expected: OperandKind::Is(VarKind::Body),
                var,
                ..
            } if var.id() == read
        ),
        "{refusal:?}"
    );
    assert!(
        doc.bit_eq(&before),
        "a refused edit leaves the document as it was"
    );
}

/// **(B, test 5) A delete leaves a typed reader.** Deleting the extrude
/// under a fillet is accepted and reports the fillet's `target` read
/// stranded, beside the strands of the names the extrude minted; the
/// fillet then refuses `UnresolvedRead` at its target. The stranded
/// document saves and loads as itself, and undo — the document the
/// edit started from — restores the fillet bit for bit.
#[test]
fn a_delete_leaves_its_reader_unresolved_and_typed() {
    let (doc, _, extrude) = block(ProfileDoc::empty_derived("s2b-strand", Tol::witness()), 0.0);
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let edges: Vec<_> = editor_core::all_edges(&ev, extrude)
        .into_iter()
        .take(1)
        .collect();
    let (doc, fillet) = insert(doc, Node::fillet(extrude, len(0.1), edges.clone()));
    let target = doc.output(extrude, 0).expect("an extrude defines its body");

    let before = doc.clone();
    let deleted = applied(&doc, DocEdit::DeleteNode { id: extrude });
    let reads: Vec<_> = deleted
        .maintenance
        .iter()
        .filter_map(|row| match row {
            Maintenance::StrandedRead { node, slot, var } => Some((node.id(), *slot, var.id())),
            _ => None,
        })
        .collect();
    assert_eq!(
        reads,
        vec![(fillet, OperandSlot::Target, target)],
        "one stranded read"
    );
    let names: Vec<_> = deleted
        .maintenance
        .iter()
        .filter_map(|row| match row {
            Maintenance::Strand {
                node,
                name,
                took: Took::Node,
            } => Some((node.id(), name.name().clone())),
            _ => None,
        })
        .collect();
    assert_eq!(
        names,
        edges
            .iter()
            .map(|e| (fillet, e.clone()))
            .collect::<Vec<_>>(),
        "and a strand per selected name the extrude minted"
    );
    assert_eq!(
        deleted.doc.node(fillet),
        doc.node(fillet),
        "the reader is not re-pointed"
    );

    let ev = fixture::run(&deleted.doc, &editor_core::EvalOptions::default());
    assert!(
        matches!(
            ev.node_error(fillet).map(|e| &e.kind),
            Some(NodeErrorKind::UnresolvedRead { slot: OperandSlot::Target, var }) if *var == target
        ),
        "{:?}",
        ev.node_error(fillet)
    );

    let text = save(&deleted.doc, &[], Tol::witness()).expect("a stranded read saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&deleted.doc), "as itself");

    let replayed = applied(&deleted.doc, DocEdit::DeleteNode { id: fillet }).doc;
    assert!(
        replayed.node(fillet).is_none(),
        "the stranded reader deletes like any node"
    );

    // Undo is the document the edit started from: the door is pure, so
    // it is untouched, and the fillet reads its target again.
    assert!(
        doc.bit_eq(&before),
        "undo restores the document bit for bit"
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    assert!(
        ev.node_error(fillet).is_none(),
        "{:?}",
        ev.node_error(fillet)
    );
}

/// **A member re-pointed forward saves, loads and cascades**
/// (`work/recipe/set-members-admits-a-forward-member-the-save-validator-refuses`):
/// a union over `[a, b]` re-pointed at `[a, b, c]`, `c` inserted after
/// it, is a document the edit door wrote, so the save door takes it
/// and the load door reads it back; deleting `c` with its readers
/// takes the union too, though the union precedes it in the document.
#[test]
fn a_forward_member_saves_loads_and_cascades() {
    let doc = ProfileDoc::empty_derived("s2b-forward", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![a.into(), b.into()],
            declare: Vec::new(),
        },
    );
    let (doc, _, c) = block(doc, 6.0);
    let doc = applied(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a.into(), b.into(), c.into()],
        },
    )
    .doc;
    assert_eq!(
        doc.upstream(union),
        vec![a, b, c],
        "the union reads its forward member"
    );

    let text = save(&doc, &[], Tol::witness()).expect("a forward member saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&doc), "as itself");

    let doomed = editor_core::cascade_delete_order(&doc, c);
    assert!(
        doomed.contains(&union) && doomed.contains(&c),
        "the cascade closes over readers, not over document order: {doomed:?}"
    );
    let at = |id| doomed.iter().position(|d| *d == id).expect("doomed");
    assert!(at(union) < at(c), "a reader goes before what it reads");
}

/// **The one slot door writes an operand** (DM6 as #4221 rewrote it):
/// a re-point is checked for liveness, kind, acyclicity and DM5, and a
/// name the re-point takes out of reach is reported, never refused.
#[test]
fn the_slot_door_re_points_an_operand_and_reports_what_it_strands() {
    let doc = ProfileDoc::empty_derived("s2b-slot-door", Tol::witness());
    let (doc, pa, a) = block(doc, 0.0);
    let (doc, pb, b) = block(doc, 3.0);
    let set = |node, slot, read: Operand| DocEdit::SetOperand { node, slot, read };

    // A re-point at the same kind is an ordinary write.
    let moved = applied(&doc, set(a, OperandSlot::Profile, pb.into()));
    assert_eq!(
        moved.doc.upstream(a),
        vec![pb],
        "the extrude reads the other profile"
    );
    assert!(
        fixture::without_anonymous(&moved.maintenance).is_empty(),
        "and strands nothing: {:?}",
        moved.maintenance
    );
    assert!(
        matches!(
            refused(&doc, set(a, OperandSlot::Profile, b.into())),
            EditError::OperandVarKind {
                slot: OperandSlot::Profile,
                found: VarKind::Body,
                ..
            }
        ),
        "a body at a profile seat refuses by kind"
    );
    assert!(
        matches!(
            refused(&doc, set(a, OperandSlot::Target, pa.into())),
            EditError::UnknownOperand {
                slot: OperandSlot::Target,
                ..
            }
        ),
        "an extrude has no target"
    );

    // Acyclicity: a transform of `a`, and `a`'s profile re-pointed at
    // nothing that reads it is fine; a union re-pointed at its own
    // reader is a cycle.
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![a.into(), b.into()],
            declare: Vec::new(),
        },
    );
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    assert!(
        matches!(
            refused(&doc, set(union, OperandSlot::Member(1), moved.into())),
            EditError::WouldCycle { .. }
        ),
        "a union re-pointed at its own reader would cycle"
    );
    assert!(
        matches!(
            refused(&doc, set(union, OperandSlot::Member(1), a.into())),
            EditError::DuplicateInput { .. }
        ),
        "DM5: a member read twice"
    );

    // A placer's output kind is fixed at minting: a transform of a body
    // re-pointed at a pattern's list refuses, at a body is a write.
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: b.into(),
            count: editor_core::Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    assert!(
        matches!(
            refused(&doc, set(moved, OperandSlot::Input, pattern.into())),
            EditError::OperandVarKind {
                found: VarKind::Bodies,
                ..
            }
        ),
        "a body's placement does not become a list's"
    );
    applied(&doc, set(moved, OperandSlot::Input, b.into()));
}

/// **The comparator reads a read as the input it names**, so a document
/// written before operands were reads and the same document after them
/// are one up to ids: and a read re-pointed at another node's output,
/// the shape a wrong lowering would write, is a mismatch it names.
#[test]
fn the_comparator_reads_reads_as_inputs_and_catches_a_re_pointed_one() {
    let doc = ProfileDoc::empty_derived("s2b-comparator", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let (doc, _) = insert(
        doc,
        Node::Union {
            members: vec![a.into(), b.into()],
            declare: Vec::new(),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let new = crate::wire::wire_body(&text);
    let as_inputs = up_to_ids::reads_as_inputs(&new);
    up_to_ids::same_up_to_ids(&as_inputs, &as_inputs).expect("a document is itself");

    // The mutant: the union's second member reads `a`'s output in place
    // of `b`'s, every other byte kept.
    let (out_a, out_b) = (
        doc.output(a, 0).expect("a body").0.to_string(),
        doc.output(b, 0).expect("a body").0.to_string(),
    );
    let mut mutant = new.clone();
    for node in mutant["snapshot"]["nodes"]
        .as_object_mut()
        .expect("nodes")
        .values_mut()
    {
        if let Some(members) = node.get_mut("Union").and_then(|u| u.get_mut("members")) {
            for member in members.as_array_mut().expect("a list") {
                if member.as_str() == Some(out_b.as_str()) {
                    *member = serde_json::json!(out_a);
                }
            }
        }
    }
    let err = up_to_ids::same_up_to_ids(&as_inputs, &up_to_ids::reads_as_inputs(&mutant))
        .expect_err("a re-pointed read is not the same document");
    assert!(err.contains("Union"), "the mismatch is at the union: {err}");
}

/// **(B, test 4, one shot) Every re-blessed document is the pre-B one
/// up to ids, with reads in place of inputs** — its roots element for
/// element and every other byte, except the tube subgraph, whose
/// anchor moved from a spine axis to a frame by design (FORK-1b) and
/// is set aside on both sides.
///
/// Run against a checkout of the base: `PRE_B_TREE=<that checkout>`
/// and `--run-ignored only`. Ignored because the base is not in this
/// tree; the PR states the run.
#[test]
#[ignore = "one shot against a checkout of the base, named by PRE_B_TREE"]
fn every_re_blessed_document_is_the_pre_b_one_up_to_ids() {
    let base = std::path::PathBuf::from(
        std::env::var("PRE_B_TREE").expect("PRE_B_TREE names a checkout of the base"),
    );
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for file in [
        "crates/editor-core/tests/golden/golden.cad",
        "crates/editor-core/tests/corpus/die_tool.pncad",
        "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
        "crates/viewer/tests/gallery_ring.pncad",
        "crates/pncad/tests/plate_param.pncad",
    ] {
        let read = |root: &std::path::Path| {
            crate::wire::wire_body(&std::fs::read_to_string(root.join(file)).expect("reads"))
        };
        let (old, new) = (read(&base), read(&here));
        // An edit log's reads name outputs its replay mints: the
        // replayed document's table says whose they are.
        let text = std::fs::read_to_string(here.join(file)).expect("reads");
        let replayed = load(&text, Tol::witness())
            .expect("the re-blessed file loads")
            .doc;
        let table = crate::wire::wire_body(&save(&replayed, &[], Tol::witness()).expect("saves"));
        let new = up_to_ids::reads_as_inputs_by(&new, &table);
        let (old, new) = (
            up_to_ids::without_nodes(&old, &up_to_ids::tube_subgraph(&old)),
            up_to_ids::without_nodes(&new, &up_to_ids::tube_subgraph(&new)),
        );
        up_to_ids::same_up_to_ids(&old, &new).unwrap_or_else(|err| panic!("{file}: {err}"));
        println!("{file}: equal up to ids, reads as inputs");
    }
}
