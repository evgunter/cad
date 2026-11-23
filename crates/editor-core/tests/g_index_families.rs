//! **Repetition is an index** (D10 `[repetition]`, INTENT stage 3 G):
//! `k = index(N)` is a `Count` over `0..N`; an operation whose reads
//! reach it is evaluated once per value, and its value is a family,
//! its members named `Member { (k), of }`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::block;
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, Coord, Dimension, DocEdit, Evaluation, Formula, Node, Operand, ProfileDoc,
    RecipeNodeId, RoleSeg, TargetAxis, ValuePayload, VarDecl, VarId, VarName, parse_formula,
};
use geom_core::Tol;

/// `doc` with the index `name = index(count)` declared, and its id.
pub(crate) fn index(doc: ProfileDoc, name: &str, count: Formula) -> (ProfileDoc, VarId) {
    let name = VarName::new(name).unwrap();
    let doc = fixture::step(
        doc,
        DocEdit::DeclareVar {
            name: name.clone(),
            def: VarDecl::Index { count, after: None },
        },
    )
    .0;
    let id = doc.var_named(name.as_str()).expect("the index is declared");
    (doc, id)
}

/// `text` parsed over the document's named variables.
fn formula(doc: &ProfileDoc, text: &str) -> Formula {
    let params: BTreeMap<VarName, Dimension> = doc
        .var_ids()
        .into_iter()
        .filter_map(|id| {
            let name = doc.var_name(id)?.clone();
            let dim = doc.var(id)?.kind().dimension()?;
            Some((name, dim))
        })
        .collect();
    parse_formula(text, &params).unwrap()
}

/// **A ring of `n` copies of a block off the z axis**, member `k` spun
/// `scalar(k)·turn/n` about the world's z by a mateless bundle: the
/// document, the block and the placement whose copy is the family.
pub(crate) fn ring(n: i64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g-ring", Tol::witness());
    let (doc, _) = index(doc, "k", Formula::count(n));
    let (doc, the_block) = block(doc, (4.0, 5.0), (-0.5, 0.5), 1.0);
    let spin = formula(&doc, &format!("scalar(k) * turn / {n}"));
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![(Coord::Spin(Some(TargetAxis::Z)), spin)],
    );
    let node = fixture::place_node(
        &doc,
        editor_core::BodyRead::plain(Operand::Node(the_block)),
        placed.bundle(),
    );
    let (doc, ring) = insert(doc, node);
    // The ring is the binder's yield (D10 `[repetition]`).
    let doc = fixture::yielded(doc, ring);
    (doc, the_block, ring)
}

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn bodies(ev: &Evaluation<f64>, id: RecipeNodeId) -> usize {
    match &ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?}: {:?}", ev.result(id)))
        .payload
    {
        ValuePayload::Instances(bodies) => bodies.len(),
        ValuePayload::Body(_) | ValuePayload::Boolean(_) => 1,
        other => panic!("a body or a family, got {}", other.kind_name()),
    }
}

/// **A ring is one placement whose spin reads the index**: twelve
/// copies, each one `Arc` with the block's (built once and mapped,
/// §10 row 16), each member's names `Member { (k), of }`, and member
/// `k` turned `k/12` of a turn. Red if the placement is evaluated once,
/// a member is built anew, or a member's name drops its index.
#[test]
fn a_ring_is_one_placement_whose_spin_reads_the_index() {
    let (doc, the_block, ring) = ring(12);
    let ev = run(&doc);
    assert_eq!(bodies(&ev, ring), 12, "one copy per value of k");
    let ValuePayload::Instances(copies) = &ev.value(ring).unwrap().payload else {
        unreachable!()
    };
    let copies = fixture::every_body(copies);
    let ValuePayload::Body(source) = &ev.value(the_block).unwrap().payload else {
        panic!("the block is a body")
    };
    for (k, copy) in copies.iter().enumerate() {
        assert!(
            std::sync::Arc::ptr_eq(copy.holding().arc(), source.holding().arc()),
            "member {k} shares the block's one body"
        );
    }
    let k = doc.var_named("k").unwrap();
    let table = fixture::table(&ev, ring);
    let mut at: Vec<u32> = table
        .iter()
        .filter_map(|(name, _)| match name.path.as_slice() {
            [RoleSeg::Member { at, .. }] => {
                assert_eq!(at.len(), 1, "one index");
                assert_eq!(at[0].0, k, "keyed by k's id");
                Some(at[0].1)
            }
            _ => None,
        })
        .collect();
    at.sort_unstable();
    at.dedup();
    assert_eq!(at, (0..12).collect::<Vec<u32>>(), "every member named");
    // Member 3 is a quarter turn, exactly: the block's span in x is
    // member 3's span in y.
    let three = fixture::in_space(copies[3]);
    for (v, _) in three.vertices() {
        let p = fixture::point(&three, v);
        assert!((4.0..=5.0).contains(&p.y), "member 3 stands on +y: {p:?}");
        assert!((-0.5..=0.5).contains(&p.x), "and across x: {p:?}");
    }
}

/// **A union of the family fuses it**; an indexed read `ring[k]` reads
/// one member.
#[test]
fn a_union_reads_the_family_whole_and_an_indexed_read_one_member() {
    let (doc, _, ring) = ring(4);
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: Bodies::Family(ring.into()),
        },
    );
    let (doc, one) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![editor_core::BodyRead::indexed(
                ring,
                vec![Formula::count(2)],
            )]),
        },
    );
    let ev = run(&doc);
    assert_eq!(bodies(&ev, ring), 4);
    assert_eq!(
        bodies(&ev, fused),
        1,
        "the union is one body: {:?}",
        ev.result(fused)
    );
    assert_eq!(bodies(&ev, one), 1, "{:?}", ev.result(one));
}

/// **`n` fins sunk 0.25 into a base**, every copy in the world: the
/// base, and one placement of the fin whose slide reads `k =
/// index(n)`; and one authored assertion, `Gap(fin's foot, base's top)
/// ≤ −0.1`, its selection of the fin read off the family. The document,
/// the index, the fins' placement and the assertion.
fn sunk_fins(n: i64) -> (ProfileDoc, VarId, RecipeNodeId, RecipeNodeId) {
    use editor_core::{AssertionRelation, CapEnd, MeasurePrimitive, SitedRef};
    let doc = ProfileDoc::empty_derived("g-sunk-fins", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", n);
    let (doc, base) = block(doc, (0.0, 13.0), (0.0, 2.0), 1.0);
    let (doc, fin) = block(doc, (0.25, 0.5), (0.5, 1.5), 1.25);
    let (doc, base_copy) = fixture::place(doc, base);
    let slide = Formula::mul(fixture::scalar_of(k), fixture::len(0.75)).unwrap();
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::slide(TargetAxis::X), slide),
            (Coord::slide(TargetAxis::Z), fixture::len(0.75)),
        ],
    );
    let node = fixture::place_node(
        &doc,
        editor_core::BodyRead::plain(Operand::Node(fin)),
        placed.bundle(),
    );
    let (doc, fins) = insert(doc, node);
    // The fins are the binder's yield, `{k in 0..n: fins = place(fin)}`,
    // and join the world's copies as one.
    let doc = fixture::yielded(doc, fins);
    let (doc, gap) = fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        // Inside the binder the measure reads the fin of its own member,
        // `fins[k]` (DM3): a selection of the member read at the key.
        vec![
            Operand::select(
                editor_core::BodyRead::indexed(fins, vec![Formula::var(k, Dimension::Count)]),
                vec![fixture::fname(fin, RoleSeg::Cap(CapEnd::Start))],
            ),
            SitedRef::new(base_copy, fixture::fname(base, RoleSeg::Cap(CapEnd::End))).into(),
        ],
    );
    let value = fixture::value_of(&doc, gap);
    let (doc, assertion) = insert(
        doc,
        Node::Assertion(editor_core::Claim::Value {
            value,
            bound: fixture::len(-0.1),
            relation: AssertionRelation::AtMost,
        }),
    );
    (doc, k, fins, assertion)
}

/// **One authored assertion over `fins[k]` is one assertion per member**
/// (§10 row 22): each fin's interference with the base is a finding of
/// its own, each quiet by its own member of the assertion's family, and
/// still all quiet after `n` goes from 12 to 16. Red if the count is
/// fixed at authoring, or one member's finding is quieted by another
/// member's assertion.
#[test]
fn one_assertion_over_a_family_quiets_each_members_own_finding() {
    let (doc, k, fins, assertion) = sunk_fins(12);
    let check = |doc: &ProfileDoc, n: u32| {
        let fins = doc.output(fins, 0).unwrap();
        let ev = run(doc);
        let findings = fixture::at_rest(doc, &ev).unwrap_or_else(|e| panic!("{e}"));
        let interference = fixture::interference(&findings);
        assert_eq!(
            interference.len(),
            n as usize,
            "one finding per fin: {interference:#?}"
        );
        let mut quieted_by: Vec<u32> = interference
            .iter()
            .map(|finding| {
                assert_eq!(finding.quiet, Some(assertion), "quiet: {finding:?}");
                let [(index, value)] = finding.quiet_at.as_slice() else {
                    panic!("one index: {finding:?}")
                };
                assert_eq!(*index, k);
                // The member that quiets it is the fin's own.
                let fin = [finding.a, finding.b]
                    .into_iter()
                    .find(|copy| copy.var == fins)
                    .expect("a fin is one of the two copies");
                assert_eq!(
                    fin.member, *value,
                    "fin {} is quieted by its own member",
                    fin.member
                );
                *value
            })
            .collect();
        quieted_by.sort_unstable();
        assert_eq!(
            quieted_by,
            (0..n).collect::<Vec<_>>(),
            "each member quiets one finding"
        );
    };
    check(&doc, 12);
    let (doc, _) = fixture::step(
        doc,
        DocEdit::DefineVar {
            var: editor_core::VarRef::Id(k),
            def: VarDecl::Index {
                count: Formula::count(16),
                after: None,
            },
            fresh: Vec::new(),
        },
    );
    check(&doc, 16);
}

/// A block of unit footprint at the origin, its own space.
pub(crate) fn unit_block(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId) {
    block(doc, (0.0, 0.5), (0.0, 0.5), 0.5)
}

/// **A grid is two indices, one within the other** (D10
/// `[repetition]`): `j = index(2) within i`, one placement sliding `x`
/// by `i` and `y` by `j`, one member per `(i, j)`, outer first, named
/// `Member { (i, j), of }`. Red if the inner index is evaluated as its
/// own family, or the order is inner first.
#[test]
fn a_grid_is_two_indices_one_within_the_other() {
    let doc = ProfileDoc::empty_derived("g-grid", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 3);
    let (doc, j) = fixture::index_of(doc, "j", Formula::count(2), Some(i));
    let (doc, cell) = unit_block(doc);
    let slide = |index: VarId| Formula::mul(fixture::scalar_of(index), fixture::len(1.0)).unwrap();
    let (doc, grid) = fixture::family_of(
        doc,
        cell,
        vec![
            (Coord::slide(TargetAxis::X), slide(i)),
            (Coord::slide(TargetAxis::Y), slide(j)),
        ],
    );
    let ev = run(&doc);
    let members = ev.value(grid).unwrap().members.as_ref().expect("a family");
    assert_eq!(&*members.over, [i, j], "outer first");
    let at: Vec<Vec<u32>> = members.members.iter().map(|(at, _)| at.clone()).collect();
    assert_eq!(
        at,
        [[0, 0], [0, 1], [1, 0], [1, 1], [2, 0], [2, 1]]
            .map(Vec::from)
            .to_vec(),
        "every (i, j), i outer"
    );
    let named: std::collections::BTreeSet<Vec<(VarId, u32)>> = fixture::table(&ev, grid)
        .iter()
        .filter_map(|(name, _)| match name.path.as_slice() {
            [RoleSeg::Member { at, .. }] => Some(at.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(named.len(), 6, "a member's names are keyed by both indices");
    assert!(named.contains(&vec![(i, 2), (j, 1)]));
}

/// **A reader repeats inside one binder**: a placement reading the
/// indices of two binders refuses at the edit door, naming both. Red if
/// the door admits it (it would repeat over both unwritten).
#[test]
fn a_reader_of_two_binders_indices_refuses_at_the_door() {
    let doc = ProfileDoc::empty_derived("g-unrelated", Tol::witness());
    let (doc, i) = fixture::index(doc, "i", 2);
    let (doc, j) = fixture::index(doc, "j", 2);
    let (doc, cell) = unit_block(doc);
    let slide = |index: VarId| Formula::mul(fixture::scalar_of(index), fixture::len(1.0)).unwrap();
    let target = fixture::space(&doc, cell);
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::slide(TargetAxis::X), slide(i)),
            (Coord::slide(TargetAxis::Y), slide(j)),
        ],
    );
    let refused = fixture::insert_refused(
        &doc,
        Node::place(
            editor_core::BodyRead::plain(Operand::Node(cell)),
            Operand::Var(target),
            placed.bundle(),
        ),
    );
    let editor_core::EditError::TwoBinders { a, b, .. } = &refused else {
        panic!("two binders' indices refuse: {refused:?}")
    };
    assert_eq!([a.id(), b.id()], [i, j], "{refused}");
    assert!(
        refused.to_string().contains("which are of two binders"),
        "{refused}"
    );
}

/// **A read out of range is unresolved, typed**: `row[3]` of a row of
/// three refuses `MemberOutOfRange` naming the index, and a negative
/// index likewise. Red if it wraps or clamps.
#[test]
fn a_member_read_out_of_range_refuses_typed() {
    let doc = ProfileDoc::empty_derived("g-out-of-range", Tol::witness());
    let (doc, cell) = unit_block(doc);
    let (doc, _, row) = fixture::row(doc, cell, ("k", 3), TargetAxis::X, 1.0);
    for at in [3, -1] {
        let (doc, read) = insert(
            doc.clone(),
            Node::Union {
                members: Bodies::spelled(vec![editor_core::BodyRead::indexed(
                    row,
                    vec![Formula::count(at)],
                )]),
            },
        );
        let ev = run(&doc);
        match ev.result(read).and_then(|r| r.error()).map(|e| &e.kind) {
            Some(editor_core::NodeErrorKind::MemberOutOfRange { input, at: was }) => {
                assert_eq!((*input, was.as_slice()), (row, [at].as_slice()));
            }
            other => panic!("row[{at}] is out of range: {other:?}"),
        }
    }
}

/// **A construction whose scalar input reads the index is built once
/// per member, and a member that refuses refuses the family, naming
/// it**: an extrude of depth `scalar(k)·0.25` is three bodies of three
/// heights for `k` in `1..`, and at `k = 0` (a zero depth) the family
/// refuses `MemberRefused` at `k = 0`. Red if one member's refusal is
/// dropped, or the family is built once.
#[test]
fn a_construction_reading_the_index_is_built_per_member() {
    let doc = ProfileDoc::empty_derived("g-construction", Tol::witness());
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(3), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let depth = |offset: i64| {
        let count =
            Formula::add(Formula::var(k, Dimension::Count), Formula::count(offset)).unwrap();
        Formula::mul(Formula::count_to_scalar(count).unwrap(), fixture::len(0.25)).unwrap()
    };
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth(1),
        },
    );
    let (doc, flat) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth(0),
        },
    );
    // Both extrusions are the binder's yields.
    let doc = fixture::yielded(fixture::yielded(doc, tall), flat);
    let ev = run(&doc);
    let members = ev
        .value(tall)
        .unwrap()
        .members
        .as_ref()
        .expect("a family of extrudes");
    let heights: Vec<f64> = members
        .members
        .iter()
        .map(|(_, member)| {
            let ValuePayload::Body(body) = &member.payload else {
                panic!("a body")
            };
            let body = fixture::in_space(body);
            body.vertices()
                .map(|(v, _)| fixture::point(&body, v).z)
                .fold(f64::NEG_INFINITY, f64::max)
        })
        .collect();
    assert_eq!(
        heights,
        [0.25, 0.5, 0.75],
        "one body per member, each its own depth"
    );
    match ev.result(flat).and_then(|r| r.error()).map(|e| &e.kind) {
        Some(editor_core::NodeErrorKind::MemberRefused { at, source }) => {
            assert_eq!(at.as_slice(), [(k, 0)], "named at k = 0: {source:?}");
        }
        other => panic!("the zero-depth member refuses the family: {other:?}"),
    }
}

/// **Neighbours in a ring are two reads through `mod`**: `ring[k]` and
/// `ring[(k + 1) mod 4]`, one union per `k`, each the union of two
/// neighbours, the last with the first. Red if `mod` is not exact or
/// the closing read leaves the range.
#[test]
fn a_rings_neighbours_are_read_through_mod() {
    let doc = ProfileDoc::empty_derived("g-mod", Tol::witness());
    let (doc, cell) = block(doc, (2.0, 3.0), (-0.25, 0.25), 0.5);
    let (doc, k, ring) = fixture::ring(doc, cell, ("k", 4), TargetAxis::Z);
    let next = Formula::modulo(
        Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).unwrap(),
        Formula::count(4),
    )
    .unwrap();
    let (doc, pairs) = insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![
                editor_core::BodyRead::indexed(ring, vec![Formula::var(k, Dimension::Count)]),
                editor_core::BodyRead::indexed(ring, vec![next]),
            ]),
        },
    );
    let ev = run(&doc);
    let members = ev
        .value(pairs)
        .unwrap_or_else(|| panic!("{:?}", ev.result(pairs)))
        .members
        .as_ref()
        .expect("one union per k");
    assert_eq!(members.members.len(), 4);
    for (at, member) in &members.members {
        let ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) = &member.payload
        else {
            panic!("k = {at:?}: a body, got {}", member.payload.kind_name())
        };
        assert_eq!(
            body.body().solids().count(),
            2,
            "k = {at:?}: two neighbours, apart"
        );
    }
    assert_eq!(
        doc.var(k).map(editor_core::Var::kind),
        Some(editor_core::VarKind::Count)
    );
}

/// **A member is reused by its key** (D9: a node reading an index keys
/// by its content key plus the index values): a second evaluation over
/// the first reuses every member of the ring, and one after the count
/// grows reuses the first members and builds the new ones. Red if the
/// family is keyed whole (no member reused after the count moves).
#[test]
fn a_member_is_reused_by_its_key() {
    let (doc, _, ring) = ring(4);
    let first = run(&doc);
    let again = fixture::run_with_prior(&doc, &first);
    assert!(
        again.reused >= 1
            && again.value(ring).unwrap().content_key == first.value(ring).unwrap().content_key,
        "the family is served again"
    );
    let k = doc.var_named("k").unwrap();
    let (grown, _) = fixture::step(
        doc,
        DocEdit::DefineVar {
            var: editor_core::VarRef::Id(k),
            def: VarDecl::Index {
                count: Formula::count(6),
                after: None,
            },
            fresh: Vec::new(),
        },
    );
    let after = fixture::run_with_prior(&grown, &first);
    let keys = |ev: &Evaluation<f64>| -> Vec<editor_core::ContentKey> {
        ev.value(ring)
            .unwrap()
            .members
            .as_ref()
            .unwrap()
            .members
            .iter()
            .map(|(_, m)| m.content_key)
            .collect()
    };
    let (before, now) = (keys(&first), keys(&after));
    assert_eq!(now.len(), 6);
    assert_eq!(now[..4], before[..], "the four members keep their keys");
}

/// **A comb**: two teeth `width` wide, three apart along x, fused into
/// one body of two solids.
fn comb(doc: ProfileDoc, width: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, a) = block(doc, (0.0, width), (0.0, 1.0), 1.0);
    let (doc, b) = crate::blocks::block_on(doc, a, (3.0, 3.0 + width), (0.0, 1.0), 0.0, 1.0);
    insert(
        doc,
        Node::Union {
            members: Bodies::spelled(vec![
                editor_core::BodyRead::plain(Operand::Node(a)),
                editor_core::BodyRead::plain(Operand::Node(b)),
            ]),
        },
    )
}

/// The union of a row of three combs `pitch` apart, the judgements it
/// ran, and its solids.
fn interleaved(width: f64, pitch: f64) -> (u64, usize) {
    let doc = ProfileDoc::empty_derived("g-fast-path", Tol::witness());
    let (doc, teeth) = comb(doc, width);
    let (doc, _, row) = fixture::row(doc, teeth, ("k", 3), TargetAxis::X, pitch);
    let (doc, fused) = insert(
        doc,
        Node::Union {
            members: Bodies::Family(editor_core::BodyRead::plain(Operand::Node(row))),
        },
    );
    let before = editor_core::EvalCounters::on_this_thread();
    let ev = run(&doc);
    let judged = editor_core::EvalCounters::on_this_thread()
        .since(before)
        .judgements;
    let ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. }) = &ev
        .value(fused)
        .unwrap_or_else(|| panic!("{:?}", ev.result(fused)))
        .payload
    else {
        panic!("the union is a body")
    };
    // The combs' own two-tooth unions judge no pair.
    (judged, body.body().solids().count())
}

/// **A union of one family of one placement is certified apart, not
/// judged** (`placed-union-places-and-fuses-in-one-node`): three combs
/// interleaved a pitch of one apart have hulls that all meet and teeth
/// that never do, so the separation certificate of the one body holds
/// every pair apart and no pairwise judgement runs; the union is six
/// teeth. Red if the union judges the pairs its members' reads show are
/// rigid images held apart.
#[test]
fn a_union_of_one_family_is_certified_apart_not_judged() {
    assert_eq!(interleaved(0.8, 1.0), (0, 6));
}

/// **An uncertified pair goes through the pairwise pass, never
/// refuses**: combs whose teeth touch face to face are judged and fuse,
/// six teeth into one run.
/// Red if a pair the certificate cannot separate refuses, as the group
/// boolean's certificate did.
#[test]
fn an_uncertified_pair_is_judged_not_refused() {
    let (judged, solids) = interleaved(1.0, 1.0);
    assert!(judged > 0, "the touching pairs are judged");
    assert_eq!(solids, 1, "the touching teeth fuse into one run of six");
}

/// **A family yields to cancelation inside its node**: a ring of a
/// million copies under a budget that admits it, canceled from another
/// thread, returns as a canceled run with the ring holding no result,
/// on the serial walk and on the level schedule alike, whether the
/// cancel lands while the ring is charged or while it is built. Red (it
/// runs on for about twenty minutes) if the token is read only between
/// nodes or levels and the cancel lands after the ring has begun.
/// Nothing here asserts on timing; which member a family stops at is
/// pinned in `eval::members::tests`.
#[test]
fn a_ring_canceled_mid_family_returns_the_completed_prefix() {
    const N: u32 = 1_000_000;
    let (doc, _, ring) = ring(i64::from(N));
    for parallel in [false, true] {
        let cancel = editor_core::CancelToken::new();
        let canceler = {
            let cancel = cancel.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(150));
                cancel.cancel();
            })
        };
        let opts = editor_core::EvalOptions {
            parallel,
            member_budget: u64::from(N),
            ..Default::default()
        };
        let ev = editor_core::evaluate::<f64>(&doc, None, &cancel, &opts, Tol::witness());
        canceler.join().unwrap();
        assert_eq!(
            ev.outcome,
            editor_core::EvalOutcome::Canceled,
            "parallel {parallel}: the run is canceled"
        );
        assert!(
            ev.result(ring).is_none(),
            "parallel {parallel}: the ring, canceled part-way, holds no result"
        );
    }
}

/// **A profile whose step reads the index is a family, and each
/// member's walls are named by that member's profile pieces** (D10
/// `[repetition]`; `[recipe.doors]`): the edit door admits the step,
/// the profile is evaluated once per member, the extrude reading it is
/// a family of three bodies, and each body's walls are
/// `Member { (k), of: Lateral(piece) }` over the profile's own step
/// ids. Red if the door refuses the step outside any member, if the
/// profile is evaluated once, or if a member's walls are named by
/// anything but the generic member machinery over the pieces.
#[test]
fn a_profile_reading_the_index_is_a_family_named_by_its_pieces() {
    let doc = ProfileDoc::empty_derived("g-profile-family", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", 3);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    // The start corner's x slides left by a tenth per member.
    let x = Formula::add(
        fixture::len(-0.5),
        Formula::mul(fixture::scalar_of(k), fixture::len(-0.1)).unwrap(),
    )
    .unwrap();
    let doc = fixture::step(
        doc,
        DocEdit::SetParam {
            node: profile,
            slot: editor_core::SlotId::Profile {
                loop_: 0,
                step: 0,
                arg: editor_core::StepArg::PointX,
            },
            value: x.into(),
            fresh: Vec::new(),
        },
    )
    .0;
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(1.0),
        },
    );
    // The extrusion is the binder's yield.
    let doc = fixture::yielded(doc, ext);
    let ev = run(&doc);
    // A family of profiles is no one value: its members are its result's.
    let profiles = ev
        .result(profile)
        .and_then(editor_core::NodeResult::members);
    assert_eq!(
        profiles.map(|m| m.members.len()),
        Some(3),
        "the profile is evaluated once per member"
    );
    assert_eq!(bodies(&ev, ext), 3, "one extruded body per member");
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{profile} is a profile")
    };
    let steps = &program.ids[0];
    let walls: std::collections::BTreeSet<(Vec<(VarId, u32)>, editor_core::StepId)> =
        fixture::table(&ev, ext)
            .iter()
            .filter_map(|(name, _)| match name.path.as_slice() {
                [RoleSeg::Member { at, of }] => match of.path.as_slice() {
                    [RoleSeg::Lateral(run)] => match run.pieces() {
                        [editor_core::ProfileEdgeRef::Piece { step, .. }] => {
                            Some((at.clone(), *step))
                        }
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
    for member in 0..3 {
        for step in &steps[1..] {
            assert!(
                walls.contains(&(vec![(k, member)], *step)),
                "member {member} names the wall of step {step:?}: {walls:?}"
            );
        }
    }
    assert_every_name_is_a_members(&ev, ext);
}

/// Every name `ext`'s table holds is a member's: a family names
/// nothing outside its members.
fn assert_every_name_is_a_members(ev: &Evaluation<f64>, ext: RecipeNodeId) {
    for (name, _) in fixture::table(ev, ext).iter() {
        assert!(
            matches!(name.path.as_slice(), [RoleSeg::Member { .. }]),
            "a family's name is a member's: {name:?}"
        );
    }
}

/// `doc`'s profile `profile`, its step `step`'s `arg` written `formula`.
fn written_at(
    doc: ProfileDoc,
    profile: RecipeNodeId,
    step: u32,
    arg: editor_core::StepArg,
    formula: Formula,
) -> ProfileDoc {
    fixture::step(
        doc,
        DocEdit::SetParam {
            node: profile,
            slot: editor_core::SlotId::Profile {
                loop_: 0,
                step,
                arg,
            },
            value: formula.into(),
            fresh: Vec::new(),
        },
    )
    .0
}

/// **A member whose values leave a fillet run undrawn names no wall for
/// it, and the others do**: a corner fillet whose radius reads `k`
/// (0.3, 1.15, 2), at 2 both runs fitting `Zero`. Each member is named
/// by its own evaluation, through the generic member machinery. Red if
/// the members share one naming (member 2 would name the runs), or the
/// family names anything outside its members.
#[test]
fn a_members_own_values_decide_which_pieces_it_names() {
    use editor_core::{LoopProgram, ProfileEdgeRef, ProgramStep, ProgramTarget};
    use profile::PieceRole;
    let doc = ProfileDoc::empty_derived("g-profile-family-fit", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", 3);
    let toward = |dx, dy| ProgramStep::Toward {
        dx: fixture::scl(dx),
        dy: fixture::scl(dy),
    };
    let (doc, profile) = insert(
        doc,
        Node::Profile(editor_core::ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At(fixture::len2([0.0, 0.0])),
                toward(1.0, 0.0),
                ProgramStep::Fillet(fixture::len(0.3)),
                toward(0.0, 1.0),
                ProgramStep::FarEndTo(fixture::len2([2.0, 2.0])),
                ProgramStep::LineTo(ProgramTarget::Point(fixture::len2([0.0, 2.0]))),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
            on: None,
        }),
    );
    let radius = Formula::add(
        fixture::len(0.3),
        Formula::mul(fixture::scalar_of(k), fixture::len(0.85)).unwrap(),
    )
    .unwrap();
    let doc = written_at(doc, profile, 2, editor_core::StepArg::Radius, radius);
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(1.0),
        },
    );
    // The extrusion is the binder's yield.
    let doc = fixture::yielded(doc, ext);
    let ev = run(&doc);
    assert_eq!(bodies(&ev, ext), 3, "one extruded body per member");
    let Some(Node::Profile(program)) = doc.node(profile) else {
        panic!("{profile} is a profile")
    };
    let fillet = program.ids[0][2];
    let named = |member: u32, role: PieceRole| {
        fixture::table(&ev, ext)
            .iter()
            .any(|(name, _)| match name.path.as_slice() {
                [RoleSeg::Member { at, of }] if at.as_slice() == [(k, member)] => {
                    match of.path.as_slice() {
                        [RoleSeg::Lateral(run)] => run
                            .pieces()
                            .contains(&ProfileEdgeRef::Piece { step: fillet, role }),
                        _ => false,
                    }
                }
                _ => false,
            })
    };
    for member in [0, 1] {
        for role in [PieceRole::RunIn, PieceRole::Arc, PieceRole::RunOut] {
            assert!(
                named(member, role),
                "member {member} names its fillet's {role:?}"
            );
        }
    }
    assert!(named(2, PieceRole::Arc), "member 2 names its fillet's arc");
    for role in [PieceRole::RunIn, PieceRole::RunOut] {
        assert!(
            !named(2, role),
            "member 2's {role:?} fits Zero: no wall names it"
        );
    }
    assert_every_name_is_a_members(&ev, ext);
}

/// **A member whose profile refuses refuses the family, naming the
/// member, and its reader is poisoned**: the start corner of a unit
/// square at `x = −0.5 + 0.6·k` crosses the right side at `k = 2`.
/// Red if the bad member is dropped, the family is built from the
/// others, or the refusal names no member.
#[test]
fn a_profile_member_that_refuses_refuses_the_family() {
    let doc = ProfileDoc::empty_derived("g-profile-family-bad", Tol::witness());
    let (doc, k) = fixture::index(doc, "k", 3);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let x = Formula::add(
        fixture::len(-0.5),
        Formula::mul(fixture::scalar_of(k), fixture::len(0.6)).unwrap(),
    )
    .unwrap();
    let doc = written_at(doc, profile, 0, editor_core::StepArg::PointX, x);
    let (doc, ext) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(1.0),
        },
    );
    // The extrusion is the binder's yield.
    let doc = fixture::yielded(doc, ext);
    let ev = run(&doc);
    match ev.result(profile).and_then(|r| r.error()).map(|e| &e.kind) {
        Some(editor_core::NodeErrorKind::MemberRefused { at, source }) => {
            assert_eq!(at.as_slice(), [(k, 2)], "named at k = 2: {source:?}");
            assert!(
                matches!(
                    **source,
                    editor_core::NodeErrorKind::Profile(profile::ProfileError::NonSimple { .. })
                ),
                "the member's loop crosses itself: {source:?}"
            );
        }
        other => panic!("the crossing member refuses the family: {other:?}"),
    }
    assert!(
        matches!(ev.result(ext), Some(editor_core::NodeResult::Poisoned { through }) if *through == profile),
        "the extrude is poisoned through the profile: {:?}",
        ev.result(ext)
    );
}
