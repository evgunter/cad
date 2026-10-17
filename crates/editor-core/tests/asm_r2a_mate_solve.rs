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
//! it moves. A mate relates two poses read off the instances' geometry
//! — a cap's plane, a lateral edge's line, a frame through an edge and
//! a corner — and sets no number; a value sets a freedom it leaves.
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
    ContactClass, Coord, DocEdit, DocumentId, EditError, EntityKind, EvalOptions, Evaluation,
    Frame, Maintenance, Mate, MateFault, MateRef, MateSide, NameRef, Node, NodeErrorKind,
    NodeResult, Operand, ProfileDoc, ProfileVertexRef, RecipeNodeId, RoleSeg, SlideAxis, Space,
    StableName, Value, apply, load, product, relative_freedom_components, save,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::{
    FIXTURE_MATE_AXIS, ang, flip, head, insert, len, on_frame, plane, point_of, rest, run, solve,
    square, step,
};
use geom_core::Tol;

/// A one-metre lever, through the one door a lever is formed by.
fn unit_arm() -> editor_core::mate::coset::Arm {
    editor_core::mate::coset::Arm::of(0.0, 1.0).expect("a metre is in range")
}

// ---- Substrate ----

/// A one-solid part: a unit square about the origin extruded 1 tall,
/// and its body.
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

/// **The stored part and the names its instances are read through**:
/// the store, the body, the part's own roles for two
/// adjacent walls (segments 0 and 1) and for the profile's four corners
/// in loop order (adjacent corners a unit apart, opposite ones `√2`).
#[derive(Clone)]
struct Kit {
    store: PartStore,
    body: RecipeNodeId,
    walls: [RoleSeg; 2],
    corners: [ProfileVertexRef; 4],
}

impl Kit {
    fn opts(&self) -> EvalOptions {
        with_resolver(self.store.clone())
    }

    /// The part's own name for `seg` of kind `kind`, as its placement
    /// carries it, worn inside `instance`'s `InPart`.
    fn name(&self, kind: EntityKind, instance: RecipeNodeId, seg: RoleSeg) -> StableName {
        let placed = StableName {
            kind,
            node: placement_of(self.body),
            path: vec![RoleSeg::Placed {
                of: NameRef::new(StableName {
                    kind,
                    node: self.body,
                    path: vec![seg],
                }),
            }],
        };
        StableName {
            kind,
            node: instance,
            path: vec![RoleSeg::InPart {
                of: NameRef::new(placed),
            }],
        }
    }

    /// The cap `end` of `instance`.
    fn cap(&self, instance: RecipeNodeId, end: CapEnd) -> StableName {
        in_part(instance, self.body, end)
    }

    /// Wall `k` of `instance`.
    fn wall(&self, instance: RecipeNodeId, k: usize) -> StableName {
        self.name(EntityKind::Face, instance, self.walls[k].clone())
    }

    /// The lateral edge at corner `v` of `instance`: a vertical line.
    fn edge(&self, instance: RecipeNodeId, v: usize) -> StableName {
        self.name(
            EntityKind::Edge,
            instance,
            RoleSeg::LateralEdge(self.corners[v]),
        )
    }

    /// The corner `v` of `instance`'s cap `end`.
    fn corner(&self, instance: RecipeNodeId, end: CapEnd, v: usize) -> StableName {
        self.name(
            EntityKind::Vertex,
            instance,
            RoleSeg::CapVertex(end, self.corners[v]),
        )
    }
}

/// **A read of `name` in the instance it is headed at.**
fn read(name: StableName) -> Operand {
    Operand::select(Operand::Node(name.node), vec![name])
}

/// An assembly of `n` instances of one part, and the kit that reads
/// them. Only the first is pinned to the world at the identity — the
/// rest sit where their constraints put them, each a loose copy until
/// one does — so a constraint between the first and another moves the
/// other onto it.
fn assembly(label: &str, n: usize) -> (ProfileDoc, Vec<RecipeNodeId>, Kit) {
    let mut store = PartStore::default();
    let (part_doc, body) = part(&format!("{label}-part"));
    let walls = [
        fixture::wall(&part_doc, body, 0),
        fixture::wall(&part_doc, body, 1),
    ];
    let corners = [0, 1, 2, 3].map(|v| fixture::vpiece(&part_doc, body, 0, v));
    let (doc_ref, body) = store.insert_part((part_doc, body), Tol::witness());
    let kit = Kit {
        store,
        body,
        walls,
        corners,
    };
    let o = kit.opts();
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = if i == 0 {
            fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &o)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    (doc, ids, kit)
}

/// **`placement`'s bundle replaced by one world pin at `frame`** — how
/// a copy is put at a world pose ([`fixture::pin`]).
fn repin(doc: ProfileDoc, placement: RecipeNodeId, frame: &Frame, o: &EvalOptions) -> ProfileDoc {
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
    fixture::pin(doc, placement, frame, o)
}

/// **`mate` added to `placement`'s bundle**, through the door.
fn add(doc: ProfileDoc, placement: RecipeNodeId, mate: Mate<Formula>) -> (ProfileDoc, MateRef) {
    fixture::constrain_at(doc, placement, mate)
}

/// **A value setting `coord` of `mate` to `by`**, through the door.
fn add_value(doc: ProfileDoc, mate: MateRef, coord: Coord, by: Formula) -> ProfileDoc {
    step(
        doc,
        DocEdit::AddValue {
            placement: mate.placement,
            value: Box::new(Value {
                mate: mate.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
    )
    .0
}

/// **`face`'s plane slid `by` along its own normal**: a standoff, which
/// places and declares no contact.
fn standoff(face: impl Into<Operand>, by: f64) -> Operand {
    let p = plane(face);
    Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: editor_core::pose::RoleRead::plain(p.clone()),
        along: editor_core::pose::RoleRead::through(p, editor_core::pose::PoseRole::Normal),
        by: len(by),
    }))
}

/// **`b`'s start cap resting on `a`'s end cap lifted `by`** along its
/// normal: a plane mate, `b` placed.
fn lifted_rest(kit: &Kit, a: RecipeNodeId, b: RecipeNodeId, by: f64) -> Mate<Formula> {
    fixture::mate(
        plane(head(kit.cap(b, CapEnd::Start))),
        flip(standoff(head(kit.cap(a, CapEnd::End)), by)),
        ContactClass::Rest,
    )
}

/// **`b`'s start cap resting on `a`'s end cap**, each read once.
fn plain_rest(kit: &Kit, a: RecipeNodeId, b: RecipeNodeId) -> Mate<Formula> {
    rest(
        head(kit.cap(b, CapEnd::Start)),
        head(kit.cap(a, CapEnd::End)),
        ContactClass::Rest,
    )
}

/// **A pin**: `b`'s lateral edge at corner `vb` coaxial with `a`'s at
/// corner `va` — an axis mate, `b` placed.
fn pin(kit: &Kit, a: RecipeNodeId, b: RecipeNodeId, va: usize, vb: usize) -> Mate<Formula> {
    fixture::mate(
        fixture::axis(read(kit.edge(b, vb))),
        fixture::axis(read(kit.edge(a, va))),
        ContactClass::Rest,
    )
}

/// **The frame through `instance`'s corner-0 edge and the corner 1 of
/// its cap `end`**: origin on the edge at that cap, `z` along the edge.
fn corner_frame(kit: &Kit, instance: RecipeNodeId, end: CapEnd) -> Operand {
    let r = |op: Operand| editor_core::pose::RoleRead::plain(op);
    Operand::Pose(Box::new(editor_core::pose::PoseDef::Through {
        axis: r(fixture::axis(read(kit.edge(instance, 0)))),
        point: r(point_of(read(kit.corner(instance, end, 1)))),
    }))
}

/// Whether two frames agree to `tol` in every stored coordinate — for
/// poses that come out of a fold, where the claim is the VALUE.
fn near(a: Frame, b: Frame, tol: f64) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(a.translation.iter())
        .zip(b.columns.iter().flatten().chain(b.translation.iter()))
        .all(|(x, y)| (x - y).abs() <= tol)
}

fn mate_fault(ev: &Evaluation<f64>, node: RecipeNodeId) -> MateFault {
    match ev.result(node) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => (**fault).clone(),
            other => panic!("expected a Mate refusal, got {other:?}"),
        },
        other => panic!("expected a failure, got {other:?}"),
    }
}

/// The band a copy's standing at the identity is decided at.
fn band() -> geom_core::predicate::Band {
    geom_core::predicate::Band::linear(Tol::witness()).expect("a band")
}

// ---- Row 1: a determined pair, evaluated, deterministic ----

/// The A11 rule-1 exemplar: **coaxial plus flush plus clocking** is
/// the DETERMINED case. Coaxial ∩ a perpendicular rest is a revolute
/// freedom about the shared line, and the clocking — a spin value of
/// zero on the coaxial mate — is what closes it. The rest's target is
/// `a`'s top cap lifted a unit along its normal (a standoff), so `b`
/// stands clear of `a`, one unit up.
fn determined_pair() -> (ProfileDoc, Vec<RecipeNodeId>, Kit, MateRef) {
    let (doc, ids, kit) = assembly("asm-r2a-row1", 2);
    let (doc, coaxial) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    let doc = add_value(doc, coaxial, Coord::Spin, ang(0.0));
    let (doc, rest) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 1.0));
    (doc, ids, kit, rest)
}

/// A pair determined by ONE mate: a frame coincidence of `b`'s bottom
/// corner frame with `a`'s top one, so `b` stands a unit up. The
/// maintenance rows want this shape because removing the single
/// constraint is the split, and the document it splits FROM is one
/// that solved.
fn stacked_pair(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>, MateRef, Kit) {
    let (doc, ids, kit) = assembly(label, 2);
    let (doc, joint) = add(
        doc,
        ids[1],
        fixture::mate(
            corner_frame(&kit, ids[1], CapEnd::Start),
            corner_frame(&kit, ids[0], CapEnd::End),
            ContactClass::Rest,
        ),
    );
    (doc, ids, joint, kit)
}

/// *Breaks if* the coaxial, the standoff or the spin value is lost
/// from the fold (the copy is loose or stands elsewhere).
#[test]
fn row1_a_coaxial_clocked_rest_pair_is_determined_and_evaluates() {
    let (doc, ids, kit, rest) = determined_pair();
    assert_eq!(rest.placement, ids[1], "the constraints place instance 1");
    let o = kit.opts();
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
        "the world-pinned instance's own pose is the bit-exact identity"
    );

    let ev = run(&doc, &o);
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
    let body = product(&doc, &ev, Tol::witness()).expect("the product gathers");
    let volume = topo::mass_properties(&body, Tol::witness())
        .expect("mass properties")
        .volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "two unit blocks, apart, gather whole: {volume}"
    );
}

/// D9 at this layer: two independent evaluations of the same document
/// agree BIT for bit, and two saves produce identical bytes. (The
/// two-fresh-PROCESS variant is the document layer's row, where ASM-2A
/// put it — a process hosts one ε, so the cross-process claim needs
/// the store.)
///
/// *Breaks if* an evaluation or a save is not a function of the
/// document.
#[test]
fn row1_evaluation_and_save_bytes_are_bit_identical_across_runs() {
    let (doc, _, kit, _) = determined_pair();
    let o = kit.opts();
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

/// **A V-block leaves a loose copy, not a refusal**: two plane mates
/// on perpendicular walls fold to a prismatic freedom, so the bundle
/// pins less than the copy needs. The copy evaluates in a space of its
/// own and has no world pose.
///
/// *Breaks if* an under-pinned bundle refuses, or is taken as pinned.
#[test]
fn row2_a_v_block_leaves_a_loose_copy() {
    let (doc, ids, kit) = assembly("asm-r2a-row2", 2);
    let doc = (0..2).fold(doc, |doc, k| {
        add(
            doc,
            ids[1],
            fixture::mate(
                plane(head(kit.wall(ids[1], k))),
                standoff(head(kit.wall(ids[0], k)), 0.0),
                ContactClass::Rest,
            ),
        )
        .0
    });
    let o = kit.opts();
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

/// Two rests of `b`'s start cap on `a`'s top cap, one lifted a unit:
/// parallel planes a unit apart, which no pose satisfies.
///
/// *Breaks if* the fold admits them, names the mates the other way
/// round, or measures any clash but the unit gap.
#[test]
fn row3_a_gap_mismatched_planar_pair_refuses_contradictory() {
    let (doc, ids, kit) = assembly("asm-r2a-row3", 2);
    let mut doc = doc;
    let mut mates = Vec::new();
    for height in [0.0, 1.0] {
        let (next, id) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], height));
        doc = next;
        mates.push(id);
    }
    let o = kit.opts();
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(ids[1]).expect("the bundle refuses").clone();
    let MateFault::Contradictory {
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
///
/// *Breaks if* the removal records a frame, the orphan keeps a pose,
/// or the surviving pin is rewritten.
#[test]
fn row4b_removing_the_placing_constraint_leaves_the_orphan_loose() {
    let (doc, ids, joint, kit) = stacked_pair("asm-r2a-row4b");
    let o = kit.opts();
    let doc = repin(doc, ids[0], &Frame::translation([0.0, 0.0, 4.0]), &o);
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
    let poses = solve(&applied.doc, &o, Tol::witness());
    assert_eq!(poses.space(ids[1]), Some(Space::Loose { root: ids[1] }));
    assert!(matches!(
        poses.placement(&applied.doc, ids[1]),
        Err(editor_core::PoseRefusal::Loose { placement, .. }) if placement == ids[1]
    ));
    assert!(
        fixture::same_offset(&doc, ids[0], &applied.doc, ids[0]),
        "the surviving copy keeps its world pin verbatim"
    );
    assert!(
        near(
            poses
                .placement(&applied.doc, ids[0])
                .expect("the world-pinned instance is placed"),
            Frame::translation([0.0, 0.0, 4.0]),
            1e-12
        ),
        "and stands where it is pinned"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

/// Row 4c — deleting the instance a constraint targets is not refused
/// either: DM7 reports the constraint's read it stranded, and the copy
/// that constraint placed has no pose.
///
/// *Breaks if* the delete takes the constraint with it, records a
/// frame, or leaves the placed copy a pose.
#[test]
fn row4c_deleting_the_target_strands_the_constraint_on_it() {
    let (doc, ids, joint, kit) = stacked_pair("asm-r2a-row4c");
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
    let read = Maintenance::StrandedRead {
        node: doc.spoken(ids[1]),
        slot: editor_core::OperandSlot::Side {
            constraint: joint.id,
            side: MateSide::To,
        },
        var: doc.spoken_var(doc.output(ids[0], 0).expect("the target's copies")),
    };
    assert_eq!(
        rows.iter().filter(|r| **r == read).count(),
        1,
        "the constraint's `to` read of the target strands: {rows:?}"
    );
    assert!(
        rows.iter()
            .all(|r| *r == read || matches!(r, Maintenance::StrandedSelection { .. })),
        "the strands are the whole report: no frame is recorded: {rows:?}"
    );
    let poses = solve(&applied.doc, &kit.opts(), Tol::witness());
    assert!(
        poses.placement(&applied.doc, ids[1]).is_err(),
        "a copy placed by a stranded constraint has no pose"
    );
    assert!(before.bit_eq(&doc), "undo is the prior value, held exactly");
}

/// Row 4d — a lone instance's world pose is where its world pin puts
/// it (A11 (5)), and a mate-less document round-trips identically.
///
/// *Breaks if* a lone pin is folded off its pose, or the round trip
/// moves a byte below the header.
#[test]
fn row4d_a_no_mates_document_round_trips_identically_below_the_header() {
    let (doc, ids, kit) = assembly("asm-r2a-row4d", 3);
    let o = kit.opts();
    let placed =
        Frame::rotate_then_translate([0.0, 0.0, 1.0], 0.25, [2.0, 3.0, 0.0], fixture::band())
            .expect("a literal axis has a definite direction");
    let doc = repin(doc, ids[2], &placed, &o);
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        near(
            poses.placement(&doc, ids[2]).expect("placed"),
            placed,
            1e-12
        ),
        "a lone instance stands where its world pin puts it"
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

/// Two pairs: `{i0,i1}` and `{i2,i3}`, each joined by one frame
/// coincidence in the second instance's bundle, each first instance
/// pinned to the world at a pose of its own.
fn two_pairs() -> (ProfileDoc, Vec<RecipeNodeId>, Kit) {
    let (doc, ids, kit) = assembly("asm-r2a-clusters", 4);
    let o = kit.opts();
    let mut doc = doc;
    for (a, b) in [(0, 1), (2, 3)] {
        doc = add(
            doc,
            ids[b],
            fixture::mate(
                corner_frame(&kit, ids[b], CapEnd::Start),
                corner_frame(&kit, ids[a], CapEnd::End),
                ContactClass::Rest,
            ),
        )
        .0;
    }
    let doc = repin(doc, ids[0], &Frame::translation([3.0, 0.0, 0.0]), &o);
    let doc = repin(doc, ids[2], &Frame::translation([7.0, 0.0, 0.0]), &o);
    (doc, ids, kit)
}

/// Row 4e — a cut of ONE WHOLE pinned pair moves as selected (A4): the
/// part's world-pinned instance keeps its pin, and the remainder
/// instance stands on the world at the identity.
///
/// *Breaks if* the cut rewrites a pin it carries or leaves, or the
/// remainder's instance stands anywhere but the identity.
#[test]
fn row4e_a_whole_pair_cut_moves_as_selected() {
    use std::collections::BTreeSet;
    let (doc, ids, kit) = two_pairs();
    let o = kit.opts();
    let cut = BTreeSet::from([ids[0], ids[1]]);
    let out = fixture::split_world(
        &doc,
        &cut,
        editor_core::DocumentId::derive("asm-r2a-4e"),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .expect("a whole-pair cut splits");
    assert!(
        fixture::same_offset(&out.part, out.node_map[&ids[0]], &doc, ids[0]),
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
    let mut store = kit.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    assert!(
        solve(&out.remainder, &with_resolver(store), Tol::witness()).at_identity(instance, band()),
        "the remainder instance stands on the world at the identity"
    );
    assert!(
        fixture::same_offset(&out.remainder, ids[2], &doc, ids[2]),
        "the untouched pair keeps its own pin"
    );
}

// ---- Row 5: the closure enumeration, executed ----

/// *Breaks if* an intersection of two closure members falls outside
/// the closure, or any table row answers another subgroup.
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
// verdicts. A pin is a coaxial of two lateral edges, so the pins'
// spacing is the block's own: adjacent corners a unit apart, opposite
// ones `√2`.

/// Two pins, inter-axis invariants MATCHING (both a unit apart) but
/// `b`'s second pin a quarter turn round from `a`'s: the table's entry
/// 3 parallel-distinct row, which requires **prismatic**. The
/// prismatic fold pins less than the copy needs, so the copy is loose;
/// the coset row below pins the witness itself.
///
/// *Breaks if* the fold pins the held representative's clocking
/// instead of solving the free angle (CONTRADICTORY).
#[test]
fn row5b_two_pins_clocked_apart_but_invariant_matched_fold_to_prismatic() {
    let (doc, ids, kit) = assembly("asm-r2a-row5b", 2);
    let (doc, _) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    // A's second pin at its corner 1; B's at its corner 3 — both
    // adjacent to corner 0, a quarter turn apart.
    let (doc, _) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 1, 3));
    let o = kit.opts();
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(
        poses.fault(ids[1]),
        None,
        "an assemblable two-pin pair is no contradiction"
    );
    assert_eq!(
        poses.space(ids[1]),
        Some(Space::Loose { root: ids[1] }),
        "the slide along the pins' shared direction leaves the copy loose"
    );
}

/// The same shape at the COSET door, so the witness itself is pinned:
/// the folded representative must be the quarter turn that carries B's
/// pattern onto A's, not the identity the held mate happened to state.
///
/// *Breaks if* the fold keeps the held representative's clocking.
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

/// Invariants that do NOT match: A's pins are `√2` apart (opposite
/// corners), B's a unit (adjacent). The table says empty, and the
/// refusal must quote the MEASURED clash — the spacing difference.
///
/// *Breaks if* the fold admits the pair, or measures another clash.
#[test]
fn row5b_mismatched_inter_axis_invariants_refuse_contradictory() {
    let (doc, ids, kit) = assembly("asm-r2a-row5b-mismatch", 2);
    let (doc, first) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    let (doc, second) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 2, 1));
    let o = kit.opts();
    let poses = solve(&doc, &o, Tol::witness());
    let fault = poses.fault(ids[1]).expect("the bundle refuses").clone();
    let MateFault::Contradictory {
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
        (metres.abs() - (2.0_f64.sqrt() - 1.0)).abs() < 1e-9,
        "the clash IS the inter-axis invariant difference (√2 − 1): {metres}"
    );
}

/// A rest plus two pins — A11 rule 1's own fully-determined territory,
/// and the shape MAJOR-1 falsely refused. The plate seats at the rest
/// (lifted a unit) and both pins land, so the fold is DETERMINED and
/// the pose is the seating height.
///
/// *Breaks if* the fold refuses an assemblable rest and two pins, or
/// turns the plate.
#[test]
fn row5b_a_rest_and_two_pins_determine_the_plate() {
    let (doc, ids, kit) = assembly("asm-r2a-row5b-plate", 2);
    let (doc, _) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 1.0));
    let (doc, _) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    let (doc, _) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 1, 1));
    let o = kit.opts();
    let poses = solve(&doc, &o, Tol::witness());
    assert_eq!(poses.fault(ids[1]), None, "rest + two pins determine");
    let relative = poses.relative(ids[1]).expect("a solved pose");
    assert!(
        near(relative, Frame::translation([0.0, 0.0, 2.0]), 1e-12),
        "the plate seats at the rest, unturned (both patterns agree): {relative:?}"
    );
    let ev = run(&doc, &o);
    assert!(matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))));
}

// ---- Row 6: the A12 partition consequences ----

/// *Breaks if* a mate does not couple the two instances it relates, or
/// either pose lands on another member.
#[test]
fn row6a_mated_instances_share_an_a9_component() {
    let (doc, ids, kit) = assembly("asm-r2a-row6a", 2);
    let apart = relative_freedom_components(&doc);
    assert!(
        !apart
            .iter()
            .any(|c| c.contains(&ids[0]) && c.contains(&ids[1])),
        "unconstrained instances are relatively free: {apart:?}"
    );
    let (doc, mate_ref) = add(doc, ids[1], plain_rest(&kit, ids[0], ids[1]));
    let together = relative_freedom_components(&doc);
    assert!(
        together
            .iter()
            .any(|c| c.contains(&ids[0]) && c.contains(&ids[1])),
        "the constraint COUPLES the components: {together:?}"
    );
    assert_eq!(
        fixture::mate_members(&doc),
        vec![(mate_ref, ids[1]), (mate_ref, ids[0])],
        "each pose lands on its instance"
    );
}

/// *Breaks if* a constraint makes or takes a placement, or its removal
/// records anything.
#[test]
fn row6b_a_constraint_add_and_remove_place_nothing() {
    let (doc, ids, kit) = assembly("asm-r2a-row6b", 2);
    let placed = doc.placements();
    assert_eq!(placed.len(), 2, "both instances are placements");
    let (doc, mate_ref) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 0.0));
    assert_eq!(
        doc.placements(),
        placed,
        "a constraint makes no placement (A10)"
    );
    // A lone rest leaves the copy loose; removing it takes nothing the
    // door has to record, so the door takes it and reports nothing.
    let applied = doc
        .apply(
            &DocEdit::RemoveConstraint {
                placement: mate_ref.placement,
                constraint: mate_ref.id,
            },
            Tol::witness(),
            &editor_core::RefusingReach,
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
///
/// *Breaks if* a constraint is a node, or the gather takes anything but
/// the two copies.
#[test]
fn row6c_the_gather_takes_the_placements_alone() {
    let (doc, ids, kit, _) = determined_pair();
    let ev = run(&doc, &kit.opts());
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

/// *Breaks if* a stranded face still lands on a member, or the
/// placement refuses with anything but the unresolved `to` read.
#[test]
fn row6d_a_dangling_head_contributes_no_edge_and_the_solve_refuses_typed() {
    let (doc, ids, kit) = assembly("asm-r2a-row6d", 2);
    let (doc, mate_ref) = add(doc, ids[1], plain_rest(&kit, ids[0], ids[1]));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: ids[0] });
    assert_eq!(
        fixture::mate_members(&doc),
        vec![(mate_ref, ids[1])],
        "the stranded head contributes NO edge (N5)"
    );
    let o = kit.opts();
    let ev = run(&doc, &o);
    assert!(
        matches!(
            ev.node_error(ids[1]).map(|e| &e.kind),
            Some(NodeErrorKind::UnresolvedRead {
                slot: editor_core::OperandSlot::Side {
                    constraint,
                    side: MateSide::To,
                },
                ..
            }) if *constraint == mate_ref.id
        ),
        "expected the stranded pose, got {:?}",
        ev.node_error(ids[1])
    );
    assert!(
        solve(&doc, &o, Tol::witness())
            .placement(&doc, ids[1])
            .is_err(),
        "a copy placed by a stranded constraint has no pose"
    );
}

// ---- Row 6f–6j: A12's repair path, and the three doors ----
// ---- that keep a head honest                              ----

/// A pose whose body is deleted is an unresolved read (D10), and its
/// repair is the pose's slot door: re-pointing the `to` pose at a face
/// of another instance resolves it again. No door re-points it for the
/// author.
///
/// *Breaks if* the slot door does not re-point the pose, or the
/// placement's key does not move with its read.
#[test]
fn row6f_the_slot_door_repairs_a_stranded_side() {
    let (doc, ids, kit) = assembly("asm-r2a-rebind-only-ref", 3);
    let (doc, mate_ref) = add(doc, ids[1], plain_rest(&kit, ids[0], ids[1]));
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
                    side: MateSide::To,
                }),
                value: editor_core::SlotValue::Read(flip(plane(head(
                    kit.cap(ids[2], CapEnd::End),
                )))),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a pose is re-pointed by its slot door");
    let mut members = fixture::mate_members(&applied.doc);
    members.sort();
    let mut expected = vec![(mate_ref, ids[1]), (mate_ref, ids[2])];
    expected.sort();
    assert_eq!(
        members, expected,
        "the repaired pose reads through the instance it now names"
    );
    assert!(
        applied.record.structural,
        "a constraint's read moved, so its placement's content key moved with it"
    );
    let ev = run(&applied.doc, &kit.opts());
    assert!(
        matches!(ev.result(ids[1]), Some(NodeResult::Ok(_))),
        "and the placement evaluates again: {:?}",
        ev.node_error(ids[1])
    );
}

/// A constraint's face read at a node that never existed is a typo,
/// refused at the constraint door as the unresolved read it is.
///
/// *Breaks if* a read at a node the document never held is admitted.
#[test]
fn row6h_the_constraint_door_refuses_a_head_naming_no_node() {
    let (doc, ids, kit) = assembly("asm-r2a-mate-insert-door", 2);
    let ghost = RecipeNodeId::new(0, 9_999);
    let err = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: ids[1],
                mate: Box::new(rest(
                    head(kit.cap(ids[1], CapEnd::Start)),
                    head(kit.cap(ghost, CapEnd::End)),
                    ContactClass::Rest,
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

/// A saved file is DATA: a constraint's face read naming an id past the
/// mint counter is as corrupt as a declared pair naming one, and worse
/// to let in — `Rebind`'s source door refuses a never-minted id, so the
/// document would load unrepairable.
///
/// *Breaks if* the load door admits a name headed at a node no mint
/// drew.
#[test]
fn row6i_the_load_check_refuses_a_mate_head_the_mint_never_minted() {
    let (doc, ids, kit) = assembly("asm-r2a-mate-wire-id", 3);
    let (doc, _) = add(doc, ids[1], plain_rest(&kit, ids[2], ids[1]));
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    // Doctored by what it is, not by position: the one selection whose
    // name is headed at `ids[2]` — the mate's `to` face — so a
    // fixture or field-order change breaks the probe instead of
    // silently moving it onto an unrelated id.
    let corrupt = doctored(&text, |wire| {
        let vars = wire["snapshot"]["vars"]
            .as_object_mut()
            .expect("the vars are a map on the wire");
        let mut aimed: Vec<&mut serde_json::Value> = vars
            .values_mut()
            .filter_map(|var| {
                let head = var.pointer_mut("/def/Select/names/0")?;
                (head["node"] == serde_json::json!(ids[2].0)).then_some(head)
            })
            .collect();
        let [head] = aimed.as_mut_slice() else {
            panic!("the probe is aimed at the one name headed at ids[2]");
        };
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
/// reads a constraint's selections under the rule it has always
/// applied to a declared pair: checkable exactly when the minting node
/// evaluated `Ok`, deferred otherwise. An instance-qualified head the
/// tables carry passes; a role the part's product does not have is
/// refused there rather than at the solve.
///
/// *Breaks if* the name door skips a constraint's selections, or
/// refuses one the tables carry.
#[test]
fn row6j_the_name_door_reads_a_mates_heads_like_any_payload_name() {
    let (doc, ids, kit) = assembly("asm-r2a-mate-name-door", 2);
    let ev = run(&doc, &kit.opts());
    let add = |mate: Mate<Formula>| DocEdit::AddConstraint {
        placement: ids[1],
        mate: Box::new(mate),
        fresh: Vec::new(),
    };
    let on = head(kit.cap(ids[1], CapEnd::Start));
    assert!(
        editor_core::apply_with_names(
            &doc,
            &add(rest(
                on.clone(),
                head(kit.cap(ids[0], CapEnd::End)),
                ContactClass::Rest
            )),
            &ev,
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .is_ok(),
        "the instance-qualified heads resolve in the instance's own table"
    );
    // A head wearing a role the part product has no entity for.
    let bogus = StableName {
        kind: EntityKind::Face,
        node: ids[0],
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: kit.body,
                path: vec![RoleSeg::Lateral(fixture::no_piece().into())],
            }
            .into(),
        }],
    };
    let err = editor_core::apply_with_names(
        &doc,
        &add(rest(on, head(bogus.clone()), ContactClass::Rest)),
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

/// **A value on a freedom its mate's kind fixes** — a spin of a frame
/// mate, a slide along a plane mate's normal — is a fact about the
/// value alone, so the value door refuses it where it is authored,
/// naming the mate's kind.
///
/// *Breaks if* the door admits a value on a fixed freedom, or names
/// another kind.
#[test]
fn row7a_a_value_on_a_freedom_its_mate_fixes_refuses_typed() {
    let (doc, ids, kit) = assembly("asm-r2a-row7a", 2);
    let (doc, frame) = add(
        doc,
        ids[1],
        fixture::mate(
            corner_frame(&kit, ids[1], CapEnd::Start),
            corner_frame(&kit, ids[0], CapEnd::End),
            ContactClass::Rest,
        ),
    );
    let (unframed, _) = step(
        doc.clone(),
        DocEdit::RemoveConstraint {
            placement: ids[1],
            constraint: frame.id,
        },
    );
    let (planed, rest) = add(unframed, ids[1], lifted_rest(&kit, ids[0], ids[1], 0.0));
    for (what, doc, mate, coord, by, kind) in [
        (
            "a frame's spin",
            &doc,
            frame,
            Coord::Spin,
            ang(0.5),
            editor_core::VarKind::Frame,
        ),
        (
            "a plane's normal slide",
            &planed,
            rest,
            Coord::Slide(SlideAxis::Z),
            len(0.5),
            editor_core::VarKind::Plane,
        ),
    ] {
        let err = apply(
            doc,
            &DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: mate.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("a value on a fixed freedom refuses");
        let EditError::MateRefused { fault, .. } = &err else {
            panic!("{what}: expected MateRefused, got {err:?}");
        };
        assert!(
            matches!(
                **fault,
                MateFault::NotAFreedom { value, coord: c, kind: Some(k) }
                    if value.placement == ids[1] && c == coord && k == kind
            ),
            "{what}: {fault:?}"
        );
        assert!(
            fault.to_string().contains("which the mate fixes"),
            "{fault}"
        );
    }
}

/// The `Fit` class is not representable in this kernel — the variant is
/// specified and not built — so the refusal is demonstrated at the WIRE
/// door, where an unknown spelling arrives. The sentence it says is
/// [`topo::FIT_DEFERRAL`], VERBATIM, and it is the same sentence the
/// solve door says when a future class reaches it.
///
/// *Breaks if* an unknown class spelling loads, or its refusal
/// paraphrases the kernel's sentence.
#[test]
fn row7c_an_unadmitted_class_refuses_naming_the_fit_deferral() {
    let (doc, ids, kit) = assembly("asm-r2a-row7c", 2);
    let (doc, _) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 0.0));
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
///
/// *Breaks if* the re-export chain carries another wording.
#[test]
fn the_fit_deferral_sentence_is_quoted_not_paraphrased() {
    assert_eq!(editor_core::FIT_DEFERRAL, topo::FIT_DEFERRAL);
    assert_eq!(editor_core::names::FIT_DEFERRAL, topo::FIT_DEFERRAL);
    assert!(editor_core::FIT_DEFERRAL.contains("Fit { gap }"));
    assert!(editor_core::FIT_DEFERRAL.contains("not yet built"));
}

/// Two rests of `b` on `a`'s top cap, the second's target turned about
/// a horizontal line of `a` by an angle whose induced displacement at
/// the mate's lever arm is 3ε — definitely inside (ε, Kε) for the
/// default K = 10 — in instance 1's bundle; the constraints, in bundle
/// order.
fn in_band_pair(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>, Kit, Vec<MateRef>) {
    let eps = geom_core::Tol::witness().get().eps;
    let tilt = 3.0 * eps;
    let (doc, ids, kit) = assembly(label, 2);
    let (doc, first) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 0.0));
    let top = plane(head(kit.cap(ids[0], CapEnd::End)));
    let r = |op: Operand| editor_core::pose::RoleRead::plain(op);
    // The line the top cap meets wall 0 in: horizontal, on `a`.
    let hinge = Operand::Pose(Box::new(editor_core::pose::PoseDef::Meet {
        a: r(top.clone()),
        b: r(plane(head(kit.wall(ids[0], 0)))),
    }));
    let tilted = Operand::Pose(Box::new(editor_core::pose::PoseDef::Spin {
        pose: r(top),
        about: r(hinge),
        by: ang(tilt),
    }));
    let (doc, second) = add(
        doc,
        ids[1],
        fixture::mate(
            plane(head(kit.cap(ids[1], CapEnd::Start))),
            flip(tilted),
            ContactClass::Rest,
        ),
    );
    (doc, ids, kit, vec![first, second])
}

/// The typed escalation: a case split constructed to land INSIDE the
/// ambiguity band at the run's own ε, so the row holds at every ε CI
/// sweeps.
///
/// *Breaks if* an in-band verdict is decided silently, or the
/// escalation names another predicate or constraint.
#[test]
fn row7d_an_in_band_case_split_escalates_typed() {
    let (doc, ids, kit, mates) = in_band_pair("asm-r2a-row7d");
    let o = kit.opts();
    let fault = solve(&doc, &o, Tol::witness())
        .fault(ids[1])
        .expect("the band escalates")
        .clone();
    let MateFault::Indeterminate { mate, diag } = &fault else {
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
/// log, and on no other.** Each bundle is folded where its placement
/// evaluates, so the decision is kept on that placement's frame. The
/// in-band case split here is decided while the fold adds `mates[1]`
/// to the bundle's intersection, so it is instance 1's: on that
/// placement's `NodeError`, on no other node's log, and not in a frame
/// the caller holds around the evaluation.
///
/// *Breaks if* the escalation is logged twice, on another node, or in
/// the caller's frame.
#[test]
fn row7e_a_mate_solve_escalation_is_on_the_refused_placements_own_log_and_not_in_an_outer_frame() {
    let (doc, ids, kit, mates) = in_band_pair("asm-r2a-row7e");
    let outer = geom_core::k_stats::Bracket::open();
    let ev = run(&doc, &kit.opts());
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
    let MateFault::Indeterminate { mate: refused, .. } = mate_fault(&ev, ids[1]) else {
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

/// A mate in one placement's bundle whose `on` reads another copy
/// pins nothing of the placement: a fact about the constraint alone,
/// which the constraint door refuses where it is authored.
///
/// *Breaks if* the door admits a mate that does not read the copy it
/// is added to.
#[test]
fn row7e_a_mate_not_on_its_copy_refuses_at_the_door() {
    let (doc, ids, kit) = assembly("asm-r2a-row7e-self", 2);
    let refused = fixture::at_the_door_of(
        &doc,
        &editor_core::RefusingReach,
        ids[1],
        rest(
            head(kit.cap(ids[0], CapEnd::Start)),
            head(kit.cap(ids[0], CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let Err((at, fault)) = refused else {
        panic!("the door admitted a mate not on its copy");
    };
    assert_eq!(at, ids[1], "the door names the placement");
    assert!(
        matches!(&fault, MateFault::NotOnTheCopy { mate } if mate.placement == ids[1]),
        "{fault:?}"
    );
    assert!(fault.to_string().contains("pins nothing"), "{fault}");
}
