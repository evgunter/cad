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
    CapEnd, DocEdit, DocumentId, EvalOptions, FaceName, MateFault, MateRef, MateSide, Node,
    PartResolver, ProfileDoc, RecipeNodeId, SitedFace, SplitError, StableName,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::round_trip::{composed, same_up_to_ids};
use fixture::{head, head_at, in_copy, insert, len, run, solve, step};
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

/// The selection a stored mate's target reads.
fn target_var(doc: &ProfileDoc, mate: MateRef) -> editor_core::VarId {
    let stored = fixture::constraint(doc, mate);
    let to = stored.to;
    let [select] = fixture::face_reads(doc, to)[..] else {
        panic!("the target reads one face")
    };
    select
}

/// **`face`'s pose in its part's own coordinates**: the part resolved
/// through `o`'s resolver and evaluated on its own, the face read at
/// the part placement that names it.
fn part_face_pose(
    o: &EvalOptions,
    part: &editor_core::DocRef,
    face: &FaceName,
) -> topo::readback::Pose<f64> {
    let resolver = o.resolver.as_ref().expect("a resolver");
    let doc = resolver
        .resolve(part, Tol::witness())
        .unwrap_or_else(|failure| panic!("the part resolves: {failure:?}"));
    let name = face.clone().into_name();
    editor_core::face_frame(&run(&doc, o), name.node, &name)
        .unwrap_or_else(|refusal| panic!("the face has a pose: {refusal:?}"))
}

/// **Where a mate's target face is in the world**: the face its head
/// names in the member's part (`head_face`), read through the part's
/// own evaluation ([`part_face_pose`]), carried by the member's
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
        assert_eq!(member.chain.len(), 1, "a member's read: {member:?}");
    }
    let face = editor_core::head_face(doc, head).expect("the head names a part face");
    // A family's member stands on the instance the family places.
    let instance = match doc.node(member.instance) {
        Some(Node::Place { source, .. }) => {
            doc.operation_of(source.read).expect("the family's source")
        }
        _ => member.instance,
    };
    let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(instance) else {
        panic!("the member stands on an instance");
    };
    let pose = part_face_pose(o, doc_ref, &face);
    let placed = solve(doc, o, Tol::witness())
        .placement(doc, instance)
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
        fixture::rest(head(p.top_cap(k)), head(p.top_upper_cap(top))),
    );
    (p, doc, [base, top, k], m)
}

// ---- the seam ----

/// **A kept mate's target written in place crosses split; the same
/// target named refuses**. `face_across`'s `m` reads the top's upper cap through a pose: written
/// in place it is re-authored over the instance's copy, named it is no
/// read the crossing walk carries, and `k` is severed from the cut.
///
/// *Breaks if* the anonymous target stops crossing, or the named one
/// is admitted without its pose being carried.
#[test]
fn a_kept_mates_named_target_is_severed_where_its_written_one_crosses() {
    for named in [false, true] {
        let label = if named {
            "p2-face-named-to"
        } else {
            "p2-face-written-to"
        };
        let (p, doc, [base, top, k], m) = face_across(label);
        let o = p.opts();
        let doc = if named {
            let to = fixture::constraint(&doc, m).to;
            step(
                doc,
                DocEdit::RenameVar {
                    var: to.into(),
                    name: Some(editor_core::VarName::from_static("seat_plane")),
                },
            )
            .0
        } else {
            doc
        };
        if named {
            match split_err(&doc, &[base, top], label, &o) {
                SplitError::SeveredEdge {
                    consumer,
                    consumer_is_cut: false,
                    ..
                } => assert_eq!(consumer.id(), k, "{label}: the kept mate's placement"),
                other => panic!("{label}: expected SeveredEdge, got {other:?}"),
            }
        } else {
            split(&doc, &[base, top], label, &o);
        }
    }
}

/// **A named number a kept mate's target slides by, and a cut node
/// reads, is one variable on both sides** (`UncutVarReference`): the
/// pose's numbers are its reader's reads, so the split refuses rather
/// than move the number and leave the kept mate reading none.
///
/// *Breaks if* a pose's numbers stop counting as its reader's reads.
#[test]
fn a_named_number_a_kept_mates_target_and_a_cut_node_read_refuses() {
    let label = "p2-face-shared-number";
    let p = parts(label);
    let o = p.opts();
    let gap = || {
        editor_core::Formula::named(
            editor_core::VarName::from_static("gap"),
            editor_core::Dimension::Length,
        )
    };
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, _) = step(
        doc,
        DocEdit::DeclareVar {
            name: editor_core::VarName::from_static("gap"),
            def: editor_core::VarDecl::defined(len(0.0)),
        },
    );
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, base) = pinned_at(doc, p.base, [4.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _seat) = seated(doc, top, head(p.top_cap(top)), head(p.base_cap(base)));
    let mut rest = fixture::rest(head(p.top_cap(k)), head(p.top_upper_cap(top)));
    rest.to = editor_core::Operand::Pose(Box::new(editor_core::pose::PoseDef::Slide {
        pose: rest.to.clone().into(),
        along: editor_core::pose::RoleRead {
            read: rest.to.clone(),
            role: Some(editor_core::pose::PoseRole::Normal),
        },
        by: gap(),
    }));
    let (doc, _) = fixture::constrain_at(doc, k, rest);
    let (doc, profile) = fixture::on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(9.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: gap(),
        },
    );
    let mut ids = cut(&doc, &[base, top]);
    ids.extend([profile, extrude]);
    match editor_core::split(
        &doc,
        &ids,
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    ) {
        Err(SplitError::UncutVarReference { var, kept_node, .. }) => {
            assert_eq!(var.name().map(editor_core::VarName::as_str), Some("gap"));
            assert_eq!(kept_node.id(), k, "the kept mate's placement reads it");
        }
        other => panic!("expected UncutVarReference, got {other:?}"),
    }
}

/// **A valued mate whose target reads a mate-placed copy crosses
/// split, and stands where it stood** (D′2). The kept constraint `m`
/// reads the top — placed by its mate, not pinned — through its head
/// face, and every bundle pins its copy, so `m` carries values. A value
/// is charted on the target space, the world on both sides of the seam,
/// not on the body the target is read off, so the top's own placement
/// is nothing the crossing has to carry: the split admits the reader,
/// and `k` lands where it landed.
///
/// *Breaks if* split refuses the reader for where its target body
/// stands, or the crossing moves `k`.
#[test]
fn a_valued_mate_reading_a_mate_placed_copy_crosses_split() {
    let label = "p2-face-nonroot";
    let (p, doc, [base, top, k], _m) = face_across(label);
    let o = p.opts();
    assert!(
        fixture::offset_of(&doc, top).is_none(),
        "the top is placed by its mate, not pinned"
    );
    let before = crate::p2_gauges::min_corner(&crate::p2_gauges::body_of(&run(&doc, &o), k));
    let out = editor_core::split(
        &doc,
        &cut(&doc, &[base, top]),
        DocumentId::derive(&format!("{label}-part")),
        Tol::witness(),
        o.resolver.as_ref(),
    )
    .unwrap_or_else(|e| panic!("a valued mate on a mate-placed copy crosses: {e}"));
    let mut store = p.store.clone();
    store.insert(out.part.clone(), Tol::witness());
    let after = crate::p2_gauges::min_corner(&crate::p2_gauges::body_of(
        &run(&out.remainder, &with_resolver(store)),
        k,
    ));
    crate::p2_gauges::close(before, after, "k stands where it stood");
}

/// **A mate whose target reads a family's member crosses split and
/// inline, and the target's world pose does not move.** The head names
/// the member's face and crosses as it is. A leg in a row of three two
/// apart along x, every member in the world, and a kept constraint
/// placing `k` whose target reads member 2's upper cap. Split cuts the
/// leg and its family, and the target then reads the instance plainly,
/// its face the member's cap as the new part names it; inline unwraps
/// it back onto member 2 of the family, and flatten folds the new space
/// away. The target's world pose is the same through the split and the
/// inline.
///
/// *Breaks if* the member's face crosses as its master's, or moves.
#[test]
fn a_mate_reading_a_family_member_crosses_split_and_inline_unmoved() {
    let label = "p2-face-copy-seam";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    // The kept copy first: split appends its instance and inline what it
    // splices, so the round trip keeps the placements' order (A4).
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, leg) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.top), &o);
    let (doc, kk, pattern) = fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 3);
    let copy_cap = head(in_copy(pattern, kk, 2, p.top_upper_cap(leg)));
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
    // Flattened, the target reads member 2 of the family again: the
    // member's name maps back through its index, so inline of split is
    // the document up to ids.
    let flat = crate::fixture::round_trip::flat(&back);
    let (map, steps) = composed(&doc, &out, &back);
    same_up_to_ids(&doc, &flat, &map, &steps)
        .unwrap_or_else(|said| panic!("inline of split is the document up to ids: {said}"));
    let corner = |doc: &ProfileDoc| {
        crate::p2_gauges::min_corner(&crate::p2_gauges::body_of(&run(doc, &o), k))
    };
    crate::p2_gauges::close(corner(&flat), corner(&doc), "k stands where it stood");
}

/// **A mate whose target reads a copy in a space of its own refuses at split**: a copy
/// in its own space has no place in the part's world for its face to
/// keep, so the face would mean another place. The cut holds a top in a
/// space of its own beside a pinned base, and `k` is placed against that
/// top: the kept mate's read of the cut is severed.
///
/// Inline's mirror — a host mate reading a copy in a space of its own of the part — is
/// not a document inline is asked about: the part's product table
/// carries no row for a body in a space of its own, so the door admits the
/// mate as written and its target does not evaluate, which is `k`'s own
/// refusal. The row pins that, so the day the product carries such rows
/// the inline half is owed.
///
/// *Breaks if* a face of a copy in a space of its own crosses the seam, or a host mate
/// reading one evaluates.
#[test]
fn a_mate_reading_a_copy_in_a_space_of_its_own_refuses_at_split() {
    let label = "p2-face-unplaced";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.base), &o);
    let (doc, aside) = insert(doc, fixture::mated_instance(p.top));
    let (doc, space) = fixture::into_new_space(doc, aside);
    let (doc, k) = insert(doc, fixture::mated_instance(p.top));
    let (doc, _) = face_mate(doc, k, head(p.top_cap(k)), head(p.top_upper_cap(aside)));
    assert_eq!(
        solve(&doc, &o, Tol::witness()).space(aside),
        Some(space),
        "the top lives in its own space"
    );
    let err = split_err(&doc, &[base, aside], label, &o);
    assert!(
        matches!(
            &err,
            SplitError::SeveredEdge { consumer, input, .. }
                if consumer.id() == k && input.id() == aside
        ),
        "{err:?}"
    );

    // The same two as a part, and a host mate reading the top aside.
    let part_doc = ProfileDoc::empty(DocumentId::derive(&format!("{label}-sub")), Tol::witness());
    let (part_doc, _sub_base) =
        fixture::insert_pinned(part_doc, crate::fixture::mated_instance(p.base), &o);
    let (part_doc, sub_aside) = insert(part_doc, fixture::mated_instance(p.top));
    let (part_doc, _) = fixture::into_new_space(part_doc, sub_aside);
    let mut store = p.store.clone();
    let part_doc = crate::fixture::assemble_world_copies(part_doc);
    let sub = store.insert(part_doc, Tol::witness());
    let with_sub = with_resolver(store);
    let host = ProfileDoc::empty(DocumentId::derive(&format!("{label}-host")), Tol::witness());
    let (host, i) = fixture::insert_pinned(host, crate::fixture::mated_instance(sub), &with_sub);
    let (host, k) = insert(host, fixture::mated_instance(p.top));
    let through = FaceName::new(p.top_upper_cap(sub_aside).name)
        .expect("a face")
        .into_name();
    // Read at the instance: a face its copy does not carry.
    let (host, _) = face_mate(host, k, head(p.top_cap(k)), fixture::head_at(i, through));
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

/// **A mate's target on a family's member reads its master's face,
/// carried by the member's motion.** A leg in a row of three two apart
/// along x, and a base seated on member 2's upper cap: the head is
/// `Member { (k, 2) }` round the leg's part name, the strip takes off
/// that one qualifier, reading the part's own name at the instance, and
/// the base's bottom lands on the master's cap plane. A value on a
/// target read off a member is charted on the target space's axes,
/// whichever member it is (D′2,
/// `work/intent/a-value-on-a-placed-copy-target-is-charted-on-the-space.md`),
/// so the seat's slide of 4 along x stands the base over member 2.
///
/// *Breaks if* the strip reads another face, or the member's motion is
/// not applied.
#[test]
fn a_mate_on_a_family_member_reads_the_masters_face_at_the_member() {
    let label = "p2-face-copy";
    let p = parts(label);
    let o = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, leg) = fixture::insert_pinned(doc, crate::fixture::mated_instance(p.top), &o);
    let (doc, kk, pattern) = fixture::linear(doc, leg, [1.0, 0.0, 0.0], 2.0, 3);
    let (doc, mover) = insert(doc, fixture::mated_instance(p.base));
    let copy_cap = in_copy(pattern, kk, 2, p.top_upper_cap(leg));
    let (doc, m) = seat_at(
        doc,
        mover,
        head_at(mover, in_part(mover, p.base_body, CapEnd::Start)),
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
    let member = editor_core::member_of(&doc, b).expect("a member of a family is a member");
    assert_eq!(
        member.chain,
        vec![editor_core::Placing::Member {
            family: pattern,
            at: vec![(kk, 2)],
        }]
    );
    let master = editor_core::head_face(&doc, b).expect("the strip");
    assert_eq!(
        master.clone().into_name(),
        fixture::resolver::in_world(p.top_body, CapEnd::End),
        "the master's own row, as the part's world names it"
    );
    let pose = part_face_pose(&o, &p.top, &master);
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
        "the master's cap, moved to member 2: {:?} vs {want:?}",
        placed.translation
    );
}

/// **A head that names no face of its part does not evaluate**: a face
/// name headed at the instance's own node, which is no name of its
/// part, walks to a member, and there is no part row under it to
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
    let (p, doc, [base, _top, k], _) = face_across(label);
    let o = p.opts();
    // The bare-headed mate is written in `m`'s place, whole.
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
    let Some(Node::Extrude { profile, .. }) = base.node(body).cloned() else {
        panic!("the base's body is an extrude");
    };
    let old_copies: Vec<RecipeNodeId> = base
        .placements()
        .into_iter()
        .filter(|&at| {
            matches!(base.node(at), Some(Node::Place { source, .. })
                if (base.operation_of(source.read) == Some(body)))
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
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let old_head = in_part(base, base_body, CapEnd::End);
    let (doc, m) = face_mate(
        doc,
        top,
        head_at(top, in_part(top, top_body, CapEnd::Start)),
        head(old_head.clone()),
    );
    assert!(
        (origin_of(&doc, &o, top)[2] - 1.0).abs() <= 1e-12,
        "on the cap"
    );

    let (rebuilt, new_body) = renamed(&base_doc, base_body, 1.5);
    assert_ne!(new_body, base_body, "the cap is renamed");
    let rebuilt = crate::fixture::assemble_world_copies(rebuilt);
    let new_ref = store.insert(rebuilt, Tol::witness());
    let o = with_resolver(store.clone());
    let (doc, _) = step(
        doc,
        DocEdit::UpdateReference {
            node: base,
            new_pin: new_ref.pin,
        },
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
    let (doc, _) = step(
        doc,
        DocEdit::Rebind {
            body: Some(base_out),
            from: old_head.name,
            to: in_part(base, new_body, CapEnd::End).name,
        },
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
/// part: the top lands on it, at the second's height, its values still
/// measured on the world's axes (D′2: a value charts on the target
/// space, not the body read), and nowhere a name read in the wrong part
/// could put it.
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
    let (doc, low) = fixture::insert_pinned(doc, crate::fixture::mated_instance(low_ref), &o);
    let (doc, high) = pinned_at(doc, high_ref, [10.0, 0.0, 0.0], &o);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, m) = face_mate(
        doc,
        top,
        head_at(top, in_part(top, top_body, CapEnd::Start)),
        head_at(low, in_part(low, low_body, CapEnd::End)),
    );
    let on_low = origin_of(&doc, &o, top);
    assert!(
        (on_low[2] - 1.0).abs() <= 1e-12,
        "on the low cap: {on_low:?}"
    );

    let (doc, _) = step(
        doc,
        DocEdit::SetParam {
            node: m.placement,
            slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Side {
                constraint: m.id,
                side: MateSide::To,
            }),
            value: editor_core::SlotValue::Read(fixture::flip(fixture::plane(head_at(
                high,
                in_part(high, high_body, CapEnd::End),
            )))),
            fresh: Vec::new(),
        },
    );
    let ev = run(&doc, &o);
    assert!(
        ev.node_error(m.placement).is_none(),
        "{:?}",
        ev.node_error(m.placement)
    );
    let on_high = origin_of(&doc, &o, top);
    assert!(
        at(on_high, [on_low[0], on_low[1], 2.0]),
        "the top sits on the high base's cap: {on_high:?}"
    );
}
