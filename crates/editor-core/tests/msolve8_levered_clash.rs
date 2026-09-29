//! MSOLVE-8 acceptance — **a levered clash names its arm, the coset's
//! directions carry the witness** (the MSOLVE-8 spec's rows A1–A3 and
//! its review claims C1–C4).
//!
//! Three of the coset fold's membership margins are a pure number
//! levered by the mate's arm — a sine, a Frobenius departure from the
//! identity, a reachability defect — and a refusal that quoted their
//! product as a bare metre figure hid the arm. The `c1` rows reach
//! each of the three through the ordinary doors with a document that
//! trips it and pin that the refusal carries
//! `Clash::Levered(Lever::Residual { value, arm })` with `value` the
//! number the predicate decided on, re-derived from the fixture's own
//! frames, and `arm` the lever the FOLD decided it over. The `c2` rows
//! pin the witness road: the axis a mate frame decides is the
//! placement's third column and the two doors refuse together; the
//! parallel verdict and the line it mints are one decision at every
//! ulp around the band's edge; a direction the fold transports never
//! refuses. The `c4` rows measure the sentence on `MateFault` about
//! its two mate-less arms. Every row runs at `Tol::witness()` unless
//! it commits a tolerance of its own.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::mate::coset::{Coset, FoldStop, Subgroup, intersect, intersect_subgroups};
use editor_core::{
    Alignment, AxisSense, CapEnd, Clash, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EvalOptions, Lever, MateFault, MateFrame, MatePrimitive, MateReach, Node, ProfileDoc,
    RecipeNodeId, mate_reach,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{FIXTURE_MATE_AXIS, at_the_door, insert, len, on_frame, run, solve, square, step};
use geom_core::k_stats::{Bracket, Recorded};
use geom_core::linalg::{Affine3, Mat3, UnitVec3, Vec3};
use geom_core::predicate::Band;
use geom_core::{Tol, Tolerance};

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

/// `n` instances of one part, the options that resolve them, and the
/// reference the reach is asked through.
struct Rig {
    doc: ProfileDoc,
    ids: Vec<RecipeNodeId>,
    o: EvalOptions,
    doc_ref: DocRef,
}

fn rig(label: &str, n: usize) -> Rig {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part(&format!("{label}-part")), Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    Rig {
        doc,
        ids,
        o: with_resolver(store),
        doc_ref,
    }
}

fn frame(origin: [f64; 3], axis: [f64; 3], reference: [f64; 3]) -> MateFrame {
    MateFrame {
        origin,
        axis,
        reference,
    }
}

/// The z-up frame at `o`, referenced along +x.
fn z_up_at(o: [f64; 3]) -> MateFrame {
    frame(o, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// The x-along frame at `o`, referenced along +z.
fn x_along_at(o: [f64; 3]) -> MateFrame {
    frame(o, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
}

fn al(
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

/// The part's reach, through the evaluation's own door.
fn reach_of(r: &Rig) -> f64 {
    mate_reach::<f64>(&r.o, Tol::witness())
        .reach(&r.doc_ref)
        .expect("the part reaches")
}

/// The mate's lever as the solve forms it: both parts' reach plus the
/// datum's own terms.
fn lever_of(r: &Rig, a: &Alignment) -> f64 {
    reach_of(r) + reach_of(r) + a.lever_arm()
}

/// The representative `mate_coset` forms for an aligned sense: the
/// primitive's own displacement ridden onto `a`'s placement, times the
/// inverse of `b`'s.
fn representative(a: &Alignment) -> Affine3<f64> {
    let tol = Tol::witness();
    let fa = a.a.placement(tol).unwrap();
    let fb = a.b.placement(tol).unwrap();
    let target = match a.primitive {
        MatePrimitive::PlanarRest { offset } => {
            fa * Affine3::translation(Vec3::new(0.0, 0.0, 1.0) * offset)
        }
        MatePrimitive::FrameCoincidence | MatePrimitive::Coaxial | MatePrimitive::Clocking => fa,
    };
    target * fb.inverse()
}

/// The Frobenius norm of `q − I`, spelled as the predicate spells it.
fn frobenius_departure(q: Mat3<f64>) -> f64 {
    let i = Mat3::identity();
    let d = Mat3::from_cols(q.c0 - i.c0, q.c1 - i.c1, q.c2 - i.c2);
    (d.c0.norm_squared() + d.c1.norm_squared() + d.c2.norm_squared()).sqrt()
}

fn bits3(v: Vec3<f64>) -> [u64; 3] {
    [v.x.to_bits(), v.y.to_bits(), v.z.to_bits()]
}

/// Two mates on one instance pair, authored `(first, second)` or the
/// other way round, and the added mate's fault.
fn two_mates(
    label: &str,
    first: Alignment,
    second: Alignment,
    reversed: bool,
) -> (Rig, RecipeNodeId, RecipeNodeId, Option<(Site, MateFault)>) {
    let r = rig(label, 2);
    let (a, b) = if reversed {
        (r.ids[1], r.ids[0])
    } else {
        (r.ids[0], r.ids[1])
    };
    let (doc, held) = add(r.doc, mate(a, b, first));
    // The second mate meets its own admission at the door and the
    // pair's verdict at the solve: whichever refuses is the fault.
    let (doc, added, fault) = match at_the_door(
        &doc,
        &mate_reach::<f64>(&r.o, Tol::witness()),
        mate(a, b, second),
    ) {
        Ok((doc, added)) => {
            let fault = solve(&doc, &r.o, Tol::witness())
                .fault(added)
                .cloned()
                .map(|fault| (Site::Solve, fault));
            (doc, added, fault)
        }
        Err((added, fault)) => (doc, added, Some((Site::Door, fault))),
    };
    (Rig { doc, ..r }, held, added, fault)
}

/// **Which door a mate's refusal came out of** — the insert door,
/// which decides the mate's own datum, or the solve, which decides
/// the pair. A row pins the site beside the fault, so a refusal that
/// moved between them cannot pass as the same fault.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Site {
    Door,
    Solve,
}

/// The residual lever a contradictory refusal carries, with the
/// sentence checked to name the predicate and both halves and to call
/// the number what it is.
fn residual_of(fault: &MateFault, predicate: &str) -> (f64, f64) {
    let MateFault::Contradictory {
        predicate: named,
        clash,
        ..
    } = fault
    else {
        panic!("expected CONTRADICTORY, got {fault:?}");
    };
    assert_eq!(*named, predicate);
    let Clash::Levered(Lever::Residual { value, arm }) = *clash else {
        panic!("`{predicate}` levers a pure number, so its refusal carries a residual: {clash:?}");
    };
    assert!(arm > 0.0, "the arm is the mated parts' own extent: {arm}");
    let message = fault.to_string();
    for want in [
        &format!("predicate `{predicate}`"),
        &format!("a dimensionless residual of {value}"),
        &format!("on a {arm} m arm"),
        &format!("a deviation of {} m", clash.deviation().unwrap()),
    ] {
        assert!(message.contains(want), "{message:?} is missing {want:?}");
    }
    assert!(
        !message.contains(" rad"),
        "a residual is never printed as radians: {message:?}"
    );
    (value, arm)
}

// ---- A1 / C1: the three residual predicates, value and arm ----

/// **`mate_member_rotation_identity` levers the Frobenius departure,
/// over the FOLD's arm.** Two frame coincidences whose `b` frames
/// disagree leave a trivial residual whose candidate rotation is the
/// held mate's; the added mate's membership measures `‖Q − I‖_F` for
/// the relative rotation `Q`. The first mate carries the larger datum
/// (origins two and three units out), so the arm the refusal names is
/// the first mate's lever and not the added mate's own.
#[test]
fn c1_rotation_identity_value_and_arm() {
    let first = al(
        MatePrimitive::FrameCoincidence,
        AxisSense::Aligned,
        z_up_at([2.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 3.0]),
        None,
    );
    let second = al(
        MatePrimitive::FrameCoincidence,
        AxisSense::Aligned,
        z_up_at([0.3, 0.0, 0.0]),
        x_along_at([0.0, 0.4, 0.0]),
        None,
    );
    let (r, held, added, fault) = two_mates("msolve8-c1-rot-id", first, second, false);
    let (site, fault) = fault.expect("the pair refuses");
    assert_eq!(site, Site::Solve, "a verdict about the pair is the solve's");
    let (value, arm) = residual_of(&fault, "mate_member_rotation_identity");
    let q = representative(&first).linear * representative(&second).linear.inverse();
    assert_eq!(
        value.to_bits(),
        frobenius_departure(q).to_bits(),
        "the residual is the Frobenius departure the predicate decided on: {value}"
    );
    assert_eq!(
        arm.to_bits(),
        lever_of(&r, &first).to_bits(),
        "the fold's arm is the largest of the mates' levers, the first mate's here"
    );
    assert!(lever_of(&r, &first) > lever_of(&r, &second));
    let MateFault::Contradictory {
        held: h, added: a, ..
    } = &fault
    else {
        unreachable!()
    };
    assert_eq!((*h, *a), (held, added));
}

/// **`mate_member_axis_fixed` levers `‖Q·n − n‖`.** A frame
/// coincidence holds the pair rigid; a planar rest added on a `b`
/// frame turned a quarter round asks its normal to be fixed by a
/// rotation that carries it onto another axis. The value is that
/// departure, re-derived from the normal the frame ladder decided;
/// the arm is the added mate's own, because its authored standoff
/// makes its lever the pair's largest.
#[test]
fn c1_axis_fixed_value_and_arm() {
    let first = al(
        MatePrimitive::FrameCoincidence,
        AxisSense::Aligned,
        z_up_at([0.1, 0.2, 0.0]),
        z_up_at([0.0, 0.0, 0.5]),
        None,
    );
    let second = al(
        MatePrimitive::PlanarRest { offset: -0.75 },
        AxisSense::Aligned,
        z_up_at([0.1, 0.2, 0.0]),
        x_along_at([0.0, 0.0, 0.5]),
        None,
    );
    let (r, _, _, fault) = two_mates("msolve8-c1-axis-fixed", first, second, false);
    let (site, fault) = fault.expect("the pair refuses");
    assert_eq!(site, Site::Solve, "a verdict about the pair is the solve's");
    let (value, arm) = residual_of(&fault, "mate_member_axis_fixed");
    let q = representative(&first).linear * representative(&second).linear.inverse();
    let n = second.a.axis(Tol::witness()).unwrap().get();
    assert_eq!(
        value.to_bits(),
        (q * n - n).norm().to_bits(),
        "the residual is the normal's departure under the relative rotation: {value}"
    );
    assert!(lever_of(&r, &second) > lever_of(&r, &first));
    assert_eq!(arm.to_bits(), lever_of(&r, &second).to_bits());
}

/// **`mate_rotation_two_axis_reachable` levers the reach.** A planar
/// rest and a coaxial mate whose axes stand at a right angle on the
/// `a` side and coincide on the `b` side admit no rotation: the
/// reachability defect is the difference of the two axes' inner
/// product across the sides, re-derived from the frames' own
/// witnesses and the representatives' rotations.
#[test]
fn c1_two_axis_reach_value_and_arm() {
    let first = al(
        MatePrimitive::PlanarRest { offset: 0.5 },
        AxisSense::Aligned,
        z_up_at([0.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let second = al(
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        x_along_at([1.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let (r, _, _, fault) = two_mates("msolve8-c1-reach", first, second, false);
    let (site, fault) = fault.expect("the pair refuses");
    assert_eq!(site, Site::Solve, "a verdict about the pair is the solve's");
    let (value, arm) = residual_of(&fault, "mate_rotation_two_axis_reachable");
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
    assert_eq!(
        arm.to_bits(),
        lever_of(&r, &first).max(lever_of(&r, &second)).to_bits()
    );
}

/// **A length predicate measures a length and a roll is decided over
/// the mate's OWN arm.** Two planar rests a standoff apart refuse on
/// `translation_in_plane` with a `Clash::Length`; two coaxial mates
/// half a unit apart refuse on a `mate_member_` length; a
/// self-contradictory rider beside a mate with a five-unit datum
/// refuses with a `Roll` whose arm is the rider's own lever — the
/// rider is decided in its own coset, before the fold's maximum is
/// formed — and whose product is the deviation.
#[test]
fn c1_length_none_and_roll_arm() {
    let f1 = al(
        MatePrimitive::PlanarRest { offset: 0.0 },
        AxisSense::Aligned,
        z_up_at([0.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let f2 = al(
        MatePrimitive::PlanarRest { offset: 0.5 },
        AxisSense::Aligned,
        z_up_at([0.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let (_, _, _, fault) = two_mates("msolve8-c1-len-plane", f1, f2, false);
    let (site, fault) = fault.expect("the pair refuses");
    assert_eq!(site, Site::Solve, "a verdict about the pair is the solve's");
    let MateFault::Contradictory {
        predicate, clash, ..
    } = &fault
    else {
        panic!("{fault:?}")
    };
    assert_eq!(*predicate, "mate_member_translation_in_plane");
    let Clash::Length { metres } = *clash else {
        panic!("a translation predicate measures a LENGTH outright: {clash:?}");
    };
    assert!((metres.abs() - 0.5).abs() < 1e-12, "{metres}");
    assert!(fault.to_string().contains("a clash of"), "{fault}");

    let c1 = al(
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        z_up_at([0.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let c2 = al(
        MatePrimitive::Coaxial,
        AxisSense::Aligned,
        z_up_at([0.5, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let (_, _, _, fault) = two_mates("msolve8-c1-len-axis", c1, c2, false);
    let (site, fault) = fault.expect("the pair refuses");
    assert_eq!(site, Site::Solve, "a verdict about the pair is the solve's");
    let MateFault::Contradictory {
        predicate, clash, ..
    } = &fault
    else {
        panic!("{fault:?}")
    };
    assert!(predicate.starts_with("mate_member_"), "{predicate}");
    assert!(matches!(clash, Clash::Length { .. }), "{clash:?}");

    let big = al(
        MatePrimitive::FrameCoincidence,
        AxisSense::Aligned,
        z_up_at([5.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        None,
    );
    let rider = al(
        MatePrimitive::FrameCoincidence,
        AxisSense::Aligned,
        z_up_at([0.0, 0.0, 0.0]),
        z_up_at([0.0, 0.0, 0.0]),
        Some(0.3),
    );
    let (r, _, _, fault) = two_mates("msolve8-c1-roll", big, rider, false);
    let (site, fault) = fault.expect("the rider refuses");
    assert_eq!(
        site,
        Site::Door,
        "the rider is decided in its own coset, at the insert door"
    );
    let MateFault::Contradictory {
        predicate, clash, ..
    } = &fault
    else {
        panic!("{fault:?}")
    };
    assert_eq!(*predicate, "mate_clocking_redundant");
    let Clash::Levered(Lever::Roll { radians, arm }) = *clash else {
        panic!("{clash:?}")
    };
    assert_eq!(radians.to_bits(), 0.3_f64.to_bits());
    assert_eq!(
        arm.to_bits(),
        lever_of(&r, &rider).to_bits(),
        "the roll is decided over the rider's own lever"
    );
    assert!(
        lever_of(&r, &big) > arm,
        "the fold's larger arm is not the roll's"
    );
    assert_eq!(clash.deviation(), Some(radians * arm));
    assert!(
        fault
            .to_string()
            .contains(&format!("a roll of {radians} rad on a {arm} m arm")),
        "{fault}"
    );
}

/// **A transported direction never refuses, and the clash it reaches
/// is levered.** The spanning tree reads the pair from the gauge, so
/// a mate authored `(second, first)` is INVERTED before the fold —
/// its directions transported by the representative's rotation and
/// re-minted under the band. The three documents above, authored the
/// other way round, each refuse CONTRADICTORY with a residual lever
/// and never a frame refusal. Which predicate fires is the fold's own
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
            al(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up_at([0.0; 3]),
                z_up_at([0.0; 3]),
                None,
            ),
            al(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up_at([0.0; 3]),
                x_along_at([0.0; 3]),
                None,
            ),
        ),
        (
            "msolve8-inverted-axis-fixed",
            "mate_member_axis_fixed",
            al(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up_at([0.0; 3]),
                z_up_at([0.0; 3]),
                None,
            ),
            al(
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Aligned,
                z_up_at([0.0; 3]),
                x_along_at([0.0; 3]),
                None,
            ),
        ),
        (
            "msolve8-inverted-two-axis-reach",
            "mate_member_axis_fixed",
            al(
                MatePrimitive::PlanarRest { offset: 0.0 },
                AxisSense::Aligned,
                z_up_at([0.0; 3]),
                z_up_at([0.0; 3]),
                None,
            ),
            al(
                MatePrimitive::Coaxial,
                AxisSense::Aligned,
                x_along_at([0.0; 3]),
                z_up_at([0.0; 3]),
                None,
            ),
        ),
    ];
    for (label, predicate, first, second) in rows {
        let (_, _, _, fault) = two_mates(label, first, second, true);
        let (site, fault) = fault.expect("the pair refuses");
        assert_eq!(
            site,
            Site::Solve,
            "{label}: a verdict about the pair is the solve's"
        );
        assert!(
            !matches!(
                fault,
                MateFault::Frame { .. } | MateFault::Indeterminate { .. }
            ),
            "{label}: a direction transported by a proper rotation never refuses: {fault}"
        );
        residual_of(&fault, predicate);
    }
}

// ---- A2 / C2: the witness is the column, and one decision at the band's edge ----

/// **The axis witness IS the placement's third column, and the two
/// doors refuse together.** Over every combination of a 22-value
/// axis grid (signed units, halves, in-band and sub-band lengths,
/// underflowing and overflowing magnitudes, the non-finite values),
/// six references and four origins — 255 552 frames — `axis(tol)` is
/// `placement(tol).linear.c2` bit for bit where both place, and the
/// two refuse with one `FrameError` where both refuse. There is no
/// third case: the doors are one construction, so a frame whose roll
/// refuses has no axis either, and none places without one.
#[test]
fn c2_axis_vs_placement_sweep() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let vals = [
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        -0.3,
        1e-3,
        3.0 * eps,
        eps,
        0.5 * eps,
        -eps,
        1e-160,
        1e-200,
        1e154,
        1e200,
        1e308,
        -1e308,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN_POSITIVE,
        5e-324,
    ];
    let refs = [
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1e-12],
        [0.0, 0.0, 2.0],
        [-0.0, 3e-9, 1.0],
    ];
    let origins = [
        [0.0, 0.0, 0.0],
        [1e6, -1e6, 1e6],
        [1e-6, 1e15, -1e-300],
        [-0.0, -0.0, -0.0],
    ];
    let (mut both_ok, mut both_err) = (0_usize, 0_usize);
    for x in vals {
        for y in vals {
            for z in vals {
                for r in refs {
                    for o in origins {
                        let f = frame(o, [x, y, z], r);
                        match (f.placement(tol), f.axis(tol)) {
                            (Ok(p), Ok(a)) => {
                                both_ok += 1;
                                assert_eq!(bits3(p.linear.c2), bits3(a.get()), "{f:?}");
                            }
                            (Err(p), Err(a)) => {
                                both_err += 1;
                                assert_eq!(p, a, "{f:?}");
                            }
                            (Err(e), Ok(_)) => {
                                panic!("{f:?}: the placement refused but the axis decided: {e:?}")
                            }
                            (Ok(_), Err(a)) => panic!("{f:?}: placed without an axis: {a:?}"),
                        }
                    }
                }
            }
        }
    }
    assert_eq!(both_ok + both_err, 22 * 22 * 22 * 6 * 4);
    assert!(both_ok > 0 && both_err > 0, "{both_ok} {both_err}");
}

/// The in-plane perturbation shapes a boundary search tilts a normal
/// by.
const SHAPES: [(f64, f64); 6] = [
    (1.0, 0.0),
    (0.0, 1.0),
    (0.6, 0.8),
    (
        -core::f64::consts::FRAC_1_SQRT_2,
        core::f64::consts::FRAC_1_SQRT_2,
    ),
    (0.3, -0.9),
    (0.28, 0.96),
];

/// `n1` tilted by `s` along `shape` in the plane perpendicular to it,
/// as a witness.
fn tilted(n1: Vec3<f64>, shape: (f64, f64), s: f64, band: Band) -> Option<UnitVec3<f64>> {
    let (b1, b2) = n1.orthonormal_basis();
    UnitVec3::new(
        n1 + b1 * (shape.0 * s) + b2 * (shape.1 * s),
        FIXTURE_MATE_AXIS,
        band,
    )
    .ok()
}

/// The tilt at which the levered sine crosses the band's escalate
/// threshold, by bisection, so a sweep of the ulps around it straddles
/// the edge.
fn boundary_tilt(n1: Vec3<f64>, shape: (f64, f64), arm: f64, band: Band) -> f64 {
    let escalate = band.escalate();
    let (mut lo, mut hi) = (escalate / arm * 0.25, escalate / arm * 4.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            break;
        }
        match tilted(n1, shape, mid, band).map(|n2| (n1.cross(n2.get()) * arm).norm()) {
            Some(l) if l >= escalate => hi = mid,
            _ => lo = mid,
        }
    }
    hi
}

/// The verdict the ONE spelling of the parallel margin gives — the
/// levered vector's norm against the band — for two witnesses at an
/// arm.
fn one_spelling(n1: Vec3<f64>, n2: Vec3<f64>, arm: f64, band: Band) -> &'static str {
    let m = (n1.cross(n2) * arm).norm();
    if m <= band.zero() {
        "parallel"
    } else if m >= band.escalate() {
        "line"
    } else {
        "escalates"
    }
}

/// **The parallel verdict and the line it mints are ONE decision.**
/// Two planar subgroups whose normals sit within ±400 ulps of the
/// tilt at which the levered sine crosses K·ε, over four normals,
/// seven arms and six tilt shapes (134 400 pairs): the table's answer
/// — the planar residual, the prismatic line, or the escalation under
/// `mate_axes_parallel` — is exactly the classification of
/// `‖(n1 × n2) · arm‖` against the band at every ulp, and the line
/// is that vector's own normalize. Spelled as two decisions (the sine
/// times the arm decided, then the levered vector minted) the two
/// lengths differ by up to two ulps and straddle the edge; spelled
/// once they cannot, and no escalation names any other predicate.
///
/// The SUBGROUP half is the subject, because the full coset door adds
/// a stage this row is not about: for a pair the table calls a line,
/// two planes this nearly parallel make the translation system
/// singular and the candidate's membership refuses a non-finite
/// margin — a refusal that names `mate_member_translation_in_plane`
/// for a cause that is the conditioning of the system, filed on
/// MSOLVE's slate as a false-cause refusal of its own.
#[test]
fn c2_parallel_boundary_direct() {
    let band = Band::linear(Tol::witness()).unwrap();
    let unit = |v: Vec3<f64>| UnitVec3::new(v, FIXTURE_MATE_AXIS, band).unwrap();
    let (mut total, mut escalations, mut lines, mut planes) = (0_usize, 0_usize, 0_usize, 0_usize);
    for n1 in [
        unit(Vec3::new(0.0, 0.0, 1.0)),
        unit(Vec3::new(0.3, 0.2, 1.0)),
        unit(Vec3::new(-0.7, 0.1, 0.4)),
        unit(Vec3::new(1.0, 1.0, 1.0)),
    ] {
        for arm in [1.0_f64, 2.0, 2.449_489_742_783_178, 3.7, 7.25, 123.456, 1e3] {
            for shape in SHAPES {
                let base = boundary_tilt(n1.get(), shape, arm, band).to_bits() as i64;
                for k in -400_i64..400 {
                    let s = f64::from_bits((base + k) as u64);
                    let Some(n2) = tilted(n1.get(), shape, s, band) else {
                        continue;
                    };
                    total += 1;
                    let held = Coset {
                        subgroup: Subgroup::Planar { normal: n1 },
                        representative: Affine3::identity(),
                    };
                    let added = Coset {
                        subgroup: Subgroup::Planar { normal: n2 },
                        representative: Affine3::identity(),
                    };
                    let want = one_spelling(n1.get(), n2.get(), arm, band);
                    let got = match intersect_subgroups(held.subgroup, added.subgroup, band, arm) {
                        Ok(Subgroup::Planar { .. }) => "parallel",
                        Ok(Subgroup::Prismatic { direction }) => {
                            let line = n1.get().cross(n2.get()) * arm;
                            assert_eq!(
                                bits3(direction.get()),
                                bits3(line.normalize()),
                                "the line is the levered cross product's own normalize"
                            );
                            "line"
                        }
                        Err(d) => {
                            assert_eq!(d.predicate, Some("mate_axes_parallel"), "{d:?}");
                            "escalates"
                        }
                        other => panic!("at s={s:e} arm={arm}: {other:?}"),
                    };
                    // The full door decides the same split first; what
                    // it adds past that is the singular translation
                    // stage named above, and nothing else.
                    match intersect(held, added, band, arm) {
                        Ok(_) => {}
                        Err(FoldStop::Indeterminate(d)) => assert!(
                            matches!(
                                d.predicate,
                                Some("mate_axes_parallel" | "mate_member_translation_in_plane")
                            ),
                            "{d:?}"
                        ),
                        Err(other) => panic!("at s={s:e} arm={arm}: {other:?}"),
                    }
                    assert_eq!(
                        got,
                        want,
                        "n1={:?} arm={arm} shape={shape:?} s={s:e}",
                        n1.get()
                    );
                    match got {
                        "parallel" => planes += 1,
                        "line" => lines += 1,
                        _ => escalations += 1,
                    }
                }
            }
        }
    }
    assert!(
        escalations > 0 && lines > 0,
        "the sweep straddles the edge: {total} pairs, {planes} planar, {lines} lines, \
         {escalations} escalations"
    );
}

/// **Through the doors, at the edge.** Two planar rests whose second
/// normal sits where the two spellings of the levered sine disagree —
/// found by search over the raw axes, with the arm realised as the
/// FOLD forms it (both parts' reach plus the first mate's `a` origin)
/// — solve to the verdict the one spelling gives: the prismatic
/// residual (UNDER) where the levered vector's norm clears K·ε, the
/// escalation under `mate_axes_parallel` where it lands inside the
/// band. Never a contradiction, never an escalation under another
/// name; the two spellings differ by at most two ulps at every
/// candidate, which is the re-baseline the one decision costs.
#[test]
fn c2_parallel_boundary_through_doors() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let escalate = band.escalate();
    let r0 = rig("msolve8-c2-doors-reach", 2);
    let rr = reach_of(&r0);
    let raw_n1s = [
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.3, 0.2, 1.0),
        Vec3::new(-0.7, 0.1, 0.4),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(0.1, -0.9, 0.35),
    ];
    let mut shapes: Vec<(f64, f64)> = SHAPES.to_vec();
    for k in 0..30 {
        let th = f64::from(k) * 0.2137 + 0.05;
        shapes.push((th.cos(), th.sin()));
    }
    // (first mate's a-origin x, raw n1, raw n2, arm): pairs where the
    // sine-times-arm spelling clears K·ε and the levered vector's norm
    // does not.
    let mut candidates: Vec<(f64, Vec3<f64>, Vec3<f64>, f64)> = Vec::new();
    'outer: for t in [0.0_f64, 0.5, 1.25, 1.2505, 2.0, 3.3, 4.8, 7.7, 12.1] {
        let first = al(
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Aligned,
            z_up_at([t, 0.0, 0.0]),
            z_up_at([0.0, 0.0, 0.0]),
            None,
        );
        let arm = rr + rr + first.lever_arm();
        for raw in raw_n1s {
            let n1 = UnitVec3::new(raw, FIXTURE_MATE_AXIS, band).unwrap().get();
            for &shape in &shapes {
                let (b1, b2) = n1.orthonormal_basis();
                let base = boundary_tilt(n1, shape, arm, band).to_bits() as i64;
                for k in -3000_i64..3000 {
                    let s = f64::from_bits((base + k) as u64);
                    let raw2 = n1 + b1 * (shape.0 * s) + b2 * (shape.1 * s);
                    let Ok(n2) = UnitVec3::new(raw2, FIXTURE_MATE_AXIS, band) else {
                        continue;
                    };
                    let cross = n1.cross(n2.get());
                    if cross.norm() * arm >= escalate && (cross * arm).norm() < escalate {
                        candidates.push((t, raw, raw2, arm));
                        if candidates.len() >= 10 {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(candidates.len(), 10, "the search finds the edge");
    let (mut under, mut escalations) = (0_usize, 0_usize);
    for (i, (t, raw1, raw2, arm)) in candidates.into_iter().enumerate() {
        let first = al(
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Aligned,
            frame([t, 0.0, 0.0], [raw1.x, raw1.y, raw1.z], [0.0, 1.0, 0.0]),
            z_up_at([0.0, 0.0, 0.0]),
            None,
        );
        let second = al(
            MatePrimitive::PlanarRest { offset: 0.0 },
            AxisSense::Aligned,
            frame([0.0, 0.0, 0.0], [raw2.x, raw2.y, raw2.z], [0.0, 1.0, 0.0]),
            z_up_at([0.0, 0.0, 0.0]),
            None,
        );
        // The witnesses the doors decide are the search's, and the
        // fold's arm is the one the search used.
        let w1 = first.a.axis(tol).unwrap().get();
        let w2 = second.a.axis(tol).unwrap().get();
        assert_eq!(
            bits3(w1),
            bits3(UnitVec3::new(raw1, FIXTURE_MATE_AXIS, band).unwrap().get())
        );
        let r = rig(&format!("msolve8-c2-doors-{i}"), 2);
        assert_eq!((rr + rr + first.lever_arm()).to_bits(), arm.to_bits());
        let (doc, _) = add(r.doc, mate(r.ids[0], r.ids[1], first));
        let (doc, added) = add(doc, mate(r.ids[0], r.ids[1], second));
        let want = one_spelling(w1, w2, arm, band);
        let sine_times_arm = w1.cross(w2).norm() * arm;
        let levered_norm = (w1.cross(w2) * arm).norm();
        assert!(
            (sine_times_arm - levered_norm).abs() <= 2.0 * f64::EPSILON * escalate,
            "the two spellings differ by ulps: {sine_times_arm:e} vs {levered_norm:e}"
        );
        match solve(&doc, &r.o, tol).fault(added) {
            Some(MateFault::Under { residual, .. }) => {
                assert_eq!(want, "line", "candidate {i}");
                assert_eq!(residual.name(), "prismatic");
                under += 1;
            }
            Some(MateFault::Indeterminate { diag, .. }) => {
                assert_eq!(want, "escalates", "candidate {i}");
                assert_eq!(diag.predicate, Some("mate_axes_parallel"));
                escalations += 1;
            }
            other => panic!("candidate {i}: {other:?}"),
        }
    }
    assert_eq!(under + escalations, 10);
}

/// **Inverting a coset adds no refusal.** Forty-seven frames — tilts
/// inside and outside the band, far origins, tiny and huge axes, a
/// reference a hair off the axis, a sweep of oblique aims — under
/// every primitive and both senses, authored `(first, second)` and
/// again `(second, first)`, where every direction is transported by
/// the representative's rotation and re-minted: the inverted
/// document's verdict is the direct document's. A frame the ladder
/// refuses — the `1e-150` axis at every ε, a reference the band
/// cannot tell from the axis at a coarse one — refuses at the READ
/// either way, with the same error; every other one solves or is
/// UNDER both ways, because a proper rotation keeps a witness's
/// length one within rounding.
#[test]
fn c2_inverted_coset_never_refuses() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut frames = vec![
        frame([0.0, 0.0, 0.0], [3.0 * eps, 0.0, 1.0], [0.0, 1.0, 0.0]),
        frame([0.0, 0.0, 0.0], [11.0 * eps, 0.0, 1.0], [0.0, 1.0, 0.0]),
        frame([1e6, -1e-6, 0.0], [1.0, 1.0, 0.0], [-1.0, 1.0, 2.0]),
        frame([0.25, -0.5, 0.75], [3.0, -4.0, 12.0], [0.0, 1.0, 0.0]),
        frame([0.0, 0.0, 0.0], [1e-150, 0.0, 1e-150], [0.0, 1.0, 0.0]),
        frame([0.0, 0.0, 0.0], [1e150, 0.0, 1e150], [0.0, 1.0, 0.0]),
        frame([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0 + 1e-7]),
    ];
    for k in 0..40 {
        let t = f64::from(k) * 0.37 + 0.01;
        frames.push(frame(
            [t, -t * 0.5, t * t * 0.1],
            [t.cos(), t.sin(), 0.3 * t],
            [-t.sin(), t.cos(), 1.0],
        ));
    }
    let (mut solved, mut refused_at_the_read) = (0_usize, 0_usize);
    for (i, f) in frames.iter().enumerate() {
        for (j, prim) in [
            MatePrimitive::FrameCoincidence,
            MatePrimitive::Coaxial,
            MatePrimitive::PlanarRest { offset: 0.1 },
        ]
        .into_iter()
        .enumerate()
        {
            for sense in [AxisSense::Aligned, AxisSense::Opposed] {
                let verdict = |reversed: bool| {
                    let r = rig(&format!("msolve8-c2-inv-{i}-{j}-{sense:?}-{reversed}"), 2);
                    let (a, b) = if reversed {
                        (r.ids[1], r.ids[0])
                    } else {
                        (r.ids[0], r.ids[1])
                    };
                    match at_the_door(
                        &r.doc,
                        &mate_reach::<f64>(&r.o, tol),
                        mate(a, b, al(prim, sense, *f, z_up_at([0.2, 0.0, 0.0]), None)),
                    ) {
                        Ok((doc, m)) => solve(&doc, &r.o, tol)
                            .fault(m)
                            .cloned()
                            .map(|fault| (Site::Solve, fault)),
                        Err((_, fault)) => Some((Site::Door, fault)),
                    }
                };
                match (verdict(false), verdict(true)) {
                    (None, None) => solved += 1,
                    (
                        Some((Site::Solve, MateFault::Under { .. })),
                        Some((Site::Solve, MateFault::Under { .. })),
                    ) => {}
                    (
                        Some((Site::Door, MateFault::Frame { error: direct, .. })),
                        Some((
                            Site::Door,
                            MateFault::Frame {
                                error: inverted, ..
                            },
                        )),
                    ) => {
                        assert_eq!(direct, inverted, "{f:?} {prim:?} {sense:?}");
                        refused_at_the_read += 1;
                    }
                    (direct, inverted) => {
                        panic!("{f:?} {prim:?} {sense:?}: direct {direct:?}, inverted {inverted:?}")
                    }
                }
            }
        }
    }
    // Which frames refuse at the read is the band's business and moves
    // with ε (the `1e-150` axis at every ε; the reference `1e-7` off
    // its axis once ε reaches it); what is pinned is that inversion
    // adds nothing to it.
    assert!(solved > 0 && refused_at_the_read > 0);
}

// ---- C4: the two mate-less arms, measured ----

/// Five instances on a reference no store resolves — a document
/// built WITHOUT the part doors, which decide under the band and so
/// refuse before a solve when it does — and the mate an author then
/// tries to add: the edit door asks the solve's own admission, which
/// begins with the band, so under a tolerance that admits none the
/// mate is refused `Band` at the insert door.
fn band_document(label: &str) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc_ref = DocRef {
        id: DocumentId::derive(&format!("{label}-part")),
        pin: ContentPin([7_u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..5 {
        let (next, id) = step(
            doc,
            DocEdit::InsertNode {
                node: Node::instantiate_part(doc_ref),
            },
        );
        doc = next;
        ids.push(id.expect("an instance inserts under a refusing band"));
    }
    (doc, ids)
}

/// `Band` is met at the insert door: every mate refuses there with
/// the solve's own `Band` fault, so no INSERT lands one — a snapshot
/// loaded under this tolerance still can — and the solve reaches
/// every instance the document holds.
fn band_refuses_every_mate(doc: &editor_core::ProfileDoc, ids: &[RecipeNodeId]) {
    let tol = Tol::witness();
    assert!(
        Band::linear(tol).is_err(),
        "the band must refuse for this row to measure anything"
    );
    let mut refused = 0_usize;
    for (x, y) in [(0, 1), (2, 3)] {
        let err = doc
            .apply(
                &DocEdit::InsertNode {
                    node: mate(
                        ids[x],
                        ids[y],
                        al(
                            MatePrimitive::FrameCoincidence,
                            AxisSense::Aligned,
                            z_up_at([0.0, 0.0, 0.0]),
                            z_up_at([0.0, 0.0, 1.0]),
                            None,
                        ),
                    ),
                },
                tol,
                &editor_core::RefusingReach,
            )
            .expect_err("no band, no admission");
        assert!(
            matches!(
                &err,
                editor_core::EditError::MateRefused { fault, .. }
                    if matches!(**fault, MateFault::Band { .. })
            ),
            "{err:?}"
        );
        refused += 1;
    }
    assert_eq!(refused, 2);
    assert!(
        doc.order()
            .iter()
            .all(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. }))),
        "the document holds its five instances and nothing else"
    );
    // And the solve of what the document does hold: `Band` reaches
    // EVERY instance — each its own singleton cluster — and nothing
    // else, since no band means no verdict for any of them.
    let poses = solve(doc, &EvalOptions::default(), tol);
    let mut instances = 0_usize;
    for &id in doc.order() {
        assert!(
            matches!(poses.fault(id), Some(MateFault::Band { .. })),
            "{id:?}: {:?}",
            poses.fault(id)
        );
        instances += 1;
    }
    assert_eq!(instances, 5);
}

/// **`Band` reaches every row: K·ε overflows** — every mate at the
/// door, every instance at the solve. One process per row —
/// `Tolerance::init` commits once — which is how this binary runs
/// under nextest.
#[test]
fn c4_band_refuses_every_mate_at_the_door_and_reaches_every_instance_overflow() {
    Tolerance::init(Tolerance {
        eps: 1e308,
        k: 10.0,
    })
    .expect("first commit in this process");
    let (doc, ids) = band_document("msolve8-c4-overflow");
    band_refuses_every_mate(&doc, &ids);
}

/// **`Band` reaches every row: K·ε rounds back onto ε.**
#[test]
fn c4_band_refuses_every_mate_at_the_door_and_reaches_every_instance_empty() {
    Tolerance::init(Tolerance {
        eps: 5e-324,
        k: 1.0 + f64::EPSILON,
    })
    .expect("first commit in this process");
    let (doc, ids) = band_document("msolve8-c4-empty");
    band_refuses_every_mate(&doc, &ids);
}

/// **`PosesOfAnotherDocument` reaches no row.** A solved document of
/// five instances and two mates records it against nothing;
/// `SolvedPoses::placement` raises it for every instance when read
/// against another document and places every one against its own;
/// and no node result of the evaluation carries it.
#[test]
fn c4_poses_of_another_document_reaches_no_row() {
    let r = rig("msolve8-c4-mispair", 5);
    let ids = r.ids.clone();
    let (doc, _) = add(
        r.doc,
        mate(
            ids[0],
            ids[1],
            al(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up_at([0.0, 0.0, 0.0]),
                z_up_at([0.0, 0.0, 1.0]),
                None,
            ),
        ),
    );
    let (doc, _) = add(
        doc,
        mate(
            ids[2],
            ids[3],
            al(
                MatePrimitive::FrameCoincidence,
                AxisSense::Aligned,
                z_up_at([0.0, 0.0, 0.0]),
                z_up_at([0.0, 0.0, 1.0]),
                None,
            ),
        ),
    );
    let tol = Tol::witness();
    let poses = solve(&doc, &r.o, tol);
    for &id in doc.order() {
        assert!(
            !matches!(
                poses.fault(id),
                Some(MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}: {:?}",
            poses.fault(id)
        );
    }
    let other = ProfileDoc::empty(DocumentId::derive("msolve8-c4-mispair-other"), tol);
    for &id in &ids {
        assert!(
            matches!(
                poses.placement(&other, id).map_err(|e| *e),
                Err(MateFault::PosesOfAnotherDocument { .. })
            ),
            "{id:?}"
        );
        assert!(
            poses.placement(&doc, id).is_ok(),
            "{id:?}: the own document places"
        );
    }
    let ev = run(&doc, &r.o);
    assert!(!format!("{:?}", ev.nodes).contains("PosesOfAnotherDocument"));
}

// ---- k-stats: the aim is decided twice per mate, not three times ----

fn decided(rec: &Recorded, name: &str) -> usize {
    rec.verdicts.iter().filter(|v| v.predicate == name).count()
}

/// **The aim is decided ONCE per frame door and twice per mate.**
/// `frame`, `placement` and `axis` each record `frame_point_at_aim`
/// once; a solve decides it twice per mate — one per side — and the
/// roll offset twice, measured as the difference between a one-mate
/// and a two-mate document on one pair, with the parts' reach warmed
/// first so their own decisions are excluded.
#[test]
fn kstats_aim_decided_twice_per_mate() {
    let tol = Tol::witness();
    let f = frame([0.25, -0.5, 0.75], [3.0, -4.0, 12.0], [0.0, 1.0, 0.0]);
    for (door, count) in [
        ("frame", {
            let b = Bracket::open();
            let _ = f.frame(tol).unwrap();
            decided(&b.finish(), "frame_point_at_aim")
        }),
        ("placement", {
            let b = Bracket::open();
            let _ = f.placement(tol).unwrap();
            decided(&b.finish(), "frame_point_at_aim")
        }),
        ("axis", {
            let b = Bracket::open();
            let _ = f.axis(tol).unwrap();
            decided(&b.finish(), "frame_point_at_aim")
        }),
    ] {
        assert_eq!(count, 1, "{door} decides the aim once");
    }
    let solve_with = |n: usize| {
        let r = rig(&format!("msolve8-kstats-{n}"), 2);
        let mut doc = r.doc;
        for _ in 0..n {
            let (next, _) = add(
                doc,
                mate(
                    r.ids[0],
                    r.ids[1],
                    al(
                        MatePrimitive::Coaxial,
                        AxisSense::Aligned,
                        z_up_at([0.0, 0.0, 0.0]),
                        z_up_at([0.0, 0.0, 0.0]),
                        None,
                    ),
                ),
            );
            doc = next;
        }
        let reach = mate_reach::<f64>(&r.o, tol);
        let _ = reach.reach(&r.doc_ref).unwrap();
        let b = Bracket::open();
        let poses = editor_core::solve_document(&doc, &reach, tol);
        let rec = b.finish();
        assert!(matches!(
            poses.fault(r.ids[1]),
            None | Some(MateFault::Under { .. })
        ));
        (
            decided(&rec, "frame_point_at_aim"),
            decided(&rec, "frame_point_at_roll_offset"),
        )
    };
    let (aim_one, roll_one) = solve_with(1);
    let (aim_two, roll_two) = solve_with(2);
    assert_eq!(
        (aim_two - aim_one, roll_two - roll_one),
        (2, 2),
        "one aim and one roll decision per side per mate"
    );
}

// ---- A3: verdicts that must not move under the witness ----

/// **A determined pair still determines, authored either way round.**
/// The A11 rule-1 exemplar (coaxial plus rest plus clocking) solves
/// with the mates on `(first, second)` and again on `(second,
/// first)`, where every direction is transported and re-minted — and
/// the second document stands the rest the other way up, the same
/// distance, because the rest's `a` plane now belongs to the other
/// instance. The V-block (two planar rests at a right angle, the
/// planes' line minted by the parallel verdict) still leaves the
/// prismatic residual along that line.
#[test]
fn a_determined_pair_and_a_v_block_keep_their_verdicts_under_the_witness() {
    let mut poses_seen = Vec::new();
    for (label, reversed) in [
        ("msolve8-determined", false),
        ("msolve8-determined-rev", true),
    ] {
        let r = rig(label, 2);
        let [first, second] = [r.ids[0], r.ids[1]];
        let (a, b) = if reversed {
            (second, first)
        } else {
            (first, second)
        };
        let (doc, _) = add(
            r.doc,
            mate(
                a,
                b,
                al(
                    MatePrimitive::Coaxial,
                    AxisSense::Aligned,
                    z_up_at([0.0; 3]),
                    z_up_at([0.0; 3]),
                    Some(0.0),
                ),
            ),
        );
        let (doc, rest) = add(
            doc,
            mate(
                a,
                b,
                al(
                    MatePrimitive::PlanarRest { offset: -1.0 },
                    AxisSense::Opposed,
                    frame([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                    frame([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0]),
                    None,
                ),
            ),
        );
        let poses = solve(&doc, &r.o, Tol::witness());
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
    let r = rig("msolve8-v-block", 2);
    let [first, second] = [r.ids[0], r.ids[1]];
    let mut doc = r.doc;
    for axis in [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]] {
        let (next, _) = add(
            doc,
            mate(
                first,
                second,
                al(
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
    let fault = solve(&doc, &r.o, Tol::witness())
        .fault(second)
        .expect("the V-block is UNDER")
        .clone();
    let MateFault::Under { residual, .. } = &fault else {
        panic!("expected UNDER, got {fault:?}");
    };
    assert_eq!(residual.name(), "prismatic");
    assert!(
        fault.to_string().contains("translation along [0, 0, 1]"),
        "the planes' line is the parallel verdict's own mint: {fault}"
    );
}
