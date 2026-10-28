//! R1 review probes (PR 5954, G).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on};
use crate::fixture::{self, insert};
use editor_core::{
    Bodies, BodyRead, BooleanValue, Coord, DocEdit, Evaluation, Formula, Node, Operand, ProfileDoc,
    RecipeNodeId, TargetAxis, ValuePayload,
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
fn r1_tangent_row_is_judged_and_fuses() {
    let got = row_union(0.0, 1.0, 1.0, 4);
    eprintln!("tangent row: {got:?}");
    assert!(got.0 > 0, "{got:?}");
    assert_eq!(got.1, 1, "{got:?}");
}

#[test]
fn r1_overlapping_row_is_judged_and_fuses() {
    let got = row_union(0.0, 1.0, 0.5, 4);
    eprintln!("overlap row: {got:?}");
    assert!(got.0 > 0, "{got:?}");
    assert_eq!(got.1, 1, "{got:?}");
}

#[test]
fn r1_far_tangent_row_is_judged_and_fuses() {
    for x0 in [1.0e3, 1.0e6, 1.0e8] {
        let got = row_union(x0, 1.0, 1.0, 3);
        eprintln!("far tangent row at {x0}: {got:?}");
        assert!(got.0 > 0, "{x0}: {got:?}");
        assert_eq!(got.1, 1, "{x0}: {got:?}");
    }
}

#[test]
fn r1_far_near_tangent_row() {
    // A gap a hair wider than nothing, far away: either judged or apart,
    // but never a wrong count.
    for (x0, gap) in [(1.0e6, 1e-9), (1.0e6, 1e-12), (0.0, 1e-12), (0.0, 1e-9)] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        let (doc, b) = block(doc, (x0, x0 + 1.0), (0.0, 1.0), 1.0);
        let (doc, _, row) = fixture::row(doc, b, ("k", 3), TargetAxis::X, 1.0 + gap);
        let (doc, u) = union_of(doc, row);
        let ev = run(&doc);
        let got = match ev.value(u).map(|v| &v.payload) {
            Some(ValuePayload::Boolean(BooleanValue::Body { body, .. })) => {
                format!("solids {}", body.body().solids().count())
            }
            _ => format!("{:?}", ev.result(u)).chars().take(200).collect(),
        };
        eprintln!("far near-tangent row at {x0} gap {gap}: {got:?}");
    }
}

#[test]
fn r1_turned_tangent_ring_is_judged_and_fuses() {
    for n in [4_i64, 3] {
        let doc = ProfileDoc::empty_derived("r1", Tol::witness());
        // A block spanning the sector edge: x in 0..1 on the +x side of y = 0,
        // touching its neighbour on the quarter-turn ring along the z axis.
        let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
        let (doc, _, ring) = fixture::ring(doc, b, ("k", n), TargetAxis::Z);
        let (doc, u) = union_of(doc, ring);
        let got = fused(&doc, u);
        eprintln!("ring {n}: {got:?}");
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
fn r1_fast_equals_slow() {
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
        eprintln!("w {width} p {pitch}: fast {fast:?} slow {slow:?}");
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
    let v = ev
        .value(cut)
        .unwrap_or_else(|| panic!("{:?}", ev.result(cut)));
    let empty = Vec::new();
    let ms = v.members.as_ref().map_or(&empty, |m| &m.members);
    for (at, member) in ms {
        let census = member.carry.census.as_ref().map(|c| format!("{c:?}").len());
        eprintln!(
            "member {at:?}: parts {} census {census:?}",
            member.carry.parts
        );
    }
    let findings = fixture::at_rest(&doc, &ev).unwrap_or_else(|e| panic!("{e}"));
    let inter = fixture::interference(&findings);
    let short = |v: editor_core::VarId| {
        let at = doc.operation_of(v);
        format!(
            "{}@{:?}",
            v.0.ordinal(),
            at.and_then(|n| doc.node(n))
                .map(|n| format!("{n:?}").chars().take(12).collect::<String>())
        )
    };
    for f in &inter {
        eprintln!(
            "  {} m{} x {} m{} quiet {:?}",
            short(f.a.var),
            f.a.member,
            short(f.b.var),
            f.b.member,
            f.quiet
        );
    }
    eprintln!(
        "{} interference findings; cut var {:?}",
        inter.len(),
        doc.output(cut, 0).map(|v| v.0.ordinal())
    );
    inter.len()
}

/// RED at 95fe8503: the family's carry is its first member's, so the
/// count of findings depends on which member keeps the `⊔`'s overlap
/// (member 0: 6, member 1: 5; with a family `⊔` product 7 against 6).
#[test]
fn r1_family_carry_is_symmetric_in_member_order() {
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

/// A negative count holds no member, silently; main refused a count
/// below one (`CountOutOfRange`).
#[test]
fn r1_negative_count() {
    let doc = ProfileDoc::empty_derived("r1", Tol::witness());
    let (doc, b) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, _, row) = fixture::row(doc, b, ("k", -3), TargetAxis::X, 2.0);
    let (doc, u) = union_of(doc, row);
    let ev = run(&doc);
    eprintln!(
        "row: {:?}",
        ev.result(row)
            .map(|r| format!("{r:?}").chars().take(300).collect::<String>())
    );
    eprintln!(
        "union: {:?}",
        ev.result(u)
            .map(|r| format!("{r:?}").chars().take(300).collect::<String>())
    );
    // RED at 95fe8503: a count below zero holds no member and the union
    // is `Empty`, silently; main refused a count below one.
    assert!(ev.value(row).is_none(), "a negative count refuses");
}

/// **One node reading one family twice**: plainly at `from` (a reader of
/// the same index: the member) and by an indexed read at `tool` (whole).
/// Rows of three blocks 1 wide, pitch 0.75 (each overlaps its neighbour);
/// `F[k] − F[1]`: member 0 keeps x in 0..0.75, member 2 keeps 1.75..2.5.
#[test]
fn r1_one_node_reads_a_family_plainly_and_indexed() {
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
    eprintln!("cut reach {:?}", doc.node_reach(cut));
    let ev = run(&doc);
    match ev.value(cut) {
        // RED at 95fe8503: `MemberRefused { WrongOperand { expected: "body",
        // found: "instances" } }` — `from` reads the same index, so it reads
        // the member, but the overlay leaves the family whole for the node.
        None => panic!(
            "cut refused: {}",
            format!("{:?}", ev.result(cut))
                .chars()
                .take(600)
                .collect::<String>()
        ),
        Some(v) => {
            let members = v.members.as_ref();
            eprintln!(
                "cut payload kind {:?} members {:?}",
                std::mem::discriminant(&v.payload),
                members.map(|m| m.members.len())
            );
            if let Some(m) = members {
                for (at, mv) in &m.members {
                    let s = match &mv.payload {
                        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
                            let xs: Vec<f64> =
                                body.body().vertex_points().map(|(_, p)| p.x).collect();
                            let lo = xs.iter().cloned().fold(f64::INFINITY, f64::min);
                            let hi = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                            format!("solids {} x {lo}..{hi}", body.body().solids().count())
                        }
                        other => format!("{:?}", std::mem::discriminant(other)),
                    };
                    eprintln!("  member {at:?}: {s}");
                }
            }
        }
    }
}

/// The ring-of-6 refusal: how many pairs were judged (15 = no pair
/// skipped by the fast path).
#[test]
fn r1_ring_of_six_judgements() {
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
        eprintln!(
            "ring {n}: judged {judged} of {} ok {}",
            n * (n - 1) / 2,
            ev.value(u).is_some()
        );
    }
}
