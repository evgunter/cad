//! **A node's keys hash what each of its reads sees** (DESIGN
//! `[determinism]`: equal keys mean equal inputs): a family member that
//! reads ANOTHER member of an upstream family, or the family whole, is
//! keyed by that member, or by the family, and never by the member
//! bound at its own indices, which is what stands in its results where
//! the family stood. So an edit that moves only the member a read picks
//! re-evaluates the reader, and an evaluation over a prior is the fresh
//! evaluation of the same document.
//!
//! The fixture: a free length `p`; an index `k` of two; `tall[k]`, an
//! extrude `0.25 + scalar(k)·p` deep, so member 0 does not move with
//! `p` and member 1 does. Each row's reader is a family over `k` whose
//! member 0 reads `tall[1]`. Every row evaluates at `p = 0.25`, sets
//! `p = 0.5` by the value door, and holds the evaluation over that
//! prior to the fresh one.
//!
//! Each row names the mutant of `eval::wire::reads_seen` it is red
//! under; all but the place row are red on the keys as they were, which
//! hashed the bound member for every read.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, step};
use editor_core::{
    Bodies, BodyRead, BooleanValue, CancelToken, Dimension, DocEdit, EvalOptions, Evaluation,
    Formula, FreeValue, FreeVar, MeasurePrimitive, Node, Operand, OwnRecord, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, UnitSym, ValuePayload, VarDecl, VarId, VarName,
};
use geom_core::{Bounds, Interval, Tol};

/// A free variable `name` at `def`.
fn declare(doc: ProfileDoc, name: &'static str, def: FreeVar) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static(name),
            def: VarDecl::Free(def),
        },
    )
    .0
}

/// `label`'s document with the free length `p = 0.25`.
fn with_p(label: &str) -> ProfileDoc {
    declare(
        ProfileDoc::empty_derived(label, Tol::witness()),
        "p",
        FreeVar::Continuous {
            dim: Dimension::Length,
            value: 0.25,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: None,
        },
    )
}

/// `count` as a scalar.
fn scalar(count: Formula) -> Formula {
    Formula::count_to_scalar(count).unwrap()
}

fn var(index: VarId) -> Formula {
    Formula::var(index, Dimension::Count)
}

/// `0.25 + by·p`, a length.
fn depth(by: Formula) -> Formula {
    let p = Formula::named(VarName::from_static("p"), Dimension::Length);
    Formula::add(fixture::len(0.25), Formula::mul(by, p).unwrap()).unwrap()
}

/// An extrude of a unit square about `(cx, 0)`, `distance` deep.
fn prism(doc: ProfileDoc, cx: f64, distance: Formula) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(cx, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance,
        },
    )
}

/// The document, the index `k` of two and `tall`.
fn tall(label: &str) -> (ProfileDoc, VarId, RecipeNodeId) {
    let (doc, k) = fixture::index_of(with_p(label), "k", Formula::count(2), None);
    let (doc, tall) = prism(doc, 0.0, depth(scalar(var(k))));
    (doc, k, tall)
}

/// `(k + 1) mod 2`.
fn next(k: VarId) -> Formula {
    Formula::modulo(
        Formula::add(var(k), Formula::count(1)).unwrap(),
        Formula::count(2),
    )
    .unwrap()
}

fn evaluate<T: OwnRecord>(doc: &ProfileDoc, prior: Option<&Evaluation<T>>) -> Evaluation<T> {
    editor_core::evaluate(
        doc,
        prior,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// `doc` with the free variable `name` set to `value`.
fn set(doc: ProfileDoc, name: &str, value: FreeValue) -> ProfileDoc {
    let var = doc.var_named(name).expect("declared");
    step(
        doc,
        DocEdit::SetVarValue {
            var: var.into(),
            value,
        },
    )
    .0
}

/// `id`'s family's members' values, in index order.
fn members<T: OwnRecord>(ev: &Evaluation<T>, id: RecipeNodeId) -> Vec<&editor_core::NodeValue<T>> {
    ev.value(id)
        .unwrap_or_else(|| panic!("{id:?}: {:?}", ev.result(id)))
        .members
        .as_ref()
        .expect("a family")
        .members
        .iter()
        .map(|(_, member)| member)
        .collect()
}

/// What each member of `id`'s family holds, as bounds: a body's
/// greatest `z`, a measure's value.
fn readings<T: OwnRecord + Bounds>(ev: &Evaluation<T>, id: RecipeNodeId) -> Vec<(f64, f64)> {
    members(ev, id)
        .into_iter()
        .map(|member| {
            let body = match &member.payload {
                ValuePayload::Body(b) => fixture::in_space(b),
                ValuePayload::Boolean(BooleanValue::Body { body, .. }) => fixture::in_space(body),
                ValuePayload::Measure { value, .. } => return (value.lo(), value.hi()),
                other => panic!("a body or a measure, got {}", other.kind_name()),
            };
            body.vertex_points()
                .map(|(_, at)| (at.z.lo(), at.z.hi()))
                .fold((f64::NEG_INFINITY, f64::NEG_INFINITY), |(lo, hi), z| {
                    (lo.max(z.0), hi.max(z.1))
                })
        })
        .collect()
}

/// **The evaluation of `doc` at `p = 0.5` over its evaluation at
/// `p = 0.25` is the fresh one**, at `reader` and at `tall`; and the
/// fresh reader's member 0 moved with `p`, so the row sees a reader
/// served the prior's value. The three evaluations, prior first.
fn served_is_fresh<T: OwnRecord + Bounds>(
    doc: ProfileDoc,
    reader: RecipeNodeId,
    what: &str,
) -> [Evaluation<T>; 3] {
    let prior = evaluate::<T>(&doc, None);
    let after = set(doc, "p", FreeValue::Continuous(0.5));
    let fresh = evaluate::<T>(&after, None);
    assert_ne!(
        readings(&prior, reader)[0],
        readings(&fresh, reader)[0],
        "{what} at {}: member 0 of the reader moves with p, or the row sees nothing",
        T::NAME
    );
    let served = evaluate(&after, Some(&prior));
    assert_eq!(
        readings(&served, reader),
        readings(&fresh, reader),
        "{what} at {}: the reader over the prior is the fresh reader",
        T::NAME
    );
    for (served, fresh) in members(&served, reader)
        .into_iter()
        .zip(members(&fresh, reader))
    {
        assert_eq!(
            (served.content_key, served.naming_key),
            (fresh.content_key, fresh.naming_key),
            "{what} at {}: a member's keys over the prior are its fresh keys",
            T::NAME
        );
    }
    [prior, fresh, served]
}

/// `reader[k] = union([tall[(k + 1) mod 2]])`.
fn union_of_the_next(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let (doc, k, tall) = tall(label);
    insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![BodyRead::indexed(tall, vec![next(k)])]),
        },
    )
}

/// **A union's indexed member seat** (`wire_combine`): `reader[0]`
/// unions `tall[1]`, which `p` moves, and is rebuilt; `reader[1]` reads
/// `tall[0]`, which did not move, and is served. At `f64`: heights
/// `[0.75, 0.25]` where the prior held `[0.5, 0.25]`. Red if a member
/// read is keyed as the value that stands (`Seen::Member` read as
/// `Seen::Stands`): `reader[0]` is then served the prior's 0.5.
#[test]
fn a_members_indexed_read_of_another_member_is_rebuilt_when_that_member_moves() {
    let (doc, reader) = union_of_the_next("member-read-key-union");
    let [prior, _, served] = served_is_fresh::<f64>(doc, reader, "union([tall[next]])");
    assert_eq!(
        readings(&served, reader),
        [(0.75, 0.75), (0.25, 0.25)],
        "the heights at p = 0.5"
    );
    assert_eq!(
        members(&served, reader)[1].content_key,
        members(&prior, reader)[1].content_key,
        "reader[1] reads tall[0], which p does not move: its key holds"
    );
    // The profile, tall[0] and reader[1] are served: the family nodes
    // count as recomputed, each holding a rebuilt member.
    assert_eq!((served.reused, served.recomputed), (1, 2));
}

/// The same at the certified scalar, where the memo is the same lookup.
#[test]
fn a_members_indexed_read_of_another_member_is_rebuilt_at_interval() {
    let (doc, reader) = union_of_the_next("member-read-key-union-interval");
    served_is_fresh::<Interval>(doc, reader, "union([tall[next]])");
}

/// **An indexed read at a one-body seat** (`reads_projected`):
/// `reader[k]`, a copy of `tall[(k + 1) mod 2]` placed where it stands.
/// Red under the same mutant as the union's row, through the seat's own
/// projection.
#[test]
fn a_members_indexed_read_at_a_body_seat_is_rebuilt_when_that_member_moves() {
    let (doc, k, tall) = tall("member-read-key-place");
    let target = fixture::space(&doc, tall);
    let still = editor_core::Placed::of(&doc, Vec::new(), Vec::new());
    let (doc, reader) = insert(
        doc,
        Node::place(
            BodyRead::indexed(tall, vec![next(k)]),
            Operand::Var(target),
            still.bundle(),
        ),
    );
    served_is_fresh::<f64>(doc, reader, "place(tall[next])");
}

/// A name of one member of `tall`, in the member's own spelling, whose
/// vertex stands on the member's top.
fn a_top_vertex(doc: &ProfileDoc, tall: RecipeNodeId) -> StableName {
    let ev = evaluate::<f64>(doc, None);
    let family = ev.value(tall).expect("tall evaluates");
    let ValuePayload::Instances(bodies) = &family.payload else {
        panic!("a family of bodies");
    };
    let body = fixture::in_space(bodies[0].as_ref().expect("member 0 is a body"));
    let table = ev
        .table(doc.output(tall, 0).expect("tall's read"))
        .expect("tall's table");
    table
        .iter()
        .find_map(|(name, entry)| {
            let [RoleSeg::Member { of, .. }] = name.path.as_slice() else {
                return None;
            };
            let editor_core::Entry::Unique(editor_core::EntityRef {
                body: 0,
                key: editor_core::EntityKey::Vertex(v),
            }) = entry
            else {
                return None;
            };
            (fixture::point(&body, *v).z > 0.0).then(|| (**of).clone())
        })
        .expect("member 0 names a vertex on its top")
}

/// **A selection of another member** (`selection_member`): a measure
/// per member, from a top vertex of `tall[k]` to the same vertex of
/// `tall[(k + 1) mod 2]`, which is `p` at both members. Red if a
/// selection's member read is keyed as the value that stands:
/// `reader[0]`, whose plain read `tall[0]` did not move, is served the
/// prior's 0.25.
#[test]
fn a_members_selection_of_another_member_is_read_again_when_that_member_moves() {
    let (doc, k, tall) = tall("member-read-key-select");
    let vertex = a_top_vertex(&doc, tall);
    let measured = editor_core::measure(
        &doc,
        &[MeasurePrimitive::Distance {
            a: Operand::select(Operand::Node(tall), vec![vertex.clone()]),
            b: Operand::select(BodyRead::indexed(tall, vec![next(k)]), vec![vertex]),
        }],
        Tol::witness(),
    )
    .expect("the measure inserts");
    let reader = measured.measured.measures[0];
    let [_, _, served] = served_is_fresh::<f64>(measured.doc, reader, "gap(tall[k], tall[next])");
    assert_eq!(readings(&served, reader), [(0.5, 0.5), (0.5, 0.5)]);
}

/// **A family read whole from inside a member**
/// (`repetition::takes_whole`): `reader[k] = assemble([tall,
/// tall[k]])`, a family over `k` by its second seat's index, whose
/// first seat reads every member of `tall`. Its second seat picks the
/// bound member, so the member read's key is the bound member's either
/// way. Red if a seat that takes a family whole is keyed as the value
/// that stands (`Seen::Whole` read as `Seen::Stands`): `reader[0]`
/// holds `tall[1]` and is served the prior's.
#[test]
fn a_member_reading_a_family_whole_is_rebuilt_when_any_member_moves() {
    let (doc, k, tall) = tall("member-read-key-whole");
    let (doc, reader) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(tall),
                BodyRead::indexed(tall, vec![var(k)]),
            ]),
        },
    );
    served_is_fresh::<f64>(doc, reader, "assemble([tall, tall[k]])");
}

/// **A member of a nested family**: `j` of two within `k`; `tall[k,
/// j]` an extrude `0.25 + scalar(k)·scalar(j)·p` deep, so `p` moves
/// member `(1, 1)` alone; `reader[k, j] = union([tall[(k + 1) mod 2,
/// j]])`, whose member `(0, 1)` reads it. Red if a member read of two
/// indices is keyed as the value that stands.
#[test]
fn a_member_of_a_nested_family_is_rebuilt_when_the_member_it_reads_moves() {
    let (doc, k) = fixture::index_of(
        with_p("member-read-key-nested"),
        "k",
        Formula::count(2),
        None,
    );
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(k));
    let by = Formula::mul(scalar(var(k)), scalar(var(j))).unwrap();
    let (doc, tall) = prism(doc, 0.0, depth(by));
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![BodyRead::indexed(tall, vec![next(k), var(j)])]),
        },
    );
    let prior = evaluate::<f64>(&doc, None);
    let after = set(doc, "p", FreeValue::Continuous(0.5));
    let fresh = evaluate::<f64>(&after, None);
    let served = evaluate(&after, Some(&prior));
    // Members in index order: (0, 0), (0, 1), (1, 0), (1, 1).
    assert_eq!(
        readings(&fresh, reader),
        [(0.25, 0.25), (0.75, 0.75), (0.25, 0.25), (0.25, 0.25)],
        "fresh: member (0, 1) reads tall[1, 1]"
    );
    assert_eq!(
        readings(&served, reader),
        readings(&fresh, reader),
        "the reader over the prior is the fresh reader"
    );
}
