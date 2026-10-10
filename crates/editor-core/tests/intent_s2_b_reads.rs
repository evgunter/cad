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
    Dimension, DocEdit, EditError, Maintenance, Node, NodeErrorKind, Operand, OperandSlot,
    PatternKind, ProfileDoc, RecipeNodeId, SlotId, SlotKind, Took, VarKind, apply, load, save,
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
            EditError::SlotVarKind {
                slot: SlotId::Operand(OperandSlot::A),
                found: VarKind::Profile,
                expected: SlotKind::Is(VarKind::Body),
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
/// document saves and loads as itself, and the stranded reader deletes.
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

    // Undo is the viewer history's: its row is `undo_tree.rs`'s
    // `undo_restores_a_reader_a_delete_stranded`, which deletes, undoes
    // and evaluates the restored reader.
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
    let set = |node, slot, read: Operand| DocEdit::SetParam {
        node,
        slot: SlotId::Operand(slot),
        value: read.into(),
        fresh: Vec::new(),
    };

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
            EditError::SlotVarKind {
                slot: SlotId::Operand(OperandSlot::Profile),
                found: VarKind::Body,
                ..
            }
        ),
        "a body at a profile seat refuses by kind"
    );
    assert!(
        matches!(
            refused(&doc, set(a, OperandSlot::Target, pa.into())),
            EditError::UnknownSlot {
                slot: SlotId::Operand(OperandSlot::Target),
                ..
            }
        ),
        "an extrude has no target"
    );

    // A name the re-point takes out of reach: a fillet of `a` selecting
    // one of `a`'s edges, re-pointed at `b`, reports the edge stranded
    // by reach and is written.
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let edge = editor_core::all_edges(&ev, a)
        .into_iter()
        .next()
        .expect("a block has edges");
    let (filleted, fillet) = insert(doc.clone(), Node::fillet(a, len(0.1), vec![edge.clone()]));
    let re_pointed = applied(&filleted, set(fillet, OperandSlot::Target, b.into()));
    assert_eq!(
        re_pointed
            .maintenance
            .iter()
            .filter_map(|row| match row {
                Maintenance::Strand {
                    node,
                    name,
                    took: Took::Reach,
                } => Some((node.id(), name.name().clone())),
                _ => None,
            })
            .collect::<Vec<_>>(),
        vec![(fillet, edge)],
        "the edge `a` minted is out of the fillet's reach, reported"
    );
    assert_eq!(
        re_pointed.doc.upstream(fillet),
        vec![b],
        "and the read moved"
    );

    // Acyclicity: a union re-pointed at its own reader is a cycle.
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
            EditError::SlotVarKind {
                found: VarKind::Bodies,
                ..
            }
        ),
        "a body's placement does not become a list's"
    );
    applied(&doc, set(moved, OperandSlot::Input, b.into()));
}

/// **One door, the slot typed by kind** (Q1 ruled): the door that
/// writes an expression writes an operand, and each slot says by its
/// kind what it takes. A formula at an operand refuses as an expression
/// of the wrong kind; a read at a scalar slot is the formula of the one
/// variable it reads, held to every rule a formula is; a read mints
/// nothing, so a fresh table beside it is unread.
#[test]
fn the_slot_door_takes_a_formula_or_a_read_by_the_slots_kind() {
    let doc = ProfileDoc::empty_derived("s2b-one-door", Tol::witness());
    let (doc, profile, extrude) = block(doc, 0.0);
    assert_eq!(
        SlotId::Operand(OperandSlot::Profile).kind(),
        SlotKind::Is(VarKind::Profile)
    );
    assert_eq!(SlotId::Operand(OperandSlot::Profile).dimension(), None);
    assert_eq!(SlotId::Distance.kind(), SlotKind::Is(VarKind::Length));
    assert_eq!(
        SlotId::Operand(OperandSlot::Input).kind(),
        SlotKind::Placeable
    );

    let formula_at_operand = DocEdit::SetParam {
        node: extrude,
        slot: SlotId::Operand(OperandSlot::Profile),
        value: len(1.0).into(),
        fresh: Vec::new(),
    };
    assert_eq!(
        refused(&doc, formula_at_operand.clone()),
        EditError::SlotDimensionMismatch {
            slot: SlotId::Operand(OperandSlot::Profile),
            expected: SlotKind::Is(VarKind::Profile),
            found: Dimension::Length,
        },
    );
    assert!(
        refused(&doc, formula_at_operand)
            .to_string()
            .contains("reads a profile, not a length expression"),
    );

    // A named length read at the distance is that variable, by id: the
    // same slot a formula naming it writes.
    let declared = applied(
        &doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("depth").expect("a name"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(
                Dimension::Length,
                2.0,
            )),
        },
    )
    .doc;
    let depth = declared.var_named("depth").expect("declared");
    let by_read = applied(
        &declared,
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            value: Operand::Var(depth).into(),
            fresh: Vec::new(),
        },
    )
    .doc;
    let by_formula = applied(
        &declared,
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            value: editor_core::Formula::var(depth, Dimension::Length).into(),
            fresh: Vec::new(),
        },
    )
    .doc;
    assert_eq!(by_read.slot(extrude, SlotId::Distance), Some(depth));
    assert_eq!(
        save(&by_read, &[], Tol::witness()).expect("saves"),
        save(&by_formula, &[], Tol::witness()).expect("saves"),
        "a read at a scalar slot is the formula of its one variable"
    );

    // A body read at the distance refuses by kind, at the slot.
    match refused(
        &doc,
        DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            value: Operand::Node(extrude).into(),
            fresh: Vec::new(),
        },
    ) {
        EditError::SlotVarKind {
            slot: SlotId::Distance,
            found: VarKind::Body,
            expected: SlotKind::Is(VarKind::Length),
            ..
        } => {}
        other => panic!("a body at a length slot refuses by kind, got {other:?}"),
    }

    // A read mints nothing for a fresh entry to name.
    assert!(matches!(
        refused(
            &doc,
            DocEdit::SetParam {
                node: extrude,
                slot: SlotId::Operand(OperandSlot::Profile),
                value: Operand::Node(profile).into(),
                fresh: vec![editor_core::FreeVar::continuous(Dimension::Length, 1.0).into()],
            },
        ),
        EditError::FreshUnread { index: 0 }
    ));
}

/// **A read of a split's port is that half** (FORK-1): a boolean over
/// the split's first port builds the union of the upper half and a
/// block clear of it, its volume the half's and the block's, and every
/// name it carries from the split is one of the upper half's rows.
#[test]
fn a_split_port_read_is_its_half() {
    let (doc, split, far) = split_block("s2b-split-port");
    let (doc, by_port) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: Operand::output(split, editor_core::SplitHalf::Above.port()),
            b: far.into(),
            declare: Vec::new(),
        },
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let volume = volume_of(&ev, by_port);
    assert!((volume - 1.5).abs() < 1e-12, "the half (0.5) and the block (1): {volume}");
    let above_rows: Vec<editor_core::StableName> = ev
        .value(split)
        .expect("the split")
        .name_table
        .iter()
        .filter(|(_, e)| {
            matches!(e, editor_core::Entry::Unique(r) if r.body == editor_core::SplitHalf::Above.output_body())
        })
        .map(|(n, _)| n.clone())
        .collect();
    let carried: Vec<editor_core::StableName> = ev
        .value(by_port)
        .expect("the union")
        .name_table
        .iter()
        .filter_map(|(n, _)| match n.path.first() {
            Some(editor_core::RoleSeg::FromA(inner)) => Some(editor_core::StableName::clone(inner)),
            _ => None,
        })
        .filter(|n| n.node == split)
        .collect();
    assert!(!carried.is_empty(), "the union carries the half's names");
    assert!(
        carried.iter().all(|n| above_rows.contains(n)),
        "every name from the split is an upper-half row: {carried:?}"
    );
}

/// The volume of a node's one body.
fn volume_of(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(crate::corpus::body_of(ev, id), Tol::witness())
        .expect("mass properties")
        .volume
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

/// **The comparator keeps a read's port** (review B's m1): a boolean
/// reading a split's upper half and the same boolean reading its lower
/// half are two documents to it, as they are two bodies.
#[test]
fn the_comparator_catches_a_read_re_pointed_from_one_port_to_another() {
    let (doc, split, far) = split_block("s2b-comparator-port");
    let (doc, _) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: Operand::output(split, 0),
            b: far.into(),
            declare: Vec::new(),
        },
    );
    let new = crate::wire::wire_body(&save(&doc, &[], Tol::witness()).expect("saves"));
    let as_inputs = up_to_ids::reads_as_inputs(&new);
    up_to_ids::same_up_to_ids(&as_inputs, &as_inputs).expect("a document is itself");
    let (above, below) = (
        doc.output(split, 0).expect("the upper half").0.to_string(),
        doc.output(split, 1).expect("the lower half").0.to_string(),
    );
    let mut mutant = new.clone();
    for node in mutant["snapshot"]["nodes"]
        .as_object_mut()
        .expect("nodes")
        .values_mut()
    {
        if let Some(a) = node.get_mut("Boolean").and_then(|b| b.get_mut("a"))
            && a.as_str() == Some(above.as_str())
        {
            *a = serde_json::json!(below);
        }
    }
    assert_ne!(mutant, new, "the mutant re-points the read");
    let err = up_to_ids::same_up_to_ids(&as_inputs, &up_to_ids::reads_as_inputs(&mutant))
        .expect_err("a read re-pointed above to below is not the same document");
    assert!(
        err.contains("Boolean"),
        "the mismatch is at the boolean: {err}"
    );
}

/// **(B, test 4, one shot) Every re-blessed document is the pre-B one
/// up to ids, with reads in place of inputs** (and a logged `SetParam`'s
/// formula under `value`, Q1, and a profile's and an in-plane axis's
/// `plane` field under its one name, `frame`) — its roots element for element and every
/// other byte, except the tube subgraph, whose
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
        let (old, new) = (
            up_to_ids::plane_field_as_frame(&up_to_ids::set_param_writes_value(&read(&base))),
            read(&here),
        );
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
    // Every operand family, a list member among them: the base's save of
    // the families document, beside this build's.
    let (old, new) = families_pair();
    let new = up_to_ids::reads_as_inputs(&new);
    let old = up_to_ids::plane_field_as_frame(&up_to_ids::split_half_parts_as_reads(&old));
    up_to_ids::same_up_to_ids(&old, &new)
        .unwrap_or_else(|err| panic!("the families document: {err}"));
    println!("the families document: equal up to ids, reads as inputs");
}

/// **The slot door asks every check the insert door does** (DM6; review
/// B's M1): an assertion over a length measure, re-pointed at an angle
/// measure, refuses `AssertionDimension` at the slot door as the insert
/// door refuses the same node — and so does saving the log that would
/// carry it, which replays through that door. The two doors call one
/// function of a written node, so neither admits what the other refuses.
#[test]
fn the_slot_door_refuses_an_assertion_re_pointed_across_dimensions() {
    let mut r = fixture::Recorder::new();
    let m_len = r.insert(Node::Measure {
        expr: editor_core::MeasureExpr::value(len(1.0)),
        refs: Vec::new(),
    });
    let m_ang = r.insert(Node::Measure {
        expr: editor_core::MeasureExpr::value(fixture::ang(0.5)),
        refs: Vec::new(),
    });
    let assertion = r.insert(Node::Assertion {
        measure: m_len.into(),
        bound: len(0.5),
        dir: editor_core::AssertionDir::AtLeast,
    });
    let by_insert = refused(
        &r.doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                measure: m_ang.into(),
                bound: len(0.5),
                dir: editor_core::AssertionDir::AtLeast,
            }),
            fresh: Vec::new(),
        },
    );
    assert!(
        matches!(
            by_insert,
            EditError::AssertionDimension {
                measured: Dimension::Angle,
                bound: Dimension::Length,
                ..
            }
        ),
        "{by_insert:?}"
    );
    let edit = DocEdit::SetParam {
        node: assertion,
        slot: SlotId::Operand(OperandSlot::Measure),
        value: Operand::Node(m_ang).into(),
        fresh: Vec::new(),
    };
    let by_slot = refused(&r.doc, edit.clone());
    assert!(
        matches!(
            &by_slot,
            EditError::AssertionDimension {
                node,
                measured: Dimension::Angle,
                bound: Dimension::Length,
                ..
            } if node.id() == assertion
        ),
        "the slot door refuses what the insert door refuses: {by_slot:?}"
    );
    let mut log = r.edits.clone();
    log.push(edit);
    assert!(
        save(
            &ProfileDoc::empty_derived("mod", Tol::witness()),
            &log,
            Tol::witness()
        )
        .is_err(),
        "a log carrying the re-point does not save"
    );
}

/// A block split across its middle, and a second block clear of it: the
/// two halves are ports 0 (above) and 1 (below) of the split.
fn split_block(name: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, _, target) = block(ProfileDoc::empty_derived(name, Tol::witness()), 0.0);
    let (doc, plane) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: target.into(),
            tool: plane.into(),
        },
    );
    let (doc, _, far) = block(doc, 5.0);
    (doc, split, far)
}

/// A body's vertex bits, sorted: what a stale body differs in.
fn vertex_bits(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> Vec<[u64; 3]> {
    let body = match ev.value(id).map(|v| &v.payload) {
        Some(editor_core::ValuePayload::Body(b)) => b,
        Some(editor_core::ValuePayload::Boolean(editor_core::BooleanValue::Body {
            body, ..
        })) => body,
        other => panic!("not a body: {other:?} / {:?}", ev.node_error(id)),
    };
    let mut bits: Vec<[u64; 3]> = body
        .points()
        .map(|(_, p)| p.to_array().map(f64::to_bits))
        .collect();
    bits.sort_unstable();
    bits
}

/// **A read's port is in the keys** (D4, DR-59's class; review B's
/// M2): a boolean reading a split's upper half, re-pointed at its lower
/// half, keys apart, so evaluating the re-pointed document against the
/// first evaluation's memo builds what a fresh evaluation builds — not
/// the upper half's body served from the memo.
#[test]
fn a_port_re_point_keys_apart_and_the_memo_serves_no_stale_half() {
    let (doc, split, far) = split_block("s2b-port-key");
    let (doc, boolean) = insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: Operand::output(split, 0),
            b: far.into(),
            declare: Vec::new(),
        },
    );
    let opts = editor_core::EvalOptions::default();
    let cancel = editor_core::CancelToken::new();
    let prior = editor_core::evaluate::<f64>(&doc, None, &cancel, &opts, Tol::witness());
    let moved = applied(
        &doc,
        DocEdit::SetParam {
            node: boolean,
            slot: SlotId::Operand(OperandSlot::A),
            value: Operand::output(split, 1).into(),
            fresh: Vec::new(),
        },
    )
    .doc;
    let fresh = editor_core::evaluate::<f64>(&moved, None, &cancel, &opts, Tol::witness());
    let memo = editor_core::evaluate::<f64>(&moved, Some(&prior), &cancel, &opts, Tol::witness());
    let key = |ev: &editor_core::Evaluation<f64>| ev.value(boolean).map(|v| v.content_key);
    assert_ne!(key(&prior), key(&fresh), "the two halves key apart");
    assert_ne!(
        vertex_bits(&prior, boolean),
        vertex_bits(&fresh, boolean),
        "and are two bodies"
    );
    assert_eq!(
        vertex_bits(&memo, boolean),
        vertex_bits(&fresh, boolean),
        "the memo builds what a fresh evaluation builds"
    );
    assert_eq!(
        format!("{:?}", memo.value(boolean).map(|v| &v.name_table)),
        format!("{:?}", fresh.value(boolean).map(|v| &v.name_table)),
        "and names it alike"
    );
}

/// **DM5 is over variables** (spec §3; the orchestrator's ruling on the
/// lane's seventh call): a split's two halves are two variables, so a
/// union of both is admitted, and so is a pattern of a revolve's body
/// about the revolve's own axis port (FORK-1b). One variable at two
/// seats still refuses.
#[test]
fn dm5_is_over_the_variables_read() {
    let (doc, split, _) = split_block("s2b-dm5-vars");
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![Operand::output(split, 0), Operand::output(split, 1)],
            declare: Vec::new(),
        },
    );
    // Admitted, the union cannot name two members read out of one
    // operation (DM4 keys a member by its operation), and says so
    // rather than joining one half to itself.
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    assert!(
        matches!(
            ev.node_error(union).map(|e| &e.kind),
            Some(NodeErrorKind::MembersShareAnOperation { operation, members: (0, 1) })
                if *operation == split
        ),
        "{:?}",
        ev.node_error(union)
    );
    assert!(
        matches!(
            fixture::insert_refused(
                &doc,
                Node::Union {
                    members: vec![Operand::output(split, 0), Operand::output(split, 0)],
                    declare: Vec::new(),
                },
            ),
            EditError::DuplicateInput { .. }
        ),
        "one half twice is one variable read twice"
    );

    let (doc, plane, profile) = on_frame_keeping(
        ProfileDoc::empty_derived("s2b-dm5-axis", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(1.5, 0.5, 0.5)],
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis: axis.into(),
            angle: fixture::ang(0.5),
        },
    );
    insert(
        doc,
        Node::Pattern {
            input: Operand::output(revolve, 0),
            count: editor_core::Formula::count(3),
            kind: PatternKind::Circular {
                axis: Operand::output(revolve, 1),
                step: fixture::ang(1.0),
            },
        },
    );
}

/// **Each of a split's two halves is read as itself, and a pair
/// declared across them is sided by the half that holds each name.**
/// Both halves are read at the split's one site. Undeclared, the pair
/// boolean of them refuses the rest contact across the section; under
/// a declared rest named in either order it is the whole block, where
/// one projection per node read one half twice. A side naming what
/// neither half holds — a wall of the block the cut renamed in both —
/// refuses as a site no operand's table answers. A union of the two
/// refuses before any of that: it keys each member by the operation it
/// reads (DM4).
#[test]
fn a_pair_declared_across_one_splits_halves_is_sided_by_table() {
    let (doc, split, _) = split_block("s2b-split-siding");
    let before = fixture::run(&doc, &editor_core::EvalOptions::default());
    let target = doc
        .operation_of(doc.output(split, 0).expect("the upper half"))
        .and_then(|_| match doc.node(split) {
            Some(Node::Split { target, .. }) => doc.operation_of(*target),
            _ => None,
        })
        .expect("the split's target");
    let held = |id| before.value(id).expect("a value").name_table.clone();
    let (whole, cut) = (held(target), held(split));
    let wall = whole
        .iter()
        .map(|(name, _)| name.clone())
        .find(|name| name.kind == editor_core::EntityKind::Face && cut.lookup(name).is_none())
        .expect("a wall the cut renamed");
    let section = |side| crate::corpus::part_select::section_face(split, side);
    let half = |h: editor_core::SplitHalf| Operand::output(split, h.port());
    let pair = |first, second| {
        editor_core::declare_rest(vec![(
            editor_core::SitedRef::new(split, first),
            editor_core::SitedRef::new(split, second),
        )])
    };
    let (above, below) = (
        section(editor_core::SplitHalf::Above),
        section(editor_core::SplitHalf::Below),
    );
    let boolean = |declare| Node::Boolean {
        op: editor_core::BooleanOp::Union,
        a: half(editor_core::SplitHalf::Above),
        b: half(editor_core::SplitHalf::Below),
        declare,
    };
    let union = |declare| Node::Union {
        members: vec![
            half(editor_core::SplitHalf::Below),
            half(editor_core::SplitHalf::Above),
        ],
        declare,
    };
    let (doc, undeclared) = insert(doc, boolean(Vec::new()));
    let (doc, joined) = insert(doc, boolean(pair(below.clone(), above.clone())));
    let (doc, flipped) = insert(doc, boolean(pair(above.clone(), below.clone())));
    let (doc, fused) = insert(doc, union(pair(above.clone(), below)));
    let (doc, stray_boolean) = insert(doc, boolean(pair(above, wall)));
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    assert!(
        matches!(
            ev.node_error(undeclared).map(|e| &e.kind),
            Some(NodeErrorKind::UndeclaredCoincidence { .. })
        ),
        "{:?}",
        ev.node_error(undeclared)
    );
    for id in [joined, flipped] {
        assert!(ev.value(id).is_some(), "{:?}", ev.node_error(id));
        let body = crate::corpus::body_of(&ev, id);
        let volume = topo::mass_properties(body, Tol::witness())
            .expect("mass properties")
            .volume;
        assert!(
            (volume - 1.0).abs() < 1e-12,
            "the pair boolean of the two halves is the whole block: {volume}"
        );
    }
    assert!(
        matches!(
            ev.node_error(fused).map(|e| &e.kind),
            Some(NodeErrorKind::MembersShareAnOperation { operation, members: (0, 1) })
                if *operation == split
        ),
        "{:?}",
        ev.node_error(fused)
    );
    assert!(
        matches!(
            ev.node_error(stray_boolean).map(|e| &e.kind),
            Some(NodeErrorKind::DeclareSiteNotAnOperand { at }) if *at == split
        ),
        "{:?}",
        ev.node_error(stray_boolean)
    );
}

/// **Every operand family in one document** (review A's MINOR-2): a
/// union, a loft, a sweep, a linear pattern and a part of one of its
/// instances, a transform, a split read at each half, a fillet, a
/// circular pattern of a revolve's body, and an instance of a part — so
/// the one-shot's roots and reads cover each family, a list member
/// among them. Inserted, never evaluated: what the one-shot compares is
/// the saved document. `pre_b_families.json` is this document as the
/// base built it, through its own fixtures, before an operand was a
/// read, with a part of each half where it now reads each port
/// ([`up_to_ids::split_half_parts_as_reads`]).
pub(crate) fn families_document() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("s2b-families", Tol::witness());
    let (doc, plane, low) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.5)],
    );
    let (doc, _, high) = on_frame_keeping(
        doc,
        [0.0, 0.0, 2.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.5, 0.5, 0.25)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: low.into(),
            distance: len(1.0),
            side: editor_core::ExtrudeSide::Along,
        },
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![low.into(), high.into()],
            v_degree: editor_core::Formula::count(1),
        },
    );
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile: high.into(),
            path: low.into(),
            stations: editor_core::Formula::count(4),
            v_degree: editor_core::Formula::count(3),
        },
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (3.0, 0.0), (0.0, 1.0)));
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: low.into(),
            axis: axis.into(),
            angle: fixture::ang(1.0),
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body.into(),
            count: editor_core::Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, _) = insert(
        doc,
        Node::Pattern {
            input: Operand::output(revolve, 0),
            count: editor_core::Formula::count(2),
            kind: PatternKind::Circular {
                axis: axis.into(),
                step: fixture::ang(1.0),
            },
        },
    );
    let (doc, middle) = insert(
        doc,
        Node::Part {
            of: pattern.into(),
            select: editor_core::PartSelect::Instance(editor_core::Formula::count(1)),
        },
    );
    let (doc, cut) = insert(
        doc,
        Node::Datum(editor_core::Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: loft.into(),
            tool: cut.into(),
        },
    );
    let (doc, moved) = insert(doc, xform(sweep, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let (doc, rounded) = insert(doc, Node::fillet(body, len(0.1), Vec::new()));
    let (doc, instance) = insert(
        doc,
        Node::instantiate_part(editor_core::DocRef {
            id: editor_core::DocumentId::derive("s2b-families-part"),
            pin: editor_core::ContentPin::of_bytes(b"s2b-families-part"),
        }),
    );
    let (doc, _) = insert(
        doc,
        Node::Union {
            members: vec![
                middle.into(),
                Operand::output(split, editor_core::SplitHalf::Above.port()),
                Operand::output(split, editor_core::SplitHalf::Below.port()),
                moved.into(),
                rounded.into(),
                instance.into(),
            ],
            declare: Vec::new(),
        },
    );
    doc
}

/// `pre_b_families.json`, the base's save of [`families_document`],
/// and this build's save of it, as JSON bodies.
fn families_pair() -> (serde_json::Value, serde_json::Value) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/intent_s2_b/pre_b_families.json");
    let base = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let text =
        save(&families_document(), &[], Tol::witness()).expect("the families document saves");
    (crate::wire::wire_body(&base), crate::wire::wire_body(&text))
}

/// A union of two blocks under a transform, saved: the file the load
/// door rows below doctor.
fn union_under_a_transform() -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("s2b-load-door", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![a.into(), b.into()],
            declare: Vec::new(),
        },
    );
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    (doc, b, union, moved)
}

/// `text` with every union member reading `from` reading `to` instead.
fn member_re_read(text: &str, from: &str, to: &str) -> String {
    crate::wire::doctored(text, |wire| {
        for node in wire["snapshot"]["nodes"]
            .as_object_mut()
            .expect("nodes")
            .values_mut()
        {
            if let Some(members) = node.get_mut("Union").and_then(|u| u.get_mut("members")) {
                for member in members.as_array_mut().expect("a list") {
                    if member.as_str() == Some(from) {
                        *member = serde_json::json!(to);
                    }
                }
            }
        }
    })
}

/// **A file whose reads close a loop refuses `ReadCycle`** (review B's
/// m5): a union member doctored to read the output of the transform
/// that reads the union. The load door walks the read relation, not
/// the file's order.
#[test]
fn a_file_whose_reads_close_a_loop_refuses_read_cycle() {
    let (doc, b, union, moved) = union_under_a_transform();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let out = |node| doc.output(node, 0).expect("an output").0.to_string();
    let corrupt = member_re_read(&text, &out(b), &out(moved));
    let refusal = load(&corrupt, Tol::witness()).err();
    assert!(
        matches!(
            &refusal,
            Some(editor_core::PersistError::Snapshot(editor_core::SnapshotError::ReadCycle { at }))
                if [union, moved].contains(&at.id())
        ),
        "{refusal:?}"
    );
}

/// **A file reading a variable its mint log never minted refuses
/// `OperandUnminted`** (review B's m5) — what a file written before an
/// operand was a read meets, its operands spelled as node ids — with
/// the regenerate recourse in its sentence.
#[test]
fn a_file_reading_an_unminted_variable_refuses_operand_unminted() {
    let (doc, b, union, _) = union_under_a_transform();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let out_b = doc.output(b, 0).expect("an output").0.to_string();
    // The member spelled as the node id, as a pre-B file spells it.
    let corrupt = member_re_read(&text, &out_b, &b.0.to_string());
    let refusal = load(&corrupt, Tol::witness()).err();
    assert!(
        matches!(
            &refusal,
            Some(editor_core::PersistError::Snapshot(editor_core::SnapshotError::OperandUnminted {
                node,
                slot: OperandSlot::Member(1),
                ..
            })) if node.id() == union
        ),
        "{refusal:?}"
    );
    let said = refusal.map(|e| e.to_string()).unwrap_or_default();
    assert!(
        said.contains(editor_core::REGENERATE_RECOURSE),
        "the sentence carries the regenerate recourse: {said}"
    );
}

/// **A measure whose site is deleted refuses typed, and its dead site
/// is no edge** (review A's MINOR-1): the delete of a block a measure
/// reads names at is accepted, `Doc::upstream` sets the dead site aside
/// — so no walk over the relation (the roots, the cascade, the mate
/// solve's components) meets an id no node is — and evaluation refuses
/// the measure `UnresolvedSite` at that site rather than a missing
/// input. The stranded document saves and loads as itself.
#[test]
fn a_measure_whose_site_is_deleted_refuses_typed_and_keeps_no_dead_edge() {
    let doc = ProfileDoc::empty_derived("s2b-dead-site", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let face = |node| {
        editor_core::all_faces(&ev, node)
            .into_iter()
            .next()
            .expect("a block has faces")
    };
    let (doc, measure) = insert(
        doc,
        Node::measure(
            editor_core::MeasureExpr::primitive(editor_core::MeasurePrimitive::Distance {
                a: 0,
                b: 1,
            }),
            vec![
                editor_core::SitedRef::new(a, face(a)),
                editor_core::SitedRef::new(b, face(b)),
            ],
        )
        .expect("both indices address a reference"),
    );
    assert_eq!(
        doc.upstream(measure),
        vec![a, b],
        "the measure reads at both"
    );
    let deleted = applied(&doc, DocEdit::DeleteNode { id: b }).doc;
    assert_eq!(
        deleted.upstream(measure),
        vec![a],
        "the dead site is no edge"
    );
    assert!(
        !editor_core::cascade_delete_order(&deleted, a).contains(&b),
        "and no walk meets it"
    );
    let ev = fixture::run(&deleted, &editor_core::EvalOptions::default());
    assert!(
        matches!(
            ev.node_error(measure).map(|e| &e.kind),
            Some(NodeErrorKind::UnresolvedSite { at }) if *at == b
        ),
        "{:?}",
        ev.node_error(measure)
    );
    let text = save(&deleted, &[], Tol::witness()).expect("a stranded site saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");
    assert!(loaded.doc.bit_eq(&deleted), "as itself");
}
