//! LIB-PLACEDUNION — the ratified A′ group boolean, pinned.
//!
//! `docs/GROUP-BOOLEAN-DESIGN.md`'s acceptance list, row for row: the
//! die tour's tool collapses to one node; a cavity face's name is one
//! `Instance(i)` segment instead of a twenty-deep `FromA`/`FromB`
//! stack; the heat sink's fin union moves INTO the document with its
//! memoized recompute intact; and the disjointness gate refuses BOTH
//! an overlapping arrangement and a box-touching-but-genuinely-disjoint
//! one, each with its typed posture.
//!
//! The two register payoffs live in the corpus (`heat_sink_fins`,
//! `die_tool`), so they carry every standard row by membership — every
//! ε, the Interval lane, persistence round-trip, latency. What is here
//! is what the corpus rows do not say: the twin equalities against the
//! chains these documents replace, and the refusals.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    BooleanOp, DocEdit, EditError, Expr, Frame, Node, NodeErrorKind, NodeResult, PatternKind,
    PlacementRuleFault, ProfileDoc, RecipeNodeId, RoleSeg, SlotId, ValuePayload, apply,
};
use fixture::{ang, desc, len, scl};

use corpus::{body_of, documents, eval, failures};
use geom_core::Tol;

/// The fin prototype the heat-sink documents share, in a fresh
/// document: the extrude alone, no base, no group.
fn fin_only() -> (ProfileDoc, RecipeNodeId) {
    let mut r = corpus::Recorder::new();
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.1875],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.25, 0.125),
            (0.4375, 0.125),
            (0.4375, 0.875),
            (0.25, 0.875),
        ]],
    )));
    let fin = r.insert(Node::Extrude {
        profile: p,
        distance: len(0.8125),
    });
    (r.doc, fin)
}

/// The corpus document by name.
fn doc_named(name: &str) -> corpus::CorpusDoc {
    documents()
        .into_iter()
        .find(|d| d.name == name)
        .unwrap_or_else(|| panic!("corpus document {name}"))
}

// ---------------------------------------------------------------------
// Payoff 1 — the heat sink's fin union, in the document
// ---------------------------------------------------------------------

/// **One node, one body.** The grouped document says with ONE
/// `PlacedUnion` what `heat_sink` says with a `Pattern` plus five
/// `Transform` + five `Boolean` nodes — and the group's value is an
/// ordinary `Body`, which is precisely what `body_operand` refused a
/// `Pattern`'s `Instances` for (the F4 note).
#[test]
fn the_fin_group_is_one_node_and_one_body() {
    let d = doc_named("heat_sink_fins");
    let groups = d
        .doc
        .order()
        .iter()
        .filter(|&&id| matches!(d.doc.node(id), Some(Node::PlacedUnion { .. })))
        .count();
    assert_eq!(groups, 1, "the whole fin group is one node");
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let root = d.result.expect("the group is the document's result");
    let payload = &ev.value(root).expect("the group evaluated").payload;
    assert!(
        matches!(payload, ValuePayload::Body(_)),
        "a group is body-denoting, not Instances: {}",
        payload.kind_name()
    );
    let body = body_of(&ev, root);
    // Five disjoint fins fuse into the PROTOTYPE's solid structure —
    // one solid, five shells — which is what the pairwise-union chain
    // this replaces also produces, and the only shape the seamed
    // boolean path accepts as an operand.
    assert_eq!(body.solids().count(), 1, "one solid");
    assert_eq!(body.shells().count(), 5, "five shells, one per placement");
    assert!(
        topo::validate_geometric(body, Tol::witness()).is_ok(),
        "the group validates"
    );
}

/// **The twin equality.** The group and the Transform + Boolean chain
/// it replaces are the same solid, bit for bit on both mass oracles —
/// the design's "same oracles, byte-level" promise, asserted with `==`
/// against a chain built here from the SAME prototype.
#[test]
fn the_fin_group_equals_the_transform_union_chain() {
    const PITCH: f64 = 0.3125;
    let (base_doc, fin) = fin_only();

    // The chain: five placed copies, unioned pairwise. The fins are
    // separated, so every union takes the disjoint path.
    let mut chain = base_doc.clone();
    let mut acc: Option<RecipeNodeId> = None;
    for i in 0..5 {
        let tr = apply(
            &chain,
            &DocEdit::InsertNode {
                node: Node::Transform {
                    input: fin,
                    translation: [len(f64::from(i) * PITCH), len(0.0), len(0.0)],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: ang(0.0),
                },
            },
            Tol::witness(),
        )
        .expect("insert transform");
        let placed = tr.record.minted.expect("minted");
        chain = tr.doc;
        acc = Some(match acc {
            None => placed,
            Some(a) => {
                let u = apply(
                    &chain,
                    &DocEdit::InsertNode {
                        node: Node::Boolean {
                            op: BooleanOp::Union,
                            a,
                            b: placed,
                            declare: None,
                        },
                    },
                    Tol::witness(),
                )
                .expect("insert union");
                let id = u.record.minted.expect("minted");
                chain = u.doc;
                id
            }
        });
    }

    // The group: one node, same prototype, same rule.
    let grouped = apply(
        &base_doc,
        &DocEdit::InsertNode {
            node: Node::placed_union(
                fin,
                Expr::count(5),
                PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(PITCH),
                },
            )
            .expect("a stepped rule takes a count"),
        },
        Tol::witness(),
    )
    .expect("insert group");
    let group = grouped.record.minted.expect("minted");

    let ev_chain = eval::<f64>(&chain);
    let ev_group = eval::<f64>(&grouped.doc);
    assert!(failures(&ev_chain).is_empty(), "{:?}", failures(&ev_chain));
    assert!(failures(&ev_group).is_empty(), "{:?}", failures(&ev_group));
    let a = body_of(&ev_chain, acc.expect("the chain's root"));
    let b = body_of(&ev_group, group);
    let (ma, mb) = (
        topo::mass_properties(a, Tol::witness()).expect("chain mass"),
        topo::mass_properties(b, Tol::witness()).expect("group mass"),
    );
    assert_eq!(ma.volume, mb.volume, "same volume, bit for bit");
    assert_eq!(ma.surface_area, mb.surface_area, "same area, bit for bit");
    assert_eq!(
        (a.faces().count(), a.edges().count(), a.vertices().count()),
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        "same census"
    );
    assert_eq!(a.solids().count(), b.solids().count(), "same solid count");
}

/// **The memoized recompute survives the collapse.** A parameter edit
/// upstream of the group recomputes the group and nothing else; an
/// edit that touches nothing the group depends on reuses it. The
/// group's content key covers the rule, so neither answer is luck.
#[test]
fn the_group_recomputes_exactly_when_its_inputs_move() {
    let d = doc_named("heat_sink_fins");
    let first = eval::<f64>(&d.doc);
    // Nothing changed: every node is reused.
    let again = editor_core::evaluate::<f64>(
        &d.doc,
        Some(&first),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(
        again.recomputed, 0,
        "an unchanged document recomputes nothing"
    );
    // The fin's distance moves: the fin and the group recompute, the
    // profile does not.
    let bumped = d.bumped();
    let after = editor_core::evaluate::<f64>(
        &bumped,
        Some(&first),
        &editor_core::CancelToken::new(),
        &editor_core::EvalOptions::default(),
        Tol::witness(),
    );
    assert!(failures(&after).is_empty(), "{:?}", failures(&after));
    assert!(after.reused > 0, "the profile is reused across the bump");
    assert!(
        after.recomputed >= 2,
        "the fin extrude and the group both recompute"
    );
}

// ---------------------------------------------------------------------
// Payoff 2 — the die tour's tool, in one node
// ---------------------------------------------------------------------

/// **The tool collapses.** `die_tool` carries ONE group node and NOT
/// ONE union node, where the tour's recipe spends N−1 unions and N
/// transforms on the same shape — and the collapsed tool is still a
/// legal boolean operand: the document subtracts it from the cube.
#[test]
fn the_die_tool_is_one_node_and_still_cuts() {
    let d = doc_named("die_tool");
    let mut groups = 0;
    let mut unions = 0;
    let mut transforms = 0;
    for &id in d.doc.order() {
        match d.doc.node(id) {
            Some(Node::PlacedUnion { .. }) => groups += 1,
            Some(Node::Boolean {
                op: BooleanOp::Union,
                ..
            }) => unions += 1,
            Some(Node::Transform { .. }) => transforms += 1,
            _ => {}
        }
    }
    assert_eq!((groups, unions, transforms), (1, 0, 0));
    let ev = eval::<f64>(&d.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let pipped = body_of(&ev, d.result.expect("the pipped die"));
    assert!(topo::validate_geometric(pipped, Tol::witness()).is_ok());
    // Six cavities: each contributes its own faces to the one solid.
    assert_eq!(pipped.solids().count(), 1);
}

/// **A cavity face's name is one segment deep.** The tour's pairwise
/// tool buries the FIRST ball's faces under one `FromA`/`FromB`
/// segment per union; the group gives every instance the SAME
/// one-segment `Instance(i)` qualifier, whatever the placement count,
/// and a name row addresses "instance i's face" directly.
#[test]
fn every_instance_is_one_instance_segment_deep() {
    let d = doc_named("die_tool");
    let ev = eval::<f64>(&d.doc);
    let tool = d
        .doc
        .order()
        .iter()
        .copied()
        .find(|&id| matches!(d.doc.node(id), Some(Node::PlacedUnion { .. })))
        .expect("the group node");
    let table = &ev.value(tool).expect("the group evaluated").name_table;
    let mut per_instance = [0usize; 6];
    for (name, _) in table.iter() {
        match name.path.as_slice() {
            // The group's OWN output body, minted like every other
            // body-producing op's.
            [RoleSeg::OutputBody] => {}
            [RoleSeg::Instance { i, of }] => {
                per_instance[*i as usize] += 1;
                assert!(
                    of.path.len() <= 2,
                    "the wrapped ball-local name stays shallow: {of:?}"
                );
            }
            other => panic!("a group name is one Instance segment: {other:?}"),
        }
    }
    assert!(
        per_instance.iter().all(|&n| n > 0 && n == per_instance[0]),
        "every instance carries the same rows: {per_instance:?}"
    );
}

// ---------------------------------------------------------------------
// The disjointness gate — both postures
// ---------------------------------------------------------------------

/// Builds a one-node document: a unit box, grouped at `frames`.
fn boxes_at(frames: Vec<Frame>) -> (ProfileDoc, RecipeNodeId) {
    let mut r = corpus::Recorder::new();
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    )));
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let group = r.insert(Node::placed_union_at(solid, frames));
    (r.doc, group)
}

/// The `(i, j)` a group's disjointness refusal named.
fn uncertified_pair(doc: &ProfileDoc, id: RecipeNodeId) -> (usize, usize) {
    match eval::<f64>(doc).nodes.get(&id) {
        Some(NodeResult::Failed(e)) => match e.kind {
            NodeErrorKind::PlacementsUncertified { i, j } => (i, j),
            ref other => panic!("expected an uncertified-placements refusal, got {other:?}"),
        },
        other => panic!("the group must refuse, got {other:?}"),
    }
}

/// **Overlap refuses, typed, naming the pair.** Two placements of one
/// unit box a quarter apart interpenetrate; nothing is built, and the
/// refusal names `(0, 1)`.
#[test]
fn overlapping_placements_refuse_typed() {
    let (doc, group) = boxes_at(vec![Frame::IDENTITY, Frame::translation([0.25, 0.0, 0.0])]);
    assert_eq!(uncertified_pair(&doc, group), (0, 1));
}

/// **Box-touching-but-genuinely-disjoint refuses too — and that is the
/// posture, not a bug.** The second box is rotated 45° about `z` and
/// pushed along `x` far enough that the two solids do not meet, but
/// its axis-aligned box still overlaps the first's. The certificate is
/// sufficient-not-necessary, so the honest answer is the SAME typed
/// refusal, refinable by a sharper predicate later — never a silent
/// maybe on a door that asserts nothing about its operands.
#[test]
fn touching_boxes_over_disjoint_solids_refuse_the_same_way() {
    use std::f64::consts::FRAC_PI_4;
    // The unit square turned 45° about the origin becomes the diamond
    // with corners (0,0), (√2/2, √2/2), (0, √2), (−√2/2, √2/2); shifted
    // to (1.6, 0.5) its leftmost corner is (0.893, 1.207) and its
    // lower-left edge is the line y = 2.1 − x, which sits at y = 1.1
    // where it reaches x = 1. The first box tops out at y = 1, so the
    // two solids are clear of each other by 0.1 — while their
    // axis-aligned boxes overlap over x ∈ [0.893, 1], y ∈ [0.5, 1].
    let rotated = Frame::rotate_then_translate([0.0, 0.0, 1.0], FRAC_PI_4, [1.6, 0.5, 0.0]);
    let (doc, group) = boxes_at(vec![Frame::IDENTITY, rotated]);
    assert_eq!(
        uncertified_pair(&doc, group),
        (0, 1),
        "a box overlap refuses even when the solids are clear"
    );
    // Pushed further out, the same arrangement certifies — so the
    // refusal above is the BOX test speaking, not a broken predicate.
    let clear = Frame::rotate_then_translate([0.0, 0.0, 1.0], FRAC_PI_4, [3.0, 0.5, 0.0]);
    let (doc, group) = boxes_at(vec![Frame::IDENTITY, clear]);
    let ev = eval::<f64>(&doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    assert_eq!(body_of(&ev, group).shells().count(), 2);
}

// ---------------------------------------------------------------------
// The rule vocabulary
// ---------------------------------------------------------------------

/// **The circular rule places around a datum axis** — the rule
/// vocabulary is `PatternKind`'s, reused whole, and the stepped map is
/// literally the pattern node's (`stepped_map`).
#[test]
fn a_circular_group_places_around_a_datum_axis() {
    let mut r = corpus::Recorder::new();
    let axis = r.insert(Node::Datum(editor_core::Datum::Axis {
        origin: [len(0.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    let p = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(4.0, -0.5), (5.0, -0.5), (5.0, 0.5), (4.0, 0.5)]],
    )));
    let solid = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    let group = r.insert(
        Node::placed_union(
            solid,
            Expr::count(4),
            PatternKind::Circular {
                axis,
                step: ang(std::f64::consts::FRAC_PI_2),
            },
        )
        .expect("a stepped rule takes a count"),
    );
    let ev = eval::<f64>(&r.doc);
    assert!(failures(&ev).is_empty(), "{:?}", failures(&ev));
    let body = body_of(&ev, group);
    assert_eq!(body.shells().count(), 4);
    assert!(topo::validate_geometric(body, Tol::witness()).is_ok());
}

/// **"How many placements" has exactly one spelling.** The edit door
/// refuses both two-answer states: an explicit rule paired with a
/// count slot, and an explicit rule on `Pattern` at all (whose count
/// is a non-optional field).
#[test]
fn the_edit_door_refuses_a_two_spelling_count() {
    let (doc, fin) = fin_only();
    let with_count = Node::PlacedUnion {
        input: fin,
        count: Some(Expr::count(2)),
        kind: PatternKind::Explicit(vec![Frame::IDENTITY, Frame::translation([9.0, 0.0, 0.0])]),
    };
    assert!(matches!(
        apply(
            &doc,
            &DocEdit::InsertNode { node: with_count },
            Tol::witness()
        ),
        Err(EditError::PlacementRuleMismatch { .. })
    ));
    let pattern_explicit = Node::Pattern {
        input: fin,
        count: Expr::count(2),
        kind: PatternKind::Explicit(vec![Frame::IDENTITY]),
    };
    assert!(matches!(
        apply(
            &doc,
            &DocEdit::InsertNode {
                node: pattern_explicit
            },
            Tol::witness(),
        ),
        Err(EditError::PlacementRuleMismatch { .. })
    ));
    // …and the constructor cannot build the first state at all.
    assert!(
        Node::<editor_core::ProfileProgram>::placed_union(
            fin,
            Expr::count(2),
            PatternKind::Explicit(vec![Frame::IDENTITY]),
        )
        .is_none()
    );
}

/// **An explicit rule has no expression slots**, and a stepped one has
/// exactly the pattern node's — the slot API is the one place the two
/// nodes could have drifted, so it is pinned directly.
#[test]
fn the_slot_surface_follows_the_rule() {
    let (_, fin) = fin_only();
    let explicit: Node<editor_core::ProfileProgram> =
        Node::placed_union_at(fin, vec![Frame::IDENTITY]);
    assert!(explicit.slots().is_empty());
    assert!(explicit.expr(SlotId::Count).is_none());
    let stepped: Node<editor_core::ProfileProgram> = Node::placed_union(
        fin,
        Expr::count(3),
        PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
    )
    .expect("a stepped rule takes a count");
    let pattern: Node<editor_core::ProfileProgram> = Node::Pattern {
        input: fin,
        count: Expr::count(3),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(2.0),
        },
    };
    assert_eq!(stepped.slots(), pattern.slots(), "one rule, one slot set");
}

/// **An EMPTY placement list is the explicit rule's `count < 1`** —
/// refused, never a body with no solids in it (review MAJOR-1).
///
/// Three doors read ONE rule check ([`Node::placement_rule_fault`]):
/// the edit gate refuses first with the best diagnostics, the snapshot
/// check re-refuses on the wire, and `wire_placed_union` backstops a
/// document that reached evaluation some other way. The rows below
/// exercise the first two directly and the shared door itself, which
/// is what the third reads.
#[test]
fn an_empty_placement_list_refuses_like_a_zero_count() {
    let (doc, fin) = fin_only();
    let empty: Node<editor_core::ProfileProgram> = Node::placed_union_at(fin, Vec::new());
    assert_eq!(
        empty.placement_rule_fault(),
        Some(PlacementRuleFault::NoPlacements),
        "the shared door names it — this is what eval backstops on"
    );
    assert!(matches!(
        apply(&doc, &DocEdit::InsertNode { node: empty }, Tol::witness()),
        Err(EditError::EmptyPlacementList { .. })
    ));
    // The stepped rule it mirrors refuses at eval the same way: an
    // empty list and a zero count are the SAME statement.
    let zero = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::placed_union(
                fin,
                Expr::count(0),
                PatternKind::Linear {
                    direction: [scl(1.0), scl(0.0), scl(0.0)],
                    spacing: len(2.0),
                },
            )
            .expect("a stepped rule takes a count"),
        },
        Tol::witness(),
    )
    .expect("a zero count is legal to WRITE; it refuses at evaluation");
    let id = zero.record.minted.expect("minted");
    match eval::<f64>(&zero.doc).nodes.get(&id) {
        Some(NodeResult::Failed(e)) => assert!(matches!(
            e.kind,
            NodeErrorKind::NonPositiveCount { count: 0 }
        )),
        other => panic!("a zero count must refuse, got {other:?}"),
    }
}

/// **A hand-edited file cannot smuggle an empty list past `load`** —
/// the snapshot check re-refuses every rule the edit door would have,
/// which is the A11 registry's own posture applied to the rule.
#[test]
fn the_wire_refuses_an_emptied_placement_list() {
    let (doc, fin) = fin_only();
    let one = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::placed_union_at(fin, vec![Frame::IDENTITY]),
        },
        Tol::witness(),
    )
    .expect("one placement is legal");
    let text = editor_core::save(&one.doc, &[], Tol::witness()).expect("saves");
    // Empty the frame list on the wire, exactly as a hand edit would.
    // Bracket-matched rather than "next `]`": a frame's own `columns`
    // nest, so the naive scan would cut inside one and produce a parse
    // error instead of the refusal under test.
    let start = text
        .find("\"Explicit\"")
        .expect("the explicit rule is on the wire");
    let open = text[start..].find('[').expect("its list") + start;
    let mut depth = 0i32;
    let mut close = open;
    for (i, c) in text[open..].char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    close = open + i;
                    break;
                }
            }
            _ => {}
        }
    }
    assert!(close > open, "the frame list's brackets must match");
    let tampered = format!("{}[]{}", &text[..open], &text[close + 1..]);
    match editor_core::load(&tampered, Tol::witness()) {
        Err(editor_core::PersistError::Snapshot(_)) => {}
        other => panic!("an emptied placement list must refuse at load, got {other:?}"),
    }
}

/// **Explicit frames meet the A6/A11 bar at the EDIT door** — the same
/// finite-and-proper test `SetPlacement` applies to a cluster frame,
/// and the same typed refusals (review MINOR-2). A non-finite frame no
/// longer reads as a separation failure; it says what it is.
#[test]
fn placement_frames_are_held_to_the_cluster_frame_bar() {
    let (doc, fin) = fin_only();
    let with = |f: Frame| Node::<editor_core::ProfileProgram>::placed_union_at(fin, vec![f]);

    let nan = Frame::translation([f64::NAN, 0.0, 0.0]);
    assert_eq!(
        with(nan).placement_rule_fault(),
        Some(PlacementRuleFault::NonFiniteFrame { index: 0 }),
        "named for what it is, not as an uncertified separation"
    );
    assert!(matches!(
        apply(
            &doc,
            &DocEdit::InsertNode { node: with(nan) },
            Tol::witness()
        ),
        Err(EditError::NonFinitePlacement { .. })
    ));

    // A mirror: the identity with one basis column negated (det = −1).
    let mut mirror = Frame::IDENTITY;
    mirror.columns[0] = [-1.0, 0.0, 0.0];
    assert!(matches!(
        with(mirror).placement_rule_fault(),
        Some(PlacementRuleFault::ImproperFrame { index: 0, .. })
    ));
    assert!(matches!(
        apply(&doc, &DocEdit::InsertNode { node: with(mirror) }, Tol::witness()),
        Err(EditError::ImproperPlacement { determinant, .. }) if determinant < 0.0
    ));

    // A proper frame at the same site still goes through — the gate
    // refuses the two states, not rotations in general.
    let turned =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_2, [0.0; 3]);
    assert_eq!(with(turned).placement_rule_fault(), None);
    assert!(
        apply(
            &doc,
            &DocEdit::InsertNode { node: with(turned) },
            Tol::witness()
        )
        .is_ok()
    );
}

/// **The Explicit path is bit-compatible with the chain too** — three
/// ROTATED placements against the Transform + Union chain of the same
/// placements (`Frame::rotate_then_translate` promises bit-identity
/// with the Transform node's composition), on both mass oracles with
/// `==` plus census. The shipped fin row covers only the Linear rule
/// and only translations, so the onto-door claim was untested exactly
/// where a rotation could have broken it.
///
/// Adopted from the LIB-PLACEDUNION review's probe
/// `probe_explicit_group_equals_rotated_transform_chain` (reviewer's
/// lane) — credit to the review.
#[test]
fn the_rotated_explicit_group_equals_the_transform_union_chain() {
    use std::f64::consts::FRAC_PI_2;

    let prototype = |mut r: corpus::Recorder| -> (ProfileDoc, RecipeNodeId) {
        let p = r.insert(Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]],
        )));
        let solid = r.insert(Node::Extrude {
            profile: p,
            distance: len(1.0),
        });
        (r.doc, solid)
    };
    // (axis, angle, translation) — two genuine rotations about
    // different axes, placed clear of each other.
    let places: [([f64; 3], f64, [f64; 3]); 3] = [
        ([0.0, 0.0, 1.0], 0.0, [0.0, 0.0, 0.0]),
        ([0.0, 0.0, 1.0], FRAC_PI_2, [6.0, 0.0, 0.0]),
        ([1.0, 0.0, 0.0], -FRAC_PI_2, [0.0, 6.0, 0.0]),
    ];

    let (gdoc, gsolid) = prototype(corpus::Recorder::new());
    let grouped = apply(
        &gdoc,
        &DocEdit::InsertNode {
            node: Node::placed_union_at(
                gsolid,
                places
                    .iter()
                    .map(|&(ax, an, t)| Frame::rotate_then_translate(ax, an, t))
                    .collect(),
            ),
        },
        Tol::witness(),
    )
    .expect("insert the group");
    let group = grouped.record.minted.expect("minted");

    let (mut cdoc, csolid) = prototype(corpus::Recorder::new());
    let mut acc: Option<RecipeNodeId> = None;
    for &(ax, an, t) in &places {
        let tr = apply(
            &cdoc,
            &DocEdit::InsertNode {
                node: Node::Transform {
                    input: csolid,
                    translation: [len(t[0]), len(t[1]), len(t[2])],
                    rotation_axis: [scl(ax[0]), scl(ax[1]), scl(ax[2])],
                    rotation_angle: ang(an),
                },
            },
            Tol::witness(),
        )
        .expect("insert transform");
        let placed = tr.record.minted.expect("minted");
        cdoc = tr.doc;
        acc = Some(match acc {
            None => placed,
            Some(a) => {
                let u = apply(
                    &cdoc,
                    &DocEdit::InsertNode {
                        node: Node::Boolean {
                            op: BooleanOp::Union,
                            a,
                            b: placed,
                            declare: None,
                        },
                    },
                    Tol::witness(),
                )
                .expect("insert union");
                let id = u.record.minted.expect("minted");
                cdoc = u.doc;
                id
            }
        });
    }

    let ev_g = eval::<f64>(&grouped.doc);
    let ev_c = eval::<f64>(&cdoc);
    assert!(failures(&ev_g).is_empty(), "{:?}", failures(&ev_g));
    assert!(failures(&ev_c).is_empty(), "{:?}", failures(&ev_c));
    let g = body_of(&ev_g, group);
    let c = body_of(&ev_c, acc.expect("the chain's root"));
    let (mg, mc) = (
        topo::mass_properties(g, Tol::witness()).expect("group mass"),
        topo::mass_properties(c, Tol::witness()).expect("chain mass"),
    );
    assert_eq!(mg.volume, mc.volume, "volume, bit for bit");
    assert_eq!(mg.surface_area, mc.surface_area, "area, bit for bit");
    assert_eq!(
        (g.faces().count(), g.edges().count(), g.vertices().count()),
        (c.faces().count(), c.edges().count(), c.vertices().count()),
        "same census"
    );
    assert_eq!(g.solids().count(), c.solids().count(), "same solid count");
    assert_eq!(g.shells().count(), c.shells().count(), "same shell count");
}
