//! r1 review probes for INTENT stage 2 E (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::fixture;
use editor_core::{
    CapEnd, DocEdit, EntityKind, ExtrudeSide, MeasurePrimitive, Node, Operand, OperandSlot,
    ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef, SlotId, SlotValue, StableName, VarName,
    apply, load, save,
};
use fixture::{fname, insert, len, on_frame, square, wall};
use geom_core::Tol;
use std::collections::BTreeSet;

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> editor_core::Applied<ProfileProgram> {
    apply(doc, edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
}

struct Prism {
    node: RecipeNodeId,
    faces: [StableName; 2],
    edges: [StableName; 2],
    vertex: StableName,
}

fn prism(doc: ProfileDoc, x: f64) -> (ProfileDoc, Prism) {
    let (doc, profile) = on_frame(
        doc,
        [x, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, node) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let faces = [
        fname(node, wall(&doc, node, 0)),
        fname(node, wall(&doc, node, 2)),
    ];
    let mut lateral = fixture::prism_edges(&doc, node, 4)
        .into_iter()
        .skip(2)
        .step_by(3);
    let edges = [lateral.next().unwrap(), lateral.next().unwrap()];
    let vertex = fixture::cap_vertex(node, CapEnd::End, fixture::vpiece(&doc, node, 0, 0));
    (doc, Prism { node, faces, edges, vertex })
}

fn blank(id: &str) -> ProfileDoc {
    ProfileDoc::empty_derived(id, Tol::witness())
}

/// Claim 3 at the slot door: re-point a min_clearance reference to an
/// edge selection (authored) and to an existing edge selection by id.
#[test]
fn r1_slot_door_refuses_unadmitted_measure_kind() {
    let (doc, p) = prism(blank("r1-slot"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, m) = fixture::measure_node(
        &doc,
        MeasurePrimitive::MinClearance { a: 0, b: 1 },
        vec![SitedRef::at_mint(p.faces[0].clone()), SitedRef::at_mint(q.faces[0].clone())],
    );
    let slot = SlotId::Operand(OperandSlot::Measured(editor_core::MeasureVerb::MinClearance, 0));
    let r = apply(
        &doc,
        &DocEdit::SetParam {
            node: m,
            slot,
            value: SlotValue::Read(Operand::select(p.node, vec![p.edges[0].clone()])),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-SLOT authored edge -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
    assert!(matches!(r, Err(editor_core::EditError::SlotVarKind { .. })));
    // An existing Edge selection by id (a distance's).
    let (doc, d) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![SitedRef::at_mint(p.edges[0].clone()), SitedRef::at_mint(q.faces[0].clone())],
    );
    let edge_sel = doc.node(d).unwrap().operand_rows()[0].1;
    let r = apply(
        &doc,
        &DocEdit::SetParam {
            node: m,
            slot,
            value: SlotValue::Read(Operand::Var(edge_sel)),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-SLOT by id -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
    assert!(matches!(r, Err(editor_core::EditError::SlotVarKind { .. })));
}

/// Claim 4: delete the body under a fillet that selects two of its
/// edges, and under a shared (named) selection read twice.
#[test]
fn r1_delete_strand_rows() {
    let (doc, p) = prism(blank("r1-del"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, fillet) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone(), p.edges[1].clone()]),
    );
    let applied = push(&doc, &DocEdit::DeleteNode { id: p.node });
    for row in &applied.maintenance {
        eprintln!("R1-DEL fillet row: {row:?}");
    }
    let _ = fillet;
    // shared named selection read by two measures
    let (doc, m1) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![SitedRef::at_mint(p.faces[0].clone()), SitedRef::at_mint(q.faces[0].clone())],
    );
    let sel = doc.node(m1).unwrap().operand_rows()[0].1;
    let doc = push(
        &doc,
        &DocEdit::RenameVar { var: editor_core::VarRef::Id(sel), name: Some(VarName::from_static("s")) },
    )
    .doc;
    let (doc, _m2) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![Operand::Name(VarName::from_static("s")), Operand::from(SitedRef::at_mint(q.faces[1].clone()))],
    );
    let applied = push(&doc, &DocEdit::DeleteNode { id: p.node });
    let n = applied
        .maintenance
        .iter()
        .filter(|r| matches!(r, editor_core::Maintenance::StrandedSelection { .. }))
        .count();
    for row in &applied.maintenance {
        eprintln!("R1-DEL shared row: {row:?}");
    }
    eprintln!("R1-DEL shared StrandedSelection count = {n}");
}

/// Claim 5: a named selection shared by two remainder readers of a cut
/// body, carried across a split.
#[test]
fn r1_split_shared_named_selection() {
    let (doc, p) = prism(blank("r1-split"), 0.0);
    let doc = fixture::place(doc, p.node).0;
    let cut: BTreeSet<RecipeNodeId> = doc.ids().iter().copied().collect();
    let (doc, q) = prism(doc, 4.0);
    let doc = fixture::place(doc, q.node).0;
    let (doc, m1) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![SitedRef::at_mint(p.faces[0].clone()), SitedRef::at_mint(q.faces[0].clone())],
    );
    let sel = doc.node(m1).unwrap().operand_rows()[0].1;
    let doc = push(
        &doc,
        &DocEdit::RenameVar { var: editor_core::VarRef::Id(sel), name: Some(VarName::from_static("s")) },
    )
    .doc;
    let (doc, m2) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![Operand::Name(VarName::from_static("s")), Operand::from(SitedRef::at_mint(q.faces[1].clone()))],
    );
    let out = fixture::split_world(&doc, &cut, editor_core::DocumentId::derive("r1-split-part"), Tol::witness(), None);
    let out = match out {
        Ok(o) => o,
        Err(e) => {
            eprintln!("R1-SPLIT refused: {e:?}");
            return;
        }
    };
    let r = &out.remainder;
    let a = r.node(m1).map(|n| n.operand_rows()[0].1);
    let b = r.node(m2).map(|n| n.operand_rows()[0].1);
    eprintln!("R1-SPLIT m1 reads {a:?}, m2 reads {b:?}, shared = {}", a == b);
    eprintln!("R1-SPLIT var_named(s) = {:?}", r.var_named("s"));
    if let Some(v) = r.var_named("s") {
        eprintln!("R1-SPLIT s is {:?}", r.var(v));
    }
    for row in &out.remainder_maintenance {
        eprintln!("R1-SPLIT maintenance: {row:?}");
    }
    let text = save(r, &[], Tol::witness());
    eprintln!("R1-SPLIT remainder saves: {:?}", text.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
}

/// Claim 2 at the load door: an edge set out of order.
#[test]
fn r1_load_door_noncanonical_edge_set() {
    let (doc, p) = prism(blank("r1-load"), 0.0);
    let (doc, _f) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone(), p.edges[1].clone()]),
    );
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let (head, body) = text.split_once('\n').unwrap();
    let mut v: serde_json::Value = serde_json::from_str(body).unwrap();
    // find the arrays named "names" of length 2 and swap
    fn walk(v: &mut serde_json::Value, hit: &mut usize) {
        match v {
            serde_json::Value::Object(m) => {
                for (k, x) in m.iter_mut() {
                    if k == "names" {
                        if let serde_json::Value::Array(a) = x {
                            if a.len() == 2 {
                                a.swap(0, 1);
                                *hit += 1;
                            }
                        }
                    }
                    walk(x, hit);
                }
            }
            serde_json::Value::Array(a) => a.iter_mut().for_each(|x| walk(x, hit)),
            _ => {}
        }
    }
    let mut hit = 0;
    walk(&mut v, &mut hit);
    eprintln!("R1-LOAD swapped {hit} name arrays");
    let corrupt = format!("{head}\n{}\n", serde_json::to_string_pretty(&v).unwrap());
    let got = load(&corrupt, Tol::witness());
    eprintln!("R1-LOAD -> {:?}", got.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
    assert!(got.is_err());
}

/// Rebind into an already-selected edge shrinks the set; the reader's
/// var id holds; and a body:None rebind of a name only a selection
/// holds refuses.
#[test]
fn r1_rebind_shapes() {
    let (doc, p) = prism(blank("r1-rebind"), 0.0);
    let (doc, f) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone(), p.edges[1].clone()]),
    );
    let sel = fixture::selection_read(&doc, f);
    let out = push(
        &doc,
        &DocEdit::Rebind { body: doc.output(p.node, 0), from: p.edges[0].clone(), to: p.edges[1].clone() },
    );
    assert_eq!(fixture::selection_read(&out.doc, f), sel);
    eprintln!("R1-REBIND shrunk -> {:?}", fixture::selected(&out.doc, sel).len());
    let none = apply(
        &doc,
        &DocEdit::Rebind { body: None, from: p.edges[0].clone(), to: p.edges[1].clone() },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-REBIND body None -> {:?}", none.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
    // A rebind addressed to the selection's own var as body, not a body.
    let wrong = apply(
        &doc,
        &DocEdit::Rebind { body: Some(sel), from: p.edges[0].clone(), to: p.edges[1].clone() },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-REBIND body=selection var -> {:?}", wrong.as_ref().map(|_| ()).map_err(|e| format!("{e:?}")));
    let _ = (EntityKind::Face, p.vertex);
}

/// Claim 6: two fillets authored apart (two selection vars, two radius
/// vars) on the same names share a content key; a rebind moves it.
#[test]
fn r1_content_key_tracks_names_not_ids() {
    let (doc, p) = prism(blank("r1-key"), 0.0);
    let (doc, f1) = insert(doc, Node::chamfer(p.node, len(0.05), vec![p.edges[0].clone()]));
    let (doc, f2) = insert(doc, Node::chamfer(p.node, len(0.05), vec![p.edges[0].clone()]));
    let (doc, f3) = insert(doc, Node::chamfer(p.node, len(0.05), vec![p.edges[1].clone()]));
    assert_ne!(fixture::selection_read(&doc, f1), fixture::selection_read(&doc, f2));
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let k = |id| ev.value(id).expect("evaluates").content_key;
    eprintln!("R1-KEY f1==f2 {} f1==f3 {}", k(f1) == k(f2), k(f1) == k(f3));
    assert_eq!(k(f1), k(f2));
    assert_ne!(k(f1), k(f3));
    let sel2 = fixture::selection_read(&doc, f2);
    let rebound = push(
        &doc,
        &DocEdit::Rebind { body: doc.output(p.node, 0), from: p.edges[0].clone(), to: p.edges[1].clone() },
    )
    .doc;
    assert_eq!(fixture::selection_read(&rebound, f2), sel2);
    let ev2 = fixture::run(&rebound, &editor_core::EvalOptions::default());
    let k2 = |id| ev2.value(id).expect("evaluates").content_key;
    eprintln!("R1-KEY after rebind f1 moved {} f1==f3 {}", k(f1) != k2(f1), k2(f1) == k2(f3));
    assert_eq!(k2(f1), k2(f3));
    // measure: two measures authored apart, same refs.
    let (doc, q) = prism(doc, 4.0);
    let refs = || vec![SitedRef::at_mint(p.faces[0].clone()), SitedRef::at_mint(q.faces[0].clone())];
    let (doc, m1) = fixture::measure_node(&doc, MeasurePrimitive::Distance { a: 0, b: 1 }, refs());
    let (doc, m2) = fixture::measure_node(&doc, MeasurePrimitive::Distance { a: 0, b: 1 }, refs());
    let (doc, m3) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![SitedRef::at_mint(p.faces[1].clone()), SitedRef::at_mint(q.faces[0].clone())],
    );
    let ev = fixture::run(&doc, &editor_core::EvalOptions::default());
    let k = |id| ev.value(id).map(|v| v.content_key);
    eprintln!("R1-KEY m1==m2 {} m1==m3 {}", k(m1) == k(m2), k(m1) == k(m3));
    assert_eq!(k(m1), k(m2));
    assert_ne!(k(m1), k(m3));
}

/// Claim 2: an authored edge set out of order at the edit door, raw
/// (not through Node::fillet's canonicalising constructor).
#[test]
fn r1_edit_door_noncanonical_authored_set() {
    let (doc, p) = prism(blank("r1-door"), 0.0);
    let mut names = vec![p.edges[0].clone(), p.edges[1].clone()];
    names.sort();
    names.reverse();
    let r = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Fillet {
                radius: len(0.05),
                selection: Operand::select(p.node, names.clone()),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-DOOR reversed edges -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e}")));
    let mut dup = vec![p.faces[0].clone(), p.faces[0].clone()];
    let r = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Shell { thickness: len(0.05), open: Operand::select(p.node, std::mem::take(&mut dup)) }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-DOOR repeated face -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e}")));
    // a fillet authored with an empty selection
    let r = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Fillet { radius: len(0.05), selection: Operand::select(p.node, vec![]) }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-DOOR empty fillet -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e}")));
    // a fillet authored with face names (kind check deferred to eval)
    let r = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Fillet { radius: len(0.05), selection: Operand::select(p.node, vec![p.faces[0].clone()]) }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    eprintln!("R1-DOOR fillet of a face -> {:?}", r.as_ref().map(|_| ()).map_err(|e| format!("{e}")));
}

/// Claim 4: a re-point that takes a selected name out of reach, with
/// the selection named and read by two fillets.
#[test]
fn r1_repoint_strands_once_per_name() {
    use editor_core::placement::Frame;
    let (doc, p) = prism(blank("r1-repoint"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, t) = insert(
        doc,
        Node::Transform {
            input: p.node.into(),
            placement: editor_core::Placement::literal(&Frame::translation([0.0, 10.0, 0.0])),
        },
    );
    let (doc, f1) = insert(doc, Node::fillet(t, len(0.05), vec![p.edges[0].clone(), p.edges[1].clone()]));
    let sel = fixture::selection_read(&doc, f1);
    let doc = push(
        &doc,
        &DocEdit::RenameVar { var: editor_core::VarRef::Id(sel), name: Some(VarName::from_static("e")) },
    )
    .doc;
    let (doc, _f2) = insert(
        doc,
        Node::Chamfer { distance: len(0.05), selection: Operand::Name(VarName::from_static("e")) },
    );
    let out = push(
        &doc,
        &DocEdit::SetParam {
            node: t,
            slot: SlotId::Operand(OperandSlot::Input),
            value: SlotValue::Read(Operand::Node(q.node)),
            fresh: Vec::new(),
        },
    );
    let n = out
        .maintenance
        .iter()
        .filter(|r| matches!(r, editor_core::Maintenance::StrandedSelection { .. }))
        .count();
    for r in &out.maintenance {
        eprintln!("R1-REPOINT row: {r}");
    }
    eprintln!("R1-REPOINT StrandedSelection count = {n}");
}

/// An insert whose authored selection names a node not upstream of its
/// body: refused, reported, or silent?
#[test]
fn r1_insert_selection_off_body() {
    let (doc, p) = prism(blank("r1-offbody"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let r = apply(
        &doc,
        &DocEdit::InsertNode { node: Box::new(Node::fillet(q.node, len(0.05), vec![p.edges[0].clone()])), fresh: Vec::new() },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    match r {
        Ok(a) => eprintln!("R1-OFFBODY accepted, maintenance = {:?}", a.maintenance.iter().map(|m| m.to_string()).collect::<Vec<_>>()),
        Err(e) => eprintln!("R1-OFFBODY refused: {e}"),
    }
}

#[test]
fn r1_setparam_selection_off_body() {
    let (doc, p) = prism(blank("r1-offbody2"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, f) = insert(doc, Node::fillet(q.node, len(0.05), vec![q.edges[0].clone()]));
    let a = push(
        &doc,
        &DocEdit::SetParam {
            node: f,
            slot: SlotId::Operand(OperandSlot::Selection),
            value: SlotValue::Read(Operand::select(q.node, vec![p.edges[0].clone()])),
            fresh: Vec::new(),
        },
    );
    eprintln!("R1-OFFBODY2 setparam maintenance = {:?}", a.maintenance.iter().map(|m| m.to_string()).collect::<Vec<_>>());
}
