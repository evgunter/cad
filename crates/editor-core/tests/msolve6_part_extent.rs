//! MSOLVE-6 acceptance — **the mate's lever is the mated parts' own
//! extent**, taken from each part's evaluated body (spec
//! `docs/MSOLVE-6-SPEC.md`, rows A2–A5).
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

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, DocEdit, DocumentId, EvalOptions, LeverRefusal,
    MateFault, MateFrame, MatePrimitive, MateReach, MateRole, Node, NodeErrorKind, NodeResult,
    PartFault, ProfileDoc, RecipeNodeId, ResolveFault, SitedRef, content_pin, mate_reach,
};
use fixture::resolver::{PartStore, in_part};
use fixture::{ang, axis_in_plane, insert, len, on_frame, on_frame_keeping, run, solve, step};
use geom_core::predicate::Band;
use geom_core::{Point3, Tol};

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
    MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    }
}

/// A frame coincidence between `a`'s top cap and `b`'s bottom cap,
/// with a clocking rider: on a coincidence the rider is
/// redundant-or-contradictory, DECIDED at the lever, so it is the one
/// arm that reports `lever: Some((θ, L))` — the row's window onto `L`.
fn clocked(
    a: RecipeNodeId,
    b: RecipeNodeId,
    alignment: Alignment,
) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a: SitedRef::at_mint(in_part(a, CapEnd::End)),
        b: SitedRef::at_mint(in_part(b, CapEnd::Start)),
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

/// The reach of every instance in `ids`, through the public door.
fn reaches(doc: &ProfileDoc, opts: &EvalOptions, ids: &[RecipeNodeId]) -> Vec<f64> {
    let reach = mate_reach::<f64>(doc, opts, Tol::witness());
    ids.iter()
        .map(|&id| {
            reach
                .reach(id)
                .expect("the part resolves and its body is bounded")
        })
        .collect()
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
    let (doc, mate) = insert(doc, clocked(ids[0], ids[1], alignment));
    let poses = solve(&doc, &opts, Tol::witness());
    let fault = poses
        .fault(mate)
        .expect("a quarter-turn rider contradicts the coincidence");
    let MateFault::Contradictory {
        lever: Some((theta, arm)),
        ..
    } = fault
    else {
        panic!("expected CONTRADICTORY with a lever, got {fault:?}");
    };
    assert_eq!(*theta, core::f64::consts::FRAC_PI_2);
    let r = reaches(&doc, &opts, &ids);
    let expected = (r[0] + r[1]) + alignment.lever_arm();
    assert_eq!(*arm, expected, "the lever is the formula, bit for bit");
    // And the datum's own term is what the formula says it is.
    assert_eq!(alignment.lever_arm(), 0.1_f64.hypot(1.0) + 0.2);
}

// ---- A3: the lever decides at the parts' scale ----

/// What the run's own band says a levered tilt decides to.
#[derive(Debug, PartialEq)]
enum Verdict {
    Parallel,
    Refused,
    Indeterminate,
}

fn verdict(band: Band, theta: f64, arm: f64) -> Verdict {
    let margin = theta * arm;
    if margin <= band.zero() {
        Verdict::Parallel
    } else if margin >= band.escalate() {
        Verdict::Refused
    } else {
        Verdict::Indeterminate
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
    let (doc, mate) = insert(doc, clocked(ids[0], ids[1], alignment));
    let r = reaches(&doc, &opts, &ids);
    let arm = (r[0] + r[1]) + alignment.lever_arm();
    let band = Band::linear(Tol::witness()).expect("the band");
    let poses = solve(&doc, &opts, Tol::witness());
    let fault = poses.fault(mate).cloned();
    let found = match &fault {
        None => {
            assert_eq!(poses.role(mate), Some(MateRole::Determining));
            Verdict::Parallel
        }
        Some(MateFault::Contradictory {
            predicate,
            lever: Some((t, l)),
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
    let (doc, mate) = insert(
        doc,
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
    assert_eq!(refusal.face, made.face);
    assert_eq!(refusal.kind, "nurbs");
    let instance = RecipeNodeId(7);
    let part = editor_core::DocRef {
        id: DocumentId::derive("msolve6-a4-unbounded"),
        pin: content_pin(&box_part("msolve6-a4-unbounded", 0.5, 1.0), Tol::witness()).unwrap(),
    };
    let lever = refusal.into_lever(instance, part);
    assert_eq!(
        lever,
        LeverRefusal::FaceUnbounded {
            instance,
            part,
            face: made.face,
            kind: "nurbs",
        }
    );
    let text = lever.to_string();
    assert!(
        text.contains("nurbs") && text.contains("instance 7"),
        "{text}"
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
    let (doc, mate) = insert(
        doc,
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
    let (doc, mate) = insert(
        doc,
        clocked(
            a,
            b,
            coincidence(frame([0.0, 0.0, 0.01]), frame([0.0; 3]), theta),
        ),
    );
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
    let first = run(&doc, &opts_small);
    let key_small = first.value(mate).map(|v| v.content_key);
    assert_eq!(
        key_small.is_some(),
        small_verdict == Verdict::Parallel,
        "the small part's mate: {:?}",
        first.result(mate)
    );
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
            assert_eq!(key_small, key_large, "no flip, no key move");
        }
    }
}
