//! MSOLVE-6 acceptance — **a bundle's lever is the copy's own
//! extent**, taken from the copied shape's evaluated body.
//!
//! The lever a bundle's angular decisions turn on is
//! `R + max over its mates of (‖on.origin‖ + ‖to.origin‖)`, `R` an
//! upper bound on the copied shape's reach from its own origin and
//! each pose's origin its distance from its coordinates' origin, with
//! no floor and no constant. These rows measure the reach against
//! bodies whose true extent is known, pin the lever to the bit, decide
//! the measure site's own example at the copy's scale (at whatever ε
//! the run is at — the expectation is derived from the run's own
//! `Band`), and pin the refusals and the exactly-once evaluation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::Formula;

use std::sync::Arc;

use editor_core::mate::SurfaceKind;
use editor_core::pose::{PoseRole, RoleRead};
use editor_core::{
    CapEnd, Clash, ContactClass, Coord, DocEdit, DocumentId, EvalOptions, Evaluation, Frame, Mate,
    MateFault, MateReach, MateRef, MateSide, Node, NodeErrorKind, NodeResult, OperandSlot,
    PartFault, PartReach, PoseValue, ProfileDoc, ReachRefusal, RecipeNodeId, ResolveFault,
    SlideAxis, Target, Value, WorldRead, content_pin, mate_reach, product,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, insert, len, on_frame, revolve_about, run, solve, step};
use geom_core::predicate::{Band, Sign};
use geom_core::{Decide, Point3, Tol};

// ---- Substrate ----

/// **`doc` with `body` placed in its world**, as
/// [`PartStore::insert_part`] places it — so [`in_part`] spells the
/// part's caps through that placement wherever the part is stored.
fn placed_part(doc: ProfileDoc, body: RecipeNodeId) -> ProfileDoc {
    let mut scratch = PartStore::new();
    let (doc_ref, _) = scratch.insert_part((doc, body), Tol::witness());
    scratch.doc(doc_ref.id)
}

/// A box part: a square of half-side `half` at the origin, extruded
/// `height` along +z. Its farthest point from the part origin is the
/// top corner, `sqrt(2 half² + height²)` away. The body is placed in
/// the part's world ([`placed_part`]).
fn box_part(label: &str, half: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    );
    placed_part(doc, body)
}

/// `part` (a [`box_part`]) re-valued in place: its square's half-width
/// and its extrude's height edited, every id kept — a later version of
/// the same document.
fn resized(part: ProfileDoc, half: f64, height: f64) -> ProfileDoc {
    let profile = part
        .ids()
        .iter()
        .copied()
        .find(|&id| matches!(part.node(id), Some(Node::Profile(_))))
        .expect("a box part has one profile");
    let Some(Node::Profile(program)) = part.node(profile) else {
        unreachable!("found as a profile")
    };
    let ids = program.kept_in_place();
    let plane = part
        .operation_of(program.frame)
        .expect("the plane read is live");
    let loops = fixture::desc(plane, vec![fixture::square(0.0, 0.0, half)]).loops;
    let body = body_node(&part);
    let (part, _) = fixture::step(
        part,
        DocEdit::SetProgram {
            node: profile,
            loops,
            ids,
            fresh: Vec::new(),
        },
    );
    let (part, _) = fixture::step(
        part,
        DocEdit::SetParam {
            node: body,
            slot: editor_core::SlotId::Distance,
            value: len(height).into(),
            fresh: Vec::new(),
        },
    );
    part
}

/// A cylinder part: a rectangle `radius × height` in the xy plane,
/// revolved a full turn about the plane's +y through the origin. The
/// cylinder stands on the origin along +y; its farthest point from
/// the origin is the top rim, `sqrt(radius² + height²)` away. The body
/// is placed in the part's world ([`placed_part`]).
fn cylinder_part(label: &str, radius: f64, height: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
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
    let (doc, body) = insert(
        doc,
        revolve_about(
            profile,
            (0.0, 0.0),
            crate::fixture::along_y(),
            ang(std::f64::consts::TAU),
        ),
    );
    placed_part(doc, body)
}

/// **The node a part's caps are named on**: its one extrude or
/// revolve, found by kind.
///
/// # Panics
///
/// If `part` has no such node, or more than one.
fn body_node(part: &ProfileDoc) -> RecipeNodeId {
    let solids: Vec<RecipeNodeId> = part
        .ids()
        .iter()
        .copied()
        .filter(|&id| {
            matches!(
                part.node(id),
                Some(Node::Extrude { .. } | Node::Revolve { .. })
            )
        })
        .collect();
    let [body] = solids[..] else {
        panic!("a part here has one body node, not {solids:?}");
    };
    body
}

/// The reach of every instance in `ids`, through the public door:
/// each instance's part, read off the document the way the solve
/// reads it.
fn reaches(doc: &editor_core::ProfileDoc, opts: &EvalOptions, ids: &[RecipeNodeId]) -> Vec<f64> {
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

/// `n` instances of `part` — the first pinned to the world at the
/// identity, the rest with empty bundles, placed by the mates a row
/// adds to them — the options that resolve them, and the part's
/// [`body_node`].
fn instances(
    label: &str,
    part: ProfileDoc,
    n: usize,
) -> (ProfileDoc, Vec<RecipeNodeId>, EvalOptions, RecipeNodeId) {
    let body = body_node(&part);
    let mut store = PartStore::new();
    let doc_ref = store.insert(part, Tol::witness());
    let opts = EvalOptions {
        resolver: Some(Arc::new(store)),
        ..EvalOptions::default()
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for i in 0..n {
        let (next, id) = if i == 0 {
            fixture::insert_pinned(doc, Node::instantiate_part(doc_ref), &opts)
        } else {
            insert(doc, fixture::mated_instance(doc_ref))
        };
        doc = next;
        ids.push(id);
    }
    (doc, ids, opts, body)
}

/// `doc` with the freedom `coord` of `mate` set to `by`.
fn with_value(doc: ProfileDoc, mate: MateRef, coord: Coord, by: Formula) -> ProfileDoc {
    editor_core::apply(
        &doc,
        &DocEdit::AddValue {
            placement: mate.placement,
            value: Box::new(Value {
                mate: mate.id,
                coord,
                by,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .unwrap_or_else(|e| panic!("the value is admitted: {e}"))
    .doc
}

/// **`upper` seated flush on `lower`'s top cap**: `upper`'s bottom cap
/// resting on it, and its three values zero — pinned in `lower`'s
/// space. Each side is an instance of the part whose body is the
/// paired [`body_node`]. Answers the rest's address.
fn seat(
    doc: ProfileDoc,
    (lower, lower_body): (RecipeNodeId, RecipeNodeId),
    (upper, upper_body): (RecipeNodeId, RecipeNodeId),
) -> (ProfileDoc, MateRef) {
    let (doc, rest) = fixture::constrain_at(
        doc,
        upper,
        fixture::rest(
            fixture::head(in_part(upper, upper_body, CapEnd::Start)),
            fixture::head(in_part(lower, lower_body, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let doc = [
        (Coord::Slide(SlideAxis::X), len(0.0)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, ang(0.0)),
    ]
    .into_iter()
    .fold(doc, |doc, (coord, by)| with_value(doc, rest, coord, by));
    (doc, rest)
}

/// The world's `xy` plane, facing down: what a bottom cap rests on.
fn floor() -> WorldRead<Formula> {
    WorldRead::through(PoseRole::Xy).flip()
}

/// [`floor`] turned `theta` about the world's `x` axis. Its origin
/// stays the world's, exactly: a turn about a line through the origin
/// moves no point of it.
fn tilted_floor(theta: f64) -> WorldRead<Formula> {
    floor().spin(WorldRead::through(PoseRole::X), ang(theta))
}

/// A document holding one box body — a square of half-side `half` at
/// the origin, extruded `height` along +z — and a loose copy of it,
/// whose bottom cap rests on [`floor`] and on [`tilted_floor`]`(theta)`:
/// two plane mates whose targets differ by `theta`, which the fold
/// decides over the bundle's lever. Answers the document, the body,
/// the copy and the two mates.
fn tilted_copy(label: &str, half: f64, height: f64, theta: f64) -> TiltedCopy {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, half)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
        },
    );
    let (doc, copy) = insert(
        doc,
        Node::place(vec![editor_core::BodyRead::plain(
            editor_core::Operand::Node(body),
        )]),
    );
    let rest_on = |to| Mate {
        on: fixture::plane(fixture::cap_ref(body, CapEnd::Start)),
        to: Target::World(to),
        class: ContactClass::Rest,
    };
    let (doc, first) = fixture::constrain_at(doc, copy, rest_on(floor()));
    let (doc, second) = fixture::constrain_at(doc, copy, rest_on(tilted_floor(theta)));
    TiltedCopy {
        doc,
        body,
        copy,
        mates: [first, second],
    }
}

struct TiltedCopy {
    doc: ProfileDoc,
    body: RecipeNodeId,
    copy: RecipeNodeId,
    mates: [MateRef; 2],
}

impl TiltedCopy {
    /// **The bundle's lever, as the formula spells it**, read off the
    /// document's own evaluation: the copied body's reach
    /// (`mate::part_reach`), plus the farthest any mate's two poses sit
    /// from their origins — here the copy's bottom-cap plane, bound
    /// where the fold binds it, and the world's two planes, which sit
    /// on the origin. Grouped as the fold groups it.
    fn lever(&self, ev: &Evaluation<f64>) -> f64 {
        let Some(editor_core::ValuePayload::Body(body)) = ev.value(self.body).map(|v| &v.payload)
        else {
            panic!("the box evaluates to a body");
        };
        let reach = editor_core::mate::part_reach(&**body).expect("the box is bounded");
        let from_origin = |m: MateRef| {
            let on = fixture::constraint(&self.doc, m).on;
            let pose = editor_core::eval::bound_pose(
                &self.doc,
                ev,
                OperandSlot::Side {
                    constraint: m.id,
                    side: MateSide::On,
                },
                RoleRead::plain(on),
                Tol::witness(),
            )
            .expect("the copy's cap reads as a plane");
            let PoseValue::Plane { origin, .. } = pose else {
                panic!("a cap reads as a plane, got {pose:?}");
            };
            (origin.x.powi(2) + origin.y.powi(2) + origin.z.powi(2)).sqrt() + 0.0
        };
        let datum = self
            .mates
            .iter()
            .map(|&m| from_origin(m))
            .fold(0.0_f64, f64::max);
        reach + datum
    }
}

/// The mate fault a placement's row carries, if it refused its bundle.
fn bundle_fault(ev: &Evaluation<f64>, placement: RecipeNodeId) -> Option<MateFault> {
    match ev.result(placement) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Mate(fault) => Some((**fault).clone()),
            other => panic!("expected a mate refusal, got {other:?}"),
        },
        Some(NodeResult::Ok(_)) => None,
        other => panic!("the placement neither evaluated nor refused: {other:?}"),
    }
}

/// **The lever of a bundle is the formula, to the bit**: a copy whose
/// bottom cap rests on two planes of the world a quarter turn apart
/// refuses CONTRADICTORY with a levered residual, and the lever it
/// carries equals the copied body's reach plus its cap plane's distance
/// from the origin, computed in the row from the same public reach and
/// the pose bound where the fold binds it.
///
/// *Breaks if* the lever loses either half, gains a floor or a
/// constant, or is grouped otherwise.
#[test]
fn a2_the_lever_is_the_formula_to_the_bit() {
    let t = tilted_copy("msolve6-a2-lever", 0.5, 1.0, core::f64::consts::FRAC_PI_2);
    let ev = run(&t.doc, &EvalOptions::default());
    let fault = bundle_fault(&ev, t.copy).expect("a quarter turn apart, the planes contradict");
    let MateFault::Contradictory {
        clash: Clash::Levered(lever),
        ..
    } = &fault
    else {
        panic!("expected CONTRADICTORY with a lever, got {fault:?}");
    };
    assert_eq!(
        lever.arm().to_bits(),
        t.lever(&ev).to_bits(),
        "the lever is the formula, bit for bit: {fault:?}"
    );
}

// ---- A3: the lever decides at the copy's scale ----

/// The measure site's own example, at the mate site: a copy of a box of
/// half-side `half` and height `2 half`, resting on two planes of the
/// world `1e-8` rad apart. The verdict the fold reaches must be the one
/// the run's band gives for `1e-8 · L` at the COPY's lever `L` — and
/// where the band separates that from the metre's verdict, the two
/// differ.
fn tilted(label: &str, half: f64) -> (Verdict, Verdict, Option<MateFault>, f64) {
    let theta = 1e-8;
    let t = tilted_copy(label, half, 2.0 * half, theta);
    let ev = run(&t.doc, &EvalOptions::default());
    let arm = t.lever(&ev);
    let band = Band::linear(Tol::witness()).expect("the band");
    let fault = bundle_fault(&ev, t.copy);
    let found = match &fault {
        None => Verdict::Parallel,
        Some(MateFault::Contradictory {
            clash: Clash::Levered(lever),
            ..
        }) => {
            assert_eq!(
                lever.arm().to_bits(),
                arm.to_bits(),
                "the refusal carries the copy's lever"
            );
            Verdict::Refused
        }
        Some(MateFault::Indeterminate { .. }) => Verdict::Indeterminate,
        Some(other) => panic!("unexpected fault {other:?}"),
    };
    (found, verdict(band, theta, 1.0), fault, arm)
}

/// **A 10 mm copy tilted by 1e-8 rad is PARALLEL across its own
/// extent**: the deviation is `1e-8 · L` with `L` a few centimetres,
/// a few 1e-10 m — under the default ε, where a metre lever priced
/// the same tilt at 1e-8 and refused it.
///
/// *Breaks if* the lever is anything but the copy's own scale.
#[test]
fn a3_a_10mm_copy_tilted_1e_8_decides_at_its_own_scale() {
    let (found, metre, fault, arm) = tilted("msolve6-a3-10mm", 0.005);
    let expected = verdict(Band::linear(Tol::witness()).expect("the band"), 1e-8, arm);
    assert_eq!(found, expected, "at L = {arm}: {fault:?}");
    assert!(
        arm < 0.1,
        "a 10 mm copy's lever is centimetres, not a metre: {arm}"
    );
    // At the default ε the copy's verdict is PARALLEL and the metre's
    // is REFUSED; at every ε the run's verdict is the copy's, not the
    // metre's, wherever the band tells them apart.
    if Tol::witness().eps() == 1e-9 {
        assert_eq!((found, metre), (Verdict::Parallel, Verdict::Refused));
    }
}

/// **A 10 m copy tilted by 1e-8 rad is REFUSED**: across tens of
/// metres the same tilt deviates by 1e-7 m, and the refusal names the
/// copy-scale lever.
///
/// *Breaks if* the lever is anything but the copy's own scale.
#[test]
fn a3_a_10m_copy_tilted_1e_8_is_refused_at_its_scale() {
    let (found, _, fault, arm) = tilted("msolve6-a3-10m", 5.0);
    let expected = verdict(Band::linear(Tol::witness()).expect("the band"), 1e-8, arm);
    assert_eq!(found, expected, "at L = {arm}: {fault:?}");
    assert!(arm > 10.0, "a 10 m copy's lever is tens of metres: {arm}");
    if Tol::witness().eps() == 1e-9 {
        assert_eq!(found, Verdict::Refused);
    }
}

// ---- A4: refusals typed ----

/// **A part that does not resolve fails its instance in the resolver's
/// voice, carrying the `PartFault` unaltered** — and the blast radius
/// is pinned: the instance whose part is lost refuses on its own, with
/// no bundle fault, and the copy its mate reads is untouched. With no
/// resolver at all, the same arm in that voice at the copy the mate
/// reads, through which the lost instance is poisoned.
///
/// *Breaks if* a lost part reads as a mate's refusal, or reaches the
/// copy the mate is pinned against.
#[test]
fn a4_an_unresolvable_part_fails_its_instance_in_the_resolvers_voice() {
    // The part lives in ANOTHER store: the reference is well formed
    // and nothing here can resolve it.
    let mut elsewhere = PartStore::new();
    let lost_part = box_part("msolve6-a4-elsewhere", 0.5, 1.0);
    let lost_body = body_node(&lost_part);
    let doc_ref = elsewhere.insert(lost_part, Tol::witness());
    let (doc, ids, opts, body) = instances(
        "msolve6-a4-unresolved",
        box_part("msolve6-a4-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, fixture::mated_instance(doc_ref));
    let (doc, mate) = seat(doc, (ids[0], body), (lost, lost_body));
    assert_eq!(mate.placement, lost, "the mate is the lost instance's");
    let ev = run(&doc, &opts);
    let part_fault = |ev: &Evaluation<f64>| match ev.result(lost) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            NodeErrorKind::Part { fault, .. } => fault.clone(),
            other => panic!("expected the part's own refusal, got {other:?}"),
        },
        other => panic!("expected a failure, got {other:?}"),
    };
    assert!(
        matches!(
            part_fault(&ev),
            PartFault::Unresolved {
                fault: ResolveFault::Unresolved,
                ..
            }
        ),
        "the resolver's own classification, unaltered: {:?}",
        part_fault(&ev)
    );
    let poses = solve(&doc, &opts, Tol::witness());
    assert_eq!(
        (poses.fault(lost), poses.fault(ids[0])),
        (None, None),
        "no bundle refused: the lost instance's refusal is its part's"
    );
    assert!(
        matches!(ev.result(ids[0]), Some(NodeResult::Ok(_))),
        "{:?}",
        ev.result(ids[0])
    );
    // No resolver at all: the same arm, in that voice, at the instance
    // the seat is read against — which the lost instance reads, so it
    // is poisoned through it before its own part is asked.
    let none = run(&doc, &EvalOptions::default());
    assert!(
        matches!(
            none.result(ids[0]),
            Some(NodeResult::Failed(e)) if matches!(
                &e.kind,
                NodeErrorKind::Part { fault: PartFault::NoResolver, .. }
            )
        ),
        "{:?}",
        none.result(ids[0])
    );
    assert!(
        matches!(none.result(lost), Some(NodeResult::Poisoned { through }) if *through == ids[0]),
        "{:?}",
        none.result(lost)
    );
}

// ---- A2: the reach is an upper bound, measured ----

/// **A box part's reach is at least its far corner's distance** — and
/// for a body whose every edge is a line the rim walk is EXACT, so it
/// is that distance to the bit.
#[test]
fn a2_a_box_parts_reach_is_its_far_corner() {
    let (half, height) = (0.5, 1.0);
    let (doc, ids, opts, _) = instances(
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
    let (doc, ids, opts, _) = instances(
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
    match (theta * arm).sign_within(band).map(|d| d.sign) {
        Ok(Sign::Zero) => Verdict::Parallel,
        Ok(Sign::Positive | Sign::Negative) => Verdict::Refused,
        Err(_) => Verdict::Indeterminate,
    }
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
        .mvfs(Point3::new(0.0, 0.0, 0.0), true)
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
}

// ---- A5: the evaluation and the memo ----

/// **A mated part is evaluated exactly once**: the fold's reach is the
/// copy's own, and the instantiate nodes read the one table the run
/// entered before either, so seating one instance on another crosses
/// into the part once — the same count as with no mate at all.
///
/// *Breaks if* forming a lever resolves the part a second time.
#[test]
fn a5_a_mated_part_is_evaluated_exactly_once() {
    let (doc, ids, opts, body) = instances(
        "msolve6-a5-once",
        box_part("msolve6-a5-once-part", 0.5, 1.0),
        2,
    );
    let before = run(&doc, &opts);
    assert_eq!(
        before.part_evaluations, 1,
        "two instances, one part, one crossing"
    );
    let (doc, mate) = seat(doc, (ids[0], body), (ids[1], body));
    let after = run(&doc, &opts);
    assert_eq!(
        after.part_evaluations, 1,
        "the fold's reach and the instantiate nodes share one evaluation"
    );
    assert_eq!(mate.placement, ids[1]);
    for &id in &ids {
        assert!(matches!(after.result(id), Some(NodeResult::Ok(_))));
    }
}

/// **A part whose content changes so that the verdict flips reaches
/// the placement's memo**: the fold's lever is the copy's reach, and
/// the placement's key is its reads' keys, so the placement that
/// evaluated `Ok` over a 10 mm part is not served from the memo over a
/// 10 m one — it refuses.
///
/// *Breaks if* a re-pinned part is served the placement's old answer.
#[test]
fn a5_a_part_change_that_flips_the_verdict_moves_the_placements_memo() {
    let theta = 1e-8;
    // Two versions of ONE part document, the later a value edit of the
    // earlier: the same ids, so the reference can be re-pinned in
    // place; different extents.
    let small = box_part("msolve6-a5-memo-part", 0.005, 0.01);
    let body = body_node(&small);
    let large = resized(small.clone(), 5.0, 10.0);
    let large_pin = content_pin(&large, Tol::witness()).unwrap();
    let mut store_small = PartStore::new();
    let small_ref = store_small.insert(small, Tol::witness());
    let mut store_large = PartStore::new();
    store_large.insert(large, Tol::witness());
    let opts_small = with_resolver(store_small);
    let opts_large = with_resolver(store_large);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-a5-memo"), Tol::witness());
    let (doc, b) = insert(doc, fixture::mated_instance(small_ref));
    let bottom = || fixture::plane(fixture::head(in_part(b, body, CapEnd::Start)));
    let mut doc = doc;
    for to in [floor(), tilted_floor(theta)] {
        doc = fixture::constrain_at(
            doc,
            b,
            Mate {
                on: bottom(),
                to: Target::World(to),
                class: ContactClass::Rest,
            },
        )
        .0;
    }
    let first = run(&doc, &opts_small);
    // The row is about the flip, so it only runs where the band flips
    // it: parallel at the small scale. Elsewhere the small part's own
    // refusal is the band's verdict, and the row ends on it.
    if let Some(fault) = bundle_fault(&first, b) {
        assert!(
            matches!(
                fault,
                MateFault::Contradictory { .. } | MateFault::Indeterminate { .. }
            ),
            "the small part's refusal is the band's verdict: {fault:?}"
        );
        eprintln!(
            "a5 memo: the band refuses at the small scale at this ε ({fault:?}), so the flip \
             is not reachable"
        );
        return;
    }
    let key_small = first
        .value(b)
        .map(|v| v.content_key)
        .expect("the small part's placement evaluates");
    let (re, _) = step(
        doc,
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
    match bundle_fault(&second, b) {
        Some(fault) => assert!(
            matches!(
                fault,
                MateFault::Contradictory { .. } | MateFault::Indeterminate { .. }
            ),
            "the re-pinned part flips the verdict and the memo cannot serve it: {fault:?}"
        ),
        None => {
            // The band did not separate the two scales at this ε, so
            // the fold's answer did not move. The key does: the
            // placement copies a re-pinned part, so the memo cannot
            // serve the old entry — it re-runs and answers the same.
            let large = second
                .value(b)
                .expect("the large part's placement evaluates");
            assert_ne!(
                key_small, large.content_key,
                "the placement reads the re-pinned part"
            );
        }
    }
}

// ---- The edit door: no edit asks the store, and replay ----

/// A reach that counts its asks and answers through the store's — the
/// witness that an edit which adds no rider never consults the
/// store.
struct Counting<'a>(core::cell::Cell<usize>, PartReach<'a, f64>);

impl MateReach for Counting<'_> {
    fn reach(&self, part: &editor_core::DocRef) -> Result<f64, ReachRefusal> {
        self.0.set(self.0.get() + 1);
        self.1.reach(part)
    }
}

/// Two instances of a box, the first pinned to the world and the
/// second seated flush on it, through the counting door: the mated
/// document the edit rows work on, with its log as the file carries
/// it, and the seat's constraints.
#[allow(clippy::type_complexity)]
fn seated(
    label: &str,
) -> (
    ProfileDoc,
    [RecipeNodeId; 2],
    Vec<MateRef>,
    EvalOptions,
    Vec<DocEdit<editor_core::ProfileProgram>>,
    RecipeNodeId,
) {
    let (doc, ids, opts, body) = instances(label, box_part(&format!("{label}-part"), 0.5, 1.0), 0);
    debug_assert!(ids.is_empty());
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let mut log = Vec::new();
    let mut doc = doc;
    let mut push = |doc: &mut ProfileDoc, edit: DocEdit<editor_core::ProfileProgram>| {
        let applied = doc
            .apply(&edit, Tol::witness(), &reach)
            .expect("the edit applies");
        log.push(edit);
        *doc = applied.doc;
        (applied.record.minted, applied.record.minted_constraints)
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
    let insert_node = |node| DocEdit::InsertNode {
        node: Box::new(node),
        fresh: Vec::new(),
    };
    let a = push(&mut doc, insert_node(Node::instantiate_part(part_ref)))
        .0
        .unwrap();
    let pin = editor_core::pin_at(
        &doc,
        &editor_core::Operand::Var(doc.output(a, 0).expect("an instance defines its copies")),
        &Frame::IDENTITY,
        &opts,
        Tol::witness(),
    )
    .expect("the instance pins over its geometry");
    push(
        &mut doc,
        DocEdit::AddConstraint {
            placement: a,
            mate: Box::new(pin),
            fresh: Vec::new(),
        },
    );
    let b = push(&mut doc, insert_node(fixture::mated_instance(part_ref)))
        .0
        .unwrap();
    let rest = MateRef::new(
        b,
        push(
            &mut doc,
            DocEdit::AddConstraint {
                placement: b,
                mate: Box::new(fixture::rest(
                    fixture::head(in_part(b, body, CapEnd::Start)),
                    fixture::head(in_part(a, body, CapEnd::End)),
                    ContactClass::Rest,
                )),
                fresh: Vec::new(),
            },
        )
        .1[0],
    );
    let mut seat = vec![rest];
    for (coord, by) in [
        (Coord::Slide(SlideAxis::X), len(0.0)),
        (Coord::Slide(SlideAxis::Y), len(0.0)),
        (Coord::Spin, ang(0.0)),
    ] {
        let id = push(
            &mut doc,
            DocEdit::AddValue {
                placement: b,
                value: Box::new(Value {
                    mate: rest.id,
                    coord,
                    by,
                }),
                fresh: Vec::new(),
            },
        )
        .1[0];
        seat.push(MateRef::new(b, id));
    }
    (doc, [a, b], seat, opts, log, body)
}

/// **A constraint edit is never refused for the placement it removes,
/// and asks no store** (A11 (2)): no edit records a pose, so removing
/// a mate from a placement whose part does not resolve applies through
/// the store's reach and through no resolver alike, and leaves the
/// copy it pinned loose — no row, no solve.
///
/// *Breaks if* a removal asks the store, or invents a constraint.
#[test]
fn a6_a_mate_graph_edit_on_an_unresolvable_part_is_not_refused() {
    let mut elsewhere = PartStore::new();
    let lost_part = box_part("msolve6-a6-elsewhere", 0.5, 1.0);
    let lost_body = body_node(&lost_part);
    let lost_ref = elsewhere.insert(lost_part, Tol::witness());
    let (doc, ids, opts, body) = instances(
        "msolve6-a6-unresolved",
        box_part("msolve6-a6-part", 0.5, 1.0),
        1,
    );
    let (doc, lost) = insert(doc, fixture::mated_instance(lost_ref));
    let (doc, mate) = fixture::constrain_at(
        doc,
        lost,
        fixture::rest(
            fixture::head(in_part(lost, lost_body, CapEnd::Start)),
            fixture::head(in_part(ids[0], body, CapEnd::End)),
            ContactClass::Rest,
        ),
    );
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let remove = DocEdit::RemoveConstraint {
        placement: mate.placement,
        constraint: mate.id,
    };
    for applied in [
        doc.apply(&remove, Tol::witness(), &reach),
        doc.apply(&remove, Tol::witness(), &editor_core::RefusingReach),
    ] {
        let applied = applied.expect("removing a placing mate is never refused");
        assert!(
            crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
            "{:?}",
            applied.maintenance
        );
        assert!(
            fixture::bundle(&applied.doc, lost).is_empty(),
            "the copy the mate pinned is loose, with no constraint the removal invented"
        );
        assert_eq!(
            fixture::bundle(&applied.doc, ids[0]),
            fixture::bundle(&doc, ids[0]),
            "the copy it was pinned against keeps its bundle"
        );
    }
}

/// **No edit asks the store**: the constraint door decides only what
/// the recipe says, so on a mated document a seat added to a third
/// instance, a world pin removed and re-added at another pose, an
/// appearance, a union, a fourth instance and every removal — the
/// seat's, the first instance's — consult the store's reach not once.
///
/// *Breaks if* any edit door evaluates a part to admit an edit.
#[test]
fn a6_no_edit_asks_the_store() {
    let (doc, [a, b], seat_refs, opts, _log, body) = seated("msolve6-a6-preserving");
    let counting = Counting(
        core::cell::Cell::new(0),
        mate_reach::<f64>(&opts, Tol::witness()),
    );
    let tol = Tol::witness();
    let part_ref = match doc.node(a) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        _ => panic!("an instance"),
    };
    let (doc, c) = insert(doc, fixture::mated_instance(part_ref));
    let applied = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: c,
                mate: Box::new(fixture::rest(
                    fixture::head(in_part(c, body, CapEnd::Start)),
                    fixture::head(in_part(b, body, CapEnd::End)),
                    ContactClass::Rest,
                )),
                fresh: Vec::new(),
            },
            tol,
            &counting,
        )
        .expect("a mate is added");
    assert!(
        crate::fixture::without_anonymous(&applied.maintenance).is_empty(),
        "{:?}",
        applied.maintenance
    );
    let doc = applied.doc;
    let [pin] = &fixture::bundle(&doc, a)[..] else {
        panic!("the first instance is pinned to the world by one constraint");
    };
    let doc = doc
        .apply(
            &DocEdit::RemoveConstraint {
                placement: a,
                constraint: pin.id,
            },
            tol,
            &counting,
        )
        .expect("removing a world pin asks nothing")
        .doc;
    let moved = editor_core::pin_at(
        &doc,
        &editor_core::Operand::Var(doc.output(a, 0).expect("an instance defines its copies")),
        &Frame::translation([0.0, 0.0, 3.0]),
        &opts,
        tol,
    )
    .expect("the instance pins over its geometry");
    let doc = doc
        .apply(
            &DocEdit::AddConstraint {
                placement: a,
                mate: Box::new(moved),
                fresh: Vec::new(),
            },
            tol,
            &counting,
        )
        .expect("a world pin asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::SetAppearance {
                body: doc.read_of_node(a).expect("the instance's one output"),
                name: in_part(a, body, CapEnd::End),
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
                node: Box::new(Node::Union {
                    members: editor_core::Bodies::Spelled(vec![
                        fixture::copy(b, body),
                        fixture::copy(c, body),
                    ]),
                }),
                fresh: Vec::new(),
            },
            tol,
            &counting,
        )
        .expect("a union asks nothing")
        .doc;
    let doc = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(fixture::mated_instance(part_ref)),
                fresh: Vec::new(),
            },
            tol,
            &counting,
        )
        .expect("a fourth instance asks nothing")
        .doc;
    let stranding = |applied: &editor_core::Applied<editor_core::ProfileProgram>| {
        assert!(
            crate::fixture::without_anonymous(&applied.maintenance)
                .iter()
                .all(|row| matches!(
                    row,
                    editor_core::Maintenance::Strand { .. }
                        | editor_core::Maintenance::StrandedRead { .. }
                        | editor_core::Maintenance::StrandedSelection { .. }
                        | editor_core::Maintenance::StrandedAppearance { .. }
                )),
            "a removal reports what it stranded and records no pose: {:?}",
            applied.maintenance
        );
    };
    // The seat's values, then the mate they set the freedoms of.
    let mut unseated = doc.clone();
    for m in seat_refs.iter().rev() {
        let applied = unseated
            .apply(
                &DocEdit::RemoveConstraint {
                    placement: m.placement,
                    constraint: m.id,
                },
                tol,
                &counting,
            )
            .expect("a removal is never refused for the placement it removes");
        stranding(&applied);
        unseated = applied.doc;
    }
    stranding(
        &doc.apply(&DocEdit::DeleteNode { id: a }, tol, &counting)
            .expect("a removal is never refused for the placement it removes"),
    );
    assert_eq!(counting.0.get(), 0, "no edit asked the store");
}

/// **A saved document whose seat is taken apart replays
/// bit-identically with no store**: the log is its edits, and replay
/// re-applies them without evaluating — the copy the removed seat
/// pinned is loose, not re-pinned.
///
/// *Breaks if* replay consults a part, or re-pins what the log loosed.
#[test]
fn a6_a_saved_unseating_replays_bit_identically_with_no_store() {
    let (doc, [a, b], seat_refs, opts, mut log, _) = seated("msolve6-a6-replay");
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    // The values first, then the mate they set the freedoms of.
    let mut live = doc;
    for m in seat_refs.iter().rev() {
        let remove = DocEdit::RemoveConstraint {
            placement: m.placement,
            constraint: m.id,
        };
        live = live
            .apply(&remove, Tol::witness(), &reach)
            .expect("the removal applies")
            .doc;
        log.push(remove);
    }
    let empty = ProfileDoc::empty(live.id(), Tol::witness());
    let text = editor_core::save(&empty, &log, Tol::witness()).expect("saves");
    let loaded = editor_core::load(&text, Tol::witness()).expect("loads with no store");
    assert!(loaded.doc.bit_eq(&live), "the document replays bit for bit");
    assert_eq!(loaded.edits, log, "the log comes back as saved");
    let replayed = ProfileDoc::replay(live.id(), &log, Tol::witness()).expect("replays");
    assert!(replayed.bit_eq(&live));
    assert!(
        fixture::bundle(&live, b).is_empty(),
        "the copy the seat pinned stays loose"
    );
    assert_eq!(fixture::bundle(&live, a).len(), 1, "the first stays pinned");
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
/// bit**: the one `f64` read of a certified body's reach is the upper
/// end of its bracket (an upper bound by definition), it bounds the
/// `f64` lane's reach, and an assembly over the part — two instances,
/// each pinned to the world over its own geometry — evaluates `Ok` at
/// `Interval` with the part evaluated once.
///
/// *Breaks if* the interval reach is read anywhere but the bracket's
/// `hi`, or a placement's fold at `Interval` resolves the part again.
#[test]
fn a5_at_interval_the_doors_reach_is_the_brackets_hi_bit_for_bit() {
    use geom_core::{Bounds, Interval};
    let (doc, ids, opts, _) = instances(
        "msolve6-a5-interval",
        cylinder_part("msolve6-a5-interval-part", 0.3, 0.7),
        2,
    );
    let doc_ref = match doc.node(ids[0]) {
        Some(Node::InstantiatePart { doc_ref, .. }) => *doc_ref,
        other => panic!("an instance, not {other:?}"),
    };
    let doc = fixture::pin(doc, ids[1], &Frame::translation([2.0, 0.0, 0.0]), &opts);
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
    for &id in &ids {
        assert!(
            matches!(ev.result(id), Some(NodeResult::Ok(_))),
            "{:?}",
            ev.result(id)
        );
    }
    assert_eq!(ev.part_evaluations, 1);
}

// ---- The correctness arm's probes, adopted ----

/// The unit cube `[0,1]³`, placed in its world ([`placed_part`]).
fn block(label: &str) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    placed_part(doc, body)
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
        let (doc, ids, opts, _) = instances(&format!("msolve6-p1-asm-{name}"), part.clone(), 1);
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

/// **A mate chained to an indeterminate one removes like any other**:
/// a tilt priced inside the band leaves a placement with NO verdict,
/// the copy seated on it is poisoned through it, and the removal of
/// that seat neither asks for a verdict nor refuses — no edit records a
/// pose.
///
/// The middle copy rests on the world's floor and on the floor tilted
/// by `t`; the table's parallel split decides `|n₁ × n₂|` over the
/// bundle's lever, and the tilt that lands inside the band is found by
/// stepping `t` by a fraction of the band's own width, so some step
/// falls in it whatever the lever.
///
/// *Breaks if* an indeterminate placement's reader evaluates, or a
/// removal decides anything.
#[test]
fn a6_a_mate_chained_to_an_indeterminate_one_removes_like_any_other() {
    let band = Band::linear(Tol::witness()).expect("band");
    let step_by = (band.escalate() / band.zero()).powf(0.25);
    let build = |theta: f64| {
        let part = box_part("msolve6-p3c-part", 5.0, 10.0);
        let body = body_node(&part);
        let mut store = PartStore::new();
        let doc_ref = store.insert(part, Tol::witness());
        let opts = with_resolver(store);
        let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p3c"), Tol::witness());
        let (doc, b) = insert(doc, fixture::mated_instance(doc_ref));
        let (doc, c) = insert(doc, fixture::mated_instance(doc_ref));
        let mut doc = doc;
        for to in [floor(), tilted_floor(theta)] {
            doc = fixture::constrain_at(
                doc,
                b,
                Mate {
                    on: fixture::plane(fixture::head(in_part(b, body, CapEnd::Start))),
                    to: Target::World(to),
                    class: ContactClass::Rest,
                },
            )
            .0;
        }
        // One rest, no value names it: the mate this row removes.
        let (doc, m2) = fixture::constrain_at(
            doc,
            c,
            fixture::rest(
                fixture::head(in_part(c, body, CapEnd::Start)),
                fixture::head(in_part(b, body, CapEnd::End)),
                ContactClass::Rest,
            ),
        );
        (doc, opts, b, c, m2)
    };
    let (doc, opts, b, c, m2) = (0..48)
        .map(|j| band.zero() * 1e-4 * step_by.powi(j))
        .map(build)
        .find(|(doc, opts, b, ..)| {
            matches!(
                solve(doc, opts, Tol::witness()).fault(*b),
                Some(MateFault::Indeterminate { .. })
            )
        })
        .expect("some tilt lands its levered sine inside the band");
    let ev = run(&doc, &opts);
    assert!(
        matches!(ev.result(c), Some(NodeResult::Poisoned { through }) if *through == b),
        "the copy seated on the undecided one is poisoned through it: {:?}",
        ev.result(c)
    );
    let reach = mate_reach::<f64>(&opts, Tol::witness());
    let remove = DocEdit::RemoveConstraint {
        placement: m2.placement,
        constraint: m2.id,
    };
    for applied in [
        doc.apply(&remove, Tol::witness(), &reach),
        doc.apply(&remove, Tol::witness(), &editor_core::RefusingReach),
    ] {
        let applied = applied.expect("removing a mate is never refused");
        assert!(crate::fixture::without_anonymous(&applied.maintenance).is_empty());
    }
}

/// **A whole-assembly split pins its instance where it stands**: the
/// cut's instances move into the new part, and the instance left
/// behind is pinned by the named interim's world pin, which reads
/// nothing, so the split asks no geometry of the part it mints and
/// pins alike whether or not the caller holds a resolver.
///
/// *Breaks if* a split leaves an unpinned instance, or pins it by a
/// mate read off geometry it must evaluate.
#[test]
fn a6_a_split_pins_its_instance_where_it_stands_with_or_without_a_resolver() {
    let (doc, ids, opts, body) = instances("msolve6-p6", box_part("msolve6-p6-part", 0.5, 1.0), 2);
    let [a, b] = [ids[0], ids[1]];
    let (doc, _) = seat(doc, (a, body), (b, body));
    // Both instances, the second's bundle holding the seat.
    let cut = fixture::with_placements(&doc, &[a, b].into_iter().collect());
    for (label, resolver) in [("held", opts.resolver.as_ref()), ("none", None)] {
        let out = editor_core::split(
            &doc,
            &cut,
            DocumentId::derive(&format!("msolve6-p6-new-part-{label}")),
            Tol::witness(),
            resolver,
        )
        .unwrap_or_else(|e| panic!("a whole-assembly cut splits ({label}): {e}"));
        let remainder = &out.remainder;
        let pinned: Vec<_> = remainder
            .placements()
            .into_iter()
            .map(|at| fixture::bundle(remainder, at))
            .collect();
        assert_eq!(pinned.len(), 1, "one instance is left behind ({label})");
        assert!(
            matches!(pinned[0].as_slice(), [c] if c.world_pin().is_some()),
            "the instance is pinned where it stands by the world pin alone ({label}): {:?}",
            pinned[0]
        );
    }
}

/// **Two mated parts evaluate once each**: a chain over two distinct
/// parts (three instances, two seats) evaluates each part exactly
/// once — the folds' reaches share the run's cache with the
/// instantiate nodes — and every node evaluates `Ok`.
///
/// *Breaks if* a fold resolves a part the instance already did.
#[test]
fn a5_two_mated_parts_evaluate_once_each() {
    let mut store = PartStore::new();
    let (pa, pb) = (box_part("msolve6-p8-a", 0.5, 1.0), block("msolve6-p8-b"));
    let (a_body, b_body) = (body_node(&pa), body_node(&pb));
    let ra = store.insert(pa, Tol::witness());
    let rb = store.insert(pb, Tol::witness());
    let opts = with_resolver(store);
    let doc = ProfileDoc::empty(DocumentId::derive("msolve6-p8"), Tol::witness());
    let (doc, a) = fixture::insert_pinned(doc, Node::instantiate_part(ra), &opts);
    let (doc, b) = insert(doc, fixture::mated_instance(rb));
    let (doc, c) = insert(doc, fixture::mated_instance(ra));
    assert_eq!(run(&doc, &opts).part_evaluations, 2);
    let (doc, _) = seat(doc, (a, a_body), (b, b_body));
    let (doc, _) = seat(doc, (b, b_body), (c, a_body));
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
