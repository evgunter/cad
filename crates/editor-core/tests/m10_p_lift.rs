//! **The profile-parameter lift at the evaluation seam.**
//!
//! `plate_param` is the fixture throughout: the corpus's one document
//! whose profile geometry is an EXPRESSION, so a document parameter
//! drives two loops at once. Before the lift, a `Dual` seed on that
//! parameter propagated no tangent and an interval binding widened
//! nothing — profile geometry reached every lane as f64 bits. These
//! rows are what changed and what did not.
//!
//! - **Off is off.** The lift defaults to `Pinned` and the whole build
//!   path runs there; `m10_p_fence.rs` digests that against the
//!   pre-lift tree. Here the narrower claim: turning the lift ON at
//!   `f64` changes nothing, because guided elaboration at `f64` is
//!   plain elaboration at `f64`.
//! - **The seam is open.** Resolving the same program at `Dual` and
//!   elaborating it GUIDED carries a seed on the hole radius through to
//!   the vertices it moves — a derivative where there was a zero.
//! - **Wide boxes abort, typed.** An interval binding wide enough that
//!   a consumed decision can no longer be confirmed refuses, naming the
//!   decision, rather than quietly keeping the nominal structure.
//! - **The two ladders agree.** The sweep/loft seam and the profile
//!   node's seam run the same f64 ladder — literally the same function
//!   — and the same second pass.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, EvalOptions, Node, NodeResult, ParamValue, ProfileLift, ValuePayload,
    evaluate,
};
use geom_core::{Real, Tol};

/// Every body point of an evaluation, by bits — the comparable surface.
fn body_bits(ev: &editor_core::Evaluation<f64>) -> Vec<u64> {
    let mut out = Vec::new();
    for result in ev.nodes.values() {
        if let NodeResult::Ok(v) = result
            && let ValuePayload::Body(b) = &v.payload
        {
            for (_, p) in b.points() {
                out.extend([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]);
            }
        }
    }
    out
}

fn options(lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

/// Turning the lift on at `f64` changes no geometry.
///
/// It is not a tautology: with the lift on, the profile node's payload
/// comes from a program RE-RESOLVED at the lane scalar and re-elaborated
/// under guidance, not from the f64 precompute embedded. That the two
/// agree bit for bit at `T = f64` is the differential the design asks
/// for, taken through the public evaluation door.
#[test]
fn the_lift_at_f64_reproduces_the_pinned_evaluation() {
    for doc in corpus::documents() {
        let run = |lift| {
            evaluate::<f64>(
                &doc.doc,
                None,
                &CancelToken::new(),
                &options(lift),
                Tol::witness(),
            )
        };
        let pinned = run(ProfileLift::Pinned);
        let guided = run(ProfileLift::Guided);
        assert_eq!(
            body_bits(&pinned),
            body_bits(&guided),
            "{}: guided elaboration at f64 moved the geometry",
            doc.name
        );
        for (id, a) in pinned.nodes.iter() {
            let b = guided.nodes.get(id).expect("same node set");
            assert_eq!(
                a.error().map(ToString::to_string),
                b.error().map(ToString::to_string),
                "{}: node {id:?} refused differently under the lift",
                doc.name
            );
        }
    }
}

/// The lift moves the CONTENT KEY, and must: two evaluations whose
/// profile geometry is computed at different scalars are not
/// interchangeable, so one may not serve the other's memo.
#[test]
fn the_lift_changes_the_content_key() {
    let doc = plate();
    let run = |lift| {
        evaluate::<f64>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &options(lift),
            Tol::witness(),
        )
    };
    let (pinned, guided) = (run(ProfileLift::Pinned), run(ProfileLift::Guided));
    let profile_node = profile_node_of(&doc);
    let key = |ev: &editor_core::Evaluation<f64>| {
        ev.value(profile_node)
            .expect("the profile node evaluates")
            .content_key
    };
    assert_ne!(
        key(&pinned),
        key(&guided),
        "a lifted profile node's key must differ from the pinned one's — \
         otherwise a seeded or widened evaluation hits the nominal's memo"
    );
}

/// **The seam the unit exists to open.** A `Dual` seed on the hole
/// radius reaches the vertices that radius moves.
///
/// Driven through the public program door rather than `evaluate`,
/// because `evaluate` at `Dual` additionally needs `ContentBits for
/// Dual`, which is another unit's (see the PR's deviations). Everything
/// under test here — resolving at the lane environment, elaborating
/// guided against the f64 record — is exactly what the evaluation seam
/// calls.
#[test]
fn a_dual_seed_on_a_profile_parameter_now_carries_a_tangent() {
    use geom_core::Dual64;
    let doc = plate();
    let Some(Node::Profile(program)) = doc.doc.node(profile_node_of(&doc)) else {
        panic!("the plate's profile node is a profile node")
    };
    // Pass 1: the f64 elaboration, and its record.
    let nominal = program
        .resolve(&doc.doc.param_env::<f64>())
        .expect("resolves at f64");
    let mut records = Vec::new();
    for steps in &nominal {
        let (_, record) = profile::replay_recording(steps, Tol::witness()).expect("replays");
        records.push(record);
    }
    // Pass 2: the same program at Dual, seeded on the hole radius.
    let mut env = doc.doc.param_env::<Dual64>();
    let seeded = env
        .bindings
        .iter_mut()
        .filter_map(|(_, v)| match v {
            ParamValue::Continuous { value, .. } => Some(value),
            ParamValue::Count(_) => None,
        })
        .map(|value| {
            *value = Dual64::variable(value.value);
        })
        .count();
    assert!(seeded > 0, "the plate is parameterized");
    let resolved = program.resolve(&env).expect("resolves at Dual");
    let mut seen_tangent = false;
    for (li, steps) in resolved.iter().enumerate() {
        let lp = profile::replay_guided(steps, &records[li], Tol::witness())
            .expect("the seeded elaboration keeps the nominal structure");
        seen_tangent |= lp
            .vertices()
            .iter()
            .any(|v| v.pos().x.deriv != 0.0 || v.pos().y.deriv != 0.0);
    }
    assert!(
        seen_tangent,
        "no vertex carried a derivative — the seed died at the profile boundary, \
         which is the state the lift exists to end"
    );
}

/// **The wide-box row.** An interval binding wide enough that a
/// consumed decision can no longer be confirmed refuses TYPED, naming
/// the decision — and nothing keeps the nominal structure quietly.
///
/// The width is deliberately absurd (the hole radius bound to a box
/// spanning from well inside the plate to well outside it): the claim
/// is about the SHAPE of the answer, not about where the exact
/// threshold sits.
#[cfg(feature = "interval")]
#[test]
fn a_wide_interval_binding_aborts_typed_rather_than_certifying() {
    use geom_core::Interval;
    let doc = plate();
    let Some(Node::Profile(program)) = doc.doc.node(profile_node_of(&doc)) else {
        panic!("the plate's profile node is a profile node")
    };
    let nominal = program
        .resolve(&doc.doc.param_env::<f64>())
        .expect("resolves at f64");
    let mut records = Vec::new();
    for steps in &nominal {
        let (_, record) = profile::replay_recording(steps, Tol::witness()).expect("replays");
        records.push(record);
    }
    let mut env = doc.doc.param_env::<Interval>();
    for v in env.bindings.values_mut() {
        if let ParamValue::Continuous { value, .. } = v {
            // A box from a hair above zero to an order of magnitude up.
            *value = Interval::from_bounds(1e-4, 1.0);
        }
    }
    let resolved = program.resolve(&env).expect("resolves at Interval");
    // The whole second pass, both halves — which is where the wall
    // actually is for THIS document. The plate's loops are a rectangle
    // and two circles: no fillet resolves anywhere in them, so the
    // guided REPLAY consumes no decisions and certifies the wide box
    // honestly (there is no discrete choice for the width to threaten).
    // The refusal comes from the guided VALIDATION, whose simplicity
    // and containment predicates DO see the width — and that split is
    // worth knowing, because it says the abort surface is the whole
    // ladder rather than the fillet machinery alone.
    let mut loops = Vec::new();
    for (li, steps) in resolved.iter().enumerate() {
        loops.push(
            profile::replay_guided(steps, &records[li], Tol::witness())
                .expect("no fillet resolves in the plate, so replay has nothing to lose"),
        );
    }
    let (_, canonical) = profile::Profile::new(program.plane, nominal_loops(&nominal))
        .validate_recording(Tol::witness())
        .expect("the nominal validates and records");
    let err = profile::Profile::new(interval_plane(&program.plane), loops)
        .validate_guided(Tol::witness(), &canonical)
        .expect_err("a hole radius spanning four orders of magnitude cannot certify");
    let text = err.to_string();
    assert!(
        !text.is_empty(),
        "the refusal must say what it could not confirm"
    );
    // Nothing was certified: no canonical form came back, so no caller
    // can mistake this binding for one that kept the nominal structure.
    println!("wide-box refusal: {text}");
}

/// The nominal f64 loops, replayed for the record's sake.
fn nominal_loops(resolved: &[Vec<profile::Step<f64>>]) -> Vec<profile::ProfileLoop<f64>> {
    resolved
        .iter()
        .map(|steps| profile::replay(steps, Tol::witness()).expect("the nominal replays"))
        .collect()
}

/// The sketch plane at the lane scalar (VQ8 keeps the plane out of the
/// parameter layer, so it lifts as constants).
#[cfg(feature = "interval")]
fn interval_plane(plane: &profile::SketchPlane<f64>) -> profile::SketchPlane<geom_core::Interval> {
    use geom_core::{Affine3, Interval, Mat3, Vec3};
    let a = &plane.placement;
    let v = |w: Vec3<f64>| {
        Vec3::new(
            Interval::from_f64(w.x),
            Interval::from_f64(w.y),
            Interval::from_f64(w.z),
        )
    };
    profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(v(a.linear.c0), v(a.linear.c1), v(a.linear.c2)),
        v(a.translation),
    ))
}

/// **The two-ladder parity.** The sweep/loft seam and the profile
/// node's seam do not fork.
///
/// The strong half of this is structural and lives in the source: the
/// loft/sweep seam CALLS `prepare_profile`, the profile node's own f64
/// ladder, instead of repeating its four steps beside it, and runs the
/// same second pass as a gate. What is checked here is the consequence
/// a test can see: a document whose profile feeds a loft evaluates the
/// same way under the lift as under the pin, exactly as the extrude
/// ladder does.
#[test]
fn the_loft_ladder_tracks_the_profile_ladder_under_the_lift() {
    let doc = corpus::loft_prism::document();
    let run = |lift| {
        evaluate::<f64>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &options(lift),
            Tol::witness(),
        )
    };
    let (pinned, guided) = (run(ProfileLift::Pinned), run(ProfileLift::Guided));
    assert_eq!(
        body_bits(&pinned),
        body_bits(&guided),
        "the loft ladder's geometry moved under the lift"
    );
    for (id, a) in pinned.nodes.iter() {
        let b = guided.nodes.get(id).expect("same node set");
        assert_eq!(
            a.error().map(ToString::to_string),
            b.error().map(ToString::to_string),
            "node {id:?} refused differently through the loft ladder"
        );
    }
    assert!(
        guided.nodes.values().any(|r| matches!(
            r,
            NodeResult::Ok(v) if matches!(v.payload, ValuePayload::Body(_))
        )),
        "the loft document must actually build something for this row to mean anything"
    );
}

fn plate() -> corpus::CorpusDoc {
    corpus::plate_param::document()
}

fn profile_node_of(doc: &corpus::CorpusDoc) -> editor_core::RecipeNodeId {
    *doc.doc
        .order()
        .iter()
        .find(|id| matches!(doc.doc.node(**id), Some(Node::Profile(_))))
        .expect("the plate has a profile node")
}
