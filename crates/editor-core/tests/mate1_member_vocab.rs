//! MATE-1 acceptance — issue 945's member-vocabulary rider (A11): a
//! mate may read a face of a pattern-placed copy (`Pattern` node +
//! `Instance(i)` qualifier) at its pattern-derived pose.
//!
//! The rows, in the spec's order: the four-legs-one-top document; the
//! pattern-derived pose asserted against hand-composed frames
//! (translation AND rotation, from the pattern's own parameters); the
//! ratified pin that mates never solve pattern parameters; and the
//! fence of mates the vocabulary refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CapEnd, ContactClass, DocEdit, DocumentId, EntityKind, Formula, Frame, MateRef, MateSide,
    NameRef, Node, PatternKind, ProfileDoc, RecipeNodeId, RoleSeg, Space, StableName,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::{
    ang, at_the_door_of, constrain, head, in_copy, insert, insert_pinned, len, on_frame, plane,
    rest, run, scl, solve, step,
};
use geom_core::Tol;

// ---- Substrate (the shared resolver, `fixture::resolver`) ----

// ---- Documents ----

/// **The axis through `origin` along `dir` in `body`'s own
/// coordinates, read off its geometry** under `o` (which reaches the
/// parts an instance copies): [`fixture::axis_on`] for a body only a
/// resolver evaluates.
pub(crate) fn axis_with(
    doc: &ProfileDoc,
    body: RecipeNodeId,
    origin: [f64; 3],
    dir: [f64; 3],
    o: &editor_core::EvalOptions,
) -> editor_core::pose::RoleRead<editor_core::Operand> {
    let n = geom_core::Vec3::new(dir[0], dir[1], dir[2]);
    let seed = if n.x.abs() <= n.y.abs().max(n.z.abs()) {
        geom_core::Vec3::new(1.0, 0.0, 0.0)
    } else {
        geom_core::Vec3::new(0.0, 1.0, 0.0)
    };
    let u = seed - n * (seed.dot(n) / n.dot(n));
    let v = n.cross(u);
    let at = fixture::frame_at(origin, [u.x, u.y, u.z], [v.x, v.y, v.z]);
    let frame = editor_core::frame_at(doc, &body.into(), &at, o, Tol::witness())
        .expect("the body offers a frame to read the axis off");
    editor_core::pose::RoleRead::through(frame, editor_core::pose::PoseRole::Z)
}

/// **A copy of `body` placed against `target`, slid by `by`**, under
/// `o` (which reaches the parts an instance copies):
/// [`fixture::slid`] for bodies only a resolver evaluates. Returns the
/// document and the placement.
pub(crate) fn slid_with(
    doc: ProfileDoc,
    target: RecipeNodeId,
    body: RecipeNodeId,
    by: [f64; 3],
    o: &editor_core::EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    against_with(doc, target, body, &Frame::translation(by), o)
}

/// **A copy of `body` placed against `target` at `at`** (the copy's
/// own frame in `target`'s own coordinates), under `o`:
/// [`fixture::against`] for bodies only a resolver evaluates. Returns
/// the document and the placement.
pub(crate) fn against_with(
    doc: ProfileDoc,
    target: RecipeNodeId,
    body: RecipeNodeId,
    at: &Frame,
    o: &editor_core::EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let read = editor_core::Operand::from(body);
    let placed = editor_core::place_against(&doc, &target.into(), &read, at, o, Tol::witness())
        .expect("the copy places against its target");
    let (doc, placement) = insert(doc, Node::place(vec![editor_core::BodyRead::plain(read)]));
    let applied = doc
        .apply(
            &DocEdit::AddConstraint {
                placement,
                mate: Box::new(placed.mate),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("the mate adds");
    let mate = applied.record.minted_constraints[0];
    let doc = placed
        .values
        .into_iter()
        .fold(applied.doc, |doc, (coord, by)| {
            step(
                doc,
                DocEdit::AddValue {
                    placement,
                    value: Box::new(editor_core::Value { mate, coord, by }),
                    fresh: Vec::new(),
                },
            )
            .0
        });
    (doc, placement)
}

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
        },
    )
}

/// The unit cube `[0,1]³` — the leg, and the top seated on it.
fn leg_part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    block_part(label, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0)
}

/// **A stored unit-cube part**: its reference, its body, and the
/// part's own names for its walls 0 (`y = 0`) and 1 (`x = 1`), as the
/// part's placement carries them.
struct Cube {
    doc_ref: editor_core::DocRef,
    body: RecipeNodeId,
    walls: [RoleSeg; 2],
}

fn cube(store: &mut PartStore, label: &str) -> Cube {
    let (doc, body) = leg_part(label);
    let walls = [fixture::wall(&doc, body, 0), fixture::wall(&doc, body, 1)];
    let (doc_ref, body) = store.insert_part((doc, body), Tol::witness());
    Cube {
        doc_ref,
        body,
        walls,
    }
}

impl Cube {
    /// The cap `end` of this part as `instance` places it.
    fn cap(&self, instance: RecipeNodeId, end: CapEnd) -> StableName {
        in_part(instance, self.body, end)
    }

    /// Wall `k` of this part as `instance` places it — the part's own
    /// name under its placement's copy, worn inside the instance's
    /// `InPart`.
    fn wall(&self, instance: RecipeNodeId, k: usize) -> StableName {
        let placed = StableName {
            kind: EntityKind::Face,
            node: placement_of(self.body),
            path: vec![RoleSeg::Placed {
                of: NameRef::new(StableName {
                    kind: EntityKind::Face,
                    node: self.body,
                    path: vec![self.walls[k].clone()],
                }),
            }],
        };
        fixture::face(placed).in_part(instance).into_name()
    }
}

/// **`face`'s plane slid `by` along its own normal**: a standoff, which
/// places and declares no contact — what a flush wall is mated to.
fn standoff(face: impl Into<editor_core::Operand>, by: f64) -> editor_core::Operand {
    let p = plane(face);
    editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(p.clone()),
        along: editor_core::pose::RoleRead::through(p, editor_core::pose::PoseRole::Normal),
        by: len(by),
    }))
}

/// **The seat**: the top's start cap resting on `copy`'s end cap, and
/// its walls 0 and 1 flush with the copy's (a standoff of zero, so
/// only the rest declares a contact) — three plane mates whose normals
/// span space, so together they pin the top with no value: it stands on
/// the copy exactly as the copy's own coordinates lifted by one,
/// whatever pose the copy has.
fn seat(
    doc: ProfileDoc,
    top: RecipeNodeId,
    top_part: &Cube,
    copy: impl Fn(StableName) -> StableName,
    copy_part: &Cube,
    copy_at: RecipeNodeId,
) -> (ProfileDoc, MateRef) {
    let (doc, m) = constrain(
        doc,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head(copy(copy_part.cap(copy_at, CapEnd::End))),
            ContactClass::Rest,
        ),
    );
    let doc = (0..2).fold(doc, |doc, k| {
        constrain(
            doc,
            fixture::mate(
                plane(head(top_part.wall(top, k))),
                standoff(head(copy(copy_part.wall(copy_at, k))), 0.0),
                ContactClass::Rest,
            ),
        )
        .0
    });
    (doc, m)
}

/// The four-legs-one-top document, the issue's own shape: ONE leg
/// instance pinned in the world, a linear pattern of it (count 4,
/// `spacing` along +x), a top instance, and the seat placing the top
/// on pattern copy `i`. Returns (doc, leg, pattern, top, rest mate,
/// options reaching the parts).
fn four_legs(
    label: &str,
    spacing: f64,
    i: u32,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    MateRef,
    editor_core::EvalOptions,
) {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, &format!("{label}-leg"));
    let top_part = cube(&mut store, &format!("{label}-top"));
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, i, n),
        &leg_part,
        leg,
    );
    (doc, leg, pattern, top, mate, o)
}

// ---- The headline row: four legs, one top ----

/// INVARIANT (the rider's headline): a mate may read a face of a
/// pattern-placed copy, and the copy it places solves to the copy's
/// pattern-derived pose composed with the mates' own relation.
///
/// The solved pose is asserted against a frame composed BY HAND from
/// the pattern's own parameters — translation from `spacing × i`, the
/// seat's lift of one — never from the solver's own output (the
/// ASM-DEMO precedent).
///
/// *Breaks if* a pattern copy's face is read at its master's pose (the
/// top lands at `x = 0`), or the seat's lift is lost (`z = 0`).
#[test]
fn a_mate_to_a_pattern_copy_places_the_other_member_at_the_derived_pose() {
    let spacing = 2.0;
    let (doc, leg, _pattern, top, mate, o) = four_legs("mate1-red-first", spacing, 2);

    // The mate's poses: the pattern-member pose reads through the
    // pattern's INPUT instance, the copy the top is placed against.
    assert_eq!(mate.placement, top, "the mate is in the top's bundle");
    assert!(
        fixture::mate_members(&doc).contains(&(mate, leg)),
        "a pattern-placed pose lands on the pattern's input instance: {:?}",
        fixture::mate_members(&doc)
    );

    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(top), None, "the mates solve — no fault");
    assert_eq!(
        poses.space(top),
        Some(Space::World),
        "the top is pinned, through the leg, to the world"
    );

    // Hand-composed: copy 2 sits at `translation(spacing·2 · x̂)` (the
    // pattern's own parameters, nothing read back from the solve), and
    // the seat lifts the top one unit onto the copy's end cap.
    let expected = Frame {
        columns: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [spacing * 2.0, 0.0, 1.0],
    };
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the top's solved pose is the copy's derived pose composed with the mates:\n\
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
/// own step angle and axis.
///
/// *Breaks if* the copy's rotation is dropped from the face it reads
/// (the top stays axis-aligned), or turned about the wrong line.
#[test]
fn a_circular_pattern_copy_rotates_the_solved_member() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "mate1-circ-leg");
    let top_part = cube(&mut store, "mate1-circ-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-circ"), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
    let axis = axis_with(&doc, leg, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], &o);
    let theta = core::f64::consts::FRAC_PI_2;
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: Formula::count(4),
            kind: PatternKind::Circular {
                axis,
                step: ang(theta),
            },
        },
    );
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, 1, n),
        &leg_part,
        leg,
    );

    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(mate.placement),
        None,
        "the mates solve — no fault"
    );

    // Hand-composed from θ and the axis alone: copy 1 is the master
    // turned θ about the z-parallel line through `(2, 0, 0)`, and the
    // top stands one unit up the copy: R_z(θ) as the linear part, and
    // R_z(θ)·((0, 0, 1) − (2, 0, 0)) + (2, 0, 0) as the translation.
    let (s, c) = theta.sin_cos();
    let expected = Frame {
        columns: [[c, s, 0.0], [-s, c, 0.0], [0.0, 0.0, 1.0]],
        translation: [2.0 - 2.0 * c, -2.0 * s, 1.0],
    };
    let got = poses.relative(top).expect("the top has a pose");
    assert!(
        near(got, expected, 1e-12),
        "the circular copy's rotation reaches the solved member:\n\
         got      {got:?}\n expected {expected:?}"
    );
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
///
/// *Breaks if* the solve writes the spacing, or the top does not follow
/// an edit of it.
#[test]
fn mates_never_solve_pattern_parameters() {
    let mut store = PartStore::default();
    let label = "mate1-pin-spacing";
    let leg_part = cube(&mut store, &format!("{label}-leg"));
    let top_part = cube(&mut store, &format!("{label}-top"));
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let (doc, mate) = seat(
        doc,
        top,
        &top_part,
        |n| in_copy(pattern, 1, n),
        &leg_part,
        leg,
    );
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
/// cannot meet (the top's start cap resting on the copy's end cap,
/// then the top's own end cap flush with that same cap: parallel
/// planes a unit apart) die in the bundle's fold with the measured
/// clash, exactly as a plain pair's would. The door admits each — it
/// decides structure alone — and the solve folds the bundle and
/// refuses it. The derived offset is a static left factor OUTSIDE the
/// fold, so it cannot absorb the clash.
///
/// *Breaks if* the fold admits two parallel planes at different
/// heights, or names the mates the other way round.
#[test]
fn conflicting_mates_on_one_copy_refuse_contradictory() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "mate1-contra-leg");
    let top_part = cube(&mut store, "mate1-contra-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-contra"), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    let copy_end = head(in_copy(pattern, 1, leg_part.cap(leg, CapEnd::End)));
    let (doc, m0) = constrain(
        doc,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            copy_end.clone(),
            ContactClass::Rest,
        ),
    );
    let (doc, _) = constrain(
        doc,
        fixture::mate(
            plane(head(top_part.cap(top, CapEnd::End))),
            plane(copy_end),
            ContactClass::Rest,
        ),
    );
    let poses = solve(&doc, &o, Tol::witness());
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
        "two mates on one copy that cannot both hold die in the fold: {fault:?}"
    );
}

// ---- The fence: what the vocabulary refuses ----

/// INVARIANT: the member vocabulary admits exactly what the rider
/// names. A copy index at or past the count reads no face: the door
/// admits the mate (it decides structure alone), and the placement
/// refuses where it evaluates, its `to` pose unevaluated.
///
/// *Breaks if* a copy past the count resolves to some face, or its
/// refusal is laid on the `on` pose.
#[test]
fn out_of_vocabulary_pattern_heads_refuse_where_the_placement_evaluates() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "mate1-fence-leg");
    let top_part = cube(&mut store, "mate1-fence-top");
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-fence"), Tol::witness());
    let (doc, leg) = insert_pinned(doc, Node::instantiate_part(leg_part.doc_ref), &o);
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
    let (doc, top) = insert(doc, fixture::mated_instance(top_part.doc_ref));
    // Copy 5 of a count-2 pattern: no such member.
    let (doc, mate) = constrain(
        doc,
        rest(
            head(top_part.cap(top, CapEnd::Start)),
            head(in_copy(pattern, 5, leg_part.cap(leg, CapEnd::End))),
            ContactClass::Rest,
        ),
    );
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(top).cloned().expect("the placement refuses");
    assert!(
        matches!(
            fault,
            editor_core::MateFault::SideUnevaluated { mate: m, side: MateSide::To, .. }
                if m == mate
        ),
        "a copy index past the count leaves the mate's `to` pose unevaluated: {fault:?}"
    );
}

/// INVARIANT: a mate whose `on` reads a copy of the instance's
/// pattern, rather than the instance's own output, pins nothing of the
/// instance — a fact about the mate alone, met at the edit door.
///
/// *Breaks if* the door takes a pattern copy's face for the copy the
/// instance defines.
#[test]
fn one_copy_twice_is_not_on_the_copy() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "mate1-selfpair-leg");
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-selfpair"), Tol::witness());
    let (doc, leg) = insert(doc, fixture::mated_instance(leg_part.doc_ref));
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
    let copy_end = || head(in_copy(pattern, 2, leg_part.cap(leg, CapEnd::End)));
    let refusal = at_the_door_of(
        &doc,
        &editor_core::RefusingReach,
        leg,
        rest(copy_end(), copy_end(), ContactClass::Rest),
    );
    assert!(
        matches!(
            refusal,
            Err((at, editor_core::MateFault::NotOnTheCopy { mate })) if at == leg && mate.placement == leg
        ),
        "a mate reading one pattern copy on both poses is not on the instance's copy: {:?}",
        refusal.err()
    );
}

/// INVARIANT: two DISTINCT copies of one instance's pattern both stand
/// on that instance, so a constraint of the instance reading copy 0 as
/// `on` and copy 1 as `to` reads no face of the instance's own copy:
/// the same refusal, met at the edit door. (A pattern of a copy moves
/// with the copy, so no bundle of the copy can pin one of its
/// pattern's copies against another.)
///
/// *Breaks if* the door takes a pattern copy's face for the copy the
/// instance defines.
#[test]
fn sibling_copies_of_one_instances_pattern_are_not_on_the_copy() {
    let mut store = PartStore::default();
    let leg_part = cube(&mut store, "mate1-siblings-leg");
    let doc = ProfileDoc::empty(DocumentId::derive("mate1-siblings"), Tol::witness());
    let (doc, leg) = insert(doc, fixture::mated_instance(leg_part.doc_ref));
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
        rest(
            head(in_copy(pattern, 0, leg_part.cap(leg, CapEnd::End))),
            head(in_copy(pattern, 1, leg_part.cap(leg, CapEnd::End))),
            ContactClass::Rest,
        ),
    );
    assert!(
        matches!(
            refusal,
            Err((at, editor_core::MateFault::NotOnTheCopy { mate })) if at == leg && mate.placement == leg
        ),
        "sibling copies of one instance's pattern are not its copy: {:?}",
        refusal.err()
    );
}
