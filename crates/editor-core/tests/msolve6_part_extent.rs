//! MSOLVE-6 acceptance — **the mate's lever is the mated parts' own
//! extent**, taken from each part's evaluated body (the MSOLVE-6
//! spec's rows A2–A5; the spec was deleted at the unit's merge).
//!
//! The lever a mate's angular decisions turn on is
//! `(R_a + ‖a.origin‖) + (R_b + ‖b.origin‖) + Σ|authored lengths|`,
//! each `R` an upper bound on the part's reach from its own origin,
//! with no floor and no constant. These rows measure the reach against
//! bodies whose true extent is known, pin the lever to the bit, decide
//! the measure site's own example at the parts' scale (at whatever ε
//! the run is at — the expectation is derived from the run's own
//! `Band`), and pin the refusals and the exactly-once evaluation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use std::sync::Arc;

use editor_core::mate::SurfaceKind;
use editor_core::{
    Alignment, AxisSense, CapEnd, Clash, ClusterMaintenance, ContactClass, DocEdit, DocumentId,
    EditError, EvalOptions, Frame, FrameFault, Lever, LeverRefusal, LoggedEdit, MateFault,
    MateFrame, MatePrimitive, MateReach, MateRole, Node, NodeErrorKind, NodeResult, PartFault,
    PartReach, PersistError, ProfileDoc, ReachRefusal, RecipeNodeId, ResolveFault, SplitError,
    content_pin, mate_reach, product, split,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{
    ang, at_the_door, axis_in_plane, insert, len, on_frame, on_frame_keeping, run, solve, step,
};
use geom_core::predicate::{Band, Sign};
use geom_core::{Decide, Point3, Tol};

// ---- Substrate ----

/// A box part: a square of half-side `half` at the origin, extruded
/// `height` along +z. Its farthest point from the part origin is the
/// top corner, `sqrt(2 half² + height²)` away.
fn box_part(label: &str, half: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
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

/// A cylinder part: a rectangle `radius × height` in the xy plane,
/// revolved a full turn about the plane's +y through the origin. The
/// cylinder stands on the origin along +y; its farthest point from
/// the origin is the top rim, `sqrt(radius² + height²)` away.
fn cylinder_part(label: &str, radius: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, plane, profile) = on_frame_keeping(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (radius, 0.0),
            (radius, height),
            (0.0, height),
        ]],
    );
    let (doc, axis) = insert(doc, axis_in_plane(plane, (0.0, 0.0), (0.0, 1.0)));
    let (doc, _) = insert(
        doc,
        Node::Revolve {
            profile,
            axis,
            angle: ang(std::f64::consts::TAU),
        },
    );
    doc
}

/// `n` instances of `part`, and the options that resolve them.
fn instances(
    label: &str,
    part: ProfileDoc,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions) {
    let mut store = PartStore::new();
    let doc_ref = store.insert(part, Tol::witness());
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..n {
        let (next, id) = insert(doc, Node::instantiate_part(doc_ref));
        doc = next;
        ids.push(id);
    }
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    (doc, ids, opts)
}

fn frame(origin: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0])
}

/// A frame coincidence between `a`'s top cap and `b`'s bottom cap,
/// with a clocking rider: on a coincidence the rider is
/// redundant-or-contradictory, DECIDED at the lever, so it is the one
/// arm that reports `Clash::Levered(Lever::Roll { radians: θ, arm: L })`
/// — the row's window onto `L`.
fn clocked(
    a: RecipeNodeId,
    b: RecipeNodeId,
    alignment: Alignment,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: fixture::head(in_part(a, CapEnd::End)),
        b: fixture::head(in_part(b, CapEnd::Start)),
        class: ContactClass::Rest,
        alignment,
    }
}

fn coincidence(fa: MateFrame, fb: MateFrame, clocking: f64) -> Alignment {
    Alignment {
        a: fa,
        b: fb,
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Aligned,
        clocking: Some(clocking),
    }
}

/// [`at_the_door`] through the store's reach — the door decides a
/// clocking rider on a coincidence over the mated parts' extent, so a
/// rider (a zero one included) needs the parts in hand where the mate
/// is authored.
fn at_the_store(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    node: Node<editor_core::ProfileProgram>,
) -> Result<(ProfileDoc, RecipeNodeId), (RecipeNodeId, MateFault)> {
    at_the_door(doc, &mate_reach::<f64>(opts, Tol::witness()), node)
}

/// [`at_the_store`] for a mate the door admits.
fn mated(
    doc: ProfileDoc,
    opts: &EvalOptions,
    node: Node<editor_core::ProfileProgram>,
) -> (ProfileDoc, RecipeNodeId) {
    at_the_store(&doc, opts, node).unwrap_or_else(|(_, fault)| panic!("the door admits: {fault}"))
}

/// The reach of every instance in `ids`, through the public door:
/// each instance's part, read off the document the way the solve
/// reads it.
fn reaches(doc: &ProfileDoc, opts: &EvalOptions, ids: &[RecipeNodeId]) -> Vec<f64> {
    let reach = mate_reach::<f64>(opts, Tol::witness());
    ids.iter()
        .map(|&id| {
            let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(id) else {
                panic!("node {} is not an instance", id.0);
            };
            reach
                .reach(doc_ref)
                .expect("the part resolves and its body is bounded")
        })
        .collect()
}

/// The options of a store holding every box part named — the reach a
/// mate is AUTHORED through where its solve is then read through a
/// store missing one of them.
fn with_both(labels: &[&str]) -> EvalOptions {
    let mut store = PartStore::new();
    for label in labels {
        store.insert(box_part(label, 0.5, 1.0), Tol::witness());
    }
    with_resolver(store)
}

// ---- A2: the reach is an upper bound, measured ----

/// **A box part's reach is at least its far corner's distance** — and
/// for a body whose every edge is a line the rim walk is EXACT, so it
/// is that distance to the bit.
#[test]
fn a2_a_box_parts_reach_is_its_far_corner() {
    let (half, height) = (0.5, 1.0);
    let (doc, ids, opts) = instances(
        "msolve6-a2-box",
        box_part("msolve6-a2-box-part", half, height),
        1,
    );
    let far = (2.0 * half * half + height * height).sqrt();
    let r = reaches(&doc, &opts, &ids)[0];
    assert!(r >= far, "the reach {r} must bound the far corner {far}");
    // Every edge is a line, so the bound is the maximum over the
    // vertices — the corner itself, to the bit.
    assert_eq!(r, far, "a box's rim walk is exact");
}

/// **A cylinder part's reach is at least `sqrt(r² + h²)`**, its top
/// rim's distance: the rim is a circle bounded by `‖centre‖ + r`,
/// which is `h + r ≥ sqrt(r² + h²)` — an upper bound, never under.
#[test]
fn a2_a_cylinder_parts_reach_is_at_least_its_far_rim() {
    let (radius, height) = (0.3, 0.7);
    let (doc, ids, opts) = instances(
        "msolve6-a2-cyl",
        cylinder_part("msolve6-a2-cyl-part", radius, height),
        1,
    );
    let far = (radius * radius + height * height).sqrt();
    let r = reaches(&doc, &opts, &ids)[0];
    assert!(r >= far, "the reach {r} must bound the far rim {far}");
    assert!(
        r <= height + radius + 1e-12,
        "and it is the rim circle's own bound, not wider: {r} vs {}",
        height + radius
    );
}

/// **The lever of a mate on two parts is the formula, to the bit**:
/// the solve's `L` equals `(R_a + R_b) + (‖a.origin‖ + ‖b.origin‖ +
/// Σ|lengths|)` computed in the row from the same public reach and
/// the alignment's own datum term — grouped as the solve groups it
/// (the two reaches first, then the datum's sum).
#[test]
fn a2_the_lever_is_the_formula_to_the_bit() {
    let (doc, ids, opts) = instances(
        "msolve6-a2-lever",
        box_part("msolve6-a2-lever-part", 0.5, 1.0),
        2,
    );
    let alignment = coincidence(
        frame([0.1, 0.0, 1.0]),
        frame([0.0, 0.2, 0.0]),
        core::f64::consts::FRAC_PI_2,
    );
    // The rider is decided where the mate is authored, over the same
    // lever the solve forms: the door refuses it with the solve's
    // own fault.
    let (_, fault) = at_the_store(&doc, &opts, clocked(ids[0], ids[1], alignment.clone()))
        .expect_err("a quarter-turn rider contradicts the coincidence");
    let MateFault::Contradictory {
        clash: Clash::Levered(Lever::Roll {
            radians: theta,
            arm,
        }),
        ..
    } = &fault
    else {
        panic!("expected CONTRADICTORY with a lever, got {fault:?}");
    };
    assert_eq!(*theta, core::f64::consts::FRAC_PI_2);
    let r = reaches(&doc, &opts, &ids);
    let expected = (r[0] + r[1]) + datum_lever(&alignment);
    assert_eq!(*arm, expected, "the lever is the formula, bit for bit");
    // And the datum's own term is what the formula says it is.
    assert_eq!(datum_lever(&alignment), 0.1_f64.hypot(1.0) + 0.2);
}

// ---- A3: the lever decides at the parts' scale ----

/// What the run's own band says a levered tilt decides to.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Verdict {
    Parallel,
    Refused,
    Indeterminate,
}

/// Through the predicate door itself (`Decide::sign_within`, the rule
/// every metered predicate decides by), so a change to the band's
/// strictness moves this expectation with the solve rather than
/// leaving the row testing a copied rule.
fn verdict(band: Band, theta: f64, arm: f64) -> Verdict {
    match (theta * arm).sign_within(band) {
        Ok(Sign::Zero) => Verdict::Parallel,
        Ok(Sign::Positive | Sign::Negative) => Verdict::Refused,
        Err(_) => Verdict::Indeterminate,
    }
}

/// The measure site's own example, at the mate site: two copies of
/// a part of half-side `half` and height `2 half`, one seated on the
/// other by a frame coincidence with a `1e-8` rad clocking rider.
/// The verdict the solve reaches must be the one the run's band gives
/// for `1e-8 · L` at the PARTS' lever `L` — and where the band
/// separates that from the metre's verdict, the two differ.
fn tilted(label: &str, half: f64) -> (Verdict, Verdict, Option<MateFault>, f64) {
    let theta = 1e-8;
    let (doc, ids, opts) = instances(
        label,
        box_part(&format!("{label}-part"), half, 2.0 * half),
        2,
    );
    let alignment = coincidence(frame([0.0, 0.0, 2.0 * half]), frame([0.0; 3]), theta);
    let r = reaches(&doc, &opts, &ids);
    let arm = (r[0] + r[1]) + datum_lever(&alignment);
    let band = Band::linear(Tol::witness()).expect("the band");
    // The verdict is reached where the mate is authored: an admitted
    // rider enters and the solve places the pair; a refused one
    // carries the solve's own fault out of the door.
    let fault = match at_the_store(&doc, &opts, clocked(ids[0], ids[1], alignment)) {
        Ok((doc, mate)) => {
            let poses = solve(&doc, &opts, Tol::witness());
            assert_eq!(
                poses.fault(mate),
                None,
                "admitted at the door, placed by the solve"
            );
            assert_eq!(poses.role(mate), Some(MateRole::Determining));
            None
        }
        Err((_, fault)) => Some(fault),
    };
    let found = match &fault {
        None => Verdict::Parallel,
        Some(MateFault::Contradictory {
            predicate,
            clash: Clash::Levered(Lever::Roll { radians: t, arm: l }),
            ..
        }) => {
            assert_eq!(*predicate, "mate_clocking_redundant");
            assert_eq!(
                (*t, *l),
                (theta, arm),
                "the refusal carries the parts' lever"
            );
            Verdict::Refused
        }
        Some(MateFault::Indeterminate { .. }) => Verdict::Indeterminate,
        Some(other) => panic!("unexpected fault {other:?}"),
    };
    (found, verdict(band, theta, 1.0), fault, arm)
}

/// **A 10 mm part tilted by 1e-8 rad is PARALLEL across its own
/// extent**: the deviation is `1e-8 · L` with `L` a few centimetres,
/// a few 1e-10 m — under the default ε, where a metre lever priced
/// the same tilt at 1e-8 and refused it.
#[test]
fn a3_a_10mm_part_tilted_1e_8_decides_at_its_own_scale() {
    let (found, metre, fault, arm) = tilted("msolve6-a3-10mm", 0.005);
    let expected = verdict(Band::linear(Tol::witness()).expect("the band"), 1e-8, arm);
    assert_eq!(found, expected, "at L = {arm}: {fault:?}");
    assert!(
        arm < 0.1,
        "a 10 mm part's lever is centimetres, not a metre: {arm}"
    );
    // At the default ε the parts' verdict is PARALLEL and the metre's
    // is REFUSED; at every ε the run's verdict is the parts', not the
    // metre's, wherever the band tells them apart.
    if Tol::witness().eps() == 1e-9 {
        assert_eq!((found, metre), (Verdict::Parallel, Verdict::Refused));
    }
}

/// **A 10 m part tilted by 1e-8 rad is REFUSED**: across tens of
/// metres the same tilt deviates by 1e-7 m, and the refusal names the
/// part-scale lever — where a metre lever, at 1e-8, decided the same
/// way only by accident of the constant, and at a coarser ε passed it.
#[test]
fn a3_a_10m_part_tilted_1e_8_is_refused_at_its_scale() {
    let (found, _, fault, arm) = tilted("msolve6-a3-10m", 5.0);
    let expected = verdict(Band::linear(Tol::witness()).expect("the band"), 1e-8, arm);
    assert_eq!(found, expected, "at L = {arm}: {fault:?}");
    assert!(arm > 10.0, "a 10 m part's lever is tens of metres: {arm}");
    if Tol::witness().eps() == 1e-9 {
        assert_eq!(found, Verdict::Refused);
    }
}

// ---- A4: refusals typed ----

/// **A part that does not resolve faults the mate `Unleverable`,
/// carrying the `PartFault` unaltered** — and the blast radius is
/// pinned: the mate faults (it used to stay `Determining`), and its
/// cluster's instances carry the mate fault, in the resolver's voice.
#[test]
fn a4_an_unresolvable_part_faults_the_mate_in_the_resolvers_voice() {
    // The part lives in ANOTHER store: the reference is well formed
    // and nothing here can resolve it.
    let mut elsewhere = PartStore::new();
    let doc_ref = elsewhere.insert(box_part("msolve6-a4-elsewhere", 0.5, 1.0), Tol::witness());
    let (doc, ids, opts) = instances(
        "msolve6-a4-unresolved",
        box_part("msolve6-a4-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, Node::instantiate_part(doc_ref));
    // Authored where both parts are in hand — the door decides the
    // rider over them — and solved where one is not.
    let (doc, mate) = mated(
        doc,
        &with_both(&["msolve6-a4-part", "msolve6-a4-elsewhere"]),
        clocked(
            ids[0],
            lost,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let poses = solve(&doc, &opts, Tol::witness());
    let fault = poses.fault(mate).expect("the mate faults with its part");
    let MateFault::Unleverable {
        mate: named,
        refusal,
    } = fault
    else {
        panic!("expected UNLEVERABLE, got {fault:?}");
    };
    assert_eq!(*named, mate);
    let LeverRefusal::PartUnresolved {
        instance,
        fault: part,
    } = refusal
    else {
        panic!("expected the part's own fault, got {refusal:?}");
    };
    assert_eq!(*instance, lost);
    assert!(
        matches!(
            part,
            PartFault::Unresolved {
                fault: ResolveFault::Unresolved,
                ..
            }
        ),
        "the resolver's own classification, unaltered: {part:?}"
    );
    assert_eq!(poses.role(mate), Some(MateRole::Refused));
    // The blast radius: every instance in the cluster carries the
    // fault, and the evaluation fails them in the mate's voice.
    assert_eq!(poses.fault(ids[0]), Some(fault));
    assert_eq!(poses.fault(lost), Some(fault));
    let ev = run(&doc, &opts);
    for node in [mate, ids[0], lost] {
        match ev.result(node) {
            Some(NodeResult::Failed(e)) => assert!(
                matches!(&e.kind, NodeErrorKind::Mate(f) if **f == *fault),
                "node {}: expected the mate fault, got {:?}",
                node.0,
                e.kind
            ),
            other => panic!("node {}: expected a failure, got {other:?}", node.0),
        }
    }
    // No resolver at all: the same arm, in that voice.
    let none = EvalOptions::default();
    let fault = solve(&doc, &none, Tol::witness())
        .fault(mate)
        .cloned()
        .expect("no resolver, no lever");
    assert!(
        matches!(
            fault,
            MateFault::Unleverable {
                refusal: LeverRefusal::PartUnresolved {
                    fault: PartFault::NoResolver,
                    ..
                },
                ..
            }
        ),
        "{fault:?}"
    );
}

/// **A body with a face whose reach cannot be bounded refuses typed,
/// naming the face and its kind.** No document door builds such a
/// face — every surface kind a validated body can hold is bounded
/// (`mate::reach`'s per-kind table) — so the arm is pinned at the
/// unit level on a hand-built body: the Euler `mvfs` seed, whose one
/// face stands on the all-poison NURBS placeholder, which has no
/// finite hull to bound.
#[test]
fn a4_a_face_whose_reach_cannot_be_bounded_refuses_typed() {
    let mut body = topo::Body::<f64>::new();
    let made = body
        .mvfs(Point3::new(0.0, 0.0, 0.0))
        .expect("the seed vertex-face-shell");
    let refusal = editor_core::mate::body_reach(&body).expect_err("the placeholder has no bound");
    assert_eq!(
        refusal,
        ReachRefusal::FaceUnbounded {
            face: made.face,
            kind: SurfaceKind::Nurbs,
        }
    );
    // The same answer through the door the solve reads.
    assert_eq!(
        editor_core::mate::part_reach(&body).err(),
        Some(refusal.clone())
    );
    let instance = RecipeNodeId(7);
    let part = editor_core::DocRef {
        id: DocumentId::derive("msolve6-a4-unbounded"),
        pin: content_pin(&box_part("msolve6-a4-unbounded", 0.5, 1.0), Tol::witness()).unwrap(),
    };
    assert_eq!(
        LeverRefusal::of(refusal, instance, part),
        LeverRefusal::FaceUnbounded {
            instance,
            part,
            face: made.face,
            kind: SurfaceKind::Nurbs,
        }
    );
    // A face whose surface key resolves to nothing is a malformed
    // body, not an unboundable face: its own arm, named the same way.
    // (No door builds one — `Body` mints a face's surface with the
    // face — so the arm is pinned at the wrap.)
    assert_eq!(
        LeverRefusal::of(
            ReachRefusal::MalformedBody { face: made.face },
            instance,
            part
        ),
        LeverRefusal::MalformedBody {
            instance,
            part,
            face: made.face,
        }
    );
}

// ---- A5: the evaluation and the memo ----

/// **A mated part is evaluated exactly once**: the solve's reach comes
/// off the run's own part cache, so the instantiate nodes hit what
/// the lever already resolved — one crossing for two instances and
/// one mate, the same count as with no mate at all.
#[test]
fn a5_a_mated_part_is_evaluated_exactly_once() {
    let (doc, ids, opts) = instances(
        "msolve6-a5-once",
        box_part("msolve6-a5-once-part", 0.5, 1.0),
        2,
    );
    let before = run(&doc, &opts);
    assert_eq!(
        before.part_evaluations, 1,
        "two instances, one part, one crossing"
    );
    let (doc, mate) = mated(
        doc,
        &opts,
        clocked(
            ids[0],
            ids[1],
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let after = run(&doc, &opts);
    assert_eq!(
        after.part_evaluations, 1,
        "the lever's ask and the instantiate nodes share one evaluation"
    );
    assert!(
        matches!(after.result(mate), Some(NodeResult::Ok(_))),
        "{:?}",
        after.result(mate)
    );
    for &id in &ids {
        assert!(matches!(after.result(id), Some(NodeResult::Ok(_))));
    }
}

/// **A part whose content changes so that the verdict flips reaches
/// the mate's memo**: the solve's answer is in the mate's content
/// key (MSOLVE-4), so the mate that evaluated `Ok` over a 10 mm part
/// is not served from the memo over a 10 m one — it faults, and its
/// instances are recomputed with it.
#[test]
fn a5_a_part_change_that_flips_the_verdict_moves_the_mates_memo() {
    let theta = 1e-8;
    // Two versions of ONE part document: the same id, so the
    // reference can be re-pinned in place; different extents.
    let small = box_part("msolve6-a5-memo-part", 0.005, 0.01);
    let large = box_part("msolve6-a5-memo-part", 5.0, 10.0);
    let large_pin = content_pin(&large, Tol::witness()).unwrap();
    let mut store_small = PartStore::new();
    let small_ref = store_small.insert(small, Tol::witness());
    let mut store_large = PartStore::new();
    store_large.insert(large, Tol::witness());
    let opts_small = EvalOptions {
        resolver: Some(Arc::new(store_small)),
        ..EvalOptions::default()
    };
    let opts_large = EvalOptions {
        resolver: Some(Arc::new(store_large)),
        ..EvalOptions::default()
    };
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-a5-memo"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(small_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(small_ref));
    // The row is about the flip, so it only runs where the band flips
    // it: parallel at the small scale, refused at the large.
    let band = Band::linear(Tol::witness()).expect("the band");
    let (r_small, r_large) = (reaches(&doc, &opts_small, &[a])[0], {
        let (re, _) = step(
            doc.clone(),
            DocEdit::UpdateReference {
                node: a,
                new_pin: large_pin,
            },
        );
        let (re, _) = step(
            re,
            DocEdit::UpdateReference {
                node: b,
                new_pin: large_pin,
            },
        );
        reaches(&re, &opts_large, &[a])[0]
    });
    let small_verdict = verdict(band, theta, 2.0 * r_small + 0.01);
    let large_verdict = verdict(band, theta, 2.0 * r_large + 0.01);
    // Authored over the small part: the door decides the rider over
    // the parts as they are, so the mate enters exactly where the
    // small part's verdict is PARALLEL — and the flip is measurable
    // only there. Elsewhere the door's refusal IS the small part's
    // verdict, and the row ends on it.
    let node = clocked(
        a,
        b,
        coincidence(frame([0.0, 0.0, 0.01]), frame([0.0; 3]), theta),
    );
    let (doc, mate) = match at_the_store(&doc, &opts_small, node) {
        Ok(admitted) => {
            assert_eq!(small_verdict, Verdict::Parallel, "admitted, so parallel");
            admitted
        }
        Err((_, fault)) => {
            assert!(
                matches!(
                    (small_verdict, &fault),
                    (Verdict::Refused, MateFault::Contradictory { .. })
                        | (Verdict::Indeterminate, MateFault::Indeterminate { .. })
                ),
                "the door's refusal is the small part's verdict: {small_verdict:?} vs {fault:?}"
            );
            // The flip's premise — parallel at the small scale — does
            // not hold at this ε, so the row's claim has no subject
            // here and this is where it ends, having pinned that the
            // door and the band agree about why.
            assert_ne!(small_verdict, Verdict::Parallel);
            eprintln!(
                "a5 memo: the band refuses at the small scale at this ε \
                 ({small_verdict:?}; large {large_verdict:?}), so the flip is not reachable"
            );
            return;
        }
    };
    let first = run(&doc, &opts_small);
    let key_small = first
        .value(mate)
        .map(|v| v.content_key)
        .expect("the small part's mate evaluates");
    let (re, _) = step(
        doc,
        DocEdit::UpdateReference {
            node: a,
            new_pin: large_pin,
        },
    );
    let (re, _) = step(
        re,
        DocEdit::UpdateReference {
            node: b,
            new_pin: large_pin,
        },
    );
    let second = editor_core::evaluate::<f64>(
        &re,
        Some(&first),
        &editor_core::CancelToken::new(),
        &opts_large,
        Tol::witness(),
    );
    match large_verdict {
        Verdict::Refused => assert!(
            matches!(second.result(mate), Some(NodeResult::Failed(e)) if matches!(e.kind, NodeErrorKind::Mate(_))),
            "the re-pinned part flips the verdict and the memo cannot serve it: {:?}",
            second.result(mate)
        ),
        Verdict::Parallel | Verdict::Indeterminate => {
            // The band did not separate the two scales at this ε, so
            // the solve's answer did not move and neither does the
            // key: the mate's key follows the VERDICT, not the part's
            // bytes, which is exactly the claim.
            let key_large = second.value(mate).map(|v| v.content_key);
            assert_eq!(Some(key_small), key_large, "no flip, no key move");
        }
    }
}

// ---- The edit door (the spec's amendment): the reach, the log, replay ----

/// A reach that counts its asks and answers the refusing reach's
/// answer — the witness that an edit which moves no gauge never
/// consults the store.
/// A reach that counts its asks and answers through the store's.
struct Counting<'a>(core::cell::Cell<usize>, PartReach<'a, f64>);

impl MateReach for Counting<'_> {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        self.1.reach(part)
    }

    fn face_pose(
        &self,
        part: &editor_core::DocRef,
        face: &editor_core::FaceName,
    ) -> Result<topo::readback::Pose<f64>, editor_core::FacePoseRefusal> {
        self.1.face_pose(part, face)
    }
}

/// The datum's own lever term, for an alignment whose two sides are
/// authored vectors (every alignment this file authors).
fn datum_lever(a: &Alignment) -> f64 {
    a.lever_arm(
        a.a.authored_vectors().expect("an authored side"),
        a.b.authored_vectors().expect("an authored side"),
    )
}

/// Two instances of a box seated by a frame coincidence (a clocked,
/// determined pair), through the store: the mated document the edit
/// rows work on, with its log as the file carries it.
fn seated(
    label: &str,
) -> (
    ProfileDoc,
    [RecipeNodeId; 2],
    RecipeNodeId,
    EvalOptions,
    Vec<editor_core::LoggedEdit<editor_core::ProfileProgram>>,
) {
    let (doc, ids, opts) = instances(label, box_part(&format!("{label}-part"), 0.5, 1.0), 0);
    debug_assert!(ids.is_empty());
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let mut log = Vec::new();
    let mut doc = doc;
    let mut push = |doc: &mut ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>| {
        let applied = doc
            .apply(&edit, Tol::witness(), &reach)
            .expect("the edit applies");
        log.push(editor_core::LoggedEdit {
            edit,
            maintenance: applied.cluster_rows(),
        });
        *doc = applied.doc;
        applied.record.minted
    };
    let part_ref = {
        // The store inside `opts` already holds the part; its reference
        // is the pin of the same document.
        let part = box_part(&format!("{label}-part"), 0.5, 1.0);
        editor_core::DocRef {
            id: part.id(),
            pin: content_pin(&part, Tol::witness()).unwrap(),
        }
    };
    let a = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::instantiate_part(part_ref),
        },
    )
    .unwrap();
    let b = push(
        &mut doc,
        DocEdit::InsertNode {
            node: Node::instantiate_part(part_ref),
        },
    )
    .unwrap();
    let mate = push(
        &mut doc,
        DocEdit::InsertNode {
            node: clocked(
                a,
                b,
                coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
            ),
        },
    )
    .unwrap();
    (doc, [a, b], mate, opts, log)
}

/// **A mate-graph edit on a document whose part does not resolve
/// refuses at the door**, carrying the solve's fault: deleting the
/// mate moves the orphan's gauge, the maintenance solves the prior
/// document for its pose, and the solve cannot lever a part it cannot
/// reach — no verdict, so no frame is recorded.
#[test]
fn a6_a_mate_graph_edit_on_an_unresolvable_part_refuses_typed() {
    let mut elsewhere = PartStore::new();
    let lost_ref = elsewhere.insert(box_part("msolve6-a6-elsewhere", 0.5, 1.0), Tol::witness());
    let (doc, ids, opts) = instances(
        "msolve6-a6-unresolved",
        box_part("msolve6-a6-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, Node::instantiate_part(lost_ref));
    let (doc, mate) = mated(
        doc,
        &with_both(&["msolve6-a6-part", "msolve6-a6-elsewhere"]),
        clocked(
            ids[0],
            lost,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let err = doc
        .apply(&DocEdit::DeleteNode { id: mate }, Tol::witness(), &reach)
        .expect_err("the orphan's frame cannot be minted");
    let editor_core::EditError::MaintenanceRefused { gauge, fault } = &err else {
        panic!("expected MaintenanceRefused, got {err:?}");
    };
    assert_eq!(*gauge, lost);
    assert!(
        matches!(
            fault.as_deref(),
            Some(MateFault::Unleverable {
                refusal: LeverRefusal::PartUnresolved { instance, .. },
                ..
            }) if *instance == lost
        ),
        "the resolver's own voice: {fault:?}"
    );
    // The same edit through no resolver at all: the same arm.
    let err = doc
        .apply(
            &DocEdit::DeleteNode { id: mate },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("no resolver, no frame");
    assert!(
        matches!(err, editor_core::EditError::MaintenanceRefused { .. }),
        "{err:?}"
    );
}

/// **An edit that moves no gauge never asks the reach for its
/// maintenance**: on a mated document, a second mate (a Join — the
/// survivor keeps its gauge) asks exactly what its OWN admission
/// needs — the rider on its coincidence is decided over its two
/// parts, once each — and a placement, an appearance, a declare and
/// a fourth instance ask nothing more. An edit that moves a gauge
/// asks exactly what the prior document's solve asks: `pairs × 2` —
/// each mated pair asks both its parts' reach once, lazily, at its
/// first mate — so on the three-instance chain `a–b`, `b–c` a Split
/// (deleting the pair's mate) and a GaugeRewrite (deleting the gauge
/// instance) each ask `2 × 2 = 4`.
#[test]
fn a6_a_gauge_preserving_edit_asks_only_its_own_admission() {
    let (doc, [a, b], mate, opts, _log) = seated("msolve6-a6-preserving");
    let counting = Counting(
        core::cell::Cell::new(0),
        mate_reach::<f64>(&opts, Tol::witness()),
    );
    let tol = Tol::witness();
    // A Join: a mate to a third instance, whose singleton cluster is
    // absorbed by the pair's, gauge unchanged.
    let part_ref = match doc.node(a) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        _ => panic!("an instance"),
    };
    let (doc, c) = insert(doc, Node::instantiate_part(part_ref));
    let applied = doc
        .apply(
            &DocEdit::InsertNode {
                node: clocked(
                    b,
                    c,
                    coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
                ),
            },
            tol,
            &counting,
        )
        .expect("a join asks nothing of the maintenance");
    assert!(
        matches!(applied.cluster_rows()[..], [editor_core::ClusterMaintenance::Join { survived, absorbed, .. }] if survived == a && absorbed == c),
        "{:?}",
        applied.maintenance
    );
    // The door's own admission asked for the rider's lever — the two
    // mated parts, once each — and the maintenance for nothing.
    let admission = counting.0.get();
    assert_eq!(admission, 2, "the rider is decided over its two parts");
    let doc = applied.doc;
    let applied = doc
        .apply(
            &DocEdit::SetPlacement {
                node: a,
                frame: editor_core::Frame::translation([0.0, 0.0, 3.0]),
            },
            tol,
            &counting,
        )
        .expect("a placement asks nothing");
    assert!(applied.maintenance.is_empty());
    let doc = applied.doc;
    // An appearance, a Declare, and a fourth instance: none touches
    // the mate graph, so none asks.
    let doc = doc
        .apply(
            &DocEdit::SetAppearance {
                name: in_part(a, CapEnd::End),
                attr: editor_core::Attr::Color(editor_core::Rgba8::opaque(200, 30, 30)),
            },
            tol,
            &counting,
        )
        .expect("an appearance asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::declare_rest(Vec::new()),
            },
            tol,
            &counting,
        )
        .expect("a declare asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Node::instantiate_part(part_ref),
            },
            tol,
            &counting,
        )
        .expect("a fourth instance asks nothing")
        .doc;
    assert_eq!(
        counting.0.get(),
        admission,
        "no gauge moved, so the maintenance never consulted the store"
    );
    // Deleting the pair's mate splits the cluster: the prior
    // document's solve asks the chain's two pairs for their two parts
    // each, and nothing else.
    let pairs = 2;
    let split = doc
        .apply(&DocEdit::DeleteNode { id: mate }, tol, &counting)
        .expect("a split solves through the store");
    assert!(
        matches!(split.cluster_rows()[..], [ClusterMaintenance::Split { from, to, frame: Some(_) }] if from == a && to == b),
        "{:?}",
        split.maintenance
    );
    assert_eq!(
        counting.0.get(),
        admission + pairs * 2,
        "asks = pairs × 2 parts"
    );
    // Deleting the gauge instance rewrites the gauge: the same prior
    // solve, the same asks.
    let rewrite = doc
        .apply(&DocEdit::DeleteNode { id: a }, tol, &counting)
        .expect("a gauge rewrite solves through the store");
    assert!(
        rewrite
            .cluster_rows()
            .iter()
            .any(|act| matches!(act, ClusterMaintenance::GaugeRewrite { from, to, .. } if *from == a && *to == b)),
        "{:?}",
        rewrite.maintenance
    );
    assert_eq!(
        counting.0.get(),
        admission + 2 * pairs * 2,
        "asks = pairs × 2 parts, again"
    );
}

/// **A saved document with a split replays bit-identically from its
/// recorded rows with no store**: the delete's `Split` row carries the
/// minted frame; `load` re-applies it and never solves.
#[test]
fn a6_a_saved_split_replays_bit_identically_with_no_store() {
    let (doc, [a, b], mate, opts, mut log) = seated("msolve6-a6-replay");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: mate }, Tol::witness(), &reach)
        .expect("the store's reach places the orphan");
    let split = applied.cluster_rows();
    assert!(
        matches!(split[..], [editor_core::ClusterMaintenance::Split { from, to, frame: Some(_) }] if from == a && to == b),
        "the orphan's frame is minted from the solved pose: {split:?}"
    );
    log.push(editor_core::LoggedEdit {
        edit: DocEdit::DeleteNode { id: mate },
        maintenance: split,
    });
    let live = applied.doc;
    assert!(live.placements().contains_key(&b), "the orphan has a row");
    // The file: an empty snapshot and the whole log, rows included.
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    // The wire shape, read as a value: EVERY entry is `{edit,
    // maintenance}`, both keys present. One that performed no
    // maintenance (the two instance inserts) carries an empty list;
    // one that did (the mate insert's Join, the delete's Split)
    // carries exactly its rows. There is no second, bare shape.
    let (header, body) = text.split_once('\n').expect("a header line");
    let value: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let entries = value["edits"].as_array().expect("an edit log");
    assert_eq!(entries.len(), log.len());
    let mut empty = 0;
    for (entry, logged) in entries.iter().zip(&log) {
        let keys: Vec<&String> = entry
            .as_object()
            .expect("an entry is an object")
            .keys()
            .collect();
        assert_eq!(
            keys,
            ["edit", "maintenance"],
            "one shape for every entry: {entry}"
        );
        assert_eq!(
            entry["maintenance"].as_array().map(Vec::len),
            Some(logged.maintenance.len()),
            "{entry}"
        );
        if logged.maintenance.is_empty() {
            empty += 1;
        }
    }
    assert_eq!(empty, 2, "the two instance inserts moved no cluster");
    // Every wire type refuses a field it does not know, a log entry
    // included.
    let mut tampered = value.clone();
    tampered["edits"][entries.len() - 1]["extra"] = serde_json::json!(1);
    let tampered = format!(
        "{header}\n{}\n",
        serde_json::to_string_pretty(&tampered).expect("re-serializes")
    );
    assert!(
        matches!(
            editor_core::load(&tampered, Tol::witness()),
            Err(PersistError::Unreadable { .. })
        ),
        "an unknown field on an entry refuses"
    );
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads with no store");
    assert_eq!(
        loaded.doc.placements(),
        live.placements(),
        "the registry replays bit for bit"
    );
    assert!(loaded.doc.bit_eq(&live), "and so does the document");
    let replayed = ProfileDoc::replay(live.id(), &log, Tol::witness()).expect("replays");
    assert_eq!(replayed.placements(), live.placements());
}

/// **A recorded row's frame is held to the placement door's rule at
/// load**: the log's rows re-enter the registry at replay without
/// passing `SetPlacement`, so a split row hand-edited into a
/// REFLECTION (an improper frame, determinant −1) refuses typed at
/// load naming the entry and the row, exactly as `save` refuses the
/// same log — never a mirror loaded into the registry that the door
/// would have refused.
#[test]
fn a6_a_recorded_row_whose_frame_is_a_mirror_refuses_at_load() {
    let (doc, [a, b], mate, opts, mut log) = seated("msolve6-a6-mirror");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: mate }, Tol::witness(), &reach)
        .expect("the delete applies");
    let index = log.len();
    log.push(LoggedEdit {
        edit: DocEdit::DeleteNode { id: mate },
        maintenance: applied.cluster_rows(),
    });
    let live = applied.doc;
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    assert!(editor_core::load(&text, Tol::witness()).is_ok());
    // The tamper: the split row's frame, mirrored across x.
    let mirror = Frame {
        columns: [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        translation: [0.0, 0.0, 0.0],
    };
    let (header, body) = text.split_once('\n').expect("a header line");
    let mut value: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let row = &mut value["edits"][index]["maintenance"][0]["Split"]["frame"];
    assert!(row.is_object(), "the split row carries a frame: {row}");
    *row = serde_json::to_value(mirror).expect("a frame serializes");
    let tampered = format!(
        "{header}\n{}\n",
        serde_json::to_string_pretty(&value).expect("re-serializes")
    );
    let err =
        editor_core::load(&tampered, Tol::witness()).expect_err("a mirror is not a placement");
    assert_eq!(
        err,
        PersistError::MaintenanceFrame {
            index,
            row: 0,
            fault: FrameFault::Improper { determinant: -1.0 },
        }
    );
    // The same log, through the save door: the shared validator.
    let mut mirrored = log.clone();
    mirrored[index].maintenance = vec![ClusterMaintenance::Split {
        from: a,
        to: b,
        frame: Some(mirror),
    }];
    assert_eq!(
        editor_core::save(&empty, &mirrored, Tol::witness()).expect_err("save refuses the same"),
        err
    );
    // And the door the rule is borrowed from says the same of it.
    assert!(matches!(
        live.apply(
            &DocEdit::SetPlacement {
                node: b,
                frame: mirror
            },
            Tol::witness(),
            &editor_core::RefusingReach
        ),
        Err(EditError::ImproperPlacement { node, determinant }) if node == b && determinant == -1.0
    ));
}

/// **A log entry that claims no maintenance its edit performed refuses
/// typed at load, and the bare pre-rows shape is not a format.**
///
/// Replay performs exactly an entry's rows, so an entry whose
/// `maintenance` is empty while its edit performs any refuses
/// `MaintenanceUnrecorded` — both kinds: a row that needs a solved
/// frame (the delete's, which moves a gauge), because replay never
/// solves; and a row replay could derive from the documents alone (the
/// mate insert's `Join`), because re-deriving it would give the log two
/// answers to what the edit did. A log in the bare shape — each entry
/// the edit itself, as files from before the rows existed carried — is
/// not read at all: it refuses `Unreadable` at the entry's first key,
/// instead of loading as a log of entries that performed nothing.
#[test]
fn a6_a_log_entry_that_drops_its_rows_refuses_at_load_and_the_bare_shape_is_not_a_format() {
    let (doc, [_a, b], mate, opts, mut log) = seated("msolve6-a6-dropped-rows");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: mate }, Tol::witness(), &reach)
        .expect("the delete applies");
    let rows = applied.cluster_rows();
    assert!(!rows.is_empty(), "the delete moved a gauge, so it has rows");
    log.push(editor_core::LoggedEdit {
        edit: DocEdit::DeleteNode { id: mate },
        maintenance: rows,
    });
    let live = applied.doc;
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    let (header, body) = text.split_once('\n').expect("a header line");
    let value: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let index = value["edits"].as_array().expect("an edit log").len() - 1;
    let reemit = |v: &serde_json::Value| {
        format!(
            "{header}\n{}\n",
            serde_json::to_string_pretty(v).expect("re-serializes")
        )
    };

    // The mate insert's join emptied: a row the documents alone would
    // derive, and still refused rather than re-derived.
    let join = 2;
    assert!(
        matches!(
            log[join].maintenance.as_slice(),
            [ClusterMaintenance::Join { absorbed, .. }] if *absorbed == b
        ),
        "the mate insert joined b's cluster: {:?}",
        log[join].maintenance
    );
    let mut joined = value.clone();
    joined["edits"][join]["maintenance"] = serde_json::json!([]);
    let err = editor_core::load(&reemit(&joined), Tol::witness()).expect_err("a join with no rows");
    assert!(
        matches!(
            &err,
            editor_core::PersistError::EditReplay {
                index: i,
                error: editor_core::EditError::MaintenanceUnrecorded { gauge }
            } if *i == join && *gauge == b
        ),
        "{err:?}"
    );

    // The delete's rows emptied, shape intact.
    let mut dropped = value.clone();
    dropped["edits"][index]["maintenance"] = serde_json::json!([]);
    let err = editor_core::load(&reemit(&dropped), Tol::witness())
        .expect_err("a moved gauge with no rows");
    assert!(
        matches!(
            &err,
            editor_core::PersistError::EditReplay {
                index: i,
                error: editor_core::EditError::MaintenanceUnrecorded { gauge }
            } if *i == index && *gauge == b
        ),
        "{err:?}"
    );

    // Every entry replaced by its bare edit: the shape files from before
    // the rows carried. Not a format this build reads — refused at the
    // first entry's edit tag, read as a field `LoggedEdit` does not have.
    let mut bare = value.clone();
    for entry in bare["edits"].as_array_mut().expect("an edit log") {
        let edit = entry
            .get("edit")
            .cloned()
            .expect("every saved entry carries its edit");
        *entry = edit;
    }
    let bare = reemit(&bare);
    assert!(
        !bare.contains("\"maintenance\""),
        "the tamper removed every row list"
    );
    let err = editor_core::load(&bare, Tol::witness()).expect_err("a bare entry");
    assert!(
        matches!(
            &err,
            editor_core::PersistError::Unreadable { detail, .. }
                if detail.contains("unknown field `InsertNode`")
        ),
        "a bare entry is not a log entry: {err:?}"
    );

    // And the file as written loads with no store in hand.
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads with no store");
    assert_eq!(loaded.doc.placements(), live.placements());
    assert_eq!(loaded.edits, log, "the log comes back as saved");
}

/// **C5, the checked-in corpus**: every TRACKED `.pncad` (git's
/// list — the viewer's suite reaches the editor-core corpus through a
/// symlink, which is not a second corpus) — the four the row names,
/// and it asserts that is what it walked — loads with no store in
/// hand and re-saves byte for byte. The re-save is structural rather
/// than lucky: `load` returns each entry as parsed, and replay performs
/// exactly an entry's rows — a non-empty list re-applied verbatim, an
/// empty one refused if its edit performs any — so the log that comes
/// back is both the one the file holds and the one its replay did.
#[test]
fn c5_every_checked_in_document_loads_with_no_store_and_re_saves_identically() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let listed = std::process::Command::new("git")
        .args(["ls-files", "-z", "--", "*.pncad"])
        .current_dir(&root)
        .output()
        .expect("git lists the tracked files");
    assert!(listed.status.success(), "{listed:?}");
    let mut walked: Vec<String> = String::from_utf8(listed.stdout)
        .expect("paths are utf-8")
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect();
    walked.sort();
    assert_eq!(
        walked,
        [
            "crates/editor-core/tests/corpus/die_tool.pncad",
            "crates/editor-core/tests/corpus/tour/die_composed_tour.pncad",
            "crates/pncad/tests/plate_param.pncad",
            "crates/viewer/tests/gallery_ring.pncad",
        ],
        "the corpus this row walks"
    );
    let found: Vec<std::path::PathBuf> = walked.iter().map(|p| root.join(p)).collect();
    let process = Tol::witness().eps();
    let mut loaded_count = 0;
    for path in &found {
        let text = std::fs::read_to_string(path).expect("readable");
        let loaded = match editor_core::load(&text, Tol::witness()) {
            Ok(loaded) => loaded,
            // D4's seam, before any replay: a document authored at
            // another ε refuses at THIS process's ε, typed. That is the
            // load door's own answer on the other ε rows and not this
            // row's subject.
            Err(editor_core::PersistError::ToleranceConflict {
                document,
                process: p,
            }) => {
                assert_ne!(
                    document,
                    p,
                    "{}: the seam refuses only a real conflict",
                    path.display()
                );
                assert_eq!(p, process);
                continue;
            }
            Err(e) => panic!("{}: loads with no store: {e}", path.display()),
        };
        loaded_count += 1;
        let again = editor_core::save(&loaded.snapshot, &loaded.edits, Tol::witness())
            .unwrap_or_else(|e| panic!("{}: re-saves: {e}", path.display()));
        assert_eq!(again, text, "{}: re-saves byte for byte", path.display());
    }
    if process == 1e-9 {
        assert_eq!(
            loaded_count,
            found.len(),
            "every checked-in document is authored at the default ε"
        );
    }
}

/// **At `Interval`, the door's reach is the bracket's `hi`, bit for
/// bit**: the one `f64` read the solve makes of a certified body is
/// the upper end of its reach bracket (an upper bound by definition),
/// it bounds the `f64` lane's reach, and the mate over it evaluates
/// `Ok` with the part evaluated once.
#[cfg(feature = "interval")]
#[test]
fn a5_at_interval_the_doors_reach_is_the_brackets_hi_bit_for_bit() {
    use geom_core::{Bounds, Interval};
    let (doc, ids, opts) = instances(
        "msolve6-a5-interval",
        cylinder_part("msolve6-a5-interval-part", 0.3, 0.7),
        2,
    );
    let doc_ref = match doc.node(ids[0]) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        other => panic!("an instance, not {other:?}"),
    };
    let (doc, mate) = mated(
        doc,
        &opts,
        clocked(
            ids[0],
            ids[1],
            coincidence(frame([0.0, 0.0, 0.7]), frame([0.0; 3]), 0.0),
        ),
    );
    let r64 = mate_reach::<f64>(&opts, Tol::witness())
        .reach(&doc_ref)
        .expect("bounded at f64");
    let ri = mate_reach::<Interval>(&opts, Tol::witness())
        .reach(&doc_ref)
        .expect("bounded at Interval");
    // The bracket, read off the certified body directly.
    let part = cylinder_part("msolve6-a5-interval-part", 0.3, 0.7);
    let evi = editor_core::evaluate::<Interval>(
        &part,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let body = editor_core::product(&part, &evi, Tol::witness()).expect("the interval product");
    let bracket = editor_core::mate::part_reach(&body).expect("bounded");
    assert_eq!(
        ri.to_bits(),
        bracket.hi().to_bits(),
        "the door reads the bracket's hi"
    );
    assert!(
        ri >= r64 && ri.is_finite(),
        "hi {ri} bounds the f64 reach {r64}"
    );
    let ev = editor_core::evaluate::<Interval>(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &opts,
        Tol::witness(),
    );
    assert!(
        matches!(ev.result(mate), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(mate)
    );
    assert_eq!(ev.part_evaluations, 1);
}

// ---- The correctness arm's probes, adopted ----

/// The unit cube `[0,1]³`.
fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
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

/// A part's evaluated body, through the ordinary doors.
fn body_of(part: &ProfileDoc) -> topo::Body<f64> {
    let ev = run(part, &EvalOptions::default());
    product(part, &ev, Tol::witness()).expect("the part's product")
}

/// The TRUE maximum distance from the origin over the body's rims:
/// line ends exactly, circles sampled densely. For a body of plane and
/// cylinder patches the interior maximum is on a rim (convexity), so
/// this is the body's true reach up to the sampling.
fn true_reach(body: &topo::Body<f64>) -> (f64, usize) {
    let origin = Point3::new(0.0, 0.0, 0.0);
    let (mut best, mut circles) = (0.0_f64, 0usize);
    for (_, face) in body.faces() {
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let (edges, _) = topo::props::loop_edges(body, lk).expect("the loop walks");
            for e in &edges {
                match &e.carrier {
                    geom::Curve3::Line { .. } => {
                        for t in [e.t0, e.t1] {
                            best = best.max((e.carrier.eval(t) - origin).norm());
                        }
                    }
                    geom::Curve3::Circle { .. } => {
                        circles += 1;
                        let n = 4096;
                        for i in 0..=n {
                            let t = e.t0 + (e.t1 - e.t0) * f64::from(i) / f64::from(n);
                            best = best.max((e.carrier.eval(t) - origin).norm());
                        }
                    }
                    other => panic!("unexpected carrier {other:?}"),
                }
            }
        }
    }
    (best, circles)
}

fn coaxial(fa: MateFrame, fb: MateFrame) -> Alignment {
    Alignment {
        a: fa,
        b: fb,
        primitive: MatePrimitive::Coaxial,
        sense: AxisSense::Aligned,
        clocking: Some(0.0),
    }
}

/// **The reach bounds the TRUE maximum over every fixture part** —
/// boxes at three scales, the unit block, two cylinders — measured
/// against the rims themselves (line ends exactly, circles sampled),
/// against the analytic far corner or rim, and through the public
/// door bit for bit; and for an all-line body the bound IS the truth.
#[test]
fn a2_the_reach_bounds_the_true_maximum_over_every_fixture_part() {
    let cases: Vec<(&str, ProfileDoc, f64)> = vec![
        (
            "box 0.5x1",
            box_part("msolve6-p1-box", 0.5, 1.0),
            (2.0_f64 * 0.25 + 1.0).sqrt(),
        ),
        ("block [0,1]^3", block("msolve6-p1-block"), 3.0_f64.sqrt()),
        (
            "box 10mm",
            box_part("msolve6-p1-10mm", 0.005, 0.01),
            (2.0_f64 * 0.005 * 0.005 + 0.01 * 0.01).sqrt(),
        ),
        (
            "box 10m",
            box_part("msolve6-p1-10m", 5.0, 10.0),
            (2.0_f64 * 25.0 + 100.0).sqrt(),
        ),
        (
            "cyl r0.3 h0.7",
            cylinder_part("msolve6-p1-cyl-a", 0.3, 0.7),
            (0.09_f64 + 0.49).sqrt(),
        ),
        (
            "cyl r0.7 h0.3",
            cylinder_part("msolve6-p1-cyl-b", 0.7, 0.3),
            (0.49_f64 + 0.09).sqrt(),
        ),
    ];
    for (name, part, analytic) in cases {
        let body = body_of(&part);
        let (truth, circles) = true_reach(&body);
        let bound = editor_core::mate::part_reach(&body).expect("bounded");
        let (doc, ids, opts) = instances(&format!("msolve6-p1-asm-{name}"), part.clone(), 1);
        let via_door = reaches(&doc, &opts, &ids)[0];
        assert!(bound >= truth, "{name}: bound {bound} < true reach {truth}");
        assert!(
            bound >= analytic - 1e-15,
            "{name}: bound {bound} < analytic {analytic}"
        );
        assert_eq!(
            via_door.to_bits(),
            bound.to_bits(),
            "{name}: the door is the bound"
        );
        if circles == 0 {
            assert_eq!(bound, truth, "{name}: all-line rims are exact");
        }
    }
}

/// **The reach fold propagates a poisoned face rather than dropping
/// it**: `body_reach` folds with `Real::max`, which is NaN-propagating
/// — where the inherent `f64::max` would drop a poisoned face's bound
/// and under-estimate silently. Pinned on the two functions, since
/// no document door builds a poisoned face.
#[test]
fn a2_the_reach_fold_propagates_a_poisoned_face_rather_than_dropping_it() {
    use geom_core::Real;
    assert!(Real::max(f64::NAN, 1.0).is_nan());
    assert!(Real::max(1.0, f64::NAN).is_nan());
    assert!(
        !f64::max(1.0, f64::NAN).is_nan(),
        "std's max drops NaN; the fold must not use it"
    );
}

/// Three instances of one box, `a` placed at a translation so the
/// orphan's inherited frame is recognisable.
fn trio(label: &str) -> (ProfileDoc, [RecipeNodeId; 3], EvalOptions, Frame) {
    let (doc, ids, opts) = instances(label, box_part(&format!("{label}-part"), 0.5, 1.0), 3);
    let f_a = Frame::translation([1.0, 2.0, 3.0]);
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: ids[0],
            frame: f_a,
        },
    );
    (doc, [ids[0], ids[1], ids[2]], opts, f_a)
}

/// **A contradictory prior records the split with the cluster's
/// frame**: the prior solve DECIDED the cluster has no pose, so
/// deleting the mate — the recourse the refusal names — splits the
/// orphan off carrying the cluster's recorded frame, not a re-solved
/// pose and not a refusal.
#[test]
fn a6_a_contradictory_prior_records_the_split_with_the_clusters_frame() {
    let (doc, [a, b, c], opts, f_a) = trio("msolve6-p3a");
    let (doc, _m1) = mated(
        doc,
        &opts,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    // A contradiction against ANOTHER mate is the pair's verdict, not
    // the mate's own: the door admits it and the solve refuses it.
    let (doc, _m2) = mated(
        doc,
        &opts,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 2.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let (doc, m3) = mated(
        doc,
        &opts,
        clocked(
            b,
            c,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(matches!(
        poses.fault(c),
        Some(MateFault::Contradictory { .. })
    ));
    assert_eq!(poses.relative(c), None);
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: m3 }, Tol::witness(), &reach)
        .expect("deleting a mate of a contradictory cluster is its recourse");
    assert_eq!(
        applied.cluster_rows(),
        vec![ClusterMaintenance::Split {
            from: a,
            to: c,
            frame: Some(f_a)
        }]
    );
    assert!(applied.doc.placements()[&c].bit_eq(&f_a));
}

/// **An under-determined prior records the split with the cluster's
/// frame** — the same recourse, the same row.
#[test]
fn a6_an_under_determined_prior_records_the_split_with_the_clusters_frame() {
    let (doc, [a, b, _c], opts, f_a) = trio("msolve6-p3b");
    let (doc, m) = insert(
        doc,
        clocked(a, b, coaxial(frame([0.0; 3]), frame([0.0; 3]))),
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert!(matches!(poses.fault(b), Some(MateFault::Under { .. })));
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: m }, Tol::witness(), &reach)
        .expect("deleting an under-determined mate is its recourse");
    assert_eq!(
        applied.cluster_rows(),
        vec![ClusterMaintenance::Split {
            from: a,
            to: b,
            frame: Some(f_a)
        }]
    );
}

/// **An indeterminate prior refuses the edit typed**: a tilt priced
/// inside the band leaves the solve with NO verdict, so an edit that
/// moves that cluster's gauge refuses carrying the fault — and the
/// same edit through the refusing reach refuses `Unleverable` in the
/// resolver's voice.
///
/// The rider is decided at the door over the parts as they are when
/// the mate is authored, so an in-band rider cannot be inserted: it
/// is authored over a SMALL part, where the same tilt is redundant,
/// and the parts are then re-pinned to a large version of the same
/// document under which the tilt lands in the band — the road a
/// verdict moves by after insert (the memo row's).
#[test]
fn a6_an_indeterminate_prior_refuses_the_edit_typed() {
    let small = box_part("msolve6-p3c-part", 0.005, 0.01);
    let large = box_part("msolve6-p3c-part", 5.0, 10.0);
    let large_pin = content_pin(&large, Tol::witness()).unwrap();
    let mut store_small = PartStore::new();
    let small_ref = store_small.insert(small, Tol::witness());
    let mut store_large = PartStore::new();
    store_large.insert(large, Tol::witness());
    let opts_small = with_resolver(store_small);
    let opts_large = with_resolver(store_large);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p3c"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(small_ref));
    let (doc, b) = insert(doc, Node::instantiate_part(small_ref));
    let (doc, c) = insert(doc, Node::instantiate_part(small_ref));
    let (doc, _) = step(
        doc,
        DocEdit::SetPlacement {
            node: a,
            frame: Frame::translation([1.0, 2.0, 3.0]),
        },
    );
    let band = Band::linear(Tol::witness()).expect("band");
    let datum = datum_lever(&coincidence(frame([0.0, 0.0, 0.01]), frame([0.0; 3]), 0.0));
    let r_large = {
        let (re, _) = step(
            doc.clone(),
            DocEdit::UpdateReference {
                node: a,
                new_pin: large_pin,
            },
        );
        reaches(&re, &opts_large, &[a])[0]
    };
    let r_small = reaches(&doc, &opts_small, &[a])[0];
    let theta = ((band.zero() + band.escalate()) / 2.0) / (r_large + r_large + datum);
    assert!(
        theta * (r_small + r_small + datum) < band.zero(),
        "over the small part the same tilt is redundant, so the door admits it"
    );
    let (doc, _m1) = mated(
        doc,
        &opts_small,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 0.01]), frame([0.0; 3]), theta),
        ),
    );
    let (doc, m2) = mated(
        doc,
        &opts_small,
        clocked(
            b,
            c,
            coincidence(frame([0.0, 0.0, 0.01]), frame([0.0; 3]), 0.0),
        ),
    );
    let mut doc = doc;
    for node in [a, b, c] {
        let (re, _) = step(
            doc,
            DocEdit::UpdateReference {
                node,
                new_pin: large_pin,
            },
        );
        doc = re;
    }
    let poses = solve(&doc, &opts_large, Tol::witness());
    assert!(matches!(
        poses.fault(c),
        Some(MateFault::Indeterminate { .. })
    ));
    let reach = mate_reach::<f64>(&opts_large, Tol::witness());
    let err = doc
        .apply(&DocEdit::DeleteNode { id: m2 }, Tol::witness(), &reach)
        .expect_err("no verdict, no frame");
    assert!(matches!(
        &err,
        EditError::MaintenanceRefused { gauge, fault: Some(f) }
            if *gauge == c && matches!(**f, MateFault::Indeterminate { .. })
    ));
    let err = doc
        .apply(
            &DocEdit::DeleteNode { id: m2 },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect_err("no parts, no frame");
    assert!(matches!(
        &err,
        EditError::MaintenanceRefused { gauge, fault: Some(f) }
            if *gauge == c && matches!(**f, MateFault::Unleverable {
                refusal: LeverRefusal::PartUnresolved { fault: PartFault::NoResolver, .. },
                ..
            })
    ));
}

/// **A logged edit has one wire shape, and its rows round-trip**: an
/// entry with no rows and an entry with rows both carry `edit` and
/// `maintenance`, and each reads back equal. The bare edit is refused
/// as an entry.
#[test]
fn a6_a_logged_edit_has_one_wire_shape_and_its_rows_round_trip() {
    let edit = DocEdit::<editor_core::ProfileProgram>::SetPlacement {
        node: RecipeNodeId(0),
        frame: Frame::translation([1.0, 2.0, 3.0]),
    };
    let shape = |text: &str| -> Vec<String> {
        let value: serde_json::Value = serde_json::from_str(text).unwrap();
        value.as_object().unwrap().keys().cloned().collect()
    };
    let none = serde_json::to_string(&LoggedEdit::bare(edit.clone())).unwrap();
    assert_eq!(shape(&none), ["edit", "maintenance"], "{none}");
    let back: LoggedEdit<editor_core::ProfileProgram> = serde_json::from_str(&none).unwrap();
    assert_eq!(back, LoggedEdit::bare(edit.clone()));
    let bare_edit = serde_json::to_string(&edit).unwrap();
    assert!(
        serde_json::from_str::<LoggedEdit<editor_core::ProfileProgram>>(&bare_edit).is_err(),
        "the edit alone is not an entry: {bare_edit}"
    );
    let with = LoggedEdit {
        edit,
        maintenance: vec![ClusterMaintenance::Split {
            from: RecipeNodeId(0),
            to: RecipeNodeId(1),
            frame: Some(Frame::translation([0.0, 0.0, 5.0])),
        }],
    };
    let text = serde_json::to_string(&with).unwrap();
    assert_eq!(shape(&text), ["edit", "maintenance"]);
    let back: LoggedEdit<editor_core::ProfileProgram> = serde_json::from_str(&text).unwrap();
    assert_eq!(back, with);
}

/// **A whole-cluster split levers through the part it is minting**
/// (the `WithPart` resolver composed with the caller's), and with no
/// resolver at all refuses typed — `Unresolved` on the new instance,
/// in the resolver's own voice, never a frame nothing decided.
#[test]
fn a6_a_split_levers_through_the_part_in_hand_and_refuses_typed_without_a_resolver() {
    let (doc, ids, opts) = instances("msolve6-p6", box_part("msolve6-p6-part", 0.5, 1.0), 2);
    let [a, b] = [ids[0], ids[1]];
    let (doc, _m) = mated(
        doc,
        &opts,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let cut = [a, b].into_iter().collect();
    split(
        &doc,
        &cut,
        DocumentId::derive("msolve6-p6-new-part"),
        Tol::witness(),
        opts.resolver.as_ref(),
    )
    .expect("a whole-cluster cut splits through the part in hand");
    let none = split(
        &doc,
        &cut,
        DocumentId::derive("msolve6-p6-new-part-none"),
        Tol::witness(),
        None,
    );
    match none {
        Err(SplitError::RemainderEdit { error }) => assert!(
            matches!(
                *error,
                EditError::MaintenanceRefused { fault: Some(ref f), .. }
                    if matches!(**f, MateFault::Unleverable {
                        refusal: LeverRefusal::PartUnresolved {
                            fault: PartFault::Unresolved { fault: ResolveFault::Unresolved, .. },
                            ..
                        },
                        ..
                    })
            ),
            "typed Unresolved expected, got {error:?}"
        ),
        other => panic!("a no-resolver split refuses its remainder edit, got {other:?}"),
    }
}

/// **Two mated parts evaluate once each**: a chain over two distinct
/// parts (three instances, two mates) evaluates each part exactly
/// once — the lever's asks share the run's cache with the
/// instantiate nodes — and every node evaluates `Ok`.
#[test]
fn a5_two_mated_parts_evaluate_once_each() {
    let mut store = PartStore::new();
    let ra = store.insert(box_part("msolve6-p8-a", 0.5, 1.0), Tol::witness());
    let rb = store.insert(block("msolve6-p8-b"), Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p8"), Tol::witness());
    let (doc, a) = insert(doc, Node::instantiate_part(ra));
    let (doc, b) = insert(doc, Node::instantiate_part(rb));
    let (doc, c) = insert(doc, Node::instantiate_part(ra));
    assert_eq!(run(&doc, &opts).part_evaluations, 2);
    let (doc, _) = mated(
        doc,
        &opts,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let (doc, _) = mated(
        doc,
        &opts,
        clocked(
            b,
            c,
            coincidence(frame([0.0, 0.0, 1.0]), frame([0.0; 3]), 0.0),
        ),
    );
    let ev = run(&doc, &opts);
    assert_eq!(ev.part_evaluations, 2);
    for id in [a, b, c] {
        assert!(
            matches!(ev.result(id), Some(NodeResult::Ok(_))),
            "{:?}",
            ev.result(id)
        );
    }
}
