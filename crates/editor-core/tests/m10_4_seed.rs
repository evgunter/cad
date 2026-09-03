//! **The E4 seed door** (`docs/ERROR-DESIGN.md` E4; `docs/M10-4-SPEC.md`
//! §1): `EvalOptions::seed`, the `param_box` seam's twin.
//!
//! What the rows pin, in the spec's own order:
//!
//! - **Zero impact unseeded.** `seed: None` is the default and leaves
//!   the environment exactly as the nominal or box door built it; the
//!   build-path bit-identity fence (`m10_p_fence.rs`) is the corpus-wide
//!   differential and runs unchanged beside this suite.
//! - **Capability is at the scalar.** A tangentless scalar (`f64`,
//!   `Interval`) refuses a seed on every node, typed; an unknown or
//!   `Count` name refuses at env construction at every scalar.
//! - **Seed hygiene.** Tangent exactly `1.0` on the seeded parameter's
//!   lift and exactly `0.0` on every other binding and every literal —
//!   read through the public `seed_env` door and through a measure's
//!   public tangent field.
//! - **DL2 exercised, not trusted.** A pass threaded from ANOTHER
//!   parameter's pass as its memo prior reads its own tangent, bit for
//!   bit, and reuses only the seed-independent subgraph.
//! - **The REQUIRED profile pin.** A seed on a profile dimension
//!   reaches the measure through the guided lift with the analytically
//!   correct nonzero tangent, and the pinned lift shows the silent zero
//!   the lift exists to end.
//! - **Composition** (interval rows): `seed` and `param_box` together
//!   are legal exactly at `Dual<Interval>`; `Dual64` keeps the
//!   degenerate-only box rule; `Interval` plus a seed refuses.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod corpus;
mod fixture;

use editor_core::UnitSym;
use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Evaluation, Expr, LoopProgram,
    MeasureExpr, MeasurePrimitive, MeasureRef, Node, NodeErrorKind, NodeResult, ParamName,
    ParamValue, ProfileDoc, ProfileLift, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId,
    SeedError, ValuePayload, evaluate, seed_env,
};
use geom_core::{Dual64, Tol};

use fixture::{Recorder, fname, len, wall};

fn name(n: &str) -> ParamName {
    ParamName::new(n)
}

fn param(n: &str) -> Expr {
    Expr::param(name(n), Dimension::Length)
}

fn continuous(value: f64) -> DocParam {
    DocParam::Continuous {
        dim: Dimension::Length,
        value,
        display_unit: UnitSym::canonical_for(Dimension::Length),
        distribution: None,
    }
}

fn opts(seed: Option<&str>, lift: ProfileLift) -> EvalOptions {
    EvalOptions {
        seed: seed.map(name),
        profile_lift: lift,
        ..EvalOptions::default()
    }
}

fn run<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    prior: Option<&Evaluation<T>>,
    o: &EvalOptions,
) -> Evaluation<T> {
    evaluate::<T>(doc, prior, &CancelToken::new(), o, Tol::witness())
}

/// The measure node of a document, by kind — the one sink these rows
/// read.
fn measure_node(doc: &ProfileDoc) -> RecipeNodeId {
    doc.order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Measure { .. })))
        .expect("the document carries a measure")
}

/// The measure payload of an evaluation at `Dual64`, both channels.
fn measured(ev: &Evaluation<Dual64>, id: RecipeNodeId) -> Dual64 {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => panic!("node {id:?} is a {}", other.kind_name()),
        },
        other => panic!("node {id:?} did not evaluate: {other:?}"),
    }
}

/// **The two-parameter measured plate**: the corpus's measured web
/// (`distance(wall, wall) − 2·hole_r`, `hole_r` driving two hole
/// profiles through the lift) with a SECOND parameter `depth` on the
/// plate's extrude distance — a magnitude slot — added to the measure
/// as `+ depth`, so one measure reads a profile-driven and a
/// magnitude-driven parameter at once: ∂m/∂hole_r = −2, ∂m/∂depth = 1.
fn two_param_web() -> ProfileDoc {
    let base = corpus::measured_web::document();
    let mut doc = base.doc;
    let push = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| -> ProfileDoc {
        editor_core::apply(doc, &edit, Tol::witness())
            .unwrap_or_else(|e| panic!("edit refused: {e}"))
            .doc
    };
    doc = push(
        &doc,
        DocEdit::SetDocParam {
            name: name("depth"),
            value: continuous(0.1),
        },
    );
    // The plate is the corpus web's extrude; its distance becomes the
    // parameter. Found by kind, not by literal id: the sketch frame
    // is a node too, so a profile's own index is no longer one less
    // than its extrude's.
    let plate = doc
        .order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
        .expect("the corpus web extrudes its plate");
    doc = push(
        &doc,
        DocEdit::SetParam {
            node: plate,
            slot: editor_core::SlotId::Distance,
            expr: param("depth"),
        },
    );
    let old = measure_node(&doc);
    let Some(Node::Measure { expr, refs }) = doc.node(old).cloned() else {
        panic!("the corpus web is a measure")
    };
    let with_depth = MeasureExpr::add(expr, MeasureExpr::value(param("depth"))).expect("Length");
    // Replace the measure: the assertion depends on the old node, so
    // it goes first (cascade), then the new measure is inserted.
    let assertion = doc
        .order()
        .iter()
        .copied()
        .find(|&id| matches!(doc.node(id), Some(Node::Assertion { .. })))
        .expect("the corpus web carries an assertion");
    doc = push(&doc, DocEdit::DeleteNode { id: assertion });
    doc = push(&doc, DocEdit::DeleteNode { id: old });
    doc = push(
        &doc,
        DocEdit::InsertNode {
            node: Node::measure(with_depth, refs).expect("indices in range"),
        },
    );
    doc
}

/// **The width slab — M10-P's shape for the REQUIRED profile pin.** A
/// rectangle whose WIDTH is a document parameter (`w`, a profile
/// dimension: it feeds a chain program's point, not a magnitude slot),
/// extruded by a literal; the measure is the distance between its two
/// `x`-walls, which is `w` exactly. Returns the document and the
/// measure node.
fn width_slab(w: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: name("w"),
        value: continuous(w),
    });
    let chain = LoopProgram::Chain(vec![
        ProgramStep::At([len(0.0), len(0.0)]),
        ProgramStep::LineTo(ProgramTarget::Point([param("w"), len(0.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([param("w"), len(1.0)])),
        ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(1.0)])),
        ProgramStep::LineTo(ProgramTarget::Start),
    ]);
    let frame = r.insert(fixture::xy_frame());
    let p = r.insert(Node::Profile(ProfileProgram {
        plane: frame,
        loops: vec![chain],
    }));
    let slab = r.insert(Node::Extrude {
        profile: p,
        distance: len(1.0),
    });
    // Segment 3 is the x = 0 wall, segment 1 the x = w wall (chain
    // order: bottom, right, top, left).
    let refs = vec![
        MeasureRef::new(slab, fname(slab, wall(3))),
        MeasureRef::new(slab, fname(slab, wall(1))),
    ];
    let width = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let m = r.insert(Node::measure(width, refs).expect("both indices address a reference"));
    (r.doc, m)
}

/// A document with a `Count` parameter beside a continuous one.
fn with_count() -> ProfileDoc {
    let (mut doc, _) = width_slab(2.0);
    doc = editor_core::apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name("n"),
            value: DocParam::Count { value: 3 },
        },
        Tol::witness(),
    )
    .expect("a count parameter")
    .doc;
    doc
}

fn every_node_refuses_seed(ev: &Evaluation<impl geom_core::Decide>, expect: impl Fn(&SeedError)) {
    assert!(!ev.order.is_empty());
    for id in &ev.order {
        match ev.result(*id) {
            Some(NodeResult::Failed(e)) => match &e.kind {
                NodeErrorKind::Seed { source } => expect(source),
                other => panic!("node {} refused for another reason: {other}", id.0),
            },
            other => panic!("node {} did not refuse the seed: {other:?}", id.0),
        }
    }
}

// --------------------------------------------------------- unseeded

/// `seed: None` IS the default, and an explicit `None` is the default
/// evaluation node for node — same keys, same reuse accounting. The
/// corpus-wide bit-identity differential against the pre-seed build
/// path is `m10_p_fence.rs`, which this unit leaves untouched.
#[test]
fn an_unseeded_evaluation_is_the_default_options_evaluation() {
    assert!(EvalOptions::default().seed.is_none());
    for doc in corpus::documents() {
        let default = run::<f64>(&doc.doc, None, &EvalOptions::default());
        let explicit = run::<f64>(&doc.doc, None, &opts(None, ProfileLift::Pinned));
        assert_eq!(default.order, explicit.order, "{}", doc.name);
        for id in &default.order {
            let (a, b) = (default.value(*id), explicit.value(*id));
            assert_eq!(
                a.map(|v| v.content_key),
                b.map(|v| v.content_key),
                "{}: node {} keyed differently under an explicit `seed: None`",
                doc.name,
                id.0
            );
        }
    }
}

// -------------------------------------------------------- refusals

/// A point scalar has no tangent channel: a seeded `f64` evaluation
/// refuses every node typed rather than dropping the seed.
#[test]
fn a_seed_at_f64_refuses_every_node_typed() {
    let doc = two_param_web();
    let ev = run::<f64>(&doc, None, &opts(Some("hole_r"), ProfileLift::Pinned));
    every_node_refuses_seed(&ev, |e| {
        assert_eq!(
            *e,
            SeedError::TangentUnrepresentable {
                param: name("hole_r")
            }
        );
    });
}

/// An unknown or structural name refuses at env construction — before
/// any node runs, and identically at a scalar that COULD carry a seed.
#[test]
fn an_unknown_or_count_seed_refuses_at_env_construction() {
    let doc = with_count();
    let unknown = run::<Dual64>(&doc, None, &opts(Some("nope"), ProfileLift::Pinned));
    every_node_refuses_seed(&unknown, |e| {
        assert_eq!(
            *e,
            SeedError::UnknownParam {
                param: name("nope")
            }
        );
    });
    let count = run::<Dual64>(&doc, None, &opts(Some("n"), ProfileLift::Pinned));
    every_node_refuses_seed(&count, |e| {
        assert_eq!(*e, SeedError::CountParam { param: name("n") });
    });
    // The same door, called directly: the name is checked against the
    // DOCUMENT first, so an unknown name refuses as unknown even at a
    // scalar that would have refused the tangent.
    assert_eq!(
        seed_env::<f64, _>(&doc, doc.param_env::<f64>(), &name("nope")).err(),
        Some(SeedError::UnknownParam {
            param: name("nope")
        })
    );
    assert_eq!(
        seed_env::<f64, _>(&doc, doc.param_env::<f64>(), &name("w")).err(),
        Some(SeedError::TangentUnrepresentable { param: name("w") })
    );
}

// --------------------------------------------------------- hygiene

/// The seed is exactly `1.0` on the seeded lift and exactly `0.0` on
/// every other binding — by construction, read off the public door.
#[test]
fn the_seed_is_exactly_one_and_zero_by_construction() {
    let doc = two_param_web();
    let env = seed_env::<Dual64, _>(&doc, doc.param_env::<Dual64>(), &name("hole_r"))
        .expect("hole_r is continuous");
    let binding = |n: &str| match env.bindings[&name(n)] {
        ParamValue::Continuous { value, .. } => value,
        ParamValue::Count(_) => panic!("{n} is continuous"),
    };
    assert_eq!(binding("hole_r").deriv.to_bits(), 1.0f64.to_bits());
    assert_eq!(binding("hole_r").value.to_bits(), 0.2f64.to_bits());
    assert_eq!(binding("depth").deriv.to_bits(), 0.0f64.to_bits());
    assert_eq!(binding("depth").value.to_bits(), 0.1f64.to_bits());
}

/// Through the evaluation door: the web's tangent is `−2` on the
/// radius (the plate's own formula, `distance − 2·hole_r`, the axis
/// separation not depending on the radius) and `+1` on the depth term;
/// the unseeded parameter and every literal contribute exactly zero;
/// the value channel is the f64 build's number, bit for bit.
#[test]
fn the_web_tangent_is_the_plates_own_formula_through_the_public_door() {
    let doc = two_param_web();
    let m = measure_node(&doc);
    let f = run::<f64>(&doc, None, &EvalOptions::default());
    let Some(NodeResult::Ok(v)) = f.result(m) else {
        panic!("the web measures at f64")
    };
    let ValuePayload::Measure { value: web, .. } = v.payload else {
        panic!("a measure")
    };
    for lift in [ProfileLift::Pinned, ProfileLift::Guided] {
        let on_r = measured(&run::<Dual64>(&doc, None, &opts(Some("hole_r"), lift)), m);
        assert_eq!(on_r.deriv.to_bits(), (-2.0f64).to_bits(), "{lift:?}");
        assert_eq!(on_r.value.to_bits(), web.to_bits(), "{lift:?}");
        let on_d = measured(&run::<Dual64>(&doc, None, &opts(Some("depth"), lift)), m);
        assert_eq!(on_d.deriv.to_bits(), 1.0f64.to_bits(), "{lift:?}");
        assert_eq!(on_d.value.to_bits(), web.to_bits(), "{lift:?}");
        let unseeded = measured(&run::<Dual64>(&doc, None, &opts(None, lift)), m);
        assert_eq!(unseeded.deriv.to_bits(), 0.0f64.to_bits(), "{lift:?}");
    }
}

/// D9 at the evaluation level: a seeded pass's tangent is the same bits
/// under either schedule.
#[test]
fn a_seeded_pass_is_schedule_independent() {
    let doc = two_param_web();
    let m = measure_node(&doc);
    let seq = measured(
        &run::<Dual64>(&doc, None, &opts(Some("hole_r"), ProfileLift::Guided)),
        m,
    );
    let par = measured(
        &run::<Dual64>(
            &doc,
            None,
            &EvalOptions {
                parallel: true,
                ..opts(Some("hole_r"), ProfileLift::Guided)
            },
        ),
        m,
    );
    assert_eq!(seq.value.to_bits(), par.value.to_bits());
    assert_eq!(seq.deriv.to_bits(), par.deriv.to_bits());
}

// ------------------------------------------------------------- DL2

/// **DL2 exercised, not trusted.** A pass seeded on `depth`, threaded
/// from a pass seeded on `hole_r` as its memo prior, reads ITS OWN
/// tangent bit for bit — the prior's seed-downstream nodes carry other
/// tangent bits and cannot be served — while the seed-independent
/// subgraph IS served: reuse is positive and the pass recomputes
/// exactly what the two seeds' cones cover.
#[test]
fn the_memo_never_serves_one_parameters_pass_to_another() {
    let doc = two_param_web();
    let m = measure_node(&doc);
    let on_r = run::<Dual64>(&doc, None, &opts(Some("hole_r"), ProfileLift::Guided));
    let on_d_fresh = run::<Dual64>(&doc, None, &opts(Some("depth"), ProfileLift::Guided));
    let on_d_threaded = run::<Dual64>(&doc, Some(&on_r), &opts(Some("depth"), ProfileLift::Guided));
    let (fresh, threaded) = (measured(&on_d_fresh, m), measured(&on_d_threaded, m));
    assert_eq!(threaded.deriv.to_bits(), fresh.deriv.to_bits());
    assert_eq!(threaded.deriv.to_bits(), 1.0f64.to_bits());
    assert_eq!(threaded.value.to_bits(), fresh.value.to_bits());
    // The nodes downstream of EITHER seed differ in tangent bits
    // between the two passes and must recompute; the rest reuses.
    // Each seed's readers by what they READ, not by literal id: the
    // hole profiles are the programs whose expressions name `hole_r`,
    // and `depth` drives the plate extrude's distance (set above).
    let r_cone = doc
        .order()
        .iter()
        .copied()
        .filter(|&id| match doc.node(id) {
            Some(Node::Profile(p)) => p.references(&name("hole_r")),
            _ => false,
        })
        .flat_map(|id| corpus::cone(&doc, id))
        .collect::<std::collections::BTreeSet<_>>();
    let d_cone = corpus::cone(
        &doc,
        doc.order()
            .iter()
            .copied()
            .find(|&id| matches!(doc.node(id), Some(Node::Extrude { .. })))
            .expect("the corpus web extrudes its plate"),
    );
    let either: std::collections::BTreeSet<_> = r_cone.union(&d_cone).copied().collect();
    assert_eq!(on_d_threaded.recomputed, either.len());
    assert_eq!(on_d_threaded.reused, doc.len() - either.len());
    assert!(on_d_threaded.reused > 0, "nothing was seed-independent");
}

// ---------------------------------------------------- the profile pin

/// **The REQUIRED profile-dimension pin.** A seed on `w` — a chain
/// program's point, reachable only through the lift — reaches the
/// width measure with tangent exactly `1.0` under the GUIDED lift, and
/// exactly `0.0` under the pinned one: the silent zero the lift exists
/// to end, shown side by side.
#[test]
fn a_profile_dimension_seed_propagates_through_the_guided_lift() {
    let (doc, m) = width_slab(2.0);
    let guided = measured(
        &run::<Dual64>(&doc, None, &opts(Some("w"), ProfileLift::Guided)),
        m,
    );
    assert_eq!(guided.value.to_bits(), 2.0f64.to_bits());
    assert_eq!(
        guided.deriv.to_bits(),
        1.0f64.to_bits(),
        "∂width/∂w through the guided lift is 1, got {}",
        guided.deriv
    );
    let pinned = measured(
        &run::<Dual64>(&doc, None, &opts(Some("w"), ProfileLift::Pinned)),
        m,
    );
    assert_eq!(pinned.value.to_bits(), 2.0f64.to_bits());
    // IEEE equality on purpose: the zero arrives as `-0.0` (the
    // distance arm's `abs` multiplies its sign factor into a zero
    // tangent), and a signed zero is still the silent zero.
    assert_eq!(
        pinned.deriv, 0.0,
        "the pinned lift embeds profile geometry through from_f64 — a zero, not a tangent"
    );
}

// ------------------------------------------------------- the σ door

/// **DATUM — the truncated normal's variance formula over a grid.** The
/// σ door clamps its variance at zero before the root, on the claim
/// that rounding can push the two-sided truncation formula a few ulps
/// negative under extreme truncation. This row measures that claim:
/// the same formula, computed here independently over a grid of
/// windows from ±0.001σ to ±8σ, symmetric and one-sided, reports its
/// MINIMUM, and every σ the door returns is finite and non-negative.
/// The minimum is printed; the clamp is a guard on the printed number,
/// not a claim beyond it.
#[test]
fn the_truncated_normal_sigma_is_finite_and_the_variance_floor_is_measured() {
    use editor_core::{Distribution, std_deviation};
    let phi = |x: f64| (-0.5 * x * x).exp() / f64::sqrt(2.0 * core::f64::consts::PI);
    let mass =
        |a: f64, b: f64| 0.5 * (libm::erf(b / f64::sqrt(2.0)) - libm::erf(a / f64::sqrt(2.0)));
    let mut min_var = f64::INFINITY;
    let mut at = (0.0, 0.0);
    let mut rows = 0_usize;
    for &sigma in &[1e-3, 1.0, 40.0] {
        for &lo in &[-8.0, -3.0, -1.0, -0.1, -0.001, 0.0] {
            for &hi in &[0.0, 0.001, 0.1, 1.0, 3.0, 8.0] {
                if lo > hi || (lo == 0.0 && hi == 0.0) {
                    continue;
                }
                let dist = Distribution::TruncatedNormal {
                    sigma,
                    lo: lo * sigma,
                    hi: hi * sigma,
                };
                let s = std_deviation(&name("p"), &dist).expect("a truncated normal prices");
                assert!(s.is_finite() && s >= 0.0, "{dist:?}: σ = {s}");
                let z = mass(lo, hi);
                if z > 0.0 {
                    let var =
                        1.0 + (lo * phi(lo) - hi * phi(hi)) / z - ((phi(lo) - phi(hi)) / z).powi(2);
                    if var < min_var {
                        min_var = var;
                        at = (lo, hi);
                    }
                    // The door's number is the formula's where the
                    // formula is non-negative and its mass is not
                    // cancellation-dominated (the door measures Z
                    // through `erfc` half-lines, this row through an
                    // `erf` difference, and the two part company only
                    // where an `erf` difference has no digits left).
                    if var >= 0.0 && z > 1e-3 {
                        assert!(
                            (s - sigma * var.sqrt()).abs() <= 1e-9 * sigma,
                            "{dist:?}: door {s} vs formula {}",
                            sigma * var.sqrt()
                        );
                    }
                }
                rows += 1;
            }
        }
    }
    println!(
        "EVIDENCE-ONLY truncated-normal variance over {rows} windows: minimum {min_var:e} \
         (in σ² units) at ({}, {})σ; the clamp fires only below zero",
        at.0, at.1
    );
}

// ---------------------------------------------------- composition

/// `seed` and `param_box` together are legal exactly at
/// `Dual<Interval>`: the value channel carries the leaf's enclosure,
/// the tangent channel the seed. `Interval` plus a seed refuses on
/// every node; `Dual64` plus a widened box keeps the degenerate-only
/// rule and refuses through the box door.
#[cfg(feature = "interval")]
#[test]
fn seed_and_box_compose_exactly_at_dual_interval() {
    use std::sync::Arc;

    use editor_core::analysis::{BoxAxis, ParamBox};
    use geom_core::{Bounds, DualInterval, Interval};
    let doc = two_param_web();
    let m = measure_node(&doc);
    let mut axes = std::collections::BTreeMap::new();
    axes.insert(name("hole_r"), BoxAxis::Fixed);
    axes.insert(
        name("depth"),
        BoxAxis::Varying {
            lo: -0.01,
            hi: 0.01,
        },
    );
    let box_ = Arc::new(ParamBox::from_axes(axes));
    let both = EvalOptions {
        param_box: Some(Arc::clone(&box_)),
        ..opts(Some("depth"), ProfileLift::Guided)
    };
    let ev = run::<DualInterval>(&doc, None, &both);
    let Some(NodeResult::Ok(v)) = ev.result(m) else {
        panic!(
            "the web measures at Dual<Interval>: {:?}",
            ev.node_error(m).map(|e| e.kind.to_string())
        )
    };
    let ValuePayload::Measure { value, .. } = &v.payload else {
        panic!("a measure")
    };
    // The value channel encloses the web over the box (0.2 + depth
    // over depth ∈ 0.1 ± 0.01); the tangent channel is the seed.
    assert!(value.value.lo() <= 0.29 && 0.31 <= value.value.hi());
    assert_eq!(value.deriv.lo(), 1.0);
    assert_eq!(value.deriv.hi(), 1.0);

    let interval = run::<Interval>(&doc, None, &both);
    every_node_refuses_seed(&interval, |e| {
        assert_eq!(
            *e,
            SeedError::TangentUnrepresentable {
                param: name("depth")
            }
        );
    });

    let dual64 = run::<Dual64>(&doc, None, &both);
    for id in &dual64.order {
        assert!(
            matches!(
                dual64.result(*id),
                Some(NodeResult::Failed(e)) if matches!(e.kind, NodeErrorKind::ParamBox { .. })
            ),
            "node {} did not refuse the widened box at Dual64",
            id.0
        );
    }
}
