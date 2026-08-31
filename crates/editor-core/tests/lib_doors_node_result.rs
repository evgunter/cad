//! **LIB-DOORS F3: failed and poisoned nodes are reachable as typed
//! data from an `Evaluation`.**
//!
//! `Evaluation::value` deliberately collapses `Failed` and `Poisoned`
//! into `None`; before these accessors, NOTHING public distinguished
//! them — `NodeResult` had no `impl` block at all, and `nodes` is a
//! map a caller could only pattern-match by naming the enum. The
//! bindings' §L4 contract (typed exceptions carrying the real
//! `NodeError`) needs the distinction, so this suite pins it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    BooleanOp, CancelToken, Dimension, DocEdit, EvalOptions, Expr, LoopProgram, Node, NodeResult,
    ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, evaluate,
};
use geom_core::Tol;
use profile::SketchPlane;

/// A square profile `[0,s]²` on the xy-plane, as a loop program.
fn square(s: f64) -> Node<ProfileProgram> {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([lit(0.0), lit(0.0)]),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(0.0)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(s), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Point([lit(0.0), lit(s)])),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    })
}

/// Two boxes SHARING the z=0 plane (and the x=0 / y=0 side planes),
/// subtracted: the kernel never infers coincidence, so the Boolean
/// node FAILS — and a node downstream of it is POISONED. Returns the
/// document plus the failing and poisoned ids.
fn doc_with_failure() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let lit = |v: f64| Expr::literal(v, Dimension::Length).unwrap();
    let mut doc = ProfileDoc::empty_derived("lib_doors_node_result", Tol::witness());
    let insert = |doc: &mut ProfileDoc, node| {
        let applied = doc
            .apply(&DocEdit::InsertNode { node }, Tol::witness())
            .unwrap();
        *doc = applied.doc;
        applied.record.minted.unwrap()
    };
    let outer_profile = insert(&mut doc, square(2.0));
    let outer = insert(
        &mut doc,
        Node::Extrude {
            profile: outer_profile,
            distance: lit(2.0),
        },
    );
    let inner_profile = insert(&mut doc, square(1.0));
    let inner = insert(
        &mut doc,
        Node::Extrude {
            profile: inner_profile,
            distance: lit(1.0),
        },
    );
    let cut = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a: outer,
            b: inner,
            declare: None,
        },
    );
    let downstream = insert(
        &mut doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: cut,
            b: outer,
            declare: None,
        },
    );
    (doc, cut, downstream)
}

fn run(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

#[test]
fn a_failed_node_answers_its_typed_error() {
    let (doc, cut, _) = doc_with_failure();
    let ev = run(&doc);
    // The collapse is unchanged: `value` still answers `None`.
    assert!(ev.value(cut).is_none());
    // The new door distinguishes what `value` collapses.
    let result = ev.result(cut).expect("the node has an entry");
    assert!(result.value().is_none());
    assert!(result.poisoned_through().is_none());
    let error = result.error().expect("a failed node carries its error");
    assert_eq!(error.node, cut);
    // And the one-hop convenience agrees.
    let root = ev.node_error(cut).expect("node_error reaches it too");
    assert_eq!(root.node, cut);
}

#[test]
fn a_poisoned_node_answers_the_nearest_failed_ancestor() {
    let (doc, cut, downstream) = doc_with_failure();
    let ev = run(&doc);
    let result = ev.result(downstream).expect("the node has an entry");
    assert!(result.value().is_none());
    assert!(
        result.error().is_none(),
        "a poisoned node did not itself fail"
    );
    assert_eq!(result.poisoned_through(), Some(cut));
    // `node_error` walks the hop: the root cause is the ANCESTOR's.
    let root = ev.node_error(downstream).expect("root cause reachable");
    assert_eq!(root.node, cut);
}

#[test]
fn ok_and_absent_nodes_answer_none() {
    let (doc, _, _) = doc_with_failure();
    let ev = run(&doc);
    let ok_node = *ev.order.first().expect("the order is non-empty");
    assert!(ev.value(ok_node).is_some());
    assert!(matches!(ev.result(ok_node), Some(NodeResult::Ok(_))));
    assert!(ev.node_error(ok_node).is_none());
    let absent = RecipeNodeId(u64::MAX);
    assert!(ev.result(absent).is_none());
    assert!(ev.node_error(absent).is_none());
}

/// F6 (reopened on the PR #308 review): the refusal enums render as
/// PROSE — problem statements, not the payloads' `Debug` guts. Pins
/// one message per new `Display` (EditError, NodeError/-Kind,
/// DimensionError, ProgramRefusal), the no-guts property on the live
/// coincidence refusal, and the same property over every arm of
/// [`forwarding_cases`] — a forwarded payload carries whatever its own
/// `Display` renders, so the no-guts rule is only as wide as the set
/// of payloads something actually renders.
#[test]
fn refusals_render_as_prose_not_debug_guts() {
    use editor_core::{DimensionError, EditError};

    let edit = EditError::UnknownNode {
        id: RecipeNodeId(7),
    };
    assert_eq!(edit.to_string(), "edit: node 7 is not live");

    let literal = Expr::literal(f64::NAN, Dimension::Length).expect_err("NaN refuses");
    assert!(matches!(literal, DimensionError::NonFiniteLiteral));
    assert_eq!(literal.to_string(), "a literal value must be finite");

    // The fourth `Display` this suite claims to pin. Its validate arm
    // holds a `profile::ProfileError`, so it forwards rather than
    // re-stating — the same rule as `NodeErrorKind`'s payload arms.
    let program = editor_core::ProgramRefusal::Validate(profile::ProfileError::EmptyProfile);
    assert_eq!(
        program.to_string(),
        format!(
            "the replayed loops failed profile validation: {}",
            profile::ProfileError::EmptyProfile
        )
    );

    // The live failure: the coincident Boolean's message states the
    // problem and the two-armed recourse (since R3 the refusal is the
    // typed menu variant); the enum's structure (variant names,
    // braces) stays OUT of the prose.
    let (doc, cut, _) = doc_with_failure();
    let ev = run(&doc);
    let error = ev.node_error(cut).expect("the Boolean failed");
    let message = error.to_string();
    assert!(
        message.starts_with(&format!("node {} failed: ", cut.0)),
        "{message}"
    );
    assert!(
        message.contains("Boolean refused an undeclared contact"),
        "{message}"
    );
    assert!(message.contains("declare that finding"), "{message}");
    for guts in [
        "UndeclaredCoincidence",
        "UndeclaredContact",
        "FlushFinding",
        "{",
        "Indeterminate",
    ] {
        assert!(!message.contains(guts), "Debug guts leaked: {message}");
    }

    // Every forwarded payload, to the depth the workspace nests them.
    // A `{` here means some layer below rendered a struct with `{:?}`
    // and the forwarding carried it out through the FFI.
    for kind in forwarding_cases() {
        let rendered = kind.to_string();
        for guts in ["{", "MarginDiag", "Value("] {
            assert!(
                !rendered.contains(guts),
                "Debug guts leaked through a forwarded payload: {rendered}"
            );
        }
    }

    // The metadata version door: three payload arms, one of which used
    // to be reported as another. Each must say its own thing, and the
    // message must not read as the "v" field being absent when it is
    // the map that is. `EditError` renders IDENTIFIERS via `Debug`
    // deliberately (its own `Display` header states why: an identifier
    // IS the location), so the property asserted here is the payload
    // one — the variant's name never reaches the prose.
    for (error, expected) in [
        (editor_core::MetaVersionError::NotAMap, "is not a map"),
        (
            editor_core::MetaVersionError::MissingVersion,
            "no \"v\" entry",
        ),
        (
            editor_core::MetaVersionError::VersionNotInt,
            "not an integer",
        ),
    ] {
        let message = EditError::MetaUnversioned {
            name: editor_core::StableName {
                kind: editor_core::EntityKind::Body,
                node: RecipeNodeId(1),
                path: vec![editor_core::RoleSeg::OutputBody],
            },
            key: "provenance".to_string(),
            error,
        }
        .to_string();
        assert!(message.contains(expected), "{error:?} rendered: {message}");
        for guts in ["NotAMap", "MissingVersion", "VersionNotInt"] {
            assert!(!message.contains(guts), "Debug guts leaked: {message}");
        }
    }
}

/// The forwarding roster: one `NodeErrorKind` arm per payload-owning
/// crate, plus the deepest nesting the workspace actually builds
/// (`Split` → `SplitFinishError` → `BandError`, three layers of
/// forwarding under one node arm). Two properties are asserted over
/// it — that each arm forwards its payload, and that none of them
/// leaks `Debug` structure — because both are properties of the same
/// roster and a fixture that holds one should hold the other.
fn forwarding_cases() -> Vec<editor_core::NodeErrorKind> {
    use editor_core::NodeErrorKind as K;
    let name = |kind| editor_core::StableName {
        kind,
        node: RecipeNodeId(3),
        path: vec![editor_core::RoleSeg::OutputBody],
    };
    vec![
        K::Profile(profile::ProfileError::EmptyProfile),
        K::Expr {
            slot: editor_core::SlotId::Distance,
            source: editor_core::EvalError::NonFiniteResult,
        },
        K::DeclareResolve {
            error: Box::new(editor_core::ResolveError::NodeGone {
                name: name(editor_core::EntityKind::Face),
                edit: editor_core::RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(3),
                },
            }),
        },
        K::BlendSelectionResolve {
            verb: sweep::blend::BlendKind::Fillet,
            error: Box::new(editor_core::ResolveError::Ambiguous {
                name: name(editor_core::EntityKind::Edge),
                candidates: vec![],
                tie: editor_core::TieWitness {
                    node: RecipeNodeId(3),
                    at: name(editor_core::EntityKind::Edge),
                    width: 2,
                },
            }),
        },
        K::WitnessBifurcation(editor_core::WitnessBifurcation {
            kind: editor_core::BifurcationKind::FoldProximity,
            margin: editor_core::BranchMarginEvidence {
                margin: 2e-10,
                band_zero: 1e-9,
                band_escalate: 1e-8,
            },
            implicated: vec![editor_core::Implicated::Constraint(1)],
            witness_age: editor_core::WitnessAge {
                solved_under: vec![],
                at_solve: vec![],
            },
        }),
        K::PlacementRule(editor_core::PlacementRuleFault::NonFiniteFrame { index: 3 }),
        K::Extrude(sweep::ExtrudeError::ObliqueExtrusion),
        K::Revolve(sweep::RevolveError::DegenerateAxis),
        K::Skin(sweep::SkinError::TooFewSections { have: 1, need: 2 }),
        // The seam arm carries the iso-extraction refusal itself. A
        // control-count mismatch is the invariant
        // `geom_brep::boundary_iso_u` can break, and carrying it is
        // what lets a kernel-bug report name WHICH structure the
        // corrupt wall broke rather than only that one did.
        K::Loft(sweep::LoftError::SeamStructure {
            source: geom_core::spline::SplineError::ControlCountMismatch {
                control: 3,
                expected: 4,
            },
        }),
        K::Blend {
            verb: sweep::blend::BlendKind::Chamfer,
            error: sweep::blend::BlendError::RepeatedEdge {
                edge: topo::EdgeKey::default(),
            },
        },
        K::Transform(topo::transform::TransformError::NurbsPlaceholder),
        K::Split(topo::SplitError::Finish(topo::SplitFinishError::Band(
            geom_core::BandError::Empty {
                zero: 1.0,
                escalate: 0.5,
            },
        ))),
    ]
}

/// **A `NodeErrorKind` arm that holds a kernel refusal RENDERS it.**
/// The variant carries the typed error for a caller who can match; the
/// message is the whole channel for one who cannot, and the bindings'
/// `kind` attribute is the discriminant alone — so an arm that names
/// the op and stops has spent the payload's class, keys and recourse
/// on nothing.
///
/// **Representative, not exhaustive**, and nothing makes it
/// exhaustive: `NodeErrorKind` cannot be enumerated at runtime and a
/// hand-kept roster of arms is the very shape this repo keeps
/// retiring. [`forwarding_cases`] is the roster, and a new arm wrapping a new
/// kernel error is not covered by it; the module comment on the
/// `Display` impl is what states the rule for such an arm.
#[test]
fn a_kernel_payload_arm_forwards_the_payloads_own_message() {
    use editor_core::NodeErrorKind as K;

    for kind in forwarding_cases() {
        let rendered = kind.to_string();
        let payload = match &kind {
            K::Profile(e) => e.to_string(),
            K::Expr { source, .. } => source.to_string(),
            K::DeclareResolve { error } => error.to_string(),
            K::BlendSelectionResolve { error, .. } => error.to_string(),
            K::WitnessBifurcation(e) => e.to_string(),
            K::PlacementRule(e) => e.to_string(),
            K::Extrude(e) => e.to_string(),
            K::Revolve(e) => e.to_string(),
            K::Skin(e) => e.to_string(),
            K::Loft(e) => e.to_string(),
            K::Blend { error, .. } => error.to_string(),
            K::Transform(e) => e.to_string(),
            K::Split(e) => e.to_string(),
            other => panic!("add the new case's payload here: {other:?}"),
        };
        assert!(
            rendered.ends_with(&payload),
            "the arm dropped its payload: rendered {rendered:?}, payload {payload:?}"
        );
        assert!(
            rendered.len() > payload.len(),
            "the arm must still name the failing op: {rendered:?}"
        );
    }
}

/// **A payload's own nested SOURCE reaches the message — asserted
/// against an oracle none of the wrappers can move.**
///
/// The roster row above cannot see this and it is worth saying why,
/// because the shape is easy to rebuild by accident. That test derives
/// its expectation with `K::Loft(e) => e.to_string()` — the very
/// `Display` under test — so an arm that quietly stopped rendering its
/// `source` would shorten BOTH sides equally and `ends_with` would
/// still hold. It pins forwarding at ONE layer and is vacuous about
/// every layer below.
///
/// So each case here builds the INNERMOST payload's message directly
/// from its own type, never through the enum that wraps it, and asks
/// the fully rendered node error to contain it. Dropping the `source`
/// interpolation from any arm below reddens this immediately.
///
/// The three cases are the workspace's source-carrying nestings:
/// `LoftError`'s two (`SeamStructure`'s `SplineError` — the
/// `boundary_iso_u` refusal whose own `# Errors` section promises it is
/// *surfaced rather than swallowed* — and `StackingEscalated`'s
/// `Indeterminate`), plus the three-layer `Split` chain the roster
/// already carries and was equally vacuous about.
#[test]
fn a_nested_source_under_a_payload_arm_survives_into_the_message() {
    use editor_core::NodeErrorKind as K;

    // Built from their own types. Nothing below is reached through
    // `LoftError`, `SplitError` or `NodeErrorKind`.
    let spline = geom_core::spline::SplineError::ControlCountMismatch {
        control: 3,
        expected: 4,
    };
    let escalation = geom_core::Indeterminate {
        margin: geom_core::MarginDiag::Value(2e-10),
        band: geom_core::Band::linear(geom_core::Tol::witness()).expect("a witness band forms"),
        predicate: Some("loft_stacking"),
    };
    let band = geom_core::BandError::Empty {
        zero: 1.0,
        escalate: 0.5,
    };

    let cases: Vec<(K, String)> = vec![
        (
            K::Loft(sweep::LoftError::SeamStructure {
                source: spline.clone(),
            }),
            spline.to_string(),
        ),
        (
            K::Loft(sweep::LoftError::StackingEscalated { source: escalation }),
            escalation.to_string(),
        ),
        (
            K::Split(topo::SplitError::Finish(topo::SplitFinishError::Band(band))),
            band.to_string(),
        ),
    ];

    for (kind, inner) in cases {
        // ANTI-VACUITY: a payload whose own `Display` renders nothing
        // would satisfy `contains` for free.
        assert!(
            inner.len() > 20,
            "the oracle must be a real message: {inner:?}"
        );
        let rendered = kind.to_string();
        assert!(
            rendered.contains(&inner),
            "the nested source did not reach the message: rendered {rendered:?}, \
             source {inner:?}"
        );
        assert!(
            rendered.len() > inner.len(),
            "the wrappers must still name what failed: {rendered:?}"
        );
    }
}

/// **The document layer's own payload types render their own story**
/// (the D54 set: `EvalError`, `ResolveError`, `WitnessBifurcation`,
/// `PlacementRuleFault`) — each message states the problem in prose,
/// carries its one recourse where the fault has one, and none of them
/// leaks `Debug` structure. One case per `Display`, plus the arms
/// whose recourse phrasing is load-bearing.
#[test]
fn the_document_layers_own_payloads_render_their_own_stories() {
    use editor_core::{
        BifurcationKind, BranchMarginEvidence, Diagnosis, EntityKind, EvalError, ParamName,
        PlacementRuleFault, RecipeEditRef, ResolveError, RoleSeg, StableName, WitnessAge,
        WitnessBifurcation,
    };

    let name = |kind| StableName {
        kind,
        node: RecipeNodeId(5),
        path: vec![RoleSeg::OutputBody],
    };
    let cases: Vec<(String, &[&str])> = vec![
        (
            EvalError::UnknownParam(ParamName::new("width")).to_string(),
            &[
                "\"width\"",
                "has no binding",
                "declare the document parameter",
            ],
        ),
        (
            EvalError::NonFiniteResult.to_string(),
            &["not finite", "pole"],
        ),
        (
            ResolveError::Vanished {
                name: name(EntityKind::Face),
                diagnosis: Diagnosis::PredicateFlip {
                    predicate: "coincidence",
                    from: geom_core::Sign::Zero,
                    to: geom_core::Sign::Positive,
                },
                last_good: None,
            }
            .to_string(),
            &[
                "face name minted by node 5",
                "no longer resolves",
                "predicate coincidence flipped",
            ],
        ),
        (
            ResolveError::NodeGone {
                name: name(EntityKind::Vertex),
                edit: RecipeEditRef::NodeDeleted {
                    node: RecipeNodeId(5),
                },
            }
            .to_string(),
            &["vertex name", "node 5 was deleted", "explicit rebind"],
        ),
        (
            WitnessBifurcation {
                kind: BifurcationKind::AmbiguousBasin,
                margin: BranchMarginEvidence {
                    margin: 2e-10,
                    band_zero: 1e-9,
                    band_escalate: 1e-8,
                },
                implicated: vec![],
                witness_age: WitnessAge {
                    solved_under: vec![],
                    at_solve: vec![],
                },
            }
            .to_string(),
            &[
                "well-separated solution basins",
                "margin 2e-10",
                "re-solve to record a fresh witness",
            ],
        ),
        (
            PlacementRuleFault::CountSpelling.to_string(),
            &["disagree about how many placements"],
        ),
        (
            PlacementRuleFault::ImproperFrame {
                index: 2,
                determinant: -1.0,
            }
            .to_string(),
            &["placement 2 is improper (mirroring)"],
        ),
    ];
    for (rendered, expected) in cases {
        for needle in expected {
            assert!(rendered.contains(needle), "{needle:?} not in: {rendered}");
        }
        // The negative pin class: prose, never a Debug dump — no
        // struct braces, no variant name.
        for guts in [
            "{",
            "UnknownParam",
            "NodeGone",
            "PredicateFlip",
            "AmbiguousBasin",
        ] {
            assert!(!rendered.contains(guts), "Debug guts leaked: {rendered}");
        }
    }
    // The W3 layer-2 vocabulary ban, pinned on the rendering: the
    // refusal is a certificate outcome, never a DOF slogan.
    let refused = WitnessBifurcation {
        kind: BifurcationKind::ResidualFailure,
        margin: BranchMarginEvidence {
            margin: 0.0,
            band_zero: 1e-9,
            band_escalate: 1e-8,
        },
        implicated: vec![],
        witness_age: WitnessAge {
            solved_under: vec![],
            at_solve: vec![],
        },
    }
    .to_string();
    for banned in ["over-constrained", "under-constrained", "converge"] {
        assert!(!refused.contains(banned), "W3 banned phrase: {refused}");
    }
}
