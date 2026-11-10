//! **A selection reads one body** (REFERENCES DM3): its body is a body
//! seat's read, a body or one member of a family, `xs[i]`, its indices
//! the selection's own `Count` variables and its names the member's
//! own. A family read plainly there is read one member at a time, as at
//! every body seat, so a name of the family's table read there names
//! nothing of the member it is read in.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::eval6_placers_over_instances::cube_doc;
use crate::fixture;
use editor_core::{
    BodyRead, Dimension, DocEdit, EditError, EntityKind, Evaluation, Formula, MeasurePrimitive,
    Node, NodeError, NodeErrorKind, NodeResult, Operand, ProfileDoc, RecipeNodeId, RoleSeg,
    SnapshotError, StableName, ValuePayload, VarId,
};
use fixture::{insert, len, run};
use geom_core::Tol;

/// A unit cube and a row of three of it along x at spacing 2 under the
/// index `k`: member `i` occupies `[2i, 2i + 1]` in x.
fn family(label: &str) -> (ProfileDoc, RecipeNodeId, VarId, RecipeNodeId) {
    let (doc, cube) = cube_doc(label);
    let (doc, k, xs) = fixture::linear(doc, cube, [1.0, 0.0, 0.0], 2.0, 3);
    (doc, cube, k, xs)
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    run(doc, &editor_core::EvalOptions::default())
}

/// The cube's first edge name: a member's own name for its edge, since
/// a copy carries its source's names verbatim.
fn an_edge(doc: &ProfileDoc, cube: RecipeNodeId) -> StableName {
    editor_core::all_edges(&eval(doc), cube)
        .into_iter()
        .next()
        .expect("the cube has edges")
}

/// `xs[i]`.
fn member(xs: RecipeNodeId, i: i64) -> BodyRead<Formula> {
    BodyRead::indexed(xs, vec![Formula::count(i)])
}

/// The selection the fillet `id` reads.
fn selection_of(doc: &ProfileDoc, id: RecipeNodeId) -> &editor_core::var::Select {
    let Some(Node::Fillet { selection, .. }) = doc.node(id) else {
        panic!("{id:?} is a fillet");
    };
    doc.selection(*selection)
        .expect("a fillet reads a selection")
}

/// The least x any vertex of `id`'s body stands at.
fn least_x(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    let body = crate::corpus::body_of(ev, id);
    body.vertices()
        .map(|(v, _)| fixture::point(&body, v).x)
        .fold(f64::INFINITY, f64::min)
}

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {id} must fail typed, got {other:?}"),
    }
}

/// **A fillet of one member**: `fillet(xs[2])` stores the indexed read
/// with its index the selection's own and the member's own name,
/// evaluates once, at member 2's place, and keys each entity it carries
/// by the family's read with the member's qualifier,
/// `From { read: xs, of: Member { (k, 2), e } }`, as a reader of the
/// family whole would. Red if the member read is stored plain, read at
/// another member, or carried under the member's own name alone.
#[test]
fn a_member_is_selected_by_its_own_names_and_carried_as_the_familys() {
    let (doc, cube, k, xs) = family("select-member");
    let edge = an_edge(&doc, cube);
    let (doc, fillet) = insert(
        doc,
        Node::fillet(member(xs, 2), len(0.1), vec![edge.clone()]),
    );
    let select = selection_of(&doc, fillet);
    assert_eq!(
        select.names,
        std::slice::from_ref(&edge),
        "the member's own name"
    );
    assert_eq!(select.body.at.len(), 1, "the selection holds its one index");
    assert_eq!(
        doc.written(&editor_core::Expr::var(select.body.at[0], Dimension::Count)),
        Formula::count(2),
        "the index is the selection's own count"
    );
    assert_eq!(
        doc.node_reach(fillet).chain(),
        Some(&[][..]),
        "a written index reaches no index: the fillet is evaluated once"
    );
    let ev = eval(&doc);
    assert_eq!(least_x(&ev, fillet), 4.0, "the fillet stands at member 2");
    let xs_read = doc.output(xs, 0).expect("the family's read");
    let table = ev
        .table(doc.output(fillet, 0).expect("the fillet's read"))
        .expect("a table");
    let keyed: Vec<&StableName> = table
        .iter()
        .map(|(name, _)| name)
        .filter_map(|name| match name.path.as_slice() {
            [RoleSeg::From { read, of }] if *read == xs_read => Some(&**of),
            _ => None,
        })
        .collect();
    assert!(
        !keyed.is_empty(),
        "the fillet carries entities keyed by `xs`"
    );
    for of in keyed {
        assert!(
            matches!(
                of.path.as_slice(),
                [RoleSeg::Member { at, .. }] if of.node == xs && at == &[(k, 2)]
            ),
            "a carried entity keys as the family names it, member 2's: {of:?}"
        );
    }
}

/// **`xs[k]` on the family's own index chain is stored as `xs`**: a
/// per-member reader's plain read, so the fillet is a family under `k`.
/// Red if the indexed form is stored.
#[test]
fn a_member_read_at_the_familys_own_index_is_stored_plain() {
    let (doc, cube, k, xs) = family("select-own-index");
    let edge = an_edge(&doc, cube);
    let own = BodyRead::indexed(xs, vec![Formula::var(k, Dimension::Count)]);
    let (doc, fillet) = insert(doc, Node::fillet(own, len(0.1), vec![edge]));
    let select = selection_of(&doc, fillet);
    assert_eq!(
        select.body,
        BodyRead::plain(doc.output(xs, 0).expect("the family's read")),
        "`xs[k]` is stored as `xs`"
    );
    assert_eq!(
        doc.node_reach(fillet).chain(),
        Some(&[k][..]),
        "the fillet is read once per member"
    );
    let ev = eval(&doc);
    assert!(
        matches!(ev.value(fillet).map(|v| &v.payload), Some(ValuePayload::Instances(b)) if b.len() == 3),
        "a fillet of every member: {:?}",
        ev.result(fillet)
    );
}

/// **The family read whole is no body a selection reads**, at the door
/// or at load. A family read plainly at a selection's body is read one
/// member at a time, as at every body seat: the selection makes its
/// reader a family under `k`. So a selection under a name of the
/// family's table (`Member { (k, 2), e }`) holds a name no member's own
/// table holds, and every member refuses resolving it; nothing reads the
/// family whole to find the member the name spells. The saved document
/// loads to the same per-member read and the same refusal. Red if any
/// door or evaluation reads a selection's family whole.
#[test]
fn a_family_read_whole_is_no_body_a_selection_reads() {
    let (doc, cube, k, xs) = family("select-whole");
    let edge = an_edge(&doc, cube);
    let whole_name = StableName {
        kind: EntityKind::Edge,
        node: xs,
        path: vec![RoleSeg::Member {
            at: vec![(k, 2)],
            of: edge.into(),
        }],
    };
    let (doc, fillet) = insert(
        doc,
        Node::fillet(Operand::Node(xs), len(0.1), vec![whole_name]),
    );
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    let loaded = editor_core::persist::load(&text, Tol::witness())
        .expect("and loads")
        .doc;
    for (door, doc) in [("the edit door", &doc), ("the load door", &loaded)] {
        assert_eq!(
            doc.node_reach(fillet).chain(),
            Some(&[k][..]),
            "{door}: a plain read of the family is read per member"
        );
        let ev = eval(doc);
        assert!(
            matches!(
                error_of(&ev, fillet),
                NodeErrorKind::MemberRefused { source, .. }
                    if matches!(**source, NodeErrorKind::SelectResolve { .. })
            ),
            "{door}: a member's table holds no name of the family's: {:?}",
            ev.result(fillet)
        );
    }
}

/// **A mate side's index is a written count**: the mate's member walk
/// reads the member from the document, so a side reading `xs[k mod 3]`
/// refuses at the door. Red if an evaluated index is admitted at a side.
#[test]
fn a_mate_sides_index_is_a_written_count() {
    let (doc, cube, k, xs) = family("select-side-index");
    let top = editor_core::all_faces(&eval(&doc), cube)
        .into_iter()
        .next()
        .expect("a face");
    let (doc, other) = insert(doc, fixture::stood(cube));
    let at =
        Formula::modulo(Formula::var(k, Dimension::Count), Formula::count(3)).expect("a count");
    let side = Operand::select(BodyRead::indexed(xs, vec![at]), vec![top.clone()]);
    let refused = editor_core::apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: other,
            mate: Box::new(fixture::rest(
                Operand::select(Operand::Node(other), vec![top]),
                side,
            )),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::SelectionShape {
                fault: editor_core::SelectionFault::SideIndex { k: 0 },
                ..
            })
        ),
        "an evaluated index at a mate side refuses: {refused:?}"
    );
}

/// **D10's per-copy reading, `gap(c[k], c[(k + 1) mod N])`**: a measure
/// evaluated once per member, between a vertex of member `k` (the
/// family read plainly) and the same vertex of member `(k + 1) mod 3`
/// (the selection's own index), is 2, 2 and 4. Red if the second read
/// is not the next member, or the index does not wrap.
#[test]
fn a_per_copy_selection_reads_the_next_member() {
    let (doc, cube, k, xs) = family("select-next");
    let vertex = editor_core::all_vertices(&eval(&doc), cube)
        .into_iter()
        .next()
        .expect("a vertex");
    let next = Formula::modulo(
        Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).expect("a count"),
        Formula::count(3),
    )
    .expect("a count");
    let measured = editor_core::measure(
        &doc,
        &[MeasurePrimitive::Distance {
            a: Operand::select(Operand::Node(xs), vec![vertex.clone()]),
            b: Operand::select(BodyRead::indexed(xs, vec![next]), vec![vertex]),
        }],
        Tol::witness(),
    )
    .expect("the measure inserts");
    let gap = measured.measured.measures[0];
    let doc = measured.doc;
    assert_eq!(
        doc.node_reach(gap).chain(),
        Some(&[k][..]),
        "a measure per member"
    );
    let ev = eval(&doc);
    let members = ev
        .value(gap)
        .and_then(|v| v.members.as_ref())
        .unwrap_or_else(|| panic!("a family of measures: {:?}", ev.result(gap)));
    let gaps: Vec<f64> = members
        .members
        .iter()
        .map(|(_, value)| match &value.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("a measure, not a {}", other.kind_name()),
        })
        .collect();
    assert_eq!(
        gaps,
        [2.0, 2.0, 4.0],
        "each member to the next, the last to the first"
    );
}

/// **A member that is a boolean's empty result refuses a selection of
/// it as an empty operand**: `F[k] − F[1]`, member 1 empty, and a
/// fillet of `cut[1]`. Red if the empty member resolves, or refuses as
/// anything but its emptiness.
#[test]
fn a_selection_of_an_empty_member_refuses_empty_operand() {
    let (doc, row, cut) = crate::g_family_of_booleans::cut_row(|doc, _, row| {
        (doc, BodyRead::indexed(row, vec![Formula::count(1)]))
    });
    let edge = editor_core::all_edges(&eval(&doc), row)
        .into_iter()
        .next()
        .expect("an edge");
    let (doc, fillet) = insert(doc, Node::fillet(member(cut, 1), len(0.05), vec![edge]));
    let ev = eval(&doc);
    assert!(
        matches!(error_of(&ev, fillet), NodeErrorKind::EmptyOperand { input } if *input == cut),
        "the empty member is an empty operand: {:?}",
        ev.result(fillet)
    );
}

/// A pinned leg, a row of two of it under `k`, and a top resting on
/// the row's member 1 by a plane mate: the side `pattern[1]`, read
/// through the plane pose of its cap. Answers the document, `k`, the
/// top, the side's index variable and the options reaching the parts.
fn side_on_member(
    label: &str,
) -> (
    ProfileDoc,
    VarId,
    RecipeNodeId,
    VarId,
    editor_core::EvalOptions,
) {
    use fixture::resolver::{PartStore, in_part, with_resolver};
    let leg = |label: &str| {
        let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), Tol::witness());
        let (doc, p) = fixture::on_frame(
            doc,
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        );
        insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(1.0),
            },
        )
    };
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg(&format!("{label}-leg")), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg(&format!("{label}-top")), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive(label), Tol::witness());
    let (doc, legs) = fixture::insert_pinned(doc, fixture::mated_instance(leg_ref), &o);
    let (doc, k, pattern) = fixture::linear(doc, legs, [1.0, 0.0, 0.0], 2.0, 2);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let to = fixture::flip(fixture::plane(fixture::member_head(
        pattern,
        &[1],
        in_part(legs, leg_body, editor_core::CapEnd::End),
    )));
    let mate = fixture::mate(
        fixture::plane(fixture::head(in_part(
            top,
            top_body,
            editor_core::CapEnd::Start,
        ))),
        to,
    );
    let (doc, _) = fixture::constrain_with(
        doc,
        mate,
        &[
            (
                editor_core::Coord::slide(editor_core::TargetAxis::X),
                len(0.0),
            ),
            (
                editor_core::Coord::slide(editor_core::TargetAxis::Y),
                len(0.0),
            ),
            (editor_core::Coord::SPIN, Formula::zero(Dimension::Angle)),
        ],
    );
    let index = doc
        .var_ids()
        .into_iter()
        .find_map(|id| {
            let select = doc.selection(id)?;
            select.body.is_indexed().then(|| select.body.at[0])
        })
        .expect("the side is a member read");
    (doc, k, top, index, o)
}

/// **A mate side's index named after the door is still a written
/// count**: its value is the document's, so the side's head reads the
/// member from the document, the top seats, and the document saves and
/// loads. Red if naming the count strands the side.
#[test]
fn a_side_index_named_after_the_door_still_reads_its_member() {
    let (doc, _, top, index, o) = side_on_member("select-side-named");
    let doc = doc
        .apply(
            &DocEdit::RenameVar {
                var: index.into(),
                name: Some(editor_core::VarName::new("n").expect("a name")),
            },
            Tol::witness(),
        )
        .expect("the count is named")
        .doc;
    assert_eq!(
        fixture::solve(&doc, &o, Tol::witness()).fault(top),
        None,
        "the top seats"
    );
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    editor_core::persist::load(&text, Tol::witness()).expect("and loads");
}

/// **The load door holds a side read through a pose to the side's
/// rules**, as the edit door does: a side whose index stops being a
/// written count (`SideIndex`), and one whose family gains an index
/// (`IndexRank`), each refuse at save, which asks the load door's
/// checks, naming the selection. Red if the load door gathers a
/// selection's seats from direct reads alone, which a mate side, read
/// through its plane pose, never is.
#[test]
fn the_load_door_holds_a_side_read_through_a_pose() {
    let (doc, k, _, index, _) = side_on_member("select-side-load");
    let defined = doc
        .apply(
            &DocEdit::DefineVar {
                var: editor_core::VarRef::Id(index),
                def: editor_core::VarDecl::Defined(
                    Formula::modulo(Formula::var(k, Dimension::Count), Formula::count(2))
                        .expect("a count"),
                ),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect(
            "the redefinition is admitted (work/refs/a-rank-change-leaves-readers-unsaveable.md)",
        )
        .doc;
    assert!(
        matches!(
            editor_core::persist::save(&defined, &[], Tol::witness()),
            Err(editor_core::PersistError::Snapshot(
                SnapshotError::SelectionShape {
                    fault: editor_core::SelectionFault::SideIndex { k: 0 },
                    ..
                }
            ))
        ),
        "a side's index defined as an expression refuses at the load door"
    );
    let (doc, j) = fixture::index(doc, "j", 2);
    let deeper = doc
        .apply(
            &DocEdit::DefineVar {
                var: editor_core::VarRef::Id(k),
                def: editor_core::VarDecl::Index {
                    count: Formula::count(2),
                    within: Some(Formula::var(j, Dimension::Count)),
                },
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect(
            "the redefinition is admitted (work/refs/a-rank-change-leaves-readers-unsaveable.md)",
        )
        .doc;
    assert!(
        matches!(
            editor_core::persist::save(&deeper, &[], Tol::witness()),
            Err(editor_core::PersistError::Snapshot(
                SnapshotError::SelectionShape {
                    fault: editor_core::SelectionFault::Input(editor_core::InputFault::IndexRank {
                        found: 1,
                        rank: 2,
                        ..
                    }),
                    ..
                }
            ))
        ),
        "a side whose family gained an index refuses at the load door"
    );
}
