//! MSOLVE-9 acceptance — **a mate frame that names a face of the part
//! and resolves at evaluation** through the reach road (the spec is
//! `docs/MSOLVE-9-SPEC.md`; `ASSEMBLY.md` A11 rule 5 is the ratified
//! sentence).
//!
//! `MateFrame::FromFace` stores the PART-LOCAL name of a face; the
//! solve asks the mated part's own evaluation for that face's
//! canonical pose (`MateReach::face_pose`, off the cached product in
//! the part's own coordinates) and reads the frame from it — origin,
//! chart axis, and the carrier's own roll reference — through the
//! same witness ladder authored vectors meet. Nothing is stored twice:
//! edit the part so the face moves, and the mate follows.
//!
//! The rows go through ordinary doors with a `PartStore`: the item's
//! own document (a block seated on a post's cap, the post's height
//! edited, the mate following by exactly the height change); every
//! analytic carrier resolving to `face_pose`'s frame bit for bit with
//! the sense bit left out; a NURBS face refusing typed; the reference
//! rule; a vanished name and an unresolvable part refusing in the
//! resolver's own voice at the door, at evaluation and never at load;
//! the memo key moving iff the face's part moves; the wire round-trip
//! of both arms and the stray-key refusal; and every tracked
//! document loading with its frames read as authored and re-saving
//! byte for byte.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;

use std::sync::Arc;

use editor_core::mate::SurfaceKind;
use editor_core::{
    Alignment, AuthoredFrame, AxisSense, CancelToken, CapEnd, ContactClass, DocEdit, DocumentId,
    EditError, EntityKind, EvalOptions, Evaluation, Expr, FaceName, FacePoseRefusal, FaceRefusal,
    Frame, LoggedEdit, LoopProgram, MateFault, MateFrame, MatePrimitive, MateSide, Node,
    NodeErrorKind, PartFault, PersistError, ProfileDoc, ProfileProgram, RecipeNodeId,
    RefusingReach, RoleSeg, SitedFace, SlotId, StableName, all_faces, face_carrier_kind,
    face_frame, load, mate_reach, save,
};
use fixture::resolver::{PART_BODY, PartStore, in_part, with_resolver};
use fixture::{
    at_the_door, gate, insert, len, on_frame, on_frame_keeping, run, solve, square, step, step_with,
};
use geom_core::Tol;

// ---- Substrate ----

/// A square block of half-side `half` centred on the origin, extruded
/// `height` along +z: the extrude is `PART_BODY`, and its cap faces are
/// the part's own rows at `RoleSeg::Cap(..)`.
fn block(label: &str, half: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, half)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(height),
        },
    );
    doc
}

/// The PART-LOCAL name of a block's cap face: the row the part's own
/// table holds, with no instance wrapped round it.
fn cap(end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: PART_BODY,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// A frame that names `local`, a face of the part, with no authored
/// reference.
fn from_face(local: &StableName) -> MateFrame {
    MateFrame::from_face(FaceName::new(local.clone()).expect("a face"), None)
}

/// The identity frame, authored: origin at the part's origin, +z, +x.
fn identity() -> MateFrame {
    MateFrame::authored([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// A frame coincidence of `a` and `b`, axes aligned, no rider.
fn coincide(a: MateFrame, b: MateFrame) -> Alignment {
    Alignment {
        a,
        b,
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Aligned,
        clocking: None,
    }
}

/// The mate `a` (its `End` cap) to `b` (its `Start` cap), at
/// `alignment`.
fn mate(a: RecipeNodeId, b: RecipeNodeId, alignment: Alignment) -> Node<ProfileProgram> {
    Node::Mate {
        a: fixture::head(in_part(a, CapEnd::End)),
        b: fixture::head(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment,
    }
}

/// **The item's document**: a post and a block in a store, the block
/// instantiated after the post (so the post is the gauge), and the
/// block SEATED on the post's cap — the post side names the cap FACE,
/// the block side is its own origin frame. Returns the assembly, the
/// two instances, the mate, the store's options and the post document
/// as stored.
struct Seat {
    doc: ProfileDoc,
    post_i: RecipeNodeId,
    block_i: RecipeNodeId,
    mate: RecipeNodeId,
    opts: EvalOptions,
    store: PartStore,
    post: ProfileDoc,
}

fn seat(label: &str, post_height: f64) -> Seat {
    let mut store = PartStore::new();
    let post = block(&format!("{label}-post"), 0.5, post_height);
    let post_ref = store.insert(post.clone(), Tol::witness());
    let block_ref = store.insert(block(&format!("{label}-block"), 0.5, 0.25), Tol::witness());
    let opts = with_resolver(store.clone());
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, post_i) = insert(doc, Node::instantiate_part(post_ref));
    let (doc, block_i) = insert(doc, Node::instantiate_part(block_ref));
    let (doc, mate) = step_with(
        doc,
        DocEdit::InsertNode {
            node: mate(
                post_i,
                block_i,
                coincide(from_face(&cap(CapEnd::End)), identity()),
            ),
        },
        &reach,
    );
    Seat {
        doc,
        post_i,
        block_i,
        mate: mate.expect("the mate is minted"),
        opts,
        store,
        post,
    }
}

/// The post's cap pose, read off the post document's OWN evaluation
/// through the name door — the oracle every resolution is held to.
fn cap_pose(post: &ProfileDoc, end: CapEnd) -> topo::readback::Pose<f64> {
    let ev = run(post, &EvalOptions::default());
    face_frame(&ev, PART_BODY, &cap(end)).expect("the cap has a pose")
}

/// **The relative pose a frame coincidence on `pose` solves to** when
/// the other side is [`identity`]: the pose's origin, its CHART axis
/// and its own reference through the witness ladder — the frame the
/// side resolves to — times the inverse of the identity side's own
/// witness, exactly as the coset table forms its representative
/// (`a`'s placement, `b`'s inverse), so the comparison is bit for
/// bit.
fn resolved(pose: &topo::readback::Pose<f64>) -> Frame {
    let u_ref = pose.u_ref.expect("the carrier fixes a reference");
    let authored = AuthoredFrame {
        origin: [pose.origin.x, pose.origin.y, pose.origin.z],
        axis: [pose.axis.x, pose.axis.y, pose.axis.z],
        reference: [u_ref.x, u_ref.y, u_ref.z],
    };
    let fa = authored
        .placement(Tol::witness())
        .expect("a definite frame");
    let fb = identity()
        .authored_vectors()
        .expect("authored")
        .placement(Tol::witness())
        .expect("a definite frame");
    Frame::from_affine(fa * fb.inverse())
}

fn bits(frame: &Frame) -> Vec<u64> {
    frame
        .columns
        .iter()
        .flatten()
        .chain(frame.translation.iter())
        .map(|x| x.to_bits())
        .collect()
}

/// The mate's fault as the evaluation records it.
fn mate_fault(ev: &Evaluation<f64>, mate: RecipeNodeId) -> MateFault {
    match &ev.node_error(mate).expect("the mate faulted").kind {
        NodeErrorKind::Mate(fault) => (**fault).clone(),
        other => panic!("a mate's own refusal: {other}"),
    }
}

/// The post re-stored at `height`, and the assembly re-pinned to it:
/// the ordinary part-edit path — the part document edited through
/// its own door, the store holding the new version, the reference
/// moved by `UpdateReference` through the store's reach.
fn shorten_or_grow(s: &mut Seat, height: f64) {
    let (post, _) = step(
        s.post.clone(),
        DocEdit::SetParam {
            node: PART_BODY,
            slot: SlotId::Distance,
            expr: len(height),
        },
    );
    let new_ref = s.store.insert(post.clone(), Tol::witness());
    s.post = post;
    s.opts = with_resolver(s.store.clone());
    let reach = mate_reach::<f64>(&s.opts, Tol::witness());
    let (doc, _) = step_with(
        s.doc.clone(),
        DocEdit::UpdateReference {
            node: s.post_i,
            new_pin: new_ref.pin,
        },
        &reach,
    );
    s.doc = doc;
}

// ---- A1: the item's document ----

/// **Edit the part, and the mate follows.** The block's solved pose
/// is the post's cap pose — origin at the cap's centre, `z` at the
/// post's height — before and after the post is grown, by exactly
/// the height change and bit-exactly where the arithmetic is exact
/// (the cap plane's origin IS the height); the gate certifies the
/// seat both times, and the saved document holds the face name and
/// no vectors for that side.
#[test]
fn a1_the_mate_follows_the_edited_face() {
    let mut s = seat("msolve9-a1", 1.0);
    let before = cap_pose(&s.post, CapEnd::End);
    assert_eq!(
        before.origin.z.to_bits(),
        1.0_f64.to_bits(),
        "the cap is at the height"
    );
    let placed = solve(&s.doc, &s.opts, Tol::witness())
        .placement(&s.doc, s.block_i)
        .expect("the block is seated");
    assert_eq!(
        bits(&placed),
        bits(&resolved(&before)),
        "seated on the cap, bit for bit"
    );
    let ev = run(&s.doc, &s.opts);
    assert!(
        ev.node_error(s.mate).is_none(),
        "{:?}",
        ev.node_error(s.mate)
    );
    assert!(
        gate(&s.doc, &ev).is_ok(),
        "the seat certifies: {:?}",
        gate(&s.doc, &ev).err()
    );

    shorten_or_grow(&mut s, 1.3);
    let after = cap_pose(&s.post, CapEnd::End);
    assert_eq!(
        after.origin.z.to_bits(),
        1.3_f64.to_bits(),
        "the cap moved with the height"
    );
    let placed = solve(&s.doc, &s.opts, Tol::witness())
        .placement(&s.doc, s.block_i)
        .expect("the block is still seated");
    assert_eq!(
        bits(&placed),
        bits(&resolved(&after)),
        "seated on the moved cap, bit for bit"
    );
    assert_eq!(
        placed.translation[2].to_bits(),
        1.3_f64.to_bits(),
        "the block came up by exactly the height change"
    );
    let ev = run(&s.doc, &s.opts);
    assert!(
        ev.node_error(s.mate).is_none(),
        "{:?}",
        ev.node_error(s.mate)
    );
    assert!(
        gate(&s.doc, &ev).is_ok(),
        "no refutation: the mate followed the face: {:?}",
        gate(&s.doc, &ev).err()
    );

    // Nothing stored twice: the side that names the face carries the
    // name and no vectors.
    let text = save(&s.doc, &[], Tol::witness()).expect("saves");
    let body = wire::wire_body(&text);
    let node = &body["snapshot"]["nodes"][s.mate.0.to_string()];
    let a = &node["Mate"]["alignment"]["a"];
    assert!(a.get("face").is_some(), "the face name is the state: {a}");
    assert!(
        a.get("origin").is_none() && a.get("axis").is_none(),
        "no vectors beside it: {a}"
    );
    let b = &node["Mate"]["alignment"]["b"];
    assert!(
        b.get("origin").is_some() && b.get("face").is_none(),
        "the authored side: {b}"
    );
}

// ---- A2: every analytic carrier, and the sense bit ----

/// A part whose product carries a face of `wanted`'s kind, built
/// through the ordinary doors, with that face's part-local name and
/// its pose off the part's own evaluation.
fn carrier(
    label: &str,
    wanted: SurfaceKind,
) -> (ProfileDoc, StableName, topo::readback::Pose<f64>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let doc = match wanted {
        SurfaceKind::Plane => block(label, 0.5, 1.0),
        // A rectangle revolved a full turn: a cylinder wall between
        // two planar ends.
        SurfaceKind::Cylinder => revolved(
            doc,
            vec![vec![(0.0, 0.0), (0.4, 0.0), (0.4, 1.0), (0.0, 1.0)]],
        ),
        // A triangle revolved: a cone flank on a planar base.
        SurfaceKind::Cone => revolved(doc, vec![vec![(0.0, 0.0), (0.5, 0.0), (0.0, 0.8)]]),
        // A half disc revolved: a sphere.
        SurfaceKind::Sphere => revolved_program(doc, crate::corpus::die_pips::half_disc_program()),
        // A circle off the axis revolved: a torus.
        SurfaceKind::Torus => revolved_program(doc, circle_program(0.6, 0.2)),
        other => panic!("no analytic fixture for {other:?}"),
    };
    let ev = run(&doc, &EvalOptions::default());
    let root = *doc.roots().first().expect("a product root");
    let name = all_faces(&ev, root)
        .into_iter()
        .find(|name| face_carrier_kind(&ev, root, name) == Ok(wanted))
        .unwrap_or_else(|| panic!("{label}: a {wanted:?} face"));
    let pose = face_frame(&ev, root, &name).expect("the carrier has a pose");
    (doc, name, pose)
}

/// `loops` on the xz plane revolved a full turn about the plane's y
/// axis (the world z).
fn revolved(doc: ProfileDoc, loops: Vec<Vec<(f64, f64)>>) -> ProfileDoc {
    let (doc, plane, profile) =
        on_frame_keeping(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], loops);
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, _) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: fixture::ang(std::f64::consts::TAU),
        },
    );
    doc
}

/// A loop PROGRAM on the xz plane revolved a full turn about its y
/// axis.
fn revolved_program(doc: ProfileDoc, program: LoopProgram) -> ProfileDoc {
    let (doc, plane) = insert(
        doc,
        fixture::frame([0.0; 3], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let (doc, axis) = insert(doc, fixture::axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![program],
        }),
    );
    let (doc, _) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: fixture::ang(std::f64::consts::TAU),
        },
    );
    doc
}

/// A full circle of radius `r` centred `big` off the revolve axis: the
/// seamless closed carrier.
fn circle_program(big: f64, r: f64) -> LoopProgram {
    let length = |v: f64| Expr::literal(v, editor_core::Dimension::Length).unwrap();
    LoopProgram::Circle {
        centre: [length(big), length(0.0)],
        radius: length(r),
    }
}

/// **A face of `part` as side `a`, an identity frame as side `b`**:
/// the block's relative pose IS the resolved frame. Returns the
/// solved relative frame, or the mate's fault.
fn resolve_through_the_solve(
    label: &str,
    part: ProfileDoc,
    frame: MateFrame,
    sense: AxisSense,
) -> Result<Frame, MateFault> {
    let mut store = PartStore::new();
    let part_ref = store.insert(part, Tol::witness());
    let block_ref = store.insert(block(&format!("{label}-block"), 0.5, 0.25), Tol::witness());
    let opts = with_resolver(store);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(part_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(block_ref));
    let node = Node::Mate {
        a: fixture::head(in_part(a, CapEnd::End)),
        b: fixture::head(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment: Alignment {
            sense,
            ..coincide(frame, identity())
        },
    };
    let (doc, mate) = at_the_door(&doc, &reach, node).map_err(|(_, fault)| fault)?;
    let poses = solve(&doc, &opts, Tol::witness());
    match poses.fault(mate) {
        Some(fault) => Err(fault.clone()),
        None => Ok(poses.relative(b).expect("the block is placed")),
    }
}

/// **The reach's `face_pose` is the part's own `face_frame`, bit for
/// bit, on every analytic carrier**: the door the solve reads through
/// (`PartReach::face_pose`, off the cached product) answers exactly
/// what the name door answers on the part's own evaluation — origin,
/// chart axis, reference and sense.
#[test]
fn a2_the_reachs_face_pose_is_the_parts_own_face_frame_bit_for_bit() {
    for (kind, label) in [
        (SurfaceKind::Plane, "msolve9-a2-door-plane"),
        (SurfaceKind::Cylinder, "msolve9-a2-door-cylinder"),
        (SurfaceKind::Cone, "msolve9-a2-door-cone"),
        (SurfaceKind::Sphere, "msolve9-a2-door-sphere"),
        (SurfaceKind::Torus, "msolve9-a2-door-torus"),
    ] {
        let (part, name, pose) = carrier(label, kind);
        let mut store = PartStore::new();
        let part_ref = store.insert(part, Tol::witness());
        let store: Arc<dyn editor_core::PartResolver> = Arc::new(store);
        let reach = editor_core::PartReach::<f64>::with_resolver(Some(&store), Tol::witness());
        let got = editor_core::MateReach::face_pose(
            &reach,
            &part_ref,
            &FaceName::new(name).expect("a face"),
        )
        .unwrap_or_else(|refusal| panic!("{kind:?}: the reach answers: {refusal:?}"));
        let p = |v: geom_core::Point3<f64>| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
        let d = |v: geom_core::Vec3<f64>| [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()];
        assert_eq!(p(got.origin), p(pose.origin), "{kind:?}: origin");
        assert_eq!(d(got.axis), d(pose.axis), "{kind:?}: axis");
        assert_eq!(
            got.u_ref.map(d),
            pose.u_ref.map(d),
            "{kind:?}: reference — got {:?}, standalone {:?}",
            got.u_ref,
            pose.u_ref
        );
        assert_eq!(got.sense, pose.sense, "{kind:?}: sense");
    }
}

/// **Every analytic carrier resolves to `face_pose`'s frame bit for
/// bit**: a plane, a cylinder, a cone, a sphere and a torus, each read
/// off its part's own evaluation by name and compared with the solved
/// relative pose of a block whose own frame is the identity.
#[test]
fn a2_every_analytic_carrier_resolves_to_face_pose_bit_for_bit() {
    for (kind, label) in [
        (SurfaceKind::Plane, "msolve9-a2-plane"),
        (SurfaceKind::Cylinder, "msolve9-a2-cylinder"),
        (SurfaceKind::Cone, "msolve9-a2-cone"),
        (SurfaceKind::Sphere, "msolve9-a2-sphere"),
        (SurfaceKind::Torus, "msolve9-a2-torus"),
    ] {
        let (part, name, pose) = carrier(label, kind);
        assert!(
            pose.u_ref.is_some(),
            "{kind:?}: the carrier fixes its own reference"
        );
        let got = resolve_through_the_solve(label, part, from_face(&name), AxisSense::Aligned)
            .unwrap_or_else(|fault| panic!("{kind:?} resolves: {fault}"));
        assert_eq!(
            bits(&got),
            bits(&resolved(&pose)),
            "{kind:?}: the pose's frame, bit for bit"
        );
    }
}

/// **The sense bit is not folded into the axis**: a face whose
/// outward normal is `-axis` (`sense: false`) resolves to the CHART
/// axis, and the mate's `AxisSense` alone turns the pair round.
#[test]
fn a2_the_sense_bit_is_not_folded_and_axis_sense_alone_decides() {
    // A revolved rectangle's two annuli are minted with opposite
    // senses: one is an inward wall, whose outward normal is the
    // chart's negation.
    let part = revolved(
        ProfileDoc::empty(DocumentId::derive("msolve9-a2-sense"), Tol::witness()),
        vec![vec![(0.0, 0.0), (0.4, 0.0), (0.4, 1.0), (0.0, 1.0)]],
    );
    let ev = run(&part, &EvalOptions::default());
    let root = *part.roots().first().expect("a product root");
    let (name, pose) = all_faces(&ev, root)
        .into_iter()
        .filter_map(|name| {
            let pose = face_frame(&ev, root, &name).ok()?;
            (face_carrier_kind(&ev, root, &name) == Ok(SurfaceKind::Plane) && !pose.sense)
                .then_some((name, pose))
        })
        .next()
        .expect("a planar face whose outward normal is -axis");
    let aligned = resolve_through_the_solve(
        "msolve9-a2-sense-aligned",
        part.clone(),
        from_face(&name),
        AxisSense::Aligned,
    )
    .expect("resolves");
    assert_eq!(
        bits(&aligned),
        bits(&resolved(&pose)),
        "the CHART axis, sense left out"
    );
    let z = |f: &Frame| f.columns[2];
    assert_eq!(z(&aligned), [pose.axis.x, pose.axis.y, pose.axis.z]);
    let opposed = resolve_through_the_solve(
        "msolve9-a2-sense-opposed",
        part,
        from_face(&name),
        AxisSense::Opposed,
    )
    .expect("resolves");
    assert_eq!(
        z(&opposed),
        [-pose.axis.x, -pose.axis.y, -pose.axis.z],
        "the mate's own sense turns the pair round"
    );
}

/// **A NURBS face refuses typed at the mate, the side, the instance
/// and the part**, in the readback's own voice, at the insert door —
/// and keeps taking authored vectors.
#[test]
fn a2_a_nurbs_face_refuses_no_canonical_frame_typed() {
    // A loft between two squares of different sizes: its flanks are
    // spline patches with no canonical frame.
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
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.3)],
    );
    let (part, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![lower, upper],
            v_degree: Expr::count(1),
        },
    );
    let ev = run(&part, &EvalOptions::default());
    let flank = all_faces(&ev, loft)
        .into_iter()
        .find(|name| face_carrier_kind(&ev, loft, name) == Ok(SurfaceKind::Nurbs))
        .expect("a lofted flank is a spline patch");
    let fault = resolve_through_the_solve(
        "msolve9-a2-nurbs-asm",
        part.clone(),
        from_face(&flank),
        AxisSense::Aligned,
    )
    .expect_err("a NURBS face has no canonical frame");
    let MateFault::FaceUnresolved {
        side: MateSide::A,
        refusal,
        ..
    } = &fault
    else {
        panic!("expected FaceUnresolved, got {fault:?}");
    };
    let FaceRefusal::Readback {
        error: topo::readback::ReadbackError::NoCanonicalFrame { carrier },
        face,
        ..
    } = refusal.as_ref()
    else {
        panic!("expected Readback(NoCanonicalFrame), got {refusal:?}");
    };
    assert_eq!(*carrier, "nurbs surface");
    assert_eq!(**face, flank);
    assert!(fault.to_string().contains("no canonical frame"), "{fault}");
    // The same face, as authored vectors: admitted.
    resolve_through_the_solve(
        "msolve9-a2-nurbs-authored",
        part,
        MateFrame::authored([0.0, 0.0, 0.5], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        AxisSense::Aligned,
    )
    .expect("authored vectors keep working");
}

// ---- The reference rule ----

/// **One reference, from one source**: a carrier that fixes its own
/// in-frame reference resolves with none authored and refuses an
/// authored one beside it (`ReferenceRefused`). Every analytic carrier
/// the readback answers fixes one today, so `NoReference` is pinned
/// at its wrap — the readback's contract admits a pose with none, and
/// no door produces it.
#[test]
fn a_carried_reference_beside_an_authored_one_refuses_and_no_reference_is_pinned() {
    let (part, name, pose) = carrier("msolve9-ref-sphere", SurfaceKind::Sphere);
    assert!(pose.u_ref.is_some(), "a sphere fixes its own reference");
    resolve_through_the_solve(
        "msolve9-ref-none",
        part.clone(),
        from_face(&name),
        AxisSense::Aligned,
    )
    .expect("resolves with the carrier's own reference");
    let doubled = MateFrame::from_face(
        FaceName::new(name.clone()).expect("a face"),
        Some([1.0, 0.0, 0.0]),
    );
    let fault = resolve_through_the_solve("msolve9-ref-twice", part, doubled, AxisSense::Aligned)
        .expect_err("one fact spelled twice");
    let MateFault::FaceUnresolved {
        side: MateSide::A,
        refusal,
        ..
    } = &fault
    else {
        panic!("expected FaceUnresolved, got {fault:?}");
    };
    assert!(
        matches!(refusal.as_ref(), FaceRefusal::ReferenceRefused { face, .. } if **face == name),
        "{fault:?}"
    );
    let instance = RecipeNodeId(3);
    let part_ref = editor_core::DocRef {
        id: DocumentId::derive("msolve9-ref-pin"),
        pin: editor_core::ContentPin([9u8; 32]),
    };
    let face = FaceName::new(name).expect("a face");
    assert_eq!(
        FaceRefusal::of(
            FacePoseRefusal::NoReference,
            instance,
            part_ref,
            face.clone()
        ),
        FaceRefusal::NoReference {
            instance,
            part: part_ref,
            face,
        }
    );
}

// ---- The vanished name and the unresolvable part ----

/// **A name the part's table lacks refuses `NoSuchName`** naming the
/// mate, the side, the instance, the part and the face — at the
/// insert door for a mate authored so, and at EVALUATION for a mate
/// whose face a part edit removed, which a load never refuses: the
/// logged insert replays with no store (the face is declined, not
/// read) and the next solve decides it.
#[test]
fn a_vanished_name_refuses_no_such_name_at_the_door_and_at_evaluation_never_at_load() {
    let s = seat("msolve9-vanished", 1.0);
    let reach = mate_reach::<f64>(&s.opts, Tol::witness());
    let bogus = StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(99),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let (named, fault) = at_the_door(
        &s.doc,
        &reach,
        mate(s.post_i, s.block_i, coincide(from_face(&bogus), identity())),
    )
    .expect_err("the part has no such face");
    let MateFault::FaceUnresolved {
        mate: m,
        side: MateSide::A,
        refusal,
    } = &fault
    else {
        panic!("expected FaceUnresolved, got {fault:?}");
    };
    let FaceRefusal::NoSuchName {
        instance,
        part,
        face,
    } = refusal.as_ref()
    else {
        panic!("expected NoSuchName, got {refusal:?}");
    };
    assert_eq!(*m, named);
    assert_eq!(*instance, s.post_i);
    assert_eq!(**face, bogus);
    let Some(Node::InstantiatePart { doc_ref, .. }) = s.doc.node(s.post_i) else {
        panic!("an instance");
    };
    assert_eq!(part, doc_ref);
    assert!(
        fault.to_string().contains("face name minted by node 99"),
        "the badge names the face: {fault}"
    );

    // The face vanishes AFTER insert: the post document becomes a
    // revolved round post under the same id — no extrude, so no
    // `Cap(End)` row at all — and the old name is nobody's. The
    // mate's state is the solve's at evaluation.
    let mut s = s;
    let post = revolved(
        ProfileDoc::empty(s.post.id(), Tol::witness()),
        vec![vec![(0.0, 0.0), (0.5, 0.0), (0.5, 1.0), (0.0, 1.0)]],
    );
    let new_ref = s.store.insert(post, Tol::witness());
    s.opts = with_resolver(s.store.clone());
    let reach = mate_reach::<f64>(&s.opts, Tol::witness());
    let (doc, _) = step_with(
        s.doc.clone(),
        DocEdit::UpdateReference {
            node: s.post_i,
            new_pin: new_ref.pin,
        },
        &reach,
    );
    let ev = run(&doc, &s.opts);
    let fault = mate_fault(&ev, s.mate);
    let MateFault::FaceUnresolved {
        side: MateSide::A,
        refusal,
        ..
    } = &fault
    else {
        panic!("expected FaceUnresolved, got {fault:?}");
    };
    assert!(
        matches!(refusal.as_ref(), FaceRefusal::NoSuchName { instance, .. } if *instance == s.post_i),
        "{fault:?}"
    );
    assert!(
        ev.node_error(s.block_i).is_some(),
        "the fault poisons the cluster"
    );

    // The load: the logged insert replays with no store and the
    // document loads; what it then evaluates to is the solve's.
    let log = vec![LoggedEdit::bare(DocEdit::InsertNode {
        node: s.doc.node(s.mate).expect("the mate").clone(),
    })];
    // Deleting the mate splits the cluster, and the block's new gauge
    // needs the prior solved frame — through the store's reach, since
    // a `FromFace` side is resolved from the part.
    let (snapshot, _) = step_with(doc.clone(), DocEdit::DeleteNode { id: s.mate }, &reach);
    let text = save(&snapshot, &log, Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("a FromFace insert replays with no store");
    let replayed = *loaded.doc.order().last().expect("the replayed mate");
    assert_eq!(
        loaded.doc.node(replayed),
        doc.node(s.mate),
        "the mate as logged"
    );
    let ev = run(&loaded.doc, &s.opts);
    let replayed_fault = mate_fault(&ev, replayed);
    assert!(
        matches!(
            &replayed_fault,
            MateFault::FaceUnresolved { refusal, .. }
                if matches!(refusal.as_ref(), FaceRefusal::NoSuchName { .. })
        ),
        "the next solve decides the declined face: {replayed_fault:?}"
    );
}

/// **A part that does not resolve faults the mate in the resolver's
/// own voice** — at the door through the refusing reach, and at
/// evaluation under no resolver — before any lever is asked.
#[test]
fn an_unresolvable_part_faults_in_the_resolvers_voice() {
    let s = seat("msolve9-unresolved", 1.0);
    let (_, fault) = at_the_door(
        &s.doc,
        &RefusingReach,
        mate(
            s.post_i,
            s.block_i,
            coincide(from_face(&cap(CapEnd::End)), identity()),
        ),
    )
    .expect_err("no resolver, no face");
    let no_resolver = |fault: &MateFault, instance_named: Option<RecipeNodeId>| {
        matches!(
            fault,
            MateFault::FaceUnresolved { side: MateSide::A, refusal, .. }
                if matches!(
                    refusal.as_ref(),
                    FaceRefusal::PartUnresolved { instance, fault: PartFault::NoResolver }
                        if instance_named.is_none_or(|named| *instance == named)
                )
        )
    };
    assert!(no_resolver(&fault, Some(s.post_i)), "{fault:?}");
    let ev = run(&s.doc, &EvalOptions::default());
    let at_evaluation = mate_fault(&ev, s.mate);
    assert!(
        no_resolver(&at_evaluation, Some(s.post_i)),
        "the evaluation's own voice: {at_evaluation}"
    );
}

// ---- A4: the memo key ----

/// **The mate's key moves iff the face's part moves**: an edit to the
/// part that moves the face re-keys the mate — its value is
/// recomputed, not served, and the block's pose follows — while an
/// edit elsewhere in the assembly leaves the key alone and the mate
/// comes off the memo.
#[test]
fn a4_the_key_moves_with_the_face_and_holds_otherwise() {
    let mut s = seat("msolve9-a4", 1.0);
    let (mate, block_i) = (s.mate, s.block_i);
    let first = run(&s.doc, &s.opts);
    let key = move |ev: &Evaluation<f64>| ev.value(mate).expect("the mate evaluates").content_key;
    let reused = move |earlier: &Evaluation<f64>, later: &Evaluation<f64>| match (
        earlier.value(mate),
        later.value(mate),
    ) {
        (Some(a), Some(b)) => Arc::ptr_eq(&a.name_table, &b.name_table),
        _ => false,
    };
    let placed_z = move |doc: &ProfileDoc, opts: &EvalOptions| {
        solve(doc, opts, Tol::witness())
            .placement(doc, block_i)
            .expect("seated")
            .translation[2]
    };
    assert_eq!(placed_z(&s.doc, &s.opts), 1.0);

    shorten_or_grow(&mut s, 1.3);
    let second = editor_core::evaluate::<f64>(
        &s.doc,
        Some(&first),
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert_ne!(key(&first), key(&second), "the face moved, the key moved");
    assert!(!reused(&first, &second), "the mate's value was recomputed");
    assert!(second.node_error(mate).is_none());
    assert_eq!(
        placed_z(&s.doc, &s.opts),
        1.3,
        "the solved pose followed the face"
    );

    // An edit that leaves the part alone: a datum inserted beside the
    // assembly's nodes.
    let (doc, _) = insert(
        s.doc.clone(),
        fixture::frame([0.0, 0.0, 5.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let third = editor_core::evaluate::<f64>(
        &doc,
        Some(&second),
        &CancelToken::new(),
        &s.opts,
        Tol::witness(),
    );
    assert_eq!(
        key(&second),
        key(&third),
        "the face did not move, the key did not"
    );
    assert!(reused(&second, &third), "the mate came off the memo");
}

// ---- The wire ----

/// **Both arms round-trip, and a stray key on either refuses at
/// load**: a document carrying a `FromFace` side and an `Authored`
/// side saves and loads as itself, and a key neither inner struct has
/// refuses the file as unreadable.
#[test]
fn both_arms_round_trip_and_a_stray_key_on_either_refuses() {
    let s = seat("msolve9-wire", 1.0);
    let text = save(&s.doc, &[], Tol::witness()).expect("saves");
    let loaded = load(&text, Tol::witness()).expect("loads with no store");
    assert_eq!(
        loaded.doc.node(s.mate),
        s.doc.node(s.mate),
        "both arms as themselves"
    );
    let again = save(&loaded.doc, &[], Tol::witness()).expect("re-saves");
    assert_eq!(again, text, "byte for byte");
    for side in ["a", "b"] {
        let doctored = wire::doctored(&text, |wire| {
            wire["snapshot"]["nodes"][s.mate.0.to_string()]["Mate"]["alignment"][side]["stray"] =
                serde_json::json!(1);
        });
        let err = load(&doctored, Tol::witness()).expect_err("a stray key refuses");
        assert!(
            matches!(err, PersistError::Unreadable { .. }),
            "side {side}: this build's types refuse the bytes, got {err:?}"
        );
    }
}

/// **Every tracked document loads with its mate frames read as
/// authored and re-saves byte for byte**: the untagged wire reads
/// three vectors as `Authored` unchanged, so C5 holds over the corpus.
#[test]
fn c5_every_tracked_document_reads_its_frames_as_authored_and_re_saves_identically() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let listed = std::process::Command::new("git")
        .args(["ls-files", "-z", "--", "*.pncad"])
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
            Err(e) => panic!("{}: loads with no store: {e}", path.display()),
        };
        for node in loaded
            .doc
            .order()
            .iter()
            .filter_map(|&id| loaded.doc.node(id))
        {
            if let Node::Mate { alignment, .. } = node {
                assert!(
                    alignment.a.authored_vectors().is_some()
                        && alignment.b.authored_vectors().is_some(),
                    "{}: a tracked mate's frames read as authored",
                    path.display()
                );
            }
        }
        let again = save(&loaded.snapshot, &loaded.edits, Tol::witness())
            .unwrap_or_else(|e| panic!("{}: re-saves: {e}", path.display()));
        assert_eq!(again, text, "{}: re-saves byte for byte", path.display());
    }
}

/// The two edit doors' shared finiteness rule reaches a face frame's
/// authored reference and nothing else of it: a non-finite reference
/// refuses `NonFiniteAlignment`, and a face frame with none is finite
/// whatever its face resolves to.
#[test]
fn a_face_frames_reference_is_the_only_number_the_finiteness_door_sees() {
    let s = seat("msolve9-finite", 1.0);
    let poisoned = MateFrame::from_face(
        FaceName::new(cap(CapEnd::End)).expect("a face"),
        Some([f64::NAN, 0.0, 0.0]),
    );
    let err = s
        .doc
        .apply(
            &DocEdit::InsertNode {
                node: mate(s.post_i, s.block_i, coincide(poisoned, identity())),
            },
            Tol::witness(),
            &RefusingReach,
        )
        .expect_err("a non-finite reference refuses");
    assert!(
        matches!(err, EditError::NonFiniteAlignment { .. }),
        "{err:?}"
    );
    assert!(coincide(from_face(&cap(CapEnd::End)), identity()).is_finite());
}

/// A `SitedFace` is what a head is; this row pins that a frame's face
/// is the PART's row and a head's is the instance's wrapper of it —
/// two spellings of one face, at two doors.
#[test]
fn the_frames_face_is_the_parts_row_under_the_heads_wrapper() {
    let s = seat("msolve9-spelling", 1.0);
    let Some(Node::Mate { a, alignment, .. }) = s.doc.node(s.mate) else {
        panic!("the mate");
    };
    let head: &SitedFace = a;
    let [RoleSeg::InPart { of }] = head.name.path.as_slice() else {
        panic!("a head is the instance's wrapper");
    };
    let face = alignment.a.face().expect("a face frame");
    assert_eq!(**of, *face.face.as_ref(), "the same row, unwrapped");
    assert_eq!(head.name.node, s.post_i);
}
