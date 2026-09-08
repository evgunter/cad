//! **Under the pinned lift the op's value is the pre-pass's validated
//! form, lifted — and that is the same value a validation at the lane
//! scalar mints.** For every Profile node of every corpus document,
//! at `f64`, `Dual64` and (in the interval lane) `Interval`, the
//! node's value under `ProfileLift::Pinned` equals what the program
//! replayed at `f64`, embedded through `from_f64` and validated at the
//! lane scalar produces: the same loops, every stored scalar the same
//! bits in every value channel (`Dual64`'s value, `Interval`'s bounds
//! — the arc carriers the lift re-derives included), and a derivative
//! channel that is zero either way. The evaluator makes the validation
//! decisions once (`kstats_bracket_rows` pins the log); this row pins
//! that skipping the second validation changes no value.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::documents;

use editor_core::{CancelToken, EvalOptions, EvalScalar, Node, ValuePayload, evaluate};
use geom_core::{Real, Sign, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SegmentKind, ValidatedProfile};

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
/// radius.
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
            let loops = program
                .resolve(&env)
                .expect("the corpus program resolves at f64")
                .iter()
                .map(|steps| embed::<T>(&profile::replay(steps, tol).expect("replays at f64")))
                .collect();
            let revalidated = Profile::new(*lifted.validated.plane(), loops)
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
