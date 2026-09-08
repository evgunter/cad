//! **Under the pinned lift the op's value is the pre-pass's validated
//! form, lifted — the same value a validation at the lane scalar
//! mints, on the plane the arm computes.** For every Profile node of
//! every corpus document, at `f64`, `Dual64` and (in the interval
//! lane) `Interval`, the node's value under `ProfileLift::Pinned`
//! equals what the program replayed at `f64`, embedded through
//! `from_f64` and validated at the lane scalar produces, on the plane
//! computed the way the arm computes it (an authored frame's `f64`
//! placement lifted; a derived frame's lane value): the same loops,
//! every stored scalar the same bits in every value channel (`Dual64`'s
//! value, `Interval`'s bounds — the arc carriers the lift rebuilds
//! included), and a derivative channel that is zero either way. The
//! evaluator makes the validation decisions once (`kstats_bracket_rows`
//! pins the log); these rows pin that skipping the second validation
//! changes no value. The last row pins what the lift DOES change: a
//! margin an `Interval` validation would escalate on is decided by its
//! f64 verdict under the pinned lift and escalates under the guided one.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::documents;

use editor_core::{
    CancelToken, Datum, DatumValue, EvalOptions, EvalScalar, Node, ValuePayload, evaluate,
};
use geom_core::{Real, Sign, Tol};
use profile::{
    Profile, ProfileLoop, ProfileVertex, RawLoop, SegmentKind, SketchPlane, ValidatedProfile,
};

/// The `f64` loop embedded at `T` through `from_f64`, vertex by
/// vertex, the declared joints carried — the raw profile the lane's
/// own validation would run on.
fn embed<T: Real>(lp: &ProfileLoop<f64>) -> ProfileLoop<T> {
    ProfileLoop::new(
        lp.vertices()
            .iter()
            .map(|v| ProfileVertex::new(v.pos().map(T::from_f64), T::from_f64(v.bulge())))
            .collect(),
    )
    .with_tangent_joints(lp.tangent_joints().to_vec())
}

/// Every scalar a validated profile stores, in one fixed order: the
/// plane's placement, then per loop each vertex's position and bulge,
/// then each segment's endpoints, bulge and (for an arc) center and
/// radius. (`profile`'s `validated_map` suite carries the same walk:
/// `test-utils` is a dependency-free leaf and cannot host a walk over
/// `profile`'s types without a cycle.)
fn scalars<T: Real>(vp: &ValidatedProfile<T>) -> Vec<T> {
    let m = &vp.plane().placement;
    let mut out = vec![
        m.linear.c0.x,
        m.linear.c0.y,
        m.linear.c0.z,
        m.linear.c1.x,
        m.linear.c1.y,
        m.linear.c1.z,
        m.linear.c2.x,
        m.linear.c2.y,
        m.linear.c2.z,
        m.translation.x,
        m.translation.y,
        m.translation.z,
    ];
    for lp in vp.loops() {
        for v in lp.vertices() {
            out.extend([v.pos().x, v.pos().y, v.bulge()]);
        }
        for s in lp.segments() {
            out.extend([s.start.x, s.start.y, s.end.x, s.end.y, s.bulge]);
            if let SegmentKind::Arc { center, radius, .. } = s.kind {
                out.extend([center.x, center.y, radius]);
            }
        }
    }
    out
}

/// The structural facts of a validated profile, rendered: loop roles,
/// joint sets, segment kinds and turns.
fn structure<T: Real>(vp: &ValidatedProfile<T>) -> String {
    let mut out = String::new();
    for lp in vp.loops() {
        out.push_str(&format!("{:?} {:?}:", lp.role(), lp.tangent_joints()));
        for s in lp.segments() {
            out.push_str(match s.kind {
                SegmentKind::Line => " L",
                SegmentKind::Arc {
                    turn: Sign::Positive,
                    ..
                } => " +",
                SegmentKind::Arc { .. } => " -",
            });
        }
        out.push('\n');
    }
    out
}

/// One value channel of a scalar, named, projected to `f64` for a bit
/// comparison.
type Channel<T> = (&'static str, fn(T) -> f64);

fn the_lifted_form_is_the_revalidated_form<T: EvalScalar>(scalar: &str, channels: &[Channel<T>]) {
    let tol = Tol::witness();
    let mut profiles = 0usize;
    for d in documents() {
        let ev = evaluate::<T>(
            &d.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let at_f64 = evaluate::<f64>(
            &d.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            tol,
        );
        let env = d.doc.param_env::<f64>();
        for &id in &ev.order {
            let Some(Node::Profile(program)) = d.doc.node(id) else {
                continue;
            };
            let value = ev
                .value(id)
                .unwrap_or_else(|| panic!("{}: profile {id:?} evaluates at {scalar}", d.name));
            let ValuePayload::Profile(lifted) = &value.payload else {
                panic!("{}: {id:?} carries a profile", d.name);
            };
            // The plane the way the arm computes it, independently of
            // the value under test: an authored frame places at its
            // `f64` placement lifted (the `f64` run's plane, which at
            // `f64` IS that placement); a derived frame at the lane's
            // own frame value.
            let plane: SketchPlane<T> = match d.doc.node(program.plane) {
                Some(Node::Datum(Datum::Frame { .. })) => {
                    let ValuePayload::Profile(p) = &at_f64.value(id).expect("evaluates").payload
                    else {
                        panic!("{}: {id:?} carries a profile at f64", d.name);
                    };
                    p.validated.plane().map(T::from_f64)
                }
                _ => {
                    let ValuePayload::Datum(DatumValue::Frame { origin, u, v }) = &ev
                        .value(program.plane)
                        .expect("the frame evaluates")
                        .payload
                    else {
                        panic!("{}: {:?} is a frame", d.name, program.plane);
                    };
                    SketchPlane::from_frame(*origin, u.get(), v.get())
                }
            };
            let loops = program
                .resolve(&env)
                .expect("the corpus program resolves at f64")
                .iter()
                .map(|steps| embed::<T>(&profile::replay(steps, tol).expect("replays at f64")))
                .collect();
            let revalidated = Profile::new(plane, loops)
                .validate(tol)
                .unwrap_or_else(|e| panic!("{}: {id:?} validates at {scalar}: {e:?}", d.name));
            assert_eq!(
                structure(&lifted.validated),
                structure(&revalidated),
                "{}: profile {id:?} at {scalar}: structure",
                d.name
            );
            let (l, r) = (scalars(&lifted.validated), scalars(&revalidated));
            assert_eq!(l.len(), r.len(), "{}: profile {id:?} at {scalar}", d.name);
            for (channel, project) in channels {
                let bits = |xs: &[T]| xs.iter().map(|&x| project(x).to_bits()).collect::<Vec<_>>();
                assert!(
                    bits(&l) == bits(&r),
                    "{}: profile {id:?} at {scalar}: the lifted form's {channel} channel differs \
                     from the re-validated one:\n lifted {l:?}\n revalidated {r:?}",
                    d.name
                );
            }
            profiles += 1;
        }
    }
    assert!(profiles > 20, "the corpus carries profiles: {profiles}");
}

#[test]
fn the_lifted_form_is_the_revalidated_form_at_f64() {
    the_lifted_form_is_the_revalidated_form::<f64>("f64", &[("value", |x| x)]);
}

/// At `Dual64` the derivative channel of a pinned profile is zero
/// either way — the lift's by `from_f64`'s contract, the
/// re-validation's because a constant's arithmetic has none — and the
/// value channel is compared bit for bit. (The sign of that zero can
/// differ: a negated constant's derivative is `-0.0`; the door's doc
/// states it.)
#[test]
fn the_lifted_form_is_the_revalidated_form_at_dual() {
    use geom_core::Dual64;
    the_lifted_form_is_the_revalidated_form::<Dual64>("Dual64", &[("value", |d| d.value)]);
    for d in documents() {
        let ev = evaluate::<Dual64>(
            &d.doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        for &id in &ev.order {
            if let Some(v) = ev.value(id)
                && let ValuePayload::Profile(p) = &v.payload
            {
                assert!(
                    scalars(&p.validated).iter().all(|x| x.deriv == 0.0),
                    "{}: {id:?}: a pinned profile carries no tangent",
                    d.name
                );
            }
        }
    }
}

#[cfg(feature = "interval")]
#[test]
fn the_lifted_form_is_the_revalidated_form_at_interval() {
    use geom_core::Bounds;
    the_lifted_form_is_the_revalidated_form::<geom_core::Interval>(
        "Interval",
        &[("lo", |i| i.lo()), ("hi", |i| i.hi())],
    );
}

/// **A margin definite at `f64` and indeterminate at `Interval` is
/// decided by its f64 verdict under the pinned lift and escalates
/// under the guided one.** A step of exactly the escalation threshold
/// K·ε: the tiny edge's length is the axis-aligned distance `sqrt(d²)`,
/// which at `f64` is `d` exactly — the lever arm of the junction turn
/// at its far end, whose displacement margin is then exactly K·ε
/// (`|m| ≥ K·ε`: definite by equality) — and at `Interval` an
/// enclosure whose lower bound rounds below `d`, so the same predicate
/// cannot decide. Under `Pinned` at `Interval` the node is served with
/// the pre-pass's log (that junction decided in it) and no escalation;
/// under `Guided` the op's own replay at `Interval` re-verifies the
/// junction and refuses the node with `path_junction_turn` escalated —
/// that is where such a margin is meant to escalate (`ProfileLift`).
#[cfg(feature = "interval")]
#[test]
fn a_margin_definite_at_f64_and_indeterminate_at_interval_is_pinned_and_guided_apart() {
    use crate::fixture::on_frame;
    use editor_core::{DocumentId, NodeResult, ProfileDoc, ProfileLift};
    use geom_core::{Band, Interval};
    let tol = Tol::witness();
    let d = Band::linear(tol).expect("the linear band").escalate();
    let doc = ProfileDoc::empty(DocumentId::derive("eval8-band-edge"), tol);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, d),
            (2.0, d),
            (2.0, 1.0),
            (0.0, 1.0),
        ]],
    );
    let at_f64 = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let pre_pass = &at_f64.value(profile).expect("validates at f64").verdicts;
    assert!(
        pre_pass.iter().any(|v| v.predicate == "path_junction_turn"),
        "the pre-pass decided the junction at f64"
    );
    let pinned = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let served = pinned
        .value(profile)
        .expect("served under the pinned lift: the f64 verdicts are the authority");
    assert_eq!(
        served.verdicts[..],
        pre_pass[..],
        "the pinned log is the pre-pass's"
    );
    assert!(
        served.escalations.is_empty(),
        "nothing escalated: {:?}",
        served.escalations
    );
    let guided = evaluate::<Interval>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        },
        tol,
    );
    let err = guided
        .result(profile)
        .and_then(NodeResult::error)
        .expect("the guided op's validation at Interval refuses");
    let named: Vec<&str> = err.escalations.iter().map(|e| e.predicate()).collect();
    assert_eq!(
        named,
        ["path_junction_turn"],
        "the junction the tiny edge levers escalates at Interval: {}",
        err.kind
    );
}
