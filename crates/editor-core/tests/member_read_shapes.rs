//! **An evaluation over a prior is the fresh evaluation, across the
//! shapes a read of a family takes** (`eval::wire::seen_reads`): each
//! document is evaluated, edited, and evaluated again over the first
//! evaluation, and the served evaluation is held to the fresh one on
//! every node's whole result. `member_read_key.rs` holds one row per
//! way a read sees its upstream, each with its mutant; these are the
//! shapes those rows do not spell: two reads of one family in one node,
//! a clamped index, an index out of range and back, a count that moves
//! under its readers, readers across ranks, poses of selections of
//! other members, a rename, both schedules. A pass through the review
//! of the member-read keys built them as probes, and nine were red on
//! the keys as they were.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, step};
use editor_core::{
    Bodies, BodyRead, CancelToken, Dimension, DocEdit, EvalOptions, Evaluation, Formula, FreeValue,
    FreeVar, MeasurePrimitive, Node, NodeResult, Operand, OwnRecord, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, UnitSym, ValuePayload, VarDecl, VarId, VarName,
};
use geom_core::{Interval, Tol};

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

fn scalar(count: Formula) -> Formula {
    Formula::count_to_scalar(count).unwrap()
}

fn var(index: VarId) -> Formula {
    Formula::var(index, Dimension::Count)
}

fn named_count(name: &'static str) -> Formula {
    Formula::named(VarName::from_static(name), Dimension::Count)
}

fn depth(by: Formula) -> Formula {
    let p = Formula::named(VarName::from_static("p"), Dimension::Length);
    Formula::add(fixture::len(0.25), Formula::mul(by, p).unwrap()).unwrap()
}

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

fn tall_of(doc: ProfileDoc, count: Formula) -> (ProfileDoc, VarId, RecipeNodeId) {
    let (doc, k) = fixture::index_of(doc, "k", count, None);
    let (doc, tall) = prism(doc, 0.0, depth(scalar(var(k))));
    let doc = fixture::yielded(doc, tall);
    (doc, k, tall)
}

fn tall(label: &str) -> (ProfileDoc, VarId, RecipeNodeId) {
    tall_of(with_p(label), Formula::count(2))
}

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

/// Every node's whole result, as its `Debug`.
fn whole<T: OwnRecord>(ev: &Evaluation<T>) -> Vec<(RecipeNodeId, String)> {
    ev.order
        .iter()
        .map(|id| (*id, format!("{:?}", ev.nodes.get(id))))
        .collect()
}

/// The served evaluation of `after` over `prior` is the fresh one, on
/// every node's whole result. Answers `(reused, recomputed)` served.
fn served_is_fresh<T: OwnRecord>(
    prior: &Evaluation<T>,
    after: &ProfileDoc,
    what: &str,
) -> (usize, usize) {
    let fresh = evaluate::<T>(after, None);
    let served = evaluate::<T>(after, Some(prior));
    assert!(served.prior_refused.is_none(), "{what}: the prior is read");
    for ((id, s), (_, f)) in whole(&served).into_iter().zip(whole(&fresh)) {
        if s != f {
            let at = s
                .bytes()
                .zip(f.bytes())
                .position(|(a, b)| a != b)
                .unwrap_or(0);
            let from = at.saturating_sub(200);
            let said = format!(
                "{what} at {}: node {id:?} served differs from fresh at byte {at}:\n served: {}\n fresh:  {}",
                T::NAME,
                &s[from..(at + 200).min(s.len())],
                &f[from..(at + 200).min(f.len())],
            );
            panic!("{said}");
        }
    }
    (served.reused, served.recomputed)
}

/// Prior at `p = 0.25`, served at `p = 0.5`, then back, each over the
/// one before.
fn p_round_trip<T: OwnRecord>(doc: ProfileDoc, what: &str) {
    let prior = evaluate::<T>(&doc, None);
    let after = set(doc, "p", FreeValue::Continuous(0.5));
    served_is_fresh::<T>(&prior, &after, what);
    let mid = evaluate::<T>(&after, Some(&prior));
    let back = set(after, "p", FreeValue::Continuous(0.25));
    served_is_fresh::<T>(&mid, &back, &format!("{what}, back"));
}

/// `reads` as a list's entries: a plain read of a yield spliced in
/// whole, `…xs`, any other read one body.
fn entries(doc: &ProfileDoc, reads: Vec<BodyRead<Formula>>) -> Bodies<BodyRead<Formula>> {
    let spliced = |read: &BodyRead<Formula>| {
        read.at.is_empty()
            && matches!(&read.read, Operand::Node(node)
                if doc.outputs(*node).iter().any(|&out| doc.yielding(out).is_some()))
    };
    Bodies::Spelled(
        reads
            .into_iter()
            .map(|read| match spliced(&read) {
                true => editor_core::Spell::All(read),
                false => editor_core::Spell::One(read),
            })
            .collect(),
    )
}

fn union(doc: ProfileDoc, reads: Vec<BodyRead<Formula>>) -> (ProfileDoc, RecipeNodeId) {
    let members = entries(&doc, reads);
    insert(doc, Node::Union { members })
}

fn assemble(doc: ProfileDoc, reads: Vec<BodyRead<Formula>>) -> (ProfileDoc, RecipeNodeId) {
    let members = entries(&doc, reads);
    insert(doc, Node::Assemble { members })
}

/// A second family over `k` in `tall`'s space, yielded beside it:
/// `wide[k] = union([tall[k]])`.
fn wide(doc: ProfileDoc, tall: RecipeNodeId, k: VarId) -> (ProfileDoc, RecipeNodeId) {
    let (doc, wide) = union(doc, vec![BodyRead::indexed(tall, vec![var(k)])]);
    (fixture::yielded(doc, wide), wide)
}

#[test]
fn own_family_at_an_index_and_whole() {
    for interval in [false, true] {
        let (doc, k, tall) = tall("rv-own-both");
        let (doc, _) = assemble(
            doc,
            vec![
                BodyRead::plain(tall),
                BodyRead::indexed(tall, vec![next(k)]),
            ],
        );
        if interval {
            p_round_trip::<Interval>(doc, "assemble([tall, tall[next]])");
        } else {
            p_round_trip::<f64>(doc, "assemble([tall, tall[next]])");
        }
    }
}

#[test]
fn two_member_reads_of_one_family() {
    let (doc, k, tall) = tall("rv-two-reads");
    let (doc, other) = wide(doc, tall, k);
    let (doc, _) = assemble(
        doc,
        vec![
            BodyRead::indexed(tall, vec![var(k)]),
            BodyRead::indexed(other, vec![next(k)]),
            BodyRead::indexed(tall, vec![next(k)]),
        ],
    );
    p_round_trip::<f64>(doc, "assemble([tall[k], wide[next], tall[next]])");
}

#[test]
fn a_clamped_index() {
    let (doc, k, tall) = tall("rv-clamp");
    let clamp = Formula::min(
        Formula::add(var(k), Formula::count(1)).unwrap(),
        Formula::count(1),
    )
    .unwrap();
    let (doc, _) = union(doc, vec![BodyRead::indexed(tall, vec![clamp])]);
    p_round_trip::<f64>(doc, "union([tall[min(k + 1, 1)]])");
}

/// A node that is no member reading one member: served when the member
/// it reads did not move, rebuilt when it did.
#[test]
fn a_plain_nodes_member_read_follows_its_member() {
    let (doc, _, tall) = tall("rv-plain-reader");
    let (doc, still) = union(doc, vec![BodyRead::indexed(tall, vec![Formula::count(0)])]);
    let (doc, moved) = union(doc, vec![BodyRead::indexed(tall, vec![Formula::count(1)])]);
    let prior = evaluate::<f64>(&doc, None);
    let after = set(doc, "p", FreeValue::Continuous(0.5));
    served_is_fresh::<f64>(&prior, &after, "union([tall[0]]), union([tall[1]])");
    let served = evaluate::<f64>(&after, Some(&prior));
    let key = |ev: &Evaluation<f64>, id| ev.value(id).unwrap().content_key;
    assert_eq!(key(&served, still), key(&prior, still), "tall[0] held");
    assert_ne!(key(&served, moved), key(&prior, moved), "tall[1] moved");
}

/// An index a free count writes, in and out of range.
#[test]
fn an_index_out_of_range_and_back() {
    let doc = declare(with_p("rv-range"), "n", FreeVar::Count { value: 1 });
    let (doc, _, tall) = tall_of(doc, Formula::count(2));
    let (doc, reader) = union(doc, vec![BodyRead::indexed(tall, vec![named_count("n")])]);
    let one = evaluate::<f64>(&doc, None);
    assert!(one.value(reader).is_some(), "{:?}", one.result(reader));
    let out = set(doc, "n", FreeValue::Count(2));
    served_is_fresh::<f64>(&one, &out, "tall[n], n = 2 over n = 1");
    let refused = evaluate::<f64>(&out, Some(&one));
    assert!(
        matches!(refused.nodes.get(&reader), Some(NodeResult::Failed(_))),
        "out of range refuses: {:?}",
        refused.result(reader)
    );
    let back = set(out, "n", FreeValue::Count(0));
    served_is_fresh::<f64>(&refused, &back, "tall[n], n = 0 over n = 2");
    let moved = set(back.clone(), "p", FreeValue::Continuous(0.5));
    let at_zero = evaluate::<f64>(&back, None);
    served_is_fresh::<f64>(&at_zero, &moved, "tall[n], n = 0, p moved");
}

/// The family's count moves under a whole reader and a member reader.
#[test]
fn a_familys_count_moves_under_its_readers() {
    let doc = declare(with_p("rv-count"), "c", FreeVar::Count { value: 2 });
    let (doc, k, tall) = tall_of(doc, named_count("c"));
    let (doc, _) = union(doc, vec![BodyRead::plain(tall)]);
    let (doc, _) = assemble(
        doc,
        vec![BodyRead::plain(tall), BodyRead::indexed(tall, vec![var(k)])],
    );
    let (doc, _) = union(doc, vec![BodyRead::indexed(tall, vec![Formula::count(1)])]);
    let two = evaluate::<f64>(&doc, None);
    let three = set(doc, "c", FreeValue::Count(3));
    served_is_fresh::<f64>(&two, &three, "count 3 over count 2");
    let at_three = evaluate::<f64>(&three, Some(&two));
    let moved = set(three, "p", FreeValue::Continuous(0.5));
    served_is_fresh::<f64>(&at_three, &moved, "count 3, p moved");
    let at_moved = evaluate::<f64>(&moved, Some(&at_three));
    let one = set(moved, "c", FreeValue::Count(1));
    served_is_fresh::<f64>(&at_moved, &one, "count 1 over count 3");
}

/// A reader over `k` alone reading a family over `(k, j)` at `[next, 1]`
/// and whole; and a reader over `(k, j)` reading a family over `k`.
#[test]
fn readers_across_ranks() {
    let (doc, k) = fixture::index_of(with_p("rv-ranks"), "k", Formula::count(2), None);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(k));
    let by = Formula::mul(scalar(var(k)), scalar(var(j))).unwrap();
    let (doc, nested) = prism(doc, 0.0, depth(by));
    let doc = fixture::yielded(doc, nested);
    // Over k alone: the nested family at a written inner index.
    let (doc, a) = union(
        doc,
        vec![BodyRead::indexed(nested, vec![next(k), Formula::count(1)])],
    );
    // Over k alone: the nested family whole beside another row's member.
    let (doc, b) = assemble(
        doc,
        vec![
            BodyRead::plain(nested),
            BodyRead::indexed(nested, vec![next(k), Formula::count(0)]),
        ],
    );
    // Over (k, j): its own member beside the other row's at its own j.
    let (doc, c) = assemble(
        doc,
        vec![
            BodyRead::indexed(nested, vec![var(k), var(j)]),
            BodyRead::indexed(nested, vec![next(k), var(j)]),
        ],
    );
    for (id, chain) in [(a, vec![k]), (b, vec![k]), (c, vec![k, j])] {
        assert_eq!(doc.node_reach(id).chain(), Some(chain.as_slice()), "{id:?}");
    }
    p_round_trip::<f64>(doc.clone(), "readers across ranks");
    p_round_trip::<Interval>(doc, "readers across ranks");
}

/// A source of another member at a member seat of a union.
#[test]
fn a_source_of_a_member_at_a_member_seat() {
    let (doc, k, tall) = tall("rv-source-union");
    let (doc, reader) = union(
        doc,
        vec![BodyRead::plain(Operand::source(BodyRead::indexed(
            tall,
            vec![next(k)],
        )))],
    );
    let _ = reader;
    p_round_trip::<f64>(doc, "union([source(tall[next])])");
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

/// A pose definition whose read is a selection of another member: a
/// measure per member from a vertex of `tall[k]` to the point of the
/// same vertex of `tall[next]`.
#[test]
fn a_pose_of_a_selection_of_another_member() {
    let (doc, k, tall) = tall("rv-pose-select");
    let vertex = a_top_vertex(&doc, tall);
    let measured = editor_core::measure(
        &doc,
        &[MeasurePrimitive::Distance {
            a: Operand::select(BodyRead::indexed(tall, vec![var(k)]), vec![vertex.clone()]),
            b: fixture::point_of(Operand::select(
                BodyRead::indexed(tall, vec![next(k)]),
                vec![vertex],
            )),
        }],
        Tol::witness(),
    )
    .expect("the measure inserts");
    let reader = measured.measured.measures[0];
    assert_eq!(
        measured.doc.node_reach(reader).chain(),
        Some(&[k][..]),
        "a measure per member"
    );
    p_round_trip::<f64>(measured.doc.clone(), "gap(tall[k].v, point(tall[next].v))");
    p_round_trip::<Interval>(measured.doc, "gap(tall[k].v, point(tall[next].v))");
}

/// Two poses of one seat's family read at different members: a measure
/// between the points of one vertex at `tall[next]` and at `tall[k]`,
/// both poses.
#[test]
fn two_poses_of_two_members() {
    let (doc, k, tall) = tall("rv-two-poses");
    let vertex = a_top_vertex(&doc, tall);
    let measured = editor_core::measure(
        &doc,
        &[MeasurePrimitive::Distance {
            a: fixture::point_of(Operand::select(
                BodyRead::indexed(tall, vec![var(k)]),
                vec![vertex.clone()],
            )),
            b: fixture::point_of(Operand::select(
                BodyRead::indexed(tall, vec![next(k)]),
                vec![vertex],
            )),
        }],
        Tol::witness(),
    )
    .expect("the measure inserts");
    p_round_trip::<f64>(measured.doc, "gap(point(tall[k].v), point(tall[next].v))");
}

/// Renaming a variable a read comes through moves no key.
#[test]
fn a_rename_keeps_every_node() {
    let doc = ProfileDoc::empty_derived("rv-rename", Tol::witness());
    let (doc, body) = prism(doc, 0.0, fixture::len(1.0));
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("spare"),
            def: VarDecl::Source(BodyRead::plain(Operand::Node(body))),
        },
    );
    let spare = doc.var_named("spare").expect("declared");
    let (doc, _) = assemble(doc, vec![BodyRead::plain(Operand::Var(spare))]);
    let prior = evaluate::<f64>(&doc, None);
    let (after, _) = step(
        doc,
        DocEdit::RenameVar {
            var: spare.into(),
            name: Some(VarName::from_static("other")),
        },
    );
    let (reused, recomputed) = served_is_fresh::<f64>(&prior, &after, "a source renamed");
    assert_eq!(recomputed, 0, "reused {reused}");
}

/// Keys and results are the same under the level-parallel walk and the
/// serial one, run after run, over a prior and fresh.
#[test]
fn keys_are_the_same_on_both_schedules() {
    let (doc, k, tall) = tall("rv-parallel");
    let (doc, other) = wide(doc, tall, k);
    let (doc, _) = assemble(
        doc,
        vec![
            BodyRead::plain(tall),
            BodyRead::indexed(other, vec![next(k)]),
            BodyRead::indexed(tall, vec![next(k)]),
        ],
    );
    let (doc, _) = union(doc, vec![BodyRead::indexed(tall, vec![Formula::count(1)])]);
    let (doc, _) = union(doc, vec![BodyRead::indexed(other, vec![next(k)])]);
    let run = |doc: &ProfileDoc, parallel: bool, prior: Option<&Evaluation<f64>>| {
        editor_core::evaluate::<f64>(
            doc,
            prior,
            &CancelToken::new(),
            &EvalOptions {
                parallel,
                ..EvalOptions::default()
            },
            Tol::witness(),
        )
    };
    let serial = run(&doc, false, None);
    let after = set(doc, "p", FreeValue::Continuous(0.5));
    let fresh = whole(&run(&after, false, None));
    for round in 0..40 {
        let parallel = run(&after, true, None);
        assert!(whole(&parallel) == fresh, "round {round}: parallel fresh");
        let served = run(&after, true, Some(&serial));
        assert!(served.prior_refused.is_none());
        assert!(whole(&served) == fresh, "round {round}: parallel served");
    }
}

/// Each face of member 0 of `tall`, in the member's own spelling.
fn member_faces(doc: &ProfileDoc, tall: RecipeNodeId) -> Vec<StableName> {
    let ev = evaluate::<f64>(doc, None);
    let table = ev
        .table(doc.output(tall, 0).expect("tall's read"))
        .expect("tall's table");
    table
        .iter()
        .filter_map(|(name, entry)| {
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
        .collect()
}

/// A pose at a one-body node: `reader[k] = mirror(wide[k],
/// plane(tall[next].f))` for each face `f` of a member, the plane a pose
/// of a selection of another member, the body a member of a second
/// family: a one-body node reading two members of ONE family projects
/// both into one map and is a kernel `unreachable!`
/// (`work/boxfinish/a-one-body-node-reading-two-members-of-one-family-reaches-an-unreachable.md`).
/// At least one face's plane moves with `p`.
#[test]
fn a_mirror_across_another_members_face() {
    let (doc, k, tall) = tall("rv-mirror");
    let (doc, wide) = wide(doc, tall, k);
    let faces = member_faces(&doc, tall);
    assert_eq!(faces.len(), 6, "a prism's faces");
    let mut moved = 0;
    for face in faces {
        let plane = fixture::plane(Operand::select(
            BodyRead::indexed(tall, vec![next(k)]),
            vec![face.clone()],
        ));
        let node = Node::Mirror {
            body: BodyRead::indexed(wide, vec![var(k)]),
            plane: editor_core::pose::RoleRead::plain(plane),
        };
        let (doc, reader) = insert(doc.clone(), node);
        assert_eq!(doc.node_reach(reader).chain(), Some(&[k][..]));
        let prior = evaluate::<f64>(&doc, None);
        let after = set(doc, "p", FreeValue::Continuous(0.5));
        let fresh = evaluate::<f64>(&after, None);
        let at = |ev: &Evaluation<f64>| format!("{:?}", ev.nodes.get(&reader));
        if at(&prior) != at(&fresh) {
            moved += 1;
        }
        served_is_fresh::<f64>(&prior, &after, "mirror(wide[k], plane(tall[next].f))");
        let mid = evaluate::<f64>(&after, Some(&prior));
        let back = set(after, "p", FreeValue::Continuous(0.25));
        served_is_fresh::<f64>(&mid, &back, "mirror(wide[k], plane(tall[next].f)), back");
    }
    assert!(moved > 0, "some face's plane moves with p");
}
