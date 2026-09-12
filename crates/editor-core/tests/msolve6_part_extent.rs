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
    PartFault, ProfileDoc, ReachRefusal, RecipeNodeId, ResolveFault, SitedRef, content_pin,
    mate_reach,
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
    // The same answer through the door the solve reads.
    assert_eq!(
        editor_core::mate::part_reach(&body).err(),
        Some(ReachRefusal::FaceUnbounded {
            face: made.face,
            kind: "nurbs"
        })
    );
    let instance = RecipeNodeId(7);
    let part = editor_core::DocRef {
        id: DocumentId::derive("msolve6-a4-unbounded"),
        pin: content_pin(&box_part("msolve6-a4-unbounded", 0.5, 1.0), Tol::witness()).unwrap(),
    };
    let lever = LeverRefusal::of(ReachRefusal::from(refusal), instance, part);
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

// ---- The edit door (the spec's amendment): the reach, the log, replay ----

/// A reach that counts its asks and answers the refusing reach's
/// answer — the witness that an edit which moves no gauge never
/// consults the store.
struct Counting(core::cell::Cell<usize>);

impl MateReach for Counting {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        editor_core::RefusingReach.reach(part)
    }
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
            maintenance: applied.maintenance,
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
    let (doc, mate) = insert(
        doc,
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
    assert!(err.to_string().contains("could not place gauge"), "{err}");
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

/// **An edit that moves no gauge never asks the reach**: on a mated
/// document, a second mate (a Join — the survivor keeps its gauge), a
/// placement, and an appearance-free parameter edit all succeed
/// through a reach that would refuse if asked, and the counter says
/// it never was. Deleting the mate — a Split — asks it.
#[test]
fn a6_a_gauge_preserving_edit_never_asks_the_reach() {
    let (doc, [a, b], mate, _opts, _log) = seated("msolve6-a6-preserving");
    let counting = Counting(core::cell::Cell::new(0));
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
        .expect("a join asks nothing");
    assert!(
        matches!(applied.maintenance[..], [editor_core::ClusterMaintenance::Join { survived, absorbed, .. }] if survived == a && absorbed == c),
        "{:?}",
        applied.maintenance
    );
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
    assert_eq!(
        counting.0.get(),
        0,
        "no gauge moved, so the store was never consulted"
    );
    // Deleting the pair's mate splits the cluster and asks — and the
    // refusing reach it wraps refuses, typed.
    let err = doc
        .apply(&DocEdit::DeleteNode { id: mate }, tol, &counting)
        .expect_err("a split needs the parts");
    assert!(
        matches!(err, editor_core::EditError::MaintenanceRefused { .. }),
        "{err:?}"
    );
    assert!(counting.0.get() >= 1, "the split asked the reach");
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
    let split = applied.maintenance.clone();
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
    assert!(
        text.contains("\"maintenance\""),
        "the split's rows are on the wire"
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
    // An entry that performed no maintenance is the bare edit on the
    // wire: the common case costs nothing.
    assert!(!text.contains("\"maintenance\": []"));
}

/// **An old-format log whose edit moved a gauge refuses typed at load,
/// and migrates through the door that takes a reach.**
#[test]
fn a6_an_old_format_log_that_moved_a_gauge_refuses_at_load_and_migrates() {
    let (doc, [_a, b], mate, opts, mut log) = seated("msolve6-a6-migrate");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let applied = doc
        .apply(&DocEdit::DeleteNode { id: mate }, Tol::witness(), &reach)
        .expect("the delete applies");
    let live = applied.doc;
    log.push(editor_core::LoggedEdit {
        edit: DocEdit::DeleteNode { id: mate },
        maintenance: applied.maintenance,
    });
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    // The old format: every entry a bare edit. Built from the saved
    // body by dropping each entry's rows — the shape a file from
    // before the rows were recorded has.
    let (header, body) = text.split_once('\n').expect("a header line");
    let mut body: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    let entries = body["edits"].as_array_mut().expect("an edit log");
    let index = entries.len() - 1;
    for entry in entries.iter_mut() {
        if let Some(edit) = entry.get("edit").cloned() {
            *entry = edit;
        }
    }
    let old = format!(
        "{header}\n{}\n",
        serde_json::to_string_pretty(&body).expect("re-serializes")
    );
    assert!(!old.contains("\"maintenance\""));
    let err = editor_core::load(&old, Tol::witness()).expect_err("a moved gauge with no rows");
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
    assert!(err.to_string().contains("migrate"), "{err}");
    // The migration door: the same file, the store's reach, the rows
    // re-derived — and re-saved, it loads with no store.
    let migrated = editor_core::load_with(&old, Tol::witness(), &reach).expect("migrates");
    assert_eq!(migrated.doc.placements(), live.placements());
    assert!(
        !migrated.edits[index].maintenance.is_empty(),
        "the rows are back"
    );
    let text2 =
        editor_core::save(&migrated.snapshot, &migrated.edits, Tol::witness()).expect("re-saves");
    assert_eq!(
        text2, text,
        "the migrated file is the file the live door would have written"
    );
    let again = editor_core::load(&text2, Tol::witness()).expect("loads with no store");
    assert_eq!(again.doc.placements(), live.placements());
}

/// **C5, the checked-in corpus**: every `.pncad` in the repository
/// loads with no store in hand and re-saves byte for byte — no logged
/// edit of any of them moved a gauge, so none needed migrating.
#[test]
fn c5_every_checked_in_document_loads_with_no_store_and_re_saves_identically() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut found = Vec::new();
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if path.is_dir() {
                if name == "target" || name.starts_with('.') || name == "node_modules" {
                    continue;
                }
                walk(&path, out);
            } else if name.ends_with(".pncad") {
                out.push(path);
            }
        }
    }
    walk(&root, &mut found);
    assert!(found.len() >= 4, "the corpus has documents: {found:?}");
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
