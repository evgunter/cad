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
//! Each row names the mutant it is red under; all but the place row
//! are red on the keys as they were, which hashed the bound member for
//! every read and no read's variable.
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
    let doc = fixture::yielded(doc, tall);
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
            members: Bodies::spelled(vec![BodyRead::indexed(tall, vec![next(k)])]),
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
            a: Operand::select(BodyRead::indexed(tall, vec![var(k)]), vec![vertex.clone()]),
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
/// (a yield spliced in, `…tall`): `reader[k] = assemble([…tall,
/// tall[k]])`, a family over `k` by its second seat's index, whose
/// first seat reads every member of `tall`. Its second seat picks the
/// bound member, so the member read's key is the bound member's either
/// way. Red if a read of a yield spliced in whole is keyed as the value
/// that stands (`Seen::Whole` read as `Seen::Stands`): `reader[0]`
/// holds `tall[1]` and is served the prior's.
#[test]
fn a_member_reading_a_family_whole_is_rebuilt_when_any_member_moves() {
    let (doc, k, tall) = tall("member-read-key-whole");
    let (doc, reader) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                editor_core::Spell::All(BodyRead::plain(tall)),
                editor_core::Spell::One(BodyRead::indexed(tall, vec![var(k)])),
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
    let doc = fixture::yielded(doc, tall);
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![BodyRead::indexed(tall, vec![next(k), var(j)])]),
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

/// **A source of another member** (`member_read`'s source arm):
/// `reader[k]`, a copy of the source `tall[(k + 1) mod 2]` placed where
/// it stands, the read at its seat plain and the member the source's
/// own. Red if a source's member read is keyed as the value that
/// stands.
#[test]
fn a_members_source_of_another_member_is_rebuilt_when_that_member_moves() {
    let (doc, k, tall) = tall("member-read-key-source");
    let target = fixture::space(&doc, tall);
    let still = editor_core::Placed::of(&doc, Vec::new(), Vec::new());
    let (doc, reader) = insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::source(BodyRead::indexed(tall, vec![next(k)]))),
            Operand::Var(target),
            still.bundle(),
        ),
    );
    served_is_fresh::<f64>(doc, reader, "place(source(tall[next]))");
}

/// The document for the place rows: `j` of `n` within `k` of two,
/// `tall[k, j]` an extrude `0.25 + (scalar(k) + scalar(j))·p` deep. At
/// `n = 1` member `(1, 0)` stands at place 1 among the family's bodies,
/// at `n = 2` at place 2, under its own key at both.
fn jagged(label: &str) -> (ProfileDoc, VarId, VarId, RecipeNodeId) {
    let doc = declare(with_p(label), "n", FreeVar::Count { value: 1 });
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(2), None);
    let n = Formula::named(VarName::from_static("n"), Dimension::Count);
    let (doc, j) = fixture::index_of(doc, "j", n, Some(k));
    let by = Formula::add(scalar(var(k)), scalar(var(j))).unwrap();
    let (doc, tall) = prism(doc, 0.0, depth(by));
    let doc = fixture::yielded(doc, tall);
    (doc, k, j, tall)
}

/// Member `(1, 0)` of `tall`: its place among the family's bodies and
/// its content key.
fn member_one(ev: &Evaluation<f64>, tall: RecipeNodeId) -> (usize, editor_core::ContentKey) {
    let family = ev
        .value(tall)
        .expect("tall")
        .members
        .as_ref()
        .expect("a family");
    let place = family
        .members
        .iter()
        .position(|(at, _)| at == &[1, 0])
        .expect("member (1, 0)");
    (place, family.members[place].1.content_key)
}

/// `doc` at `n = 1` and at `n = 2`, each fresh, with member `(1, 0)` of
/// `tall` a place on in the second under the key it had in the first.
fn at_two_places(doc: ProfileDoc, tall: RecipeNodeId) -> [Evaluation<f64>; 2] {
    let one = evaluate::<f64>(&doc, None);
    let two = evaluate::<f64>(&set(doc, "n", FreeValue::Count(2)), None);
    assert_eq!(
        (member_one(&one, tall).0, member_one(&two, tall).0),
        (1, 2),
        "member (1, 0) stands a place on once member (0, 1) stands"
    );
    assert_eq!(
        member_one(&one, tall).1,
        member_one(&two, tall).1,
        "under its own key"
    );
    [one, two]
}

/// **A member read's key holds while the member's place moves, where
/// its reader reads no place**: `reader[k, j] = union([tall[k, j]])`,
/// each member reading its own; the inner count goes 1 to 2, which
/// moves member `(1, 0)` of `tall` a place on and nothing of it.
/// `reader[1, 0]`'s key is the same at both. Red if a read that picks a
/// member writes the member's place into a key whose op reads none:
/// every member after a count that moved is then rebuilt.
#[test]
fn a_member_reads_key_holds_while_the_members_place_moves() {
    let (doc, k, j, tall) = jagged("member-read-key-place-holds");
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![BodyRead::indexed(tall, vec![var(k), var(j)])]),
        },
    );
    let [one, two] = at_two_places(doc, tall);
    let key = |ev: &Evaluation<f64>| {
        let family = ev.value(reader).expect("the reader builds");
        let members = family.members.as_ref().expect("a family");
        members.at(&[1, 0]).expect("member (1, 0)").content_key
    };
    assert_eq!(key(&one), key(&two), "reader[1, 0]'s key holds");
}

/// **A clearance's key moves with the place of the member a seat
/// picks**: a clearance reports each seat by its member's place among
/// its family's bodies, and its verifier reads the rebuild's body at
/// that place (`wire::member_place`). `clearance(tall[1, 0].f,
/// tall[0, 0].f)`, no family; the inner count goes 1 to 2. Red if a
/// clearance's key leaves the place out: its key is then the same at
/// both, and it is served a report that names another body.
#[test]
fn a_clearances_key_moves_with_the_place_of_the_member_its_seat_picks() {
    let (doc, _, _, tall) = jagged("member-read-key-place-moves");
    let face = {
        let ev = evaluate::<f64>(&doc, None);
        let table = ev
            .table(doc.output(tall, 0).expect("tall's read"))
            .expect("tall's table");
        table
            .iter()
            .find_map(|(name, entry)| {
                let [RoleSeg::Member { of, .. }] = name.path.as_slice() else {
                    return None;
                };
                matches!(
                    entry,
                    editor_core::Entry::Unique(editor_core::EntityRef {
                        body: 0,
                        key: editor_core::EntityKey::Face(_),
                    })
                )
                .then(|| (**of).clone())
            })
            .expect("member (0, 0) names a face")
    };
    let seat = |k: i64| {
        Operand::select(
            BodyRead::indexed(tall, vec![Formula::count(k), Formula::count(0)]),
            vec![face.clone()],
        )
    };
    let (doc, reader) = insert(
        doc,
        Node::clearance(
            seat(1),
            seat(0),
            editor_core::ClearanceBounds::AtLeast(fixture::len(0.01)),
        ),
    );
    let [one, two] = at_two_places(doc, tall);
    let key = |ev: &Evaluation<f64>| {
        ev.value(reader)
            .unwrap_or_else(|| panic!("the clearance evaluates: {:?}", ev.result(reader)))
            .content_key
    };
    assert_ne!(key(&one), key(&two), "the clearance's key moves");
}

/// **A read's variable is an input of its reader's names**: a body's
/// own read and a source of it reach one output, so re-pointing a seat
/// from the one to the other moves no upstream key, and the reader's
/// names and carried records cite the read they came through
/// (`RoleSeg::From`). `reader = assemble([body])`, then the same seat
/// reading `spare`, a plain source of `body`: the evaluation over the
/// prior names as the fresh one does. Red if the naming key leaves out
/// the variable a read comes through: both keys are then the prior's,
/// and the reader is served the names keyed by `body`'s own read.
#[test]
fn a_seat_re_pointed_to_a_source_of_the_same_body_is_named_through_it() {
    let doc = ProfileDoc::empty_derived("member-read-key-read-var", Tol::witness());
    let (doc, body) = prism(doc, 0.0, fixture::len(1.0));
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("spare"),
            def: VarDecl::Source(BodyRead::plain(Operand::Node(body))),
        },
    );
    let spare = doc.var_named("spare").expect("declared");
    let (doc, reader) = insert(
        doc,
        Node::Assemble {
            members: Bodies::spelled(vec![BodyRead::plain(Operand::Node(body))]),
        },
    );
    let prior = evaluate::<f64>(&doc, None);
    let (after, _) = step(
        doc,
        DocEdit::SetMembers {
            node: reader,
            members: Bodies::spelled(vec![BodyRead::plain(Operand::Var(spare))]),
        },
    );
    let names = |ev: &Evaluation<f64>| -> Vec<StableName> {
        let read = after.output(reader, 0).expect("the reader's read");
        let table = ev.table(read).expect("the reader's table");
        let mut names: Vec<StableName> = table.iter().map(|(name, _)| name.clone()).collect();
        names.sort();
        names
    };
    let fresh = evaluate::<f64>(&after, None);
    assert_ne!(
        names(&prior),
        names(&fresh),
        "the two reads name apart, or the row sees nothing"
    );
    let served = evaluate(&after, Some(&prior));
    assert_eq!(
        names(&served),
        names(&fresh),
        "over the prior, the reader is named through the read it now holds"
    );
    assert_eq!(
        served.value(reader).map(|v| v.content_key),
        prior.value(reader).map(|v| v.content_key),
        "the content key holds: the body read is the same"
    );
}

/// **Every read past the value that stands is one of ten, in one
/// file**: `NodeValue::whole` is the one door a reader takes from the
/// member bound in its results to the family, and `wire::seen_reads`
/// says for each read whether its op goes through it. The two are two
/// walks of a node's reads that no type holds equal, so a new caller of
/// `whole()` is a read the keys may not see: it is counted here until
/// someone says which of `seen_reads`' arms sees it.
///
/// The ten, in `eval/wire.rs`: `member_place`, `selection_table`,
/// `reads_projected` twice (the family, and its halves where the read
/// is a half of a split of each member) and `project_selection` (a
/// member read, seen as that member); `wire_combine` twice and
/// `wire_assemble` (a yield, or a member read already projected); and
/// `Seen::of` twice (the keys' own).
///
/// Blind spot: a read of `NodeValue::family` by the field, which
/// nothing outside `whole()` and the overlay makes today.
#[test]
fn every_read_past_the_value_that_stands_is_counted() {
    let root = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR"));
    let src = root.join("crates/editor-core/src");
    let mut found: Vec<(String, usize)> = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("readable source file");
        let hits = test_utils::source::code_only(&text)
            .matches(".whole()")
            .count();
        if hits > 0 {
            let rel = path.strip_prefix(&src).expect("under src");
            found.push((rel.to_string_lossy().replace('\\', "/"), hits));
        }
    }
    assert_eq!(
        found,
        [("eval/wire.rs".to_owned(), 10)],
        "a reader of a family past the member bound in its results was added or moved: say \
         which arm of `wire::seen_reads` sees its read, then count it here"
    );
}
