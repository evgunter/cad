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
use editor_core::Formula;

use editor_core::CapEnd;
use editor_core::{
    ContactClass, Coord, DocEdit, DocumentId, EditError, EntityKind, EvalOptions, Evaluation,
    Frame, Maintenance, Mate, MateFault, MateRef, MateSide, NameRef, Node, NodeErrorKind,
    NodeResult, Operand, ProfileDoc, ProfileVertexRef, RecipeNodeId, RoleSeg, Space, StableName,
    TargetAxis, Value, apply, load, product, relative_freedom_components, save,
};
use fixture::resolver::{PartStore, in_part, placement_of, with_resolver};
use fixture::{
    ang, flip, head, insert, len, on_frame, plane, point_of, rest, run, solve, square, step,
};
use geom_core::Tol;

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
    let doc = fixture::emptied(doc, placement);
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
                mate: Some(mate.id),
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

// ---- Row 1: a determined pair, evaluated, deterministic ----

/// The A11 rule-1 exemplar, **a coaxial mate and its slide and spin
/// values**, is the DETERMINED case: the coaxial between the two
/// corner-0 lateral edges leaves a cylindrical freedom along and about
/// the shared line, and its two values close it — a spin of zero clocks
/// `b` square to `a`, and a slide of two along the line stands `b` a
/// unit clear above `a`'s top cap.
fn determined_pair() -> (ProfileDoc, Vec<RecipeNodeId>, Kit, MateRef) {
    let (doc, ids, kit) = assembly("asm-r2a-row1", 2);
    let (doc, coaxial) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    let doc = add_value(doc, coaxial, Coord::Spin, ang(0.0));
    let doc = add_value(doc, coaxial, Coord::Slide(TargetAxis::Z), len(2.0));
    (doc, ids, kit, coaxial)
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

/// *Breaks if* the coaxial or either value is lost from the solve (the
/// copy is loose or stands elsewhere).
#[test]
fn row1_a_coaxial_slid_and_clocked_pair_is_determined_and_evaluates() {
    let (doc, ids, kit, coaxial) = determined_pair();
    assert_eq!(
        coaxial.placement, ids[1],
        "the constraint places instance 1"
    );
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
        "the child stands its height plus a unit clear above the parent: {relative:?}"
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

// ---- Row 2: a V-block refuses, and a lone plane mate leaves a loose copy ----

/// **A V-block refuses at the door; a lone plane mate leaves a loose
/// copy** (FORK-S3O): two plane mates on perpendicular walls fix a
/// normal twice, so the door refuses the second, naming both, and the
/// families' meeting leaves a slide. The first alone pins less than the
/// copy needs: the copy evaluates in a space of its own and has no world
/// pose.
///
/// *Breaks if* the door admits the second wall, or an under-pinned
/// bundle refuses or is taken as pinned.
#[test]
fn row2_a_v_block_refuses_and_one_wall_leaves_a_loose_copy() {
    let (doc, ids, kit) = assembly("asm-r2a-row2", 2);
    let wall = |k| {
        fixture::mate(
            plane(head(kit.wall(ids[1], k))),
            standoff(head(kit.wall(ids[0], k)), 0.0),
            ContactClass::Rest,
        )
    };
    let (doc, first) = add(doc, ids[1], wall(0));
    let refused = fixture::door_refusal(&doc, wall(1));
    assert!(
        matches!(
            &refused,
            MateFault::Overconstrained { placement, constraints, free: 1 }
                if *placement == ids[1] && constraints.len() == 2 && constraints.contains(&first.id)
        ),
        "the second wall fixes the normal the first fixes: {refused:?}"
    );
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

// ---- Row 3: a second rest refuses whatever its height ----

/// Two rests of `b`'s start cap on `a`'s top cap, the second lifted by
/// `h`: whether it agrees with the first (`h = 0`) or not, it fixes the
/// normal and the standoff the first fixes, so the door refuses it,
/// measuring nothing, in the same words at every height.
///
/// *Breaks if* the door admits the agreeing rest (the fallback to an
/// assertion Ev rejected), or tells the two heights apart.
#[test]
fn row3_a_second_rest_refuses_at_the_door_whatever_its_height() {
    let (doc, ids, kit) = assembly("asm-r2a-row3", 2);
    let (doc, held) = add(doc, ids[1], lifted_rest(&kit, ids[0], ids[1], 0.0));
    let said: Vec<String> = [0.0, 1.0]
        .into_iter()
        .map(|h| {
            let fault = fixture::door_refusal(&doc, lifted_rest(&kit, ids[0], ids[1], h));
            assert!(
                matches!(
                    &fault,
                    MateFault::Overconstrained { placement, constraints, .. }
                        if *placement == ids[1] && constraints.len() == 2 && constraints.contains(&held.id)
                ),
                "h = {h}: {fault:?}"
            );
            fault.to_string()
        })
        .collect();
    assert_eq!(said[0], said[1], "the refusal measures nothing");
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
    let applied = apply(&doc, &DocEdit::DeleteNode { id: ids[0] }, Tol::witness())
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
        solve(&out.remainder, &with_resolver(store), Tol::witness()).at_identity(instance),
        "the remainder instance stands on the world at the identity"
    );
    assert!(
        fixture::same_offset(&out.remainder, ids[2], &doc, ids[2]),
        "the untouched pair keeps its own pin"
    );
}

// ---- Row 5: the family meet, enumerated ----

/// **The families meet within the closure, unordered, and transversely
/// only where a mate may join another** (FORK-S3O): every pair of the
/// ten families meets in one of them, the same in either order; SE(3)
/// meets anything transversely, and of two mates' families only a
/// ball's and a direction's meet transversely (in the revolute about
/// the line through the point along the direction).
///
/// *Breaks if* a meeting falls outside the closure or depends on order,
/// or a pair of mate kinds that fixes an equation twice is admitted.
#[test]
fn row5_the_family_meet_is_closed_unordered_and_transverse_only_where_mates_join() {
    use editor_core::SubgroupFamily as F;
    let all = [
        F::Se3,
        F::Planar,
        F::Cylindrical,
        F::Prismatic,
        F::Revolute,
        F::Spherical,
        F::Parallel,
        F::Translation,
        F::PlaneTranslation,
        F::Trivial,
    ];
    for a in all {
        for b in all {
            assert_eq!(a.meet(b), b.meet(a), "{a:?} ∩ {b:?} is unordered");
            assert!(
                a.meet(b).dimension() <= a.dimension().min(b.dimension()),
                "{a:?} ∩ {b:?} = {:?} lies in both",
                a.meet(b)
            );
        }
        assert_eq!(F::Se3.transverse(a), Some(a), "SE(3) meets {a:?} as {a:?}");
    }
    let mates = [
        F::Trivial,
        F::Planar,
        F::Cylindrical,
        F::Parallel,
        F::Spherical,
    ];
    for a in mates {
        for b in mates {
            let want = matches!(
                (a, b),
                (F::Spherical, F::Parallel) | (F::Parallel, F::Spherical)
            );
            assert_eq!(
                a.transverse(b).is_some(),
                want,
                "{a:?} beside {b:?}: {:?}",
                a.transverse(b)
            );
        }
    }
    assert_eq!(F::Spherical.transverse(F::Parallel), Some(F::Revolute));
}

// ---- Row 5b: pins beyond the first ----

/// Two pins, their spacings matched (adjacent corners a unit apart) or
/// not (A's opposite corners, `√2`, against B's adjacent): either way
/// the second fixes the direction and the line the first fixes, so the
/// door refuses it, measuring nothing. Where two copies meet beyond
/// what their mates fix is a contact, and its recourse an assertion.
///
/// *Breaks if* the door measures the spacing (admitting the matched
/// pair or quoting the mismatch).
#[test]
fn row5b_a_second_pin_refuses_whatever_its_spacing() {
    let (doc, ids, kit) = assembly("asm-r2a-row5b", 2);
    let (doc, first) = add(doc, ids[1], pin(&kit, ids[0], ids[1], 0, 0));
    for (what, a, b) in [("matched", 1, 3), ("mismatched", 2, 1)] {
        let fault = fixture::door_refusal(&doc, pin(&kit, ids[0], ids[1], a, b));
        assert!(
            matches!(
                &fault,
                MateFault::Overconstrained { placement, constraints, free: 0 }
                    if *placement == ids[1] && constraints.len() == 2 && constraints.contains(&first.id)
            ),
            "{what}: {fault:?}"
        );
    }
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
    // The first instance stands by the identity bundle, which reads no
    // pose, so the one mate's two poses land on the two instances.
    assert!(
        fixture::bundle(&doc, ids[0])
            .iter()
            .all(|c| c.mate().is_none()),
        "the first instance stands with no mate"
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
            Tol::witness()
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
            Coord::Slide(TargetAxis::Z),
            len(0.5),
            editor_core::VarKind::Plane,
        ),
    ] {
        let err = apply(
            doc,
            &DocEdit::AddValue {
                placement: mate.placement,
                value: Box::new(Value {
                    mate: Some(mate.id),
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a value on a fixed freedom refuses");
        let EditError::MateRefused { fault, .. } = &err else {
            panic!("{what}: expected MateRefused, got {err:?}");
        };
        assert!(
            matches!(
                **fault,
                MateFault::NotAFreedom { value, coord: c, kind: k }
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
