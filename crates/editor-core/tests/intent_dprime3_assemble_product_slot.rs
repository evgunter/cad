//! INTENT stage 3 D′3 acceptance — `⊔` is an operator, the product is
//! a slot, a placement copies one source and a reader of one piece of a
//! copy holds a pick (`docs/INTENT-STAGE3-SPEC.md` §5′.3, tests 19, 25
//! and 26).
//!
//! A copy carries its source's names as they are, and an operator keys
//! each operand's names by its read, so no name spells a placement. The
//! census runs at each `⊔`, between its operands, whether or not the
//! product slot reads it, and its verdict is kept on the `⊔`'s value: a
//! copy carries it, and an outer `⊔` or a host reading the copy as one
//! arrival decides nothing inside it. A body of several parts is an
//! ordinary operand: an operator reading one keeps the meetings inside
//! it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;
use editor_core::{
    Bodies, BodyRead, DocEdit, DocumentId, EditError, EvalOptions, Evaluation, Node, Operand,
    ProfileDoc, RecipeNodeId, RoleSeg, SplitHalf, ValuePayload, VarId, apply,
};
use fixture::{len, on_frame, run, square};
use geom_core::Tol;

/// A block of half-width `half` centred at `(x, y)`, from `0` to
/// `height`, a construction in a space of its own.
fn block(doc: ProfileDoc, [x, y]: [f64; 2], half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(x, y, half)],
    );
    put(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// A block brought into `of`'s space, raised to stand on `z`.
fn beside(
    doc: ProfileDoc,
    of: RecipeNodeId,
    [x, y, z]: [f64; 3],
    half: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, body) = block(doc, [x, y], half, height);
    fixture::slid(doc, of, body, [0.0, 0.0, z])
}

/// Inserts `node` as written: the fixture's `insert` makes a world
/// placement join the product, and these rows say what the product is.
fn put(doc: ProfileDoc, node: editor_core::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
    let (doc, minted) = fixture::step(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
    );
    (doc, minted.expect("the insert mints the node"))
}

fn spelled(reads: &[RecipeNodeId]) -> Bodies<BodyRead<editor_core::Formula>> {
    Bodies::Spelled(reads.iter().map(|&r| r.into()).collect())
}

fn set_product(doc: &ProfileDoc, product: Option<Operand>) -> ProfileDoc {
    apply(doc, &DocEdit::SetProduct { product }, Tol::witness())
        .expect("the slot takes a body in the world")
        .doc
}

fn body_of(ev: &Evaluation<f64>, node: RecipeNodeId) -> topo::Body<f64> {
    match ev.value(node).map(|v| &v.payload) {
        Some(ValuePayload::Body(b)) => fixture::in_space(b),
        Some(ValuePayload::Boolean(editor_core::BooleanValue::Body { body, .. })) => {
            fixture::in_space(body)
        }
        other => panic!(
            "{node:?} reads one body, not {other:?}: {:?}",
            ev.result(node)
        ),
    }
}

fn bounds(body: &topo::Body<f64>) -> ([f64; 3], [f64; 3]) {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for (_, p) in body.points() {
        for (k, c) in p.to_array().into_iter().enumerate() {
            lo[k] = lo[k].min(c);
            hi[k] = hi[k].max(c);
        }
    }
    (lo, hi)
}

fn solids(body: &topo::Body<f64>) -> usize {
    body.solids().count()
}

fn near(a: ([f64; 3], [f64; 3]), b: ([f64; 3], [f64; 3])) -> bool {
    a.0.iter()
        .chain(&a.1)
        .zip(b.0.iter().chain(&b.1))
        .all(|(x, y)| (x - y).abs() < 1e-12)
}

fn doc(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

/// A copy of `node`'s value standing in the world.
fn in_world(
    doc: ProfileDoc,
    node: impl Into<BodyRead<editor_core::Formula>>,
) -> (ProfileDoc, RecipeNodeId) {
    put(doc, fixture::stood(node))
}

/// The `⊔` of `members`, inserted as written.
fn assembled(doc: ProfileDoc, members: &[RecipeNodeId]) -> (ProfileDoc, RecipeNodeId) {
    put(
        doc,
        Node::Assemble {
            members: spelled(members),
        },
    )
}

/// **The verdict the census at `node`'s door kept** on its value
/// (`editor_core::product::Census`).
fn kept<'e>(
    doc: &ProfileDoc,
    ev: &'e Evaluation<f64>,
    node: RecipeNodeId,
) -> &'e editor_core::Product<f64> {
    ev.value(node)
        .and_then(|value| value.carry.census.as_ref())
        .expect("the door keeps its census")
        .verdict(doc, Tol::witness())
        .expect("the census runs")
}

/// The `unproven-coincidence` findings the checks report.
fn unproven(doc: &ProfileDoc, ev: &Evaluation<f64>) -> usize {
    editor_core::run_checks(
        doc,
        ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run")
    .findings
    .iter()
    .filter(|f| f.check == editor_core::CheckId::UnprovenCoincidence)
    .count()
}

// ---- Test 19: picks ----

/// **Two placements of one body into one space, unioned, name each
/// operand by its read.** The union's names carried from an operand are
/// `From { read }` of that copy's read, and what each wraps is the
/// block's own name: a copy carries its source's names verbatim, so no
/// name spells the placement.
///
/// *Breaks if* a name spells the placement.
#[test]
fn two_copies_unioned_name_each_operand_by_its_read() {
    let doc = doc("dprime3-two-copies");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, near) = fixture::stood_in(doc, a, a);
    let (doc, far) = fixture::slid(doc, a, a, [0.5, 0.0, 0.0]);
    let (doc, union) = put(
        doc,
        Node::Union {
            members: spelled(&[near, far]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let reads = [doc.output(near, 0).unwrap(), doc.output(far, 0).unwrap()];
    let table = ev
        .table(doc.output(union, 0).unwrap())
        .expect("the union's table");
    let mut seen = std::collections::BTreeSet::new();
    for (name, _) in table.iter() {
        let Some(RoleSeg::From { read, of }) = name.path.first() else {
            continue;
        };
        assert!(reads.contains(read), "keyed by a member's read: {name:?}");
        assert_eq!(of.node, a, "a copy carries the block's names: {name:?}");
        seen.insert(*read);
    }
    assert_eq!(seen.len(), 2, "both copies' names reach the union");
}

/// **A pick of one piece of a copy of `bolt ⊔ plate` reads the bolt's
/// copy**; picking it twice gives two variables; a pick through a `∪`
/// refuses at the door.
///
/// *Breaks if* a reader reads one piece of a copy without a pick.
#[test]
fn a_pick_reads_one_piece_and_two_picks_are_two_variables() {
    let doc = doc("dprime3-pick");
    let (doc, bolt) = block(doc, [0.0; 2], 0.2, 1.0);
    let (doc, plate) = beside(doc, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (doc, seat) = put(
        doc,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    let (doc, place) = put(doc, fixture::stood(seat));
    let bolt_var = doc.output(bolt, 0).unwrap();
    let pick = || BodyRead::plain(Operand::pick(Operand::Node(place), vec![bolt_var]));
    let (doc, first) = put(doc, fixture::stood(pick()));
    let (doc, second) = put(doc, fixture::stood(pick()));
    let source = |at: RecipeNodeId| match doc.node(at) {
        Some(Node::Place { source, .. }) => source.read,
        other => panic!("a placement, not {other:?}"),
    };
    let (one, two) = (source(first), source(second));
    assert_ne!(one, two, "a pick authored twice is two variables");
    for var in [one, two] {
        let pick = doc.pick(var).expect("the source is a pick");
        assert_eq!(pick.of, vec![bolt_var], "the pick's path is the bolt");
    }
    let ev = run(&doc, &EvalOptions::default());
    for at in [first, second] {
        assert!(
            near(
                bounds(&body_of(&ev, at)),
                ([-0.2, -0.2, 0.0], [0.2, 0.2, 1.0])
            ),
            "the pick reads the bolt's copy alone"
        );
    }
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[bolt, plate]),
        },
    );
    let (doc, through) = put(doc, fixture::stood(fused));
    let refused = fixture::insert_refused(
        &doc,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(through),
            vec![bolt_var],
        ))),
    );
    assert!(
        matches!(refused, EditError::PickPath { .. }),
        "a pick through a union refuses: {refused:?}"
    );
}

// ---- Test 25: each operator decides its operands' meetings ----

/// **`A ⊔ A` is a finding**: the census at the `⊔` reads two pieces in
/// one place, and the at-rest check reports it.
#[test]
fn one_body_assembled_twice_is_an_interference_finding() {
    let doc = doc("dprime3-twice");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, copy) = in_world(doc, a);
    let (doc, twice) = assembled(doc, &[copy, copy]);
    let doc = set_product(&doc, Some(Operand::Node(twice)));
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        !product.census.is_empty(),
        "the census reads the two pieces meeting"
    );
    let (_, findings) = outcome(&doc, &ev);
    assert!(findings > 0, "the census's findings travel to the check");
}

/// **`A ∪ A` is `A`.**
#[test]
fn one_body_unioned_twice_is_that_body() {
    let doc = doc("dprime3-union-twice");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, twice) = put(
        doc,
        Node::Union {
            members: spelled(&[a, a]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, twice);
    assert_eq!(solids(&body), 1);
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [0.5, 0.5, 1.0])));
}

/// **`(p ⊔ q) − t` keeps two pieces**: a `⊔`'s result is an ordinary
/// operand, and a tool through both pieces cuts each.
#[test]
fn a_cut_through_an_assemble_keeps_its_pieces() {
    let doc = doc("dprime3-cut");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, q) = beside(doc, p, [2.0, 0.0, 0.0], 0.5, 1.0);
    let (doc, both) = put(
        doc,
        Node::Assemble {
            members: spelled(&[p, q]),
        },
    );
    let (doc, t) = beside(doc, p, [1.0, 0.0, 0.5], 2.0, 1.0);
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: both.into(),
            tool: t.into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, cut);
    assert_eq!(solids(&body), 2, "both pieces survive the cut");
    assert!(
        near(bounds(&body), ([-0.5, -0.5, 0.0], [2.5, 0.5, 0.5])),
        "{:?}",
        bounds(&body)
    );
}

/// **`h1 ⊔ h2` of a split's halves records the cut, and `h1 ∪ h2`
/// glues it.** The halves rest on the one tool plane, so the census at
/// the `⊔` records the meeting and the coincidence door proves it: no
/// `unproven-coincidence` finding. The union of the halves is the block
/// again.
#[test]
fn a_splits_halves_assembled_record_the_cut_and_unioned_glue_it() {
    let doc = doc("dprime3-halves");
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    // Split in the world, so the halves are the product's to read.
    let (doc, a) = in_world(doc, a);
    let tool = fixture::plane_on(&doc, a, [0.0, 0.0, 0.5], [0.0, 0.0, 1.0]);
    let (doc, split) = put(
        doc,
        Node::Split {
            target: a.into(),
            tool,
        },
    );
    let halves = Bodies::Spelled(
        [SplitHalf::Above, SplitHalf::Below]
            .map(|h| BodyRead::plain(Operand::Var(doc.output(split, h.port()).unwrap())))
            .to_vec(),
    );
    let (doc, glued) = put(
        doc,
        Node::Union {
            members: halves.clone(),
        },
    );
    let (doc, assembled) = put(doc, Node::Assemble { members: halves });
    let doc = set_product(&doc, Some(Operand::Node(assembled)));
    let ev = run(&doc, &EvalOptions::default());
    let body = body_of(&ev, glued);
    assert_eq!(solids(&body), 1, "the union glues the cut");
    assert!(near(bounds(&body), ([-0.5, -0.5, 0.0], [0.5, 0.5, 1.0])));
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the census leaves nothing: {:?}",
        product.census
    );
    assert!(
        !product.coincidences.is_empty() && !product.contacts.is_empty(),
        "the meeting is recorded: rows and the record citing them"
    );
    crate::fixture::could_look(&doc, &ev).expect("the halves assemble at rest");
    let report = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run");
    assert!(report.findings.is_empty(), "the cut is proved: {report}");
}

/// A bolt standing on a plate, `bolt ⊔ plate`: two pieces that meet
/// over the bolt's foot.
fn seat(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let doc = doc(label);
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, seat) = put(
        doc,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    (doc, seat, bolt, plate)
}

/// The reads `node`'s value holds pieces by, in order.
fn piece_reads(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<VarId> {
    ev.value(node)
        .and_then(|v| v.carry.pieces.as_ref())
        .map(|p| p.pieces.iter().map(|piece| piece.read).collect())
        .unwrap_or_default()
}

/// The boolean refusal `node` failed with, by its kind.
fn refused(ev: &Evaluation<f64>, node: RecipeNodeId) -> Option<topo::BooleanErrorKind> {
    match ev.node_error(node).map(|e| &e.kind) {
        Some(editor_core::NodeErrorKind::Boolean(e)) => Some(e.kind()),
        _ => None,
    }
}

/// **A cut through a seat cuts each piece and keeps both**: a pin down
/// through the bolt and the plate, through the face they meet over,
/// makes `(bolt − pin) ⊔ (plate − pin)`. Its pieces are keyed by the
/// reads the `⊔` read them through, and its names carry each piece's
/// names keyed by that read. The same cut of the `⊔` read whole is the
/// kernel's `VertexReadTwice`: a pin vertex pierces the bolt's foot and
/// the plate's top, two faces the boolean reads apart. A placement of
/// the seat is cut the same way where it stands.
///
/// *Breaks if* a cut re-decides the meeting inside the `⊔` it reads.
#[test]
fn a_cut_through_a_seat_cuts_each_piece_and_keeps_both() {
    let (doc, seat, bolt, plate) = seat("dprime3-seat-cut");
    let (doc, pin) = beside(doc, plate, [0.0, 0.0, -0.5], 0.1, 2.0);
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: seat.into(),
            tool: pin.into(),
        },
    );
    let (doc, placed) = in_world(doc, seat);
    let (doc, pin_placed) = in_world(doc, pin);
    let (doc, cut_placed) = put(
        doc,
        Node::Subtract {
            from: placed.into(),
            tool: pin_placed.into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let reads = [bolt, plate].map(|n| doc.output(n, 0).unwrap());
    for node in [cut, cut_placed] {
        let body = body_of(&ev, node);
        assert_eq!(solids(&body), 2, "the bolt and the plate, each holed");
        assert!(
            near(bounds(&body), ([-1.0, -1.0, 0.0], [1.0, 1.0, 1.2])),
            "{:?}",
            bounds(&body)
        );
        assert_eq!(piece_reads(&ev, node), reads, "keyed by the `⊔`'s reads");
        let table = ev
            .table(doc.output(node, 0).unwrap())
            .expect("the cut is named");
        for read in reads {
            assert!(
                table.iter().any(|(name, _)| matches!(
                    name.path.as_slice(),
                    [RoleSeg::From { read: r, .. }] if *r == read
                )),
                "a name keyed by {read:?}"
            );
        }
    }
}

/// **A cut that empties a piece drops it, and one that empties every
/// piece is the empty result**: `(p ⊔ q) − t` with `t` swallowing `q`
/// keeps `p` alone, and with `t` swallowing both keeps nothing.
#[test]
fn a_cut_drops_the_pieces_it_empties() {
    let doc = doc("dprime3-cut-empties");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, q) = beside(doc, p, [2.0, 0.0, 0.0], 0.5, 1.0);
    let (doc, both) = put(
        doc,
        Node::Assemble {
            members: spelled(&[p, q]),
        },
    );
    let (doc, over_q) = beside(doc, p, [2.0, 0.0, -1.0], 1.0, 3.0);
    let (doc, over_all) = beside(doc, p, [1.0, 0.0, -1.0], 3.0, 3.0);
    let (doc, one) = put(
        doc,
        Node::Subtract {
            from: both.into(),
            tool: over_q.into(),
        },
    );
    let (doc, none) = put(
        doc,
        Node::Subtract {
            from: both.into(),
            tool: over_all.into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(solids(&body_of(&ev, one)), 1);
    assert_eq!(piece_reads(&ev, one), [doc.output(p, 0).unwrap()]);
    assert!(
        matches!(
            ev.value(none).map(|v| &v.payload),
            Some(ValuePayload::Boolean(editor_core::BooleanValue::Empty))
        ),
        "{:?}",
        ev.result(none)
    );
}

/// **`∩` distributes over `⊔` on either side**: `(bolt ⊔ plate) ∩ t`
/// and `t ∩ (bolt ⊔ plate)`, `t` a post standing through the bolt's
/// foot, keep the part of each piece inside `t`, two pieces, where the
/// `⊔` read whole is the kernel's `VertexReadTwice`.
#[test]
fn a_meet_with_a_seat_meets_each_piece() {
    let (doc, seat, bolt, plate) = seat("dprime3-seat-meet");
    let (doc, t) = beside(doc, plate, [0.0, 0.0, 0.1], 0.1, 0.3);
    let (doc, after) = put(
        doc,
        Node::Intersect {
            members: spelled(&[seat, t]),
        },
    );
    let (doc, before) = put(
        doc,
        Node::Intersect {
            members: spelled(&[t, seat]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    let reads = [bolt, plate].map(|n| doc.output(n, 0).unwrap());
    for node in [after, before] {
        let body = body_of(&ev, node);
        assert_eq!(solids(&body), 2, "{:?}", ev.result(node));
        assert!(
            near(bounds(&body), ([-0.1, -0.1, 0.1], [0.1, 0.1, 0.4])),
            "{:?}",
            bounds(&body)
        );
        assert_eq!(piece_reads(&ev, node), reads);
    }
}

/// **A `⊔` read as a cut's tool is cut by whole, today**: `t − (bolt ⊔
/// plate)`, `t` a post standing through the bolt's foot, refuses as the
/// kernel's `VertexReadTwice`.
/// `t − (a ⊔ b)` is `(t − a) − b`, a fold of cuts rather than a cut per
/// piece; it lands with the kernel reading a declared seat as one
/// interface
/// (`work/cleave/a-vertex-piercing-two-declared-coincident-faces-reads-as-one.md`),
/// when this row builds.
#[test]
fn a_seat_as_a_cuts_tool_is_cut_by_whole() {
    let (doc, seat, _, plate) = seat("dprime3-seat-tool");
    let (doc, t) = beside(doc, plate, [0.0, 0.0, 0.1], 0.1, 0.3);
    let (doc, cut) = put(
        doc,
        Node::Subtract {
            from: t.into(),
            tool: seat.into(),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(
        refused(&ev, cut),
        Some(topo::BooleanErrorKind::VertexReadTwice),
        "{:?}",
        ev.result(cut)
    );
}

/// **`(bolt ⊔ plate) ∪ bead` keeps the `⊔`'s pieces**: a bead against
/// the bolt alone fuses with it and leaves the plate a piece of its
/// own, the bolt–plate meeting not decided again.
///
/// A bead bridging bolt and plate makes one solid touching itself where
/// they met. Today the union refuses where the kernel reads the bolt's
/// foot and the plate's top apart: `VertexReadTwice`, a bead vertex
/// piercing both. When the kernel reads a declared seat as one
/// interface
/// (`work/cleave/a-vertex-piercing-two-declared-coincident-faces-reads-as-one.md`)
/// this row's bridged union is the typed self-touch refusal naming the
/// seat's pair, and its assert says so.
#[test]
fn a_seat_unioned_with_a_bead_keeps_its_pieces_and_a_bridge_is_read_twice() {
    let (doc, seat, _, plate) = seat("dprime3-bead");
    let (doc, side) = beside(doc, plate, [0.25, 0.0, 0.6], 0.1, 0.2);
    let (doc, bridge) = beside(doc, plate, [0.25, 0.0, 0.1], 0.1, 0.3);
    let (doc, kept) = put(
        doc,
        Node::Union {
            members: spelled(&[seat, side]),
        },
    );
    let (doc, fused) = put(
        doc,
        Node::Union {
            members: spelled(&[seat, bridge]),
        },
    );
    let ev = run(&doc, &EvalOptions::default());
    assert_eq!(
        solids(&body_of(&ev, kept)),
        2,
        "the bolt and its bead, and the plate"
    );
    let Some(value) = ev.value(kept).filter(|v| {
        matches!(
            v.payload,
            ValuePayload::Boolean(editor_core::BooleanValue::Body { .. })
        )
    }) else {
        panic!("the union builds: {:?}", ev.result(kept))
    };
    let contacts = &value.carry.contacts;
    assert_eq!(
        contacts.face_face().count(),
        1,
        "the seat's record, the `⊔`'s decision, carried through the union: {contacts:?}"
    );
    assert_eq!(
        refused(&ev, fused),
        Some(topo::BooleanErrorKind::VertexReadTwice),
        "a bridged seat refuses where the kernel reads the seat's two faces apart: {:?}",
        ev.node_error(fused)
    );
}

// ---- Test 26: the product is a slot ----

/// Three disjoint pieces in one space, `p ⊔ bolt ⊔ nut`, the product,
/// and a fourth body placed in the world that the slot does not read.
fn three_pieces(label: &str) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = doc(label);
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [2.0, 0.0, 0.0], 0.2, 1.0);
    let (doc, nut) = beside(doc, p, [4.0, 0.0, 0.0], 0.3, 0.3);
    let (doc, three) = put(
        doc,
        Node::Assemble {
            members: spelled(&[p, bolt, nut]),
        },
    );
    let (doc, copy) = in_world(doc, three);
    let doc = set_product(&doc, Some(Operand::Node(copy)));
    let (doc, spare) = block(doc, [9.0, 0.0], 0.5, 0.5);
    let (doc, unread) = in_world(doc, spare);
    (doc, three, unread)
}

/// **The product is what the slot reads**: `p ⊔ bolt ⊔ nut` gathers
/// those three pieces and nothing else, however many other bodies are
/// placed in the world; a slot reading nothing is the empty product.
///
/// *Breaks if* the product is derived from which bodies go unread.
#[test]
fn the_product_is_what_the_slot_reads() {
    let (doc, _, unread) = three_pieces("dprime3-slot");
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product(&doc, &ev, Tol::witness()).expect("it gathers");
    assert_eq!(solids(&product), 3, "the three pieces the slot reads");
    assert!(
        bounds(&product).1[0] < 9.0,
        "the body placed at x = 9 is not read, so not in the product"
    );
    assert!(
        ev.value(unread).is_some(),
        "the unread placement evaluates all the same"
    );
    let empty = set_product(&doc, None);
    let ev = run(&empty, &EvalOptions::default());
    assert!(matches!(
        editor_core::product(&empty, &ev, Tol::witness()),
        Err(editor_core::ProductError::EmptyProduct)
    ));
}

/// **An instance of a document defines one body**: the copy of its
/// product, all three pieces, and a pick of one piece reads it.
#[test]
fn an_instance_defines_one_body_of_its_parts_product() {
    let (part, three, _) = three_pieces("dprime3-instance-part");
    let bolt_var = match part.node(three) {
        Some(Node::Assemble {
            members: Bodies::Spelled(reads),
        }) => reads[1].read,
        other => panic!("the `⊔`, not {other:?}"),
    };
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = doc("dprime3-instance");
    let (doc, instance) = put(
        doc,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    assert_eq!(doc.outputs(instance).len(), 1, "one output, one body");
    let (doc, picked) = put(
        doc,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(instance),
            vec![bolt_var],
        ))),
    );
    let opts = fixture::resolver::with_resolver(store);
    let ev = run(&doc, &opts);
    assert_eq!(
        solids(&body_of(&ev, instance)),
        3,
        "the part's three pieces"
    );
    assert!(
        near(
            bounds(&body_of(&ev, picked)),
            ([1.8, -0.2, 0.0], [2.2, 0.2, 1.0])
        ),
        "the pick reads the bolt: {:?}",
        bounds(&body_of(&ev, picked))
    );
}

/// **An instance's census decisions are its part's, reported once**:
/// a bolt resting on a plate, `p ⊔ bolt`, is the part's product, and
/// the census at that `⊔` records the touch, which the part's
/// `unproven-coincidence` check reports. An instance of the part
/// carries that decision: the host's census decides the touch inside
/// the instance's copy no second time, and the host reports the part's
/// findings, each the part's, carried, and none of its own.
///
/// *Breaks if* an instance's census rows are decided again in the host.
#[test]
fn an_instances_census_decisions_are_its_parts() {
    let doc = doc("dprime3-carried-part");
    let (doc, p) = block(doc, [0.0; 2], 0.5, 0.2);
    let (doc, bolt) = beside(doc, p, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, p) = in_world(doc, p);
    let (doc, bolt) = in_world(doc, bolt);
    let (part, seat) = assembled(doc, &[p, bolt]);
    let part = set_product(&part, Some(Operand::Node(seat)));
    let ev = run(&part, &EvalOptions::default());
    let decided = editor_core::product_recorded(&part, &ev, Tol::witness()).expect("it gathers");
    assert!(
        !decided.coincidences.is_empty(),
        "the part's `⊔` records the touch"
    );
    let own = unproven(&part, &ev);
    assert!(own > 0, "and the part reports it");

    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let doc = self::doc("dprime3-carried-host");
    let (doc, instance) = put(
        doc,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let doc = set_product(&doc, Some(Operand::Node(instance)));
    let ev = run(&doc, &fixture::resolver::with_resolver(store));
    let host = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        host.coincidences.is_empty() && host.census.is_empty(),
        "the host decides nothing inside the instance's copy"
    );
    let reported = editor_core::run_checks(
        &doc,
        &ev,
        &editor_core::ChecksConfig::default(),
        Tol::witness(),
    )
    .expect("the checks run")
    .findings
    .into_iter()
    .filter(|f| f.check == editor_core::CheckId::UnprovenCoincidence)
    .collect::<Vec<_>>();
    assert_eq!(reported.len(), own, "the part's touch, once: {reported:?}");
    assert!(
        reported.iter().all(|f| matches!(
            &f.evidence,
            editor_core::CheckEvidence::CarriedCoincidence { finding }
                if finding.route.through == instance
        )),
        "each the part's, carried through the instance, none the host's: {reported:?}"
    );
    crate::fixture::could_look(&doc, &ev).expect("the host assembles");
}

/// **A placement of `bolt ⊔ plate` made the product is at rest** (fix
/// pass row (a)): the copy carries the census its source's door kept,
/// the seat's record with it, so the product's census reads the copy as
/// one arrival a door already decided and discards what it finds inside
/// it. The seat is reported once, at the `⊔` that decided it, as when
/// the slot reads the `⊔` itself.
///
/// *Breaks if* a placement of a `⊔` is censused again where it lands.
#[test]
fn a_placed_seat_made_the_product_is_at_rest() {
    let doc = doc("dprime3-placed-seat");
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, plate) = in_world(doc, plate);
    let (doc, bolt) = in_world(doc, bolt);
    let (doc, seat) = assembled(doc, &[bolt, plate]);
    let (doc, placed) = in_world(doc, seat);
    let direct = set_product(&doc, Some(Operand::Node(seat)));
    let doc = set_product(&doc, Some(Operand::Node(placed)));
    let ev = run(&doc, &EvalOptions::default());
    let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty() && product.coincidences.is_empty(),
        "nothing inside the copy is decided again: {:?}",
        product.census
    );
    let decided = kept(&doc, &ev, seat);
    assert_eq!(
        decided.contacts.face_face().count(),
        1,
        "the `⊔` records the seat"
    );
    assert_eq!(
        product.contacts.face_face().count(),
        1,
        "and the copy carries that record"
    );
    crate::fixture::could_look(&doc, &ev).expect("a placed seat is at rest");
    let ev_direct = run(&direct, &EvalOptions::default());
    crate::fixture::could_look(&direct, &ev_direct).expect("the seat is at rest");
    assert_eq!(
        unproven(&doc, &ev),
        unproven(&direct, &ev_direct),
        "the seat is reported once, at its `⊔`, however the product reads it"
    );
}

/// A plate in the world, a bolt standing on it and a nut resting on it
/// beside the bolt, each a world copy.
fn seat_and_nut(label: &str) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = doc(label);
    let (doc, plate) = block(doc, [0.0; 2], 1.0, 0.2);
    let (doc, bolt) = beside(doc, plate, [0.0, 0.0, 0.2], 0.2, 1.0);
    let (doc, nut) = beside(doc, plate, [0.6, 0.0, 0.2], 0.15, 0.3);
    let (doc, plate) = in_world(doc, plate);
    let (doc, bolt) = in_world(doc, bolt);
    let (doc, nut) = in_world(doc, nut);
    (doc, [plate, bolt, nut])
}

/// **`(p ⊔ b) ⊔ n` is `p ⊔ b ⊔ n`** (fix pass row (b)): the bolt's seat
/// is the inner `⊔`'s decision and the nut's the outer's, so the nested
/// spelling records what the flat one records, each touch once. The
/// outer census reads `p ⊔ b` as one arrival its own door decided: it
/// decides the seat no second time and counts it no second time.
///
/// *Breaks if* an outer `⊔` re-decides or re-counts an inner one's
/// meeting.
#[test]
fn a_nested_assemble_records_what_the_flat_one_records_once() {
    let (flat, [p, b, n]) = seat_and_nut("dprime3-flat");
    let (flat, three) = assembled(flat, &[p, b, n]);
    let flat = set_product(&flat, Some(Operand::Node(three)));
    let ev_flat = run(&flat, &EvalOptions::default());
    let flat_product =
        editor_core::product_recorded(&flat, &ev_flat, Tol::witness()).expect("it gathers");

    let (nested, [p, b, n]) = seat_and_nut("dprime3-nested");
    let (nested, inner) = assembled(nested, &[p, b]);
    let (nested, outer) = assembled(nested, &[inner, n]);
    let nested = set_product(&nested, Some(Operand::Node(outer)));
    let ev = run(&nested, &EvalOptions::default());
    let product = editor_core::product_recorded(&nested, &ev, Tol::witness()).expect("it gathers");
    let inner_rows = kept(&nested, &ev, inner).coincidences.len();

    assert_eq!(
        flat_product.contacts.face_face().count(),
        2,
        "the seat and the nut"
    );
    assert_eq!(
        product.contacts.face_face().count(),
        2,
        "the outer carries the seat's record and records the nut's"
    );
    assert!(inner_rows > 0, "the inner `⊔` decides the seat");
    assert_eq!(
        product.coincidences.len() + inner_rows,
        flat_product.coincidences.len(),
        "each touch decided once: the outer decides none of the inner's"
    );
    assert!(product.census.is_empty() && flat_product.census.is_empty());
    crate::fixture::could_look(&flat, &ev_flat).expect("the flat stand is at rest");
    crate::fixture::could_look(&nested, &ev).expect("the nested stand is at rest");
    assert_eq!(
        unproven(&nested, &ev),
        unproven(&flat, &ev_flat),
        "and the checks report each touch once"
    );
}

/// What the at-rest check answers: whether it could look at every
/// pair, and how many findings it reports, carried ones included.
fn outcome(doc: &ProfileDoc, ev: &Evaluation<f64>) -> (bool, usize) {
    let findings = crate::fixture::at_rest(doc, ev).unwrap_or_else(|e| panic!("it gathers: {e}"));
    (
        findings.iter().all(|f| !f.evidence.could_not_look()),
        findings.len(),
    )
}

/// Two blocks overlapping in general position (no face of one on a face
/// of the other), each a world copy, in `doc`.
fn overlapping(doc: ProfileDoc) -> (ProfileDoc, [RecipeNodeId; 2]) {
    let (doc, a) = block(doc, [0.0; 2], 0.5, 1.0);
    let (doc, again) = beside(doc, a, [0.3, 0.2, 0.25], 0.5, 1.0);
    let (doc, a) = in_world(doc, a);
    let (doc, again) = in_world(doc, again);
    (doc, [a, again])
}

/// **An overlap inside an operand is reported once** (fix pass row (b),
/// interference): two blocks that overlap, `(a ⊔ a') ⊔ n`, report the
/// overlap as `a ⊔ a' ⊔ n` does, at the inner `⊔`, and the outer
/// census finds nothing inside its operand.
#[test]
fn an_overlap_inside_a_nested_assemble_is_reported_once() {
    let (base, [a, again]) = overlapping(doc("dprime3-nested-overlap"));
    let (base, n) = block(base, [5.0, 0.0], 0.3, 0.3);
    let (base, n) = in_world(base, n);
    let (flat, three) = assembled(base.clone(), &[a, again, n]);
    let flat = set_product(&flat, Some(Operand::Node(three)));
    let ev_flat = run(&flat, &EvalOptions::default());
    let (nested, inner) = assembled(base, &[a, again]);
    let (nested, outer) = assembled(nested, &[inner, n]);
    let nested = set_product(&nested, Some(Operand::Node(outer)));
    let ev = run(&nested, &EvalOptions::default());
    let product = editor_core::product_recorded(&nested, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the outer census finds nothing inside its operand: {:?}",
        product.census
    );
    let flat = outcome(&flat, &ev_flat);
    assert!(flat.1 > 0, "the overlap is a finding: {flat:?}");
    assert_eq!(outcome(&nested, &ev), flat, "reported once, nested or not");
}

/// **An overlap read along shared edge lines is discarded inside an
/// arrival too** (R1's delta probe,
/// `r1_nested_assemble_interference_reported_once`, on the census's own
/// pinwheel, `topo`'s `coincident_line_wedges`): two prisms whose
/// profiles both hold the unit square, their arms swept opposite ways,
/// double the square's material, and the census reports that on the
/// square's corner lines as `CoincidentWedgesOverlap` rows.
/// `(a ⊔ a') ⊔ n` decides those at the inner `⊔`, and the outer census,
/// and the census of a host reading an instance of `a ⊔ a'`, find none
/// of them.
///
/// *Breaks if* the in-arrival discard cannot place a
/// coincident-wedges finding on its two solids
/// (`checks::at_rest::finding_solids`), and so keeps it at the outer
/// door.
#[test]
fn an_overlap_along_shared_lines_inside_an_arrival_is_reported_once() {
    let wedges = |census: &[topo::ValidationError]| {
        census
            .iter()
            .filter(|f| matches!(f, topo::ValidationError::CoincidentWedgesOverlap { .. }))
            .count()
    };
    let prism = |doc: ProfileDoc, outline: &[(f64, f64)], height: f64| {
        let (doc, profile) = on_frame(
            doc,
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![outline.to_vec()],
        );
        put(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(height),
            },
        )
    };
    let base = doc("dprime3-nested-lines");
    let (base, a) = prism(
        base,
        &[
            (-2.0, -2.0),
            (3.0, -2.0),
            (1.0, 0.0),
            (1.0, 1.0),
            (3.0, 3.0),
            (-2.0, 3.0),
            (0.0, 1.0),
            (0.0, 0.0),
        ],
        1.0,
    );
    let (base, again) = prism(
        base,
        &[
            (0.0, 0.0),
            (1.0, 0.0),
            (3.0, -2.0),
            (3.0, 3.0),
            (1.0, 1.0),
            (0.0, 1.0),
            (-2.0, 3.0),
            (-2.0, -2.0),
        ],
        1.25,
    );
    let (base, again) = fixture::slid(base, a, again, [0.0, 0.0, 0.25]);
    let (base, n) = beside(base, a, [10.0, 0.0, 0.0], 0.3, 0.3);
    let (base, a) = in_world(base, a);
    let (base, again) = in_world(base, again);
    let (base, n) = in_world(base, n);

    let (part, both) = assembled(base.clone(), &[a, again]);
    let part = set_product(&part, Some(Operand::Node(both)));
    let ev = run(&part, &EvalOptions::default());
    let inner = editor_core::product_recorded(&part, &ev, Tol::witness()).expect("it gathers");
    assert!(
        wedges(&inner.census) > 0,
        "the overlap is read along the shared lines at the inner door: {:?}",
        inner.census
    );

    let (nested, inner) = assembled(base, &[a, again]);
    let (nested, outer) = assembled(nested, &[inner, n]);
    let nested = set_product(&nested, Some(Operand::Node(outer)));
    let ev = run(&nested, &EvalOptions::default());
    let product = editor_core::product_recorded(&nested, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the outer census finds nothing inside its operand: {:?}",
        product.census
    );

    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let (host, instance) = put(
        doc("dprime3-nested-lines-host"),
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let host = set_product(&host, Some(Operand::Node(instance)));
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the host decides nothing inside the instance's copy: {:?}",
        product.census
    );
}

/// **A kept verdict is reused only with its cone** (R1's N-B): the
/// verdict a `⊔`'s value keeps reads the document only inside the
/// `⊔`'s cone (what each member read resolves to, and an instance's
/// reference through it), and the gather reads the document's own id
/// and its assertions afresh. So an evaluation that reuses the `⊔`'s
/// value after an edit outside its cone reads the same verdict a fresh
/// evaluation runs, and an edit inside the cone runs a new one.
///
/// *Breaks if* the kept verdict reads the document outside the `⊔`'s
/// cone.
#[test]
fn a_kept_verdict_is_reused_only_with_its_cone() {
    let evaluate = |doc: &ProfileDoc, prior: Option<&Evaluation<f64>>| {
        editor_core::evaluate::<f64>(
            doc,
            prior,
            &editor_core::CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
    let said = |doc: &ProfileDoc, ev: &Evaluation<f64>| {
        let product = editor_core::product_recorded(doc, ev, Tol::witness()).expect("it gathers");
        format!(
            "{:?} {:?} {:?}",
            product.document, product.coincidences, product.census
        )
    };
    let (base, [a, again]) = overlapping(doc("dprime3-kept-verdict"));
    let (base, both) = assembled(base, &[a, again]);
    let base = set_product(&base, Some(Operand::Node(both)));
    let first = evaluate(&base, None);
    let _ = said(&base, &first);
    let kept = |ev: &Evaluation<f64>| {
        ev.value(both)
            .and_then(|value| value.carry.census.clone())
            .expect("the `⊔` keeps its census")
    };

    // Outside the cone: a block nothing in the `⊔` reads.
    let (outside, _) = block(base.clone(), [9.0, 0.0], 0.5, 1.0);
    let again_ev = evaluate(&outside, Some(&first));
    assert!(
        Arc::ptr_eq(&kept(&first), &kept(&again_ev)),
        "the `⊔`'s value, and the verdict it keeps, are reused"
    );
    assert_eq!(
        said(&outside, &again_ev),
        said(&outside, &evaluate(&outside, None)),
        "the reused verdict is the one a fresh evaluation runs"
    );

    // Inside the cone: the second block's copy moved off the first.
    let inside = fixture::pin(
        base,
        again,
        &editor_core::Frame::translation([5.0, 0.0, 0.0]),
        &EvalOptions::default(),
    );
    let moved = evaluate(&inside, Some(&first));
    assert!(
        !Arc::ptr_eq(&kept(&first), &kept(&moved)),
        "an edit in the cone runs the census again"
    );
    assert_eq!(
        said(&inside, &moved),
        said(&inside, &evaluate(&inside, None))
    );
}

/// **A host does not re-decide an instance's self-overlap** (fix pass
/// row (c)): a part whose product `a ⊔ a'` overlaps itself reports the
/// overlap; an instance of it carries that finding up as the part's,
/// and the host's census, reading the instance's copy as one arrival
/// the part's census decided, finds nothing of its own inside it.
///
/// *Breaks if* a host's census re-decides an instance's meetings.
#[test]
fn a_host_does_not_redecide_an_instances_self_overlap() {
    let (part, [a, again]) = overlapping(doc("dprime3-overlap-part"));
    let (part, both) = assembled(part, &[a, again]);
    let part = set_product(&part, Some(Operand::Node(both)));
    let ev = run(&part, &EvalOptions::default());
    let (_, own) = outcome(&part, &ev);
    assert!(own > 0, "the part reports its overlap");

    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = doc("dprime3-overlap-host");
    let (host, instance) = put(
        host,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let host = set_product(&host, Some(Operand::Node(instance)));
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    let product = editor_core::product_recorded(&host, &ev, Tol::witness()).expect("it gathers");
    assert!(
        product.census.is_empty(),
        "the host decides nothing inside the instance's copy: {:?}",
        product.census
    );
    assert_eq!(
        outcome(&host, &ev).1,
        own,
        "the part's findings, carried up once as the part's, and none of the host's own"
    );
}

// ---- Every door holds the slot and the picks ----

/// **The slot reads a body in the world, at every door**: a re-target
/// of the copy the slot reads, out of the world, is reported
/// (`Maintenance::ProductOutsideWorld`, as a re-target leaving reads in
/// two spaces is), the gather refuses the slot typed, and the state
/// saves and loads as the doors left it.
#[test]
fn a_retarget_of_the_products_copy_out_of_the_world_is_reported_and_refused() {
    let d = doc("dprime3-retarget");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, copy) = in_world(d, a);
    let d = set_product(&d, Some(Operand::Node(copy)));
    let (d, space) = put(d, Node::Space);
    let space = d.output(space, 0).unwrap();
    let moved = apply(
        &d,
        &DocEdit::SetParam {
            node: copy,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Space),
            value: Operand::Var(space).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("a re-target is admitted");
    assert!(
        moved.maintenance.iter().any(|row| matches!(
            row,
            editor_core::Maintenance::ProductOutsideWorld { read, .. }
                if Some(read.id()) == d.product()
        )),
        "the edit reports the slot's body leaving the world: {:?}",
        moved.maintenance
    );
    let ev = run(&moved.doc, &EvalOptions::default());
    assert!(
        matches!(
            editor_core::product(&moved.doc, &ev, Tol::witness()),
            Err(editor_core::ProductError::ProductOutsideWorld { .. })
        ),
        "the gather refuses a slot outside the world"
    );
    let text = editor_core::persist::save(&moved.doc, &[], Tol::witness()).unwrap();
    let loaded =
        editor_core::persist::load(&text, Tol::witness()).expect("what an edit leaves loads");
    assert_eq!(loaded.doc.product(), moved.doc.product());
    // `SetProduct` itself refuses a read outside the world.
    let refused = apply(
        &moved.doc,
        &DocEdit::SetProduct {
            product: Some(Operand::Node(copy)),
        },
        Tol::witness(),
    );
    assert!(
        matches!(
            refused,
            Err(EditError::ProductRead {
                fault: editor_core::ProductReadFault::Space { .. },
                ..
            })
        ),
        "the slot's own door refuses a body outside the world: {:?}",
        refused.map(|a| a.doc.product())
    );
}

/// **The load door holds the slot as the edit doors do**: a stored
/// document whose slot reads a variable it never minted refuses typed; one
/// reading a body outside the world, a state an edit can leave, loads,
/// and the gather refuses it.
#[test]
fn the_load_door_holds_the_slot_as_the_edit_doors_do() {
    let d = doc("dprime3-load-slot");
    let (d, p) = block(d, [0.0; 2], 0.5, 0.5);
    let (d, copy) = in_world(d, p);
    let d = set_product(&d, Some(Operand::Node(copy)));
    let text = editor_core::persist::save(&d, &[], Tol::witness()).unwrap();
    let ser = |v: VarId| serde_json::to_string(&v).unwrap();
    let product = ser(d.product().unwrap());
    let needles = [
        format!("\"product\":{product}"),
        format!("\"product\": {product}"),
    ];
    assert!(
        needles.iter().any(|n| text.contains(n.as_str())),
        "the slot is on the wire"
    );
    let swapped = |to: &str| {
        needles.iter().fold(text.clone(), |t, n| {
            t.replace(n.as_str(), &n.replace(&product, to))
        })
    };
    let unminted = editor_core::persist::load(&swapped(&ser(VarId::new(999, 5))), Tol::witness());
    assert!(
        matches!(
            unminted,
            Err(editor_core::PersistError::Snapshot(
                editor_core::SnapshotError::ProductRead { .. }
            ))
        ),
        "a slot reading no variable loads: {:?}",
        unminted.map(|l| l.doc.product())
    );
    let inner = editor_core::persist::load(&swapped(&ser(d.output(p, 0).unwrap())), Tol::witness())
        .expect("a slot reading a body outside the world loads");
    let ev = run(&inner.doc, &EvalOptions::default());
    assert!(
        matches!(
            editor_core::product(&inner.doc, &ev, Tol::witness()),
            Err(editor_core::ProductError::ProductOutsideWorld { .. })
        ),
        "the gather refuses it"
    );
}

/// **A pick of a pick walks the piece the inner pick kept**: through a
/// copy of the bolt's pick, the bolt is a piece and the plate is not.
#[test]
fn a_pick_through_a_pick_walks_the_piece_it_kept() {
    let d = doc("dprime3-pick-pick");
    let (d, bolt) = block(d, [0.0; 2], 0.2, 1.0);
    let (d, plate) = beside(d, bolt, [2.0, 0.0, 0.0], 0.5, 0.2);
    let (d, seat) = put(
        d,
        Node::Assemble {
            members: spelled(&[bolt, plate]),
        },
    );
    let (d, place) = in_world(d, seat);
    let [bolt_var, plate_var] = [bolt, plate].map(|n| d.output(n, 0).unwrap());
    let pick = |at: RecipeNodeId, of: Vec<VarId>| {
        fixture::stood(BodyRead::plain(Operand::pick(Operand::Node(at), of)))
    };
    let (d, kept) = put(d, pick(place, vec![bolt_var]));
    let (d, again) = in_world(d, kept);
    let insert = |node| {
        apply(
            &d,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
    };
    let plate_through_bolt = insert(pick(again, vec![plate_var]));
    assert!(
        matches!(
            plate_through_bolt,
            Err(EditError::PickPath { at: Some(0), .. })
        ),
        "the plate is no piece of the bolt's copy: {:?}",
        plate_through_bolt.map(|a| a.record.minted)
    );
    // The bolt's own copy holds no `⊔`, so no step names a piece of it.
    let bolt_through_bolt = insert(pick(again, vec![bolt_var]));
    assert!(
        matches!(
            bolt_through_bolt,
            Err(EditError::PickPath { at: Some(0), .. })
        ),
        "the bolt's copy has no pieces: {:?}",
        bolt_through_bolt.map(|a| a.record.minted)
    );
}

/// **A pick of a `⊔` reading one body twice, by that body, is
/// ambiguous and refuses**: two pieces answer to the read.
#[test]
fn a_pick_by_a_read_the_assemble_reads_twice_refuses() {
    let d = doc("dprime3-pick-twice");
    let (d, a) = block(d, [0.0; 2], 0.5, 1.0);
    let (d, twice) = put(
        d,
        Node::Assemble {
            members: spelled(&[a, a]),
        },
    );
    let (d, place) = in_world(d, twice);
    let a_var = d.output(a, 0).unwrap();
    let picked = apply(
        &d,
        &DocEdit::InsertNode {
            node: Box::new(fixture::stood(BodyRead::plain(Operand::pick(
                Operand::Node(place),
                vec![a_var],
            )))),
            fresh: Vec::new(),
        },
        Tol::witness(),
    );
    assert!(
        matches!(picked, Err(EditError::PickPath { at: Some(0), .. })),
        "two pieces answer to the read: {:?}",
        picked.map(|a| a.record.minted)
    );
}

/// **A pick into an instance's part is the evaluation's to answer**: the
/// edit door reads nothing across the seam (`DocEdit::UpdateReference`),
/// so a path naming no piece of the part is admitted and refuses at
/// evaluation, typed.
#[test]
fn a_pick_into_a_part_naming_no_piece_refuses_at_evaluation() {
    let part = doc("dprime3-pick-part");
    let (part, bolt) = block(part, [0.0; 2], 0.2, 1.0);
    let (part, bolt) = in_world(part, bolt);
    let part = set_product(&part, Some(Operand::Node(bolt)));
    let mut store = fixture::resolver::PartStore::default();
    let part_ref = store.insert(part, Tol::witness());
    let host = doc("dprime3-pick-part-host");
    let (host, inst) = put(
        host,
        Node::instantiate_part(
            part_ref,
            Operand::Var(VarId::WORLD),
            editor_core::stand().bundle(),
        ),
    );
    let (host, read) = put(
        host,
        fixture::stood(BodyRead::plain(Operand::pick(
            Operand::Node(inst),
            vec![VarId::WORLD],
        ))),
    );
    let ev = run(&host, &fixture::resolver::with_resolver(store));
    assert!(
        matches!(
            ev.node_error(read).map(|e| &e.kind),
            Some(editor_core::NodeErrorKind::NoSuchPiece { .. })
        ),
        "a pick naming no piece of the part: {:?}",
        ev.node_error(read)
    );
}
