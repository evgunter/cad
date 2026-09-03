//! MATE-1 acceptance — issue 945's member-vocabulary rider (A11),
//! implemented: a mate reference head is a live `InstantiatePart` OR a
//! pattern-placed instance (`Pattern` node + `Instance(i)` qualifier)
//! at its pattern-derived pose.
//!
//! The rows, in the spec's order: the red-first four-legs-one-top
//! document (refused `DanglingHead` before this unit); the
//! pattern-derived pose asserted against hand-composed frames
//! (translation AND rotation, from the pattern's own parameters); the
//! sibling-instance loop (tree + declaring, verified both directions);
//! and the two ratified pins — mates never solve pattern parameters,
//! and `Instance(i)` heads are canonical (the master-name spelling
//! still refuses `Vanished`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AssemblyError, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocRef,
    DocumentId, EntityKind, EvalOptions, Evaluation, Expr, Frame, MateFrame, MatePrimitive,
    MateRole, Node, PartResolver, PatternKind, ProfileDoc, RecipeNodeId, ResolveFailure,
    ResolveFault, RoleSeg, StableName, assemble, clusters, content_pin, evaluate, solve_document,
};
use fixture::{insert, len, on_frame, scl, step};
use geom_core::Tol;

// ---- Substrate (the stub resolver, as in the sibling suites) ----

#[derive(Debug, Default)]
struct StubStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl StubStore {
    fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }
}

impl PartResolver for StubStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

fn opts(store: StubStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    }
}

fn run(doc: &ProfileDoc, o: &EvalOptions) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), o, Tol::witness())
}

// ---- Documents ----

/// An axis-aligned block part: `[x]×[y]` at `z0`, extruded `dz`. Its
/// extrude is [`PART_BODY`], so the part's caps name through it.
/// The extrude in a one-block part document. A block is three nodes
/// — the sketch frame, the profile drawn on it, then the extrude.
const PART_BODY: RecipeNodeId = RecipeNodeId(2);

fn block_part(label: &str, x: (f64, f64), y: (f64, f64), z0: f64, dz: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(dz),
        },
    );
    doc
}

/// The unit cube `[0,1]³` — the leg.
fn leg_part(label: &str) -> ProfileDoc {
    block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

/// A face of `instance`'s part product, named through the instance
/// qualifier — the plain member spelling.
fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}

/// A face of pattern copy `i` — the `Instance(i)` spelling the rider
/// makes canonical: the PATTERN node as head, the master's own name
/// under the qualifier.
fn in_copy(pattern: RecipeNodeId, i: u32, master: StableName) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: pattern,
        path: vec![RoleSeg::Instance {
            i,
            of: Box::new(master),
        }],
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference: [1.0, 0.0, 0.0],
    }
}

/// A determining `Rest` mate by frame coincidence: `b`'s bottom frame
/// onto the `a`-side frame at `origin` (one coincidence, DETERMINED —
/// the A11 rule-4 tree edge wants no residual).
fn seat_mate(
    a: StableName,
    b: StableName,
    origin: [f64; 3],
    sense: AxisSense,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: frame(origin, [0.0, 0.0, 1.0]),
            b: frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::FrameCoincidence,
            sense,
            clocking: None,
        },
    }
}

/// The four-legs-one-top document, the issue's own shape: ONE leg
/// instance, a linear pattern of it (count 4, `spacing` along +x), a
/// top instance, and a seat mate from the top onto pattern copy `i`.
/// Returns (doc, leg, pattern, top, mate, store).
fn four_legs(
    label: &str,
    top_part: ProfileDoc,
    spacing: f64,
    i: u32,
    sense: AxisSense,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    StubStore,
) {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part(&format!("{label}-leg")), Tol::witness());
    let top_ref = store.insert(top_part, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(4),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(spacing),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, i, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                sense,
            ),
        },
    );
    (doc, leg, pattern, top, mate.expect("the mate mints"), store)
}

/// Every declaration the findings name, and in what relation.
fn relations(findings: &[editor_core::AtRestFinding]) -> Vec<(RecipeNodeId, &'static str)> {
    findings
        .iter()
        .map(|f| match &f.attribution {
            editor_core::Attribution::Refuted(m) => (m.mate, "refuted"),
            editor_core::Attribution::Declined(m) => (m.mate, "declined"),
            editor_core::Attribution::Unattributed => (RecipeNodeId(u64::MAX), "unattributed"),
        })
        .collect()
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

    // The reading edges: the pattern-member mate reads through the
    // pattern's INPUT instance — the vertex that joins the top into
    // the pattern's cluster.
    assert_eq!(
        editor_core::reading_edges(&doc),
        vec![(mate, leg), (mate, top)],
        "a pattern-placed head contributes the reading edge at the pattern's input instance"
    );
    assert_eq!(
        clusters(&doc),
        vec![vec![leg, top]],
        "the mate joins the top into the pattern's cluster; the gauge is the leg (document-first)"
    );

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None, "the mate solves — no fault");
    assert_eq!(poses.role(mate), Some(MateRole::Determining));
    assert_eq!(poses.gauge(top), Some(leg));

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
    let ev = run(&doc, &opts(store));
    assert!(
        matches!(ev.result(mate), Some(editor_core::NodeResult::Ok(_))),
        "the mate evaluates: {:?}",
        ev.result(mate)
    );
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
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-circ-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("mate1-circ-top"), Tol::witness());
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
            input: leg,
            count: Expr::count(4),
            kind: PatternKind::Circular {
                axis,
                step: Expr::literal(theta, editor_core::Dimension::Angle)
                    .expect("an angle literal"),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.5, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(mate), None, "the mate solves — no fault");

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

// ---- The loop row: a sibling-instance mate declares, both ways ----

/// The two-legs-one-top document with TWO seat mates — copy 0 (the
/// tree edge) and copy 1 (the sibling). Returns
/// (doc, pattern, mates, store).
fn two_seats(
    label: &str,
    spacing: f64,
    top_x: (f64, f64),
) -> (ProfileDoc, RecipeNodeId, [RecipeNodeId; 2], StubStore) {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part(&format!("{label}-leg")), Tol::witness());
    let top_ref = store.insert(
        block_part(&format!("{label}-top"), top_x, (0.0, 1.0), 0.0, 0.5),
        Tol::witness(),
    );
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(spacing),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let (doc, m0) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 0, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    (
        doc,
        pattern,
        [m0.expect("mate 0 mints"), m1.expect("mate 1 mints")],
        store,
    )
}

/// INVARIANT (the rider's loop clause; rule 4's stud-stack promise): a
/// second tree mate from a SIBLING instance closes a loop — non-tree,
/// DECLARING — and a consistent loop VERIFIES at the gate rather than
/// over-determining. Legs at spacing 1.5 under a `[0, 2.5]` top: both
/// declared seats hold, the gate certifies, nothing refuses.
#[test]
fn a_consistent_sibling_loop_declares_and_verifies() {
    let (doc, _, [m0, m1], store) = two_seats("mate1-loop-ok", 1.5, (0.0, 2.5));
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(m0), None);
    assert_eq!(poses.fault(m1), None);
    assert_eq!(
        poses.role(m0),
        Some(MateRole::Determining),
        "the copy-0 seat is the tree edge"
    );
    assert_eq!(
        poses.role(m1),
        Some(MateRole::Declaring),
        "the sibling seat closes a loop: non-tree, declaring"
    );

    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    // The branch this fixture takes is `Ok` with both declarations
    // minted — asserted hard, so the row reds if loop verification
    // degrades to an uncertified frontier (or any refusal) rather than
    // absorbing the degradation.
    let Ok(assembly) = &result else {
        panic!("a consistent loop verifies; the gate said: {result:?}");
    };
    assert_eq!(
        assembly.minted.len(),
        2,
        "both seats minted their declarations"
    );
}

/// INVARIANT (the same clause, the other direction): an INCONSISTENT
/// loop dies at the CLOSING mate's verification, naming it. Spacing 4
/// puts copy 1 clean off the `[0, 2.5]` top, so the sibling seat's
/// declared contact does not hold at the solved geometry — the gate
/// refuses with a finding against mate 1, and mate 0's seat (which
/// holds) is not the one named.
#[test]
fn an_inconsistent_sibling_loop_dies_at_the_closing_mates_verification() {
    let (doc, _, [m0, m1], store) = two_seats("mate1-loop-bad", 4.0, (0.0, 2.5));
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(
        poses.fault(m1),
        None,
        "the solve does not refuse — verification does"
    );
    assert_eq!(poses.role(m1), Some(MateRole::Declaring));

    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let Err(AssemblyError::AtRest { findings }) = &result else {
        panic!("an inconsistent loop is a finding against the document, got {result:?}");
    };
    let rel = relations(findings);
    assert!(
        rel.contains(&(m1, "refuted")),
        "the closing mate is named, refuted: {rel:?}"
    );
    assert!(
        !rel.iter().any(|&(m, _)| m == m0),
        "the tree seat holds and is not named: {rel:?}"
    );
}

// ---- Pin 1: mates never solve pattern parameters ----

/// INVARIANT (ratified pin): a seat satisfiable only at a DIFFERENT
/// spacing is refused with the measured clash — the recourse is to
/// edit the parameter, and nothing anywhere back-solves it. The
/// pattern is authored at spacing 3 under a `[0, 2.5]` top whose
/// sibling seat holds at spacing 1.5: the gate refuses naming that
/// seat, the document's spacing expression is untouched, the solved
/// poses sit at the AUTHORED spacing — and the parameter EDIT (the
/// pin's recourse) verifies the same seat.
#[test]
fn mates_never_solve_pattern_parameters() {
    let (doc, pattern, [m0, m1], store) = two_seats("mate1-pin-spacing", 3.0, (0.0, 2.5));
    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(poses.fault(m0), None);
    assert_eq!(poses.fault(m1), None);

    let o = opts(store);
    let ev = run(&doc, &o);
    let result = assemble(&doc, &ev, Tol::witness());
    let Err(AssemblyError::AtRest { findings }) = &result else {
        panic!("the unsatisfiable seat refuses at verification, got {result:?}");
    };
    assert!(
        relations(findings).contains(&(m1, "refuted")),
        "the refusal names the seat, with the kernel's measured finding: {findings:?}"
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
        spacing.bit_eq(&len(3.0)),
        "the spacing expression is untouched: {spacing:?}"
    );

    // The recourse the pin promises: EDIT the parameter. At the
    // satisfiable spacing the same seat verifies — the gate certifies
    // the document with both declarations minted.
    let (repaired, _) = step(
        doc,
        DocEdit::SetParam {
            node: pattern,
            slot: editor_core::SlotId::Spacing,
            expr: len(1.5),
        },
    );
    let ev = run(&repaired, &o);
    let result = assemble(&repaired, &ev, Tol::witness());
    let Ok(assembly) = &result else {
        panic!("the edited spacing satisfies the seat, got {result:?}");
    };
    assert_eq!(
        assembly.minted.len(),
        2,
        "both seats verify at the edited spacing"
    );
}

/// INVARIANT: the literal CONTRADICTORY refusal is reachable through a
/// pattern-member pair — two mates on the SAME copy whose cosets
/// cannot meet die in the per-pair fold with the measured clash,
/// exactly as a plain pair's would. The derived offset is a static
/// left factor OUTSIDE the fold, so it cannot absorb the clash.
#[test]
fn conflicting_mates_on_one_copy_refuse_contradictory() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-contra-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("mate1-contra-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-contra"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let seat = |origin| {
        seat_mate(
            in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
            in_part(top, CapEnd::Bottom),
            origin,
            AxisSense::Aligned,
        )
    };
    let (doc, m0) = step(
        doc,
        DocEdit::InsertNode {
            node: seat([0.0, 0.0, 1.0]),
        },
    );
    let (doc, m1) = step(
        doc,
        DocEdit::InsertNode {
            node: seat([0.5, 0.0, 1.0]),
        },
    );
    let m0 = m0.expect("mate 0 mints");
    let m1 = m1.expect("mate 1 mints");

    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(m0).expect("the pair's fold refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::Contradictory { held, added, .. }
                if *held == m0 && *added == m1
        ),
        "two seats on one copy that cannot both hold die in the fold: {fault:?}"
    );
    let _ = store;
}

// ---- Pin 2: `Instance(i)` heads are canonical ----

/// INVARIANT (ratified pin): the pattern consumed its master's root,
/// so the MASTER-NAME spelling of a seat still refuses `Vanished` at
/// the gate — honestly, and pinned as a refusal, not fixed. The
/// canonical spelling is the `Instance(i)` head the other rows use.
#[test]
fn the_master_name_spelling_still_refuses_vanished() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-pin-master-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("mate1-pin-master-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-pin-master"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, _pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    // The master's own name — the spelling the pattern consumed.
    let (doc, mate) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_part(leg, CapEnd::Top),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let mate = mate.expect("the mate mints");

    let ev = run(&doc, &opts(store));
    let result = assemble(&doc, &ev, Tol::witness());
    let Err(AssemblyError::Reference {
        mate: named,
        side,
        why,
        ..
    }) = &result
    else {
        panic!("the master-name seat refuses at the gate, got {result:?}");
    };
    assert_eq!(*named, mate);
    assert_eq!(*side, editor_core::MateSide::A);
    assert!(
        matches!(why, editor_core::RefusedRef::Vanished),
        "the consumed master's face answers to nothing: {why:?}"
    );
}

// ---- The fence: what the vocabulary still refuses ----

/// INVARIANT: the member vocabulary admits exactly what the rider
/// names. A copy index at or past the count resolves to no member; a
/// pattern of a non-instance body stands no member at all. Both refuse
/// `DanglingHead` at the PATTERN node, exactly as every out-of-
/// vocabulary head does.
#[test]
fn out_of_vocabulary_pattern_heads_still_refuse_dangling() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-fence-leg"), Tol::witness());
    let top_ref = store.insert(leg_part("mate1-fence-top"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-fence"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    // Copy 5 of a count-2 pattern: no such member.
    let (doc, stale) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 5, in_part(leg, CapEnd::Top)),
                in_part(top, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let stale = stale.expect("the mate mints");
    let poses = solve_document(&doc, Tol::witness());
    let fault = poses.fault(stale).expect("an out-of-range copy refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::DanglingHead { head, .. } if *head == pattern
        ),
        "a copy index past the count is a dangling head at the pattern: {fault:?}"
    );
    let _ = store;

    // A pattern of a non-instance body (an extrude patterned in the
    // same document): its copies stand on no instance — no member.
    let doc2 = block_part("mate1-fence-body", (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let extrude = PART_BODY;
    let (doc2, body_pattern) = insert(
        doc2,
        Node::Pattern {
            input: extrude,
            count: Expr::count(2),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let mut store2 = StubStore::default();
    let other_ref = store2.insert(leg_part("mate1-fence-other"), Tol::witness());
    let (doc2, other) = insert(doc2, Node::instantiate_part(other_ref));
    let master_face = StableName {
        kind: EntityKind::Face,
        node: extrude,
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let (doc2, m) = step(
        doc2,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(body_pattern, 0, master_face),
                in_part(other, CapEnd::Bottom),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let m = m.expect("the mate mints");
    let poses2 = solve_document(&doc2, Tol::witness());
    let fault2 = poses2.fault(m).expect("a patterned non-instance refuses");
    assert!(
        matches!(
            fault2,
            editor_core::MateFault::DanglingHead { head, .. } if *head == body_pattern
        ),
        "a pattern of a non-instance stands no member: {fault2:?}"
    );
}

/// INVARIANT: two DISTINCT copies of one pattern are a pair like any
/// other — their mate DECLARES (the pattern already determined both
/// ends; the edge can never be a tree edge) — while one copy named on
/// BOTH sides is still the self-mate refusal.
#[test]
fn sibling_copies_declare_and_one_copy_twice_is_a_self_mate() {
    let mut store = StubStore::default();
    let leg_ref = store.insert(leg_part("mate1-selfpair-leg"), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-selfpair"), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(leg_ref));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(1.0),
            },
        },
    );
    // Copy 0's side face against copy 1's — the stud-stack shape, at
    // its smallest.
    let (doc, declared) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 0, in_part(leg, CapEnd::Top)),
                in_copy(pattern, 1, in_part(leg, CapEnd::Top)),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let declared = declared.expect("the mate mints");
    let (doc, selfish) = step(
        doc,
        DocEdit::InsertNode {
            node: seat_mate(
                in_copy(pattern, 2, in_part(leg, CapEnd::Top)),
                in_copy(pattern, 2, in_part(leg, CapEnd::Top)),
                [0.0, 0.0, 1.0],
                AxisSense::Aligned,
            ),
        },
    );
    let selfish = selfish.expect("the mate mints");

    let poses = solve_document(&doc, Tol::witness());
    assert_eq!(
        poses.role(declared),
        Some(MateRole::Declaring),
        "sibling copies of one pattern declare — never a tree edge"
    );
    assert_eq!(poses.fault(declared), None);
    let fault = poses.fault(selfish).expect("one copy twice refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::SelfMate { instance, .. } if *instance == leg
        ),
        "one member on both sides is the self-mate refusal: {fault:?}"
    );
    let _ = store;
}
