//! **A family's carry, its reading per seat, its count's range, its
//! memo and the union's fast path** (D10 `[repetition]`; INTENT stage 3
//! G's review rows).
//!
//! A family read whole carries each member's own carry, so what a
//! member holds is reported whichever member holds it; one operation
//! reads a family plainly at one seat and whole at another; a count
//! outside the counts an index holds refuses typed; a count edit serves
//! the members it keeps from the memo; and the union of one family
//! judges every pair its certificate does not separate, building what
//! the slow path builds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, BooleanValue, Coord, Dimension, DocEdit, Evaluation, Formula, Node, Operand,
    ProfileDoc, RecipeNodeId, TargetAxis, ValuePayload,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn union_of(doc: ProfileDoc, family: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Union {
            members: Bodies::Family(BodyRead::plain(Operand::Node(family))),
        },
    )
}

/// (judgements, solids, faces) of the union at `fused`.
fn fused(doc: &ProfileDoc, fused: RecipeNodeId) -> (u64, usize, usize) {
    let before = editor_core::EvalCounters::on_this_thread();
    let ev = run(doc);
    let judged = editor_core::EvalCounters::on_this_thread()
        .since(before)
        .judgements;
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &ev
        .value(fused)
        .unwrap_or_else(|| panic!("{:?}", ev.result(fused)))
        .payload
    else {
        panic!("the union is a body")
    };
    (
        judged,
        body.body().solids().count(),
        body.body().faces().count(),
    )
}

fn row_union(x0: f64, width: f64, pitch: f64, n: i64) -> (u64, usize, usize) {
    let doc = ProfileDoc::empty_derived("r1", Tol::witness());
    let (doc, b) = block(doc, (x0, x0 + width), (0.0, 1.0), 1.0);
    let (doc, _, row) = fixture::row(doc, b, ("k", n), TargetAxis::X, pitch);
    let (doc, u) = union_of(doc, row);
    fused(&doc, u)
}

#[test]
fn a_tangent_row_is_judged_and_fuses() {
    let got = row_union(0.0, 1.0, 1.0, 4);
    assert!(got.0 > 0, "{got:?}");
    assert_eq!(got.1, 1, "{got:?}");
}

#[test]
fn an_overlapping_row_is_judged_and_fuses() {
    let got = row_union(0.0, 1.0, 0.5, 4);
    assert!(got.0 > 0, "{got:?}");
    assert_eq!(got.1, 1, "{got:?}");
}

#[test]
fn a_far_tangent_row_is_judged_and_fuses() {
    for x0 in [1.0e3, 1.0e6, 1.0e8] {
        let got = row_union(x0, 1.0, 1.0, 3);
        assert!(got.0 > 0, "{x0}: {got:?}");
        assert_eq!(got.1, 1, "{x0}: {got:?}");
    }
}

/// A gap a hair wider than nothing, near and far: the row builds one
/// body (judged and fused) or three (apart), or refuses typed, and never
/// builds a wrong count.
#[test]
fn a_far_near_tangent_row_builds_one_body_or_three() {
    for (x0, gap) in [(1.0e6, 1e-9), (1.0e6, 1e-12), (0.0, 1e-12), (0.0, 1e-9)] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        let (doc, b) = block(doc, (x0, x0 + 1.0), (0.0, 1.0), 1.0);
        let (doc, _, row) = fixture::row(doc, b, ("k", 3), TargetAxis::X, 1.0 + gap);
        let (doc, u) = union_of(doc, row);
        let ev = run(&doc);
        match (ev.value(u).map(|v| &v.payload), ev.result(u)) {
            (Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })), _) => {
                let solids = body.body().solids().count();
                assert!(
                    solids == 1 || solids == 3,
                    "{x0} gap {gap}: {solids} solids"
                );
            }
            (None, Some(editor_core::NodeResult::Failed(_))) => {}
            (_, other) => panic!("{x0} gap {gap}: {other:?}"),
        }
    }
}

#[test]
fn a_turned_tangent_ring_is_judged_and_fuses() {
    for n in [4_i64, 3] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        // A block spanning the sector edge: x in 0..1 on the +x side of y = 0,
        // touching its neighbour on the quarter-turn ring along the z axis.
        let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
        let (doc, _, ring) = fixture::ring(doc, b, ("k", n), TargetAxis::Z);
        let (doc, u) = union_of(doc, ring);
        let got = fused(&doc, u);
        // Every pair meets (at least along the z axis), so none is
        // certified apart; a ring of 3 meets only along that edge.
        assert!(got.0 > 0, "{n}: {got:?}");
        assert_eq!(got.1, if n == 3 { 3 } else { 1 }, "{n}: {got:?}");
    }
}

/// Fast path against the slow path: three separately constructed
/// blocks, spelled, against a row family of one block. Same geometry,
/// same solid and face counts.
#[test]
fn the_fast_path_builds_what_the_slow_path_builds() {
    for (width, pitch) in [(0.8, 1.0), (1.0, 1.0), (1.5, 1.0)] {
        let fast = row_union(0.0, width, pitch, 3);
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        let (doc, a) = block(doc, (0.0, width), (0.0, 1.0), 1.0);
        let (doc, b) = block_on(doc, a, (pitch, pitch + width), (0.0, 1.0), 0.0, 1.0);
        let (doc, c) = block_on(
            doc,
            a,
            (2.0 * pitch, 2.0 * pitch + width),
            (0.0, 1.0),
            0.0,
            1.0,
        );
        let (doc, u) = insert(
            doc,
            Node::Union {
                members: Bodies::Spelled(vec![
                    BodyRead::plain(Operand::Node(a)),
                    BodyRead::plain(Operand::Node(b)),
                    BodyRead::plain(Operand::Node(c)),
                ]),
            },
        );
        let slow = fused(&doc, u);
        assert_eq!((fast.1, fast.2), (slow.1, slow.2), "w {width} p {pitch}");
    }
}

/// **A family whose members are not copies of one body**: a `⊔` with
/// an internal overlap, cut per member by a tool slid `scalar(k)·1.2`.
/// Member 0's tool removes the overlap; member 1's cuts elsewhere and
/// keeps it. Returns the interference findings at rest, product = the
/// family read by a `⊔`.
fn cut_family(overlap_kept_first: bool, product_plain: bool) -> usize {
    cut_family_n(overlap_kept_first, product_plain, 2)
}

fn cut_family_n(overlap_kept_first: bool, product_plain: bool, n: i64) -> usize {
    let doc = ProfileDoc::empty_derived("r1-carry", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (1.0, 3.0), (0.0, 1.0), 1.0);
    let ra = fixture::body_read(&doc, a.into());
    let (doc, aw) = insert(doc, fixture::stood(BodyRead::plain(ra)));
    let rb = fixture::body_read(&doc, b.into());
    let (doc, bw) = insert(doc, fixture::stood(BodyRead::plain(rb)));
    let (doc, asm) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Node(aw)),
                BodyRead::plain(Operand::Node(bw)),
            ]),
        },
    );
    let (start, step) = if overlap_kept_first {
        (2.2, -1.2)
    } else {
        (1.0, 1.2)
    };
    let (doc, tool) = block(doc, (start, start + 1.0), (-0.5, 1.5), 3.0);
    let slide = if n == 0 {
        fixture::len(0.0)
    } else {
        let (d, k) = fixture::index(doc.clone(), "k", n);
        let _ = &d;
        Formula::mul(fixture::scalar_of(k), fixture::len(step)).unwrap()
    };
    let doc = if n == 0 {
        doc
    } else {
        fixture::index(doc, "k", n).0
    };
    let placed = editor_core::Placed::of(
        &doc,
        Vec::new(),
        vec![
            (Coord::Slide(TargetAxis::X), slide),
            (Coord::Slide(TargetAxis::Z), fixture::len(-1.0)),
        ],
    );
    let rt = fixture::body_read(&doc, tool.into());
    let node = fixture::place_node(&doc, BodyRead::plain(rt), placed.bundle());
    let (doc, tools) = insert(doc, node);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: BodyRead::plain(Operand::Node(asm)),
            tool: BodyRead::plain(Operand::Node(tools)),
        },
    );
    if product_plain {
        finish(doc, Operand::Node(cut), cut)
    } else {
        let (d, p) = insert(
            doc,
            Node::Assemble {
                members: Bodies::Family(BodyRead::plain(Operand::Node(cut))),
            },
        );
        finish(d, Operand::Node(p), cut)
    }
}

fn finish(doc: ProfileDoc, product: Operand, cut: RecipeNodeId) -> usize {
    let doc = fixture::step(
        doc,
        DocEdit::SetProduct {
            product: Some(product),
        },
    )
    .0;
    let ev = run(&doc);
    assert!(ev.value(cut).is_some(), "{:?}", ev.result(cut));
    let findings = fixture::at_rest(&doc, &ev).unwrap_or_else(|e| panic!("{e}"));
    fixture::interference(&findings).len()
}

/// **A family's carry is every member's**: the count of findings is the
/// same whichever member keeps the `⊔`'s overlap, with the family the
/// product and with a `⊔` of it the product.
#[test]
fn a_familys_carry_is_every_members_whichever_order_they_stand_in() {
    assert_eq!(
        cut_family(true, true),
        cut_family(false, true),
        "plain product"
    );
    assert_eq!(
        cut_family(true, false),
        cut_family(false, false),
        "assembled product"
    );
}

/// **A count outside the counts an index holds refuses typed**: below
/// zero, and past the largest member index the evaluator reads
/// (`u32::MAX`); the union reading the family stands poisoned through
/// it. A count of zero is a family of no member.
#[test]
fn a_count_below_zero_refuses_typed() {
    for count in [-3_i64, i64::from(u32::MAX) + 1] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
        let (doc, _, row) = fixture::row(doc, b, ("k", count), TargetAxis::X, 2.0);
        let (doc, u) = union_of(doc, row);
        let ev = run(&doc);
        match ev.result(row) {
            Some(editor_core::NodeResult::Failed(editor_core::NodeError {
                kind:
                    editor_core::NodeErrorKind::IndexCountOutOfRange {
                        count: got, admits, ..
                    },
                ..
            })) => {
                assert_eq!(*got, count);
                assert_eq!(*admits, 0..=i64::from(u32::MAX));
            }
            other => panic!("{count}: {other:?}"),
        }
        assert!(
            matches!(ev.result(u), Some(editor_core::NodeResult::Poisoned { through }) if *through == row),
            "{count}: {:?}",
            ev.result(u)
        );
    }
    let doc = ProfileDoc::empty_derived("r1", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = fixture::row(doc, b, ("k", 0), TargetAxis::X, 2.0);
    let ev = run(&doc);
    let members = ev
        .value(row)
        .and_then(|v| v.members.as_ref())
        .map(|m| m.members.len());
    assert_eq!(members, Some(0), "{:?}", ev.result(row));
}

/// **One node reading one family twice**: plainly at `from` (a reader of
/// the same index: the member) and by an indexed read at `tool` (whole).
/// Rows of three blocks 1 wide, pitch 0.75 (each overlaps its neighbour);
/// `F[k] − F[1]`: member 0 keeps the 0.75 below member 1, member 1 is
/// empty, and member 2 keeps the 0.75 above it.
#[test]
fn one_node_reads_a_family_plainly_at_one_seat_and_indexed_at_another() {
    let doc = ProfileDoc::empty_derived("r1-twice", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = fixture::row(doc, b, ("k", 3), TargetAxis::X, 0.75);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: BodyRead::plain(Operand::Node(row)),
            tool: BodyRead::indexed(row, vec![Formula::count(1)]),
        },
    );
    let ev = run(&doc);
    let value = ev
        .value(cut)
        .unwrap_or_else(|| panic!("{:?}", ev.result(cut)));
    let members = &value.members.as_ref().expect("the cut is a family").members;
    assert_eq!(members.len(), 3);
    // Each member's extent along x, in its own body's coordinates.
    let span = |i: usize| match &members[i].1.payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            assert_eq!(body.body().solids().count(), 1, "member {i}");
            let xs: Vec<f64> = body.body().vertex_points().map(|(_, p)| p.x).collect();
            let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
            let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            Some((lo, hi))
        }
        ValuePayload::Boolean(BooleanValue::Empty) => None,
        other => panic!("member {i}: {}", other.kind_name()),
    };
    assert_eq!(span(0), Some((0.0, 0.75)));
    assert_eq!(span(1), None, "a member less itself is empty");
    assert_eq!(span(2), Some((0.25, 1.0)), "member 2 stands 1.5 on");
}

/// **A ring of unit blocks meeting on its axis edge**: no pair is
/// certified apart, so every pair is judged. Rings of 5 and 6 build one
/// solid, as the spelled union of the same turned copies does; a ring of
/// 8 refuses its borders, as the spelled union does
/// (`work/borders/a-ring-of-eight-blocks-meeting-on-one-edge-refuses-its-borders.md`).
#[test]
fn a_ring_of_unit_blocks_meeting_on_its_axis_judges_every_pair() {
    for n in [5_i64, 6, 8] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
        let (doc, _, ring) = fixture::ring(doc, b, ("k", n), TargetAxis::Z);
        let (doc, u) = union_of(doc, ring);
        let before = editor_core::EvalCounters::on_this_thread();
        let ev = run(&doc);
        let judged = editor_core::EvalCounters::on_this_thread()
            .since(before)
            .judgements;
        assert_eq!(judged, u64::try_from(n * (n - 1) / 2).unwrap(), "ring {n}");
        match (n, ev.value(u).map(|v| &v.payload)) {
            (5 | 6, Some(ValuePayload::Boolean(BooleanValue::Body { body, .. }))) => {
                assert_eq!(body.body().solids().count(), 1, "ring {n}");
            }
            (8, None) => assert!(
                matches!(
                    ev.result(u),
                    Some(editor_core::NodeResult::Failed(editor_core::NodeError {
                        kind: editor_core::NodeErrorKind::Naming(
                            editor_core::NamingError::Emission { .. }
                        ),
                        ..
                    }))
                ),
                "ring {n}: {:?}",
                ev.result(u)
            ),
            _ => panic!("ring {n}: {:?}", ev.result(u)),
        }
    }
}

/// **A body of two lumps whose copy's small lump stands inside the
/// other copy's big lump**: the boxes separate the copies' boundaries,
/// so the fast path judges no pair, and the union still builds what the
/// slow path builds — the big lump absorbing the small one inside it.
#[test]
fn a_lump_of_one_copy_inside_a_lump_of_another_builds_what_the_slow_path_builds() {
    let doc = ProfileDoc::empty_derived("two-lump", Tol::witness());
    let (doc, big) = block(doc, (0.0, 10.0), (0.0, 10.0), 10.0);
    let (doc, small) = block_on(doc, big, (20.0, 21.0), (4.0, 5.0), 4.0, 1.0);
    let lumps_of = |doc: ProfileDoc, big, small| {
        insert(
            doc,
            Node::Assemble {
                members: Bodies::Spelled(vec![
                    BodyRead::plain(Operand::Node(big)),
                    BodyRead::plain(Operand::Node(small)),
                ]),
            },
        )
    };
    let (doc, lumps) = lumps_of(doc, big, small);
    let (fast_doc, _, row) = fixture::row(doc.clone(), lumps, ("k", 2), TargetAxis::X, -16.0);
    let (fast_doc, fast) = union_of(fast_doc, row);
    // The slow path: the second copy built on its own, spelled beside the
    // first.
    let (doc, big2) = block_on(doc, big, (-16.0, -6.0), (0.0, 10.0), 0.0, 10.0);
    let (doc, small2) = block_on(doc, big, (4.0, 5.0), (4.0, 5.0), 4.0, 1.0);
    let (doc, lumps2) = lumps_of(doc, big2, small2);
    let (slow_doc, slow) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Node(lumps)),
                BodyRead::plain(Operand::Node(lumps2)),
            ]),
        },
    );
    let measure = |doc: &ProfileDoc, u: RecipeNodeId| {
        let before = editor_core::EvalCounters::on_this_thread();
        let ev = run(doc);
        let judged = editor_core::EvalCounters::on_this_thread()
            .since(before)
            .judgements;
        let Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })) =
            ev.value(u).map(|v| &v.payload)
        else {
            panic!("{:?}", ev.result(u))
        };
        let props = topo::mass_properties(
            &topo::AtRestBody::validate(body.body().clone(), Tol::witness()).expect("valid"),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
        .expect("the union measures");
        (judged, body.body().solids().count(), props.enclosure.volume)
    };
    let (fast_judged, fast_solids, fast_volume) = measure(&fast_doc, fast);
    let (_, slow_solids, slow_volume) = measure(&slow_doc, slow);
    assert_eq!(fast_judged, 0, "the copies are certified apart");
    assert_eq!((fast_solids, fast_volume), (slow_solids, slow_volume));
    assert_eq!((fast_solids, fast_volume), (3, 2001.0));
}

/// A family over `k = index(2)` of per-member `⊔`s: `tall[k] ⊔ lid`,
/// `tall[k]` an extrude of depth `(k + 1)·0.25` (or reversed), `lid` a
/// slab over z ∈ [0.3, 0.55]. Only the member whose extrude is 0.5 tall
/// overlaps its lid.
fn per_member_assembles(reversed: bool) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("r2-carry", Tol::witness());
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(2), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let kv = Formula::var(k, Dimension::Count);
    let count = if reversed {
        Formula::sub(Formula::count(2), kv).unwrap()
    } else {
        Formula::add(kv, Formula::count(1)).unwrap()
    };
    let depth = Formula::mul(Formula::count_to_scalar(count).unwrap(), fixture::len(0.25)).unwrap();
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let (doc, slab) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: fixture::len(0.25),
        },
    );
    let (doc, lid) = fixture::family_of(
        doc,
        slab,
        vec![(Coord::Slide(TargetAxis::Z), fixture::len(0.3))],
    );
    let target = fixture::space(&doc, slab);
    let (doc, tall) = insert(
        doc,
        Node::place(
            BodyRead::plain(Operand::Node(tall)),
            Operand::Var(target),
            editor_core::stand().bundle(),
        ),
    );
    let (doc, sq) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::indexed(tall, vec![Formula::var(k, Dimension::Count)]),
                BodyRead::plain(Operand::Node(lid)),
            ]),
        },
    );
    (doc, tall, sq)
}

fn interference_count(doc: &ProfileDoc) -> usize {
    let ev = fixture::run(doc, &Default::default());
    let findings = fixture::at_rest(doc, &ev).unwrap_or_else(|e| panic!("checks refused: {e}"));
    fixture::interference(&findings).len()
}

/// **A family of per-member `⊔`s** (members that are no copies of one
/// source): its overlap stands in one member only, and the at-rest
/// check reports it whichever member holds it — read as the product
/// alone, and beside the overlap between the members a second `⊔`
/// reading the family whole meets.
#[test]
fn a_per_member_overlap_is_reported_whichever_member_holds_it() {
    for reversed in [false, true] {
        let (doc, _, sq) = per_member_assembles(reversed);
        assert_eq!(interference_count(&doc), 1, "reversed {reversed}");
        let (doc, _) = insert(
            doc,
            Node::Assemble {
                members: Bodies::Family(sq.into()),
            },
        );
        assert_eq!(
            interference_count(&doc),
            2,
            "reversed {reversed}, read whole"
        );
    }
}

/// **A count edit serves the members it keeps from the memo**: after
/// 4 → 6, the first four members are the prior evaluation's (the same
/// `Arc`s), not rebuilt with equal keys. A construction per member,
/// whose body is its own.
#[test]
fn a_count_edit_serves_the_first_members_from_the_memo() {
    let doc = ProfileDoc::empty_derived("r2-reuse", Tol::witness());
    let (doc, k) = fixture::index_of(doc, "k", Formula::count(4), None);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let count = Formula::add(Formula::var(k, Dimension::Count), Formula::count(1)).unwrap();
    let depth = Formula::mul(Formula::count_to_scalar(count).unwrap(), fixture::len(0.25)).unwrap();
    let (doc, tall) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: depth,
        },
    );
    let first = fixture::run(&doc, &Default::default());
    let grown = fixture::recount(doc, k, Formula::count(6));
    let after = fixture::run_with_prior(&grown, &first);
    let bodies = |ev: &editor_core::Evaluation<f64>| -> Vec<editor_core::Framed<f64>> {
        ev.value(tall)
            .unwrap()
            .members
            .as_ref()
            .unwrap()
            .members
            .iter()
            .map(|(_, m)| match &m.payload {
                editor_core::ValuePayload::Body(b) => b.clone(),
                other => panic!("{}", other.kind_name()),
            })
            .collect()
    };
    let (b0, b1) = (bodies(&first), bodies(&after));
    assert_eq!(b1.len(), 6);
    let shared: Vec<bool> = (0..4)
        .map(|i| std::sync::Arc::ptr_eq(b0[i].shared(), b1[i].shared()))
        .collect();
    assert!(
        shared.iter().all(|&s| s),
        "the first four members are served, not rebuilt: {shared:?}"
    );
}
