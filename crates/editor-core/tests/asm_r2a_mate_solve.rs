//! ASM-R2a acceptance — the mate solve.
//!
//! Rows 1–7 of the unit's acceptance list; row 8 is CI's clippy lanes.
//! Everything here runs on a STUB resolver, the substrate ASM-2A D-3
//! established for this layer.
//!
//! Every mate is a constraint in the bundle of the instance it places
//! (`docs/INTENT-STAGE3-SPEC.md` §3): the first instance of each
//! assembly is pinned to the world, the rest start with empty bundles,
//! and a constraint between two instances joins the bundle of the one
//! it moves.
//!
//! Undo note, as in `asm_roots` and `asm2a_instantiate`: this layer has
//! no undo STACK. `apply` is pure and undo is keeping prior values, so
//! "undo restores exactly" is checked the way the layer offers it — the
//! pre-edit document, held across the edit, is unchanged, bit for bit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire::doctored;
use editor_core::ExtrudeSide;
use editor_core::Formula;

use editor_core::CapEnd;
use editor_core::{
    Alignment, AxisSense, ContactClass, DocEdit, DocumentId, EditError, EntityKind, Evaluation,
    Frame, Maintenance, Mate, MateFrame, MatePrimitive, MateRef, Node, NodeErrorKind, NodeResult,
    Placement, ProfileDoc, RecipeNodeId, RoleSeg, Space, StableName, apply, load, product,
    relative_freedom_components, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{FIXTURE_MATE_AXIS, door_refusal, insert, len, on_frame, run, solve, square, step};
use geom_core::Tol;

/// A one-metre lever, through the one door a lever is formed by.
fn unit_arm() -> editor_core::mate::coset::Arm {
    editor_core::mate::coset::Arm::of(0.0, 1.0).expect("a metre is in range")
}

// ---- Substrate ----

/// A one-solid part: a unit square extruded 1 tall, and its body.
fn part(label: &str) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    )
}

/// An assembly of `n` instances of one part, plus the store that
/// resolves them and the part's body. Only the first is pinned to the
/// world — the rest sit where their constraints put them, each a loose
/// copy until one does — so a constraint between the first and another
/// moves the other onto it.
fn assembly(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, PartStore, RecipeNodeId) {
    let mut store = PartStore::default();
    let (doc_ref, body) = store.insert_part(part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let node = if i == 0 {
            Node::instantiate_part(doc_ref.clone())
        } else {
            fixture::mated_instance(doc_ref.clone())
        };
        let (next, id) = insert(doc, node);
        doc = next;
        ids.push(id);
    }
    (doc, ids, store, body)
}

/// **`placement`'s bundle replaced by one world pin at `frame`** — how
/// a copy is put at a world pose.
fn repin(doc: ProfileDoc, placement: RecipeNodeId, frame: &Frame) -> ProfileDoc {
    let doc = fixture::bundle(&doc, placement)
        .into_iter()
        .fold(doc, |doc, c| {
            step(
                doc,
                DocEdit::RemoveConstraint {
                    placement,
                    constraint: c.id,
                },
            )
            .0
        });
    fixture::constrain_at(doc, placement, Mate::world_pin(Placement::literal(frame))).0
}

/// **`mate` added to the bundle of the copy it places**, through the
/// door levered by `store`'s parts: a bundle the door folds may need
/// the parts' extent.
fn add(doc: ProfileDoc, store: &PartStore, mate: Mate<Formula>) -> (ProfileDoc, MateRef) {
    let o = with_resolver(store.clone());
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    fixture::constrain_with(doc, &reach, mate)
}

fn frame(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, axis, reference, geom_core::Tol::witness())
        .expect("a definite frame")
}

/// A rest mate between instances `a` and `b` of the part whose body is
/// `body`, each head the body's start cap.
#[allow(clippy::too_many_arguments)] // the mate's whole datum, and the body its heads name
fn mate(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    primitive: MatePrimitive,
    sense: AxisSense,
    fa: MateFrame<Formula>,
    fb: MateFrame<Formula>,
    clocking: Option<f64>,
) -> Mate<Formula> {
    fixture::mate(
        fixture::head(in_part(a, body, CapEnd::Start)),
        fixture::head(in_part(b, body, CapEnd::Start)),
        ContactClass::Rest,
        Alignment {
            a: fa,
            b: fb,
            primitive,
            sense,
            clocking,
        },
    )
}

/// Whether two frames agree to `tol` in every stored coordinate — for
/// poses that come out of a closed form through the transcendentals,
/// where the claim is the VALUE and bit-equality is not on offer.
fn near(a: Frame, b: Frame, tol: f64) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(a.translation.iter())
        .zip(b.columns.iter().flatten().chain(b.translation.iter()))
        .all(|(x, y)| (x - y).abs() <= tol)
}

fn z_up() -> MateFrame<Formula> {
    frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

fn mate_fault(ev: &Evaluation<f64>, node: RecipeNodeId) -> editor_core::MateFault {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a Mate refusal, got {other:?}"),
        },
        other => panic!("expected a failure, got {other:?}"),
    }
}

// ---- Row 1: a determined pair, evaluated, deterministic ----

/// The A11 rule-1 exemplar, verbatim: **coaxial plus flush plus
/// clocking** is the DETERMINED case. The spec row names it
/// "coaxial+planar(⊥)", but the binding table has no trivial entry for
/// coaxial ∩ planar at any angle — parallel gives revolute,
/// perpendicular gives prismatic — so the clocking rider is what
/// closes it, which is exactly the combination A11 rule 1 lists as its
/// own DETERMINED example.
fn determined_pair() -> (ProfileDoc, Vec<RecipeNodeId>, PartStore, MateRef) {
    let (doc, ids, store, body) = assembly("asm-r2a-row1", 2);
    // Coaxial about the shared +z, clocked at zero: prismatic along z.
    let (doc, _) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    // The seating face: a's outward normal is +z at z = 1; b's own
    // outward normal is −z at its origin. Opposed, so b lands facing
    // it, at the authored standoff (the offset runs along the TARGET
    // frame's own axis, which the opposed sense has already flipped —
    // so −1 lifts b clear rather than driving it in).
    let (doc, rest) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::PlanarRest { offset: -1.0 },
            AxisSense::Opposed,
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            None,
        ),
    );
    (doc, ids, store, rest)
}

/// A pair determined by ONE mate: frame coincidence, offset a unit up
/// the shared axis. The maintenance rows want this shape because
/// removing the single constraint is the split, and the document it
/// splits FROM is one that solved.
fn stacked_pair(
    label: &str,
) -> (
    ProfileDoc,
    Vec<RecipeNodeId>,
    MateRef,
    PartStore,
    RecipeNodeId,
) {
    let (doc, ids, store, body) = assembly(label, 2);
    let (doc, joint) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::FrameCoincidence,
            AxisSense::Aligned,
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            z_up(),
            None,
        ),
    );
    (doc, ids, joint, store, body)
}

#[test]
fn row1_a_coaxial_clocked_rest_pair_is_determined_and_evaluates() {
    let (doc, ids, store, rest) = determined_pair();
    assert_eq!(rest.placement, ids[1], "the constraints place instance 1");
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(ids[1]), None, "the pair solves");
    assert_eq!(
        poses.space(ids[1]),
        Some(Space::World),
        "pinned to an instance on the world, it is in the world"
    );
    let relative = poses.relative(ids[1]).expect("a solved pose");
    assert!(
        relative.bit_eq(&Frame::translation([0.0, 0.0, 2.0])),
        "the child sits at the seating height plus the standoff: {relative:?}"
    );
    assert!(
        poses
            .relative(ids[0])
            .expect("the world-pinned instance")
            .is_identity_bits(),
        "the world-pinned instance's own relative pose is the bit-exact identity"
    );

    let ev = run(&doc, &o);
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "two unit blocks, stacked, share only a face: {volume}"
    );
}

/// D9 at this layer: two independent evaluations of the same document
/// agree BIT for bit, and two saves produce identical bytes. (The
/// two-fresh-PROCESS variant is the document layer's row, where ASM-2A
/// put it — a process hosts one ε, so the cross-process claim needs
/// the store.)
#[test]
fn row1_evaluation_and_save_bytes_are_bit_identical_across_runs() {
    let (doc, _, store, _) = determined_pair();
    let o = with_resolver(store);
    let first = run(&doc, &o);
    let second = run(&doc, &o);
    let volume = |ev: &Evaluation<f64>| {
        let body = product(&doc, ev, Tol::witness()).expect("gathers");
        topo::mass_properties(&body, Tol::witness())
            .expect("props")
            .volume
    };
    assert_eq!(
        volume(&first).to_bits(),
        volume(&second).to_bits(),
        "two evaluations of one document agree bit for bit"
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text,
        save(&doc, &[], Tol::witness()).expect("saves"),
        "byte-stable"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mates round-trip bit for bit");
}

// ---- Row 2: a bundle that pins less is a loose copy ----

/// **A V-block leaves a loose copy, not a refusal**: two planar rests
/// on perpendicular planes fold to a prismatic freedom, so the bundle
/// pins less than the copy needs. The copy evaluates in a space of its
/// own and has no world pose.
#[test]
fn row2_a_v_block_leaves_a_loose_copy() {
    let (doc, ids, store, body) = assembly("asm-r2a-row2", 2);
    let mut doc = doc;
    for axis in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        doc = add(
            doc,
            &store,
            mate(
                body,
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Opposed,
                frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                None,
            ),
        )
        .0;
    }
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(ids[1]),
        None,
        "an under-pinned copy is no refusal"
    );
    assert_eq!(poses.space(ids[1]), Some(Space::Loose { root: ids[1] }));
    assert!(matches!(
        poses.placement(&doc, ids[1]),
        Err(editor_core::PoseRefusal::Loose { placement, root })
            if placement == ids[1] && root == ids[1]
    ));
    let ev = run(&doc, &o);
    assert!(
        matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))),
        "the loose copy evaluates: {:?}",
        ev.result(ids[1])
    );
    assert_eq!(ev.space(ids[1]), Space::Loose { root: ids[1] });
}

// ---- Row 3: CONTRADICTORY, naming both mates and the clash ----

#[test]
fn row3_a_gap_mismatched_planar_pair_refuses_contradictory() {
    let (doc, ids, store, body) = assembly("asm-r2a-row3", 2);
    let mut doc = doc;
    let mut mates = Vec::new();
    for height in [0.0, 1.0] {
        let (next, id) = add(
            doc,
            &store,
            mate(
                body,
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Opposed,
                frame([0.0, 0.0, height], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                None,
            ),
        );
        doc = next;
        mates.push(id);
    }
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(ids[1]).expect("the bundle refuses").clone();
    let editor_core::MateFault::Contradictory {
        held,
        added,
        predicate,
        clash,
    } = &fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(*held, mates[0], "the constraint already folded is named");
    assert_eq!(
        *added, mates[1],
        "the constraint that died against it is named"
    );
    assert_eq!(*predicate, "mate_member_translation_in_plane");
    let editor_core::Clash::Length { metres } = *clash else {
        panic!(
            "a translation predicate measures a LENGTH outright, so there is no lever: {clash:?}"
        );
    };
    assert!(
        (metres.abs() - 1.0).abs() < 1e-9,
        "the measured clash IS the authored gap mismatch: {metres}"
    );
    let message = fault.to_string();
    assert!(
        message.contains("the translation leaves the shared plane") && !message.contains(predicate),
        "the sentence says in words what the predicate found: {message}"
    );
    assert!(message.contains("clash"), "{message}");
}

// ---- Row 4: the constraint door, and no edit records a frame (A11 (2)) ----

/// Row 4b — removing the placing constraint is not refused, records no
/// frame, and leaves the orphan LOOSE: its bundle is empty, so it lives
/// in its own space and has no world pose.
#[test]
fn row4b_removing_the_placing_constraint_leaves_the_orphan_loose() {
    let (doc, ids, joint, store, _) = stacked_pair("asm-r2a-row4b");
    let doc = repin(doc, ids[0], &Frame::translation([0.0, 0.0, 4.0]));
    let before = doc.clone();
    let applied = apply(
        &doc,
        &DocEdit::RemoveConstraint {
            placement: joint.placement,
            constraint: joint.id,
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the constraint is removed");
    assert!(
        fixture::without_anonymous(&applied.maintenance).is_empty(),
        "no frame is recorded: {:?}",
        applied.maintenance
    );
    let poses = solve(&applied.doc, &with_resolver(store), Tol::witness());
    assert_eq!(poses.space(ids[1]), Some(Space::Loose { root: ids[1] }));
    assert!(matches!(
        poses.placement(&applied.doc, ids[1]),
        Err(editor_core::PoseRefusal::Loose { placement, .. }) if placement == ids[1]
    ));
    assert!(
        poses
            .placement(&applied.doc, ids[0])
            .expect("the world-pinned instance is placed")
            .bit_eq(&Frame::translation([0.0, 0.0, 4.0])),
        "the surviving copy keeps its world pin verbatim"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

/// Row 4c — deleting the instance a constraint targets is not refused
/// either: DM7 reports the constraint side it stranded, and the copy
/// that constraint placed has no pose.
#[test]
fn row4c_deleting_the_target_strands_the_constraint_on_it() {
    let (doc, ids, joint, store, _body) = stacked_pair("asm-r2a-row4c");
    let before = doc.clone();
    let applied = apply(
        &doc,
        &DocEdit::DeleteNode { id: ids[0] },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the target deletes");
    assert!(
        fixture::bundle(&applied.doc, ids[1])
            .iter()
            .any(|c| c.id == joint.id),
        "the constraint survives its target"
    );
    let rows = fixture::without_anonymous(&applied.maintenance);
    assert!(
        matches!(rows.last(), Some(Maintenance::StrandedSelection { .. })),
        "the constraint's selection of the target's face strands: {rows:?}"
    );
    assert_eq!(
        rows[..rows.len() - 1].to_vec(),
        vec![Maintenance::StrandedRead {
            node: doc.spoken(ids[1]),
            slot: editor_core::OperandSlot::Side {
                constraint: joint.id,
                side: editor_core::MateSide::A,
            },
            var: doc.spoken_var(doc.output(ids[0], 0).expect("the target's copies")),
        }],
        "the strands are the whole report: no frame is recorded"
    );
    let poses = solve(&applied.doc, &with_resolver(store), Tol::witness());
    assert!(
        poses.placement(&applied.doc, ids[1]).is_err(),
        "a copy placed by a stranded constraint has no pose"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

/// Row 4d — a lone instance's world pose is its world pin's frame, bit
/// for bit (A11 (5)), and a mate-less document round-trips identically.
#[test]
fn row4d_a_no_mates_document_round_trips_identically_below_the_header() {
    let (doc, ids, store, _) = assembly("asm-r2a-row4d", 3);
    let placed =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.25, [2.0, 3.0, 0.0], fixture::band())
            .expect("a literal axis has a definite direction");
    let doc = repin(doc, ids[2], &placed);
    let poses = solve(&doc, &with_resolver(store), Tol::witness());
    assert!(
        poses
            .placement(&doc, ids[2])
            .expect("placed")
            .bit_eq(&placed),
        "a lone instance returns its world pin's frame bit for bit"
    );
    assert!(
        poses
            .placement(&doc, ids[0])
            .expect("placed")
            .is_identity_bits(),
        "an instance pinned to the world at the identity sits at the identity"
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the round trip is bit-exact");
    let body = |t: &str| t.split_once('\n').expect("a header").1.to_string();
    assert_eq!(
        body(&text),
        body(&save(&back, &[], Tol::witness()).expect("saves")),
        "every byte below the header survives the round trip"
    );
}

// ---- Row 4e: the cut takes whole placements (ASM-4 rider ii) ----

/// Two pairs: `{i0,i1}` and `{i2,i3}`, each joined by one constraint
/// in the second instance's bundle, each first instance pinned to the
/// world at a pose of its own.
fn two_pairs() -> (ProfileDoc, Vec<RecipeNodeId>, PartStore) {
    let (doc, ids, store, body) = assembly("asm-r2a-clusters", 4);
    let mut doc = doc;
    for (a, b) in [(0, 1), (2, 3)] {
        doc = add(
            doc,
            &store,
            mate(
                body,
                ids[a],
                ids[b],
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                z_up(),
                None,
            ),
        )
        .0;
    }
    let doc = repin(doc, ids[0], &Frame::translation([3.0, 0.0, 0.0]));
    let doc = repin(doc, ids[2], &Frame::translation([7.0, 0.0, 0.0]));
    (doc, ids, store)
}

/// Row 4e — a cut of ONE WHOLE pinned pair moves as selected (A4): the
/// part's world-pinned instance keeps its pin, and the remainder
/// instance sits on the world at the identity.
#[test]
fn row4e_a_whole_pair_cut_moves_as_selected() {
    use std::collections::BTreeSet;
    let (doc, ids, store) = two_pairs();
    let o = with_resolver(store);
    let cut = BTreeSet::from([ids[0], ids[1]]);
    let out = fixture::split_world(
        &doc,
        &cut,
        editor_core::DocumentId::derive("asm-r2a-4e"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a whole-pair cut splits");
    assert_eq!(
        fixture::offset_of(&out.part, out.node_map[&ids[0]]),
        Some(Placement::literal(&Frame::translation([3.0, 0.0, 0.0]))),
        "the part's world-pinned instance keeps its pin"
    );
    let instance = *out
        .remainder
        .ids()
        .iter()
        .find(|id| {
            doc.node(**id).is_none()
                && matches!(out.remainder.node(**id), Some(Node::InstantiatePart { .. }))
        })
        .expect("the remainder gained an instance");
    assert_eq!(
        fixture::offset_of(&out.remainder, instance),
        Some(Placement::IDENTITY),
        "the remainder instance sits on the world at the identity"
    );
    assert_eq!(
        fixture::offset_of(&out.remainder, ids[2]),
        Some(Placement::literal(&Frame::translation([7.0, 0.0, 0.0]))),
        "the untouched pair keeps its own pin"
    );
}

// ---- Row 5: the closure enumeration, executed ----

#[test]
fn row5_the_closure_set_is_closed_under_intersection() {
    use editor_core::Subgroup;
    use geom_core::linalg::{Point3, UnitVec3, Vec3};
    use geom_core::predicate::Band;
    let band = Band::linear(Tol::witness()).expect("a band");
    let unit = |v| UnitVec3::new(v, FIXTURE_MATE_AXIS, band).expect("a basis vector");
    let x = unit(Vec3::new(1.0, 0.0, 0.0));
    let y = unit(Vec3::new(0.0, 1.0, 0.0));
    let z = unit(Vec3::new(0.0, 0.0, 1.0));
    let o = Point3::origin();
    let off = Point3::new(0.0, 1.0, 0.0);
    let planar = |n| Subgroup::Planar { normal: n };
    let cyl = |p, d| Subgroup::Cylindrical {
        point: p,
        direction: d,
    };
    let prism = |d| Subgroup::Prismatic { direction: d };
    let rev = |p, d| Subgroup::Revolute {
        point: p,
        direction: d,
    };
    let meet = |a, b| {
        editor_core::mate::coset::intersect_subgroups(a, b, band, unit_arm()).expect("decided")
    };
    // The universal rows: SE(3) is the identity, empty absorbs,
    // trivial is the zero.
    for g in [
        Subgroup::Se3,
        planar(z),
        cyl(o, z),
        prism(z),
        rev(o, z),
        Subgroup::Trivial,
        Subgroup::Empty,
    ] {
        assert_eq!(meet(Subgroup::Se3, g), g, "X ∩ SE(3) = X");
        assert_eq!(meet(g, Subgroup::Se3), g, "unordered");
        assert_eq!(
            meet(Subgroup::Empty, g),
            Subgroup::Empty,
            "X ∩ empty = empty"
        );
        if !matches!(g, Subgroup::Empty) {
            assert_eq!(meet(Subgroup::Trivial, g), Subgroup::Trivial);
        }
    }
    // Table row 1 — planar ∩ planar.
    assert_eq!(meet(planar(z), planar(z)), planar(z));
    assert_eq!(meet(planar(x), planar(y)), prism(z), "the V-block");
    // Table row 2 — planar ∩ cylindrical.
    assert_eq!(meet(planar(z), cyl(o, z)), rev(o, z), "the pin in the hole");
    assert_eq!(meet(planar(z), cyl(o, x)), prism(x), "the slot");
    assert_eq!(
        meet(planar(z), cyl(o, unit(Vec3::new(1.0, 0.0, 1.0)))),
        Subgroup::Trivial,
        "a generic angle kills both freedoms"
    );
    // Table row 3 — cylindrical ∩ cylindrical.
    assert_eq!(meet(cyl(o, z), cyl(o, z)), cyl(o, z), "the same line");
    assert_eq!(meet(cyl(o, z), cyl(off, z)), prism(z), "parallel, distinct");
    assert_eq!(meet(cyl(o, z), cyl(o, x)), Subgroup::Trivial, "concurrent");
    assert_eq!(meet(cyl(o, z), cyl(off, x)), Subgroup::Trivial, "skew");
    // Table row 4 — prismatic against everything.
    assert_eq!(meet(prism(x), planar(z)), prism(x), "in-plane");
    assert_eq!(meet(prism(z), planar(z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), cyl(o, z)), prism(z));
    assert_eq!(meet(prism(x), cyl(o, z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), prism(z)), prism(z));
    assert_eq!(meet(prism(x), prism(z)), Subgroup::Trivial);
    assert_eq!(meet(prism(z), rev(o, z)), Subgroup::Trivial);
    // Table row 5 — revolute against everything.
    assert_eq!(meet(rev(o, z), planar(z)), rev(o, z));
    assert_eq!(meet(rev(o, z), planar(x)), Subgroup::Trivial);
    assert_eq!(meet(rev(o, z), cyl(o, z)), rev(o, z));
    assert_eq!(meet(rev(o, z), cyl(off, z)), Subgroup::Trivial);
    assert_eq!(meet(rev(o, z), rev(o, z)), rev(o, z));
    assert_eq!(meet(rev(o, z), rev(off, z)), Subgroup::Trivial);
    // Closedness, stated as the obligation it is: every result above is
    // itself a member of the closure set.
    for a in [planar(z), cyl(o, z), prism(z), rev(o, z), Subgroup::Trivial] {
        for b in [planar(x), cyl(off, x), prism(y), rev(o, y), Subgroup::Se3] {
            let out = meet(a, b);
            assert!(
                out.dimension().is_some() || matches!(out, Subgroup::Empty),
                "the closure is closed: {} ∩ {} = {}",
                a.name(),
                b.name(),
                out.name()
            );
        }
    }
}

// ---- Row 5b: the COSET level — invariant matches, not just types ----
//
// Row 5 enumerates SUBGROUP types. The table's other half is the
// representative: entry 3's "inter-axis displacement invariants match
// A vs B (decided) → prismatic, else empty". Those matches had no
// direct rows, and that gap is exactly where review MAJOR-1 hid — the
// candidate pinned the held representative's clocking instead of
// solving the free angle about the shared axis, so an assemblable
// two-pin pattern refused CONTRADICTORY. These rows are at the coset
// level: same subgroup types, different representatives, different
// verdicts.

/// A pin: a coaxial mate whose two axes are authored at `a_at` on the
/// A side and `b_at` on the B side, both along +z.
fn pin(
    body: RecipeNodeId,
    a: RecipeNodeId,
    b: RecipeNodeId,
    a_at: [f64; 3],
    b_at: [f64; 3],
) -> Mate<Formula> {
    mate(
        body,
        a,
        b,
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        frame(a_at, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        frame(b_at, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        None,
    )
}

/// Two pins, inter-axis invariants MATCHING (both distances 1) but the
/// B-side pattern clocked 90° round: the table's entry 3
/// parallel-distinct row, which requires **prismatic**. Before the
/// MAJOR-1 fix this refused `Contradictory { mate_member_point_on_axis,
/// clash 1.414… }` — the candidate was in the held coset but was not a
/// witness of the intersection. The prismatic fold pins less than the
/// copy needs, so the copy is loose; the coset row below pins the
/// witness itself.
#[test]
fn row5b_two_pins_clocked_apart_but_invariant_matched_fold_to_prismatic() {
    let (doc, ids, store, body) = assembly("asm-r2a-row5b", 2);
    let (doc, _) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    );
    // A's second pin sits at +x; B's at +y — same radius, quarter turn.
    let (doc, _) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(ids[1]),
        None,
        "an assemblable two-pin pair is no contradiction"
    );
    assert_eq!(
        poses.space(ids[1]),
        Some(Space::Loose { root: ids[1] }),
        "the slide along the pins' shared axis leaves the copy loose"
    );
}

/// The same shape at the COSET door, so the witness itself is pinned:
/// the folded representative must be the quarter turn that carries B's
/// pattern onto A's, not the identity the held mate happened to state.
#[test]
fn row5b_the_folded_representative_is_the_solved_clocking() {
    use editor_core::mate::coset::{Coset, Subgroup, intersect};
    use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
    use geom_core::predicate::Band;
    let band = Band::linear(Tol::witness()).expect("a band");
    let z = UnitVec3::new(Vec3::new(0.0, 0.0, 1.0), FIXTURE_MATE_AXIS, band).expect("unit z");
    // Pin 1 pins the shared axis through the origin; pin 2's A-side
    // axis is at +x while its representative carries B's at +y.
    let held = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::origin(),
            direction: z,
        },
        representative: Affine3::identity(),
    };
    let added = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::new(1.0, 0.0, 0.0),
            direction: z,
        },
        representative: Affine3::from_parts(Mat3::identity(), Vec3::new(1.0, -1.0, 0.0)),
    };
    let out = intersect(held, added, band, unit_arm()).expect("the pair is assemblable");
    assert_eq!(out.subgroup.name(), "prismatic");
    let turned = out
        .representative
        .transform_point(Point3::new(0.0, 1.0, 0.0));
    assert!(
        (turned - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-12,
        "the witness carries B's second pin onto A's: {turned:?}"
    );
}

/// Invariants that do NOT match: A's pins are 1 apart, B's are 2. The
/// table says empty, and the refusal must quote the MEASURED clash —
/// the radius difference, 1.
#[test]
fn row5b_mismatched_inter_axis_invariants_refuse_contradictory() {
    let (doc, ids, store, body) = assembly("asm-r2a-row5b-mismatch", 2);
    let (doc, first) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    );
    let (doc, second) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]),
    );
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(ids[1]).expect("the bundle refuses").clone();
    let editor_core::MateFault::Contradictory {
        held,
        added,
        predicate,
        clash,
    } = &fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!((*held, *added), (first, second), "both mates are named");
    assert_eq!(*predicate, "mate_member_point_on_axis");
    let editor_core::Clash::Length { metres } = *clash else {
        panic!("a point-on-axis offset is a LENGTH outright, so there is no lever: {clash:?}");
    };
    assert!(
        (metres.abs() - 1.0).abs() < 1e-9,
        "the clash IS the inter-axis invariant difference (2 − 1): {metres}"
    );
}

/// A rest plus two pins — A11 rule 1's own fully-determined territory,
/// and the shape MAJOR-1 falsely refused. The plate seats at the rest
/// and both pins land, so the fold is DETERMINED and the pose is the
/// seating height.
#[test]
fn row5b_a_rest_and_two_pins_determine_the_plate() {
    let (doc, ids, store, body) = assembly("asm-r2a-row5b-plate", 2);
    let (doc, _) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::PlanarRest { offset: -1.0 },
            AxisSense::Opposed,
            frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            None,
        ),
    );
    let (doc, _) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    );
    let (doc, _) = add(
        doc,
        &store,
        pin(body, ids[0], ids[1], [1.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
    );
    let o = with_resolver(store);
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(ids[1]), None, "rest + two pins determine");
    let relative = poses.relative(ids[1]).expect("a solved pose");
    // VALUE-exact, not bit-exact, and the difference is the point: this
    // pose comes out of the clocking solve (`atan2` then
    // `rotation_about`), where row 1's came out of the forced-rotation
    // arm with no trigonometry in it. A closed form through the
    // transcendentals leaves a rounding step — here the residue of
    // turning the rest's half-turn back, sin(π) ≈ 1.2e-16 off the
    // identity. D9 is untouched: the same expression yields the same
    // bits on every run, which is what determinism asks for, and the
    // pncad cross-process row proves it across processes.
    assert!(
        near(relative, Frame::translation([0.0, 0.0, 2.0]), 1e-12),
        "the plate seats at the rest, unturned (both patterns agree): {relative:?}"
    );
    let ev = run(&doc, &o);
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
}

// ---- Row 6: the A12 partition consequences ----

#[test]
fn row6a_mated_instances_share_an_a9_component() {
    let (doc, ids, store, body) = assembly("asm-r2a-row6a", 2);
    let apart = relative_freedom_components(&doc);
    assert!(
        !apart
            .iter()
            .any(|c| c.contains(&ids[0]) && c.contains(&ids[1])),
        "unconstrained instances are relatively free: {apart:?}"
    );
    let (doc, mate_ref) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    let together = relative_freedom_components(&doc);
    assert!(
        together
            .iter()
            .any(|c| c.contains(&ids[0]) && c.contains(&ids[1])),
        "the constraint COUPLES the components: {together:?}"
    );
    assert_eq!(
        fixture::mate_members(&doc),
        vec![(mate_ref, ids[0]), (mate_ref, ids[1])],
        "each side lands on its instance"
    );
}

#[test]
fn row6b_a_constraint_add_and_remove_place_nothing() {
    let (doc, ids, store, body) = assembly("asm-r2a-row6b", 2);
    let o = with_resolver(store.clone());
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let placed = doc.placements();
    assert_eq!(placed.len(), 2, "both instances are placements");
    let (doc, mate_ref) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    assert_eq!(
        doc.placements(),
        placed,
        "a constraint makes no placement (A10)"
    );
    // A lone coaxial leaves the copy loose; removing it takes nothing
    // the door has to record, so the door takes it and reports nothing.
    let applied = doc
        .apply(
            &DocEdit::RemoveConstraint {
                placement: mate_ref.placement,
                constraint: mate_ref.id,
            },
            Tol::witness(),
            &reach,
        )
        .expect("removing a constraint is admitted");
    assert!(
        fixture::without_anonymous(&applied.maintenance).is_empty(),
        "{:?}",
        applied.maintenance
    );
    let doc = applied.doc;
    assert_eq!(doc.placements(), placed, "and leaves them as it found them");
}

/// **The gather takes the placements and nothing else**: the
/// constraints are no nodes of the document, and the product is the
/// two instances.
#[test]
fn row6c_the_gather_takes_the_placements_alone() {
    let (doc, ids, store, _) = determined_pair();
    let ev = run(&doc, &with_resolver(store));
    assert_eq!(doc.len(), ids.len(), "the constraints are no nodes");
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("props")
        .volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "the gather took the two instances: {volume}"
    );
}

#[test]
fn row6d_a_dangling_head_contributes_no_edge_and_the_solve_refuses_typed() {
    let (doc, ids, store, body) = assembly("asm-r2a-row6d", 2);
    let (doc, mate_ref) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[0] });
    assert_eq!(
        fixture::mate_members(&doc),
        vec![(mate_ref, ids[1])],
        "the stranded head contributes NO edge (N5)"
    );
    let o = with_resolver(store);
    let fault = solve(&doc, &o, Tol::witness())
        .fault(ids[1])
        .expect("the solve refuses")
        .clone();
    assert!(
        matches!(
            &fault,
            editor_core::MateFault::SideUnresolved {
                side: editor_core::MateSide::A,
                ..
            }
        ),
        "expected the stranded side, got {fault:?}"
    );
    assert!(fault.to_string().contains("re-author the side"), "{fault}");
    let ev = run(&doc, &o);
    assert!(
        matches!(
            ev.node_error(ids[1]).map(|e| &e.kind),
            Some(NodeErrorKind::UnresolvedRead { .. })
        ),
        "{:?}",
        ev.node_error(ids[1])
    );
}

// ---- Row 6f–6j: A12's repair path, and the three doors ----
// ---- that keep a head honest                              ----

/// A side whose body is deleted is an unresolved read (D10), and its
/// repair is the side's slot door: re-pointing the side at a face of
/// another instance resolves it again. No door re-points it for the
/// author.
#[test]
fn row6f_the_slot_door_repairs_a_stranded_side() {
    let (doc, ids, store, body) = assembly("asm-r2a-rebind-only-ref", 3);
    let (doc, mate_ref) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[0] });
    assert_eq!(
        fixture::mate_members(&doc),
        vec![(mate_ref, ids[1])],
        "the stranded head contributes NO edge (N5)"
    );
    let applied = doc
        .apply(
            &DocEdit::SetParam {
                node: mate_ref.placement,
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Side {
                    constraint: mate_ref.id,
                    side: editor_core::MateSide::A,
                }),
                value: editor_core::SlotValue::Read(
                    fixture::head(in_part(ids[2], body, CapEnd::Start)).into(),
                ),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a side is re-pointed by its slot door");
    let mut members = fixture::mate_members(&applied.doc);
    members.sort();
    let mut expected = vec![(mate_ref, ids[1]), (mate_ref, ids[2])];
    expected.sort();
    assert_eq!(
        members, expected,
        "the repaired side reads through the instance it now names"
    );
    assert!(
        applied.record.structural,
        "a constraint's read moved, so its placement's content key moved with it"
    );
    let o = with_resolver(store);
    assert!(
        solve(&applied.doc, &o, Tol::witness())
            .fault(ids[1])
            .is_none_or(|f| !matches!(f, editor_core::MateFault::SideUnresolved { .. })),
        "and the solve no longer refuses the side"
    );
}

/// A constraint side read at a node that never existed is a typo,
/// refused at the constraint door as the unresolved read it is.
#[test]
fn row6h_the_constraint_door_refuses_a_head_naming_no_node() {
    let (doc, ids, _, body) = assembly("asm-r2a-mate-insert-door", 2);
    let ghost = RecipeNodeId::new(0, 9_999);
    let err = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: ids[1],
                mate: Box::new(mate(
                    body,
                    ghost,
                    ids[1],
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    z_up(),
                    z_up(),
                    Some(0.0),
                )),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("the head names no node");
    assert!(
        matches!(&err, EditError::UnresolvedInput { input } if input.id() == ghost),
        "{err:?}"
    );
}

/// A saved file is DATA: a constraint side's name naming an id past the
/// mint counter is as corrupt as a declared pair naming one, and worse
/// to let in — `Rebind`'s source door refuses a never-minted id, so the
/// document would load unrepairable.
#[test]
fn row6i_the_load_check_refuses_a_mate_head_the_mint_never_minted() {
    let (doc, ids, store, body) = assembly("asm-r2a-mate-wire-id", 3);
    let (doc, mate_ref) = add(
        doc,
        &store,
        mate(
            body,
            ids[2],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // Doctored BY PATH, not by position: the `a` head of this
    // constraint, reached through the wire's own structure, so a
    // fixture or field-order change breaks the probe instead of
    // silently moving it onto an unrelated id.
    let corrupt = doctored(&text, |wire| {
        let read: editor_core::VarId = serde_json::from_value(
            wire["snapshot"]["nodes"][mate_ref.placement.0.to_string()]["InstantiatePart"]
                ["constraints"][0]["mate"]["a"]["face"]
                .clone(),
        )
        .expect("a constraint's face side is a variable id");
        let head = &mut wire["snapshot"]["vars"][read.0.to_string()]["def"]["Select"]["names"][0];
        assert_eq!(
            head["node"],
            serde_json::json!(ids[2].0),
            "the probe is aimed at the `a` side's name"
        );
        head["node"] = serde_json::json!(RecipeNodeId::new(0, 99).0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(editor_core::PersistError::Snapshot(editor_core::SnapshotError::NodeNotMinted {
            id,
        })) => assert_eq!(id.id(), RecipeNodeId::new(0, 99)),
        other => panic!("expected NodeNotMinted, got {other:?}"),
    }
}

/// The name-level edit door (`apply_with_names`, PR 3's R6 obligation)
/// reads a constraint side's selection under the rule it has always
/// applied to a declared pair: checkable exactly when the minting node
/// evaluated `Ok`, deferred otherwise. An instance-qualified head the
/// tables carry passes; a role the part's product does not have is
/// refused there rather than at the solve.
#[test]
fn row6j_the_name_door_reads_a_mates_heads_like_any_payload_name() {
    let (doc, ids, store, body) = assembly("asm-r2a-mate-name-door", 2);
    let ev = run(&doc, &with_resolver(store));
    let add = |mate: Mate<Formula>| DocEdit::AddConstraint {
        placement: ids[1],
        mate: Box::new(mate),
        fresh: Vec::new(),
    };
    let good = mate(
        body,
        ids[0],
        ids[1],
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        z_up(),
        z_up(),
        Some(0.0),
    );
    assert!(
        editor_core::apply_with_names(
            &doc,
            &add(good.clone()),
            &ev,
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .is_ok(),
        "the instance-qualified heads resolve in the instance's own table"
    );
    // The same head, wearing a role the part product has no entity for.
    let bogus = StableName {
        kind: EntityKind::Face,
        node: ids[0],
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: body,
                path: vec![RoleSeg::Lateral(fixture::no_piece().into())],
            }
            .into(),
        }],
    };
    let err = editor_core::apply_with_names(
        &doc,
        &add(Mate {
            a: editor_core::SideRead::Face(fixture::head(bogus.clone()).into()),
            ..good
        }),
        &ev,
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_err();
    assert_eq!(
        err,
        EditError::NameUnresolvedInEvaluation {
            name: doc.spoken_name(&bogus)
        },
        "the constraint head is checkable, so it is checked"
    );
}

// ---- Row 7: every refusal arm ----

#[test]
fn row7a_a_standalone_clocking_refuses_typed() {
    let (doc, ids, _store, body) = assembly("asm-r2a-row7a", 2);
    // The table's static gap is a fact about the constraint alone, so
    // the constraint door refuses it where it is authored, carrying
    // the solve's own fault — and asks no reach for it.
    let fault = door_refusal(
        &doc,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Clocking,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.5),
        ),
    );
    let editor_core::MateFault::TableLacks { what, .. } = &fault else {
        panic!("expected TableLacks, got {fault:?}");
    };
    assert!(what.contains("standalone clocking"), "{what}");
    assert!(fault.to_string().contains("coset table"), "{fault}");
}

#[test]
fn row7b_a_clocking_rider_on_a_planar_rest_refuses_typed() {
    let (doc, ids, _store, body) = assembly("asm-r2a-row7b", 2);
    let fault = door_refusal(
        &doc,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Opposed,
            z_up(),
            z_up(),
            Some(0.5),
        ),
    );
    assert!(
        matches!(&fault, editor_core::MateFault::TableLacks { what, .. }
                 if what.contains("planar rest")),
        "{fault:?}"
    );
}

/// The `Fit` class is not representable in this kernel — the variant is
/// specified and not built — so the refusal is demonstrated at the WIRE
/// door, where an unknown spelling arrives. The sentence it says is
/// [`topo::FIT_DEFERRAL`], VERBATIM, and it is the same sentence the
/// solve door says when a future class reaches it.
#[test]
fn row7c_an_unadmitted_class_refuses_naming_the_fit_deferral() {
    let (doc, ids, store, body) = assembly("asm-r2a-row7c", 2);
    let (doc, _) = add(
        doc,
        &store,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(0.0),
        ),
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let doctored = text.replacen("\"rest\"", "\"fit\"", 1);
    let message = match load(&doctored, Tol::witness()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("an unknown class spelling must refuse"),
    };
    assert!(
        message.contains(topo::FIT_DEFERRAL),
        "the refusal quotes the kernel's deferral sentence verbatim: {message}"
    );
    assert!(message.contains("fit"), "{message}");
}

/// The `Fit` deferral sentence a refusal steers to is the KERNEL's,
/// quoted verbatim through the re-export chain — one wording, and the
/// mate verification cites the same constant.
#[test]
fn the_fit_deferral_sentence_is_quoted_not_paraphrased() {
    assert_eq!(editor_core::FIT_DEFERRAL, topo::FIT_DEFERRAL);
    assert_eq!(editor_core::names::FIT_DEFERRAL, topo::FIT_DEFERRAL);
    assert!(editor_core::FIT_DEFERRAL.contains("Fit { gap }"));
    assert!(editor_core::FIT_DEFERRAL.contains("not yet built"));
}

/// Two planar rests whose normals differ by an angle whose induced
/// displacement at the mate's lever arm is 3ε — definitely inside
/// (ε, Kε) for the default K = 10 — in instance 1's bundle; the
/// constraints, in bundle order.
fn in_band_pair(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>, PartStore, Vec<MateRef>) {
    let eps = geom_core::Tol::witness().get().eps;
    let tilt = 3.0 * eps;
    let (doc, ids, store, body) = assembly(label, 2);
    let mut doc = doc;
    let mut mates = Vec::new();
    for axis in [[0.0, 0.0, 1.0], [tilt, 0.0, 1.0]] {
        let (next, id) = add(
            doc,
            &store,
            mate(
                body,
                ids[0],
                ids[1],
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Opposed,
                frame([0.0, 0.0, 0.0], axis, [0.0, 1.0, 0.0]),
                frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]),
                None,
            ),
        );
        doc = next;
        mates.push(id);
    }
    (doc, ids, store, mates)
}

/// The typed escalation: a case split constructed to land INSIDE the
/// ambiguity band at the run's own ε, so the row holds at every ε CI
/// sweeps.
#[test]
fn row7d_an_in_band_case_split_escalates_typed() {
    let (doc, ids, store, mates) = in_band_pair("asm-r2a-row7d");
    let o = with_resolver(store);
    let fault = solve(&doc, &o, Tol::witness())
        .fault(ids[1])
        .expect("the band escalates")
        .clone();
    let editor_core::MateFault::Indeterminate { mate, diag } = &fault else {
        panic!("expected the typed escalation, got {fault:?}");
    };
    assert_eq!(*mate, mates[1], "the constraint being folded when it fired");
    assert_eq!(
        diag.predicate,
        Some("mate_axes_parallel"),
        "the escalation NAMES the predicate that could not decide"
    );
    assert!(
        fault.to_string().contains("could not be decided"),
        "{fault}"
    );
}

/// **The mate solve's escalation is on the refused placement's own
/// log, and on no other.** The solve is a whole-document computation
/// that runs before any node's frame opens, so it keeps each decision
/// for the placement whose bundle it decided, and that placement's
/// node splices it into its own frame. The in-band case split here is
/// decided while the fold adds `mates[1]` to the bundle's
/// intersection, so it is instance 1's: on that placement's
/// `NodeError`, on no other node's log, and not in a frame the caller
/// holds around the evaluation.
#[test]
fn row7e_a_mate_solve_escalation_is_on_the_refused_placements_own_log_and_not_in_an_outer_frame() {
    let (doc, ids, store, mates) = in_band_pair("asm-r2a-row7e");
    let outer = geom_core::k_stats::Bracket::open();
    let ev = run(&doc, &with_resolver(store));
    let outside = outer.finish();
    let named = |escalations: &[geom_core::k_stats::Escalation]| {
        escalations
            .iter()
            .filter(|e| e.predicate() == "mate_axes_parallel")
            .count()
    };
    let mut carriers = Vec::new();
    for (id, result) in &ev.nodes {
        let on_node = match result {
            NodeResult::Ok(v) => named(&v.escalations),
            NodeResult::Failed(e) => named(&e.escalations),
            NodeResult::Poisoned { .. } => 0,
        };
        carriers.extend(std::iter::repeat_n(*id, on_node));
    }
    assert_eq!(
        carriers,
        vec![ids[1]],
        "the escalation is on the refused placement's log, once, and on no other node's"
    );
    let editor_core::MateFault::Indeterminate { mate: refused, .. } = mate_fault(&ev, ids[1])
    else {
        panic!("the placement fails typed through the error enum");
    };
    assert_eq!(
        refused, mates[1],
        "the fault names the constraint whose placement's log holds it"
    );
    assert_eq!(
        named(&outside.escalations),
        0,
        "no frame around the evaluation sees the solve's escalation: {:?}",
        outside.escalations
    );
}

#[test]
fn row7e_a_self_mate_refuses_naming_the_instance_it_names_twice() {
    let (doc, ids, _store, body) = assembly("asm-r2a-row7e-self", 2);
    // One member on both sides is a fact about the constraint alone:
    // the constraint door refuses it with the solve's own fault.
    let fault = door_refusal(
        &doc,
        mate(
            body,
            ids[1],
            ids[1],
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            None,
        ),
    );
    assert!(
        matches!(&fault, editor_core::MateFault::SelfMate { instance, .. } if *instance == ids[1]),
        "{fault:?}"
    );
    assert!(fault.to_string().contains("a PAIR"), "{fault}");
}

/// A non-finite number never enters a constraint: the authored-vectors
/// door refuses it typed, a literal frame offset holding one refuses at
/// the constraint door as a placement's literal does (named by its
/// constraint, side and step), and a non-finite rider refuses as a
/// non-finite alignment.
#[test]
fn row7f_a_non_finite_alignment_refuses_at_the_edit_door() {
    assert!(matches!(
        MateFrame::<Formula>::authored(
            [f64::NAN, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            Tol::witness()
        ),
        Err(geom_core::FrameError::NonFiniteLength { .. })
    ));
    let (doc, ids, _, body) = assembly("asm-r2a-row7f", 2);
    let insert = |a: MateFrame<Formula>, clocking| {
        apply(
            &doc,
            &DocEdit::AddConstraint {
                placement: ids[1],
                mate: Box::new(mate(
                    body,
                    ids[0],
                    ids[1],
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    a,
                    z_up(),
                    clocking,
                )),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a non-finite alignment never enters the document")
    };
    let literal = MateFrame::on_part(Placement::literal(&Frame::translation([
        f64::NAN,
        0.0,
        0.0,
    ])));
    let refusal = insert(literal, None);
    assert!(
        matches!(
            refusal,
            editor_core::EditError::NonFinitePlacement {
                at: editor_core::FrameSite::MateStep {
                    side: editor_core::MateSide::A,
                    index: 0,
                    ..
                },
                ..
            }
        ),
        "{refusal:?}"
    );
    let refusal = insert(z_up(), Some(f64::NAN));
    assert!(
        matches!(refusal, editor_core::EditError::NonFiniteAlignment { .. }),
        "{refusal:?}"
    );
}

/// A frame coincidence with a nonzero clocking rider: the coincidence
/// has already pinned the roll, so the table refuses the rider — and
/// the refusal names ONE constraint, because one constraint is at
/// fault. The magnitude the predicate decided on is the roll LEVERED by
/// the contact arm, so the message carries both halves and the reader
/// can multiply them back into the metre figure.
#[test]
fn row7g_a_self_contradictory_rider_names_one_mate_and_its_lever() {
    let (doc, ids, store, body) = assembly("asm-r2a-row7g", 2);
    // The rider on a coincidence is DECIDED over the constraint's own
    // lever, so the door asks the reach for the two parts and refuses
    // the contradiction where the constraint is authored.
    let o = with_resolver(store);
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (at, fault) = fixture::at_the_door(
        &doc,
        &reach,
        mate(
            body,
            ids[0],
            ids[1],
            MatePrimitive::FrameCoincidence,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            Some(core::f64::consts::FRAC_PI_2),
        ),
    )
    .expect_err("the rider contradicts the coincidence");
    assert_eq!(at, ids[1], "the door names the placement");
    let editor_core::MateFault::Contradictory {
        held,
        added,
        predicate,
        clash,
    } = &fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(
        held, added,
        "the constraint contradicts ITSELF, so it stands on both sides"
    );
    assert_eq!(held.placement, ids[1]);
    assert_eq!(*predicate, "mate_clocking_redundant");
    let editor_core::Clash::Levered(editor_core::Lever::Roll { radians, arm }) = *clash else {
        panic!(
            "the clocking clash is levered by an authored ROLL, not measured as a length: {clash:?}"
        );
    };
    assert!((radians - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
    assert_eq!(
        clash.deviation(),
        Some(radians * arm),
        "the deviation IS the product of the halves, to the bit"
    );
    let message = fault.to_string();
    assert!(
        message.contains("contradicts itself"),
        "one constraint at fault is named ONCE: {message}"
    );
    assert!(
        message.contains(&format!("a roll of {radians} rad"))
            && message.contains(&format!("on a {arm} m arm")),
        "the levered magnitude names its roll and the arm it was decided at: {message}"
    );
}
