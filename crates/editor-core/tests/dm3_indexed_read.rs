//! **One member of a family is read as `xs[i]`** (REFERENCES DM3): a
//! read of the family `xs` at one `Count` per index, at every body seat
//! ([`BodyRead`]). The member keeps its `Instance { i, of }` names and
//! an operation reading it keys them by `xs` (DM4), so `Union(xs)` and
//! `Union([xs[0], xs[1], xs[2]])` name and build alike. An index out of
//! range leaves the reader unresolved and typed at evaluation; the index
//! is a structural slot of its reader ([`SlotId::Index`]), written by the
//! slot door and read as an input.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::eval6_placers_over_instances::{cube_doc, linear};
use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, EditError, Evaluation, Formula, Node, NodeError, NodeErrorKind,
    NodeResult, OperandSlot, ProfileDoc, RecipeNodeId, RoleSeg, SlotId, StableName, VarName,
};
use fixture::{insert, insert_refused, len, run, step, table};
use geom_core::Tol;

/// A unit cube and a linear pattern of three of it along x at spacing
/// 2: instance `i` occupies `[2i, 2i + 1]` in x.
fn family(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, cube) = cube_doc(label);
    let (doc, xs) = insert(doc, linear(cube, [1.0, 0.0, 0.0], 2.0, 3));
    (doc, cube, xs)
}

/// `xs[i]`.
fn member(xs: RecipeNodeId, i: i64) -> BodyRead<Formula> {
    BodyRead::indexed(xs, vec![Formula::count(i)])
}

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    run(doc, &editor_core::EvalOptions::default())
}

fn error_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NodeErrorKind {
    match ev.nodes.get(&id) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => kind,
        other => panic!("node {id} must fail typed, got {other:?}"),
    }
}

/// The least x any vertex of `id`'s body stands at.
fn least_x(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    let body = crate::corpus::body_of(ev, id);
    body.vertices()
        .map(|(v, _)| fixture::point(body, v).x)
        .fold(f64::INFINITY, f64::min)
}

fn volume(ev: &Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(crate::corpus::body_of(ev, id), Tol::witness())
        .expect("mass properties")
        .volume
}

/// Every name in `at`'s table, sorted.
fn names(ev: &Evaluation<f64>, at: RecipeNodeId) -> Vec<StableName> {
    let mut n: Vec<StableName> = table(ev, at).iter().map(|(n, _)| n.clone()).collect();
    n.sort();
    n
}

/// The instance index a name carried in from the family `xs` (read as
/// `read`) says, `None` for a name not so carried.
fn carried_instance(name: &StableName, read: editor_core::VarId) -> Option<u32> {
    match name.path.first()? {
        RoleSeg::From { read: r, of } if *r == read => match of.path.first()? {
            RoleSeg::Instance { i, .. } => Some(*i),
            _ => None,
        },
        _ => None,
    }
}

#[test]
fn a_union_of_a_family_and_of_its_members_spelled_name_and_build_alike() {
    let (doc, _, xs) = family("dm3-union-alike");
    let (doc, whole) = insert(
        doc,
        Node::Union {
            members: Bodies::Family(xs.into()),
            declare: Vec::new(),
        },
    );
    let (doc, spelled) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![member(xs, 0), member(xs, 1), member(xs, 2)]),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    let read = fixture::out(&doc, xs);
    let mut renamed: Vec<StableName> = names(&ev, whole)
        .iter()
        .map(|n| fixture::renoded(n, whole, spelled))
        .collect();
    renamed.sort();
    assert_eq!(
        renamed,
        names(&ev, spelled),
        "the family read whole and its members spelled publish one table"
    );
    let instances: std::collections::BTreeSet<u32> = names(&ev, spelled)
        .iter()
        .filter_map(|n| carried_instance(n, read))
        .collect();
    assert_eq!(
        instances,
        [0, 1, 2].into_iter().collect(),
        "every member's names are keyed by the family read, the index said once in `Instance`"
    );
    assert!(
        (volume(&ev, whole) - 3.0).abs() < 1e-9 && (volume(&ev, spelled) - 3.0).abs() < 1e-9,
        "both build the three cubes: {} and {}",
        volume(&ev, whole),
        volume(&ev, spelled)
    );
}

#[test]
fn an_indexed_read_at_a_body_seat_reads_that_member() {
    let (doc, _, xs) = family("dm3-member-at-seat");
    let read = fixture::out(&doc, xs);
    let ev = eval(&doc);
    let edge = names(&ev, xs)
        .into_iter()
        .find(|n| {
            n.kind == editor_core::EntityKind::Edge
                && matches!(n.path.first(), Some(RoleSeg::Instance { i: 1, .. }))
        })
        .expect("instance 1 has edges");
    let (doc, blend) = insert(doc, Node::fillet(member(xs, 1), len(0.1), vec![edge]));
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: member(xs, 1),
            tool: member(xs, 0),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    assert!(
        (least_x(&ev, blend) - 2.0).abs() < 1e-12,
        "the fillet's target is instance 1: least x {}",
        least_x(&ev, blend)
    );
    assert!(volume(&ev, blend) < 1.0, "the fillet blended an edge of it");
    assert!(
        (least_x(&ev, cut) - 2.0).abs() < 1e-12 && (volume(&ev, cut) - 1.0).abs() < 1e-9,
        "the subtract cuts instance 1 by the disjoint instance 0: least x {}, volume {}",
        least_x(&ev, cut),
        volume(&ev, cut)
    );
    assert!(
        names(&ev, cut)
            .iter()
            .any(|n| carried_instance(n, read) == Some(1)),
        "the cut's names are keyed by the family read, carrying instance 1's"
    );
}

#[test]
fn an_index_out_of_range_refuses_typed_and_an_index_edit_restores_it() {
    let (doc, _, xs) = family("dm3-out-of-range");
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: member(xs, 5),
            tool: member(xs, 0),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    assert!(
        matches!(
            error_of(&ev, cut),
            NodeErrorKind::InstanceOutOfRange { input, index: 5, count: 3 } if *input == xs
        ),
        "{:?}",
        error_of(&ev, cut)
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetStructuralParam {
            node: cut,
            slot: SlotId::Index {
                seat: OperandSlot::From,
                k: 0,
            },
            expr: Formula::count(2),
            fresh: Vec::new(),
        },
    );
    let ev = eval(&doc);
    assert!(
        (least_x(&ev, cut) - 4.0).abs() < 1e-12,
        "the index edited to 2 reads instance 2: least x {}",
        least_x(&ev, cut)
    );
}

#[test]
fn the_index_variable_is_an_input_of_its_reader() {
    let (doc, _, xs) = family("dm3-index-upstream");
    let k = VarName::from_static("k");
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: k.clone(),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: 0 }),
        },
    );
    let (doc, one) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![BodyRead::indexed(
                xs,
                vec![Formula::named(k, editor_core::Dimension::Count)],
            )]),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    assert!(
        (least_x(&ev, one) - 0.0).abs() < 1e-12,
        "k = 0 reads instance 0"
    );
    let var = doc.var_named("k").expect("declared");
    let (doc, _) = step(
        doc,
        DocEdit::SetVarValue {
            var: var.into(),
            value: editor_core::FreeValue::Count(2),
        },
    );
    let ev = eval(&doc);
    assert!(
        (least_x(&ev, one) - 4.0).abs() < 1e-12,
        "k = 2 recomputes the reader at instance 2: least x {}",
        least_x(&ev, one)
    );
}

/// The committed `die_tool` document (a subtract, a world placement and
/// the rest, every read plain) loads and saves to its own bytes: a plain
/// read is serialized bare. The snapshot's one `"epsilon"` line follows
/// the swept tolerance and is left out of the comparison.
#[test]
fn a_document_of_plain_reads_saves_to_its_own_bytes() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/die_tool.pncad");
    let text = std::fs::read_to_string(path).expect("the fixture reads");
    // The file records the ε it was blessed at; the swept rows load it
    // at their own, so its one ε line is re-stamped to the process's.
    let is_epsilon = |line: &str| line.trim_start().starts_with("\"epsilon\":");
    let probe = editor_core::persist::save(
        &editor_core::ProfileDoc::empty_derived("dm3-epsilon-probe", Tol::witness()),
        &[],
        Tol::witness(),
    )
    .expect("an empty document saves");
    let wanted = probe
        .lines()
        .find(|l| is_epsilon(l))
        .expect("a save records its ε");
    let restamped: String = text
        .lines()
        .map(|l| if is_epsilon(l) { wanted } else { l })
        .map(|l| format!("{l}\n"))
        .collect();
    let loaded = editor_core::persist::load(&restamped, Tol::witness()).expect("the fixture loads");
    let saved = editor_core::persist::save(&loaded.snapshot, &loaded.edits, Tol::witness())
        .expect("the document saves");
    let sans_epsilon = |t: &str| -> Vec<String> {
        t.lines()
            .filter(|l| !l.contains("\"epsilon\""))
            .map(str::to_owned)
            .collect()
    };
    assert_eq!(sans_epsilon(&saved), sans_epsilon(&text));
}

#[test]
fn an_indexed_read_round_trips_persistence() {
    let (doc, _, xs) = family("dm3-round-trip");
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: member(xs, 2),
            tool: member(xs, 0),
            declare: Vec::new(),
        },
    );
    let text = editor_core::persist::save(&doc, &[], Tol::witness()).expect("the document saves");
    assert!(
        text.contains("\"at\""),
        "an indexed read is written with its indices"
    );
    let loaded = editor_core::persist::load(&text, Tol::witness()).expect("the document loads");
    assert_eq!(
        loaded.doc.node(cut),
        doc.node(cut),
        "the node loads as saved"
    );
    let again =
        editor_core::persist::save(&loaded.doc, &[], Tol::witness()).expect("it saves again");
    assert_eq!(again, text, "a load and a save are the identity");
    let ev = eval(&loaded.doc);
    assert!((least_x(&ev, cut) - 4.0).abs() < 1e-12);
}

#[test]
fn the_doors_refuse_an_index_where_no_family_is_read() {
    let (doc, cube, xs) = family("dm3-door");
    // An index at a seat that is not a body seat's: no such slot.
    match editor_core::apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: cube,
            slot: SlotId::Index {
                seat: OperandSlot::Profile,
                k: 0,
            },
            expr: Formula::count(0),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::UnknownSlot { slot, .. }) => assert_eq!(
            slot,
            SlotId::Index {
                seat: OperandSlot::Profile,
                k: 0
            }
        ),
        other => panic!("an index at an extrude's profile has no slot, got {other:?}"),
    }
    // An index that is not a count.
    let wrong = BodyRead::indexed(xs, vec![len(1.0)]);
    assert!(
        matches!(
            insert_refused(&doc, Node::fillet(wrong, len(0.1), Vec::new())),
            EditError::SlotDimensionMismatch {
                slot: SlotId::Index {
                    seat: OperandSlot::Target,
                    k: 0
                },
                ..
            }
        ),
        "an index is a count"
    );
    // A body indexed: the read is not a family.
    assert!(
        matches!(
            insert_refused(&doc, Node::fillet(member(cube, 0), len(0.1), Vec::new())),
            EditError::SlotVarKind {
                slot: SlotId::Operand(OperandSlot::Target),
                found: editor_core::VarKind::Body,
                ..
            }
        ),
        "only a family is indexed"
    );
    // A family read whole at a body seat, as before.
    assert!(
        matches!(
            insert_refused(&doc, Node::fillet(xs, len(0.1), Vec::new())),
            EditError::SlotVarKind {
                found: editor_core::VarKind::Bodies,
                ..
            }
        ),
        "a family read whole is not a body"
    );
    // A family argument carries no index.
    assert!(
        matches!(
            insert_refused(
                &doc,
                Node::Union {
                    members: Bodies::Family(member(xs, 0)),
                    declare: Vec::new(),
                },
            ),
            EditError::IndexedRead {
                fault: editor_core::InputFault::IndexedFamily,
                ..
            }
        ),
        "a family argument reads every member"
    );
    // A family is indexed by one count.
    let two = BodyRead::indexed(xs, vec![Formula::count(0), Formula::count(1)]);
    assert!(
        matches!(
            insert_refused(
                &doc,
                Node::Union {
                    members: Bodies::Spelled(vec![two]),
                    declare: Vec::new(),
                },
            ),
            EditError::IndexedRead {
                fault: editor_core::InputFault::IndexRank { found: 2, .. },
                ..
            }
        ),
        "a pattern's family has one index"
    );
}

#[test]
fn set_members_writes_indexed_members() {
    let (doc, _, xs) = family("dm3-set-members");
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![member(xs, 0), member(xs, 1), member(xs, 2)]),
            declare: Vec::new(),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: union,
            members: Bodies::Spelled(vec![member(xs, 0), member(xs, 2)]),
        },
    );
    let ev = eval(&doc);
    let read = fixture::out(&doc, xs);
    let instances: std::collections::BTreeSet<u32> = names(&ev, union)
        .iter()
        .filter_map(|n| carried_instance(n, read))
        .collect();
    assert_eq!(
        instances,
        [0, 2].into_iter().collect(),
        "the dropped member's names leave and the others keep theirs"
    );
    assert!((volume(&ev, union) - 2.0).abs() < 1e-9);
}

/// **A subtract of two members of one family builds as the same pair
/// read through two variables** (review r2, M1). Both seats read `xs`,
/// so their names share one read; which seat an edge descends from is
/// the operand it was taken from, never its name's read. Two unit cubes
/// along `(1, 1, 1)` at spacing 0.5 overlap with no coplanar face, so
/// the cut mints seam vertices whose parentage reads both seats.
///
/// Red if the pair emitter sides a carried edge by its read: every edge
/// lands on one side and the seam vertex refuses `SeamVertexParentage`.
#[test]
fn a_subtract_of_two_overlapping_members_of_one_family_builds_as_two_parts_do() {
    use crate::eval6_placers_over_instances::part;
    let (doc, cube) = cube_doc("dm3-overlap-cut");
    let (doc, xs) = insert(doc, linear(cube, [1.0, 1.0, 1.0], 0.5, 2));
    let (doc, one) = insert(doc, part(xs, 1));
    let (doc, zero) = insert(doc, part(xs, 0));
    let (doc, by_parts) = insert(
        doc,
        Node::Subtract {
            from: one.into(),
            tool: zero.into(),
            declare: Vec::new(),
        },
    );
    let (doc, by_index) = insert(
        doc,
        Node::Subtract {
            from: member(xs, 1),
            tool: member(xs, 0),
            declare: Vec::new(),
        },
    );
    let (doc, reversed) = insert(
        doc,
        Node::Subtract {
            from: member(xs, 0),
            tool: member(xs, 1),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    let want = volume(&ev, by_parts);
    for (cut, what) in [(by_index, "xs[1] − xs[0]"), (reversed, "xs[0] − xs[1]")] {
        assert!(
            (volume(&ev, cut) - want).abs() < 1e-9,
            "{what} builds the parts' volume {want}: {}",
            volume(&ev, cut)
        );
        assert_eq!(
            names(&ev, cut).len(),
            names(&ev, by_parts).len(),
            "{what} names as many entities as the cut of the two parts"
        );
    }
}

/// `name` with every occurrence of the node `from` — at any depth, in a
/// seam's or a crossing's citations as in its head — read as `to`.
fn renoded_deep(name: &StableName, from: RecipeNodeId, to: RecipeNodeId) -> StableName {
    let text = serde_json::to_string(name).expect("a name serializes");
    let (from, to) = (
        serde_json::to_string(&from).expect("an id serializes"),
        serde_json::to_string(&to).expect("an id serializes"),
    );
    serde_json::from_str(&text.replace(&from, &to)).expect("the renamed name parses")
}

/// **An overlapping family names alike read whole and spelled** (review
/// r1 of PR 4527, S4). Three unit cubes along `(1, 1, 1)` at spacing 0.5
/// overlap, so the union mints seams and crossings that cite the union
/// itself; renamed at every depth, the family read whole and its members
/// spelled publish one table and build one body.
///
/// Red if the spelled members are keyed or folded otherwise than the
/// family's instances, which a disjoint family cannot show.
#[test]
fn an_overlapping_family_names_alike_read_whole_and_spelled() {
    let (doc, cube) = cube_doc("dm3-overlap-alike");
    let (doc, xs) = insert(doc, linear(cube, [1.0, 1.0, 1.0], 0.5, 3));
    let (doc, whole) = insert(
        doc,
        Node::Union {
            members: Bodies::Family(xs.into()),
            declare: Vec::new(),
        },
    );
    let (doc, spelled) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![member(xs, 0), member(xs, 1), member(xs, 2)]),
            declare: Vec::new(),
        },
    );
    let ev = eval(&doc);
    let mut renamed: Vec<StableName> = names(&ev, whole)
        .iter()
        .map(|n| renoded_deep(n, whole, spelled))
        .collect();
    renamed.sort();
    assert_eq!(
        renamed,
        names(&ev, spelled),
        "the overlapping family read whole and spelled publish one table"
    );
    assert!(
        (volume(&ev, whole) - volume(&ev, spelled)).abs() < 1e-12,
        "and build one body: {} and {}",
        volume(&ev, whole),
        volume(&ev, spelled)
    );
}
