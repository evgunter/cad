//! **BLEND-6 review probes (R2)** — adversarial rows against the
//! verb-vocabulary unit's own claims, written against the public
//! doors and the public refusal type only.
//!
//! Three claims are probed, each one an EXECUTION of something the
//! unit argued in prose:
//!
//! 1. **The fillet door's single-render guard is blind to the very
//!    bug shape the unit was born measuring.** The shipped row counts
//!    occurrences of the exact string `"fillet: "`; the pre-fix
//!    baseline rendered `"fillet assembly: "` / `"fillet chain: "`,
//!    which that count cannot see. The stronger invariant — the
//!    wrapper supplies the verb, so the INNER error never opens with
//!    a verb word — is what V2 actually says, and it is asserted here
//!    on both doors.
//!
//! 2. **The chamfer-purity property is preserved by an unreachability
//!    argument that nothing guards.** The surgery's closed-chamfer arm
//!    renders a message that DOES speak the other verb once the
//!    conditioned assembly recourse is appended; it is unreachable
//!    today by a geometric argument recorded only in prose.
//!
//! 3. **The ball-only arm list is a measurement without a mechanical
//!    guard.** The three ball-gated variants are asserted absent from
//!    every refusal the chamfer door produces over the shipped
//!    fixtures, so the gate citations go red if a predicate is ever
//!    ungated.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::blend::build::fillet_edges;
use sweep::blend::{BlendError, BlendKind, BlendRefusal};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::cube;
use sweep::{Extrusion, extrude};
use topo::query;
use topo::{Body, EdgeKey, EntityId};

/// The cube side, meters.
const L: f64 = 1.0;
/// The blend size (radius or setback), meters.
const D: f64 = 0.1;

/// The shipped fillet-side guard, transcribed: what
/// `blend6_verb_vocab.rs`'s `assert_speaks_once_as_the_fillet`
/// actually tests. Returns `true` where that row would be GREEN.
fn shipped_fillet_guard_is_green(text: &str) -> bool {
    text.starts_with("fillet: ")
        && text.matches("fillet: ").count() == 1
        && !text.contains("chamfer")
}

/// **PROBE 1a — the shipped fillet-side guard cannot see the pre-fix
/// bug shape.**
///
/// Issue 917's measured residue was `Display` arms opening
/// `"fillet assembly: "` and `"fillet chain: "`, not `"fillet: "`.
/// Composed under the new wrapper those render the verb TWICE, and
/// the shipped row's `matches("fillet: ").count() == 1` still passes,
/// because `"fillet assembly: "` does not contain `"fillet: "`.
///
/// Built from the public refusal type with a `detail` carrying the
/// historical prefix, so the row states the blindness rather than
/// relying on a source mutation.
#[test]
fn the_shipped_fillet_single_render_guard_is_blind_to_the_prefix_family() {
    let body = cube(L, Tol::witness());
    let vertex = body
        .vertices()
        .map(|(k, _)| k)
        .next()
        .expect("a cube vertex");
    let regressed = BlendRefusal {
        verb: BlendKind::Fillet,
        error: BlendError::UnsupportedRunOut {
            at: EntityId::Vertex(vertex),
            detail: "fillet assembly: a chain terminates at a trivalent vertex whose three \
                     edges are not all requested",
        },
    };
    let text = format!("{regressed}");

    // The verb is plainly rendered twice.
    assert!(text.starts_with("fillet: fillet assembly: "), "{text}");

    // And the shipped guard is GREEN on it anyway. This is the finding.
    assert!(
        shipped_fillet_guard_is_green(&text),
        "if this row ever goes red the shipped guard has been strengthened, \
         and this probe can be retired: {text}"
    );
}

/// **PROBE 1b — the invariant V2 actually states, on both doors.**
///
/// "The `Display` literals drop their hard-coded verb (the wrapper
/// prefixes)" means the INNER error never opens with a verb word.
/// That is one assertion, it is symmetric across the doors, and it
/// goes red on the pre-fix shape which 1a shows the shipped row
/// cannot see.
#[test]
fn no_inner_error_opens_with_a_verb_word_on_either_door() {
    let inner_opens_with_a_verb = |error: &BlendError| -> bool {
        let inner = format!("{error}");
        inner.starts_with("fillet") || inner.starts_with("chamfer")
    };

    for (label, error) in reachable_refusals() {
        assert!(
            !inner_opens_with_a_verb(&error),
            "the {label} arm's inner Display opens with a verb word; the wrapper is \
             the one place the verb crosses: {error}"
        );
    }

    // The regressed shape 1a builds is what this invariant catches.
    let body = cube(L, Tol::witness());
    let vertex = body
        .vertices()
        .map(|(k, _)| k)
        .next()
        .expect("a cube vertex");
    let regressed = BlendError::UnsupportedRunOut {
        at: EntityId::Vertex(vertex),
        detail: "fillet assembly: a chain terminates at a trivalent vertex",
    };
    assert!(
        inner_opens_with_a_verb(&regressed),
        "the invariant must be able to see the pre-fix shape"
    );
}

/// **PROBE 2 — the chamfer-purity property rests on an unguarded
/// unreachability argument.**
///
/// The surgery's closed-chamfer arm (`surgery.rs`, the `Chamfer` arm
/// of the closure match) mints `UnsupportedChain`, whose `Display`
/// appends the CONDITIONED assembly recourse — and that recourse
/// names the other verb ("for a fillet, closed chains that are
/// circular plane–sphere rims also carve"). So if that arm ever
/// fires, a chamfer caller reads a message containing "fillet", which
/// is exactly what the unit's own `assert_speaks_as_the_chamfer`
/// forbids.
///
/// The arm is argued unreachable (a G1-closed chain of plane–plane
/// links has line carriers and cannot close). Nothing computes that
/// argument; this row makes the consequence explicit.
#[test]
fn the_closed_chamfer_arm_would_speak_the_other_verb_if_it_ever_fired() {
    let body = cube(L, Tol::witness());
    let edge = query::all_edges(&body)[0];
    let refusal = BlendRefusal {
        verb: BlendKind::Chamfer,
        error: BlendError::UnsupportedChain {
            edge,
            detail: "a closed chamfer chain has no band; only open chains between \
                     trivalent corners are implemented",
        },
    };
    let text = format!("{refusal}");
    let without_rosters = text.replace("fillet3_", "");
    assert!(
        without_rosters.contains("fillet"),
        "if this row goes red the conditioned clause was reworded and the tension \
         is gone: {text}"
    );

    // And the shipped purity helper's own predicate is what it fails.
    assert!(text.starts_with("chamfer: "), "{text}");
}

/// **PROBE 3 — the ball-only list, as a guard rather than a
/// measurement.**
///
/// `RadiusHeadroom`, `SpineIrregular` and `SpineUnsupported` are
/// claimed unreachable from the chamfer battery, each with a gate
/// cited in prose. This asserts the consequence over every chamfer
/// refusal the shipped fixtures reach, so ungating a predicate goes
/// red here instead of being noticed by a reader.
#[test]
fn no_chamfer_refusal_is_ever_one_of_the_ball_only_arms() {
    for (label, error) in chamfer_refusals() {
        assert!(
            !matches!(
                error,
                BlendError::RadiusHeadroom { .. }
                    | BlendError::SpineIrregular { .. }
                    | BlendError::SpineUnsupported { .. }
            ),
            "the {label} chamfer request reached a ball-only arm: {error:?}"
        );
        // The escalation routes for the three ball-gated predicates are
        // the same claim one layer down.
        if let BlendError::Escalated { ref source, .. } = error {
            assert!(
                !matches!(
                    source.predicate,
                    Some(
                        "fillet3_radius_headroom"
                            | "fillet3_spine_regularity"
                            | "fillet3_support_coaxiality"
                    )
                ),
                "the {label} chamfer request metered a ball-only predicate: {error:?}"
            );
        }
    }
}

/// **PROBE 3b — the converse half of the split, executed**: the
/// chamfer-only arm is unreachable from the FILLET door on the same
/// fixture that reaches it from the chamfer door.
///
/// A circular prism's rim edges carry a plane and a cylinder. Through
/// `chamfer_edges` that is the chamfer's own arm-table refusal; through
/// `fillet_edges` the same edges go to the analytic table instead, so
/// `ChamferArmUnsupported` — claimed chamfer-only — must never appear.
/// Together with probe 3 this pins both directions of the V2 split
/// rather than only the ball-only half.
#[test]
fn the_chamfer_only_arm_is_unreachable_from_the_fillet_door() {
    let cyl = cylinder(0.5, 1.0);
    let edges = query::all_edges(&cyl);
    let t = Tol::witness();

    // The chamfer door reaches its own arm on this fixture.
    let chamfered =
        chamfer_edges(&cyl, &edges, D, t).expect_err("a curved support has no ruled strip");
    assert!(
        matches!(chamfered.error, BlendError::ChamferArmUnsupported { .. }),
        "the fixture must reach the chamfer's own arm: {:?}",
        chamfered.error
    );

    // The fillet door, same fixture, same size: whatever it answers,
    // it is never the chamfer's arm.
    if let Err(refusal) = fillet_edges(&cyl, &edges, D, t) {
        assert!(
            !matches!(refusal.error, BlendError::ChamferArmUnsupported { .. }),
            "a fillet reached the chamfer-only arm: {:?}",
            refusal.error
        );
    }

    // And per-edge, so a whole-body refusal cannot mask the claim.
    for e in &edges {
        if let Err(refusal) = fillet_edges(&cyl, &[*e], D, t) {
            assert!(
                !matches!(refusal.error, BlendError::ChamferArmUnsupported { .. }),
                "a single-edge fillet reached the chamfer-only arm: {:?}",
                refusal.error
            );
        }
    }
}

/// A circular prism: two half-arc profile segments extruded, so every
/// rim edge has a plane and a CYLINDER for supports.
fn cylinder(r: f64, h: f64) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        ProfileVertex::new(Point2::new(r, 0.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a circle is a valid profile");
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .expect("a circular prism")
        .body
}

/// Every refusal the shipped fixtures reach through EITHER door, each
/// labelled. Used by the verb-neutrality invariant.
fn reachable_refusals() -> Vec<(&'static str, BlendError)> {
    let mut out = chamfer_refusals();
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let t = Tol::witness();

    out.push((
        "fillet run-out",
        fillet_edges(&body, &edges[..1], D, t)
            .expect_err("a partially-requested corner is a run-out")
            .error,
    ));
    out.push((
        "fillet clearance",
        fillet_edges(&body, &edges, 0.55, t)
            .expect_err("a 0.55 m radius does not fit a 1 m face")
            .error,
    ));
    out.push((
        "fillet chain-break",
        fillet_edges(&body, &top_loop(&body), D, t)
            .expect_err("square junctions are not tangent-continuous")
            .error,
    ));
    out
}

/// Every refusal the shipped fixtures reach through the CHAMFER door.
fn chamfer_refusals() -> Vec<(&'static str, BlendError)> {
    let body = cube(L, Tol::witness());
    let edges = query::all_edges(&body);
    let t = Tol::witness();
    let mut out = vec![(
        "nonpositive size",
        chamfer_edges(&body, &edges[..1], 0.0, t)
            .expect_err("a zero setback has no band")
            .error,
    )];

    out.push((
        "repeated edge",
        chamfer_edges(&body, &[edges[0], edges[0]], D, t)
            .expect_err("a repeated edge doubles a link")
            .error,
    ));
    out.push((
        "run-out",
        chamfer_edges(&body, &edges[..1], D, t)
            .expect_err("a partially-requested corner is a run-out")
            .error,
    ));
    out.push((
        "chain-break",
        chamfer_edges(&body, &top_loop(&body), D, t)
            .expect_err("square junctions are not tangent-continuous")
            .error,
    ));
    out.push((
        "clearance",
        chamfer_edges(&body, &edges, 0.55, t)
            .expect_err("two 0.55 m setbacks do not fit a 1 m face")
            .error,
    ));
    let eps = t.get().eps;
    out.push((
        "escalated clearance",
        chamfer_edges(&body, &edges, 0.5 - 2.5 * eps, t)
            .expect_err("an in-band clearance margin escalates")
            .error,
    ));

    let bracket = l_bracket();
    let concave = concave_edge(&bracket);
    out.push((
        "corner configuration",
        chamfer_edges(&bracket, &[concave], D, t)
            .expect_err("a mixed-convexity corner is out of scope")
            .error,
    ));

    let mut two = cube(L, Tol::witness());
    let other = cube(L, Tol::witness());
    topo::instance::graft_disjoint_all(&mut two, &other, Tol::witness()).expect("a disjoint graft");
    let two_edges = query::all_edges(&two);
    out.push((
        "two-solid body",
        chamfer_edges(&two, &two_edges[..1], D, t)
            .expect_err("the in-place surgery is built for one solid")
            .error,
    ));

    out
}

/// The four edges of the cube's top face — a closed, non-tangent chain.
fn top_loop(body: &Body<f64>) -> Vec<EdgeKey> {
    let at_top = |e: EdgeKey| -> bool {
        let Some(edge) = body.get_edge(e) else {
            return false;
        };
        let Some(start) = body.get_half_edge(edge.he_plus).map(|h| h.start) else {
            return false;
        };
        let Some(end) = body.half_edge_end(edge.he_plus) else {
            return false;
        };
        [start, end].into_iter().all(|v| {
            body.get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .is_some_and(|p| p.z > L - 1e-9)
        })
    };
    query::all_edges(body)
        .into_iter()
        .filter(|e| at_top(*e))
        .collect()
}

/// An L-bracket: the six-vertex L profile extruded by 1 m.
fn l_bracket() -> Body<f64> {
    let lp = ProfileLoop::new(
        [
            (0.0, 0.0),
            (1.0, 0.0),
            (1.0, 0.5),
            (0.5, 0.5),
            (0.5, 1.0),
            (0.0, 1.0),
        ]
        .into_iter()
        .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
        .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("an L is a valid profile");
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .expect("an L-bracket extrudes")
        .body
}

/// The bracket's one concave edge — the vertical through (0.5, 0.5).
///
/// Taken from the SAME body instance it will be used against, which
/// the shipped suite's corner row does not do (it calls `l_bracket()`
/// twice and indexes one body with the other's key).
fn concave_edge(body: &Body<f64>) -> EdgeKey {
    let near = |p: geom_core::Point3<f64>| (p.x - 0.5).abs() < 1e-9 && (p.y - 0.5).abs() < 1e-9;
    let vertical = |e: EdgeKey| -> bool {
        let Some(edge) = body.get_edge(e) else {
            return false;
        };
        let (Some(start), Some(end)) = (
            body.get_half_edge(edge.he_plus).map(|h| h.start),
            body.half_edge_end(edge.he_plus),
        ) else {
            return false;
        };
        [start, end].into_iter().all(|v| {
            body.get_vertex(v)
                .and_then(|x| body.get_point(x.point))
                .is_some_and(|p| near(*p))
        })
    };
    body.edges()
        .map(|(k, _)| k)
        .find(|e| vertical(*e))
        .expect("the L-bracket has a vertical edge through its reflex corner")
}
