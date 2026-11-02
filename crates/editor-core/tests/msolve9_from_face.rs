//! MSOLVE-9 acceptance — **a mate that reads a face of a part, which
//! resolves where the placement evaluates** (`ASSEMBLY.md` A11 rule 5
//! is the ratified sentence).
//!
//! A mate's pose is a read off geometry: a face read as its plane is
//! bound where the placement holding the mate evaluates, off the
//! copy's own evaluated body, so nothing is stored twice — edit the
//! part so the face moves, and the mate follows.
//!
//! The rows go through ordinary doors with a `PartStore`: the item's
//! own document (a block seated on a post's cap, the post's height
//! edited, the mate following by the height change); a NURBS face, a tie, a
//! vanished name and an unresolvable part refusing where the placement
//! evaluates and never at the door or at load; the memo key moving
//! under an edit to the face's part and holding under one outside it;
//! an analysis lane resolving; and every tracked document loading on
//! this wire and re-saving byte for byte.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;
use test_utils::refusal::tagged;

use std::sync::Arc;

use editor_core::mate::SurfaceKind;
use editor_core::pose::{PoseDef, PoseReadFault};
use editor_core::{
    CancelToken, CapEnd, DocEdit, DocumentId, EntityKind, EvalOptions, Evaluation, Formula, Frame,
    MateFault, MateRef, MateSide, Node, NodeErrorKind, NodeResult, PartFault, PersistError,
    ProfileDoc, ProfileProgram, REGENERATE_RECOURSE, RecipeNodeId, RoleSeg, SlotId, StableName,
    VarDef, VarKind, all_faces, face_carrier_kind, face_frame, load, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{could_look, insert, len, on_frame, run, solve, square, step};
use geom_core::Tol;

// ---- Substrate ----

/// A square block of half-side `half` centred on the origin, extruded
/// `height` along +z, and its body: the extrude, whose cap faces are
/// the part's own rows at `RoleSeg::Cap(..)`.
fn block(label: &str, half: f64, height: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, half)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    )
}

/// **`doc` with `body` placed in its world**, as
/// [`PartStore::insert_part`] places it — so [`fixture::resolver::in_world`]
/// spells the part's caps through that placement wherever the part is
/// stored.
fn placed(doc: ProfileDoc, body: RecipeNodeId) -> ProfileDoc {
    let mut scratch = PartStore::new();
    let (doc_ref, _) = scratch.insert_part((doc, body), Tol::witness());
    scratch.doc(doc_ref.id)
}

/// **`part` with `root` placed in its world, and `face` of `root` as
/// the part's product names it**: under that placement's copy.
fn world_face(part: ProfileDoc, root: RecipeNodeId, face: &StableName) -> (ProfileDoc, StableName) {
    let (part, _) = fixture::place(part, root);
    (part, face.clone())
}

fn cap(body: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// `face`, a face of a part by the part's own name, as instance
/// `at` wears it.
fn worn_face(at: RecipeNodeId, face: &StableName) -> editor_core::SitedFace {
    fixture::head_at(at, face.clone())
}

/// **`block_i`'s bottom cap seated on `face` of instance `at`**, the
/// face named by its part's own name: the rest, and the block's origin
/// over the target body's and its axes the target's — the three
/// values zero. Answers the rest's address.
fn seated_on(
    doc: ProfileDoc,
    at: RecipeNodeId,
    face: &StableName,
    (block_i, block_body): (RecipeNodeId, RecipeNodeId),
) -> (ProfileDoc, MateRef) {
    // The rest and its three values, each 0.
    fixture::constrain_at(
        doc,
        block_i,
        fixture::rest(
            fixture::head_at(block_i, in_part(block_i, block_body, CapEnd::Start)),
            worn_face(at, face),
        ),
    )
}

/// **The item's document**: a post and a block in a store, the post
/// pinned to the world and the block SEATED on the post's top cap by
/// its bundle. Returns the assembly, the two instances and their
/// parts' bodies, the rest, the store's options and the post document
/// as stored.
struct Seat {
    doc: ProfileDoc,
    post_i: RecipeNodeId,
    block_i: RecipeNodeId,
    post_body: RecipeNodeId,
    block_body: RecipeNodeId,
    mate: MateRef,
    opts: EvalOptions,
    store: PartStore,
    post: ProfileDoc,
}

fn seat(label: &str, post_height: f64) -> Seat {
    let mut store = PartStore::new();
    let (post, post_body) = block(&format!("{label}-post"), 0.5, post_height);
    let post = placed(post, post_body);
    let post = crate::fixture::assemble_world_copies(post);
    let post_ref = store.insert(post.clone(), Tol::witness());
    let (block_ref, block_body) =
        store.insert_part(block(&format!("{label}-block"), 0.5, 0.25), Tol::witness());
    let opts = with_resolver(store.clone());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, post_i) = insert(doc, crate::fixture::mated_instance(post_ref));
    let doc = stood_on_its_base(doc, post_i, post_body);
    let (doc, block_i) = insert(doc, fixture::mated_instance(block_ref));
    let (doc, mate) = seated_on(
        doc,
        post_i,
        &fixture::resolver::in_world(post_body, CapEnd::End),
        (block_i, block_body),
    );
    assert_eq!(mate.placement, block_i, "the seat is the block's");
    Seat {
        doc,
        post_i,
        block_i,
        post_body,
        block_body,
        mate,
        opts,
        store,
        post,
    }
}

/// **`instance` standing in the world on its part's base**: by the
/// identity bundle, its part's start cap on the world's `xy` — so an
/// edit growing the part moves its top and keeps its base where it
/// stands.
fn stood_on_its_base(doc: ProfileDoc, instance: RecipeNodeId, _body: RecipeNodeId) -> ProfileDoc {
    fixture::stand_again(doc, instance)
}

/// The post's cap pose, read off the post document's OWN evaluation
/// through the name door — the oracle every resolution is held to.
fn cap_pose(post: &ProfileDoc, body: RecipeNodeId, end: CapEnd) -> topo::readback::Pose<f64> {
    let ev = run(post, &EvalOptions::default());
    face_frame(&ev, body, &cap(body, end)).expect("the cap has a pose")
}

/// Whether `frame` is the identity turn moved to `at`, to rounding:
/// the post is pinned over its geometry, which stands at the identity
/// to the rounding of that pin.
fn at_translation(frame: &Frame, at: [f64; 3]) -> bool {
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-12;
    let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    frame
        .columns
        .iter()
        .zip(identity)
        .all(|(c, i)| c.iter().zip(i).all(|(&x, y)| near(x, y)))
        && frame.translation.iter().zip(at).all(|(&x, y)| near(x, y))
}

/// The bundle refusal a placement carries.
fn bundle_fault(ev: &Evaluation<f64>, placement: RecipeNodeId) -> MateFault {
    match &ev
        .node_error(placement)
        .expect("the placement refused")
        .kind
    {
        NodeErrorKind::Mate(fault) => (**fault).clone(),
        other => panic!("a mate's own refusal: {other}"),
    }
}

/// The refusal `mate`'s target met where its placement evaluated.
fn target_refusal(ev: &Evaluation<f64>, mate: MateRef) -> Box<editor_core::NodeRefusal> {
    match bundle_fault(ev, mate.placement) {
        MateFault::SideUnevaluated {
            mate: m,
            side: MateSide::To,
            refusal,
        } if m == mate => refusal,
        other => panic!("expected the target unevaluated, got {other:?}"),
    }
}

/// The post re-stored at `height`, and the assembly re-pinned to it:
/// the ordinary part-edit path — the part document edited through
/// its own door, the store holding the new version, the reference
/// moved by `UpdateReference`.
fn shorten_or_grow(s: &mut Seat, height: f64) {
    edit_the_post(
        s,
        DocEdit::SetParam {
            node: s.post_body,
            slot: SlotId::Distance,
            value: len(height).into(),
            fresh: Vec::new(),
        },
    );
}

/// An edit to the post document, stored under a new pin, and the
/// assembly's reference moved to it — the post as the store now holds
/// it.
fn edit_the_post(s: &mut Seat, edit: DocEdit<ProfileProgram>) {
    let (post, _) = step(s.post.clone(), edit);
    let new_ref = s.store.insert(post.clone(), Tol::witness());
    s.post = post;
    s.opts = with_resolver(s.store.clone());
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::UpdateReference {
            node: s.post_i,
            new_pin: new_ref.pin,
        },
    );
    s.doc = doc;
}

// ---- A1: the item's document ----

/// **Edit the part, and the mate follows.** The block's solved pose
/// is on the post's cap — origin over the post's, `z` at the post's
/// height — before and after the post is grown, by the height change;
/// the gate certifies the seat both times, and the document stores the
/// target as a read of the face: no name copied, no number.
///
/// *Breaks if* the target is bound anywhere but the post's evaluated
/// cap, or the document stores where the cap was.
#[test]
fn a1_the_mate_follows_the_edited_face() {
    let mut s = seat("msolve9-a1", 1.0);
    let before = cap_pose(&s.post, s.post_body, CapEnd::End);
    assert_eq!(
        before.origin.z.to_bits(),
        1.0_f64.to_bits(),
        "the cap is at the height"
    );
    let placed = solve(&s.doc, &s.opts, Tol::witness())
        .placement(&s.doc, s.block_i)
        .expect("the block is seated");
    assert!(
        at_translation(&placed, [0.0, 0.0, 1.0]),
        "seated on the cap: {placed:?}"
    );
    let ev = run(&s.doc, &s.opts);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    assert!(
        could_look(&s.doc, &ev).is_ok(),
        "the seat certifies: {:?}",
        could_look(&s.doc, &ev).err()
    );

    shorten_or_grow(&mut s, 1.3);
    let after = cap_pose(&s.post, s.post_body, CapEnd::End);
    assert_eq!(
        after.origin.z.to_bits(),
        1.3_f64.to_bits(),
        "the cap moved with the height"
    );
    let moved = solve(&s.doc, &s.opts, Tol::witness())
        .placement(&s.doc, s.block_i)
        .expect("the block is still seated");
    assert!(
        at_translation(&moved, [0.0, 0.0, 1.3]),
        "seated on the moved cap: {moved:?}"
    );
    let ev = run(&s.doc, &s.opts);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    assert!(
        could_look(&s.doc, &ev).is_ok(),
        "no refutation: the mate followed the face: {:?}",
        could_look(&s.doc, &ev).err()
    );

    // Nothing stored twice: the target is the face's plane, flipped —
    // a read of the face, with no number of where it was.
    let mate = fixture::constraint(&s.doc, s.mate);
    let to = mate.to;
    let flipped = match s.doc.var(to).map(editor_core::Var::def) {
        Some(VarDef::Pose(PoseDef::Flip { pose })) => pose.read,
        other => panic!("the target is the cap's plane, flipped: {other:?}"),
    };
    assert!(
        matches!(
            s.doc.var(flipped).map(editor_core::Var::def),
            Some(VarDef::Pose(PoseDef::Plane { .. }))
        ),
        "the flipped pose is the face read as a plane"
    );
}

/// `loops` revolved a full turn about the profile's own y axis.
fn revolved(doc: ProfileDoc, loops: Vec<Vec<(f64, f64)>>) -> ProfileDoc {
    let (doc, profile) = insert(doc, Node::Profile(fixture::desc(loops)));
    let (doc, _) = insert(
        doc,
        fixture::revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            fixture::ang(std::f64::consts::TAU),
        ),
    );
    doc
}

/// **`face`, a face of `part` by the part's product name, as the target
/// of a block resting on it**: the block's bundle refuses with its
/// target's own refusal where it evaluates, or not at all. Answers the
/// evaluation and the rest's address.
fn rest_on_the_face(
    label: &str,
    part: ProfileDoc,
    face: &StableName,
) -> (Evaluation<f64>, MateRef) {
    let mut store = PartStore::new();
    let part = crate::fixture::assemble_world_copies(part);
    let part_ref = store.insert(part, Tol::witness());
    let (block_ref, block_body) =
        store.insert_part(block(&format!("{label}-block"), 0.5, 0.25), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = fixture::insert_pinned(doc, crate::fixture::mated_instance(part_ref), &opts);
    let (doc, b) = insert(doc, fixture::mated_instance(block_ref));
    // The door decides nothing that needs a pose: the rest enters
    // whatever the face is.
    let (doc, mate) = fixture::constrain_at(
        doc,
        b,
        fixture::rest(
            fixture::head_at(b, in_part(b, block_body, CapEnd::Start)),
            worn_face(a, face),
        ),
    );
    (run(&doc, &opts), mate)
}

/// **A NURBS face refuses typed where the placement evaluates**: a
/// lofted flank is a spline patch, which reads as no plane, and the
/// block resting on it refuses with its target's read, in the
/// reader's own voice — never at the door.
///
/// *Breaks if* a face with no plane is read as one, or the door reads
/// a pose.
#[test]
fn a2_a_nurbs_face_refuses_not_planar_where_its_reader_evaluates() {
    // A loft between two squares of different sizes: its flanks are
    // spline patches.
    let doc = ProfileDoc::empty(DocumentId::derive("msolve9-a2-nurbs"), Tol::witness());
    let (doc, lower) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, upper) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.3)],
    );
    let (part, loft) = insert(
        doc,
        Node::Loft {
            first: lower.into(),
            then: vec![editor_core::LoftSection {
                profile: upper.into(),
                station: vec![editor_core::StationMove::Slide {
                    along: editor_core::Axis3::Z,
                    by: len(1.0),
                }],
            }],
            v_degree: Formula::count(1),
        },
    );
    let ev = run(&part, &EvalOptions::default());
    let flank = all_faces(&ev, loft)
        .into_iter()
        .find(|name| face_carrier_kind(&ev, loft, name) == Ok(SurfaceKind::Nurbs))
        .expect("a lofted flank is a spline patch");
    let (part, flank) = world_face(part, loft, &flank);
    let (ev, mate) = rest_on_the_face("msolve9-a2-nurbs-asm", part, &flank);
    let refusal = target_refusal(&ev, mate);
    assert!(
        matches!(
            refusal.kind(),
            NodeErrorKind::PoseRead {
                pose: VarKind::Plane,
                fault: PoseReadFault::NotPlanar { .. },
            }
        ),
        "{refusal:?}"
    );
}

// ---- A tie, and an analysis lane ----

/// **A name the part's table ties refuses where its reader
/// evaluates**: the U cutter's two congruent arms mint one tied face
/// row, so there is no one face to read, and the block resting on it
/// refuses with the selection's own refusal.
///
/// *Breaks if* a tie is broken by picking a fragment.
#[test]
fn a_tied_face_refuses_where_its_reader_evaluates() {
    let (part, sub) = fixture::u_cutter_tie(ProfileDoc::empty(
        DocumentId::derive("msolve9-tie-part"),
        Tol::witness(),
    ));
    let ev = run(&part, &EvalOptions::default());
    let tied = ev
        .table_at(sub)
        .expect("the U subtract evaluates")
        .iter()
        .find_map(|(name, entry)| match entry {
            editor_core::Entry::Tied(_) if name.kind == EntityKind::Face => Some(name.clone()),
            _ => None,
        })
        .expect("the U cutter ties a face");
    let (part, tied) = world_face(part, sub, &tied);
    let (ev, mate) = rest_on_the_face("msolve9-tie", part, &tied);
    let refusal = target_refusal(&ev, mate);
    assert!(
        matches!(refusal.kind(), NodeErrorKind::SelectResolve { .. }),
        "a tied face has no one pose: {refusal:?}"
    );
}

/// **A face read resolves on every lane**: the same seat evaluated at
/// `Dual64` — the scalar `stackup::sensitivities` runs its passes at —
/// and at `Interval` — a certified `clearance`'s — evaluates the
/// placement and places the block, the dual's value channel on the
/// `f64` build's bits and the interval's enclosure around them. The
/// document is evaluated with no box and no seed, so the pose carries
/// no tangent and an enclosure only as wide as rounding.
///
/// *Breaks if* a face read or a value's chart is bound differently on
/// an analysis lane.
#[test]
fn a_face_read_under_an_analysis_evaluation_resolves() {
    use geom_core::Bounds;
    let s = seat("msolve9-dual", 1.0);
    let at_f64 = run(&s.doc, &s.opts);
    assert!(at_f64.node_error(s.mate.placement).is_none());
    let nominal: Vec<_> = editor_core::all_vertices(&at_f64, s.block_i)
        .into_iter()
        .map(|v| {
            let p = editor_core::vertex_position(&at_f64, s.block_i, &v).expect("a vertex");
            (v, [p.x, p.y, p.z])
        })
        .collect();
    let dual = editor_core::evaluate::<geom_core::Dual64>(
        &s.doc,
        None,
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    let interval = editor_core::evaluate::<geom_core::Interval>(
        &s.doc,
        None,
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert!(
        dual.node_error(s.mate.placement).is_none(),
        "{:?}",
        dual.node_error(s.mate.placement)
    );
    assert!(
        interval.node_error(s.mate.placement).is_none(),
        "{:?}",
        interval.node_error(s.mate.placement)
    );
    for (v, at) in nominal {
        let d = editor_core::vertex_position(&dual, s.block_i, &v).expect("placed at Dual64");
        assert_eq!(
            [d.x.value, d.y.value, d.z.value].map(f64::to_bits),
            at.map(f64::to_bits),
            "the dual's value channel is the f64 build's at {v:?}"
        );
        assert_eq!([d.x.deriv, d.y.deriv, d.z.deriv], [0.0; 3]);
        let i = editor_core::vertex_position(&interval, s.block_i, &v).expect("placed");
        for (x, t) in [i.x, i.y, i.z].into_iter().zip(at) {
            assert!(
                x.lo() <= t && t <= x.hi(),
                "the enclosure holds {t} at {v:?}"
            );
        }
    }
}

// ---- The vanished name and the unresolvable part ----

/// **A name the part's table lacks refuses where its reader
/// evaluates, never at the door and never at load** — a seat authored
/// on a face the post has no row for, and a seat whose face a part
/// edit removed: the door admits both, each block refuses with its
/// target's selection refusal, and the document saved after the face
/// vanished loads with no store, bit for bit, and refuses the same way
/// when evaluated.
///
/// *Breaks if* the door or the load door reads a part, or a vanished
/// face resolves to another.
#[test]
fn a_vanished_name_refuses_where_its_reader_evaluates_never_at_the_door_or_load() {
    let s = seat("msolve9-vanished", 1.0);
    let bogus = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, tagged(99)),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (doc, other) = insert(
        s.doc.clone(),
        fixture::mated_instance(match s.doc.node(s.block_i) {
            Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
            _ => panic!("an instance"),
        }),
    );
    let (doc, bogus_mate) = fixture::constrain_at(
        doc,
        other,
        fixture::rest(
            fixture::head_at(other, in_part(other, s.block_body, CapEnd::Start)),
            worn_face(s.post_i, &bogus),
        ),
    );
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            target_refusal(&ev, bogus_mate).kind(),
            NodeErrorKind::SelectResolve { .. }
        ),
        "the part has no such face: {:?}",
        target_refusal(&ev, bogus_mate)
    );
    assert!(
        ev.node_error(s.block_i).is_none(),
        "the well-named seat is untouched"
    );

    // The face vanishes AFTER insert: the post document becomes a
    // revolved round post under the same id — no extrude, so no
    // `Cap(End)` row at all — and the old name is nobody's.
    let mut s = s;
    let post = revolved(
        ProfileDoc::empty(s.post.id(), Tol::witness()),
        vec![vec![(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)]],
    );
    let root = *post.ids().last().expect("the revolve");
    let post = fixture::place(post, root).0;
    let new_ref = s.store.insert(post, Tol::witness());
    s.opts = with_resolver(s.store.clone());
    let (doc, _) = step(
        s.doc.clone(),
        DocEdit::UpdateReference {
            node: s.post_i,
            new_pin: new_ref.pin,
        },
    );
    // The post's own pin is values on the world's axes, reading none of
    // the geometry the extrude had, so what remains unresolved is the
    // seat's face alone.
    let bundle = fixture::bundle(&doc, s.post_i);
    assert_eq!(
        bundle.iter().filter(|c| c.mate().is_some()).count(),
        0,
        "the post is pinned by values alone"
    );
    let ev = run(&doc, &s.opts);
    assert!(
        matches!(
            target_refusal(&ev, s.mate).kind(),
            NodeErrorKind::SelectResolve { .. }
        ),
        "the face is gone: {:?}",
        target_refusal(&ev, s.mate)
    );

    // The load: the document holding the seat on the vanished face
    // loads with no store; what it then evaluates to is its reader's.
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let loaded =
        load(&text, Tol::witness()).expect("a seat on a vanished face loads with no store");
    assert!(loaded.doc.bit_eq(&doc), "the seat as saved");
    let ev = run(&loaded.doc, &s.opts);
    assert!(
        matches!(
            target_refusal(&ev, s.mate).kind(),
            NodeErrorKind::SelectResolve { .. }
        ),
        "the loaded seat refuses where it evaluates: {:?}",
        target_refusal(&ev, s.mate)
    );
}

/// **A part that does not resolve fails its instance in the
/// resolver's own voice**, and the block whose target reads that
/// instance is poisoned through it: the door admits a seat on such a
/// part, since it reads nothing.
///
/// *Breaks if* the door asks a part, or a reader of a failed instance
/// evaluates.
#[test]
fn an_unresolvable_part_fails_its_instance_in_the_resolvers_voice() {
    let s = seat("msolve9-unresolved", 1.0);
    let (doc, other) = insert(
        s.doc.clone(),
        fixture::mated_instance(match s.doc.node(s.block_i) {
            Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
            _ => panic!("an instance"),
        }),
    );
    let seat = fixture::squared(
        &doc,
        fixture::rest(
            fixture::head_at(other, in_part(other, s.block_body, CapEnd::Start)),
            worn_face(
                s.post_i,
                &fixture::resolver::in_world(s.post_body, CapEnd::End),
            ),
        ),
        &[],
    );
    editor_core::apply(&doc, &seat.write(other), Tol::witness())
        .unwrap_or_else(|e| panic!("the door reads no part: {e}"));
    let ev = run(&s.doc, &EvalOptions::default());
    assert!(
        matches!(
            ev.result(s.post_i),
            Some(NodeResult::Failed(e)) if matches!(
                &e.kind,
                NodeErrorKind::Part { fault: PartFault::NoResolver, .. }
            )
        ),
        "the post fails in the resolver's voice: {:?}",
        ev.result(s.post_i)
    );
    assert!(
        matches!(
            ev.result(s.block_i),
            Some(NodeResult::Poisoned { through }) if *through == s.post_i
        ),
        "the seated block is poisoned through the post: {:?}",
        ev.result(s.block_i)
    );
}

// ---- A4: the memo key ----

/// **The seated block's key moves under an edit to the face's part,
/// and holds under an edit outside it.** The block reads the post's
/// copy, and the post's key carries its part's content pin, which
/// over-approximates the face in the direction that re-reads: an edit
/// to the part that moves the face re-keys the block — its value is
/// recomputed and its pose follows — and so does an edit to the part
/// that leaves the face bit for bit where it was (a datum frame
/// inserted into the post), while an edit outside the part leaves the
/// key alone and the block's value comes off the memo.
///
/// *Breaks if* the block's key misses its target's part, or carries
/// the document's.
#[test]
fn a4_the_key_moves_under_an_edit_to_the_faces_part_and_holds_under_one_outside_it() {
    let mut s = seat("msolve9-a4", 1.0);
    let (mate, block_i) = (s.mate.placement, s.block_i);
    let first = run(&s.doc, &s.opts);
    let key = move |ev: &Evaluation<f64>| ev.value(mate).expect("the block evaluates").content_key;
    let reused = move |earlier: &Evaluation<f64>, later: &Evaluation<f64>| match (
        earlier.value(mate),
        later.value(mate),
    ) {
        (Some(a), Some(b)) => Arc::ptr_eq(&a.carry.contacts, &b.carry.contacts),
        _ => false,
    };
    let placed = move |doc: &ProfileDoc, opts: &EvalOptions| {
        solve(doc, opts, Tol::witness())
            .placement(doc, block_i)
            .expect("seated")
    };
    assert!(at_translation(&placed(&s.doc, &s.opts), [0.0, 0.0, 1.0]));

    shorten_or_grow(&mut s, 1.3);
    let second = editor_core::evaluate::<f64>(
        &s.doc,
        Some(&first),
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert_ne!(key(&first), key(&second), "the face moved, the key moved");
    assert!(!reused(&first, &second), "the block's value was recomputed");
    assert!(second.node_error(mate).is_none());
    let grown = placed(&s.doc, &s.opts);
    assert!(
        at_translation(&grown, [0.0, 0.0, 1.3]),
        "the solved pose followed the face: {grown:?}"
    );

    // An edit to the part that leaves the face where it was: a spare
    // profile inserted into the post. The cap's pose is the same bits,
    // and the key moves anyway — the pin is the part's, not the face's.
    let cap_before = cap_pose(&s.post, s.post_body, CapEnd::End);
    edit_the_post(
        &mut s,
        DocEdit::InsertNode {
            node: Box::new(Node::Profile(fixture::desc(vec![square(0.0, 0.0, 0.5)]))),
            fresh: Vec::new(),
        },
    );
    let cap_after = cap_pose(&s.post, s.post_body, CapEnd::End);
    let pose_bits = |p: &topo::readback::Pose<f64>| {
        let u = p.u_ref.expect("a cap fixes its reference");
        [
            p.origin.x, p.origin.y, p.origin.z, p.axis.x, p.axis.y, p.axis.z, u.x, u.y, u.z,
        ]
        .map(f64::to_bits)
    };
    assert_eq!(
        pose_bits(&cap_after),
        pose_bits(&cap_before),
        "the face did not move"
    );
    let third = editor_core::evaluate::<f64>(
        &s.doc,
        Some(&second),
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert_ne!(
        key(&second),
        key(&third),
        "an edit to the face's part moves the key, whether or not the face moved"
    );
    assert!(!reused(&second, &third), "the block's value was recomputed");
    assert_eq!(
        crate::fixture::seat::map_gap(
            &placed(&s.doc, &s.opts).affine::<f64>(),
            &grown.affine::<f64>()
        ),
        0.0,
        "to the same pose"
    );

    // An edit outside the part: a profile inserted beside the
    // assembly's nodes.
    let (doc, _) = insert(
        s.doc.clone(),
        Node::Profile(fixture::desc(vec![square(0.0, 0.0, 0.5)])),
    );
    let fourth = editor_core::evaluate::<f64>(
        &doc,
        Some(&third),
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert_eq!(
        key(&third),
        key(&fourth),
        "an edit outside the part leaves the key"
    );
    assert!(
        reused(&third, &fourth),
        "the block's value came off the memo"
    );
}

// ---- The wire ----

/// **The seat round-trips byte for byte**: a document whose block
/// rests on a face of the post saves, loads with no store as itself,
/// and re-saves to the same bytes.
///
/// *Breaks if* a face read or a value does not survive the wire.
#[test]
fn the_seat_round_trips_byte_for_byte() {
    let s = seat("msolve9-wire", 1.0);
    let text = save(&s.doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads with no store");
    assert_eq!(
        loaded.doc.node(s.block_i),
        s.doc.node(s.block_i),
        "the seat as itself"
    );
    let again = save(&loaded.doc, &[], Tol::witness()).expect("re-saves");
    assert_eq!(again, text, "byte for byte");
}

/// **Every tracked document loads and re-saves byte for byte, and
/// every mate it carries is two poses and a class**: the corpus is on
/// this wire, not an older one, so no reader has an older shape to
/// accept.
///
/// **The mate half is near-vacuous today**: no tracked document holds
/// a mate between copies, so the walk meets only world mates, if any.
/// It is kept for the day one is checked in — a mated document on an
/// older wire would load-fail here, and one on this wire is checked
/// key by key — and the load-and-re-save half runs over every file
/// now. Both suffixes a document is saved under are walked; an older
/// build's bytes kept as refusal evidence refuse with the regenerate
/// recourse, and are held to holding no mate.
///
/// *Breaks if* a tracked document stops loading or re-saving as itself,
/// or carries a mate on another wire.
#[test]
fn c5_every_tracked_document_loads_on_this_wire_and_re_saves_identically() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let listed = std::process::Command::new("git")
        .args(["ls-files", "-z", "--", "*.pncad", "*.cad"])
        .current_dir(&root)
        .output()
        .expect("git lists the tracked files");
    assert!(listed.status.success(), "{listed:?}");
    let walked: Vec<String> = String::from_utf8(listed.stdout)
        .expect("paths are utf-8")
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect();
    assert!(!walked.is_empty(), "a corpus to walk");
    let process = Tol::witness().eps();
    for path in walked.iter().map(|p| root.join(p)) {
        let text = std::fs::read_to_string(&path).expect("readable");
        let loaded = match load(&text, Tol::witness()) {
            Ok(loaded) => loaded,
            Err(PersistError::ToleranceConflict {
                document,
                process: p,
            }) => {
                assert_ne!(document, p);
                assert_eq!(p, process);
                continue;
            }
            // An older build's bytes, kept as evidence of the load
            // door's refusal (`bool13_goldens/`): refused with the
            // regenerate recourse, as every pre-wire file is. None holds
            // a mate, so no mate on an older wire hides behind the
            // refusal.
            Err(e) if e.to_string().contains(REGENERATE_RECOURSE) => {
                assert!(
                    !text.contains("\"Mate\""),
                    "{}: an unloadable file holds a mate: {e}",
                    path.display()
                );
                continue;
            }
            Err(e) => panic!("{}: loads with no store: {e}", path.display()),
        };
        // Every mate the file spells — the snapshot's and any the edit
        // log inserts — by its wire alone.
        let mut seen = 0usize;
        every_mate(&wire::wire_body(&text), &mut |mate| {
            let keys: Vec<_> = mate
                .as_object()
                .map(|o| o.keys().cloned().collect())
                .unwrap_or_default();
            assert!(
                keys == ["on", "orient", "to"],
                "{}: a mate is two poses and an orientation: {mate}",
                path.display()
            );
            seen += 1;
        });
        let mates: usize = loaded
            .doc
            .placements()
            .into_iter()
            .map(|p| {
                fixture::bundle(&loaded.doc, p)
                    .iter()
                    .filter(|c| c.mate().is_some())
                    .count()
            })
            .sum();
        assert!(seen >= mates, "{}: the walk met every mate", path.display());
        let again = save(&loaded.snapshot, &loaded.edits, Tol::witness())
            .unwrap_or_else(|e| panic!("{}: re-saves: {e}", path.display()));
        assert_eq!(again, text, "{}: re-saves byte for byte", path.display());
    }
}

/// Calls `f` on every mate object in a wire body, wherever it sits —
/// a snapshot node or an edit-log entry: the object under a `"mate"`
/// key (a value's `"mate"` names its mate's id, and is no object).
fn every_mate(value: &serde_json::Value, f: &mut dyn FnMut(&serde_json::Value)) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, inner) in map {
                if key == "mate" && inner.is_object() {
                    f(inner);
                }
                every_mate(inner, f);
            }
        }
        serde_json::Value::Array(items) => items.iter().for_each(|v| every_mate(v, f)),
        _ => {}
    }
}
