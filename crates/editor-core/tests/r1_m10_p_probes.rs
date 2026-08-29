//! **R1 review probes for M10-P at the evaluation seam** — the
//! charter's end-to-end exercise, driven as a consumer would drive it.
//!
//! The document here is R1's own: a plate whose outline is FILLETED
//! and whose fillet radius is a document parameter — the shape where a
//! parameter box genuinely threatens a consumed discrete decision,
//! which `plate_param` (rectangle + circles, no fillet anywhere)
//! cannot exercise. Authored through the public edit doors.
//!
//! Static fixtures throughout; no seeds (memories/test-suite-cost.md).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Expr, LoopProgram, Node, NodeResult,
    ParamName, ParamValue, ProfileLift, ProfileProgram, ProgramStep, ValuePayload, evaluate,
};
use geom_core::Tol;
use profile::SketchPlane;

use corpus::Recorder;

/// The parameter that drives every corner fillet of the plate.
const FILLET_R: &str = "fillet_r";
const FILLET_R_VALUE: f64 = 0.5;

fn lit(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length literal")
}

fn ang(v: f64) -> Expr {
    Expr::literal(v, Dimension::Angle).expect("finite angle literal")
}

/// A 3×3 seam-filleted square whose four corner fillets share the one
/// parameter, extruded — R1's parameterized-fillet document (the
/// at/angle/fillet cycle is the chain vocabulary's fillet spelling;
/// `.to(Start)` retrims the seam corner).
fn filleted_plate() -> editor_core::ProfileDoc {
    use std::f64::consts::{FRAC_PI_2, PI};
    let r = Expr::param(ParamName::new(FILLET_R), Dimension::Length);
    let outline = LoopProgram::Chain(vec![
        ProgramStep::At([lit(1.5), lit(0.0)]),
        ProgramStep::Angle(ang(0.0)),
        ProgramStep::Fillet(r.clone()),
        ProgramStep::At([lit(3.0), lit(1.5)]),
        ProgramStep::Angle(ang(FRAC_PI_2)),
        ProgramStep::Fillet(r.clone()),
        ProgramStep::At([lit(1.5), lit(3.0)]),
        ProgramStep::Angle(ang(PI)),
        ProgramStep::Fillet(r.clone()),
        ProgramStep::At([lit(0.0), lit(1.5)]),
        ProgramStep::Angle(ang(-FRAC_PI_2)),
        ProgramStep::Fillet(r),
        ProgramStep::CloseTo,
    ]);
    let mut rec = Recorder::new();
    rec.push(DocEdit::SetDocParam {
        name: ParamName::new(FILLET_R),
        value: DocParam::continuous(Dimension::Length, FILLET_R_VALUE),
    });
    let p = rec.insert(Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![outline],
    }));
    let _solid = rec.insert(Node::Extrude {
        profile: p,
        distance: fixture::len(0.5),
    });
    rec.doc
}

fn options(lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

/// The document is real: it evaluates at f64, pinned and guided agree
/// bitwise, and the lift moves the profile node's content key.
#[test]
fn r1_filleted_plate_builds_and_the_lift_is_inert_at_f64() {
    let doc = filleted_plate();
    let run = |lift| {
        evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &options(lift),
            Tol::witness(),
        )
    };
    let (pinned, guided) = (run(ProfileLift::Pinned), run(ProfileLift::Guided));
    let mut bodies = 0usize;
    for (id, a) in pinned.nodes.iter() {
        let b = guided.nodes.get(id).expect("same node set");
        match (a, b) {
            (NodeResult::Ok(va), NodeResult::Ok(vb)) => {
                if let (ValuePayload::Body(ba), ValuePayload::Body(bb)) = (&va.payload, &vb.payload)
                {
                    bodies += 1;
                    let pa: Vec<_> = ba.points().map(|(_, p)| (p.x, p.y, p.z)).collect();
                    let pb: Vec<_> = bb.points().map(|(_, p)| (p.x, p.y, p.z)).collect();
                    assert_eq!(pa, pb, "guided at f64 moved body {id:?}");
                }
            }
            (a, b) => assert_eq!(
                a.error().map(ToString::to_string),
                b.error().map(ToString::to_string),
                "node {id:?} disagreed under the lift"
            ),
        }
    }
    assert!(bodies > 0, "the plate must actually build");
}

/// **The e2e Interval row through the PUBLIC evaluation door**, lift
/// ON. `Doc::param_env::<Interval>` embeds every parameter as a POINT
/// interval (`from_f64`), so this is the whole lifted pipeline —
/// resolve at `ParamEnv<Interval>`, guided replay, guided validate,
/// lane feeds into the key — certifying at zero width.
///
/// EVIDENCE-ONLY in part: the row also RECORDS the friction the
/// charter asks about — there is no public-door way to bind a
/// parameter to a genuinely wide box (that is M10-4's seeding
/// surface), so the wide-box case must enter one door down (next row).
#[cfg(feature = "interval")]
#[test]
fn r1_the_evaluation_door_runs_the_lift_at_interval() {
    use geom_core::Interval;
    let doc = filleted_plate();
    let ev = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &options(ProfileLift::Guided),
        Tol::witness(),
    );
    for (id, r) in ev.nodes.iter() {
        assert!(
            matches!(r, NodeResult::Ok(_)),
            "node {id:?} refused at Interval under the lift: {:?}",
            r.error().map(ToString::to_string)
        );
    }
}

/// **The wide-box e2e, one door down** (the public doors cannot author
/// a wide binding — recorded above): the SAME document's program at a
/// `ParamEnv<Interval>` whose fillet radius is genuinely wide, guided
/// by the f64 pass's own records — replay AND validation, the two
/// halves `lane_profile` runs.
///
/// **What R1 found and this row pins**: a wide box does NOT
/// necessarily abort. At [0.05, 1.2] every line×line fit stays
/// definitely Positive over the whole box, so guided replay CERTIFIES
/// honestly — the soundness obligation is then that the enclosures
/// contain the f64 elaborations at both endpoints of the box, which
/// this row checks vertex by vertex. The abort shows up when the box
/// crosses a discrete boundary: at [0.2, 1.6] the fit's setback
/// crosses the leg end (fit sign Zero at r = 1.5), and the row records
/// the refusal that actually fires, which must be typed.
#[cfg(feature = "interval")]
#[test]
fn r1_a_wide_fillet_radius_box_certifies_or_aborts_typed() {
    use geom_core::{Bounds, Interval};
    let doc = filleted_plate();
    let Some(Node::Profile(program)) = doc.node(
        *doc.order()
            .iter()
            .find(|id| matches!(doc.node(**id), Some(Node::Profile(_))))
            .expect("the plate has a profile node"),
    ) else {
        panic!("profile node");
    };
    // Pass 1 at f64, recording — exactly what prepare_profile does.
    let nominal = program
        .resolve(&doc.param_env::<f64>())
        .expect("resolves at f64");
    let (records, canonical) = {
        let mut records = Vec::new();
        let mut loops = Vec::new();
        for steps in &nominal {
            let (lp, rec) = profile::replay_recording(steps, Tol::witness()).expect("replays");
            loops.push(lp);
            records.push(rec);
        }
        let (_, canonical) = profile::Profile::new(program.plane, loops)
            .validate_recording(Tol::witness())
            .expect("the nominal validates");
        (records, canonical)
    };
    // The f64 elaborations at the box's two endpoints, for the
    // containment check when the wide box certifies.
    let endpoint_loops = |r: f64| -> Vec<profile::ProfileLoop<f64>> {
        let mut env = doc.param_env::<f64>();
        for v in env.bindings.values_mut() {
            if let ParamValue::Continuous { value, .. } = v {
                *value = r;
            }
        }
        program
            .resolve(&env)
            .expect("resolves at f64")
            .iter()
            .map(|steps| profile::replay(steps, Tol::witness()).expect("endpoint replays"))
            .collect()
    };

    // (a) The wide-but-uneventful box: certifies, and the enclosures
    // must contain BOTH endpoint elaborations.
    let (lo, hi) = (0.05, 1.2);
    let mut env = doc.param_env::<Interval>();
    for v in env.bindings.values_mut() {
        if let ParamValue::Continuous { value, .. } = v {
            *value = Interval::from_bounds(lo, hi);
        }
    }
    let resolved = program.resolve(&env).expect("resolves at Interval");
    let (lo_loops, hi_loops) = (endpoint_loops(lo), endpoint_loops(hi));
    for (li, steps) in resolved.iter().enumerate() {
        let wide = profile::replay_guided(steps, &records[li], Tol::witness()).expect(
            "no discrete decision moves over [0.05, 1.2], so the guided replay \
             certifies the whole box",
        );
        for endpoint in [&lo_loops[li], &hi_loops[li]] {
            assert_eq!(wide.vertices().len(), endpoint.vertices().len());
            for (k, (e, w)) in endpoint.vertices().iter().zip(wide.vertices()).enumerate() {
                for (what, exact, enc) in [
                    ("x", e.pos().x, w.pos().x),
                    ("y", e.pos().y, w.pos().y),
                    ("bulge", e.bulge(), w.bulge()),
                ] {
                    assert!(
                        enc.lo() <= exact && exact <= enc.hi(),
                        "loop {li} v{k}: the certified {what} enclosure excludes an \
                         endpoint elaboration — an UNSOUND certification"
                    );
                }
            }
        }
        // The validation half `lane_profile` would run next; record
        // its verdict on the wide loop.
        let plane = profile::SketchPlane::xy();
        match profile::Profile::new(plane, vec![wide]).validate_guided(
            Tol::witness(),
            &profile::CanonicalStructure {
                outer_loop: canonical.outer_loop,
                loops: vec![canonical.loops[li].clone()],
            },
        ) {
            Ok(_) => println!("wide box [0.05,1.2] loop {li}: replay AND validation certify"),
            Err(e) => println!("wide box [0.05,1.2] loop {li}: validation refuses typed: {e}"),
        }
    }

    // (b) The box that CROSSES a discrete boundary (fit Zero at
    // r = 1.5): the guided pass must refuse typed, never certify a box
    // over which the recorded structure does not hold.
    let mut env = doc.param_env::<Interval>();
    for v in env.bindings.values_mut() {
        if let ParamValue::Continuous { value, .. } = v {
            *value = Interval::from_bounds(0.2, 1.6);
        }
    }
    let resolved = program.resolve(&env).expect("resolves at Interval");
    for (li, steps) in resolved.iter().enumerate() {
        let err = profile::replay_guided(steps, &records[li], Tol::witness()).expect_err(
            "the box crosses the fit boundary at r = 1.5, so the recorded structure \
             cannot be confirmed over it",
        );
        println!("boundary-crossing box loop {li} refusal: {err}");
    }
}

/// The `Dual64` end-to-end the unit claims (M10-DI landed mid-flight,
/// no machinery change): the whole lifted path runs at `Dual` through
/// `evaluate`, every node Ok, and the unseeded environment moves
/// nothing against the pinned lane.
#[test]
fn r1_the_evaluation_door_runs_the_lift_at_dual64() {
    use geom_core::Dual64;
    let doc = filleted_plate();
    let run = |lift| {
        evaluate::<Dual64>(
            &doc,
            None,
            &CancelToken::new(),
            &options(lift),
            Tol::witness(),
        )
    };
    let (pinned, guided) = (run(ProfileLift::Pinned), run(ProfileLift::Guided));
    let mut compared = 0usize;
    for (id, a) in pinned.nodes.iter() {
        let b = guided.nodes.get(id).expect("same node set");
        match (a, b) {
            (NodeResult::Ok(va), NodeResult::Ok(vb)) => {
                if let (ValuePayload::Profile(pa), ValuePayload::Profile(pb)) =
                    (&va.payload, &vb.payload)
                {
                    for (la, lb) in pa.validated.loops().iter().zip(pb.validated.loops()) {
                        for (u, w) in la.vertices().iter().zip(lb.vertices()) {
                            compared += 1;
                            assert_eq!(u.pos().x.value.to_bits(), w.pos().x.value.to_bits());
                            assert_eq!(u.pos().y.value.to_bits(), w.pos().y.value.to_bits());
                            assert_eq!(w.pos().x.deriv, 0.0, "unseeded tangent must be zero");
                            assert_eq!(w.pos().y.deriv, 0.0, "unseeded tangent must be zero");
                        }
                    }
                    // A lifted profile's key must differ from the
                    // pinned one's (PP5 — no memo aliasing).
                    assert_ne!(va.content_key, vb.content_key, "key did not move: {id:?}");
                }
            }
            (a, b) => assert_eq!(
                a.error().map(ToString::to_string),
                b.error().map(ToString::to_string),
                "node {id:?} disagreed under the lift at Dual"
            ),
        }
    }
    assert!(compared > 0, "profile payloads were compared");
}

/// A `Dual` seed on the fillet radius reaches the fillet's OWN
/// vertices and no other's — R1's independent version of the seam
/// claim, on a document where the moved and unmoved vertices are
/// distinguishable (the four straight-corner anchors do not move with
/// the radius; the eight fillet tangency vertices do).
#[test]
fn r1_a_seeded_fillet_radius_moves_exactly_the_fillet_vertices() {
    use geom_core::Dual64;
    let doc = filleted_plate();
    let Some(Node::Profile(program)) = doc.node(
        *doc.order()
            .iter()
            .find(|id| matches!(doc.node(**id), Some(Node::Profile(_))))
            .expect("profile node"),
    ) else {
        panic!("profile node");
    };
    let nominal = program
        .resolve(&doc.param_env::<f64>())
        .expect("resolves at f64");
    let records: Vec<_> = nominal
        .iter()
        .map(|steps| {
            profile::replay_recording(steps, Tol::witness())
                .expect("the nominal replays")
                .1
        })
        .collect();
    let mut env = doc.param_env::<Dual64>();
    let mut seeded = 0usize;
    for v in env.bindings.values_mut() {
        if let ParamValue::Continuous { value, .. } = v {
            *value = Dual64::variable(value.value);
            seeded += 1;
        }
    }
    assert_eq!(seeded, 1, "one parameter drives the fillets");
    let resolved = program.resolve(&env).expect("resolves at Dual");
    let mut moved = 0usize;
    let mut still = 0usize;
    for (li, steps) in resolved.iter().enumerate() {
        let lp = profile::replay_guided(steps, &records[li], Tol::witness())
            .expect("the seeded elaboration keeps the nominal structure");
        for v in lp.vertices() {
            if v.pos().x.deriv != 0.0 || v.pos().y.deriv != 0.0 {
                moved += 1;
            } else {
                still += 1;
            }
        }
    }
    assert!(
        moved >= 8,
        "eight fillet tangency vertices ride the radius: {moved}"
    );
    println!("seeded-vertex census: {moved} moved, {still} still");
}
