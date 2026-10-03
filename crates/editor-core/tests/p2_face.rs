//! **A `FromFace` mate side across the seam, and under `Rebind`**
//! (ASSEMBLY.md A3, A4, A11 (5); the spec's `## P2-split` face rows).
//!
//! A `FromFace` side names no face: its frame is its own head's face,
//! the head with the member walk's qualifiers stripped
//! (`head_face`), read in the member's part. So whatever carries the
//! head carries the frame. Split and inline re-anchor the head, and
//! the frame rule holds a face side only to its member being placed
//! in the part's world; `Rebind` rewrites the head, and the frame
//! follows it onto a renamed face or another part's instance; a
//! pattern copy's head reads its master's face.
//!
//! The scenes are `p2_gauges`'s two literal blocks; every placement is
//! a translation by dyadic lengths, so the world frames compared are
//! exact.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::fixture;
use crate::p2_gauges::{Parts, block, cut, literal, parts, seat, set_gauge, set_offset};

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions, FaceName,
    FacePoseRefusal, FaceRefusal, MateFault, MateFrame, MatePrimitive, MateRole, MateSide, Node,
    NodeErrorKind, PartResolver, PatternKind, ProfileDoc, RecipeNodeId, SitedFace, SplitError,
    StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{head, in_copy, insert, len, run, scl, solve, step, step_with};
use geom_core::Tol;

// ---- substrate ----

/// A declaring `Rest` coincidence between two heads, both sides framed
/// on their own head faces, outward normals opposed.
fn face_mate(a: SitedFace, b: SitedFace) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::FromFace,
            b: MateFrame::FromFace,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// `node` (a mate) with its `a` side authored at the part's origin
/// instead: the authored twin the frame rule holds to all three
/// conditions.
fn authored_a(node: Node<editor_core::ProfileProgram>) -> Node<editor_core::ProfileProgram> {
    let Node::Mate {
        a,
        b,
        class,
        mut alignment,
    } = node
    else {
        panic!("a mate");
    };
    alignment.a = MateFrame::authored([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]);
    Node::Mate {
        a,
        b,
        class,
        alignment,
    }
}

/// Inserts a mate through the store's reach, which resolves its face
/// sides at the door.
fn insert_mate(
    doc: ProfileDoc,
    node: Node<editor_core::ProfileProgram>,
    o: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
    let reach = editor_core::mate_reach::<f64>(o, Tol::witness());
    let (doc, id) = step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
        },
        &reach,
    );
    (doc, id.expect("the mate is minted"))
}

/// **Where a face side's frame is in the world**: the face its head
/// names in the member's part (`head_face`), read through the part's
/// own evaluation (`MateReach::face_pose`), carried by the member's
/// solved world pose and then by `copy_shift`, the translation the
/// scene's pattern puts between a copy and its master — origin, chart
/// axis and roll reference, by bits. A plain read (its own instance,
/// no copy, no placer) takes no shift; a copy read takes its copy's.
fn side_world(
    doc: &ProfileDoc,
    o: &EvalOptions,
    mate: RecipeNodeId,
    side: MateSide,
    copy_shift: [f64; 3],
) -> [u64; 9] {
    let Some(Node::Mate {
        a, b, alignment, ..
    }) = doc.node(mate)
    else {
        panic!("{mate:?} is a mate");
    };
    let (head, frame) = match side {
        MateSide::A => (a, &alignment.a),
        MateSide::B => (b, &alignment.b),
    };
    assert_eq!(
        *frame,
        MateFrame::FromFace,
        "side {} is a face side",
        side.name()
    );
    let member = editor_core::member_of(doc, head).expect("the head reads a member");
    if copy_shift == [0.0; 3] {
        assert!(
            member.copy.is_empty() && member.at == member.instance,
            "a plain read: {member:?}"
        );
    } else {
        assert_eq!(member.copy.len(), 1, "a copy read: {member:?}");
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
    .map(f64::to_bits)
}

fn split(
    doc: &ProfileDoc,
    ids: &[RecipeNodeId],
    label: &str,
    o: &EvalOptions,
) -> editor_core::SplitOutcome {
    editor_core::split(
        doc,
        &cut(ids),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("{label}: split refused: {e}"))
}

fn split_err(doc: &ProfileDoc, ids: &[RecipeNodeId], label: &str, o: &EvalOptions) -> SplitError {
    match editor_core::split(
        doc,
        &cut(ids),
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

/// **The face-crossing scene**: a base at `[4, 0, 0]` and a top mated
/// onto it — one placed group, the top a NON-root member — and a
/// third block `k` on a gauge of its own, so a mate between it and the
/// top declares. `m` is that mate: the top's upper cap to `k`'s bottom
/// cap, both sides framed on their heads. Returns the store's parts,
/// the document and `[base, top, seat, k, m]`.
fn face_across(label: &str) -> (Parts, ProfileDoc, [RecipeNodeId; 5]) {
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 8.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let doc = set_offset(doc, base, Some(literal([4.0, 0.0, 0.0])));
    let (doc, top) = insert(doc, Node::instantiate_part(p.top));
    let (doc, seat_mate) = insert(doc, seat(head(p.top_cap(top)), head(p.base_cap(base))));
    let (doc, k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, k, Some(g));
    let (doc, m) = insert_mate(
        doc,
        face_mate(head(p.top_upper_cap(top)), head(p.top_cap(k))),
        &o,
    );
    (p, doc, [base, top, seat_mate, k, m])
}

// ---- the seam ----

/// **A face side reading a non-root member crosses split and inline,
/// and its world frame does not move.** The kept mate `m` reads the
/// top — the second member of its group, which the authored frame rule
/// refuses — through its head face. Split re-anchors the head through
/// the instance left behind, and the face it names is then a row of
/// the new part; inline unwraps it back onto the inner top. At each of
/// the three documents the side's resolved world frame is the same
/// bits, and `inline(split(d))` is `d` up to node ids under the R1
/// comparator — the mate's two face sides included. The authored twin refuses `MateFrameCrosses`, since
/// its vectors are coordinates of the top and the top is no root.
#[test]
fn a_face_side_reading_a_non_root_member_crosses_split_and_inline_unmoved() {
    let label = "p2-face-nonroot";
    let (p, doc, [base, top, seat_mate, _k, m]) = face_across(label);
    let o = p.opts();
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(m),
        Some(MateRole::Declaring)
    );
    assert_eq!(
        editor_core::root_of(&doc, top),
        base,
        "the top is a non-root member"
    );
    let before = side_world(&doc, &o, m, MateSide::A, [0.0; 3]);

    let out = split(&doc, &[base, top, seat_mate], label, &o);
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let with_part = with_resolver(store.clone());
    let Some(Node::Mate { a, alignment, .. }) = out.remainder.node(m) else {
        panic!("the kept mate");
    };
    assert_eq!(alignment.a, MateFrame::FromFace);
    assert_eq!(
        a.name.node, out.instance,
        "the head re-anchors through the instance"
    );
    assert_eq!(
        side_world(&out.remainder, &with_part, m, MateSide::A, [0.0; 3]),
        before,
        "split: the face side's world frame, bit for bit"
    );

    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(&store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    assert_eq!(
        side_world(&back.doc, &o, m, MateSide::A, [0.0; 3]),
        before,
        "inline: the face side's world frame, bit for bit"
    );
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));

    // The authored twin is held to (a), (b) and (c): the top is no root.
    let Some(face_side) = doc.node(m).cloned() else {
        panic!("the mate");
    };
    let (twin, twin_m) = {
        let (unmated, _) = step(doc.clone(), DocEdit::DeleteNode { id: m });
        insert_mate(unmated, authored_a(face_side), &o)
    };
    let err = split_err(&twin, &[base, top, seat_mate], label, &o);
    assert!(
        matches!(&err, SplitError::MateFrameCrosses { mate, side: MateSide::A } if mate.id() == twin_m),
        "{err:?}"
    );
}

/// **A face side reading a pattern copy crosses split and inline, and
/// its world frame does not move.** Condition (a) — no copy between —
/// holds an authored side alone, and dropping it for a face side is
/// deliberate: the head names the copy's face and crosses as it is. A
/// leg patterned three times two apart along x, and a kept declaring
/// mate whose face side reads copy 2's upper cap. Split cuts the leg
/// and its pattern, and the side then reads the instance plainly, its
/// face the copy's cap as the new part names it; inline unwraps it back
/// onto copy 2 of the inner pattern. At each of the three documents the
/// side's world frame is the same bits, and `inline(split(d))` is `d`
/// up to node ids under the R1 comparator. The authored twin refuses
/// `MateFrameCrosses` at split, since it reads a copy.
#[test]
fn a_face_side_on_a_pattern_copy_crosses_split_and_inline_unmoved() {
    let label = "p2-face-copy-seam";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 8.0])));
    let (doc, leg) = insert(doc, Node::instantiate_part(p.top));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: editor_core::Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, k, Some(g));
    let (doc, m) = insert_mate(
        doc,
        face_mate(
            head(in_copy(pattern, 2, p.top_upper_cap(leg))),
            head(p.top_cap(k)),
        ),
        &o,
    );
    assert_eq!(
        solve(&doc, &o, Tol::witness()).role(m),
        Some(MateRole::Declaring)
    );
    let copy_two = [4.0, 0.0, 0.0];
    let before = side_world(&doc, &o, m, MateSide::A, copy_two);

    let out = split(&doc, &[leg, pattern], label, &o);
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let with_part = with_resolver(store.clone());
    assert_eq!(
        side_world(&out.remainder, &with_part, m, MateSide::A, [0.0; 3]),
        before,
        "split: the face side's world frame, bit for bit"
    );

    let back = editor_core::inline(
        &out.remainder,
        out.instance,
        &resolver(&store),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("inline refused: {e}"));
    assert_eq!(
        side_world(&back.doc, &o, m, MateSide::A, copy_two),
        before,
        "inline: the face side's world frame, bit for bit"
    );
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &back.doc, &map, &steps)
        .unwrap_or_else(|e| panic!("inline(split(d)) is d up to node ids:\n{e}"));

    // The authored twin is held to (a): it reads a copy.
    let Some(face_side) = doc.node(m).cloned() else {
        panic!("the mate");
    };
    let (twin, twin_m) = {
        let (unmated, _) = step(doc.clone(), DocEdit::DeleteNode { id: m });
        insert_mate(unmated, authored_a(face_side), &o)
    };
    let err = split_err(&twin, &[leg, pattern], label, &o);
    assert!(
        matches!(&err, SplitError::MateFrameCrosses { mate, side: MateSide::A } if mate.id() == twin_m),
        "{err:?}"
    );
}

/// **A face side whose member is unplaced refuses `MateFrameCrosses`**
/// at split: a member in its group's own space has no place in the
/// part's world for its face to keep, so the frame would mean another
/// place. The cut holds an unplaced top beside a placed base.
///
/// Inline's mirror — a host side reading an unplaced member of the
/// part — is not a document the doors build: the part's product table
/// carries no row for a body in an own space, so the side's own insert
/// refuses `NoSuchName` before any inline is asked. The row pins that,
/// so the day the product carries such rows the inline half is owed.
#[test]
fn a_face_side_reading_an_unplaced_member_refuses_mate_frame_crosses() {
    let label = "p2-face-unplaced";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, g) = insert(doc, Node::gauge(None, literal([0.0, 0.0, 8.0])));
    let (doc, base) = insert(doc, Node::instantiate_part(p.base));
    let (doc, loose) = insert(doc, fixture::mated_instance(p.top));
    let (doc, k) = insert(doc, Node::instantiate_part(p.top));
    let doc = set_gauge(doc, k, Some(g));
    let (doc, m) = insert_mate(
        doc,
        face_mate(head(p.top_upper_cap(loose)), head(p.top_cap(k))),
        &o,
    );
    assert!(
        solve(&doc, &o, Tol::witness()).unplaced(loose).is_some(),
        "the loose top lives in its own space"
    );
    let err = split_err(&doc, &[base, loose], label, &o);
    assert!(
        matches!(&err, SplitError::MateFrameCrosses { mate, side: MateSide::A } if mate.id() == m),
        "{err:?}"
    );

    // The same two as a part, and a host side reading the loose top.
    let part_doc = ProfileDoc::empty(DocumentId::derive(&format!("{label}-sub")), Tol::witness());
    let (part_doc, _) = insert(part_doc, Node::instantiate_part(p.base));
    let (part_doc, sub_loose) = insert(part_doc, fixture::mated_instance(p.top));
    let mut store = p.store.clone();
    let sub = store.insert(part_doc, Tol::witness());
    let with_sub = with_resolver(store);
    let host = ProfileDoc::empty(DocumentId::derive(&format!("{label}-host")), Tol::witness());
    let (host, g) = insert(host, Node::gauge(None, literal([0.0, 0.0, 8.0])));
    let (host, i) = insert(host, Node::instantiate_part(sub));
    let (host, k) = insert(host, Node::instantiate_part(p.top));
    let host = set_gauge(host, k, Some(g));
    let through = FaceName::new(p.top_upper_cap(sub_loose))
        .expect("a face")
        .in_part(i)
        .into_name();
    let reach = editor_core::mate_reach::<f64>(&with_sub, Tol::witness());
    let refused = host.apply(
        &DocEdit::InsertNode {
            node: Box::new(face_mate(head(through), head(p.top_cap(k)))),
        },
        Tol::witness(),
        &reach,
    );
    assert!(
        matches!(
            &refused,
            Err(editor_core::EditError::MateRefused { fault, .. })
                if matches!(
                    &**fault,
                    MateFault::FaceUnresolved { refusal, .. }
                        if matches!(
                            refusal.as_ref(),
                            FaceRefusal::Reach { refusal: FacePoseRefusal::NoSuchName, .. }
                        )
                )
        ),
        "{:?}",
        refused.as_ref().err()
    );
}

// ---- the strip at a pattern copy ----

/// **A face side on a pattern copy reads its master's face, carried by
/// the copy map.** A leg patterned three times two apart along x, and
/// a base seated on copy 2's upper cap with that side framed on its
/// head: the head is `Instance(2)` round the leg instance's name, the
/// strip takes off that one qualifier and then the instance's
/// `InPart`, and the base lands on the cap of the copy four along x —
/// the master's cap pose moved by the copy's offset.
#[test]
fn a_face_side_on_a_pattern_copy_reads_the_masters_face_at_the_copy() {
    let label = "p2-face-copy";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = insert(doc, Node::instantiate_part(p.top));
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: leg,
            count: editor_core::Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(2.0),
            },
        },
    );
    let (doc, mover) = insert(doc, fixture::mated_instance(p.base));
    let copy_cap = in_copy(pattern, 2, p.top_upper_cap(leg));
    let node = Node::Mate {
        a: head(in_part(mover, p.base_body, CapEnd::Start)),
        b: head(copy_cap.clone()),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored([0.0; 3], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            b: MateFrame::FromFace,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    };
    let (doc, m) = insert_mate(doc, node, &o);
    let ev = run(&doc, &o);
    assert!(ev.node_error(m).is_none(), "{:?}", ev.node_error(m));
    let Some(Node::Mate { b, .. }) = doc.node(m) else {
        panic!("the mate");
    };
    let member = editor_core::member_of(&doc, b).expect("a copy is a member");
    assert_eq!(member.copy, vec![(pattern, 2)]);
    let master = editor_core::head_face(&doc, b).expect("the strip");
    assert_eq!(
        master.clone().into_name(),
        StableName {
            kind: editor_core::EntityKind::Face,
            node: p.top_body,
            path: vec![editor_core::RoleSeg::Cap(CapEnd::End)],
        },
        "the master's own row"
    );
    let reach = editor_core::PartReach::<f64>::with_resolver(o.resolver.as_ref(), Tol::witness());
    let pose = editor_core::MateReach::face_pose(&reach, &p.top, &master).expect("a pose");
    let placed = solve(&doc, &o, Tol::witness())
        .placement(&doc, mover)
        .expect("the mover is placed");
    assert_eq!(
        placed.translation.map(f64::to_bits),
        [pose.origin.x + 4.0, pose.origin.y, pose.origin.z].map(f64::to_bits),
        "the master's cap, moved to copy 2"
    );
}

/// **A head that names no face of its part refuses `NoPartFace`** at
/// the insert door, with its recourse: a face name headed at the
/// instance but not wrapped in the instance's `InPart` walks to a
/// member, and there is no part row under it to read a frame off. An
/// instance mints no such name, so only a head written by hand meets
/// this.
#[test]
fn a_head_naming_no_part_face_refuses_no_part_face_at_the_door() {
    let label = "p2-face-no-part-face";
    let (p, doc, [base, _top, _seat, k, m]) = face_across(label);
    let o = p.opts();
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: m });
    let bare = StableName {
        kind: editor_core::EntityKind::Face,
        node: base,
        path: Vec::new(),
    };
    assert!(
        editor_core::member_of(&doc, &head(bare.clone())).is_some(),
        "the bare name walks to the base's member"
    );
    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let refused = doc.apply(
        &DocEdit::InsertNode {
            node: Box::new(face_mate(head(bare), head(p.top_cap(k)))),
        },
        Tol::witness(),
        &reach,
    );
    let Err(editor_core::EditError::MateRefused { fault, .. }) = &refused else {
        panic!("the door refuses the mate: {:?}", refused.as_ref().err());
    };
    assert!(
        matches!(
            &**fault,
            MateFault::FaceUnresolved { side: MateSide::A, refusal, .. }
                if matches!(
                    refusal.as_ref(),
                    FaceRefusal::NoPartFace { instance, .. } if *instance == base
                )
        ),
        "{fault:?}"
    );
    assert!(
        fault.to_string().contains("Recourse: delete the mate"),
        "the refusal states its recourse: {fault}"
    );
}

// ---- Rebind ----

/// The base part rebuilt with a NEW extrude of `height` over the same
/// profile — every face of it renamed, since a face name derives from
/// the node that mints it — and the new body.
fn renamed(base: &ProfileDoc, body: RecipeNodeId, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let Some(Node::Extrude { profile, .. }) = base.node(body).cloned() else {
        panic!("the base's body is an extrude");
    };
    let (base, _) = step(base.clone(), DocEdit::DeleteNode { id: body });
    insert(
        base,
        Node::Extrude {
            profile,
            distance: len(height),
        },
    )
}

/// The top seated on the base's upper cap, the base side framed on its
/// head: the top's origin lands on the cap's canonical origin.
fn on_base_cap(top: SitedFace, base_cap: SitedFace) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: top,
        b: base_cap,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored([0.0; 3], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
            b: MateFrame::FromFace,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// The top's world origin, by bits.
fn origin_of(doc: &ProfileDoc, o: &EvalOptions, id: RecipeNodeId) -> [u64; 3] {
    solve(doc, o, Tol::witness())
        .placement(doc, id)
        .expect("placed")
        .translation
        .map(f64::to_bits)
}

/// **A face side follows its head through a part-side rename and its
/// repair.** The base part is rebuilt so its upper cap carries a new
/// name, and the assembly's reference moves to it: the head now names
/// a row the part does not have, and the side faults `NoSuchName`.
/// `Rebind` of the head onto the new row repairs the mate whole — the
/// frame IS the head's face, so there is nothing else to rewrite — and
/// the top lands on the rebuilt cap, at its new height.
#[test]
fn a_rename_update_and_rebind_carry_the_face_side_with_the_head() {
    let label = "p2-face-rebind";
    let mut store = PartStore::new();
    let (base_doc, base_body) = block(&format!("{label}-base"), 3.0, 1.0);
    let base_ref = store.insert(base_doc.clone(), Tol::witness());
    let (top_ref, top_body) =
        store.insert_part(block(&format!("{label}-top"), 1.0, 3.0), Tol::witness());
    let o = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let old_head = in_part(base, base_body, CapEnd::End);
    let (doc, m) = insert_mate(
        doc,
        on_base_cap(
            head(in_part(top, top_body, CapEnd::Start)),
            head(old_head.clone()),
        ),
        &o,
    );
    assert_eq!(origin_of(&doc, &o, top)[2], 1.0_f64.to_bits(), "on the cap");

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
    let ev = run(&doc, &o);
    let fault = match &ev.node_error(m).expect("the stranded head faults").kind {
        NodeErrorKind::Mate(fault) => (**fault).clone(),
        other => panic!("a mate's own refusal: {other}"),
    };
    assert!(
        matches!(
            &fault,
            MateFault::FaceUnresolved { side: MateSide::B, refusal, .. }
                if matches!(refusal.as_ref(), FaceRefusal::Reach { refusal: FacePoseRefusal::NoSuchName, .. })
        ),
        "{fault:?}"
    );

    let (doc, _) = step_with(
        doc,
        DocEdit::Rebind {
            from: old_head,
            to: in_part(base, new_body, CapEnd::End),
        },
        &reach,
    );
    let ev = run(&doc, &o);
    assert!(
        ev.node_error(m).is_none(),
        "repaired whole: {:?}",
        ev.node_error(m)
    );
    assert_eq!(
        origin_of(&doc, &o, top)[2],
        1.5_f64.to_bits(),
        "the top follows the rebuilt cap"
    );
}

/// **A head rebound onto another part's instance reads that part's
/// face.** Two bases built alike — the same node ids, one cap name
/// row for row — at different heights and places; the top seated on
/// the first's cap, then its head rebound onto the second's. The frame
/// reads the second base's cap, in the second part: the top lands
/// there, at the second's height and place, and nowhere a name read in
/// the wrong part could put it.
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
    let (doc, low) = insert(doc, Node::instantiate_part(low_ref));
    let (doc, high) = insert(doc, Node::instantiate_part(high_ref));
    let doc = set_offset(doc, high, Some(literal([10.0, 0.0, 0.0])));
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let low_head = in_part(low, low_body, CapEnd::End);
    let (doc, m) = insert_mate(
        doc,
        on_base_cap(
            head(in_part(top, top_body, CapEnd::Start)),
            head(low_head.clone()),
        ),
        &o,
    );
    let on_low = origin_of(&doc, &o, top);
    assert_eq!(on_low[2], 1.0_f64.to_bits());

    let reach = editor_core::mate_reach::<f64>(&o, Tol::witness());
    let (doc, _) = step_with(
        doc,
        DocEdit::Rebind {
            from: low_head,
            to: in_part(high, high_body, CapEnd::End),
        },
        &reach,
    );
    let ev = run(&doc, &o);
    assert!(ev.node_error(m).is_none(), "{:?}", ev.node_error(m));
    let on_high = origin_of(&doc, &o, top);
    assert_eq!(
        on_high,
        [
            f64::from_bits(on_low[0]) + 10.0,
            f64::from_bits(on_low[1]),
            2.0
        ]
        .map(f64::to_bits),
        "the top sits on the high base's cap"
    );
}
