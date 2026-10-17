//! **A mate's face read across the seam, and under `Rebind`**
//! (ASSEMBLY.md A3, A4, A11 (5); the spec's `## P2-split` face rows).
//!
//! A mate's target is a pose read off a face: the plane of the face its
//! head names, the head with the member walk's qualifiers stripped
//! (`head_face`), read in the member's part. So whatever carries the
//! head carries the pose. Split and inline re-anchor the head, which
//! crosses where its member's copy stands in the part's world; `Rebind`
//! rewrites the head, and the pose follows it onto a renamed face or
//! another part's instance; a pattern copy's head reads its master's
//! face.
//!
//! The scenes are `p2_gauges`'s two literal blocks, pinned over their
//! own geometry at dyadic translations.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{Parts, block, cut, parts};
use crate::p2_split::{pinned_at, seat_at, seated};

use editor_core::{
    CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions, FaceName, MateFault, MateRef, MateSide,
    Node, PartResolver, PatternKind, ProfileDoc, RecipeNodeId, SitedFace, SplitError, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{head, in_copy, insert, len, run, scl, solve, step, step_with};
use geom_core::Tol;

// ---- substrate ----

/// **`mover`'s face seated on `target`** by `placement`'s mate, the
/// mover's origin over the target body's own ([`seat_at`]).
fn face_mate(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mover: SitedFace,
    target: SitedFace,
) -> (ProfileDoc, MateRef) {
    seat_at(doc, placement, mover, target, [0.0, 0.0])
}

/// `mate` and the values on it taken out of its placement's bundle.
fn remove(doc: ProfileDoc, mate: MateRef) -> ProfileDoc {
    fixture::bundle(&doc, mate.placement)
        .into_iter()
        .rev()
        .filter(|c| c.id == mate.id || c.value().is_some_and(|v| v.mate == mate.id))
        .fold(doc, |doc, c| {
            step(
                doc,
                DocEdit::RemoveConstraint {
                    placement: mate.placement,
                    constraint: c.id,
                },
            )
            .0
        })
}

/// The selection a stored mate's target reads.
fn target_var(doc: &ProfileDoc, mate: MateRef) -> editor_core::VarId {
    let stored = fixture::constraint(doc, mate);
    let to = *stored.to.pose().expect("a mate to a pose");
    let [select] = fixture::face_reads(doc, to)[..] else {
        panic!("the target reads one face")
    };
    select
}

/// **Where a mate's target face is in the world**: the face its head
/// names in the member's part (`head_face`), read through the part's
/// own evaluation (`MateReach::face_pose`), carried by the member's
/// solved world pose and then by `copy_shift`, the translation the
/// scene's pattern puts between a copy and its master — origin, normal
/// and reference direction. A plain read (its own instance, no copy, no
/// placer) takes no shift; a copy read takes its copy's.
fn target_world(
    doc: &ProfileDoc,
    o: &EvalOptions,
    mate: MateRef,
    copy_shift: [f64; 3],
) -> [f64; 9] {
    let head = &fixture::side_head(doc, target_var(doc, mate));
    let member = editor_core::member_of(doc, head).expect("the head reads a member");
    if copy_shift == [0.0; 3] {
        assert!(member.chain.is_empty(), "a plain read: {member:?}");
    } else {
        assert_eq!(member.copy().len(), 1, "a copy read: {member:?}");
    }
    let face = editor_core::head_face(doc, head).expect("the head names a part face");
    let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(member.instance) else {
        panic!("the member stands on an instance");
    };
    let reach = editor_core::PartReach::<f64>::with_resolver(o.resolver.as_ref(), Tol::witness());
    let pose = editor_core::MateReach::face_pose(&reach, doc_ref, &face)
        .unwrap_or_else(|refusal| panic!("the face has a pose: {refusal:?}"));
    let placed = solve(doc, o, Tol::witness())
        .placement(doc, member.instance)
        .expect("the member is placed")
        .affine::<f64>();
    let origin = placed.transform_point(pose.origin);
    let axis = placed.transform_vec(pose.axis);
    let u = placed.transform_vec(pose.u_ref.expect("a cap fixes its reference"));
    [
        origin.x + copy_shift[0],
        origin.y + copy_shift[1],
        origin.z + copy_shift[2],
        axis.x,
        axis.y,
        axis.z,
        u.x,
        u.y,
        u.z,
    ]
}

/// Two [`target_world`]s agree bit for bit: an instance a split leaves
/// pinned at the identity moves nothing.
fn same_world(a: [f64; 9], b: [f64; 9], what: &str) {
    assert_eq!(
        a.map(f64::to_bits),
        b.map(f64::to_bits),
        "{what}: {a:?} vs {b:?}"
    );
}

fn split(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    label: &str,
    o: &EvalOptions,
) -> editor_core::SplitOutcome {
    editor_core::split(
        doc,
        &cut(doc, ids),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("{label}: split refused: {e}"))
}

fn split_err(doc: &ProfileDoc, ids: &[RecipeNodeId], label: &str, o: &EvalOptions) -> SplitError {
    match editor_core::split(
        doc,
        &cut(doc, ids),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    ) {
        Ok(_) => panic!("{label}: split admitted what it should refuse"),
        Err(e) => e,
    }
}

fn resolver(store: &PartStore) -> Arc<dyn PartResolver> {
    Arc::new(store.clone())
}

/// **The face-crossing scene**: a base pinned to the world at
/// `[4, 0, 0]` and a top mated onto it, so the top is placed by its
/// mate rather than pinned; and a third block `k` whose one mate `m`
/// rests `k`'s bottom cap on the top's upper cap, its in-plane freedoms
/// left free — a value would be charted on the top's frame, which a
/// split cannot carry off the identity. Returns the store's parts, the
/// document, `[base, top, k]` and `m`.
fn face_across(label: &str) -> (Parts, ProfileDoc, [RecipeNodeId; 3], MateRef) {
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    // `k` is listed first, so a cut of the base and the top holds the
    // last placements and a round trip keeps their order.
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, base) = pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _seat) = seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let (doc, m) = fixture::constrain_at(
        doc,
        k,
        fixture::rest(
            head(p.top_cap(k)),
            head(p.top_upper_cap(top)),
            ContactClass::Rest,
        ),
    );
    (p, doc, [base, top, k], m)
}

// ---- the seam ----

/// **A mate whose target reads a mate-placed copy crosses split and
/// inline, and the target's world pose does not move.** The kept
/// constraint `m` reads the top — placed by its mate, not pinned —
/// through its head face. Split re-anchors the head through the
/// instance left behind, and the face it names is then a row of the new
/// part; inline unwraps it back onto the inner top. At each of the
/// three documents the target's world pose is the same, and
/// `inline(split(d))` is `d` up to node ids under the R1 comparator —
/// the constraint's poses included.
///
/// *Breaks if* the head stays on the cut node, the face reads another
/// place after either move, or the round trip loses the mate's read.
#[test]
fn a_mate_reading_a_mate_placed_copy_crosses_split_and_inline_unmoved() {
    let label = "p2-face-nonroot";
    let (p, doc, [base, top, _k], m) = face_across(label);
    let o = p.opts();
    assert!(
        fixture::offset_of(&doc, top).is_none(),
        "the top is placed by its mate, not pinned"
    );
    let before = target_world(&doc, &o, m, [0.0; 3]);

    let out = split(&doc, &[base, top], label, &o);
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let with_part = with_resolver(store.clone());
    assert_eq!(
        fixture::side_head(&out.remainder, target_var(&out.remainder, m))
            .name
            .node,
        out.instance,
        "the head re-anchors through the instance"
    );
    same_world(
        target_world(&out.remainder, &with_part, m, [0.0; 3]),
        before,
        "split: the target's world pose",
    );

    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(&store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    same_world(
        target_world(&back.doc, &o, m, [0.0; 3]),
        before,
        "inline: the target's world pose",
    );
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));
}

/// **A mate whose target reads a pattern copy crosses split and inline,
/// and the target's world pose does not move.** The head names the
/// copy's face and crosses as it is, the copy placed at the identity. A
/// leg patterned three times two apart along x, and a kept constraint
/// placing `k` whose target reads copy 2's upper cap. Split cuts the
/// leg and its pattern, and the target then reads the instance plainly,
/// its face the copy's cap as the new part names it; inline unwraps it
/// back onto copy 2 of the inner pattern. At each of the three
/// documents the target's world pose is the same, and
/// `inline(split(d))` is `d` up to node ids under the R1 comparator.
///
/// *Breaks if* the copy's face crosses as its master's, or moves.
#[test]
fn a_mate_reading_a_pattern_copy_crosses_split_and_inline_unmoved() {
    let label = "p2-face-copy-seam";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    // The kept copy first: split appends its instance and inline what it
    // splices, so the round trip keeps the placements' order (A4).
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, leg) = fixture::insert_pinned(doc, Node::instantiate_part(p.top), &o);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: editor_core::Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let doc = crate::fix_pattern_mate_crossing::place_copies(doc, pattern, 3, &o);
    // One copy of the pattern is read through its `Part`, as a reader of
    // one copy reads it, so the round trip below reads it there again.
    let part_two = doc
        .ids()
        .into_iter()
        .filter(|&id| matches!(doc.node(id), Some(Node::Part { .. })))
        .nth(2)
        .expect("copy 2's Part, the third `place_copies` inserts");
    let copy_cap = fixture::head_at(part_two, in_copy(pattern, 2, p.top_upper_cap(leg)));
    let (doc, m) = face_mate(doc, k, head(p.top_cap(k)), copy_cap);
    let copy_two = [4.0, 0.0, 0.0];
    let before = target_world(&doc, &o, m, copy_two);

    let out = split(&doc, &[leg, pattern], label, &o);
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let with_part = with_resolver(store.clone());
    same_world(
        target_world(&out.remainder, &with_part, m, [0.0; 3]),
        before,
        "split: the target's world pose",
    );

    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(&store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    same_world(
        target_world(&back.doc, &o, m, copy_two),
        before,
        "inline: the target's world pose",
    );
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));
}

/// **A mate whose target reads a loose copy refuses at split**: a copy
/// in its own space has no place in the part's world for its face to
/// keep, so the face would mean another place. The cut holds a loose
/// top beside a pinned base, and `k` is placed against the loose top:
/// the kept mate's read of the cut is severed.
///
/// Inline's mirror — a host mate reading a loose copy of the part — is
/// not a document inline is asked about: the part's product table
/// carries no row for a body in a loose space, so the door admits the
/// mate as written and its target does not evaluate, which is `k`'s own
/// refusal. The row pins that, so the day the product carries such rows
/// the inline half is owed.
///
/// *Breaks if* a face of a loose copy crosses the seam, or a host mate
/// reading one evaluates.
#[test]
fn a_mate_reading_a_loose_copy_refuses_at_split() {
    let label = "p2-face-unplaced";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(p.base), &o);
    let (doc, loose) = insert(doc, fixture::mated_instance(p.top));
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = face_mate(doc, k, head(p.top_cap(k)), head(p.top_upper_cap(loose)));
    assert_eq!(
        solve(&doc, &o, Tol::witness()).space(loose),
        Some(editor_core::Space::Loose { root: loose }),
        "the loose top lives in its own space"
    );
    let err = split_err(&doc, &[base, loose], label, &o);
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, input, .. }
                if consumer.id() == k && input.id() == loose
        ),
        "{err:?}"
    );

    // The same two as a part, and a host mate reading the loose top.
    let part_doc = ProfileDoc::empty(DocumentId::derive(&format!("{label}-sub")), Tol::witness());
    let (part_doc, _sub_base) =
        fixture::insert_pinned(part_doc, Node::instantiate_part(p.base), &o);
    let (part_doc, sub_loose) = insert(part_doc, fixture::mated_instance(p.top));
    let mut store = p.store.clone();
    let sub = store.insert(part_doc, Tol::witness());
    let with_sub = with_resolver(store);
    let host = ProfileDoc::empty(DocumentId::derive(&format!("{label}-host")), Tol::witness());
    let (host, i) = fixture::insert_pinned(host, Node::instantiate_part(sub), &with_sub);
    let (host, k) = insert(host, fixture::mated_instance(p.top));
    let through = FaceName::new(p.top_upper_cap(sub_loose))
        .expect("a face")
        .in_part(i)
        .into_name();
    let (host, _) = face_mate(host, k, head(p.top_cap(k)), head(through));
    let poses = solve(&host, &with_sub, Tol::witness());
    assert!(
        matches!(
            poses.fault(k),
            Some(MateFault::SideUnevaluated {
                side: MateSide::To,
                ..
            })
        ),
        "{:?}",
        poses.fault(k)
    );
}

// ---- the strip at a pattern copy ----

/// **A mate's target on a pattern copy reads its master's face, carried
/// by the copy map.** A leg patterned three times two apart along x,
/// and a base seated on copy 2's upper cap: the head is `Instance(2)`
/// round the leg instance's name, the strip takes off that one
/// qualifier and then the instance's `InPart`, and the base's bottom
/// lands on the master's cap plane. A value on a target read off a
/// pattern's copy is charted on the space's frame
/// (`work/intent/a-value-on-a-placed-copy-target-is-charted-on-the-space.md`),
/// so the seat's slide of 4 along x stands the base over copy 2.
///
/// *Breaks if* the strip reads another face, or the copy map is not
/// applied.
#[test]
fn a_mate_on_a_pattern_copy_reads_the_masters_face_at_the_copy() {
    let label = "p2-face-copy";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = fixture::insert_pinned(doc, Node::instantiate_part(p.top), &o);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg.into(),
            count: editor_core::Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, mover) = insert(doc, fixture::mated_instance(p.base));
    let doc = crate::fix_pattern_mate_crossing::place_copies(doc, pattern, 3, &o);
    let copy_cap = in_copy(pattern, 2, p.top_upper_cap(leg));
    let (doc, m) = seat_at(
        doc,
        mover,
        head(in_part(mover, p.base_body, CapEnd::Start)),
        head(copy_cap),
        [4.0, 0.0],
    );
    let ev = run(&doc, &o);
    assert!(
        ev.node_error(m.placement).is_none(),
        "{:?}",
        ev.node_error(m.placement)
    );
    let b = &fixture::side_head(&doc, target_var(&doc, m));
    let member = editor_core::member_of(&doc, b).expect("a copy is a member");
    assert_eq!(member.copy(), vec![(pattern, 2)]);
    let master = editor_core::head_face(&doc, b).expect("the strip");
    assert_eq!(
        master.clone().into_name(),
        fixture::resolver::in_world(p.top_body, CapEnd::End),
        "the master's own row, as the part's world names it"
    );
    let reach = editor_core::PartReach::<f64>::with_resolver(o.resolver.as_ref(), Tol::witness());
    let pose = editor_core::MateReach::face_pose(&reach, &p.top, &master).expect("a pose");
    let placed = solve(&doc, &o, Tol::witness())
        .placement(&doc, mover)
        .expect("the mover is placed");
    let want = [4.0, 0.0, pose.origin.z];
    assert!(
        placed
            .translation
            .iter()
            .zip(want)
            .all(|(x, y)| (x - y).abs() <= 1e-12),
        "the master's cap, moved to copy 2: {:?} vs {want:?}",
        placed.translation
    );
}

/// **A head that names no face of its part does not evaluate**: a face
/// name headed at the instance but not wrapped in the instance's
/// `InPart` walks to a member, and there is no part row under it to
/// read a plane off. The door is structural and admits the mate; its
/// target is `k`'s own refusal where it evaluates, with its recourse.
/// An instance mints no such name, so only a head written by hand meets
/// this.
///
/// *Breaks if* a bare name reads some face of the part, or the mate
/// folds without its target.
#[test]
fn a_head_naming_no_part_face_does_not_evaluate() {
    let label = "p2-face-no-part-face";
    let (p, doc, [base, _top, k], m) = face_across(label);
    let o = p.opts();
    let doc = remove(doc, m);
    let bare = StableName {
        kind: editor_core::EntityKind::Face,
        node: base,
        path: Vec::new(),
    };
    assert!(
        editor_core::member_of(&doc, &head(bare.clone())).is_some(),
        "the bare name walks to the base's member"
    );
    let (doc, _) = face_mate(doc, k, head(p.top_cap(k)), head(bare));
    let poses = solve(&doc, &o, Tol::witness());
    let Some(fault) = poses.fault(k) else {
        panic!("the target does not evaluate");
    };
    assert!(
        matches!(
            fault,
            MateFault::SideUnevaluated {
                side: MateSide::To,
                ..
            }
        ),
        "{fault:?}"
    );
    assert!(
        fault
            .to_string()
            .contains("Recourse: repair the pose's definition"),
        "the refusal states its recourse: {fault}"
    );
}

// ---- Rebind ----

/// The base part rebuilt with a NEW extrude of `height` over the same
/// profile — every face of it renamed, since a face name derives from
/// the node that mints it — placed in the world in the old body's
/// place, and the new body. The new placement is recorded as
/// [`PartStore::insert_part`] records one, so [`in_part`] spells the
/// new body's caps.
fn renamed(base: &ProfileDoc, body: RecipeNodeId, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let Some(Node::Extrude { profile, side, .. }) = base.node(body).cloned() else {
        panic!("the base's body is an extrude");
    };
    let old_copies: Vec<RecipeNodeId> = base
        .placements()
        .into_iter()
        .filter(|&at| {
            matches!(base.node(at), Some(Node::Place { shapes, .. })
                if shapes.iter().any(|s| base.operation_of(s.read) == Some(body)))
        })
        .collect();
    let base = old_copies
        .into_iter()
        .chain([body])
        .fold(base.clone(), |doc, id| {
            step(doc, DocEdit::DeleteNode { id }).0
        });
    let (base, new_body) = insert(
        base,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
            side,
        },
    );
    let mut scratch = PartStore::new();
    let (placed, _) = scratch.insert_part((base, new_body), Tol::witness());
    (scratch.doc(placed.id), new_body)
}

/// The top's world origin.
fn origin_of(doc: &ProfileDoc, o: &EvalOptions, id: RecipeNodeId) -> [f64; 3] {
    solve(doc, o, Tol::witness())
        .placement(doc, id)
        .expect("placed")
        .translation
}

/// Whether two points agree to well inside the witness band.
fn at(a: [f64; 3], b: [f64; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-12)
}

/// **A face side follows its head through a part-side rename and its
/// repair.** The base part is rebuilt so its upper cap carries a new
/// name, and the assembly's reference moves to it: the head now names
/// a row the part does not have, and the target does not evaluate.
/// `Rebind` of the head onto the new row repairs the mate whole — the
/// pose IS the head's face's plane, so there is nothing else to
/// rewrite — and the top lands on the rebuilt cap, at its new height.
///
/// *Breaks if* the stranded target evaluates, or the rebound mate keeps
/// the old cap's height.
#[test]
fn a_rename_update_and_rebind_carry_the_face_side_with_the_head() {
    let label = "p2-face-rebind";
    let mut store = PartStore::new();
    let (base_ref, base_body) =
        store.insert_part(block(&format!("{label}-base"), 3.0, 1.0), Tol::witness());
    let base_doc = store.doc(base_ref.id);
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), 1.0, 3.0), Tol::witness());
    let o = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, Node::instantiate_part(base_ref), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let old_head = in_part(base, base_body, CapEnd::End);
    let (doc, m) = face_mate(
        doc,
        top,
        head(in_part(top, top_body, CapEnd::Start)),
        head(old_head.clone()),
    );
    assert!(
        (origin_of(&doc, &o, top)[2] - 1.0).abs() <= 1e-12,
        "on the cap"
    );

    let (rebuilt, new_body) = renamed(&base_doc, base_body, 1.5);
    assert_ne!(new_body, base_body, "the cap is renamed");
    let new_ref = store.insert(rebuilt, Tol::witness());
    let o = with_resolver(store.clone());
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::UpdateReference {
            node: base,
            new_pin: new_ref.pin,
        },
        &reach,
    );
    // The base's own pin read the old body's edges, which the rebuild
    // renamed too: pinned afresh, so only the mate's head is stranded.
    let doc = crate::p2_split::repin(doc, base, [0.0; 3], &o);
    let poses = solve(&doc, &o, Tol::witness());
    assert!(
        matches!(
            poses.fault(m.placement),
            Some(MateFault::SideUnevaluated {
                side: MateSide::To,
                ..
            })
        ),
        "the stranded head faults: {:?}",
        poses.fault(m.placement)
    );

    let base_out = doc.output(base, 0).expect("the base's body");
    let (doc, _) = step_with(
        doc,
        DocEdit::Rebind {
            body: Some(base_out),
            from: old_head,
            to: in_part(base, new_body, CapEnd::End),
        },
        &reach,
    );
    let ev = run(&doc, &o);
    assert!(
        ev.node_error(m.placement).is_none(),
        "repaired whole: {:?}",
        ev.node_error(m.placement)
    );
    assert!(
        (origin_of(&doc, &o, top)[2] - 1.5).abs() <= 1e-12,
        "the top follows the rebuilt cap"
    );
}

/// **A target re-pointed onto another part's instance reads that
/// part's face.** Two bases built alike — the same node ids, one cap
/// name row for row — at different heights and places; the top seated
/// on the first's cap, then its target re-pointed by its slot door onto
/// the second's. The pose reads the second base's cap, in the second
/// part: the top lands there, at the second's height and place, and
/// nowhere a name read in the wrong part could put it.
///
/// *Breaks if* the re-pointed head is read in the first part.
#[test]
fn a_head_rebound_onto_another_parts_instance_reads_the_new_heads_face() {
    let label = "p2-face-reparent";
    let mut store = PartStore::new();
    let (low_ref, low_body) =
        store.insert_part(block(&format!("{label}-low"), 3.0, 1.0), Tol::witness());
    let (high_ref, high_body) =
        store.insert_part(block(&format!("{label}-high"), 3.0, 2.0), Tol::witness());
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), 1.0, 3.0), Tol::witness());
    let o = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, low) = fixture::insert_pinned(doc, Node::instantiate_part(low_ref), &o);
    let (doc, high) = pinned_at(doc, high_ref, [10.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, m) = face_mate(
        doc,
        top,
        head(in_part(top, top_body, CapEnd::Start)),
        head(in_part(low, low_body, CapEnd::End)),
    );
    let on_low = origin_of(&doc, &o, top);
    assert!(
        (on_low[2] - 1.0).abs() <= 1e-12,
        "on the low cap: {on_low:?}"
    );

    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::SetParam {
            node: m.placement,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Side {
                constraint: m.id,
                side: MateSide::To,
            }),
            value: editor_core::SlotValue::Read(fixture::flip(fixture::plane(head(in_part(
                high,
                high_body,
                CapEnd::End,
            ))))),
            fresh: Vec::new(),
        },
        &reach,
    );
    let ev = run(&doc, &o);
    assert!(
        ev.node_error(m.placement).is_none(),
        "{:?}",
        ev.node_error(m.placement)
    );
    let on_high = origin_of(&doc, &o, top);
    assert!(
        at(on_high, [on_low[0] + 10.0, on_low[1], 2.0]),
        "the top sits on the high base's cap: {on_high:?}"
    );
}
