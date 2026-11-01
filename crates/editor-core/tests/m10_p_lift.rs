//! **The profile-parameter lift at the evaluation seam.**
//!
//! `plate_param` is the fixture throughout: the corpus's one document
//! whose profile geometry is an EXPRESSION, so a document parameter
//! drives two loops at once. Before the lift, a `Dual` seed on that
//! parameter propagated no tangent and an interval binding widened
//! nothing — profile geometry reached every lane as f64 bits. These
//! rows are what changed and what did not.
//!
//! - **The `f64` lane is the first pass.** It takes the first pass's
//!   validated form, runs no second one and logs the first's decisions
//!   in its place (`editor_core::ProfileLane`); here the claim that
//!   justifies it: the second pass, run at `f64`, is that form and that
//!   log bit for bit, over the whole corpus.
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

use crate::corpus;

use editor_core::{CancelToken, EvalOptions, Node, NodeResult, ParamValue, ValuePayload, evaluate};
use geom_core::Tol;
use profile::ValidatedProfile;

/// A validated profile's stored scalars as bits and its decided facts
/// (`profile::test_support`).
fn form(vp: &ValidatedProfile<f64>) -> (Vec<u64>, String) {
    use profile::test_support::{decided_facts, stored_scalars};
    let bits = stored_scalars(vp).iter().map(|x| x.to_bits()).collect();
    (bits, decided_facts(vp))
}

/// **The `f64` lane's profile is the second pass, bit for bit, and so
/// is its log.** At `f64` over the nominal the evaluator serves the
/// first pass's validated form on the lane's plane, runs no second
/// pass, and logs the first pass's decisions again in its place
/// (`editor_core::ProfileLane`). Here the same evaluation runs the
/// second pass through the evaluator's own path
/// (`editor_core::eval::evaluate_running_pass_two`), for every corpus
/// document, and the two agree on every node's outcome and verdict and
/// escalation logs, and on every profile's stored scalars' bits and
/// decided facts.
#[test]
fn the_f64_lane_is_the_second_pass_bit_for_bit() {
    let tol = Tol::witness();
    let opts = EvalOptions::default();
    let mut profiles = 0usize;
    for doc in corpus::documents() {
        let skipped = evaluate::<f64>(&doc.doc, None, &CancelToken::new(), &opts, tol);
        let ran = editor_core::eval::evaluate_running_pass_two::<f64>(&doc.doc, &opts, tol);
        for (&id, result) in &skipped.nodes {
            let second = ran.nodes.get(&id);
            let (NodeResult::Ok(v), Some(NodeResult::Ok(w))) = (result, second) else {
                assert_eq!(
                    format!("{result:?}"),
                    format!("{second:?}"),
                    "{}: node {id:?}: the outcome differs with the second pass run",
                    doc.name
                );
                continue;
            };
            assert_eq!(
                (&v.verdicts, &v.escalations),
                (&w.verdicts, &w.escalations),
                "{}: node {id:?}: the log differs with the second pass run",
                doc.name
            );
            let (ValuePayload::Profile(served), ValuePayload::Profile(run)) =
                (&v.payload, &w.payload)
            else {
                continue;
            };
            assert_eq!(
                form(&served.validated),
                form(&run.validated),
                "{}: profile {id:?}: the served form is not the second pass's",
                doc.name
            );
            profiles += 1;
        }
    }
    assert!(profiles > 20, "the corpus carries profiles: {profiles}");
}

/// **An `f64` profile log and an `Interval` one diff by their decisions
/// alone** (`editor_core::diff_verdicts`, whose verdict logs are
/// scalar-free): at the nominal, every corpus profile node that
/// evaluates at both scalars logs the same predicates the same number
/// of times with the same signs, though its `f64` lane skips the second
/// pass that `Interval` runs.
#[test]
fn an_f64_profile_log_and_an_interval_one_diff_to_no_flip() {
    let tol = Tol::witness();
    let opts = EvalOptions::default();
    let mut compared = 0usize;
    let mut parted = Vec::new();
    for doc in corpus::documents() {
        let at_f64 = evaluate::<f64>(&doc.doc, None, &CancelToken::new(), &opts, tol);
        let at_interval =
            evaluate::<geom_core::Interval>(&doc.doc, None, &CancelToken::new(), &opts, tol);
        let flips = editor_core::diff_verdicts(&at_f64, &at_interval);
        for (&id, result) in &at_f64.nodes {
            let profile = matches!(result, NodeResult::Ok(v) if matches!(v.payload, ValuePayload::Profile(_)));
            if !profile || !matches!(at_interval.nodes.get(&id), Some(NodeResult::Ok(_))) {
                continue;
            }
            compared += 1;
            if let Some(delta) = flips.nodes.get(&id) {
                parted.push(format!(
                    "{} {id:?}: flips {:?} diverged {:?}",
                    doc.name, delta.flips, delta.diverged
                ));
            }
        }
    }
    assert!(
        parted.is_empty(),
        "{} of {compared} profile nodes diff between f64 and Interval at the nominal:\n{}",
        parted.len(),
        parted.join("\n")
    );
    assert!(
        compared > 20,
        "the corpus's profiles evaluate at both: {compared}"
    );
}

/// **Under a degenerate offset box the `f64` lane runs the second
/// pass.** A box at `f64` whose axis is `Varying { c, c }` binds
/// `nominal + c`, which the first pass never read, so the lane is not
/// the first pass there (`editor_core::ProfileLane`): the plate's
/// profile under a box offsetting the hole radius is the second pass's
/// over that environment, bit for bit, and not the nominal's.
#[test]
fn under_a_degenerate_offset_box_the_f64_lane_runs_the_second_pass() {
    use editor_core::analysis::{BoxAxis, ParamBox};
    let tol = Tol::witness();
    let doc = plate();
    let id = profile_node_of(&doc);
    let hole_r = (doc.doc)
        .var_named(corpus::plate_param::HOLE_R)
        .expect("the plate declares the hole radius");
    let boxed = EvalOptions {
        param_box: Some(std::sync::Arc::new(ParamBox::from_axes(
            [(hole_r, BoxAxis::Varying { lo: 0.05, hi: 0.05 })].into(),
        ))),
        ..EvalOptions::default()
    };
    let form_of = |ev: &editor_core::Evaluation<f64>| match ev.value(id).map(|v| &v.payload) {
        Some(ValuePayload::Profile(p)) => form(&p.validated),
        other => panic!("the plate's profile evaluates: {other:?}"),
    };
    let nominal = form_of(&evaluate(
        &doc.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    ));
    let served = form_of(&evaluate(&doc.doc, None, &CancelToken::new(), &boxed, tol));
    let ran = form_of(&editor_core::eval::evaluate_running_pass_two(
        &doc.doc, &boxed, tol,
    ));
    assert_ne!(served.0, nominal.0, "the offset moves the profile");
    assert_eq!(served, ran, "the boxed profile is the second pass's");
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
        .resolve(&doc.doc.var_env::<f64>())
        .expect("resolves at f64");
    let mut records = Vec::new();
    for steps in &nominal {
        let (_, record) = profile::replay_recording(steps, Tol::witness()).expect("replays");
        records.push(record);
    }
    // Pass 2: the same program at Dual, seeded on the hole radius.
    let mut env = doc.doc.var_env::<Dual64>();
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
            .as_loop()
            .vertices()
            .iter()
            .any(|v| v.x.deriv != 0.0 || v.y.deriv != 0.0);
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
#[test]
fn a_wide_interval_binding_aborts_typed_rather_than_certifying() {
    use geom_core::Interval;
    /// The nominal f64 loops, replayed for the record's sake.
    fn nominal_loops(resolved: &[Vec<profile::Step<f64>>]) -> Vec<profile::ConstructedLoop<f64>> {
        resolved
            .iter()
            .map(|steps| profile::replay(steps, Tol::witness()).expect("the nominal replays"))
            .collect()
    }

    let doc = plate();
    let Some(Node::Profile(program)) = doc.doc.node(profile_node_of(&doc)) else {
        panic!("the plate's profile node is a profile node")
    };
    let nominal = program
        .resolve(&doc.doc.var_env::<f64>())
        .expect("resolves at f64");
    let mut records = Vec::new();
    for steps in &nominal {
        let (_, record) = profile::replay_recording(steps, Tol::witness()).expect("replays");
        records.push(record);
    }
    let mut env = doc.doc.var_env::<Interval>();
    // The hole radius BY NAME. Widening every continuous parameter
    // would make the row's own subject unclear: the claim is about a
    // box on the dimension that drives this profile, and a helper that
    // clobbers whatever else the document happens to carry would keep
    // passing if the plate grew a second parameter that did the
    // refusing instead.
    let hole_r = doc
        .doc
        .var_named(corpus::plate_param::HOLE_R)
        .expect("the plate declares its hole radius");
    let Some(ParamValue::Continuous { value, .. }) = env.bindings.get_mut(&hole_r) else {
        panic!("the plate's hole radius is a continuous parameter named hole_r")
    };
    // A box from a hair above zero to an order of magnitude up.
    *value = Interval::from_bounds(1e-4, 1.0);
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
    // The profile's own xy: a profile reads no plane, so this is the
    // plane the evaluator builds too.
    let plane = profile::SketchPlane::<f64>::xy();
    let (_, canonical) = profile::ConstructedProfile::new(plane, nominal_loops(&nominal))
        .validate_recording(Tol::witness())
        .expect("the nominal validates and records");
    let err = profile::ConstructedProfile::new(profile::SketchPlane::<Interval>::xy(), loops)
        .validate_guided(Tol::witness(), &canonical)
        .expect_err("a hole radius spanning four orders of magnitude cannot certify");
    // The FAMILY, not the fact that some string came back. This wall is
    // an ordinary validation predicate going indeterminate on a box too
    // wide to classify — `carrier_line_circle`, the hole's carrier
    // against the plate's sides, which is NOT a consumed structure
    // decision and so does not (and should not)
    // arrive in the `Structure` vocabulary. (It was
    // `arc_diameter_clearance` while that margin read 2r − |a − apex|;
    // read on the carrier, 2r·cos²(|Δθ|/4)/(1 + sin(|Δθ|/4)), the box's semicircles
    // are decided Positive at every radius in it, and the wall is the
    // next predicate the width reaches.) That distinction is the
    // whole point of asserting the family here: a reader who sees
    // "aborts typed" should be able to tell which of the two kinds of
    // typed abort this row is about.
    match &err {
        profile::ProfileError::Escalated { source, .. } => assert_eq!(
            source.predicate,
            Some("carrier_line_circle"),
            "the wide box is expected to stall the line/circle clearance"
        ),
        other => panic!(
            "expected an escalation from a validation predicate, got {other:?} — if this \
             became a `Structure` refusal, a consumed decision started covering this \
             wall and the claim text needs to move with it"
        ),
    }
    // Nothing was certified: no canonical form came back, so no caller
    // can mistake this binding for one that kept the nominal structure.
    println!("wide-box refusal: {err}");
}

/// The lift runs end to end at `Dual64` through the evaluation door,
/// and puts the value channel exactly where the `f64` lane does.
///
/// What this row does NOT show is a seeded tangent: an unseeded
/// document evaluation has none to carry, and the seed is
/// `EvalOptions::seed`'s door (`m10_4_seed`). The seam this unit opened
/// is that a seed PRESENT in the environment reaches the vertices, and
/// that is asserted one door down, at
/// [`a_dual_seed_on_a_profile_parameter_now_carries_a_tangent`], on the
/// exact call `wire_profile` makes.
///
/// What this row does show is that the whole lifted path — resolving
/// the program at `VarEnv<Dual64>`, elaborating it guided, feeding
/// both dual channels into the content key — runs at `Dual` and leaves
/// the value channel bit for bit at the `f64` lane's profile, with no
/// derivative anywhere.
#[test]
fn the_evaluation_door_runs_the_lift_at_dual() {
    use geom_core::Dual64;
    let doc = plate();
    let dual = evaluate::<Dual64>(
        &doc.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let at_f64 = evaluate::<f64>(
        &doc.doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    fn vertices<T: editor_core::EvalScalar>(ev: &editor_core::Evaluation<T>) -> Vec<(T, T)> {
        let mut out = Vec::new();
        for result in ev.nodes.values() {
            if let NodeResult::Ok(v) = result
                && let ValuePayload::Profile(p) = &v.payload
            {
                for lp in p.validated.loops() {
                    for vx in lp.vertices() {
                        out.push((vx.x, vx.y));
                    }
                }
            }
        }
        out
    }
    let lane: Vec<(Dual64, Dual64)> = vertices(&dual);
    let nominal: Vec<(f64, f64)> = vertices(&at_f64);
    assert!(!lane.is_empty(), "the plate has profile loops to compare");
    assert_eq!(
        lane.iter()
            .map(|(x, y)| (x.value.to_bits(), y.value.to_bits()))
            .collect::<Vec<_>>(),
        nominal
            .iter()
            .map(|(x, y)| (x.to_bits(), y.to_bits()))
            .collect::<Vec<_>>(),
        "the Dual lane's profile value channel is the f64 lane's"
    );
    assert!(
        lane.iter().all(|(x, y)| x.deriv == 0.0 && y.deriv == 0.0),
        "an unseeded environment has no derivative to carry"
    );
}

/// **A loft and a profile evaluate at `Interval`.** A loft reads its
/// sections' profile values, and at `Interval` it encloses the `f64`
/// skin of their value channel (`sweep::loft_profiles`). The loft
/// prism's sections are polygons on unrotated frames, so with no
/// parameter box their values are points and the loft builds. An arc
/// section or a rotated authored frame is not such a document: its
/// values are a few ulps wide, and the loft refuses
/// (`e6_drive_over_a_loft_at_interval`). The profile node's own payload
/// is elaborated at the lane (its vertex enclosures are the lane's
/// own).
#[test]
fn a_lofted_body_and_a_profile_evaluate_at_interval() {
    use geom_core::{Bounds, Interval};
    let run = |doc: &editor_core::ProfileDoc| {
        evaluate::<Interval>(
            doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
    let loft = corpus::loft_prism::document();
    let ev = run(&loft.doc);
    assert!(
        ev.nodes.values().any(|r| matches!(
            r,
            NodeResult::Ok(v) if matches!(v.payload, ValuePayload::Body(_))
        )),
        "the loft document builds a body at Interval"
    );

    // The extrude half: the profile node's own payload IS elaborated at
    // the lane. This evaluation carries no parameter box, so its
    // bindings are the document's nominals and the enclosure is
    // degenerate by construction. What is asserted is that the payload
    // is produced and well formed, not that it is wider; a widened
    // binding is `EvalOptions::param_box`'s door and is driven in
    // `m10_3_driver_interval`.
    let plate = plate();
    let g = run(&plate.doc);
    let mut profiles = 0usize;
    for result in g.nodes.values() {
        if let NodeResult::Ok(v) = result
            && let ValuePayload::Profile(p) = &v.payload
        {
            profiles += 1;
            for lp in p.validated.loops() {
                for vx in lp.vertices() {
                    assert!(
                        vx.x.lo() <= vx.x.hi() && vx.y.lo() <= vx.y.hi(),
                        "the lane-elaborated profile must carry well-formed enclosures"
                    );
                }
            }
        }
    }
    assert!(profiles > 0, "the plate evaluates a profile node");
}

fn plate() -> corpus::CorpusDoc {
    corpus::plate_param::document()
}

fn profile_node_of(doc: &corpus::CorpusDoc) -> editor_core::RecipeNodeId {
    *doc.doc
        .ids()
        .iter()
        .find(|id| matches!(doc.doc.node(**id), Some(Node::Profile(_))))
        .expect("the plate has a profile node")
}

/// The plate at `Sym<f64>` under `box_`, in a deferring session at the
/// door's budget: each node's content key, and the profile node's
/// verdict log.
fn at_sym(
    doc: &corpus::CorpusDoc,
    box_: Option<editor_core::analysis::ParamBox>,
) -> (Vec<Option<editor_core::ContentKey>>, Vec<String>) {
    let opts = EvalOptions {
        param_box: box_.map(std::sync::Arc::new),
        ..EvalOptions::default()
    };
    // Deferring, at the door's budget: the row reads which lift is taken
    // and what it logs, not what the tier discharges, and an eager
    // session over the guided plate is the memory the replay session
    // exists to avoid (`coincide::with_replay_session`).
    let budget = geom_core::SymBudget {
        max_terms: 256,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    geom_core::sym::deferred::with_deferred_session(
        budget,
        geom_core::SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let ev = evaluate::<geom_core::Sym<f64>>(
                &doc.doc,
                None,
                &CancelToken::new(),
                &opts,
                Tol::witness(),
            );
            let keys = ev
                .order
                .iter()
                .map(|id| ev.value(*id).map(|v| v.content_key))
                .collect();
            let log = ev
                .value(profile_node_of(doc))
                .expect("the profile evaluates")
                .verdicts
                .iter()
                .map(|v| format!("{v:?}"))
                .collect();
            (keys, log)
        },
    )
    .0
}

/// **A box that varies nothing is the build at `Sym` too.** A `Fixed`
/// axis HOLDS its variable at the nominal, which at `Sym` binds a
/// constant, not a parameter symbol (`analysis::var_env_over`), so every
/// node is the unboxed build's: same content key, same profile verdict
/// log.
#[test]
fn at_sym_a_fixed_box_is_the_build() {
    use editor_core::analysis::{BoxAxis, ParamBox};
    let doc = plate();
    let continuous: Vec<editor_core::VarId> = doc
        .doc
        .free_vars()
        .filter(|(_, p)| matches!(p, editor_core::FreeVar::Continuous { .. }))
        .map(|(id, _)| id)
        .collect();
    assert!(!continuous.is_empty(), "the plate has a variable to hold");
    let over =
        |axis: BoxAxis| ParamBox::from_axes(continuous.iter().map(|&id| (id, axis)).collect());
    let fixed = over(BoxAxis::Fixed);

    let build = at_sym(&doc, None);
    assert_eq!(
        at_sym(&doc, Some(fixed.clone())),
        build,
        "an all-Fixed box at Sym is the build, node for node"
    );
    // And it binds no symbol: every margin the held run decides re-values
    // to its own value with every parameter symbol moved, which a margin
    // over a symbol would not. Keys and logs read the same either way, so
    // this is the half that sees what a `Fixed` axis binds.
    let opts = EvalOptions {
        param_box: Some(std::sync::Arc::new(fixed)),
        ..EvalOptions::default()
    };
    let decisions = geom_core::sym::deferred::with_deferred_session(
        geom_core::SymBudget {
            max_terms: 256,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        },
        geom_core::SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let (_, decisions) = geom_core::sym::revalue::record(|| {
                let _ = evaluate::<geom_core::Sym<f64>>(
                    &doc.doc,
                    None,
                    &CancelToken::new(),
                    &opts,
                    Tol::witness(),
                );
            });
            let moved = decisions
                .iter()
                .filter(|d| {
                    let v0 = d.value.expect("Sym<f64> has a value");
                    geom_core::sym::revalue::revalue(d.node, &|_| Some(1.0e-3))
                        .is_some_and(|r| r.to_bits() != v0.to_bits())
                })
                .count();
            (decisions.len(), moved)
        },
    )
    .0;
    assert!(decisions.0 > 0, "the held run decides");
    assert_eq!(
        decisions.1, 0,
        "a margin of the held run moved with a parameter symbol: a Fixed axis bound one"
    );
}

/// Each node's content key and the first profile node's verdict log, for
/// `doc` at `T` over an EMPTY box or none, in a deferring session at the
/// door's budget (a no-op at a numeric scalar).
fn under_an_empty_box<T: editor_core::EvalScalar>(
    doc: &editor_core::ProfileDoc,
    boxed: bool,
) -> (Vec<Option<editor_core::ContentKey>>, Vec<String>) {
    let opts = EvalOptions {
        param_box: boxed.then(|| {
            std::sync::Arc::new(editor_core::analysis::ParamBox::from_axes(
                std::collections::BTreeMap::new(),
            ))
        }),
        ..EvalOptions::default()
    };
    geom_core::sym::deferred::with_deferred_session(
        geom_core::SymBudget {
            max_terms: 256,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        },
        geom_core::SymRules::shipped(),
        editor_core::drive::DEFAULT_SYM_RETRY,
        || {
            let ev = evaluate::<T>(doc, None, &CancelToken::new(), &opts, Tol::witness());
            let profile = *doc
                .ids()
                .iter()
                .find(|id| matches!(doc.node(**id), Some(Node::Profile(_))))
                .expect("a profile node");
            let keys = ev
                .order
                .iter()
                .map(|id| ev.value(*id).map(|v| v.content_key))
                .collect();
            let log = ev
                .value(profile)
                .expect("the profile evaluates")
                .verdicts
                .iter()
                .map(|v| format!("{v:?}"))
                .collect();
            (keys, log)
        },
    )
    .0
}

/// **An empty box is the build at a numeric scalar.** The two-hole
/// plate has toleranced variables; at `f64` and `Interval` an EMPTY box
/// binds every one at its nominal, and the evaluation is the build, key
/// for key and verdict for verdict.
#[test]
fn an_empty_box_is_the_build_at_a_numeric_scalar() {
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, Tol::witness()).0;
    assert!(
        doc.free_vars()
            .any(|(_, p)| editor_core::analysis::is_axis(p)),
        "the plate has a toleranced variable"
    );
    assert_eq!(
        under_an_empty_box::<f64>(&doc, true),
        under_an_empty_box::<f64>(&doc, false),
        "at f64 an empty box is the build"
    );
    assert_eq!(
        under_an_empty_box::<geom_core::Interval>(&doc, true),
        under_an_empty_box::<geom_core::Interval>(&doc, false),
        "at Interval an empty box is the build"
    );
}
