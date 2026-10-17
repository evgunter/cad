//! MATE-1 acceptance — issue 945's member-vocabulary rider (A11),
//! implemented: a mate reference head is a live `InstantiatePart` OR a
//! pattern-placed instance (`Pattern` node + `Instance(i)` qualifier)
//! at its pattern-derived pose.
//!
//! The rows, in the spec's order: the red-first four-legs-one-top
//! document (refused `DanglingHead` before this unit); the
//! pattern-derived pose asserted against hand-composed frames
//! (translation AND rotation, from the pattern's own parameters); the
//! ratified pin that mates never solve pattern parameters; and the
//! fence of heads the vocabulary still refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, Formula, Frame, MateFrame,
    MatePrimitive, MateRef, Node, PatternKind, ProfileDoc, RecipeNodeId, Space, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, at_the_door_of, constrain, door_refusal, in_copy, insert, len, on_frame, run, scl, solve,
    step,
};
use geom_core::Tol;

// ---- Substrate (the shared resolver, `fixture::resolver`) ----

// ---- Documents ----

fn block_part(
    label: &str,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// The unit cube `[0,1]³` — the leg.
fn leg_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0], geom_core::Tol::witness())
        .expect("a definite frame")
}

/// A pinning `Rest` mate by frame coincidence: `b`'s bottom frame
/// onto the `a`-side frame at `origin` (one coincidence, which pins
/// the copy it places outright).
fn seat_mate(
    a: StableName,
    b: StableName,
    origin: [f64; 3],
    sense: AxisSense,
) -> editor_core::Mate<Formula> {
    primitive_mate(a, b, origin, sense, MatePrimitive::FrameCoincidence)
}

/// [`seat_mate`] under any primitive.
fn primitive_mate(
    a: StableName,
    b: StableName,
    origin: [f64; 3],
    sense: AxisSense,
    primitive: MatePrimitive,
) -> editor_core::Mate<Formula> {
    fixture::mate(
        fixture::head(a),
        fixture::head(b),
        ContactClass::Rest,
        Alignment {
            a: frame(origin, [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive,
            sense,
            clocking: None,
        },
    )
}

/// The four-legs-one-top document, the issue's own shape: ONE leg
/// instance, a linear pattern of it (count 4, `spacing` along +x), a
/// top instance, and a seat mate placing the top on pattern copy `i`.
/// Returns (doc, leg, pattern, top, mate, store).
fn four_legs(
    label: &str,
    top_part: (ProfileDoc, RecipeNodeId),
    spacing: f64,
    i: u32,
    sense: AxisSense,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    MateRef,
    PartStore,
) {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part(&format!("{label}-leg")), Tol::witness());
    let (top_ref, top_body) = store.insert_part(top_part, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(spacing),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, i, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            sense,
        ),
    );
    (doc, leg, pattern, top, mate, store)
}

// ---- The red-first row: four legs, one top ----

/// INVARIANT (the rider's headline): a mate may name a pattern-placed
/// instance, and the OTHER member solves to the copy's pattern-derived
/// pose composed with the mate's own relation. Before this unit the
/// same document refused `MateFault::DanglingHead` at the pattern head
/// (the red half, demonstrated against main in the PR).
///
/// The solved pose is asserted against a frame composed BY HAND from
/// the pattern's own parameters — translation from `spacing × i`, the
/// rotation the OPPOSED sense promises — never from the solver's own
/// output (the ASM-DEMO precedent).
#[test]
fn a_mate_to_a_pattern_copy_places_the_other_member_at_the_derived_pose() {
    let spacing = 2.0;
    let (doc, leg, _pattern, top, mate, store) = four_legs(
        "mate1-red-first",
        leg_part("mate1-red-first-top"),
        spacing,
        2,
        AxisSense::Opposed,
    );

    // The mate's sides: the pattern-member side reads through the
    // pattern's INPUT instance, the copy the top is placed against.
    assert_eq!(mate.placement, top, "the mate is in the top's bundle");
    assert_eq!(
        crate::fixture::mate_members(&doc),
        vec![(mate, leg), (mate, top)],
        "a pattern-placed side lands on the pattern's input instance"
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(top), None, "the mate solves — no fault");
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the top is pinned, through the leg, to the world"
    );

    // Hand-composed: copy 2 sits at `translation(spacing·2 · x̂)` (the
    // pattern's own parameters, nothing read back from the solve); the
    // OPPOSED coincidence at the a-frame `origin [0,0,1], axis +z,
    // reference +x` turns the top half a turn IN the seat plane —
    // about `axis × reference = ŷ` (the flip is about the mate
    // frame's local X, and `point_at` lays the roll reference along
    // local +Y, so local X is `reference × axis`; a half-turn about
    // `−ŷ` and `ŷ` are the same map) — rotation diag(-1, 1, -1).
    let expected = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]],
        translation: [spacing * 2.0, 0.0, 1.0],
    };
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the top's solved pose is the copy's derived pose composed with the mate:\n\
         got      {got:?}\n expected {expected:?}"
    );

    // End to end: the evaluation runs the same solve; no node refuses.
    let ev = run(&doc, &o);
    assert!(
        matches!(ev.result(top), Some(editor_core::NodeResult::Ok(_))),
        "the placed top evaluates: {:?}",
        ev.result(top)
    );
}

/// The rotational half of the derived pose: a CIRCULAR pattern's copy
/// carries a rotation, and the solved member pose carries it too —
/// asserted against sin/cos composed in the test from the pattern's
/// own step angle.
#[test]
fn a_circular_pattern_copy_rotates_the_solved_member() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("mate1-circ-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("mate1-circ-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-circ"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, axis) = insert(
        doc,
        Node::Datum(editor_core::Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        }),
    );
    let theta = core::f64::consts::FRAC_PI_2;
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: PatternKind::Circular {
                axis: axis.into(),
                step: ang(theta),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.5, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );

    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(mate.placement),
        None,
        "the mate solves — no fault"
    );

    // Hand-composed from θ alone: copy 1 is the master rotated θ about
    // the z axis through the origin, so the member frame — an aligned
    // coincidence at `[0.5, 0, 1]` in the master's coordinates — lands
    // rotated and swung around: R_z(θ) as the linear part,
    // R_z(θ)·[0.5, 0, 1] as the translation.
    let (s, c) = theta.sin_cos();
    let expected = Frame {
        columns: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.5 * c, 0.5 * s, 1.0],
    };
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the circular copy's rotation reaches the solved member:\n\
         got      {got:?}\n expected {expected:?}"
    );
    let _ = store;
}

/// Whether two frames agree to `tol` in every stored coordinate.
fn near(a: Frame, b: Frame, tol: f64) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(a.translation.iter())
        .zip(b.columns.iter().flatten().chain(b.translation.iter()))
        .all(|(x, y)| (x - y).abs() <= tol)
}

// ---- Pin 1: mates never solve pattern parameters ----

/// INVARIANT (ratified pin): a mate onto a pattern copy reads the
/// pattern's parameters and never writes them. A top seated on copy 1
/// of a spacing-3 pattern sits at the AUTHORED spacing, the document's
/// spacing expression is untouched by the solve, and an EDIT of the
/// parameter — the one road that moves a copy — moves the top with it.
#[test]
fn mates_never_solve_pattern_parameters() {
    let mut store = PartStore::default();
    let label = "mate1-pin-spacing";
    let (leg_ref, leg_body) = store.insert_part(leg_part(&format!("{label}-leg")), Tol::witness());
    let (top_ref, top_body) = store.insert_part(
        block_part(&format!("{label}-top"), (0.0, 2.5), (0.0, 1.0), 0.0, 0.5),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, mate) = constrain(
        doc,
        seat_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(mate.placement), None);
    let at = poses.placement(&doc, top).expect("the top places");
    assert!(
        (at.translation[0] - 3.0).abs() < 1e-12,
        "the top sits on copy 1 at the authored spacing: {at:?}"
    );

    // The parameter did not move: the pattern node still carries the
    // authored spacing, bit for bit.
    let Some(Node::Pattern {
        kind: PatternKind::Linear { spacing, .. },
        ..
    }) = doc.node(pattern)
    else {
        panic!("the pattern is live");
    };
    assert!(
        doc.written(&editor_core::Expr::var(
            *spacing,
            editor_core::Dimension::Length
        ))
        .bit_eq(&len(3.0)),
        "the spacing expression is untouched: {spacing:?}"
    );

    // The recourse the pin promises: EDIT the parameter, and the copy
    // and the top seated on it move together.
    let (repaired, _) = step(
        doc,
        DocEdit::SetParam {
            node: pattern,
            slot: editor_core::SlotId::Spacing,
            value: len(1.5).into(),
            fresh: Vec::new(),
        },
    );
    let poses = solve(&repaired, &o, Tol::witness());
    let at = poses.placement(&repaired, top).expect("the top places");
    assert!(
        (at.translation[0] - 1.5).abs() < 1e-12,
        "the edited spacing moves the top with its copy: {at:?}"
    );
}

/// INVARIANT: the literal CONTRADICTORY refusal is reachable through a
/// pattern-member pair — two mates on the SAME copy whose cosets
/// cannot meet (a coaxial about the copy's seat axis, then a seat half
/// a unit off it) die in the bundle's fold with the measured clash,
/// exactly as a plain pair's would: the door admits each, its pin not
/// yet whole, and the solve folds the bundle and refuses it. The
/// derived offset is a static left factor OUTSIDE the fold, so it
/// cannot absorb the clash.
#[test]
fn conflicting_mates_on_one_copy_refuse_contradictory() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("mate1-contra-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("mate1-contra-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-contra"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let seat = |origin, primitive| {
        primitive_mate(
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            origin,
            AxisSense::Aligned,
            primitive,
        )
    };
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, m0) = fixture::constrain_through(
        doc,
        &reach,
        top,
        seat([0.0, 0.0, 1.0], MatePrimitive::Coaxial),
    );
    let (doc, _) = fixture::constrain_through(
        doc,
        &reach,
        top,
        seat([0.5, 0.0, 1.0], MatePrimitive::FrameCoincidence),
    );
    let poses = editor_core::solve_document(&doc, &reach, Tol::witness());
    let fault = poses
        .fault(top)
        .cloned()
        .expect("the bundle's fold refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::Contradictory { held, added, .. }
                if held == m0 && added.placement == top && added != m0
        ),
        "two seats on one copy that cannot both hold die in the fold: {fault:?}"
    );
}

// ---- The fence: what the vocabulary still refuses ----

/// INVARIANT: the member vocabulary admits exactly what the rider
/// names. A copy index at or past the count resolves to no member, and
/// refuses `DanglingHead` at the PATTERN node, exactly as every out-of-
/// vocabulary head does.
#[test]
fn out_of_vocabulary_pattern_heads_still_refuse_dangling() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("mate1-fence-leg"), Tol::witness());
    let (top_ref, top_body) = store.insert_part(leg_part("mate1-fence-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-fence"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    // Copy 5 of a count-2 pattern: no such member. A head that
    // resolves to no member is a fact about the mate alone, so the
    // edit door refuses it where it is authored, with the solve's own
    // fault: the walk's, naming the pattern.
    let fault = door_refusal(
        &doc,
        seat_mate(
            in_copy(pattern, 5, in_part(leg, leg_body, CapEnd::End)),
            in_part(top, top_body, CapEnd::Start),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert!(
        matches!(
            fault,
            editor_core::MateFault::DanglingHead { head, .. } if head == pattern
        ),
        "a copy index past the count is a dangling head at the pattern: {fault:?}"
    );
    let _ = store;
}

/// INVARIANT: one copy named on BOTH sides is the self-mate refusal,
/// a fact about the mate alone, met at the edit door with the solve's
/// own fault — whichever placement the mate is offered to.
#[test]
fn one_copy_twice_is_a_self_mate() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("mate1-selfpair-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-selfpair"), Tol::witness());
    let (doc, leg) = insert(doc, fixture::mated_instance(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.0),
            },
        },
    );
    let refusal = at_the_door_of(
        &doc,
        &editor_core::RefusingReach,
        leg,
        seat_mate(
            in_copy(pattern, 2, in_part(leg, leg_body, CapEnd::End)),
            in_copy(pattern, 2, in_part(leg, leg_body, CapEnd::End)),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert!(
        matches!(
            refusal,
            Err((_, editor_core::MateFault::SelfMate { instance, .. })) if instance == leg
        ),
        "one member on both sides is the self-mate refusal: {:?}",
        refusal.err()
    );
    let _ = store;
}

/// INVARIANT: two DISTINCT copies of one instance's pattern both stand
/// on that instance, so a constraint of the instance reading copy 0 on
/// one side and copy 1 on the other relates the copy to itself: the
/// same self-mate refusal, met at the edit door. (A pattern of a copy
/// moves with the copy, so no bundle of the copy can pin one of its
/// pattern's copies against another.)
#[test]
fn sibling_copies_of_one_instances_pattern_are_a_self_mate() {
    let mut store = PartStore::default();
    let (leg_ref, leg_body) = store.insert_part(leg_part("mate1-siblings-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-siblings"), Tol::witness());
    let (doc, leg) = insert(doc, fixture::mated_instance(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.0),
            },
        },
    );
    let refusal = at_the_door_of(
        &doc,
        &editor_core::RefusingReach,
        leg,
        seat_mate(
            in_copy(pattern, 0, in_part(leg, leg_body, CapEnd::End)),
            in_copy(pattern, 1, in_part(leg, leg_body, CapEnd::End)),
            [0.0, 0.0, 1.0],
            AxisSense::Aligned,
        ),
    );
    assert!(
        matches!(
            refusal,
            Err((_, editor_core::MateFault::SelfMate { instance, .. })) if instance == leg
        ),
        "sibling copies of one instance's pattern are one member: {:?}",
        refusal.err()
    );
    let _ = store;
}
