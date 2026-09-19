//! MSOLVE-8 acceptance — **a levered clash names its arm, the coset's
//! directions carry the witness** (the MSOLVE-8 spec's rows A1–A3).
//!
//! Three of the coset fold's membership margins are a pure number
//! levered by the mate's arm — a sine, a Frobenius departure from the
//! identity, a reachability defect — and a refusal that quoted their
//! product as a bare metre figure hid the arm. These rows reach each
//! of the three through the ordinary doors with a document that trips
//! it, and pin that the refusal carries `Lever::Residual { value, arm }`
//! with `value * arm` the stored clash bit for bit and `value` the
//! number the predicate decided on, re-derived from the fixture's own
//! frames. The other rows pin the witness road: the axis a mate frame
//! decides is the placement's third column bit for bit, and a
//! direction the fold transports by a rotation never refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions, Lever, MateFault,
    MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{insert, len, on_frame, solve, square, step};
use geom_core::linalg::frame::FrameError;
use geom_core::linalg::{Affine3, Mat3, Vec3};
use geom_core::{Point3, Tol};

// ---- Substrate ----

/// A one-solid part: a unit square extruded 1 tall.
fn part(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    doc
}

/// Two instances of one part, plus the options that resolve them.
fn pair(label: &str) -> (ProfileDoc, [RecipeNodeId; 2], EvalOptions) {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part(&format!("{label}-part")), Tol::witness());
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, first) = insert(doc, Node::instantiate_part(doc_ref));
    let (doc, second) = insert(doc, Node::instantiate_part(doc_ref));
    (doc, [first, second], with_resolver(store))
}

fn frame(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference,
    }
}

/// The z-up frame at the origin, referenced along +x.
fn z_up() -> MateFrame {
    frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// The x-along frame at the origin, referenced along +z.
fn x_along() -> MateFrame {
    frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
}

fn alignment(
    primitive: MatePrimitive,
    sense: AxisSense,
    a: MateFrame,
    b: MateFrame,
    clocking: Option<f64>,
) -> Alignment {
    Alignment {
        a,
        b,
        primitive,
        sense,
        clocking,
    }
}

/// A rest-class mate over the two instances' start caps.
fn mate(
    a: RecipeNodeId,
    b: RecipeNodeId,
    alignment: Alignment,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: fixture::head(in_part(a, CapEnd::Start)),
        b: fixture::head(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment,
    }
}

/// Inserts `node`, unwrapping the id every insert here mints.
fn add(doc: ProfileDoc, node: Node<editor_core::ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
    let (doc, id) = step(doc, DocEdit::InsertNode { node });
    (doc, id.expect("the insert minted an id"))
}

/// The two mates a document of two instances declares, in order, and
/// the contradictory refusal the second earns against the first.
fn contradiction(
    label: &str,
    first: Alignment,
    second: Alignment,
) -> (Alignment, Alignment, RecipeNodeId, RecipeNodeId, MateFault) {
    let (doc, [a, b], o) = pair(label);
    let (doc, held) = add(doc, mate(a, b, first));
    let (doc, added) = add(doc, mate(a, b, second));
    let fault = solve(&doc, &o, Tol::witness())
        .fault(added)
        .expect("the pair refuses")
        .clone();
    (first, second, held, added, fault)
}

/// A mate's coset representative as the solve forms it for an aligned
/// sense: `a`'s placement times the inverse of `b`'s.
fn representative(al: &Alignment) -> Affine3<f64> {
    let tol = Tol::witness();
    al.a.placement(tol).unwrap() * al.b.placement(tol).unwrap().inverse()
}

/// The Frobenius norm of `q − I`, spelled as the predicate spells it.
fn frobenius_departure(q: Mat3<f64>) -> f64 {
    let i = Mat3::identity();
    let d = Mat3::from_cols(q.c0 - i.c0, q.c1 - i.c1, q.c2 - i.c2);
    (d.c0.norm_squared() + d.c1.norm_squared() + d.c2.norm_squared()).sqrt()
}

/// The residual lever a contradictory refusal carries, checked for
/// the identity the type states: the halves multiply to the clash bit
/// for bit, and the sentence names the predicate and both halves.
fn residual_of(fault: &MateFault, predicate: &str) -> (f64, f64) {
    let MateFault::Contradictory {
        predicate: named,
        clash,
        lever,
        ..
    } = fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(*named, predicate);
    let Some(Lever::Residual { value, arm }) = *lever else {
        panic!("`{predicate}` levers a pure number, so its refusal carries a residual: {lever:?}");
    };
    assert_eq!(
        (value * arm).to_bits(),
        clash.to_bits(),
        "the stored clash IS the product of the halves: {value} * {arm} vs {clash}"
    );
    assert!(arm > 0.0, "the arm is the mated parts' own extent: {arm}");
    let message = fault.to_string();
    for want in [
        &format!("predicate `{predicate}`"),
        &format!("a dimensionless residual of {value}"),
        &format!("on a {arm} m arm"),
        &format!("a deviation of {} m", value * arm),
    ] {
        assert!(message.contains(want), "{message:?} is missing {want:?}");
    }
    assert!(
        !message.contains(" rad"),
        "a residual is never printed as radians: {message:?}"
    );
    (value, arm)
}

// ---- A1: the three residual margins, each reached through the doors ----

/// **`mate_member_rotation_identity` levers the Frobenius departure.**
/// Two frame coincidences whose `b` frames disagree leave a trivial
/// residual whose candidate rotation is the held mate's; the added
/// mate's membership then measures `‖Q − I‖_F` for the relative
/// rotation `Q`, levered by the arm. The value the refusal carries is
/// that norm, re-derived here from the two representatives the frames
/// denote — a half turn here (`x_along · z_up⁻¹` carries `x` to `z`,
/// `z` to `x` and `y` to `−y`), whose departure is `2√2`.
#[test]
fn a_rotation_identity_clash_carries_the_frobenius_departure_and_its_arm() {
    let (first, second, held, added, fault) = contradiction(
        "msolve8-rotation-identity",
        alignment(
            MatePrimitive::FrameCoincidence,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            None,
        ),
        alignment(
            MatePrimitive::FrameCoincidence,
            AxisSense::Aligned,
            z_up(),
            x_along(),
            None,
        ),
    );
    let (value, _) = residual_of(&fault, "mate_member_rotation_identity");
    let MateFault::Contradictory {
        held: h, added: a, ..
    } = &fault
    else {
        unreachable!()
    };
    assert_eq!((*h, *a), (held, added));
    // The candidate keeps the held representative's rotation, and the
    // relative rotation the added membership measures is that against
    // the added representative's.
    let q = representative(&first).linear * representative(&second).linear.inverse();
    assert_eq!(
        value.to_bits(),
        frobenius_departure(q).to_bits(),
        "the residual is the Frobenius departure the predicate decided on: {value} vs {}",
        frobenius_departure(q)
    );
    assert!(
        (value - 2.0 * core::f64::consts::SQRT_2).abs() < 1e-12,
        "a half turn departs by 2√2: {value}"
    );
}

/// **`mate_member_axis_fixed` levers `‖Q·n − n‖`.** A frame coincidence
/// holds the pair rigid; a planar rest added on a `b` frame turned a
/// quarter round then asks its normal to be fixed by a rotation that
/// carries it onto another axis. The value is the length of that
/// departure, re-derived from the normal the frame ladder decided.
#[test]
fn an_axis_fixed_clash_carries_the_normals_departure_and_its_arm() {
    let (first, second, _, _, fault) = contradiction(
        "msolve8-axis-fixed",
        alignment(
            MatePrimitive::FrameCoincidence,
            AxisSense::Aligned,
            z_up(),
            z_up(),
            None,
        ),
        alignment(
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Aligned,
            z_up(),
            x_along(),
            None,
        ),
    );
    let (value, _) = residual_of(&fault, "mate_member_axis_fixed");
    let q = representative(&first).linear * representative(&second).linear.inverse();
    let n = second.a.axis(Tol::witness()).unwrap().get();
    assert_eq!(
        value.to_bits(),
        (q * n - n).norm().to_bits(),
        "the residual is the normal's departure under the relative rotation: {value}"
    );
    assert!(
        (value - core::f64::consts::SQRT_2).abs() < 1e-12,
        "a normal carried onto a perpendicular axis departs by √2: {value}"
    );
}

/// **`mate_rotation_two_axis_reachable` levers the reach.** A planar
/// rest and a coaxial mate whose axes stand at a right angle on the
/// `a` side and coincide on the `b` side admit no rotation: the
/// reachability defect is the difference of the two axes' inner
/// product across the sides, re-derived here from the frames' own
/// witnesses and the representatives' rotations.
#[test]
fn a_two_axis_reach_clash_carries_the_reach_and_its_arm() {
    let (first, second, _, _, fault) = contradiction(
        "msolve8-two-axis-reach",
        alignment(
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Aligned,
            z_up(),
            z_up(),
            None,
        ),
        alignment(
            MatePrimitive::Coaxial,
            AxisSense::Aligned,
            x_along(),
            z_up(),
            None,
        ),
    );
    let (value, _) = residual_of(&fault, "mate_rotation_two_axis_reachable");
    let tol = Tol::witness();
    let a1 = first.a.axis(tol).unwrap().get();
    let a2 = second.a.axis(tol).unwrap().get();
    let (q1, q2) = (
        representative(&first).linear,
        representative(&second).linear,
    );
    let v = (q1 * q2.transpose()) * a2;
    let reach = v.dot(a1) - a2.dot(a1);
    assert_eq!(
        value.to_bits(),
        reach.to_bits(),
        "the residual is the reach the predicate decided on: {value} vs {reach}"
    );
    assert!(
        (value - 1.0).abs() < 1e-12,
        "the sides disagree by a full cosine: {value}"
    );
}

// ---- A1, the arm: the same three under the reversed authored order ----

/// **A transported direction never refuses, and the clash it reaches
/// is levered.** The spanning tree reads the pair from the gauge, so
/// a mate authored `(second, first)` is INVERTED before the fold —
/// its directions transported by the representative's rotation and
/// re-minted under the band. The three documents above, authored the
/// other way round, each refuse CONTRADICTORY with a residual lever
/// and never escalate. Which predicate fires is the fold's own
/// business and not always the direct order's: inverted, the third
/// document's coaxial axis is carried onto the rest's normal, so the
/// pair meets on the parallel branch and the added membership's
/// `axis_fixed` is what measures the disagreement.
#[test]
fn the_three_residual_clashes_survive_the_inverted_authored_order() {
    let rows: [(&str, &str, Alignment, Alignment); 3] = [
        (
            "msolve8-inverted-rotation-identity",
            "mate_member_rotation_identity",
            alignment(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                None,
            ),
            alignment(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up(),
                x_along(),
                None,
            ),
        ),
        (
            "msolve8-inverted-axis-fixed",
            "mate_member_axis_fixed",
            alignment(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up(),
                z_up(),
                None,
            ),
            alignment(
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Aligned,
                z_up(),
                x_along(),
                None,
            ),
        ),
        (
            "msolve8-inverted-two-axis-reach",
            "mate_member_axis_fixed",
            alignment(
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Aligned,
                z_up(),
                z_up(),
                None,
            ),
            alignment(
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                x_along(),
                z_up(),
                None,
            ),
        ),
    ];
    for (label, predicate, first, second) in rows {
        let (doc, [a, b], o) = pair(label);
        let (doc, _) = add(doc, mate(b, a, first));
        let (doc, added) = add(doc, mate(b, a, second));
        let fault = solve(&doc, &o, Tol::witness())
            .fault(added)
            .expect("the pair refuses")
            .clone();
        assert!(
            !matches!(fault, MateFault::Indeterminate { .. }),
            "{label}: a direction transported by a proper rotation never refuses: {fault}"
        );
        residual_of(&fault, predicate);
    }
}

// ---- A2: the witness is the placement's column, bit for bit ----

/// Mate frames of every shape the mate suites author: the axis-aligned
/// ones, the opposed ones, offset origins, a tilt inside the band,
/// and a sweep of oblique aims and references.
fn frame_corpus() -> Vec<MateFrame> {
    let eps = Tol::witness().get().eps;
    let mut v = vec![
        z_up(),
        x_along(),
        frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
        frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        frame([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        frame([0.0, 0.0, 0.0], [3.0 * eps, 0.0, 1.0], [0.0, 1.0, 0.0]),
        frame([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        frame([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        frame([1e-9, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 3.0, 0.0]),
        frame([0.25, -0.5, 0.75], [3.0, -4.0, 12.0], [0.0, 1.0, 0.0]),
        frame([-0.0, 3.5, -2.0], [0.0, 0.0, 1.0], [-1.0, 0.0, 0.0]),
        frame([1e6, -1e-6, 0.0], [1.0, 1.0, 0.0], [-1.0, 1.0, 2.0]),
    ];
    for k in 0..32 {
        let t = f64::from(k) * 0.19;
        v.push(frame(
            [t, -t, t * t],
            [t.cos(), t.sin(), 0.3 * t],
            [-t.sin(), t.cos(), 1.0],
        ));
    }
    v
}

/// **The axis witness IS the placement's third column.** The one
/// decision the ladder makes for its aim, asked again through the
/// same door, yields the same bits — which is what lets the solve
/// read its directions as witnesses while every other reader's affine
/// is untouched. Pinned over the corpus at both ε rows the band can
/// take, since the decision is the band's.
#[test]
fn the_axis_witness_is_the_placements_third_column_bit_for_bit() {
    let tol = Tol::witness();
    for f in frame_corpus() {
        let placement = f.placement(tol).expect("a frame in the corpus places");
        let axis = f.axis(tol).expect("and its axis decides").get();
        let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
        assert_eq!(
            bits(axis),
            bits(placement.linear.c2),
            "at {f:?}: witness {axis:?} vs column {:?}",
            placement.linear.c2
        );
        // The witness is what the column was: the ladder's own
        // normalize, not a re-normalize of the column.
        let eye = Point3::new(f.origin[0], f.origin[1], f.origin[2]);
        let raw = Vec3::new(f.axis[0], f.axis[1], f.axis[2]);
        assert_eq!(
            bits(axis),
            bits(((eye + raw) - eye).normalize()),
            "at {f:?}"
        );
    }
}

/// **The two doors refuse together, with one projection.** An aim
/// the ladder refuses is refused by the axis door with the same
/// `FrameError`, arm for arm — a zero axis, one that underflows the
/// norm, one that overflows it — so a caller that asks both never
/// meets a frame with a placement and no axis, or the reverse.
#[test]
fn the_axis_door_refuses_exactly_as_the_placement_does() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    for axis in [
        [0.0, 0.0, 0.0],
        [1e-200, 0.0, 0.0],
        [0.0, 1e200, 0.0],
        [eps * 0.5, 0.0, 0.0],
    ] {
        let f = frame([0.0, 0.0, 0.0], axis, [1.0, 0.0, 0.0]);
        let placed: Result<(), FrameError> = f.placement(tol).map(|_| ());
        let decided: Result<(), FrameError> = f.axis(tol).map(|_| ());
        assert_eq!(
            decided, placed,
            "at axis {axis:?} the two doors state one refusal"
        );
        assert!(
            decided.is_err(),
            "at axis {axis:?} the aim has no direction"
        );
    }
    // A reference on the aim line refuses the PLACEMENT (the roll) and
    // not the axis: the axis is decided before the roll is asked.
    let f = frame([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 2.0]);
    assert!(f.placement(tol).is_err());
    assert!(f.axis(tol).is_ok());
}

// ---- A3: verdicts that must not move under the witness ----

/// **A determined pair still determines, authored either way round.**
/// The A11 rule-1 exemplar (coaxial plus rest plus clocking) solves
/// with the mates on `(first, second)` and again on `(second,
/// first)`, where every direction is transported and re-minted — and
/// the second document stands the rest the other way up, the same
/// distance, because the rest's `a` plane now belongs to the other
/// instance. The V-block (two planar rests at a right angle, the
/// planes' line minted from the levered cross product) still leaves
/// the prismatic residual along that line.
#[test]
fn a_determined_pair_and_a_v_block_keep_their_verdicts_under_the_witness() {
    let mut poses_seen = Vec::new();
    for (label, reversed) in [
        ("msolve8-determined", false),
        ("msolve8-determined-rev", true),
    ] {
        let (doc, [first, second], o) = pair(label);
        let (a, b) = if reversed {
            (second, first)
        } else {
            (first, second)
        };
        let (doc, _) = add(
            doc,
            mate(
                a,
                b,
                alignment(
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    z_up(),
                    z_up(),
                    Some(0.0),
                ),
            ),
        );
        let (doc, rest) = add(
            doc,
            mate(
                a,
                b,
                alignment(
                    MatePrimitive::PlanarRest { offset: -1.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                    frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                    None,
                ),
            ),
        );
        let poses = solve(&doc, &o, Tol::witness());
        assert!(
            poses.fault(rest).is_none() && poses.fault(second).is_none(),
            "{label}: the exemplar is DETERMINED: {:?}",
            poses.fault(rest)
        );
        poses_seen.push(
            poses
                .relative(second)
                .expect("the second instance is placed"),
        );
    }
    // The gauge is the document-order-first instance either way. The
    // direct document seats the second instance two units up the
    // shared axis; the inverted one seats the FIRST on the second, so
    // the second sits two units down — the mirror, not a moved pose.
    let [direct, inverted] = poses_seen.as_slice() else {
        unreachable!()
    };
    let close = |a: &[f64], b: &[f64]| a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-12);
    assert!(
        direct
            .columns
            .iter()
            .zip(&inverted.columns)
            .all(|(a, b)| close(a, b))
            && close(&direct.translation, &[0.0, 0.0, 2.0])
            && close(&inverted.translation, &[0.0, 0.0, -2.0]),
        "the two authored orders seat b two units apart along the axis, each way up: \
         {direct:?} vs {inverted:?}"
    );
    let (doc, [first, second], o) = pair("msolve8-v-block");
    let mut doc = doc;
    for axis in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let (next, _) = add(
            doc,
            mate(
                first,
                second,
                alignment(
                    MatePrimitive::PlanarRest { offset: 0.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                    frame([0.0, 0.0, 0.0], axis, [0.0, 0.0, 1.0]),
                    None,
                ),
            ),
        );
        doc = next;
    }
    let fault = solve(&doc, &o, Tol::witness())
        .fault(second)
        .expect("the V-block is UNDER")
        .clone();
    let MateFault::Under { residual, .. } = &fault else {
        panic!("expected UNDER, got {fault:?}");
    };
    assert_eq!(residual.name(), "prismatic");
    assert!(
        fault.to_string().contains("translation along [0, 0, 1]"),
        "the planes' line is minted from the levered cross product: {fault}"
    );
}
