//! Reviewer A's probes for PR 4342 (not for merging).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code, unused_imports)]

use crate::fixture::{self, insert, len, on_frame_keeping, scl, xform};
use crate::wire::{doctored, up_to_ids, wire_body};
use editor_core::{
    Datum, Dimension, DocEdit, EditError, Maintenance, Node, NodeErrorKind, Operand, OperandSlot,
    PatternKind, ProfileDoc, RecipeNodeId, SlotId, SlotKind, Took, VarKind, apply, load, save,
};
use geom_core::Tol;

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

fn try_apply(
    doc: &ProfileDoc,
    edit: DocEdit<editor_core::ProfileProgram>,
) -> Result<editor_core::Applied<editor_core::ProfileProgram>, EditError> {
    apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
}

fn set(node: RecipeNodeId, slot: OperandSlot, read: Operand) -> DocEdit<editor_core::ProfileProgram> {
    DocEdit::SetParam {
        node,
        slot: SlotId::Operand(slot),
        value: read.into(),
        fresh: Vec::new(),
    }
}

/// P1: `Operand::Node(revolve)` at an Axis seat — does the sugar pick
/// "the one output of the seat's kind" (as `Operand::Node`'s doc says)
/// or port 0 (as Q5 says)?
#[test]
fn p1_node_sugar_at_an_axis_seat() {
    let doc = ProfileDoc::empty_derived("probe-p1", Tol::witness());
    let (doc, frame, profile) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(2.0, 0.0, 0.5)],
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(frame, (0.0, 0.0), (0.0, 1.0)));
    let (doc, revolve) = insert(
        doc,
        Node::Revolve {
            profile: profile.into(),
            axis: axis.into(),
            angle: editor_core::test_support::ang(1.0),
        },
    );
    let (doc, _, other) = block(doc, 10.0);
    let r = try_apply(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Pattern {
                input: other.into(),
                count: editor_core::Formula::count(2),
                kind: PatternKind::Circular {
                    axis: revolve.into(),
                    step: editor_core::test_support::ang(1.0),
                },
            }),
            fresh: Vec::new(),
        },
    );
    println!("P1 Node(revolve) at a circular pattern's axis seat: {:?}", r.as_ref().err());
    // Spelling the port works:
    let r2 = try_apply(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Pattern {
                input: other.into(),
                count: editor_core::Formula::count(2),
                kind: PatternKind::Circular {
                    axis: Operand::output(revolve, 1),
                    step: editor_core::test_support::ang(1.0),
                },
            }),
            fresh: Vec::new(),
        },
    );
    println!("P1 Output(revolve, 1): {:?}", r2.as_ref().err());
}

/// P2: a profile on a `Datum::Plane` (a Plane, not a Frame).
#[test]
fn p2_profile_on_a_datum_plane() {
    let doc = ProfileDoc::empty_derived("probe-p2", Tol::witness());
    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.0)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let r = try_apply(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Profile(fixture::desc(plane, vec![fixture::square(0.0, 0.0, 0.5)]))),
            fresh: Vec::new(),
        },
    );
    println!("P2 profile on Datum::Plane: {:?}", r.as_ref().err());
}

/// P3: self-read and ambiguity.
#[test]
fn p3_self_read_and_ambiguous_split() {
    let doc = ProfileDoc::empty_derived("probe-p3", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: vec![a.into(), b.into()],
            declare: Vec::new(),
        },
    );
    let r = try_apply(&doc, set(union, OperandSlot::Member(0), Operand::Node(union)));
    println!("P3 union reads itself: {:?}", r.as_ref().err());
    let out = doc.output(union, 0).unwrap();
    let r = try_apply(&doc, set(union, OperandSlot::Member(0), Operand::Var(out)));
    println!("P3 union reads its own output by id: {:?}", r.as_ref().err());
    let r = try_apply(&doc, set(union, OperandSlot::Member(7), Operand::Node(a)));
    println!("P3 Member(7) of a 2-member union: {:?}", r.as_ref().err());

    let (doc, plane) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(0.0), len(0.0), len(0.5)],
            normal: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: a.into(),
            tool: plane.into(),
        },
    );
    let r = try_apply(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Boolean {
                op: editor_core::BooleanOp::Union,
                a: split.into(),
                b: b.into(),
                declare: Vec::new(),
            }),
            fresh: Vec::new(),
        },
    );
    println!("P3 Node(split) at a boolean: {:?}", r.as_ref().err());
    // DM5 over operations: both halves in one boolean.
    let r = try_apply(
        &doc,
        DocEdit::InsertNode {
            node: Box::new(Node::Boolean {
                op: editor_core::BooleanOp::Union,
                a: Operand::output(split, 0),
                b: Operand::output(split, 1),
                declare: Vec::new(),
            }),
            fresh: Vec::new(),
        },
    );
    println!("P3 both halves in one boolean: {:?}", r.as_ref().err());
    // SetMembers: kind and DM5 at the list door.
    let (doc, pc) = {
        let (d, _, p) = on_frame_keeping(
            doc,
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![fixture::square(9.0, 0.5, 0.5)],
        );
        (d, p)
    };
    let r = try_apply(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a.into(), pc.into()],
        },
    );
    println!("P3 SetMembers with a profile: {:?}", r.as_ref().err());
    let r = try_apply(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a.into(), Operand::Var(doc.output(a, 0).unwrap())],
        },
    );
    println!("P3 SetMembers duplicate by id: {:?}", r.as_ref().err());
    // Acyclic through SetMembers: a transform of the union as a member.
    let (doc, moved) = insert(doc, xform(union, [0.0, 0.0, 5.0], [0.0, 0.0, 1.0], 0.0));
    let r = try_apply(
        &doc,
        DocEdit::SetMembers {
            node: union,
            members: vec![a.into(), moved.into()],
        },
    );
    println!("P3 SetMembers cycle: {:?}", r.as_ref().err());
}

/// P4: delete the node a measure is sited at (a measure's refs are
/// not reads in B). Accepted? reported? evaluation? save/load?
#[test]
fn p4_delete_a_measure_site() {
    let doc = ProfileDoc::empty_derived("probe-p4", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let (doc, _, b) = block(doc, 3.0);
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let fa = editor_core::all_faces(&ev, a).into_iter().next().unwrap();
    let fb = editor_core::all_faces(&ev, b).into_iter().next().unwrap();
    let (doc, measure) = insert(
        doc,
        Node::measure(
            editor_core::MeasureExpr::primitive(editor_core::MeasurePrimitive::Distance {
                a: 0,
                b: 1,
            }),
            vec![
                editor_core::SitedRef::new(a, fa.clone()),
                editor_core::SitedRef::new(b, fb.clone()),
            ],
        )
        .unwrap(),
    );
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            measure: measure.into(),
            bound: len(10.0),
            dir: editor_core::AssertionDir::AtMost,
        },
    );
    println!("P4 upstream(measure) = {:?}", doc.upstream(measure));
    let r = try_apply(&doc, DocEdit::DeleteNode { id: b });
    match r {
        Err(e) => println!("P4 delete of a measured body REFUSED: {e:?}"),
        Ok(applied) => {
            println!("P4 delete accepted; maintenance = {:?}", applied.maintenance);
            let ev = fixture::run(&applied.doc, &editor_core::EvalOptions::default());
            println!("P4 measure error = {:?}", ev.node_error(measure).map(|e| &e.kind));
            println!("P4 assertion error = {:?}", ev.node_error(assertion).map(|e| &e.kind));
            let text = save(&applied.doc, &[], Tol::witness());
            match text {
                Err(e) => println!("P4 save refused: {e}"),
                Ok(text) => match load(&text, Tol::witness()) {
                    Err(e) => println!("P4 load refused: {e}"),
                    Ok(l) => println!("P4 load ok, bit_eq = {}", l.doc.bit_eq(&applied.doc)),
                },
            }
            println!("P4 upstream(measure) after = {:?}", applied.doc.upstream(measure));
        }
    }
    // Delete the measure itself: the assertion's read strands.
    let r = try_apply(&doc, DocEdit::DeleteNode { id: measure }).unwrap();
    println!("P4 delete measure: maintenance = {:?}", r.maintenance);
    let ev = fixture::run(&r.doc, &editor_core::EvalOptions::default());
    println!("P4 assertion error = {:?}", ev.node_error(assertion).map(|e| &e.kind));
}

/// P5: the load door on doctored files.
#[test]
fn p5_load_door_on_doctored_files() {
    let doc = ProfileDoc::empty_derived("probe-p5", Tol::witness());
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
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let out_b = doc.output(b, 0).unwrap().0.to_string();
    let out_moved = doc.output(moved, 0).unwrap().0.to_string();
    let distance = doc.slot(a, SlotId::Distance).unwrap().0.to_string();
    let union_key = union.0.to_string();
    // (a) an operand reading a scalar variable
    let t = doctored(&text, |w| {
        w["snapshot"]["nodes"][&union_key]["Union"]["members"][1] = serde_json::json!(distance);
    });
    println!("P5a scalar at a member: {:?}", load(&t, Tol::witness()).err().map(|e| e.to_string()));
    // (b) a never-minted id
    let t = doctored(&text, |w| {
        w["snapshot"]["nodes"][&union_key]["Union"]["members"][1] =
            serde_json::json!("999:0123456789abcdef");
    });
    println!("P5b unminted: {:?}", load(&t, Tol::witness()).err().map(|e| e.to_string()));
    // (c) a cycle: union member 1 reads the transform of the union
    let t = doctored(&text, |w| {
        w["snapshot"]["nodes"][&union_key]["Union"]["members"][1] = serde_json::json!(out_moved);
    });
    println!("P5c cycle: {:?}", load(&t, Tol::witness()).err().map(|e| e.to_string()));
    // (d) a node id (pre-B spelling) at an operand
    let t = doctored(&text, |w| {
        w["snapshot"]["nodes"][&union_key]["Union"]["members"][1] = serde_json::json!(b.0.to_string());
    });
    println!("P5d node id at an operand: {:?}", load(&t, Tol::witness()).err().map(|e| e.to_string()));
    let _ = out_b;
    // (e) the base tree's files on this build
    for f in [
        "/home/user/cad-base/crates/editor-core/tests/golden/golden.cad",
        "/home/user/cad-base/crates/editor-core/tests/corpus/die_tool.pncad",
        "/home/user/cad-base/crates/pncad/tests/plate_param.pncad",
    ] {
        let text = std::fs::read_to_string(f).unwrap();
        let e = load(&text, Tol::witness()).err().map(|e| e.to_string());
        println!("P5e {f}: {}", e.map(|s| s.chars().take(300).collect::<String>()).unwrap_or("LOADED".into()));
    }
}

/// P6: comparator mutants: a name, a value, an added node.
#[test]
fn p6_comparator_mutants() {
    let doc = ProfileDoc::empty_derived("probe-p6", Tol::witness());
    let (doc, _, a) = block(doc, 0.0);
    let doc = try_apply(
        &doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::new("depth").unwrap(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::continuous(Dimension::Length, 2.0)),
        },
    )
    .unwrap()
    .doc;
    let depth = doc.var_named("depth").unwrap();
    let doc = try_apply(
        &doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            value: Operand::Var(depth).into(),
            fresh: Vec::new(),
        },
    )
    .unwrap()
    .doc;
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let new = wire_body(&text);
    let key = depth.0.to_string();
    // name mutant
    let mut m = new.clone();
    m["snapshot"]["var_names"][&key] = serde_json::json!("depth2");
    println!("P6 renamed: {:?}", up_to_ids::same_up_to_ids(&new, &m).err());
    // value mutant
    let mut m = new.clone();
    m["snapshot"]["vars"][&key]["def"]["Free"]["Continuous"]["value"] = serde_json::json!(2.5);
    println!("P6 value: {:?}", up_to_ids::same_up_to_ids(&new, &m).err());
    // a name moved to another variable (same count)
    let mut m = new.clone();
    let other = m["snapshot"]["vars"].as_object().unwrap().keys().find(|k| **k != key).unwrap().clone();
    let names = m["snapshot"]["var_names"].as_object_mut().unwrap();
    let v = names.remove(&key).unwrap();
    names.insert(other.clone(), v);
    println!("P6 name moved: {:?}", up_to_ids::same_up_to_ids(&new, &m).err());
    // added node
    let (doc2, _, _) = block(doc.clone(), 5.0);
    let t2 = save(&doc2, &[], Tol::witness()).unwrap();
    println!("P6 added node: {:?}", up_to_ids::same_up_to_ids(&new, &wire_body(&t2)).err());
    // roots reordered (if two roots): skip
}

/// P7: strand, then repair by the slot door; then re-strand via SetMembers naming the stranded var.
#[test]
fn p7_strand_then_repair() {
    let doc = ProfileDoc::empty_derived("probe-p7", Tol::witness());
    let (doc, pa, a) = block(doc, 0.0);
    let (doc, pb, _b) = block(doc, 3.0);
    let gone = try_apply(&doc, DocEdit::DeleteNode { id: pa }).unwrap();
    println!("P7 delete profile: {:?}", gone.maintenance);
    let stranded = doc.output(pa, 0).unwrap();
    // Re-point to pb
    let fixed = try_apply(&gone.doc, set(a, OperandSlot::Profile, Operand::Node(pb))).unwrap();
    let ev = fixture::run(&fixed.doc, &editor_core::EvalOptions::default());
    println!("P7 after repair, extrude error = {:?}", ev.node_error(a).map(|e| &e.kind));
    // Re-point at the stranded id (should refuse: not live)
    let r = try_apply(&gone.doc, set(a, OperandSlot::Profile, Operand::Var(stranded)));
    println!("P7 re-point at the stranded var: {:?}", r.err());
    // A scalar slot edit on the stranded node still works?
    let r = try_apply(
        &gone.doc,
        DocEdit::SetParam {
            node: a,
            slot: SlotId::Distance,
            value: len(2.0).into(),
            fresh: Vec::new(),
        },
    );
    println!("P7 scalar edit on a stranded node: {:?}", r.err());
    // roots of the stranded doc
    println!("P7 roots before {:?} after {:?}", doc.roots(), gone.doc.roots());
}
