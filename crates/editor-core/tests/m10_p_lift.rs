//! **The profile-parameter lift at the evaluation seam.**
//!
//! `plate_param` is the fixture throughout: the corpus's one document
//! whose profile geometry is an EXPRESSION, so a document parameter
//! drives two loops at once. Before the lift, a `Dual` seed on that
//! parameter propagated no tangent and an interval binding widened
//! nothing — profile geometry reached every lane as f64 bits. These
//! rows are what changed and what did not.
//!
//! - **Off is off.** The build is `Pinned` (the lift is read off what
//!   the evaluation binds, `EvalOptions::profile_lift_over`) and the whole build
//!   path runs there; `m10_p_fence.rs` digests that against the
//!   pre-lift tree. Here the narrower claim: the guided lift at `f64`,
//!   asked for by a box varying every continuous variable over `[0, 0]`
//!   (`corpus::options_under`), changes nothing, because guided
//!   elaboration at `f64` is plain elaboration at `f64`.
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

use editor_core::{
    CancelToken, EvalOptions, Node, NodeResult, ParamValue, ProfileLift, ValuePayload, evaluate,
};
use geom_core::Tol;

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

fn options(doc: &editor_core::ProfileDoc, lift: ProfileLift) -> EvalOptions {
    corpus::options_under(doc, lift).expect("the document has a continuous variable")
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
    let mut compared = 0usize;
    for doc in corpus::documents() {
        // A document with no continuous variable has no analysis to ask
        // for at its nominal: its every evaluation is the build.
        if corpus::options_under(&doc.doc, ProfileLift::Guided).is_none() {
            continue;
        }
        compared += 1;
        let run = |lift| {
            evaluate::<f64>(
                &doc.doc,
                None,
                &CancelToken::new(),
                &options(&doc.doc, lift),
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
    assert!(
        compared >= 10,
        "only {compared} corpus documents have a continuous variable to ask the guided lift with"
    );
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
            &options(&doc.doc, lift),
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
    // All three of these are used ONLY by this row, so they are
    // imported here rather than at module scope.

    use geom_core::{Interval, Real};
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
    // The sketch plane at the lane scalar lifts as constants: VQ8 keeps
    // the plane out of the parameter layer.
    let err = profile::ConstructedProfile::new(plane.map(Interval::from_f64), loops)
        .validate_guided(Tol::witness(), &canonical)
        .expect_err("a hole radius spanning four orders of magnitude cannot certify");
    // The FAMILY, not the fact that some string came back. This wall is
    // an ordinary validation predicate going indeterminate on a box too
    // wide to classify — `carrier_line_circle`, the hole's carrier
    // against the plate's sides, which is NOT a consumed structure
    // decision and so does not (and should not)
    // arrive in the `Structure` vocabulary. (It was
    // `arc_diameter_clearance` while that margin read 2r − |a − apex|;
    // read on the carrier, 2r·(1 − sin(|Δθ|/4)), the box's semicircles
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
            &options(&doc.doc, lift),
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

/// The lift runs end to end at `Dual64` through the evaluation door,
/// and does not move the value channel.
///
/// What this row does NOT show is a seeded tangent, and the reason is
/// worth stating rather than leaving as an absence: `Doc::var_env`
/// embeds every parameter through `from_f64`, so a document evaluation
/// has no seed to carry — putting one there is the seeding surface,
/// which is another unit's. The seam this unit opened is that a seed
/// PRESENT in the environment now reaches the vertices, and that is
/// asserted one door down, at
/// [`a_dual_seed_on_a_profile_parameter_now_carries_a_tangent`], on the
/// exact call `wire_profile` makes.
///
/// What this row does show is that the whole lifted path — resolving
/// the program at `VarEnv<Dual64>`, elaborating it guided, feeding
/// both dual channels into the content key — runs at `Dual` and leaves
/// the value channel exactly where the pinned lane put it.
#[test]
fn the_evaluation_door_runs_the_lift_at_dual() {
    use geom_core::Dual64;
    let doc = plate();
    let run = |lift| {
        evaluate::<Dual64>(
            &doc.doc,
            None,
            &CancelToken::new(),
            &options(&doc.doc, lift),
            Tol::witness(),
        )
    };
    let profiles = |ev: &editor_core::Evaluation<Dual64>| {
        let mut out = Vec::new();
        for result in ev.nodes.values() {
            if let NodeResult::Ok(v) = result
                && let ValuePayload::Profile(p) = &v.payload
            {
                for lp in p.validated.loops() {
                    for vx in lp.vertices() {
                        out.push((
                            vx.x.value.to_bits(),
                            vx.y.value.to_bits(),
                            vx.x.deriv,
                            vx.y.deriv,
                        ));
                    }
                }
            }
        }
        out
    };
    let (pinned, guided) = (run(ProfileLift::Pinned), run(ProfileLift::Guided));
    let (a, b) = (profiles(&pinned), profiles(&guided));
    assert!(!a.is_empty(), "the plate has profile loops to compare");
    assert_eq!(
        a, b,
        "the lift moved the Dual lane's profile payload — value channel or \
         tangent — and with no seed in the environment it may move neither"
    );
    assert!(
        a.iter().all(|(_, _, dx, dy)| *dx == 0.0 && *dy == 0.0),
        "an unseeded environment has no derivative to carry, at either lane"
    );
}

/// **A lofted body at `Interval` is the same under both lifts.** A loft
/// reads its sections' profile values, and at `Interval` it encloses
/// the `f64` skin of their value channel (`sweep::loft_profiles`). The
/// loft prism's sections are polygons on unrotated frames, so with no
/// parameter box the two lifts' profile values hold the same points,
/// and the lofted body is bit for bit one body. An arc section or a
/// rotated authored frame is not such a document: under the guided lift
/// its values are a few ulps wide, and the loft refuses. The profile node's own
/// payload under the guided lift is elaborated at the lane (its vertex
/// enclosures are the lane's own).
#[test]
fn a_lofted_body_at_interval_is_one_body_under_both_lifts() {
    use geom_core::Interval;
    let run = |doc: &editor_core::ProfileDoc, lift| {
        evaluate::<Interval>(
            doc,
            None,
            &CancelToken::new(),
            &options(doc, lift),
            Tol::witness(),
        )
    };
    let body_bits = |ev: &editor_core::Evaluation<Interval>| {
        use geom_core::Bounds;
        let mut out = Vec::new();
        for result in ev.nodes.values() {
            if let NodeResult::Ok(v) = result
                && let ValuePayload::Body(b) = &v.payload
            {
                for (_, p) in b.points() {
                    for c in p.to_array() {
                        out.push((c.lo().to_bits(), c.hi().to_bits()));
                    }
                }
            }
        }
        out
    };

    // The loft half: both lifts' sections have one value channel here,
    // and the loft encloses its `f64` skin.
    let loft = corpus::loft_prism::document();
    let (pinned, guided) = (
        run(&loft.doc, ProfileLift::Pinned),
        run(&loft.doc, ProfileLift::Guided),
    );
    assert!(
        !body_bits(&pinned).is_empty(),
        "the loft document must build a body for this row to mean anything"
    );
    assert_eq!(
        body_bits(&pinned),
        body_bits(&guided),
        "the lift moved a lofted body at Interval, though both lifts' sections hold the \
         same points"
    );

    // The extrude half: the profile node's own payload IS elaborated at
    // the lane, which is the other side of the asymmetry. Both lanes
    // enclose the same point here — this evaluation carries no
    // parameter box, so its bindings are the document's nominals and
    // the enclosure is degenerate by construction. What is asserted is
    // that the payload is produced and encloses the nominal, not that
    // it is wider; a widened binding is `EvalOptions::param_box`'s
    // door and is driven in `m10_3_driver_interval`.
    let plate = plate();
    let g = run(&plate.doc, ProfileLift::Guided);
    let mut profiles = 0usize;
    for result in g.nodes.values() {
        if let NodeResult::Ok(v) = result
            && let ValuePayload::Profile(p) = &v.payload
        {
            use geom_core::Bounds;
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
/// constant, not a parameter symbol (`analysis::var_env_over`), so the
/// lift is pinned and every node is the unboxed build's: same content
/// key, same profile verdict log. And `Varying { lo: 0, hi: 0 }` — the
/// way to ask for the nominal as a symbol — is an analysis: guided, its
/// profile log the guided op's own (the pinned log, then the op's replay
/// and validation at the lane).
#[test]
fn at_sym_a_fixed_box_is_the_build_and_a_degenerate_varying_one_is_guided() {
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
    let varying = over(BoxAxis::Varying { lo: 0.0, hi: 0.0 });
    let lift = |b: &ParamBox| {
        EvalOptions {
            param_box: Some(std::sync::Arc::new(b.clone())),
            ..EvalOptions::default()
        }
        .profile_lift()
    };
    assert_eq!(lift(&fixed), ProfileLift::Pinned);
    assert_eq!(lift(&varying), ProfileLift::Guided);

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
    let (_, guided_log) = at_sym(&doc, Some(varying));
    assert!(
        guided_log.len() > build.1.len() && guided_log[..build.1.len()] == build.1[..],
        "the guided log is the build's log, then the op's own: build {} decisions, guided {}",
        build.1.len(),
        guided_log.len()
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

/// **The lift follows what the environment binds.** The two-hole plate
/// has toleranced variables. Under an EMPTY box at `Sym` each binds as
/// its id's symbol (VR8: a toleranced variable is an axis and a symbol),
/// so the environment is not the build's and the lift is guided though
/// the box varies nothing — its profile log the build's, then the op's
/// own. At `f64` and `Interval` the same empty box binds every variable
/// at its nominal, and the evaluation is the build, key for key and
/// verdict for verdict.
#[test]
fn an_empty_box_is_the_build_at_a_numeric_scalar_and_guided_at_sym_over_a_toleranced_variable() {
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
    let (_, build) = under_an_empty_box::<geom_core::Sym<f64>>(&doc, false);
    let (_, boxed) = under_an_empty_box::<geom_core::Sym<f64>>(&doc, true);
    assert!(
        boxed.len() > build.len() && boxed[..build.len()] == build[..],
        "at Sym the empty box binds symbols, so the lift is guided: build {} decisions, boxed {}",
        build.len(),
        boxed.len()
    );
}
