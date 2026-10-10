//! **INTENT stage 2 E — a selection is a definition** (spec §9 rows
//! 17 and 18, and FORK-VTX's door table).
//!
//! A fillet, a chamfer, a shell, a derived frame and a measure each
//! read a selection variable (`VarDef::Select { body, names }`) the
//! insert door mints from what was authored at the seat. These rows pin
//! what that representation promises beyond the readers' own suites:
//!
//! - **Row 17**: `Rebind { body, from, to }` rewrites exactly the
//!   selections of `body` naming `from`, in place, so every reader keeps
//!   the variable it read.
//! - **Row 18**: a selection authored at two seats is two variables,
//!   and a seat shares one by reading it by name; a repair of the shared
//!   one reaches both readers at once.
//! - **FORK-VTX**: a measure's references admit the kinds their
//!   primitive admits, refused at the edit door and the load door as
//!   the seat's kind.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    CapEnd, DocEdit, EditError, EntityKind, ExtrudeSide, MeasurePrimitive, MeasureVerb, Node,
    Operand, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId, SitedRef, SlotKind,
    SnapshotError, StableName, VarKind, VarName, apply, load, save,
};
use fixture::{fname, insert, len, on_frame, square, wall};
use geom_core::Tol;

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

/// A unit square prism at `x`, and the names the rows read off it: two
/// walls, two lateral edges and a cap vertex.
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
    let edges = [
        lateral.next().expect("a strut"),
        lateral.next().expect("a strut"),
    ];
    let vertex = fixture::cap_vertex(node, CapEnd::End, fixture::vpiece(&doc, node, 0, 0));
    (
        doc,
        Prism {
            node,
            faces,
            edges,
            vertex,
        },
    )
}

fn blank(id: &str) -> ProfileDoc {
    ProfileDoc::empty_derived(id, Tol::witness())
}

/// **Row 17: a rebind rewrites exactly the selections of its body, in
/// place.** Two selections on one prism name the same edge (a fillet's
/// and a measure's) and a third, on another prism, names an edge of
/// that one. Rebinding the shared edge on the first prism rewrites both
/// of its selections and leaves the third alone, and every reader reads
/// the variable it read before: the repair mints nothing.
#[test]
fn a_rebind_rewrites_exactly_its_bodys_selections_in_place() {
    let (doc, p) = prism(blank("s2e-rebind"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, fillet) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone()]),
    );
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::at_mint(p.edges[0].clone()),
            SitedRef::at_mint(q.edges[0].clone()),
        ],
    );
    let (doc, other) = insert(
        doc,
        Node::fillet(q.node, len(0.05), vec![q.edges[0].clone()]),
    );
    let reads = |doc: &ProfileDoc, node| {
        doc.node(node)
            .expect("the node is there")
            .operand_rows()
            .into_iter()
            .map(|(_, var)| var)
            .collect::<Vec<_>>()
    };
    let before = [
        reads(&doc, fillet),
        reads(&doc, measure),
        reads(&doc, other),
    ];

    let rebound = push(
        &doc,
        &DocEdit::Rebind {
            body: doc.output(p.node, 0),
            from: p.edges[0].clone(),
            to: p.edges[1].clone(),
        },
    );
    assert_eq!(
        [
            reads(&rebound, fillet),
            reads(&rebound, measure),
            reads(&rebound, other)
        ],
        before,
        "every reader reads the variable it read: the repair mints nothing"
    );
    let fillet_select = fixture::selection_read(&rebound, fillet);
    assert_eq!(
        fixture::selected(&rebound, fillet_select),
        vec![p.edges[1].clone()]
    );
    let [first, second] = [0, 1].map(|i| reads(&rebound, measure)[i]);
    assert_eq!(fixture::selected(&rebound, first), vec![p.edges[1].clone()]);
    assert_eq!(
        fixture::selected(&rebound, second),
        vec![q.edges[0].clone()],
        "the measure's selection of the other prism is not this body's"
    );
    let other_select = fixture::selection_read(&rebound, other);
    assert_eq!(
        fixture::selected(&rebound, other_select),
        vec![q.edges[0].clone()],
        "another body's selection is untouched"
    );
}

/// **Row 17's other half: a rebind addressed to a body no selection of
/// which names `from` refuses**, even when a selection of another body
/// does — the body is part of the address, not a filter on it.
#[test]
fn a_rebind_at_a_body_whose_selections_do_not_name_it_refuses() {
    let (doc, p) = prism(blank("s2e-rebind-address"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, _) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone()]),
    );
    let refused = apply(
        &doc,
        &DocEdit::Rebind {
            body: doc.output(q.node, 0),
            from: p.edges[0].clone(),
            to: p.edges[1].clone(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    );
    assert!(
        matches!(refused, Err(EditError::RebindNoReferences { .. })),
        "{refused:?}"
    );
}

/// **Row 18: two seats, two selections, unless one reads the other's.**
/// The same reference authored at two measures mints two variables
/// (the door does not dedupe by value); a third measure that reads the
/// first's selection by name shares it, and one rebind of that body
/// repairs every reader of every selection naming the edge.
#[test]
fn a_selection_authored_twice_is_two_variables_and_a_named_one_is_shared() {
    let (doc, p) = prism(blank("s2e-shared"), 0.0);
    let reference = || SitedRef::at_mint(p.edges[0].clone());
    let face = || SitedRef::at_mint(p.faces[1].clone());
    let (doc, m1) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![reference(), face()],
    );
    let (doc, m2) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![reference(), face()],
    );
    let first = |doc: &ProfileDoc, m| doc.node(m).expect("a measure").operand_rows()[0].1;
    assert_ne!(
        first(&doc, m1),
        first(&doc, m2),
        "authored apart, the two references are two variables"
    );

    let strut = VarName::from_static("strut");
    let doc = push(
        &doc,
        &DocEdit::RenameVar {
            var: editor_core::VarRef::Id(first(&doc, m1)),
            name: Some(strut.clone()),
        },
    );
    let (doc, m3) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![Operand::Name(strut), Operand::from(face())],
    );
    assert_eq!(
        first(&doc, m3),
        first(&doc, m1),
        "read by name, the selection is one variable two measures read"
    );

    let rebound = push(
        &doc,
        &DocEdit::Rebind {
            body: doc.output(p.node, 0),
            from: p.edges[0].clone(),
            to: p.edges[1].clone(),
        },
    );
    for m in [m1, m2, m3] {
        assert_eq!(
            fixture::selected(&rebound, first(&rebound, m)),
            vec![p.edges[1].clone()],
            "measure {m:?} reads the repaired edge"
        );
    }
}

/// What each measure primitive admits, as the door says it (FORK-VTX).
const TABLE: [(MeasureVerb, [bool; 4]); 4] = [
    // [body, face, edge, vertex]
    (MeasureVerb::Distance, [false, true, true, true]),
    (MeasureVerb::Angle, [false, true, true, false]),
    (MeasureVerb::MinClearance, [true, true, false, false]),
    (MeasureVerb::Gap, [false, true, false, false]),
];

fn primitive<R>(verb: MeasureVerb, a: R, b: R) -> MeasurePrimitive<R> {
    match verb {
        MeasureVerb::Distance => MeasurePrimitive::Distance { a, b },
        MeasureVerb::Angle => MeasurePrimitive::Angle { a, b },
        MeasureVerb::MinClearance => MeasurePrimitive::MinClearance { a, b },
        MeasureVerb::Gap => MeasurePrimitive::Gap { outer: a, inner: b },
    }
}

/// **FORK-VTX at the edit door**: every primitive against every kind a
/// reference can name, in the first argument, against a face (which
/// every primitive admits) in the second. An admitted kind inserts; any
/// other refuses `SlotVarKind`, naming the kind found and the seat's
/// admitted kinds, at the first reference.
#[test]
fn each_primitive_admits_its_kinds_at_the_edit_door() {
    let (doc, p) = prism(blank("s2e-vtx-table"), 0.0);
    let body = StableName {
        kind: EntityKind::Body,
        node: p.node,
        path: Vec::new(),
    };
    let kinds = [
        (VarKind::Body, body),
        (VarKind::Face, p.faces[0].clone()),
        (VarKind::Edge, p.edges[0].clone()),
        (VarKind::Vertex, p.vertex.clone()),
    ];
    for (verb, admits) in TABLE {
        for ((kind, name), admitted) in kinds.iter().zip(admits) {
            let got = editor_core::measure(
                &doc,
                &[primitive(
                    verb,
                    SitedRef::at_mint(name.clone()),
                    SitedRef::at_mint(p.faces[1].clone()),
                )],
                Tol::witness(),
                &editor_core::RefusingReach,
            );
            match (got, admitted) {
                (Ok(_), true) => {}
                (
                    Err(EditError::SlotVarKind {
                        found, expected, ..
                    }),
                    false,
                ) => {
                    assert_eq!(found, *kind, "{verb:?} at a {kind}");
                    assert_eq!(expected, SlotKind::Measured(verb), "{verb:?} at a {kind}");
                }
                (other, _) => panic!(
                    "{verb:?} at a {kind}: admitted {admitted}, got {:?}",
                    other.map(|_| ())
                ),
            }
        }
    }
}

/// **FORK-VTX at the load door**: a file whose measure reads a vertex
/// under a primitive that does not admit one refuses `SlotVarKind` at
/// load. The file is a saved `distance` between a vertex and a face
/// with its primitive's tag re-spelled `MinClearance` (a length too, so
/// the output's signature still holds): every other byte is what a
/// door wrote.
#[test]
fn a_file_whose_measure_reads_an_unadmitted_kind_refuses_at_load() {
    let (doc, p) = prism(blank("s2e-vtx-load"), 0.0);
    let (doc, _) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::at_mint(p.vertex.clone()),
            SitedRef::at_mint(p.faces[1].clone()),
        ],
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(load(&text, Tol::witness()).is_ok(), "the saved file loads");
    assert_eq!(text.matches("\"Distance\"").count(), 1, "one distance");
    let corrupt = text.replace("\"Distance\"", "\"MinClearance\"");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            found: VarKind::Vertex,
            expected: SlotKind::Measured(MeasureVerb::MinClearance),
            ..
        })) => {}
        other => {
            panic!("a min_clearance at a vertex must refuse by the seat's kind, got {other:?}")
        }
    }
}

/// A placed unit prism the split rows cut out: its frame, profile,
/// extrude and world placement, and the extrude's walls and an edge.
struct Cut {
    nodes: std::collections::BTreeSet<RecipeNodeId>,
    extrude: RecipeNodeId,
    faces: [StableName; 2],
    edge: StableName,
}

fn placed_prism(doc: ProfileDoc) -> (ProfileDoc, Cut) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let frame = doc.ids()[doc.ids().len() - 2];
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, placed) = fixture::place(doc, extrude);
    let faces = [
        fname(extrude, wall(&doc, extrude, 0)),
        fname(extrude, wall(&doc, extrude, 2)),
    ];
    let edge = fixture::prism_edges(&doc, extrude, 4).remove(2);
    let cut = Cut {
        nodes: [frame, profile, extrude, placed].into(),
        extrude,
        faces,
        edge,
    };
    (doc, cut)
}

/// **A split keeps a named selection one variable.** Two measures in
/// the remainder read one named selection of the cut body; after the
/// split they still read one variable, under the same name, now a
/// selection of the instance's body, and no selection of the dead cut
/// body is left behind.
#[test]
fn a_split_keeps_a_named_crossing_selection_shared() {
    let (doc, p) = placed_prism(ProfileDoc::empty(
        editor_core::DocumentId::derive("s2e-split-shared"),
        Tol::witness(),
    ));
    let (doc, q) = prism(doc, 4.0);
    let (doc, m1) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::at_mint(p.faces[0].clone()),
            SitedRef::at_mint(q.faces[0].clone()),
        ],
    );
    let first = |doc: &ProfileDoc, m| doc.node(m).expect("a measure").operand_rows()[0].1;
    let s = VarName::from_static("s");
    let doc = push(
        &doc,
        &DocEdit::RenameVar {
            var: editor_core::VarRef::Id(first(&doc, m1)),
            name: Some(s.clone()),
        },
    );
    let (doc, m2) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            Operand::Name(s.clone()),
            Operand::from(SitedRef::at_mint(q.faces[1].clone())),
        ],
    );
    let out = editor_core::split(
        &doc,
        &p.nodes,
        editor_core::DocumentId::derive("s2e-split-shared-part"),
        Tol::witness(),
        None,
    )
    .expect("the placed prism splits out");
    let rem = &out.remainder;
    let (r1, r2) = (first(rem, m1), first(rem, m2));
    assert_eq!(r1, r2, "the two measures still read one selection");
    assert_eq!(rem.var_named(s.as_str()), Some(r1), "under its name");
    assert_eq!(
        rem.selection(r1).map(|select| select.body),
        rem.output(out.instance, 0),
        "a selection of the instance's body"
    );
    assert!(
        rem.vars()
            .values()
            .filter_map(|var| var.def().select())
            .all(|select| rem.operation_of(select.body).is_some()),
        "no selection of a dead body is left behind"
    );
}

/// **Inline re-hosts a selection downstream of the instance.** A
/// remainder fillet selects the cut's edge on a union of the instance
/// and a local prism; inlining the instance re-anchors that selection's
/// name to the spliced node, so split then inline round-trips.
#[test]
fn inline_rehosts_a_selection_downstream_of_the_instance() {
    use crate::fixture::resolver::PartStore;
    use editor_core::BooleanOp;
    use std::sync::Arc;
    let (doc, p) = placed_prism(ProfileDoc::empty(
        editor_core::DocumentId::derive("s2e-inline-downstream"),
        Tol::witness(),
    ));
    let (doc, q) = prism(doc, 0.5);
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: p.extrude.into(),
            b: q.node.into(),
            declare: Vec::new(),
        },
    );
    let (doc, fillet) = insert(doc, Node::fillet(union, len(0.05), vec![p.edge.clone()]));
    let (doc, _) = fixture::place(doc, fillet);
    let out = editor_core::split(
        &doc,
        &p.nodes,
        editor_core::DocumentId::derive("s2e-inline-downstream-part"),
        Tol::witness(),
        None,
    )
    .expect("the placed prism splits out");
    let mut store = PartStore::new();
    store.insert(out.part.clone(), Tol::witness());
    let inlined = editor_core::inline(
        &out.remainder,
        out.instance,
        &(Arc::new(store) as Arc<dyn editor_core::PartResolver>),
        Tol::witness(),
    )
    .expect("the instance inlines");
    let names = fixture::selected(&inlined.doc, fixture::selection_read(&inlined.doc, fillet));
    assert!(
        names.iter().all(|n| inlined.doc.node(n.node).is_some()),
        "every selected name is minted by a live node: {names:?}"
    );
    assert!(
        !inlined
            .maintenance
            .iter()
            .any(|row| matches!(row, editor_core::Maintenance::StrandedSelection { .. })),
        "nothing is stranded: {:?}",
        inlined.maintenance
    );
}

fn stranded_selections(rows: &[editor_core::Maintenance]) -> usize {
    rows.iter()
        .filter(|row| matches!(row, editor_core::Maintenance::StrandedSelection { .. }))
        .count()
}

fn union_of(doc: ProfileDoc, a: RecipeNodeId, b: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Boolean {
            op: editor_core::BooleanOp::Union,
            a: a.into(),
            b: b.into(),
            declare: Vec::new(),
        },
    )
}

/// **A re-point strands a named selection whether or not anything
/// reads it**, as a delete does. A fillet's selection of a union is
/// named and the fillet deleted; re-pointing the union's first operand
/// away from the prism the name was minted on reports the selection,
/// once, exactly as deleting that prism does.
#[test]
fn a_re_point_reports_an_unread_named_selection_as_a_delete_does() {
    let (doc, p) = prism(blank("s2e-repoint-unread"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let (doc, r) = prism(doc, 8.0);
    let (doc, u) = union_of(doc, p.node, q.node);
    let (doc, fillet) = insert(doc, Node::fillet(u, len(0.05), vec![p.edges[0].clone()]));
    let doc = push(
        &doc,
        &DocEdit::RenameVar {
            var: editor_core::VarRef::Id(fixture::selection_read(&doc, fillet)),
            name: Some(VarName::from_static("kept")),
        },
    );
    let doc = push(&doc, &DocEdit::DeleteNode { id: fillet });
    let edit = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| {
        apply(doc, &edit, Tol::witness(), &editor_core::RefusingReach)
            .expect("the edit applies")
            .maintenance
    };
    let repointed = edit(
        &doc,
        DocEdit::SetParam {
            node: u,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::A),
            value: editor_core::SlotValue::Read(Operand::Node(r.node)),
            fresh: Vec::new(),
        },
    );
    let deleted = edit(&doc, DocEdit::DeleteNode { id: p.node });
    assert_eq!(stranded_selections(&repointed), 1, "{repointed:?}");
    assert_eq!(stranded_selections(&deleted), 1, "{deleted:?}");
}

/// **A selection a slot write authors strands nothing**, as one the
/// insert door authors does not: a name off the selection's body is
/// evaluation's to refuse (`SelectResolve`), and neither door reports
/// it as something an edit took.
#[test]
fn a_selection_authored_off_its_body_is_reported_by_neither_door() {
    let (doc, p) = prism(blank("s2e-authored-off-body"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let off = || Operand::select(q.node, vec![p.edges[0].clone()]);
    let inserted = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Fillet {
                radius: len(0.05),
                selection: off(),
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the insert applies");
    assert_eq!(stranded_selections(&inserted.maintenance), 0);
    let (doc, fillet) = insert(
        doc,
        Node::fillet(q.node, len(0.05), vec![q.edges[0].clone()]),
    );
    let written = apply(
        &doc,
        &DocEdit::SetParam {
            node: fillet,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Selection),
            value: editor_core::SlotValue::Read(off()),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the slot write applies");
    assert_eq!(stranded_selections(&written.maintenance), 0);
}

/// **FORK-VTX at the slot door**: re-pointing a `min_clearance`
/// reference to an edge refuses as the seat's kind, whether the edge is
/// a selection authored at the write or a variable read by name.
#[test]
fn the_slot_door_holds_a_measure_reference_to_its_primitive_kinds() {
    let (doc, p) = prism(blank("s2e-vtx-slot"), 0.0);
    let (doc, measure) = fixture::measure_node(
        &doc,
        MeasurePrimitive::MinClearance { a: 0, b: 1 },
        vec![
            SitedRef::at_mint(p.faces[0].clone()),
            SitedRef::at_mint(p.faces[1].clone()),
        ],
    );
    let (doc, fillet) = insert(
        doc,
        Node::fillet(p.node, len(0.05), vec![p.edges[0].clone()]),
    );
    let doc = push(
        &doc,
        &DocEdit::RenameVar {
            var: editor_core::VarRef::Id(fixture::selection_read(&doc, fillet)),
            name: Some(VarName::from_static("strut")),
        },
    );
    for read in [
        Operand::select(p.node, vec![p.edges[1].clone()]),
        Operand::Name(VarName::from_static("strut")),
    ] {
        let refused = apply(
            &doc,
            &DocEdit::SetParam {
                node: measure,
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Measured(
                    MeasureVerb::MinClearance,
                    0,
                )),
                value: editor_core::SlotValue::Read(read.clone()),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        );
        assert!(
            matches!(
                refused,
                Err(EditError::SlotVarKind {
                    expected: SlotKind::Measured(MeasureVerb::MinClearance),
                    ..
                })
            ),
            "{read}: {:?}",
            refused.map(|_| ())
        );
    }
}

/// **A whole body named at a measure seat must be the body read.** The
/// body's own name is read as the body; another body's name at it
/// refuses `OtherBody`, rather than measuring the body read under a
/// name that says otherwise.
#[test]
fn a_measure_reads_a_whole_body_only_under_its_own_name() {
    let (doc, p) = prism(blank("s2e-other-body"), 0.0);
    let (doc, q) = prism(doc, 4.0);
    let body = |node| StableName {
        kind: EntityKind::Body,
        node,
        path: Vec::new(),
    };
    let clearance = |a: SitedRef| {
        editor_core::measure(
            &doc,
            &[MeasurePrimitive::MinClearance {
                a,
                b: SitedRef::at_mint(q.faces[0].clone()),
            }],
            Tol::witness(),
            &editor_core::RefusingReach,
        )
    };
    let own = clearance(SitedRef::new(p.node, body(p.node))).expect("its own body");
    let read = own
        .doc
        .node(own.measured.measures[0])
        .expect("a measure")
        .operand_rows()[0]
        .1;
    assert_eq!(
        Some(read),
        own.doc.output(p.node, 0),
        "read as the body itself"
    );
    assert!(
        matches!(
            clearance(SitedRef::new(p.node, body(q.node))),
            Err(EditError::SelectionShape {
                fault: editor_core::SelectionFault::OtherBody,
                ..
            })
        ),
        "another body's name refuses"
    );
}
