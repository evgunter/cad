//! Tests for the Python-independent half of the crate.
//!
//! These run on the DEFAULT build path — no `python` feature, no
//! interpreter — which is the point: hosted CI executes them without a
//! Python toolchain present.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use crate::errors::{
    ErrorClass, QuantityOpMismatch, canonical_unit, dimension_tag, reads_as_prose,
};
use crate::tags::{
    expr_dimension_error_tag, normalization_kind_tag, path_error_tag, persist_error_tag,
    promoted_curve_kind_tag, promoted_kind_tag, step_import_error_tag, workspace_error_tag,
};
use pncad::document::Dimension;
use pncad::tolerance::Tol;
use pncad::topo::{FaceKey, VertexKey};
use std::collections::BTreeMap;
use std::path::Path;
// The shared Rust-source lexer: `src/tags.rs` is READ by the tag-table
// guard below, and this is the tree's one answer to "is this text code,
// prose or a literal". `crates/test-utils/tests/reader_census.rs`
// carries the line that says so.
use test_utils::source::{balanced_end, code_and_literals, code_only};

#[test]
fn dimension_tags_are_stable() {
    assert_eq!(dimension_tag(Dimension::Length), "length");
    assert_eq!(dimension_tag(Dimension::Angle), "angle");
    assert_eq!(dimension_tag(Dimension::Count), "count");
    assert_eq!(dimension_tag(Dimension::Scalar), "scalar");
}

/// The FFI tag and the kernel's prose word are two spellings of one
/// closed list. This crate owns the tag, so they are free to differ —
/// but they do not, and a silent divergence between a refusal a user
/// reads and the tag they branch on is worth a test rather than a
/// convention.
#[test]
fn dimension_tags_match_the_kernel_prose() {
    for dim in [
        Dimension::Length,
        Dimension::Angle,
        Dimension::Count,
        Dimension::Scalar,
    ] {
        assert_eq!(
            dimension_tag(dim),
            dim.to_string(),
            "the FFI tag and the kernel's prose word have drifted apart"
        );
    }
}

#[test]
fn canonical_units_match_the_gq5_ratification() {
    // GQ5 / §L4: canonical metres and radians underneath.
    assert_eq!(canonical_unit(Dimension::Length), Some("m"));
    assert_eq!(canonical_unit(Dimension::Angle), Some("rad"));
    assert_eq!(canonical_unit(Dimension::Count), None);
    assert_eq!(canonical_unit(Dimension::Scalar), None);
}

#[test]
fn a_quantity_operator_mismatch_carries_structure_not_prose() {
    let err = QuantityOpMismatch::new("+", Dimension::Length, Dimension::Angle);
    assert_eq!(err.op, "+");
    assert_eq!(err.left, Dimension::Length);
    assert_eq!(err.right, Dimension::Angle);
    // The message exists for humans, but the fields above are the
    // contract (§L4: never strings).
    assert_eq!(err.to_string(), "cannot apply `+` to length and angle");
}

/// Every class name is pinned, and the pin cannot go stale: the
/// expected spelling comes from a SECOND exhaustive match, so a new
/// [`ErrorClass`] variant stops this test compiling rather than
/// slipping past a list someone forgot to extend.
#[test]
fn error_classes_name_the_python_hierarchy() {
    fn expected(class: ErrorClass) -> &'static str {
        match class {
            ErrorClass::Edit => "EditError",
            ErrorClass::Evaluation => "EvaluationError",
            ErrorClass::Validation => "ValidationError",
            ErrorClass::Dimension => "DimensionError",
            ErrorClass::FmtQuantity => "FmtQuantityError",
            ErrorClass::Literal => "LiteralError",
            ErrorClass::Parse => "ParseError",
            ErrorClass::Eval => "EvalError",
            ErrorClass::Persist => "PersistError",
            ErrorClass::Export => "ExportError",
            ErrorClass::Tessellate => "TessellateError",
            ErrorClass::StlExport => "StlError",
            ErrorClass::StepImport => "StepImportError",
            ErrorClass::Path => "PathError",
            ErrorClass::Select => "SelectRefusal",
            ErrorClass::Frame => "FrameError",
            ErrorClass::Identity => "IdentityError",
            ErrorClass::Workspace => "WorkspaceError",
            ErrorClass::Mate => "MateError",
            ErrorClass::Assembly => "AssemblyError",
            ErrorClass::Product => "ProductError",
            ErrorClass::Split => "SplitError",
            ErrorClass::Inline => "InlineError",
            ErrorClass::Update => "UpdateError",
            ErrorClass::Readback => "ReadbackError",
            ErrorClass::HitTest => "HitTestError",
            ErrorClass::NodePick => "NodePickError",
            ErrorClass::Checks => "ChecksError",
            ErrorClass::Enforce => "CheckRefusal",
            ErrorClass::Distribution => "DistributionFault",
            ErrorClass::Measure => "MeasureUnavailable",
            ErrorClass::MeasureNode => "MeasureNodeFault",
            ErrorClass::MeasureUnavailableAt => "MeasureUnavailableAt",
            ErrorClass::AnalysisPolicy => "AnalysisPolicyError",
            ErrorClass::Mc => "McRefusal",
        }
    }
    for class in [
        ErrorClass::Edit,
        ErrorClass::Evaluation,
        ErrorClass::Validation,
        ErrorClass::Dimension,
        ErrorClass::FmtQuantity,
        ErrorClass::Literal,
        ErrorClass::Parse,
        ErrorClass::Eval,
        ErrorClass::Persist,
        ErrorClass::Export,
        ErrorClass::Tessellate,
        ErrorClass::StlExport,
        ErrorClass::StepImport,
        ErrorClass::Path,
        ErrorClass::Select,
        ErrorClass::Frame,
        ErrorClass::Identity,
        ErrorClass::Workspace,
        ErrorClass::Mate,
        ErrorClass::Assembly,
        ErrorClass::Product,
        ErrorClass::Split,
        ErrorClass::Inline,
        ErrorClass::Update,
        ErrorClass::Readback,
        ErrorClass::HitTest,
        ErrorClass::NodePick,
        ErrorClass::Checks,
        ErrorClass::Enforce,
        ErrorClass::Distribution,
        ErrorClass::Measure,
        ErrorClass::MeasureNode,
        ErrorClass::MeasureUnavailableAt,
        ErrorClass::AnalysisPolicy,
        ErrorClass::Mc,
    ] {
        assert_eq!(class.class_name(), expected(class));
    }
}

/// LIB-B-DISTRIBUTIONS: the four form words and the three faults,
/// CONSTRUCTED rather than listed.
///
/// The tag inventory pins the words `src/tags.rs` can emit; it cannot
/// say which kernel value emits which. These rows do, from real
/// `Distribution` values built through the kernel's own doors — so a
/// form silently renamed onto another word reds here rather than at a
/// Python caller.
///
/// The faults come out of `Distribution::check` rather than being
/// written by hand, which is the point one rung further: the binding
/// raises what the check answers, and this pins that the check
/// answers what the tags claim it does. `check` is also the reason the
/// Python constructor needs no rule of its own.
#[test]
fn distribution_form_and_fault_tags_are_stable() {
    use crate::tags::{distribution_fault_tag, distribution_field_tag, distribution_kind_tag};
    use pncad::document::{Distribution as D, DistributionField};

    let band = D::Band { lo: -1.0, hi: 1.0 };
    let uniform = D::Uniform { lo: -1.0, hi: 1.0 };
    let normal = D::Normal { sigma: 1.0 };
    let window = D::TruncatedNormal {
        sigma: 1.0,
        lo: -1.0,
        hi: 1.0,
    };
    assert_eq!(distribution_kind_tag(&band), "band");
    assert_eq!(distribution_kind_tag(&uniform), "uniform");
    assert_eq!(distribution_kind_tag(&normal), "normal");
    assert_eq!(distribution_kind_tag(&window), "truncated_normal");
    // Every one of those four is an inhabitant: a form word is only
    // reachable from Python if the constructor that mints it passes.
    for form in [band, uniform, normal, window] {
        assert_eq!(form.check(), Ok(()), "{form:?}");
    }

    let fault = |d: D| {
        d.check()
            .expect_err("this distribution breaks an E2 invariant")
    };
    assert_eq!(
        distribution_fault_tag(&fault(D::Normal { sigma: 0.0 })),
        "sigma_not_positive"
    );
    assert_eq!(
        distribution_fault_tag(&fault(D::Band { lo: 1.0, hi: 2.0 })),
        "nominal_outside_support"
    );
    assert_eq!(
        distribution_fault_tag(&fault(D::Normal {
            sigma: f64::INFINITY
        })),
        "non_finite"
    );
    assert_eq!(distribution_field_tag(&DistributionField::Sigma), "sigma");
    assert_eq!(distribution_field_tag(&DistributionField::Lo), "lo");
    assert_eq!(distribution_field_tag(&DistributionField::Hi), "hi");
}

/// LIB-B-DISTRIBUTIONS: the analysis lane's two refusals, from the
/// doors that answer them.
///
/// Both are constructed by CALLING the door rather than by naming the
/// variant, so the rows say the band really does refuse a
/// shape-dependent price and the policy really does refuse a mass
/// outside `(0, 1)` — which is what the Python classes are for.
#[test]
fn analysis_refusal_tags_are_stable() {
    use crate::tags::{analysis_policy_error_tag, measure_unavailable_tag};
    use pncad::analysis::{AnalysisPolicy, box_mass};
    use pncad::document::{Distribution, ParamName};

    let bore = ParamName::new("bore");
    let refusal = box_mass(
        &bore,
        &Distribution::Band { lo: -1.0, hi: 1.0 },
        (-0.5, 0.5),
    )
    .expect_err("a band prices nothing whose answer depends on its shape");
    assert_eq!(measure_unavailable_tag(&refusal), "band_has_no_measure");

    let policy = AnalysisPolicy::new(1.0).expect_err("mass 1 asks for an infinite box");
    assert_eq!(
        analysis_policy_error_tag(&policy),
        "quantile_mass_out_of_range"
    );
    // And the whole point of the pair: the same band ANSWERS the two
    // set-theoretic cases, so the refusal above is about the shape and
    // not about bands.
    assert_eq!(
        box_mass(
            &bore,
            &Distribution::Band { lo: -1.0, hi: 1.0 },
            (-2.0, 2.0)
        ),
        Ok(1.0)
    );
    assert!(AnalysisPolicy::new(0.5).is_ok());
}

/// LIB-B-MEASURES: the measurement AUTHORING vocabulary, minted from
/// real kernel values on the default no-interpreter build path.
///
/// The four verbs, their fixed dimensions and their ARGUMENT-ordered
/// reference pairs, taken from the kernel's own `verb`, `dim` and
/// `refs` rather than restated — so a fifth primitive breaks the
/// exhaustive matches behind those three and this row goes red on the
/// vocabulary rather than on a list.
#[test]
fn the_measure_verb_vocabulary_is_stable() {
    use pncad::document::{Dimension, MeasurePrimitive};

    let distance = MeasurePrimitive::Distance { a: 0, b: 1 };
    let angle = MeasurePrimitive::Angle { a: 2, b: 3 };
    let clearance = MeasurePrimitive::MinClearance { a: 4, b: 5 };
    let gap = MeasurePrimitive::Gap { outer: 6, inner: 7 };

    assert_eq!(distance.verb(), "distance");
    assert_eq!(angle.verb(), "angle");
    assert_eq!(clearance.verb(), "min_clearance");
    assert_eq!(gap.verb(), "gap");

    // Three of the four are lengths and exactly one is an angle: the
    // kind rides the verb, which is what makes an assertion's bound
    // type-checkable against the measure it constrains.
    assert_eq!(distance.dim(), Dimension::Length);
    assert_eq!(clearance.dim(), Dimension::Length);
    assert_eq!(gap.dim(), Dimension::Length);
    assert_eq!(angle.dim(), Dimension::Angle);

    // A gap's pair is (outer, inner) and NOT re-sorted — C5's formulas
    // are asymmetric in the roles, so the order is authored data.
    assert_eq!(gap.refs(), [6, 7]);
    assert_eq!(distance.refs(), [0, 1]);
}

/// LIB-B-MEASURES: the construction door's refusal, from the door.
///
/// `Node::measure` is called with an index past the end of the
/// reference list, so the fault is the kernel's answer rather than a
/// named variant — the shape `analysis_refusal_tags_are_stable` uses
/// one family over.
#[test]
fn the_measure_node_fault_tag_is_stable() {
    use crate::tags::measure_node_fault_tag;
    use pncad::document::{
        MeasureExpr, MeasureNodeFault, MeasurePrimitive, Node, ProfileProgram, RecipeNodeId,
        SitedRef,
    };
    use pncad::prelude::StableName;
    use pncad::select::{EntityKind, RoleSeg};

    let one_reference = vec![SitedRef::at_mint(StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(0),
        path: vec![RoleSeg::OutputBody],
    })];
    let fault = Node::<ProfileProgram>::measure(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        one_reference,
    )
    .expect_err("reference 1 of a one-reference measure names nothing");
    assert_eq!(measure_node_fault_tag(&fault), "ref_index_out_of_range");
    let MeasureNodeFault::RefIndexOutOfRange { verb, index, refs } = fault;
    assert_eq!((verb, index, refs), ("distance", 1, 1));
    // The message is prose, which is what `typed_err` asserts on every
    // raise — pinned here so the Python class's human half is checked
    // on the build path that has no interpreter.
    assert!(crate::errors::reads_as_prose(&fault.to_string()));
}

/// LIB-B-MEASURES: the two refusals the FOURTH verb adds, and the
/// asymmetry between them.
///
/// `MeasureUnavailableAt` is what the `f64` lane answers a
/// `min_clearance` with, and the binding evaluates at `f64` — so it is
/// reachable from Python and `tests/test_measures.py` reaches it
/// through a real document. `MinClearanceRefusal` is the interval
/// engine's own, and its ONLY producer is
/// `impl MinClearanceLane for geom_core::Interval`, behind the
/// `interval` feature; no Python evaluation reaches it at any feature
/// set, because the lane and not the feature is what gates it. So this
/// row is where the second one's tag and prose are pinned at all.
#[test]
fn the_fourth_verbs_two_refusals_are_stable() {
    use crate::tags::{measure_unavailable_at_tag, node_error_tag};
    use pncad::document::{MeasureUnavailableAt, MinClearanceRefusal};

    let absent = MeasureUnavailableAt::NeedsEnclosure {
        verb: "min_clearance",
        scalar: "f64",
        door: "clearance::min_separation",
    };
    assert_eq!(measure_unavailable_at_tag(&absent), "needs_enclosure");
    assert_eq!(absent.verb(), "min_clearance");
    assert!(crate::errors::reads_as_prose(&absent.to_string()));
    // The recourse is IN the refusal: it names the door that answers
    // rather than handing back a worse number.
    assert!(absent.to_string().contains("clearance::min_separation"));

    let refused = MinClearanceRefusal {
        class: "SubdivisionBudget",
        payload: "depth 12".to_string(),
    };
    assert_eq!(
        node_error_tag(&pncad::document::NodeErrorKind::MeasureClearanceRefused(
            refused.clone()
        )),
        "measure_clearance_refused"
    );
    assert!(crate::errors::reads_as_prose(&refused.to_string()));
}

/// LIB-B-MEASURES: an assertion's two directions, and the symbols a
/// report reads them as.
#[test]
fn the_assertion_directions_keep_their_symbols() {
    use pncad::document::AssertionDir;

    assert_eq!(AssertionDir::AtLeast.symbol(), ">=");
    assert_eq!(AssertionDir::AtMost.symbol(), "<=");
}

/// LIB-B-READBACK: the read-back doors' tag map, arm by arm.
///
/// Unlike `SelectRefusal`'s, this map IS the compile-time drift
/// alarm: neither `InterrogateError` nor `ReadbackError` is
/// `#[non_exhaustive]`, so a kernel arm added without a tag stops
/// this crate compiling. What the pin adds on top is the tag TEXT,
/// which the alarm cannot see — a renamed tag compiles fine and
/// silently breaks every caller branching on it.
///
/// Every arm is constructible here, `Dangling`'s two lanes included:
/// `DanglingRef` rides on the curated surface beside the refusal that
/// carries it, so this crate names both lanes and pins both tags.
/// The keys inside a lane are `topo`'s and come through the façade's
/// whole re-export of that layer; the tag does not depend on which
/// key kind a lane names, so a default key is the honest fixture.
#[test]
fn readback_refusal_tags_are_stable() {
    use crate::tags::interrogate_error_tag as tag;
    use pncad::document::RecipeNodeId;
    use pncad::select::{DanglingRef, EntityKind, InterrogateError as E, ReadbackError as R};
    use pncad::topo::{EntityId, GeomRef, SurfaceKey, VertexKey};

    let node = RecipeNodeId(0);
    assert_eq!(tag(&E::NodeNotEvaluated { node }), "node_not_evaluated");
    assert_eq!(tag(&E::NodeFailed { node }), "node_failed");
    assert_eq!(
        tag(&E::NodePoisoned {
            node,
            through: node
        }),
        "node_poisoned"
    );
    assert_eq!(tag(&E::NoSuchName), "no_such_name");
    assert_eq!(tag(&E::Ambiguous { candidates: 2 }), "ambiguous");
    assert_eq!(
        tag(&E::WrongKind {
            wanted: EntityKind::Face,
            found: EntityKind::Edge,
        }),
        "wrong_kind"
    );
    assert_eq!(tag(&E::WholeBody), "whole_body");
    assert_eq!(tag(&E::NoBodies { payload: "datum" }), "no_bodies");
    assert_eq!(tag(&E::NoSuchBody { index: 1 }), "no_such_body");
    // The geometry half arrives under its OWN tag, not a wrapper's —
    // and `Dangling`'s two lanes arrive under one tag each, because
    // a stale handle and a body whose own geometry reference dangles
    // are different facts and a caller branches on which.
    assert_eq!(
        tag(&E::Readback(R::Dangling {
            what: DanglingRef::Entity(EntityId::Vertex(VertexKey::default())),
        })),
        "dangling_entity"
    );
    assert_eq!(
        tag(&E::Readback(R::Dangling {
            what: DanglingRef::Geometry(GeomRef::Surface(SurfaceKey::default())),
        })),
        "dangling_geometry"
    );
    assert_eq!(
        tag(&E::Readback(R::NoCanonicalFrame { carrier: "nurbs" })),
        "no_canonical_frame"
    );
    assert_eq!(tag(&E::Readback(R::NoCarrier)), "no_carrier");
}

/// LIB-B-PICKING: the two picking refusals, pinned tag by tag.
///
/// The standing ladder is spelled EXACTLY as the read-back doors spell
/// it (the test above), and that is the property worth a pin rather
/// than a comment: "node 7 has no result in this evaluation" is one
/// fact about the run, and a caller that already branches on
/// `node_not_evaluated` from a frame read must not have to learn a
/// second word for it at the pick. The forwarding is what makes that
/// true, so the assertions below are written to fail if a wrapper tag
/// is ever introduced.
///
/// ONE arm has no façade constructor and so no line here — the
/// `select_refusal_tags_are_stable` caveat, for a different reason.
/// `HitTestError::Unnamed`'s payload is an `EntityRef`, an arena key
/// beside a body index, and the façade deliberately does not name that
/// type; its tag is covered by the match itself, which is exhaustive
/// and would stop compiling if the arm moved.
///
/// The index arm HAS one, and the pin below is what that buys: the
/// payload rides on the curated surface beside the refusal that
/// carries it, so this crate names it, constructs it, and pins both
/// words — the carrier's `mesh_index` and the payload's own.
#[test]
fn picking_refusal_tags_are_stable() {
    use crate::tags::{hit_test_error_tag, mesh_pick_error_tag, node_pick_error_tag};
    use pncad::document::RecipeNodeId;
    use pncad::mesh::TessellateError;
    use pncad::select::{HitTestError as H, MeshPickError as M, NodePickError as N};

    let node = RecipeNodeId(0);
    assert_eq!(
        hit_test_error_tag(&H::NodeNotEvaluated { node }),
        "node_not_evaluated"
    );
    assert_eq!(hit_test_error_tag(&H::NodeFailed { node }), "node_failed");
    assert_eq!(
        hit_test_error_tag(&H::NodePoisoned {
            node,
            through: node
        }),
        "node_poisoned"
    );

    // The pick door's own two arms: "never draws" and "draws nothing
    // today" are different states and keep different tags.
    assert_eq!(node_pick_error_tag(&N::NotABody { node }), "not_a_body");
    assert_eq!(
        node_pick_error_tag(&N::NoSuchBody { node, body: 1 }),
        "no_such_body"
    );

    // The standing arm FORWARDS: no `standing` wrapper tag exists, and
    // a caller reads the same three words at either door.
    for standing in [
        H::NodeNotEvaluated { node },
        H::NodeFailed { node },
        H::NodePoisoned {
            node,
            through: node,
        },
    ] {
        assert_eq!(
            node_pick_error_tag(&N::Standing(standing)),
            hit_test_error_tag(&standing)
        );
    }

    // ...and so does the tessellation arm, under the tessellator's own
    // word rather than a `tessellate` wrapper.
    assert_eq!(
        node_pick_error_tag(&N::Tessellate(TessellateError::InvalidChordalTolerance {
            value: 0.0
        })),
        "invalid_chordal_tolerance"
    );

    // The index arm does NOT forward: `mesh_index` names the door
    // whose invariant broke, and the payload's own word is pinned
    // beside it rather than in place of it. Both are Python-visible —
    // `variant` and `index_variant` — so both are contract.
    let corrupt = M::PositionOutOfRange {
        patch: 0,
        triangle: 0,
        index: 0,
    };
    assert_eq!(node_pick_error_tag(&N::Index(corrupt)), "mesh_index");
    assert_eq!(mesh_pick_error_tag(&corrupt), "position_out_of_range");
}

/// **Every `NodePickError` arm's index numbers, constructed and
/// read.**
///
/// `crate::pick_payload::index_payload` is what the exception's
/// `patch`, `triangle` and `index` attributes are read off, and this
/// pin says which arm carries them and what they are.
///
/// It is here rather than in `tests/test_picking.py` because the arm
/// that carries them cannot be provoked from Python at all: a
/// tessellated mesh whose triangles index outside their own position
/// buffer is a kernel-side invariant break, unauthorable through any
/// door and unconstructible through the façade's Python surface. The
/// payload IS constructible here, which is the whole reason the
/// flattening sits outside `crate::py`. The Python rows own the other
/// half — that the three attributes exist and read `None` on the arms
/// a caller can reach.
#[test]
fn every_pick_arm_projects_the_index_numbers_it_carries() {
    use crate::pick_payload::index_payload;
    use pncad::document::RecipeNodeId;
    use pncad::mesh::TessellateError;
    use pncad::select::{HitTestError as H, MeshPickError as M, NodePickError as N};

    let node = RecipeNodeId(0);
    // The one arm that carries them, at numbers no two of which are
    // equal: a slot swapped for another shows up as a moved number
    // rather than as three zeroes agreeing.
    let corrupt = N::Index(M::PositionOutOfRange {
        patch: 3,
        triangle: 11,
        index: 47,
    });
    let numbers = index_payload(&corrupt);
    assert_eq!(numbers.present(), ["patch", "triangle", "index"]);
    assert_eq!(numbers.patch, Some(3));
    assert_eq!(numbers.triangle, Some(11));
    assert_eq!(numbers.index, Some(47));

    // Every other arm answers all three by name, not by wildcard.
    for other in [
        N::Standing(H::NodeNotEvaluated { node }),
        N::NotABody { node },
        N::NoSuchBody { node, body: 1 },
        N::Tessellate(TessellateError::InvalidChordalTolerance { value: 0.0 }),
    ] {
        let numbers = index_payload(&other);
        assert_eq!(numbers, crate::pick_payload::IndexPayload::NONE);
        assert!(numbers.present().is_empty());
    }
}

/// **Every constructible `MateFault` arm's payload, built and read.**
///
/// The arm table, executable. `crate::mate_payload::mate_payload` is
/// the projection `MateFault`'s seventeen Python attributes are read
/// off, and this pin says what each arm puts on the wire: the exact
/// set it CARRIES, in publication order, with the rest `None`.
///
/// **Nine of the thirteen arms are built here.** The other four —
/// `Frame`, `Band`, `Indeterminate` and `Unleverable` — are exactly
/// the arms whose payload is a nested refusal, which the projection
/// does not flatten, so building one would add nothing to the wire.
/// Three of the four types are nameable from here — `FrameError` and
/// `BandField` one module hop below the curated lists at
/// `pncad::geom_core`, `MarginDiag` on the prelude — and
/// `LeverRefusal` is lifted to no crate root at all, so `Unleverable`
/// cannot be built here whatever this table wants. That costs the
/// table its totality and nothing else: totality of the PROJECTION is
/// a different guarantee and a stronger one — `mate_payload`'s match
/// is exhaustive with no wildcard, so an arm that reached Python
/// unprojected would not compile. Filling the four is
/// `work/lib/mate-fault-arms-carry-payload-that-does-not-cross.md`.
#[test]
fn every_mate_fault_arm_projects_the_payload_it_carries() {
    use crate::mate_payload::mate_payload;
    use pncad::document::{
        DocumentId, MateFault as F, MateSide, NodeErrorKind, NodeRefusal, RecipeNodeId, Subgroup,
    };

    let id = RecipeNodeId;
    let carries = |fault: &F, want: &[&str]| {
        assert_eq!(
            mate_payload(fault).present(),
            want,
            "the payload `{}` puts on the wire has moved",
            crate::tags::mate_fault_tag(fault)
        );
    };

    // The two documents a mispaired read named are the arm's whole
    // payload and neither crosses: the subject is not a mate, and a
    // document id is not one of this value's attributes.
    carries(
        &F::PosesOfAnotherDocument {
            expected: DocumentId::derive("a"),
            found: DocumentId::derive("b"),
        },
        &[],
    );
    carries(&F::ClassNotAdmitted { mate: id(1) }, &["mate"]);
    carries(
        &F::TableLacks {
            mate: id(1),
            what: "clocking on a planar rest",
        },
        &["mate", "what"],
    );
    carries(
        &F::Contradictory {
            held: id(1),
            added: id(2),
            predicate: "mate_member_empty",
            clash: 0.25,
            lever: None,
        },
        &["held", "added", "predicate", "clash"],
    );
    carries(
        &F::Under {
            mate: id(1),
            parent: id(2),
            child: id(3),
            residual: Subgroup::Planar {
                normal: pncad::authoring::v3(0.0, 0.0, 1.0),
            },
        },
        &["mate", "parent", "child", "residual"],
    );
    carries(
        &F::DanglingHead {
            mate: id(1),
            side: MateSide::B,
            head: id(2),
        },
        &["mate", "side", "head"],
    );
    carries(
        &F::PlacerRefused {
            mate: id(1),
            side: MateSide::A,
            placer: id(2),
            error: NodeRefusal::from(NodeErrorKind::NonFiniteDirection { role: "axis" }),
        },
        &["mate", "side", "placer", "error"],
    );
    carries(
        &F::PartSelectsAnotherCopy {
            mate: id(1),
            side: MateSide::B,
            part: id(2),
            named: 3,
            selected: -1,
        },
        &["mate", "side", "part", "named", "selected"],
    );
    carries(
        &F::SelfMate {
            mate: id(1),
            instance: id(2),
        },
        &["mate", "instance"],
    );

    // The node roles answer with the ids they were given, not with
    // the first id repeated: the roles are what a caller acts on.
    let under = F::Under {
        mate: id(4),
        parent: id(9),
        child: id(16),
        residual: Subgroup::Trivial,
    };
    let payload = mate_payload(&under);
    assert_eq!(payload.mate, Some(id(4)));
    assert_eq!(payload.parent, Some(id(9)));
    assert_eq!(payload.child, Some(id(16)));

    // The placer arm's `error` is the evaluation layer's own tag,
    // unaltered — the vocabulary `EvaluationError.kind` speaks.
    let placer = F::PlacerRefused {
        mate: id(1),
        side: MateSide::A,
        placer: id(2),
        error: NodeRefusal::from(NodeErrorKind::NonFiniteDirection { role: "axis" }),
    };
    assert_eq!(mate_payload(&placer).error, Some("non_finite_direction"));
}

/// LIB-B-CANCEL: the evaluation door joins the standing ladder, and
/// says so against the doors that already speak it.
///
/// A canceled run holds the completed PREFIX, so `Evaluation.value`
/// on a node past it has to answer "this run has no result for that
/// node" — the ladder's first rung, the same fact `ReadbackError` and
/// `HitTestError` report. Those two reach the word through a `match`
/// on a kernel arm; the evaluation door cannot, because
/// `Evaluation::result` answers a bare `None` and the reason tag is
/// this crate's own. So the word is a CONST and this is the pin that
/// keeps the copy honest.
///
/// It runs in BOTH directions on purpose: renaming the kernel arms'
/// tag fails here, and so does editing the const away from them. That
/// is the property `picking_refusal_tags_are_stable` protects for the
/// pick, one door further out.
#[test]
fn the_evaluation_door_speaks_the_standing_ladder() {
    use crate::tags::{NODE_NOT_EVALUATED, hit_test_error_tag, interrogate_error_tag};
    use pncad::document::RecipeNodeId;
    use pncad::select::{HitTestError as H, InterrogateError as I};

    let node = RecipeNodeId(0);
    assert_eq!(
        NODE_NOT_EVALUATED,
        hit_test_error_tag(&H::NodeNotEvaluated { node })
    );
    assert_eq!(
        NODE_NOT_EVALUATED,
        interrogate_error_tag(&I::NodeNotEvaluated { node })
    );

    // And it is NOT the other no-entry fact. "The document has no such
    // node" and "this run never reached it" are two states the door
    // kept collapsed while only one of them could arise, and the whole
    // of what B-CANCEL changed at this door is that both now can.
    assert_ne!(NODE_NOT_EVALUATED, "unknown_node");
}

/// LIB-B-RESOLVE: the three resolution states, pinned word by word —
/// and pinned by CONSTRUCTING them, because nothing else can.
///
/// Every other pin in this file builds its subject by naming a variant
/// and filling its fields. That is unavailable here: `Resolved` is
/// decided absent from the façade (`crates/pncad/tests/all.rs`'s
/// `NOT_CARRIED`, and its field is an arena key), and the two failure
/// arms that can be spelled cannot be FILLED — `Vanished` needs a
/// `Diagnosis`, `Ambiguous` a `TieWitness`, `NodeGone` a
/// `RecipeEditRef`, all three of them interior. So a `Resolution`
/// cannot be assembled through `pncad` at all; it can only be
/// OBTAINED, by resolving a real name against a real run. This test
/// builds a document, and the three states are three things that
/// happen to it.
///
/// That is a stronger pin than the literal one it replaces, and worth
/// naming as such: it asserts that each state is REACHABLE by the
/// route a caller reaches it, not merely that a match arm returns a
/// string. It runs on the default no-Python path, so hosted CI checks
/// the words a Python caller branches on without an interpreter.
///
/// **The per-arm words are pinned wherever this fixture reaches the
/// arm**, which is two of six: `node_gone` on the deleted node and
/// `target_not_evaluated` on the canceled run. `ResolveIndeterminate`
/// is constructible — its arms carry a `RecipeNodeId` and nothing
/// else — so the other two of ITS three are pinned as literals below.
/// `vanished` needs two runs of two documents, which
/// `tests/test_resolve.py` already builds, so it is pinned there
/// rather than duplicated here. `ambiguous` is reached by no test on
/// either side of the boundary: an N2 tie needs a tie-marked table
/// and no door on this surface authors one. Its word cannot silently
/// move even so — the match is exhaustive and the inventory pins
/// every literal — but nothing here asserts that a real tie arrives
/// under it, and that is the honest statement of this pin's reach.
#[test]
fn resolution_status_tags_are_stable() {
    use crate::tags::{resolution_status_tag, resolve_error_tag, resolve_indeterminate_tag};
    use pncad::document::{
        CancelToken, Datum, DocEdit, EvalOptions, Expr, LoopProgram, Node, ProfileDoc,
        ProfileProgram, apply, evaluate,
    };
    use pncad::prelude::Dimension;
    use pncad::select::{Resolution, ResolveIndeterminate, RunCtx, all_faces, resolve};

    // The indeterminate arms carry a node id and nothing else, so all
    // three are spellable here; the failure arms are not (this
    // function's own doc comment says why).
    let node = pncad::document::RecipeNodeId(0);
    assert_eq!(
        resolve_indeterminate_tag(&ResolveIndeterminate::TargetFailed { node }),
        "target_failed"
    );
    assert_eq!(
        resolve_indeterminate_tag(&ResolveIndeterminate::TargetPoisoned { through: node }),
        "target_poisoned"
    );
    assert_eq!(
        resolve_indeterminate_tag(&ResolveIndeterminate::TargetNotEvaluated { node }),
        "target_not_evaluated"
    );

    let tol = Tol::witness();
    let doc: ProfileDoc = crate::identity::derived("resolution-status-probe", tol);
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");

    let insert = |doc: &ProfileDoc, node: Node<ProfileProgram>| {
        let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the node inserts");
        let id = applied.record.minted.expect("an inserted id");
        (applied.doc, id)
    };
    let (doc, plane) = insert(
        &doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
    );
    let (doc, profile) = insert(
        &doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
                    .expect("finite corners"),
            ],
        }),
    );
    let (doc, extrude) = insert(
        &doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );

    let run = |doc: &ProfileDoc, cancel: &CancelToken| {
        evaluate::<f64>(doc, None, cancel, &EvalOptions::default(), tol)
    };
    let live = CancelToken::new();
    let ev = run(&doc, &live);
    let mut faces = all_faces(&ev, extrude);
    faces.sort();
    assert_eq!(faces.len(), 6, "a cube's six faces");
    let stored = faces.remove(0);

    // RESOLVED: the ordinary run, asked about its own name.
    assert_eq!(
        resolution_status_tag(&resolve(
            RunCtx {
                doc: &doc,
                eval: &ev
            },
            &stored
        )),
        "resolved"
    );

    // FAILED: the minting node is gone from the document, so the name
    // is stranded and the repair is an explicit rebind.
    let pruned = apply(&doc, &DocEdit::DeleteNode { id: extrude }, tol)
        .expect("the leaf deletes")
        .doc;
    let after = run(&pruned, &live);
    let verdict = resolve(
        RunCtx {
            doc: &pruned,
            eval: &after,
        },
        &stored,
    );
    assert_eq!(resolution_status_tag(&verdict), "failed");
    // ...and WHICH failure, which is the word a repair branches on: a
    // node that left the document is rebound onto a different
    // feature, where a tie would have been refined among `offers`.
    match &verdict {
        Resolution::Failed(failure) => {
            assert_eq!(resolve_error_tag(&failure.error), "node_gone");
        }
        other => panic!("a deleted minting node must fail: {other:?}"),
    }

    // INDETERMINATE: the node is still there and the RUN did not reach
    // it — a canceled run's suffix, which is the one arm of this state
    // reachable without breaking a feature. The name is unharmed and
    // the repair is to evaluate again, which is exactly why this must
    // not answer `failed`.
    let canceled = CancelToken::new();
    canceled.cancel();
    let partial = run(&doc, &canceled);
    let verdict = resolve(
        RunCtx {
            doc: &doc,
            eval: &partial,
        },
        &stored,
    );
    assert_eq!(resolution_status_tag(&verdict), "indeterminate");
    match &verdict {
        Resolution::Indeterminate(cause) => {
            assert_eq!(resolve_indeterminate_tag(cause), "target_not_evaluated");
        }
        other => panic!("a canceled run's suffix must be indeterminate: {other:?}"),
    }
}

/// LIB-PYSEL: `SelectRefusal` is `#[non_exhaustive]`, so the tag
/// match cannot be the compile-time drift alarm the other tag
/// functions are, and this pin does NOT restore one: it constructs
/// every arm whose payload the curated surface can build and asserts
/// its tag, which means it cannot construct — and cannot fail on — an
/// arm the kernel has not shipped yet. What it gives is the
/// enumeration the wildcard hides: one line per arm this binding
/// speaks, so a kernel arm added without a tag here is an absence in
/// a list rather than invisible behind the wildcard. The safety
/// property is the crossing's own typed `unclassified` refusal
/// (`py/select.rs`), not this test. (`InBand`/`PairInBand`/
/// `BadValue` carry funnel/expression internals with no public
/// constructor; their tags are covered by the match itself.)
#[test]
fn select_refusal_tags_are_stable() {
    use crate::tags::select_refusal_tag;
    use pncad::document::{Dimension, RecipeNodeId};
    use pncad::select::{EntityKind, InterrogateError, SelectRefusal};

    let name = Box::new(pncad::prelude::StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(0),
        path: Vec::new(),
    });
    assert_eq!(
        select_refusal_tag(&SelectRefusal::TiedDisagrees {
            name: name.clone(),
            matched: 1,
            candidates: 2,
        }),
        "tied_disagrees"
    );
    assert_eq!(
        select_refusal_tag(&SelectRefusal::Unreadable {
            name,
            error: InterrogateError::NoSuchName,
        }),
        "unreadable"
    );
    assert_eq!(
        select_refusal_tag(&SelectRefusal::NotADatum {
            datum: RecipeNodeId(0),
            found: "body",
        }),
        "not_a_datum"
    );
    assert_eq!(
        select_refusal_tag(&SelectRefusal::NotALength {
            dim: Dimension::Angle,
        }),
        "not_a_length"
    );
    assert_eq!(select_refusal_tag(&SelectRefusal::Band), "band");
}

/// LIB-PYG5: `ContactClass` is `#[non_exhaustive]` kernel-side, so
/// the Python mirror (`py/flush.rs`) is forced to carry a wildcard
/// arm and the compile-time drift alarm is unavailable — an unknown
/// class refuses typed (`unclassified`) at the crossing instead.
///
/// That forced wildcard has a cost this pin pays: a wildcarded alarm
/// cannot fire, so the pin ENUMERATES what the mirror speaks, one line
/// per class, and a class added to the kernel without a line here is
/// visible as an absence in a list rather than invisible behind a
/// wildcard.
///
/// It is deliberately NOT a `_ => panic!()` over the kernel enum:
/// that would red on every downstream build the moment the kernel
/// reserved a class, which is precisely the coupling
/// `#[non_exhaustive]` exists to prevent. The crossing's typed
/// refusal is the safety property; this list is the reminder.
#[test]
fn the_contact_class_mirror_matches_the_kernel() {
    let spoken = |class| match class {
        pncad::select::ContactClass::Rest => "rest",
        pncad::select::ContactClass::Tangent => "tangent",
        _ => "unclassified",
    };
    assert_eq!(spoken(pncad::select::ContactClass::Rest), "rest");
    assert_eq!(
        spoken(pncad::select::ContactClass::Tangent),
        "tangent",
        "Tangent crossed into the mirror with M9-1; a class the binding \
         cannot name refuses typed at the crossing instead"
    );
}

/// LIB-PYG5: the declare-sugar refusal tags, exercised through the
/// real doors on the default (no-Python) path. The `Edit` arm
/// carries the document layer's own tag through.
#[test]
fn declare_error_tags_are_stable() {
    use crate::tags::declare_error_tag;
    use pncad::select::{DeclareError, declare_node};

    let empty =
        declare_node::<pncad::document::ProfileProgram>(&[]).expect_err("an empty declare refuses");
    assert_eq!(declare_error_tag(&empty), "no_findings");
    assert_eq!(declare_error_tag(&DeclareError::NoMintedId), "no_minted_id");
}

/// The binding matches `Expr::literal`'s OWN refusals rather than
/// pre-checking them, and the tags Python sees are stable.
///
/// **Scope: the literal-construction door only.** It is one of TWO
/// doors that reach the document layer's `DimensionError`; the other
/// is `load`, and
/// `the_load_door_reaches_dimension_mismatch_arms_as_an_untyped_unreadable_refusal`
/// below is its half. Read the two together — either alone is a
/// premise that excludes the mode the other covers.
#[test]
fn literal_refusals_come_from_the_kernel_with_stable_tags() {
    use pncad::document::Expr;
    let non_finite = Expr::literal(f64::NAN, Dimension::Length).expect_err("NaN refuses");
    assert_eq!(expr_dimension_error_tag(&non_finite), "non_finite");
    let count = Expr::literal(3.0, Dimension::Count).expect_err("a continuous count refuses");
    assert_eq!(expr_dimension_error_tag(&count), "count_is_integer");
    assert!(Expr::literal(1.5, Dimension::Length).is_ok());

    // The reachable set, exhaustively: every dimension, a finite and
    // a non-finite value each. Nothing here is a dimension MISMATCH,
    // which is what makes `LiteralError` the right class.
    let mut reachable = std::collections::BTreeSet::new();
    for dim in [
        Dimension::Length,
        Dimension::Angle,
        Dimension::Count,
        Dimension::Scalar,
    ] {
        for value in [0.0, 1.5, 3.0, -2.0, f64::NAN, f64::INFINITY] {
            if let Err(err) = Expr::literal(value, dim) {
                reachable.insert(expr_dimension_error_tag(&err));
            }
        }
    }
    assert_eq!(
        reachable.into_iter().collect::<Vec<_>>(),
        ["count_is_integer", "non_finite"],
        "literal construction now refuses on an arm outside the \
         literal-value pair — it raises `LiteralError`, so decide \
         whether that is still the right class before widening this pin"
    );
}

/// LIB-B-FORMAT: the display formatter's tag map, and the CLASS
/// question it settles.
///
/// The map has one arm, so the interesting content is not the string
/// — it is that the string is `non_finite`, the SAME tag
/// [`expr_dimension_error_tag`] answers for `NonFiniteLiteral`, while
/// the two are nonetheless different exception classes. That is the
/// deliberate shape: the tag names the fact (a float that is NaN or
/// ±∞), the class names the door (INTO a recipe, or OUT to a human),
/// and a caller who wants to know which asks the class it already
/// caught rather than parsing a discriminant.
///
/// Driven through `fmt_length` / `fmt_angle` themselves rather than
/// by constructing the arm, on
/// [`expression_text_door_tags_are_stable`]'s reasoning: the question
/// is what a caller sees when the door refuses, and a hand-built
/// value pins the map against something the door might never produce.
/// The finite half is asserted too, and it is not filler — a
/// formatter that refused everything would satisfy the refusal
/// assertions alone.
#[test]
fn display_formatter_refusals_carry_the_shared_non_finite_tag() {
    use crate::tags::fmt_quantity_error_tag as tag;
    use pncad::quantity::{DEG, MM, fmt_angle, fmt_length};

    for poison in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let refused = fmt_length(poison, MM).expect_err("poison has no display form");
        assert_eq!(tag(&refused), "non_finite");
        let refused = fmt_angle(poison, DEG).expect_err("poison has no display form");
        assert_eq!(tag(&refused), "non_finite");
    }
    assert_eq!(fmt_length(0.025, MM).expect("finite"), "25 mm");
    assert_eq!(fmt_angle(0.0, DEG).expect("finite"), "0 deg");

    // Same fact, same tag, different class — the paragraph above, as
    // an assertion rather than as a claim about what someone meant.
    let into_a_recipe = pncad::document::Expr::literal(f64::NAN, Dimension::Length)
        .expect_err("a non-finite literal refuses");
    assert_eq!(expr_dimension_error_tag(&into_a_recipe), "non_finite");
    assert_eq!(ErrorClass::Literal.class_name(), "LiteralError");
    assert_eq!(ErrorClass::FmtQuantity.class_name(), "FmtQuantityError");

    // The refusal's own prose is what crosses as the message, and it
    // is prose rather than a `Debug` dump — the rule
    // `crate::py::typed_err` asserts on every raise.
    let refused = fmt_length(f64::NAN, MM).expect_err("poison has no display form");
    assert!(reads_as_prose(&refused.to_string()));
    assert!(!reads_as_prose(&format!("{refused:?}")));
}

/// LIB-B-EXPR-READ: the text door's tag map, arm by arm, driven
/// through `parse_expr` itself rather than by constructing arms.
///
/// Every case here is a SOURCE STRING, which is the honest fixture: a
/// hand-built `ParseError` would pin the map against a value the
/// parser might never produce, and the question the map answers is
/// what a Python caller sees when their text is refused. The tags are
/// the compile-time alarm's blind spot — `ParseError` is not
/// `#[non_exhaustive]`, so a new arm stops this crate compiling, but a
/// RENAMED tag compiles fine and silently breaks every caller
/// branching on it.
///
/// Ten of the eleven arms are reachable from text, and this pins
/// those ten. `malformed_number` is the exception, and it is a
/// measurement rather than an omission: the lexer hands
/// `f64::from_str` only a run of digits with at most one dot, so
/// every malformed shape is refused EARLIER and under a different arm
/// — `"1.2.3"` is an `unexpected_char` at the second dot, `"1e999"` a
/// `dimension` refusal on the non-finite literal it reads to, `"1e"`
/// an `unknown_unit`. The arm is defensive rather than removable (the
/// lexer's rule is not `f64`'s and need not stay a subset of it), so
/// it keeps its tag; what it does not have is a source string that
/// produces it, which is why it is absent below rather than pinned
/// against a hand-built value.
///
/// The `Dimension` arm IS reachable, and it is the one that shows why
/// it carries a tag of its own — `"1 m + 1 rad"` is a dimension
/// mismatch AT a byte offset, and the offset is what the inner
/// refusal cannot say.
#[test]
fn expression_text_door_tags_are_stable() {
    use crate::tags::parse_error_tag as tag;
    use pncad::document::{ParamName, parse_expr};

    let mut declared = BTreeMap::new();
    declared.insert(ParamName::new("width"), Dimension::Length);
    let refuse = |src: &str| {
        parse_expr(src, &declared).expect_err("this source is not a well-formed expression")
    };

    assert_eq!(tag(&refuse("1 m $ 2")), "unexpected_char");
    assert_eq!(tag(&refuse("1 m +")), "unexpected_end");
    assert_eq!(tag(&refuse("(1 m 2 m)")), "unexpected_token");
    assert_eq!(tag(&refuse("1 m 2 m")), "trailing_input");
    assert_eq!(tag(&refuse("99999999999999999999999")), "integer_overflow");
    assert_eq!(tag(&refuse("1 furlong")), "unknown_unit");
    assert_eq!(tag(&refuse("hypot(1, 2)")), "unknown_function");
    assert_eq!(tag(&refuse("sin(1 rad, 2 rad)")), "wrong_arity");
    assert_eq!(tag(&refuse("height")), "unknown_param");
    assert_eq!(tag(&refuse("1 m + 1 rad")), "dimension");

    // The `Dimension` arm's position is the whole reason it keeps its
    // own tag: the inner refusal carries no byte offset, so routing
    // it to `LiteralError` would drop the one fact that says where to
    // edit. The inner tag rides along as the exception's `kind`.
    match refuse("1 m + 1 rad") {
        pncad::document::ParseError::Dimension { pos, error } => {
            assert!(
                pos > 0,
                "the refused reduction has a position in the source"
            );
            assert_eq!(expr_dimension_error_tag(&error), "mismatch");
        }
        other => panic!("a dimension mismatch, not {other}"),
    }

    // A well-formed source is not refused, so the assertions above
    // are about the grammar and not about a door that refuses
    // everything.
    assert!(parse_expr("width / 2.0 + 3 mm", &declared).is_ok());
}

/// LIB-B-EXPR-READ: the evaluator's tag map, arm by arm.
///
/// Six of the seven arms are provoked through `eval`/`eval_count`
/// themselves against a real document's environment;
/// `count_overflow` is constructed, because reaching it needs a count
/// expression whose exact arithmetic overflows `i64` and the text
/// door refuses the literals that would build one.
///
/// The environments come from `Doc::param_env`, which is the door the
/// binding uses — building a `ParamEnv` by hand would pin the map
/// against bindings no document produces, and the
/// `param_dimension_mismatch` case in particular is only honest
/// because it is what a redeclared parameter actually does: an
/// expression parsed against a document that declares `width` as a
/// length, evaluated against one that declares it as a count.
///
/// What is NOT in this map is the point of the last assertion:
/// division by zero is not a refusal in the expression layer at all.
/// The evaluator has no branches, so the poison flows through the
/// scalar and is caught at the END, as `non_finite_result` on the
/// finished value.
#[test]
fn expression_evaluation_tags_are_stable() {
    use crate::tags::eval_error_tag as tag;
    use pncad::document::{
        DocEdit, DocParam, EvalError, Expr, ParamName, ProfileDoc, apply, eval, eval_count,
        parse_expr,
    };

    let tol = Tol::witness();
    let width = ParamName::new("width");
    let declare = |name: &ParamName, param: DocParam| {
        let doc: ProfileDoc = crate::identity::derived("expression-evaluation-probe", tol);
        apply(
            &doc,
            &DocEdit::SetDocParam {
                name: name.clone(),
                value: param,
            },
            tol,
        )
        .expect("a parameter declaration applies")
        .doc
    };

    let lengths = declare(&width, DocParam::continuous(Dimension::Length, 0.1));
    let counts = declare(&width, DocParam::Count { value: 3 });
    let empty: ProfileDoc = crate::identity::derived("expression-evaluation-empty", tol);

    let mut declared = BTreeMap::new();
    declared.insert(width.clone(), Dimension::Length);
    let parse = |src: &str| parse_expr(src, &declared).expect("a well-formed expression");

    let bound = lengths.param_env::<f64>();

    // The value the whole family exists for: an expression a caller
    // could not otherwise evaluate without re-implementing the
    // evaluator.
    assert_eq!(
        eval(&parse("width / 2.0 + 3 mm"), &bound).expect("it evaluates"),
        // Spelled as the arithmetic rather than as `0.053`, because
        // that is the claim: the evaluator IS the `f64` arithmetic
        // over the document's exact stored values, with no rounding
        // step anywhere in it. The decimal literal is not equal to
        // this and saying so would be the wrong pin.
        0.1 / 2.0 + 0.003
    );

    assert_eq!(
        tag(&eval(&parse("width"), &empty.param_env::<f64>()).expect_err("no binding")),
        "unknown_param"
    );

    // The expression's reference recorded a length; this document
    // declares the same name as a count.
    assert_eq!(
        tag(&eval(&parse("width"), &counts.param_env::<f64>())
            .expect_err("the dimensions disagree")),
        "param_dimension_mismatch"
    );

    assert_eq!(
        tag(&eval(&parse("3"), &bound).expect_err("a count does not evaluate continuously")),
        "count_expr_in_continuous_eval"
    );
    assert_eq!(
        tag(&eval_count(&parse("1 m"), &bound).expect_err("a length is not a count")),
        "continuous_expr_in_count_eval"
    );
    assert_eq!(
        tag(&eval(&parse("scalar(9999999999)"), &bound)
            .expect_err("that count does not promote exactly")),
        "count_to_scalar_out_of_range"
    );
    assert_eq!(tag(&EvalError::CountOverflow), "count_overflow");

    // Division by zero: no refusal at the operation, a poisoned value
    // caught at the boundary.
    let zero = Expr::literal(0.0, Dimension::Scalar).expect("finite");
    let one = Expr::literal(1.0, Dimension::Length).expect("finite");
    let pole = Expr::div(one, zero).expect("a scalar divisor is legal");
    assert_eq!(
        tag(&eval(&pole, &bound).expect_err("the pole refuses at the boundary")),
        "non_finite_result"
    );
}

/// **The second door.** `WireExpr::rebuild` (the load path) re-runs
/// every dimension check through `Expr`'s OPERATOR builders, so a
/// hand-edited save file reaches the genuine dimension-mismatch arms
/// with no new binding at all — six of them, executed here.
///
/// Today they arrive in Python as `PersistError` with `variant ==
/// "unreadable"` — the persistence door's one refusal for valid JSON
/// its types reject, recourse attached — because the deserializer
/// `Debug`-formats the structured refusal into a serde message and
/// serde classifies that as data it could not place. That is a real
/// misrouting and it is **issue #694**, not this crate's to fix: a
/// dimension mismatch is not "vocabulary this build lacks", and a
/// `format!("{err:?}")` message is not the "typed exception carrying
/// the structured error" this crate's taxonomy promises.
///
/// What this test is for is the DECISION the fix will force. When
/// #694 gives these a typed class, this assertion goes red, and
/// whoever changes it has to answer the question the three names make
/// easy to get wrong: a dimension mismatch from the load path is not
/// a `LiteralError` (nothing about it is a literal) and it is not the
/// quantity boundary's `DimensionError` either.
#[test]
fn the_load_door_reaches_dimension_mismatch_arms_as_an_untyped_unreadable_refusal() {
    let tol = Tol::witness();
    use pncad::document::{
        Datum, DocEdit, Expr, LoopProgram, Node, ProfileDoc, ProfileProgram, apply, save,
    };
    use pncad::prelude::Dimension;

    let doc: ProfileDoc = crate::identity::derived("dimension-routing-probe", tol);
    let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite corners");
    // The frame the profile is drawn on. Its components are literals
    // too, and they come first in the wire, so the FIRST-literal
    // replacement below now lands on the frame's origin rather than on
    // a profile point. The probe is about the load door's dimension
    // walk, which reaches both alike.
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite");
    let framed = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Datum(Datum::Frame {
                origin: [len(0.0), len(0.0), len(0.0)],
                u: [scl(1.0), scl(0.0), scl(0.0)],
                v: [scl(0.0), scl(1.0), scl(0.0)],
            }),
        },
        tol,
    )
    .expect("the frame inserts");
    let plane = framed.record.minted.expect("a frame id");
    let applied = apply(
        &framed.doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane,
                loops: vec![square],
            }),
        },
        tol,
    )
    .expect("the profile inserts");
    let text = save(&applied.doc, &[], tol).expect("the document saves");
    let (header, body) = text.split_once("\n{").expect("a header line then the body");
    let body = format!("{{{body}");
    let saved: serde_json::Value = serde_json::from_str(&body).expect("the save body is JSON");

    // Every case replaces the FIRST literal in the document, so this
    // is driven by the wire SHAPE rather than by a node id.
    let length = serde_json::json!({ "Literal": { "value": 1.0, "dim": "Length" } });
    let angle = serde_json::json!({ "Literal": { "value": 1.0, "dim": "Angle" } });
    let cases = [
        ("mismatch", serde_json::json!({ "Add": [length, angle] })),
        (
            "mul_needs_scalar",
            serde_json::json!({ "Mul": [length, length] }),
        ),
        (
            "div_needs_scalar_divisor",
            serde_json::json!({ "Div": [length, angle] }),
        ),
        ("trig_needs_angle", serde_json::json!({ "Sin": length })),
        ("not_count", serde_json::json!({ "CountToScalar": length })),
        (
            "unknown_display_unit",
            serde_json::json!({
                "Literal": { "value": 1.0, "dim": "Length", "unit": "furlong" }
            }),
        ),
        (
            "display_unit_mismatch",
            serde_json::json!({
                "Literal": { "value": 1.0, "dim": "Angle", "unit": "mm" }
            }),
        ),
    ];

    for (arm, expr) in cases {
        let mut mutated = saved.clone();
        assert!(
            replace_first_literal(&mut mutated, &expr),
            "{arm}: the save body has no literal expression to replace — \
             the wire shape moved and this probe was about to pass vacuously"
        );
        let text = format!(
            "{header}\n{}",
            serde_json::to_string(&mutated).expect("re-serializing")
        );
        let err = pncad::document::load(&text, tol)
            .err()
            .unwrap_or_else(|| panic!("{arm}: an ill-dimensioned save file must refuse"));
        assert_eq!(
            persist_error_tag(&err),
            "unreadable",
            "{arm}: the load path's dimension refusal has changed class \
             (#694). It is neither a literal-value refusal nor the \
             quantity boundary's operator check — decide which typed \
             class it raises, and say so on both Python classes' docs, \
             before updating this pin"
        );
    }
}

/// Replaces the first single-key `Literal` object found in a
/// depth-first walk. Returns whether one was found — a probe that
/// silently replaced nothing would assert nothing.
#[cfg(test)]
fn replace_first_literal(value: &mut serde_json::Value, with: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => {
            if map.len() == 1 && map.contains_key("Literal") {
                *value = with.clone();
                return true;
            }
            map.values_mut().any(|v| replace_first_literal(v, with))
        }
        serde_json::Value::Array(items) => items.iter_mut().any(|v| replace_first_literal(v, with)),
        _ => false,
    }
}

/// LIB-DOORS F1: a load refusal's tag, exercised through the real
/// door (the exhaustive match itself is the drift alarm; this pins
/// two tags' spellings against the wire).
#[test]
fn persist_error_tags_are_stable() {
    let header =
        pncad::document::load("not a header", Tol::witness()).expect_err("garbage refuses");
    assert_eq!(persist_error_tag(&header), "header_id");
    let unreadable = pncad::document::load(
        "id: 00000000000000000000000000000000\n{\"snapshot\": {\"no_such_field\": 1}}",
        Tol::witness(),
    )
    .expect_err("a body this build cannot read refuses");
    assert_eq!(persist_error_tag(&unreadable), "unreadable");
}

/// The persistence door's four NESTED arms, by construction: each
/// wraps a refusal of another layer, and the word that rides out on
/// `inner_variant` is the inner refusal's own.
///
/// Constructed rather than driven through `load`, and the reason is
/// per arm:
///
/// * `profile_program` — a `ProgramFault` is a profile-program
///   structure fault. A file carrying one is reachable, but building
///   the tampered bytes means hand-assembling a lattice-violating
///   step order, which pins the WIRE shape rather than the tag.
/// * `distribution` — the same fault the edit door refuses, so a
///   parsed file that reaches it has to smuggle a distribution the
///   authoring doors cannot mint.
/// * `snapshot` — reachable from Python (`tests/test_document.py`
///   drives one), pinned here for the arms a tamper cannot select.
/// * `edit_replay` — needs a save file whose LOG replays into a
///   refusal, and the save door verifies the log symmetrically, so
///   the file has to be assembled by hand.
///
/// What each pins is the tag the exhaustive map mints, which is what
/// `inner_variant` carries; the projection itself is one positional
/// tuple over the same arms, so an arm reaching Python unprojected is
/// a compile error, not a missing test.
#[test]
fn the_persist_doors_nested_arms_carry_their_own_word() {
    use crate::tags::{
        distribution_fault_tag, edit_error_tag, program_fault_tag, snapshot_error_tag,
    };
    use pncad::document::{
        DistributionFault, DistributionField, EditError, PersistError, ProgramFault, RecipeNodeId,
        SnapshotError,
    };

    let program = ProgramFault::Lattice {
        loop_: 0,
        step: 1,
        state: pncad::profile::TipState::Entry,
        verb: None,
    };
    assert_eq!(program_fault_tag(&program), "lattice");

    let distribution = DistributionFault::NonFinite {
        field: DistributionField::Sigma,
    };
    assert_eq!(distribution_fault_tag(&distribution), "non_finite");

    let snapshot = SnapshotError::OrderMismatch;
    assert_eq!(snapshot_error_tag(&snapshot), "order_mismatch");

    let replayed = EditError::UnknownNode {
        id: RecipeNodeId(7),
    };
    let carrier = PersistError::EditReplay {
        index: 3,
        error: replayed.clone(),
    };
    assert_eq!(persist_error_tag(&carrier), "edit_replay");
    assert_eq!(edit_error_tag(&replayed), "unknown_node");
}

/// The frame door's `band` arm, by construction: it is the one arm no
/// Python door can reach.
///
/// Every `Frame` constructor derives its band from the process
/// tolerance witness, whose invariant is ε finite and strictly
/// positive with K > 1, so `Band::linear` cannot fail there and no
/// argument a Python caller can pass changes that. The three
/// `BandError` arms are therefore pinned here, with the payload each
/// carries: which threshold (`field`), the rejected number (`value`),
/// and the attempted pair a band could not be formed from.
#[test]
fn the_frame_doors_band_arm_is_construction_only() {
    use crate::tags::{band_error_tag, band_field_tag, frame_error_tag};
    use pncad::geom_core::{BandError, BandField, FrameError};

    let invalid = FrameError::Band(BandError::InvalidValue {
        field: BandField::Escalate,
        value: f64::INFINITY,
    });
    assert_eq!(frame_error_tag(&invalid), "band");
    let FrameError::Band(inner) = invalid else {
        panic!("the arm just built is the band arm")
    };
    assert_eq!(band_error_tag(&inner), "invalid_value");
    assert_eq!(band_field_tag(&BandField::Escalate), "escalate");
    assert_eq!(band_field_tag(&BandField::Zero), "zero");

    assert_eq!(
        band_error_tag(&BandError::InvalidLeverArm { value: 0.0 }),
        "invalid_lever_arm"
    );
    assert_eq!(
        band_error_tag(&BandError::Empty {
            zero: 1.0,
            escalate: 1.0,
        }),
        "empty"
    );
}

/// The STL writers' four arms, by construction: none is reachable
/// from Python.
///
/// `Mesh.to_stl_ascii` and `Mesh.to_stl_binary` write into a `Vec<u8>`
/// and tessellate their own mesh, so `io` has no failing sink,
/// `degenerate_triangle` and `index_out_of_range` need a mesh that
/// broke its own contract, and `too_many_triangles` needs more than
/// `u32::MAX` facets. The two OPTION refusals are the reachable half
/// and are driven through the real doors in `tests/test_mesh.py`.
#[test]
fn the_stl_writers_arms_are_construction_only() {
    use crate::tags::stl_error_tag;
    use pncad::stl::StlError;

    assert_eq!(
        stl_error_tag(&StlError::DegenerateTriangle {
            triangle: [0, 1, 2]
        }),
        "degenerate_triangle"
    );
    assert_eq!(
        stl_error_tag(&StlError::IndexOutOfRange { index: 9 }),
        "index_out_of_range"
    );
    assert_eq!(
        stl_error_tag(&StlError::TooManyTriangles { count: 1 << 33 }),
        "too_many_triangles"
    );
    assert_eq!(
        stl_error_tag(&StlError::Io(std::io::Error::other("sink"))),
        "io"
    );
}

/// The shell node's refusal tags, exercised by CONSTRUCTION for every
/// arm buildable without geometry: the op family at its f64 witness,
/// the mis-kinded open name, the lane refusal. `shell_open_resolve`
/// carries a `ResolveError`, whose constructors are the document
/// layer's own, so its spelling is driven through a real document in
/// `tests/test_shell.py` rather than minted here.
#[test]
fn shell_refusal_tags_are_stable() {
    use crate::tags::node_error_tag;
    use pncad::document::{NodeErrorKind, RecipeNodeId};
    use pncad::prelude::StableName;
    use pncad::select::{EntityKind, RoleSeg};
    use pncad::topo::ShellError;
    let op = NodeErrorKind::Shell(Box::new(ShellError::Thickness { thickness: -0.5 }));
    assert_eq!(node_error_tag(&op), "shell");
    let kind = NodeErrorKind::ShellOpenKind {
        name: Box::new(StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId(0),
            path: vec![RoleSeg::OutputBody],
        }),
        found: EntityKind::Edge,
    };
    assert_eq!(node_error_tag(&kind), "shell_open_kind");
    let lane = NodeErrorKind::ShellLaneUnsupported { lane: "Dual" };
    assert_eq!(node_error_tag(&lane), "shell_lane_unsupported");
}

/// **The two words a refusal puts on the wire, together.** The carrier
/// says which door refused; the inner arm says what the refusal that
/// door holds actually was.
///
/// Every row here is a PAIR, because the pair is the contract: the
/// carrier word is unchanged by this map's existence (a caller
/// branching on `revolve` still gets `revolve`), and the second word
/// is the payload's own discriminant. The `None` rows are the other
/// half of it and are not filler — an arm with no inner refusal, and
/// an arm whose word is ALREADY the payload's, both answer `None`,
/// and a change that started projecting either would move a shipped
/// attribute.
///
/// Arms whose payload needs real geometry are covered by the
/// exhaustive match alone, which is the alarm that matters: no map in
/// `crate::tags` has a wildcard, so deleting a kernel arm stops this
/// crate compiling rather than quietly dropping its word.
#[test]
fn inner_arm_tags_are_stable() {
    use crate::tags::{node_error_tag, node_inner_kind_tag};
    use pncad::document::{NodeErrorKind, PlacementRuleFault};
    use pncad::profile::ProfileError;
    use pncad::sweep::blend::{BlendError, BlendKind};
    use pncad::sweep::{ExtrudeError, RevolveError, TubeError};
    use pncad::topo::{ShellError, TransformError};

    let pair = |kind: &NodeErrorKind| (node_error_tag(kind), node_inner_kind_tag(kind));

    assert_eq!(
        pair(&NodeErrorKind::Revolve(RevolveError::DegenerateAxis)),
        ("revolve", Some("degenerate_axis"))
    );
    assert_eq!(
        pair(&NodeErrorKind::Tube(Box::new(TubeError::NonUnitAxis))),
        ("tube", Some("non_unit_axis"))
    );
    assert_eq!(
        pair(&NodeErrorKind::Extrude(ExtrudeError::ObliqueExtrusion)),
        ("extrude", Some("oblique_extrusion"))
    );
    assert_eq!(
        pair(&NodeErrorKind::Transform(TransformError::NurbsPlaceholder)),
        ("transform", Some("nurbs_placeholder"))
    );
    assert_eq!(
        pair(&NodeErrorKind::Shell(Box::new(ShellError::Thickness {
            thickness: -0.5
        }))),
        ("shell", Some("thickness"))
    );
    assert_eq!(
        pair(&NodeErrorKind::Profile(ProfileError::EmptyProfile)),
        ("profile", Some("empty_profile"))
    );
    // The blend's carrier word is the VERB, so the two words here are
    // "which blend" and "what it refused about" — the clearest case
    // for keeping them apart.
    assert_eq!(
        pair(&NodeErrorKind::Blend {
            verb: BlendKind::Chamfer,
            error: BlendError::NonpositiveSize { size: -1.0 },
        }),
        ("chamfer", Some("nonpositive_size"))
    );

    // No inner refusal: the payload is a pair of numbers.
    assert_eq!(
        pair(&NodeErrorKind::ToleranceConflict {
            document_eps: 1e-9,
            process_eps: 1e-12,
        }),
        ("tolerance_conflict", None)
    );
    // No payload at all.
    assert_eq!(
        pair(&NodeErrorKind::UnschedulableCycle),
        ("unschedulable_cycle", None)
    );
    // The word is already the fault's, under the carrier's own name:
    // `kind` IS the inner discriminant here, and projecting it twice
    // would say the same thing in two places.
    assert_eq!(
        pair(&NodeErrorKind::PlacementRule(
            PlacementRuleFault::CountSpelling
        )),
        ("placement_rule_mismatch", None)
    );
}

/// The same pair at the edit door: `variant` and `inner_variant`.
#[test]
fn edit_inner_variant_tags_are_stable() {
    use crate::tags::{edit_error_tag, edit_inner_variant_tag};
    use pncad::document::{
        Distribution, EditError, MetaVersionError, ParamName, RecipeNodeId, RootFault,
    };
    use pncad::prelude::StableName;
    use pncad::select::{EntityKind, RoleSeg};

    let pair = |err: &EditError| (edit_error_tag(err), edit_inner_variant_tag(err));

    let fault = Distribution::Normal { sigma: 0.0 }
        .check()
        .expect_err("a zero sigma breaks an E2 invariant");
    assert_eq!(
        pair(&EditError::InvalidDistribution {
            name: ParamName::new("bore"),
            fault,
        }),
        ("invalid_distribution", Some("sigma_not_positive"))
    );
    assert_eq!(
        pair(&EditError::EmptyWitnessBulk),
        ("empty_witness_bulk", None)
    );
    // The shape refusal under the metadata arm: which of the three
    // ways the D7 producer convention was broken.
    assert_eq!(
        pair(&EditError::MetaUnversioned {
            name: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(7),
                path: vec![RoleSeg::OutputBody],
            },
            key: "fit".to_owned(),
            error: MetaVersionError::VersionNotInt,
        }),
        ("meta_unversioned", Some("version_not_int"))
    );
    // `Roots` reads its word off the fault already, the way
    // `PlacementRule` does one carrier over.
    assert_eq!(
        pair(&EditError::Roots(RootFault::Duplicate {
            root: RecipeNodeId(1)
        })),
        ("root_duplicate", None)
    );
}

/// **Every `EditError` arm's payload, constructed and read.**
///
/// The arm table, executable. `crate::edit_payload::edit_payload` is
/// the projection Python reads its attributes off, and this pin says
/// what each of the 58 arms puts on the wire: the exact set of
/// attributes it CARRIES, in publication order, with the rest `None`.
///
/// It is here rather than in `tests/*.py` because most of these arms
/// have no Python door — the bound `DocEdit` surface is ten verbs, and
/// a rebind, a witness, an appearance write or an expression-path edit
/// is not among them. A rename or a re-slotting of any arm's payload
/// is a breaking change to the bindings whether or not a Python row
/// can provoke it, so it is pinned where it can be provoked: by
/// construction, on the row with no interpreter.
///
/// The pin is TOTAL over the enum: all 58 arms are built here, so an
/// arm whose projection is dropped shows up as a changed set rather
/// than as an absence nobody counted. Totality of the PROJECTION is a
/// different guarantee and a stronger one: `edit_payload`'s match is
/// exhaustive with no wildcard, so an arm that reached Python
/// unprojected would not compile.
#[test]
fn every_edit_arm_projects_the_payload_it_carries() {
    use crate::edit_payload::edit_payload;
    use pncad::document::{
        AttrKind, Axis3, ContentPin, Dimension, DimensionError, Distribution, DocParamValue,
        EditError as E, ExprPath, Frame, MeasureNodeFault, MetaVersionError, ParamName,
        RecipeNodeId, RootFault, SlotId,
    };
    use pncad::prelude::StableName;
    use pncad::select::{EntityKind, RoleSeg};

    let id = |n: u64| RecipeNodeId(n);
    let param = || ParamName::new("bore");
    let named = || StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(7),
        path: vec![RoleSeg::OutputBody],
    };
    let carries = |err: &E, want: &[&str]| {
        assert_eq!(
            edit_payload(err).present(),
            want,
            "the payload `{}` puts on the wire has moved",
            crate::tags::edit_error_tag(err)
        );
    };

    // ---- the node roles ----
    carries(&E::UnknownNode { id: id(1) }, &["node"]);
    carries(&E::WouldCycle { at: id(1) }, &["node"]);
    carries(&E::ReadSiteMissingNode { at: id(1) }, &["node"]);
    carries(&E::SetMembersOnNonList { node: id(1) }, &["node"]);
    carries(&E::WitnessOnNonSketch { node: id(1) }, &["node"]);
    carries(&E::DuplicateWitnessEntry { node: id(1) }, &["node"]);
    carries(&E::PlacementOnNonInstance { node: id(1) }, &["node"]);
    carries(&E::PlacementRuleMismatch { node: id(1) }, &["node"]);
    carries(&E::EmptyPlacementList { node: id(1) }, &["node"]);
    carries(&E::NonFinitePlacement { node: id(1) }, &["node"]);
    carries(&E::NonFiniteAlignment { node: id(1) }, &["node"]);
    carries(&E::UpdateOnNonInstance { node: id(1) }, &["node"]);
    carries(&E::UnresolvedInput { input: id(2) }, &["input"]);
    carries(
        &E::DuplicateInput {
            node: id(1),
            input: id(2),
        },
        &["node", "input"],
    );
    carries(
        &E::DeclareInputNotDeclare {
            node: id(1),
            input: id(2),
        },
        &["node", "input"],
    );
    carries(
        &E::AssertionTarget {
            node: id(1),
            measure: id(2),
        },
        &["node", "input"],
    );
    carries(
        &E::DeleteWouldDangle {
            id: id(1),
            referenced_by: id(2),
        },
        &["node", "referenced_by"],
    );

    // The two-node arms answer with the ids they were given, not with
    // the first id twice: the roles are what a caller acts on.
    let dangle = E::DeleteWouldDangle {
        id: id(4),
        referenced_by: id(9),
    };
    let payload = edit_payload(&dangle);
    assert_eq!(payload.node, Some(id(4)));
    assert_eq!(payload.referenced_by, Some(id(9)));

    // ---- slots and dimensions ----
    carries(
        &E::UnknownSlot {
            id: id(1),
            slot: SlotId::Count,
        },
        &["node", "slot"],
    );
    carries(
        &E::SlotDimensionMismatch {
            slot: SlotId::Distance,
            expected: Dimension::Length,
            found: Dimension::Angle,
        },
        &["slot", "expected", "found"],
    );
    carries(
        &E::StructuralSlotNeedsStructuralEdit {
            slot: SlotId::Count,
        },
        &["slot"],
    );
    carries(
        &E::NotStructuralSlot {
            slot: SlotId::Radius,
        },
        &["slot"],
    );
    carries(
        &E::AssertionDimension {
            node: id(1),
            measure: id(2),
            measured: Dimension::Length,
            bound: Dimension::Angle,
        },
        &["node", "input", "expected", "found"],
    );

    // `expected`/`found` are the DIMENSION pair under every spelling
    // the kernel gives them, and they are tag words rather than prose.
    let mismatch = E::SlotDimensionMismatch {
        slot: SlotId::Origin(Axis3::Y),
        expected: Dimension::Length,
        found: Dimension::Count,
    };
    let payload = edit_payload(&mismatch);
    assert_eq!(payload.slot, Some("origin_y"));
    assert_eq!(payload.expected, Some("length"));
    assert_eq!(payload.found, Some("count"));

    // ---- document parameters ----
    carries(
        &E::UnknownPayloadParam {
            name: param(),
            node: id(1),
        },
        &["node", "param"],
    );
    carries(
        &E::PayloadParamDimensionMismatch {
            name: param(),
            node: id(1),
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        },
        &["node", "param", "expected", "found"],
    );
    carries(
        &E::UnknownDocParam {
            name: param(),
            node: id(1),
            slot: SlotId::Count,
        },
        &["node", "slot", "param"],
    );
    carries(
        &E::DocParamDimensionMismatch {
            name: param(),
            node: id(1),
            slot: SlotId::Count,
            declared: Dimension::Count,
            referenced: Dimension::Length,
        },
        &["node", "slot", "param", "expected", "found"],
    );
    carries(
        &E::ContinuousParamCannotBeCount { name: param() },
        &["param"],
    );
    carries(&E::DocParamNotDeclared { name: param() }, &["param"]);
    carries(&E::NonFiniteDocParam { name: param() }, &["param"]);
    carries(
        &E::DocParamValueKindMismatch {
            name: param(),
            declared: Dimension::Length,
            offered: DocParamValue::Count(3),
        },
        &["param", "expected", "offered"],
    );

    // ---- the list-shape arms ----
    carries(
        &E::TooFewMembers {
            node: id(1),
            found: 1,
        },
        &["node", "count"],
    );
    carries(
        &E::RepeatedDesignation {
            node: id(1),
            first: 0,
            again: 3,
        },
        &["node", "first", "again"],
    );
    // `found` on a short list is a COUNT and takes the `count`
    // attribute, so it never lands where a dimension word would.
    let short = E::TooFewMembers {
        node: id(1),
        found: 1,
    };
    let payload = edit_payload(&short);
    assert_eq!(payload.count, Some(1));
    assert_eq!(payload.found, None);

    // ---- names, kinds and appearance ----
    for arm in [
        E::DeclareNamesMissingNode { name: named() },
        E::RebindTargetMissingNode { name: named() },
        E::RebindUnknownName { name: named() },
        E::RebindIdentity { name: named() },
        E::RebindNoReferences { name: named() },
        E::NameUnresolvedInEvaluation { name: named() },
        E::AppearanceWrongKind { name: named() },
        E::AppearanceNamesMissingNode { name: named() },
    ] {
        carries(&arm, &["name"]);
    }
    carries(
        &E::RebindKindMismatch {
            from: EntityKind::Face,
            to: EntityKind::Edge,
        },
        &["from_kind", "to_kind"],
    );
    carries(
        &E::RebindAppearanceCollision {
            name: named(),
            kind: AttrKind::Color,
        },
        &["name", "kind"],
    );
    carries(
        &E::AppearanceNotSet {
            name: named(),
            kind: AttrKind::Visibility,
        },
        &["name", "kind"],
    );
    let collision = E::RebindAppearanceCollision {
        name: named(),
        kind: AttrKind::Label,
    };
    assert_eq!(edit_payload(&collision).kind, Some("label"));

    // ---- appearance metadata ----
    carries(
        &E::MetaNotSet {
            name: named(),
            key: "fit".to_owned(),
        },
        &["name", "key"],
    );
    carries(
        &E::RebindMetadataCollision {
            name: named(),
            key: "fit".to_owned(),
        },
        &["name", "key"],
    );
    // The shape refusal is the arm's third field and rides on
    // `inner_variant`, not on the payload: the projection is
    // `MetaNotSet`'s, the same `name` and `key`.
    carries(
        &E::MetaUnversioned {
            name: named(),
            key: "fit".to_owned(),
            error: MetaVersionError::MissingVersion,
        },
        &["name", "key"],
    );
    carries(
        &E::MetaNonFinite {
            name: named(),
            key: "fit".to_owned(),
            path: "clearance.lo".to_owned(),
        },
        &["name", "key", "value_path"],
    );

    // ---- scalars and addresses ----
    carries(&E::InvalidTolerance { value: -1.0 }, &["value"]);
    carries(
        &E::ImproperPlacement {
            node: id(1),
            determinant: -1.0,
        },
        &["node", "determinant"],
    );
    carries(
        &E::PinUnchanged {
            node: id(1),
            pin: ContentPin([0u8; 32]),
        },
        &["node", "pin"],
    );
    // An expression address decomposes into the two attributes that
    // already name its halves plus the child indices below the slot;
    // the metadata float's address is a `str` under `value_path`, a
    // different address in a different tree.
    let off_tree = E::PathOffTree {
        path: ExprPath {
            node: id(5),
            slot: SlotId::Distance,
            path: vec![0, 1],
        },
    };
    carries(&off_tree, &["node", "slot", "path"]);
    let payload = edit_payload(&off_tree);
    assert_eq!(payload.node, Some(id(5)));
    assert_eq!(payload.slot, Some("distance"));
    assert_eq!(payload.path, Some(&[0u8, 1][..]));

    // ---- the product-root invariants ----
    carries(&E::Roots(RootFault::NotLive { root: id(1) }), &["node"]);
    carries(&E::Roots(RootFault::Duplicate { root: id(1) }), &["node"]);
    carries(&E::Roots(RootFault::Uncovered { node: id(1) }), &["node"]);
    carries(
        &E::Roots(RootFault::Ancestor {
            ancestor: id(1),
            descendant: id(2),
        }),
        &["node", "referenced_by"],
    );

    // ---- the arms that carry a nested refusal, and the empty one ----
    //
    // `inner_variant` names the arm of the refusal each holds and the
    // fields INSIDE it stay on that type's own door, so what these
    // project is the carrier's own payload and nothing more. The one
    // with no payload at all is the whole of the empty case.
    carries(
        &E::ProfileProgramRefused {
            node: id(1),
            refusal: pncad::document::ProgramRefusal::Validate(
                pncad::profile::ProfileError::EmptyProfile,
            ),
        },
        &["node"],
    );
    carries(
        &E::MeasureMalformed {
            node: id(1),
            fault: MeasureNodeFault::RefIndexOutOfRange {
                verb: "distance",
                index: 5,
                refs: 0,
            },
        },
        &["node"],
    );
    carries(
        &E::Dimension(DimensionError::Mismatch {
            op: "+",
            left: Dimension::Length,
            right: Dimension::Angle,
        }),
        &[],
    );
    carries(
        &E::InvalidDistribution {
            name: param(),
            fault: Distribution::Normal { sigma: 0.0 }
                .check()
                .expect_err("a zero sigma breaks an E2 invariant"),
        },
        &["param"],
    );
    let band = pncad::geom_core::Band::linear(Tol::witness()).expect("the witness band");
    let axis = Frame::rotate_then_translate([0.0, 0.0, 0.0], 1.0, [0.0, 0.0, 0.0], band)
        .expect_err("a zero axis has no definite direction");
    carries(&E::from(axis), &[]);
    carries(&E::EmptyWitnessBulk, &[]);
}

/// The workspace tags `Doc()` publishes. `randomness_unavailable` is
/// the one `pncad.pyi` names, and it is minted here rather than
/// provoked: `getrandom::fill` has no injection seam (see
/// `crate::identity::interactive`), so the reachable-arm door cannot
/// be driven from a test. `Io` is driven through a real workspace
/// door — `Workspace::open`, which is NOT the door that raises
/// `IdentityError`, and that is the point: the map answers about the
/// VALUE, so it is exercisable wherever a `WorkspaceError` can be
/// produced rather than only where this one is raised.
#[test]
fn workspace_error_tags_are_stable() {
    use pncad::workspace::{Workspace, WorkspaceError};

    assert_eq!(
        workspace_error_tag(&WorkspaceError::RandomnessUnavailable {
            message: "entropy source refused".to_string(),
        }),
        "randomness_unavailable"
    );
    let missing = Workspace::open(Path::new("/nonexistent/pncad-workspace"))
        .expect_err("a directory that is not there refuses");
    assert_eq!(workspace_error_tag(&missing), "io");

    // The save door's own two arms, constructed: neither is reachable
    // from a store that holds nothing, and both are Python-visible.
    assert_eq!(
        workspace_error_tag(&WorkspaceError::SaveWouldDuplicateId {
            id: pncad::document::DocumentId::derive("tagged"),
            existing: std::path::PathBuf::from("/store/a.pncad"),
            requested: std::path::PathBuf::from("/store/b.pncad"),
        }),
        "save_would_duplicate_id"
    );
    assert_eq!(
        workspace_error_tag(&WorkspaceError::SaveTargetNotInStore {
            path: std::path::PathBuf::from("/elsewhere/a.pncad"),
        }),
        "save_target_not_in_store"
    );
}

/// The STEP importer's tags. Every arm of this enum is reachable
/// through `import_step`, so unlike the workspace map there is no
/// single-reachable-arm caveat to make: the exhaustive match is the
/// drift alarm and these two pin its spelling against the wire. The
/// first goes through the real door; the second is minted, because
/// reaching `NothingToImport` needs a well-formed Part 21 file and
/// that is a fixture, not a literal.
#[test]
fn step_import_error_tags_are_stable() {
    let opts = pncad::step_import::ImportOptions::default();
    let garbage = pncad::step_import::import_step("not a step file", &opts, Tol::witness())
        .expect_err("garbage refuses");
    assert_eq!(step_import_error_tag(&garbage), "syntax");
    assert_eq!(
        step_import_error_tag(&pncad::step_import::StepImportError::NothingToImport),
        "nothing_to_import"
    );
}

/// The recognition arm's PAYLOAD tag, beside the arm's own.
///
/// Minted rather than reached, and the reason is the arm: firing
/// `recognition_ambiguous` needs a file with a multi-bound curved face
/// on a NURBS surface whose estimator is ill-conditioned at the
/// declared tolerance, which is a fixture and `step-import`'s own
/// suite's. What this pins is the wire spelling of both words and the
/// fact that they arrive TOGETHER — the carrier's word naming the
/// condition, the payload's naming which estimator declined.
#[test]
fn promoted_kind_tags_are_stable() {
    let ambiguous = |kind| pncad::step_import::StepImportError::RecognitionAmbiguous {
        id: 104,
        surface: 105,
        kind,
        margin: 1e-9,
    };
    for (kind, word) in [
        (pncad::step_import::PromotedKind::Plane, "plane"),
        (pncad::step_import::PromotedKind::Cylinder, "cylinder"),
    ] {
        assert_eq!(promoted_kind_tag(&kind), word);
        assert_eq!(
            step_import_error_tag(&ambiguous(kind)),
            "recognition_ambiguous"
        );
    }
}

/// The SUCCESS side's two value discriminants — what a report's rows
/// say, as opposed to what a refusal says.
///
/// Minted rather than reached, and for a sharper reason than the
/// refusal above: four of these five normalizations need a file
/// exercising a specific Open CASCADE export shape (an edge-free
/// sphere, a degenerate apex, a full-period torus face, a seamless
/// band) and those are `step-import`'s own corpus fixtures. What this
/// pins is the wire spelling every row carries, and that
/// `surface_promotion` does NOT fold its analytic kind into the word:
/// which kind certified is the payload beside it, at
/// `promoted_kind_tag`'s two words, so a caller reading "a patch was
/// promoted" reads one word whichever kind it was.
#[test]
fn import_report_row_tags_are_stable() {
    use pncad::step_import::{NormalizationKind, PromotedCurveKind, PromotedKind};
    for (kind, word) in [
        (NormalizationKind::EdgeFreeSphere, "edge_free_sphere"),
        (
            NormalizationKind::DegenerateApexCone,
            "degenerate_apex_cone",
        ),
        (NormalizationKind::FullPeriodTorus, "full_period_torus"),
        (
            NormalizationKind::SeamlessPeriodicBand,
            "seamless_periodic_band",
        ),
    ] {
        assert_eq!(normalization_kind_tag(&kind), word);
    }
    for kind in [PromotedKind::Plane, PromotedKind::Cylinder] {
        assert_eq!(
            normalization_kind_tag(&NormalizationKind::SurfacePromotion {
                to: kind,
                residual: 1e-11,
            }),
            "surface_promotion",
            "the arm's word is the normalization, not the kind"
        );
    }
    assert_eq!(
        promoted_curve_kind_tag(&PromotedCurveKind::Circle),
        "circle"
    );
}

#[test]
fn path_error_tags_are_stable() {
    use pncad::prelude::{Open, Start, circle, p2, polygon};

    let zero = circle(p2(0.0, 0.0), 0.0, Tol::witness()).expect_err("a zero radius refuses");
    assert_eq!(path_error_tag(&zero), "nonpositive_circle_radius");

    let tangent = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0), Tol::witness())
        .expect("a leg east")
        .angle(0.0, Tol::witness())
        .expect_err("a corner tangent to its incoming leg refuses");
    assert_eq!(path_error_tag(&tangent), "junction_tangent");

    // The collinear tangent-arc close: carrier identity is no longer a
    // refusal (Ev, in-chat, 2026-09-02 — every zero-turn joint is a
    // declared tangent joint). What refuses is the GEOMETRY: `Start` is
    // collinear with the declared departure and BEHIND it, so the
    // tangent-chord angle is pi, the bulge unbounded, and no arc spans
    // the chord.
    let degenerate = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0), Tol::witness())
        .expect("a leg east")
        .tangent()
        .tangent_arc_to(Start, Tol::witness())
        .expect_err("no arc spans a chord behind the departure");
    assert_eq!(path_error_tag(&degenerate), "degenerate_arc_chord");

    // The envelope: one tag for the refusal, and one per entry for the
    // corner's own reason, so a caller branches on the reason without
    // parsing the sentence. A straight pair derives one corner and the
    // radius outruns the arrival leg, so the entry is the anchor fit.
    let overrun = Open
        .at(p2(0.0, 0.0))
        .toward(1.0, 0.0, Tol::witness())
        .expect("the incoming ray runs +x")
        .fillet(2.5, Tol::witness())
        .expect("positive radius")
        .toward(0.0, 1.0, Tol::witness())
        .expect("the arrival side runs +y")
        .to(p2(3.0, 2.0), Tol::witness())
        .expect_err("the setback outruns the arrival leg");
    assert_eq!(path_error_tag(&overrun), "no_corner_of_pair");
    let pncad::profile::PathError::NoCornerOfPair { corners, .. } = &overrun else {
        panic!("expected the envelope, got {overrun:?}")
    };
    let entries: Vec<&'static str> = corners
        .iter()
        .map(|c| crate::tags::corner_reason_tag(&c.reason))
        .collect();
    assert_eq!(entries, ["anchor_outside_trimmed_extent"]);

    // The whole-table door's count precondition: the lattice's own
    // verbs take one vertex at a time and have no count to gate, so
    // `polygon` is the only place this arm is reachable from.
    let too_few = polygon::<f64>(&[(0.0, 0.0), (1.0, 0.0)], Tol::witness())
        .expect_err("two vertices name no polygon");
    assert_eq!(path_error_tag(&too_few), "polygon_too_few_vertices");
}

/// The prose rule's guard, checked against what it actually guards
/// against: real kernel refusals rendered both ways.
///
/// `crate::py::typed_err` asserts [`reads_as_prose`] on every raise,
/// so this test is the half that proves the predicate can go RED — a
/// guard verified only by a green suite is not verified. The `Debug`
/// renderings below are exactly what the crate used to send to Python
/// at the tessellate and select doors.
#[test]
fn the_prose_rule_separates_a_display_from_a_debug_dump() {
    use pncad::prelude::{circle, p2};

    let zero = circle(p2(0.0, 0.0), 0.0, Tol::witness()).expect_err("a zero radius refuses");
    assert!(reads_as_prose(&zero.to_string()));
    assert!(!reads_as_prose(&format!("{zero:?}")));

    let entropy = pncad::workspace::WorkspaceError::RandomnessUnavailable {
        message: "entropy source refused".to_string(),
    };
    assert!(reads_as_prose(&entropy.to_string()));
    assert!(!reads_as_prose(&format!("{entropy:?}")));

    // The second fingerprint: a fieldless variant renders as one bare
    // word, which no sentence is.
    assert!(!reads_as_prose("SeamRetrimsArcFirstSide"));
    // And the shapes prose legitimately carries: a quoted user string
    // (`Debug` on a `&str`, which the id doors use for its escaping),
    // and a sentence that opens on a capital.
    assert!(reads_as_prose(
        "not a document id: \"nope\" — an id is 32 hex digits"
    ));
    assert!(reads_as_prose("Tessellate refused"));
}

/// Read one flat `key = "value"` TOML table, selected by its exact
/// header line.
///
/// A deliberately tiny scanner in the LB13 self-scanning style — the
/// alternative is a `toml` dev-dependency this crate does not
/// otherwise need. Its blind spots, stated rather than hidden: it
/// understands only flat tables of quoted scalars (which is all a
/// `[lints.*]` table ever is), it does not follow `workspace = true`
/// inheritance, and it would silently return nothing for a header
/// that does not exist — which is exactly why the caller asserts the
/// workspace tables came back NON-EMPTY before comparing.
fn toml_table(source: &str, header: &str) -> BTreeMap<String, String> {
    let mut table = BTreeMap::new();
    let mut inside = false;
    for line in source.lines() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            inside = line == header;
            continue;
        }
        if !inside || line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            table.insert(
                key.trim().to_owned(),
                value.trim().trim_matches('"').to_owned(),
            );
        }
    }
    table
}

/// The crate's hand-restated `[lints]` MUST equal the workspace's,
/// minus exactly `unsafe_code`.
///
/// This crate cannot inherit `[workspace.lints]` (see the Cargo.toml
/// header: `unsafe_code = "forbid"` versus PyO3's macro-generated
/// `unsafe impl`), so the table is restated by hand — and this test
/// makes the equality an enforced invariant rather than a claim:
/// adding a lint to `[workspace.lints]` breaks
/// this crate's build until it is mirrored, LOUDLY, on the default
/// (no-Python) path hosted CI takes.
#[test]
fn crate_lints_match_the_workspace_minus_unsafe_code() {
    // `source::crate_dir`, not the baked path alone, for the reason
    // the tag-table guard below states at its own read: under a
    // nextest ARCHIVE replayed on another runner the compile-time
    // directory need not exist, and a guard that cannot find its
    // subject reds for the wrong reason. Converted alongside that
    // guard, since the helper arrived with it.
    let crate_dir = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"));
    let root_manifest = crate_dir.join("..").join("..").join("Cargo.toml");
    let root = std::fs::read_to_string(&root_manifest)
        .expect("the workspace root Cargo.toml is two levels above this crate");
    let mine =
        std::fs::read_to_string(crate_dir.join("Cargo.toml")).expect("this crate's own Cargo.toml");

    for (workspace_header, crate_header) in [
        ("[workspace.lints.rust]", "[lints.rust]"),
        ("[workspace.lints.clippy]", "[lints.clippy]"),
    ] {
        let mut expected = toml_table(&root, workspace_header);
        // The single sanctioned deviation, and the ONLY one.
        let removed = expected.remove("unsafe_code");
        assert!(
            !expected.is_empty(),
            "scanner found no lints under {workspace_header} — the header \
             moved or the format changed, so this guard was about to pass \
             vacuously"
        );
        if workspace_header.ends_with("rust]") {
            assert_eq!(
                removed.as_deref(),
                Some("forbid"),
                "the workspace is expected to FORBID unsafe_code; if that \
                 changed, this crate's exemption needs rethinking"
            );
        }

        let actual = toml_table(&mine, crate_header);
        assert_eq!(
            actual,
            expected,
            "{crate_header} has drifted from {workspace_header}.\n  \
             missing here: {:?}\n  unexpected here: {:?}",
            expected
                .iter()
                .filter(|(k, v)| actual.get(*k) != Some(v))
                .collect::<Vec<_>>(),
            actual
                .iter()
                .filter(|(k, v)| expected.get(*k) != Some(v))
                .collect::<Vec<_>>(),
        );
    }
}

// ---------------------------------------------------------------
// Document identity: the id a Python-authored document carries.
// ---------------------------------------------------------------

/// **Two Python-authored documents are two PARTS**: distinct ids, and
/// one workspace holds both.
///
/// The store's uniqueness invariant is keyed on the id, so a constant
/// id makes the second document unstorable beside the first — and per
/// the assembly model it is not a second part at all, because
/// `DocRef`/`ContentPin` references resolve by id. This test refuses
/// both halves at once: it fails on the ids if a constant comes back,
/// and it fails on `create` if the store ever stops enforcing what
/// the ids are for.
#[test]
fn two_python_authored_documents_are_two_parts_in_one_workspace() {
    let a = crate::identity::interactive(Tol::witness()).expect("OS entropy");
    let b = crate::identity::interactive(Tol::witness()).expect("OS entropy");
    assert_ne!(
        a.id(),
        b.id(),
        "two interactively authored documents share an id, so they are \
         one part and one workspace cannot hold both"
    );

    let dir = std::env::temp_dir().join(format!(
        "pncad-py-identity-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch workspace directory");

    let mut store = pncad::workspace::Workspace::open(&dir).expect("an empty workspace opens");
    let first = store
        .create(&a, Tol::witness())
        .expect("the first document writes");
    let second = store
        .create(&b, Tol::witness())
        .expect("the second document writes beside it");
    assert_ne!(first, second, "two parts, two files");
    assert_eq!(
        store.documents().len(),
        2,
        "both documents are in the store's id map"
    );

    // And the scan agrees from cold: the header ids are what the map
    // was built from, so a re-open is the store's own verdict.
    let reopened = pncad::workspace::Workspace::open(&dir).expect("the store rescans clean");
    assert_eq!(reopened.documents().len(), 2);

    std::fs::remove_dir_all(&dir).expect("cleanup");
}

/// The LABELLED spelling is deterministic — same label, same part —
/// which is what makes it the reproducible door and NOT the default.
#[test]
fn a_labelled_document_is_the_same_part_every_time() {
    assert_eq!(
        crate::identity::derived("plate-param", Tol::witness()).id(),
        crate::identity::derived("plate-param", Tol::witness()).id()
    );
    assert_ne!(
        crate::identity::derived("plate-param", Tol::witness()).id(),
        crate::identity::derived("bracket", Tol::witness()).id()
    );
}

/// The registry's two tag namespaces are pinned, and stated honestly:
/// this constructs every arm the curated surface can BUILD and asserts
/// its tag. ONE arm still carries a kernel internal with no public
/// constructor — [`ChecksError::Band`]'s `BandError` — so its tag is
/// covered by the exhaustive match alone, which is the real alarm
/// here: neither enum is `#[non_exhaustive]`, so a kernel arm added
/// without a tag stops this crate compiling. The shell door's refusal
/// behind `Escalated`/`Unsupported` is no longer one of them:
/// `pncad::document` carries the type, so both arms are built and the
/// refusal's own word is asserted beside them.
///
/// What the pin adds over the match is the STRINGS. A tag is the
/// branchable half of a typed refusal, so renaming one is a surface
/// break the compiler cannot see.
#[test]
fn check_registry_tags_are_stable() {
    use crate::tags::{check_evidence_tag, checks_error_tag, shell_classify_error_tag};
    use pncad::document::{CheckEvidence, ChecksError, RecipeNodeId, ShellClassifyError};

    assert_eq!(
        checks_error_tag(&ChecksError::Root {
            node: RecipeNodeId(3)
        }),
        "root_without_value"
    );
    assert_eq!(
        checks_error_tag(&ChecksError::Product {
            reason: "no body roots".into()
        }),
        "product_unavailable"
    );

    assert_eq!(
        check_evidence_tag(&CheckEvidence::Connectedness {
            actual: 2,
            expected: 1
        }),
        "connectedness"
    );
    assert_eq!(
        check_evidence_tag(&CheckEvidence::StaleExpectation { expected: 1 }),
        "stale_expectation"
    );
    assert_eq!(
        check_evidence_tag(&CheckEvidence::NotSeparated {
            other_root: RecipeNodeId(4),
            other_output: 0
        }),
        "not_separated"
    );
    assert_eq!(
        check_evidence_tag(&CheckEvidence::SeparationUnavailable {
            kind: pncad::topo::BooleanErrorKind::ClassificationInvariant,
            reason: "boxes refused".into()
        }),
        "separation_unavailable"
    );

    let refused = ShellClassifyError::ZeroVolume {
        shell: pncad::topo::ShellKey::default(),
    };
    assert_eq!(
        check_evidence_tag(&CheckEvidence::Escalated {
            source: refused.clone()
        }),
        "escalated"
    );
    assert_eq!(
        check_evidence_tag(&CheckEvidence::Unsupported {
            source: refused.clone()
        }),
        "unsupported"
    );
    assert_eq!(shell_classify_error_tag(&refused), "zero_volume");
}

/// **Every constructible `CheckEvidence` arm's payload, built and
/// read.**
///
/// The arm table, executable. `crate::check_payload::check_payload`
/// is the projection `CheckEvidence`'s five Python attributes are
/// read off, and this pin says what each arm puts on the wire: the
/// exact set it CARRIES, in publication order, with the rest `None`.
///
/// **All six arms are built here.** `Escalated` and `Unsupported`
/// hold the shell door's refusal, which `pncad::document` carries
/// beside the evidence that rides it, so a value can be named and the
/// table is total: each of those two carries the refusal's own word
/// beside the sentence it renders.
///
/// Three of the six are unreachable from Python entirely (the shell
/// door escalating on a gathered subject, the box builder refusing
/// over the whole product), so this is where their projection is
/// pinned at all: `tests/test_checks.py` reads the other three.
#[test]
fn every_check_evidence_arm_projects_the_payload_it_carries() {
    use crate::check_payload::check_payload;
    use crate::tags::check_evidence_tag;
    use pncad::document::{CheckEvidence as E, RecipeNodeId};

    let carries = |evidence: &E, want: &[&str]| {
        assert_eq!(
            check_payload(evidence).present(),
            want,
            "the payload `{}` puts on the wire has moved",
            check_evidence_tag(evidence)
        );
    };

    carries(
        &E::Connectedness {
            actual: 2,
            expected: 1,
        },
        &["actual", "expected"],
    );
    carries(&E::StaleExpectation { expected: 1 }, &["expected"]);
    carries(
        &E::NotSeparated {
            other_root: RecipeNodeId(4),
            other_output: 2,
        },
        &["other_root", "other_output"],
    );
    let unavailable = E::SeparationUnavailable {
        kind: pncad::topo::BooleanErrorKind::ClassificationInvariant,
        reason: "boxes refused".into(),
    };
    carries(&unavailable, &["reason"]);

    // The two arms that hold another door's refusal: its sentence and
    // its own word, which is the half a caller branches on.
    let refused = pncad::document::ShellClassifyError::ZeroVolume {
        shell: pncad::topo::ShellKey::default(),
    };
    carries(
        &E::Escalated {
            source: refused.clone(),
        },
        &["reason", "inner_variant"],
    );
    carries(
        &E::Unsupported {
            source: refused.clone(),
        },
        &["reason", "inner_variant"],
    );
    assert_eq!(
        check_payload(&E::Escalated { source: refused }).inner_variant,
        Some("zero_volume")
    );

    // The numbers and the sentence themselves, not just which fields
    // are set: the counterpart names the root it names, and the
    // separation arm's prose crosses as the kernel wrote it.
    let pair = check_payload(&E::NotSeparated {
        other_root: RecipeNodeId(4),
        other_output: 2,
    });
    assert_eq!(pair.other_root, Some(RecipeNodeId(4)));
    assert_eq!(pair.other_output, Some(2));
    assert_eq!(
        check_payload(&unavailable).reason.as_deref(),
        Some("boxes refused")
    );
}

/// **The tier-3′ census findings read as prose, and that is a KERNEL
/// rendering, not a binding one.**
///
/// `crate::py::typed_err` asserts every message it raises satisfies
/// [`reads_as_prose`], and every door in this crate obeys the rule the
/// assertion stands for — the binding never authors a `Debug` dump.
/// Three `ValidationError` arms once broke that rule from the OTHER
/// side, the kernel wording them out of `Debug`: `UndeclaredContact`
/// rendered its `CensusContact` as `{contact:?}`, `StaleContactDeclaration`
/// its `StaleDeclaration` the same way, and the `witness` the kernel
/// built with `format!("{p:?}")` carried a `Point3`'s field braces.
/// Each now renders through `Display`, so `Body::run_validator` raises
/// through `typed_err` like every other door and needs no exemption.
///
/// **What this row does and does not cover.** Both findings are built
/// here with a hand-written `witness`, so the assertion is that the
/// ARMS' format strings interpolate through `Display` — it says nothing
/// about `census::witness`, which no code path in this crate reaches.
/// The witness rendering is guarded in the kernel, by
/// `topo/tests/mate4a_ef_bound_rung.rs` (a `Debug` golden over the
/// whole finding list) and `topo/tests/review_mate9_r1_probes.rs`
/// (which reads coordinates out of the witness text). What this row
/// adds is that the check runs in the no-interpreter CI row, where the
/// Python suite cannot.
#[test]
fn the_census_findings_read_as_prose_by_this_crate_s_own_rule() {
    use pncad::topo::{CensusContact, StaleDeclaration, ValidationError};

    // Both arms are built here rather than by evaluating a document:
    // this row is about the RENDERING, and a default arena key is
    // enough to render one — nothing dereferences it.
    let census = ValidationError::UndeclaredContact {
        contact: CensusContact::VertexOnFace {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        },
        // The kernel builds this field as a coordinate triple
        // (`census::witness`), so it carries no braces of its own.
        witness: "(0.0, 0.0, 0.0)".to_owned(),
    };
    let stale = ValidationError::StaleContactDeclaration {
        declaration: StaleDeclaration::VertexOnFace {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        },
    };

    for finding in [&census, &stale] {
        let message = finding.to_string();
        assert!(
            reads_as_prose(&message),
            "a tier-3′ census finding must read as prose: it is raised \
             through `typed_err`, whose assertion is live in every \
             profile. Message: {message}"
        );
        assert!(
            !message.contains(" { "),
            "the struct-brace fingerprint is exactly what \
             `reads_as_prose` rejects, and a payload that regained a \
             `Debug` rendering is how it comes back: {message}"
        );
    }

    // The payload survives the rewording: an arena key still names
    // each entity, so the prose is a diagnosis a caller can act on
    // rather than a sentence that dropped its subject.
    let message = census.to_string();
    assert!(
        message.contains("vertex") && message.contains("(0.0, 0.0, 0.0)"),
        "the finding still names its entities and its witness: {message}"
    );
    assert!(
        message.contains("never blessed from discovery"),
        "the undeclared-contact recourse is the actionable half"
    );
}

/// **What one validator finding says, arm by arm** — including the
/// arms no Python door can produce.
///
/// `ValidationError` has seventy-one arms and Python reaches them
/// through four `Body` methods, so most of the enum is unreachable
/// from an authoring script: `census_unsupported` and
/// `census_lane_unsupported` want a carrier outside the certifiable
/// inventory or a scalar with no certified chart-overlap lane, and
/// the structural arms want a corrupt arena, which the public API
/// cannot mint. Those are exactly the arms whose projection the
/// Python suite cannot exercise, so they are constructed here and
/// read directly — the no-interpreter row, where a value class is
/// still a plain Rust struct.
///
/// `crates/pncad-py/tests/test_validate.py` is the other half: the
/// two arms a real document DOES reach, off a real refusal.
#[test]
fn every_validation_finding_carries_every_word_its_arm_has() {
    use crate::validation::{Finding, project};
    use pncad::topo::{CensusContact, CensusSubject, EntityId, ValidationError};

    // The arm whose subject is ONE entity: the recourse is that
    // carrier's, so the kind of carrier is the word a caller acts on.
    assert_eq!(
        project(&ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Edge(Default::default())),
        }),
        Finding {
            variant: "census_unsupported",
            subject_kind: Some("entity"),
            entity_kind: Some("edge"),
            contact_kind: None,
            stale_kind: None,
            ring_contact_kind: None,
        }
    );

    // The arm whose subject is a candidate CONTACT: the recourse is
    // the declaration protocol instead, and both sides are faces by
    // construction — so there is no entity kind to name, and `None`
    // says that rather than repeating "face" twice.
    assert_eq!(
        project(&ValidationError::CensusLaneUnsupported {
            subject: CensusSubject::FacePair(FaceKey::default(), FaceKey::default()),
        }),
        Finding {
            variant: "census_lane_unsupported",
            subject_kind: Some("face_pair"),
            entity_kind: None,
            contact_kind: None,
            stale_kind: None,
            ring_contact_kind: None,
        }
    );

    // The coincidence arm, whose payload is the branch the issue this
    // unit closes was raised about: declare-this versus
    // you-cannot-declare-this, off one refusal.
    assert_eq!(
        project(&ValidationError::UndeclaredContact {
            contact: CensusContact::EdgeFacePierce {
                edge: Default::default(),
                face: FaceKey::default(),
            },
            witness: "(0.0, 0.0, 0.0)".to_owned(),
        })
        .contact_kind,
        Some("edge_face_pierce")
    );

    // The one fieldless arm, and the shape of every arm that carries
    // no payload at all: the variant alone, five `None`s beside it,
    // so `getattr` never raises on a finding a caller did not expect.
    assert_eq!(
        project(&ValidationError::NegativeVolume),
        Finding {
            variant: "negative_volume",
            subject_kind: None,
            entity_kind: None,
            contact_kind: None,
            stale_kind: None,
            ring_contact_kind: None,
        }
    );
}

/// **Every `StaleDeclaration` arm's word, built and read.**
///
/// The arm table for `stale_kind`, executable. Each arm names the
/// GRANULARITY of the record the tier-3′ census could not confirm,
/// which is the recourse — withdraw or re-seat THAT record — so the
/// pin is one row per arm rather than a spot check.
///
/// **None of the four is reachable from Python**, and the reason is
/// the same for all four: a stale record is a declaration the
/// geometry stopped backing, and every door that hands Python a body
/// with declarations attached mints those declarations from the
/// geometry it is looking at (`Value.body`) or gates them before it
/// answers (`assemble`, whose gate refuses on exactly this census).
/// A Python caller cannot edit a `ContactRecords`, so it cannot part
/// a record from its witness; the kernel's own suites do it by
/// tampering with the record set directly. This is therefore where
/// the projection is pinned at all.
#[test]
fn every_stale_declaration_arm_projects_the_payload_it_carries() {
    use crate::validation::project;
    use pncad::topo::{StaleDeclaration, ValidationError};

    let word = |declaration: StaleDeclaration| {
        project(&ValidationError::StaleContactDeclaration { declaration }).stale_kind
    };

    assert_eq!(
        word(StaleDeclaration::VertexVertex {
            a: VertexKey::default(),
            b: VertexKey::default(),
        }),
        Some("vertex_vertex")
    );
    assert_eq!(
        word(StaleDeclaration::VertexOnFace {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        }),
        Some("vertex_on_face")
    );
    assert_eq!(
        word(StaleDeclaration::CurveLocus {
            face_a: FaceKey::default(),
            face_b: FaceKey::default(),
            witness: Default::default(),
        }),
        Some("curve_locus")
    );
    assert_eq!(
        word(StaleDeclaration::Patch {
            face_a: FaceKey::default(),
            face_b: FaceKey::default(),
        }),
        Some("patch")
    );

    // The word rides the arm that carries the record and no other:
    // the contradiction arm is the OTHER direction of the same
    // certification diff and carries a declaration that IS witnessed,
    // by counter-evidence.
    assert_eq!(project(&ValidationError::NegativeVolume).stale_kind, None);
}

/// **Every `RingContact` arm's word, built and read.**
///
/// The arm table for `ring_contact_kind`, executable. The three arms
/// are three different repairs — a shared position one vertex move
/// clears, a ring vertex standing on an outer edge's interior, and a
/// shared arc no single move separates — so each is pinned by name.
///
/// **None of the three is reachable from Python.** A ring meeting its
/// own face's outer loop is minted by raw Euler surgery on a body
/// (the shell verb's suites glue a lifted counterpart chart on with
/// `kfmrh` to build one); every Python door answers a body its own
/// producer already validated, and the binding exposes no Euler
/// operator to build one with. So the three words are pinned here,
/// and `tests/test_validate.py` says the gap is the DOORS' rather
/// than the projection's.
#[test]
fn every_ring_contact_arm_projects_the_payload_it_carries() {
    use crate::validation::project;
    use pncad::geom_core::{Band, Indeterminate, MarginDiag};
    use pncad::topo::{RingContact, ValidationError};

    let word = |contact: RingContact| {
        project(&ValidationError::RingMeetsOuter {
            face: FaceKey::default(),
            ring: Default::default(),
            contact,
        })
        .ring_contact_kind
    };

    assert_eq!(
        word(RingContact::Vertex {
            ring_vertex: VertexKey::default(),
            outer_vertex: VertexKey::default(),
        }),
        Some("vertex_vertex")
    );
    assert_eq!(
        word(RingContact::VertexOnEdge {
            ring_vertex: VertexKey::default(),
            outer_edge: Default::default(),
        }),
        Some("vertex_on_edge")
    );
    assert_eq!(
        word(RingContact::Edge {
            ring_edge: Default::default(),
            outer_edge: Default::default(),
        }),
        Some("edge_along_edge")
    );

    // The escalated sibling carries a margin, not a shape: it is a
    // ring contact that could not be decided, so there is no way the
    // ring meets the loop to name, and the arm's own word is the
    // whole answer.
    assert_eq!(
        project(&ValidationError::RingContactEscalated {
            face: FaceKey::default(),
            ring: Default::default(),
            source: Indeterminate {
                margin: MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).expect("a well-ordered band"),
                predicate: Some("ring_contact"),
            },
        })
        .ring_contact_kind,
        None
    );
}

// ---------------------------------------------------------------
// The tag table's VALUES, pinned as a set.
// ---------------------------------------------------------------

/// **The slot alphabet reads back the way it was written.**
///
/// `slot_id_tag` writes the word a refusal publishes; `slot_from_word`
/// reads the same word off a caller. A door that takes one is an
/// address a caller can RETRY at only while the two agree word for
/// word, so this pins them against each other in both directions.
///
/// The roster is not restated here. It is [`TAG_INVENTORY`]'s own
/// `slot_id_tag` row — pinned to `src/tags.rs` by the guard below — so
/// a slot the kernel adds arrives in this test through a table that is
/// already required to move with it, rather than through a list
/// someone has to remember to extend.
#[test]
fn every_slot_word_reads_back_to_the_slot_it_names() {
    let entry = TAG_INVENTORY
        .iter()
        .find(|entry| entry.function == "slot_id_tag")
        .expect("the inventory carries the slot alphabet");
    for word in entry.values {
        match crate::slot_word::slot_from_word(word) {
            Some(slot) => assert_eq!(
                crate::tags::slot_id_tag(&slot),
                *word,
                "`{word}` reads back as a slot the forward map spells otherwise"
            ),
            // The one word an address is not completed by: a profile
            // program's expression is reached by a loop index, a step
            // index and an argument role, none of which the word
            // carries.
            None => assert_eq!(
                *word, "profile",
                "`{word}` is a slot a caller can read off a refusal and cannot write back at"
            ),
        }
    }
    // Nothing OUTSIDE the alphabet reads: a near miss is a refusal at
    // the boundary, not a slot chosen by prefix or by case.
    for junk in [
        "",
        "origin",
        "Distance",
        "distance ",
        "count_x",
        "profile_0_0",
    ] {
        assert!(
            crate::slot_word::slot_from_word(junk).is_none(),
            "{junk:?} is not a slot word"
        );
    }
}

/// One row of [`TAG_INVENTORY`]: a tag function in `src/tags.rs`, and
/// the exact vocabulary it can put on the wire.
struct TagEntry {
    /// The `pub fn`'s name, as `src/tags.rs` spells it.
    function: &'static str,
    /// Every string literal the function returns ITSELF, sorted, and
    /// duplicates kept — a multiset, so a second arm minting a word
    /// the map already speaks is a change like any other rather than
    /// one a `contains` check would swallow.
    values: &'static [&'static str],
    /// Every tag function it hands an arm to, sorted. Delegation is
    /// part of the shape and not an implementation detail: flattening
    /// `Roots(fault) => root_fault_tag(fault)` into a bare `"roots"`
    /// swaps four Python-visible words for one, and would otherwise
    /// read here as four values quietly leaving the table.
    delegates: &'static [&'static str],
}

/// **The committed inventory of `src/tags.rs`.**
///
/// Generated by reading the file and then COMMITTED, which is the
/// whole mechanism: the test below re-derives it from the source at
/// test time and compares. A tag value that moves without this table
/// moving with it is a red row, by name.
///
/// The order is by function name, and the values inside a row are
/// sorted rather than in arm order — arm order is not the contract,
/// the value set is, and pinning the order would red on a rustfmt-
/// level reshuffle that no Python caller can observe.
const TAG_INVENTORY: &[TagEntry] = &[
    TagEntry {
        function: "analysis_policy_error_tag",
        values: &["quantile_mass_out_of_range"],
        delegates: &[],
    },
    TagEntry {
        function: "assembly_error_tag",
        values: &[
            "at_rest",
            "carried_mint_refusal",
            "mate_reference_refused",
            "no_at_rest_record",
            "uncertified",
        ],
        delegates: &["product_error_tag"],
    },
    TagEntry {
        function: "attr_kind_tag",
        values: &["color", "label", "visibility"],
        delegates: &[],
    },
    TagEntry {
        function: "attribution_tag",
        values: &[
            "carried_declined",
            "carried_refuted",
            "declined",
            "refuted",
            "unattributed",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "band_error_tag",
        values: &["empty", "invalid_lever_arm", "invalid_value"],
        delegates: &[],
    },
    TagEntry {
        function: "band_field_tag",
        values: &["escalate", "zero"],
        delegates: &[],
    },
    TagEntry {
        function: "binary_header_error_tag",
        values: &["binary_header_sniffs_ascii", "binary_header_too_long"],
        delegates: &[],
    },
    TagEntry {
        function: "blend_error_tag",
        values: &[
            "band",
            "body_not_intact",
            "certify",
            "chain_not_connected",
            "chain_not_g1",
            "chamfer_arm_unsupported",
            "convexity_sign_flip",
            "escalated",
            "face_clearance_uncertified",
            "nonpositive_size",
            "op",
            "radius_headroom",
            "repeated_edge",
            "ring_clearance",
            "spine_irregular",
            "spine_unsupported",
            "surgery_invariant",
            "tangential_edge",
            "unsupported_body",
            "unsupported_chain",
            "unsupported_corner",
            "unsupported_geometry",
            "unsupported_run_out",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "boolean_error_tag",
        values: &[
            "arc_loop_containment_unsupported",
            "band",
            "classification_invariant",
            "contact_contradicted",
            "containment",
            "corrupt_operand",
            "crossing_insertion",
            "curved_boolean_unsupported",
            "curved_edge_unsupported",
            "curved_pair_unsupported",
            "curved_pierce_unsupported",
            "curved_sector_side_unsupported",
            "declaration_contradicted",
            "escalated",
            "euler",
            "fallback_extent_unsupported",
            "germ_frame_cylinder_pinch",
            "germ_frame_unsupported",
            "graft_recertify",
            "invalid_declaration",
            "join",
            "join_desync",
            "merge",
            "non_maximal_faces",
            "nurbs_extent_unsupported",
            "pairing_mismatch",
            "pcurves",
            "point_split_carrier_unsupported",
            "rest_zip_unsupported",
            "result_invalid",
            "result_volume_implausible",
            "revert",
            "rim_cusp_arm_unbuilt",
            "rim_seam_not_declarable",
            "scaffolding_operand",
            "seam_orientation",
            "torn_component",
            "undeclared_coincidence",
            "unrepresentable_result",
            "unsupported_declaration_class",
            "zip_correspondence",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "census_contact_tag",
        values: &[
            "conformal_patch",
            "edge_edge_cross",
            "edge_edge_overlap",
            "edge_face_overlap",
            "edge_face_pierce",
            "vertex_on_edge",
            "vertex_on_face",
            "vertex_vertex",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "census_subject_tag",
        values: &["entity", "face_pair"],
        delegates: &[],
    },
    TagEntry {
        function: "check_evidence_tag",
        values: &[
            "connectedness",
            "escalated",
            "not_separated",
            "separation_unavailable",
            "stale_expectation",
            "unsupported",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "checks_error_tag",
        values: &[
            "band",
            "evaluation_of_another_document",
            "product_unavailable",
            "root_without_value",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "declare_error_tag",
        values: &["no_findings", "no_minted_id"],
        delegates: &["edit_error_tag"],
    },
    TagEntry {
        function: "distribution_fault_tag",
        values: &[
            "nominal_outside_support",
            "non_finite",
            "sigma_not_positive",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "distribution_field_tag",
        values: &["hi", "lo", "sigma"],
        delegates: &[],
    },
    TagEntry {
        function: "distribution_kind_tag",
        values: &["band", "normal", "truncated_normal", "uniform"],
        delegates: &[],
    },
    TagEntry {
        function: "edit_error_tag",
        values: &[
            "appearance_names_missing_node",
            "appearance_not_set",
            "appearance_wrong_kind",
            "assertion_dimension",
            "assertion_target",
            "continuous_param_cannot_be_count",
            "declare_input_not_declare",
            "declare_names_missing_node",
            "delete_would_dangle",
            "dimension",
            "doc_param_dimension_mismatch",
            "doc_param_not_declared",
            "doc_param_value_kind_mismatch",
            "duplicate_input",
            "duplicate_witness_entry",
            "empty_placement_list",
            "empty_witness_bulk",
            "improper_placement",
            "invalid_distribution",
            "invalid_tolerance",
            "measure_malformed",
            "meta_non_finite",
            "meta_not_set",
            "meta_unversioned",
            "name_unresolved_in_evaluation",
            "non_finite_alignment",
            "non_finite_doc_param",
            "non_finite_placement",
            "not_structural_slot",
            "path_off_tree",
            "payload_param_dimension_mismatch",
            "pin_unchanged",
            "placement_axis",
            "placement_on_non_instance",
            "placement_rule_mismatch",
            "profile_program_refused",
            "read_site_missing_node",
            "rebind_appearance_collision",
            "rebind_identity",
            "rebind_kind_mismatch",
            "rebind_metadata_collision",
            "rebind_no_references",
            "rebind_target_missing_node",
            "rebind_unknown_name",
            "repeated_designation",
            "set_members_on_non_list",
            "slot_dimension_mismatch",
            "structural_slot_needs_structural_edit",
            "too_few_members",
            "unknown_doc_param",
            "unknown_node",
            "unknown_payload_param",
            "unknown_slot",
            "unresolved_input",
            "update_on_non_instance",
            "witness_on_non_sketch",
            "would_cycle",
        ],
        delegates: &["root_fault_tag"],
    },
    TagEntry {
        function: "edit_inner_variant_tag",
        values: &[],
        delegates: &[
            "distribution_fault_tag",
            "expr_dimension_error_tag",
            "measure_node_fault_tag",
            "meta_version_error_tag",
            "node_error_tag",
            "program_refusal_tag",
        ],
    },
    TagEntry {
        function: "entity_id_tag",
        values: &[
            "edge",
            "face",
            "half_edge",
            "loop",
            "shell",
            "solid",
            "vertex",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "eval_error_tag",
        values: &[
            "continuous_expr_in_count_eval",
            "count_expr_in_continuous_eval",
            "count_overflow",
            "count_to_scalar_out_of_range",
            "non_finite_result",
            "param_dimension_mismatch",
            "unknown_param",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "export_error_tag",
        values: &[
            "empty_boolean",
            "node_failed",
            "not_a_body",
            "poisoned",
            "step_refused",
            "unknown_node",
        ],
        delegates: &["product_error_tag"],
    },
    TagEntry {
        function: "expr_dimension_error_tag",
        values: &[
            "count_is_integer",
            "count_needs_explicit_promotion",
            "display_unit_mismatch",
            "div_needs_scalar_divisor",
            "mismatch",
            "mul_needs_scalar",
            "non_finite",
            "not_count",
            "trig_needs_angle",
            "unknown_display_unit",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "extrude_error_tag",
        values: &[
            "band",
            "cap_plane",
            "cosurface_escalated",
            "degenerate_extrusion",
            "extrusion_escalated",
            "oblique_extrusion",
            "op",
            "side_plane",
            "sliver_join",
            "sliver_rim",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "fmt_quantity_error_tag",
        values: &["non_finite"],
        delegates: &[],
    },
    TagEntry {
        function: "frame_error_tag",
        values: &[
            "band",
            "degenerate_aim",
            "degenerate_mirror_normal",
            "degenerate_reference_ladder",
            "degenerate_roll_reference",
            "degenerate_tangent",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "hit_test_error_tag",
        values: &[
            "node_failed",
            "node_not_evaluated",
            "node_poisoned",
            "unnamed",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "inline_error_tag",
        values: &[
            "epsilon_seam",
            "foreign_instance_name",
            "inline_edit",
            "instance_body_name_referenced",
            "instance_consumed",
            "not_an_instance",
            "param_conflict",
            "part_carries_metadata",
            "stranded_part_name",
            "unknown_node",
            "unplaceable_frame",
        ],
        delegates: &["resolve_fault_tag"],
    },
    TagEntry {
        function: "interrogate_error_tag",
        values: &[
            "ambiguous",
            "no_bodies",
            "no_such_body",
            "no_such_name",
            "node_failed",
            "node_not_evaluated",
            "node_poisoned",
            "whole_body",
            "wrong_kind",
        ],
        delegates: &["readback_error_tag"],
    },
    TagEntry {
        function: "loft_error_tag",
        values: &[
            "band",
            "cap_plane",
            "degenerate_stacking",
            "euler",
            "pcurve",
            "profile",
            "reversed_stacking",
            "seam_structure",
            "section_structure",
            "skin",
            "stacking_escalated",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "mate_fault_tag",
        values: &[
            "mate_band",
            "mate_class_not_admitted",
            "mate_contradictory",
            "mate_dangling_head",
            "mate_datum_too_small_to_lever",
            "mate_frame_degenerate",
            "mate_indeterminate",
            "mate_part_selects_another_copy",
            "mate_placer_refused",
            "mate_poses_of_another_document",
            "mate_self",
            "mate_table_lacks",
            "mate_under",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "mc_refusal_tag",
        values: &["no_samples", "nominal_does_not_build"],
        delegates: &["measure_unavailable_tag"],
    },
    TagEntry {
        function: "measure_node_fault_tag",
        values: &["ref_index_out_of_range"],
        delegates: &[],
    },
    TagEntry {
        function: "measure_unavailable_at_tag",
        values: &["needs_enclosure"],
        delegates: &[],
    },
    TagEntry {
        function: "measure_unavailable_tag",
        values: &["band_has_no_measure"],
        delegates: &[],
    },
    TagEntry {
        function: "mesh_pick_error_tag",
        values: &["position_out_of_range"],
        delegates: &[],
    },
    TagEntry {
        function: "meta_version_error_tag",
        values: &["missing_version", "not_a_map", "version_not_int"],
        delegates: &[],
    },
    TagEntry {
        function: "naming_error_tag",
        values: &[
            "duplicate",
            "emission",
            "escalated",
            "missing_upstream",
            "unnamed",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "node_error_tag",
        values: &[
            "assertion_dimension",
            "axis_in_different_plane",
            "band",
            "boolean",
            "chamfer",
            "chamfer_selection_empty",
            "chamfer_selection_kind",
            "chamfer_selection_resolve",
            "crossing_unverified",
            "curved_solid_frontier",
            "declare_both_operands",
            "declare_resolve",
            "declare_unsupported_pair",
            "degenerate_direction",
            "derived_frame_section",
            "empty_half",
            "empty_operand",
            "escalated",
            "expr",
            "extrude",
            "face_frame_kind",
            "face_frame_not_planar",
            "face_frame_readback",
            "face_frame_resolve",
            "fillet",
            "fillet_selection_empty",
            "fillet_selection_kind",
            "fillet_selection_resolve",
            "instance_out_of_range",
            "loft",
            "measure_clearance_refused",
            "measure_malformed",
            "measure_non_finite",
            "measure_not_parallel",
            "measure_ref_resolve",
            "measure_ref_unreadable",
            "measure_selection_kind",
            "measure_unsupported",
            "missing_input",
            "missing_slot",
            "naming",
            "non_finite_direction",
            "non_positive_count",
            "param_box",
            "param_source_attach",
            "payload_expr",
            "placements_uncertified",
            "profile",
            "profile_anchor",
            "profile_lane_replay",
            "profile_replay",
            "revolve",
            "seed",
            "seed_pinned_section",
            "shell",
            "shell_lane_unsupported",
            "shell_open_kind",
            "shell_open_resolve",
            "skin",
            "split",
            "tolerance_conflict",
            "transform",
            "tube",
            "undeclared_contact",
            "union_declare_step",
            "unschedulable_cycle",
            "verb_arity",
            "witness_bifurcation",
            "wrong_operand",
        ],
        delegates: &[
            "mate_fault_tag",
            "part_fault_tag",
            "placement_rule_fault_tag",
        ],
    },
    TagEntry {
        function: "node_inner_kind_tag",
        values: &[],
        delegates: &[
            "band_error_tag",
            "blend_error_tag",
            "boolean_error_tag",
            "eval_error_tag",
            "eval_error_tag",
            "eval_error_tag",
            "extrude_error_tag",
            "interrogate_error_tag",
            "loft_error_tag",
            "measure_node_fault_tag",
            "naming_error_tag",
            "param_attach_error_tag",
            "param_box_error_tag",
            "profile_error_tag",
            "readback_error_tag",
            "replay_error_tag",
            "resolve_error_tag",
            "resolve_error_tag",
            "resolve_error_tag",
            "resolve_error_tag",
            "resolve_error_tag",
            "revolve_error_tag",
            "seed_error_tag",
            "shell_error_tag",
            "skin_error_tag",
            "split_op_error_tag",
            "structure_refusal_tag",
            "transform_error_tag",
            "tube_error_tag",
        ],
    },
    TagEntry {
        function: "node_pick_error_tag",
        values: &["mesh_index", "no_such_body", "not_a_body"],
        delegates: &["hit_test_error_tag", "tessellate_error_tag"],
    },
    TagEntry {
        function: "normalization_kind_tag",
        values: &[
            "degenerate_apex_cone",
            "edge_free_sphere",
            "full_period_torus",
            "seamless_periodic_band",
            "surface_promotion",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "param_attach_error_tag",
        values: &["field_not_on_kind", "stale_key"],
        delegates: &[],
    },
    TagEntry {
        function: "param_box_error_tag",
        values: &["axis_unrepresentable", "unknown_param"],
        delegates: &[],
    },
    TagEntry {
        function: "parse_error_tag",
        values: &[
            "dimension",
            "integer_overflow",
            "malformed_number",
            "trailing_input",
            "unexpected_char",
            "unexpected_end",
            "unexpected_token",
            "unknown_function",
            "unknown_param",
            "unknown_unit",
            "wrong_arity",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "part_fault_tag",
        values: &[
            "part_depth_exceeded",
            "part_no_resolver",
            "part_product",
            "part_reference_cycle",
            "part_root_failed",
        ],
        delegates: &["resolve_fault_tag"],
    },
    TagEntry {
        function: "path_error_tag",
        values: &[
            "arc_center_not_equidistant",
            "arc_continue_needs_arc_carrier",
            "arc_continue_off_carrier",
            "arc_leg_on_open_fillet",
            "arc_via_collinear",
            "band",
            "circle_split_count",
            "continuation_target_off_ray",
            "degenerate_arc_center",
            "degenerate_arc_chord",
            "degenerate_arc_spec",
            "escalated",
            "far_end_anchor_without_fillet",
            "fillet_offset_lever_too_short",
            "guided_structure",
            "junction_cusp",
            "junction_tangent",
            "no_corner_for_fillet",
            "no_corner_of_pair",
            "nonpositive_circle_radius",
            "nonpositive_fillet_radius",
            "nonpositive_leg",
            "overdetermined_junction",
            "polygon_too_few_vertices",
            "seam_arrival_lever_too_short",
            "seam_arrival_off_direction",
            "seam_retrims_arc_first_side",
            "seam_tangent",
            "underdetermined_leg",
            "zero_direction",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "corner_reason_tag",
        values: &[
            "anchor_outside_trimmed_extent",
            "behind_arrival_anchor",
            "behind_incoming_ray",
            "encloses_leg_carrier",
            "no_corner_side_candidate",
            "offset_carriers_disjoint",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "persist_error_tag",
        values: &[
            "display_unit",
            "distribution",
            "edit_replay",
            "header_id",
            "id_mismatch",
            "non_finite",
            "parse",
            "profile_program",
            "serialize",
            "snapshot",
            "tolerance_conflict",
            "tolerance_invalid",
            "unreadable",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "placement_rule_fault_tag",
        values: &[
            "empty_placement_list",
            "improper_placement",
            "non_finite_placement",
            "placement_rule_mismatch",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "product_error_tag",
        values: &[
            "contact_lineage",
            "evaluation_of_another_document",
            "graft_refused",
            "no_body_roots",
            "product_invalid",
            "product_naming",
            "root_failed",
            "root_poisoned",
            "solid_invalid",
            "unknown_node",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "profile_error_tag",
        values: &[
            "band",
            "degenerate_segment",
            "empty_profile",
            "escalated",
            "multiple_outer_loops",
            "near_full_arc",
            "nesting_too_deep",
            "non_simple",
            "ray_casting_exhausted",
            "sliver_loop",
            "structure",
            "tangency_contradicted",
            "tangent_joint_out_of_range",
            "tangential_contact",
            "too_few_vertices",
            "undeclared_tangency",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "program_fault_tag",
        values: &["lattice", "slot_dimension"],
        delegates: &[],
    },
    TagEntry {
        function: "program_refusal_tag",
        values: &["geometry", "resolve", "transition", "validate"],
        delegates: &[],
    },
    TagEntry {
        function: "promoted_curve_kind_tag",
        values: &["circle"],
        delegates: &[],
    },
    TagEntry {
        function: "promoted_kind_tag",
        values: &["cylinder", "plane"],
        delegates: &[],
    },
    TagEntry {
        function: "readback_error_tag",
        values: &[
            "dangling_entity",
            "dangling_geometry",
            "no_canonical_frame",
            "no_carrier",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "recorded_program_error_tag",
        values: &["carrier_in_chain", "subdivision_count"],
        delegates: &["expr_dimension_error_tag"],
    },
    TagEntry {
        function: "refused_ref_tag",
        values: &[
            "ref_ambiguous",
            "ref_not_a_face",
            "ref_read_below_a_root",
            "ref_vanished",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "replay_error_tag",
        values: &["path", "transition"],
        delegates: &[],
    },
    TagEntry {
        function: "resolution_status_tag",
        values: &["failed", "indeterminate", "resolved"],
        delegates: &[],
    },
    TagEntry {
        function: "resolve_error_tag",
        values: &["ambiguous", "node_gone", "vanished"],
        delegates: &[],
    },
    TagEntry {
        function: "resolve_fault_tag",
        values: &["part_epsilon_seam", "part_pin_mismatch", "part_unresolved"],
        delegates: &[],
    },
    TagEntry {
        function: "resolve_indeterminate_tag",
        values: &["target_failed", "target_not_evaluated", "target_poisoned"],
        delegates: &[],
    },
    TagEntry {
        function: "revolve_error_tag",
        values: &[
            "angle_escalated",
            "arc_crosses_axis",
            "axis_escalated",
            "band",
            "cap_plane",
            "cosurface_escalated",
            "degenerate_angle",
            "degenerate_axis",
            "full_range_angle",
            "hole_touches_axis",
            "multiple_axis_runs",
            "non_manifold_axis_contact",
            "op",
            "pcurve",
            "sliver_axis_clearance",
            "sliver_join",
            "sliver_radius",
            "sliver_rim",
            "unsupported_toroid",
            "vertex_crosses_axis",
            "void_insertion",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "ring_contact_tag",
        values: &["edge_along_edge", "vertex_on_edge", "vertex_vertex"],
        delegates: &[],
    },
    TagEntry {
        function: "root_fault_tag",
        values: &[
            "root_ancestor",
            "root_duplicate",
            "root_not_live",
            "root_uncovered",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "seed_error_tag",
        values: &["count_param", "tangent_unrepresentable", "unknown_param"],
        delegates: &[],
    },
    TagEntry {
        function: "select_refusal_tag",
        values: &[
            "bad_value",
            "band",
            "in_band",
            "not_a_datum",
            "not_a_length",
            "pair_in_band",
            "tied_disagrees",
            "unclassified",
            "unreadable",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "shell_classify_error_tag",
        values: &["band", "escalated", "props", "zero_volume"],
        delegates: &[],
    },
    TagEntry {
        function: "shell_error_tag",
        values: &[
            "band",
            "chart_sense_mixed",
            "chart_spans_solids",
            "corrupt",
            "escalated",
            "face",
            "insert",
            "lift",
            "no_solid",
            "not_valid",
            "open_face_chart_partial",
            "open_face_repeated",
            "open_face_rim_not_expressible",
            "open_face_ring_unsupported",
            "open_face_stale",
            "open_faces_disconnect",
            "open_faces_exhaust_shell",
            "operand_outer_shells",
            "partition",
            "pcurve",
            "rim",
            "roles",
            "thickness",
            "wall_clearance",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "skin_error_tag",
        values: &[
            "bad_degree",
            "degenerate_section",
            "domain_not_unit",
            "fit",
            "knot_algebra",
            "path_tangent_reversal",
            "section_profile",
            "section_shape_mismatch",
            "structure",
            "too_few_sections",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "slot_id_tag",
        values: &[
            "chamfer_distance",
            "count",
            "direction_x",
            "direction_y",
            "direction_z",
            "distance",
            "instance",
            "normal_x",
            "normal_y",
            "normal_z",
            "origin_x",
            "origin_y",
            "origin_z",
            "profile",
            "radius",
            "revolve_angle",
            "rotation_angle",
            "rotation_axis_x",
            "rotation_axis_y",
            "rotation_axis_z",
            "shell_thickness",
            "spacing",
            "spin",
            "stations",
            "step",
            "translation_x",
            "translation_y",
            "translation_z",
            "tube_major_radius",
            "tube_minor_radius",
            "tube_wall",
            "tube_window_end",
            "tube_window_start",
            "u_x",
            "u_y",
            "u_z",
            "v_degree",
            "v_x",
            "v_y",
            "v_z",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "snapshot_error_tag",
        values: &[
            "assertion_bound",
            "blend_selection_not_canonical",
            "count_continuous",
            "dangling_input",
            "declare_input",
            "epsilon_invalid",
            "forward_input",
            "id_beyond_counter",
            "input_list",
            "mate_alignment",
            "measure_refs",
            "metadata_unversioned",
            "order_mismatch",
            "placement_frame",
            "placement_not_gauge",
            "placement_rule",
            "placement_site",
            "witness_site",
        ],
        delegates: &["root_fault_tag"],
    },
    TagEntry {
        function: "solid_name_error_tag",
        values: &["solid_name_unrepresentable"],
        delegates: &[],
    },
    TagEntry {
        function: "split_error_tag",
        values: &[
            "body_name_crosses_cut",
            "empty_cut",
            "name_straddles_cut",
            "operand_severed_from_mate",
            "part_edit",
            "part_id_collides",
            "part_name_reaches_remainder",
            "remainder_edit",
            "severed_edge",
            "split_pin",
            "torn_cluster",
            "uncut_param_reference",
            "unknown_cut_node",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "split_op_error_tag",
        values: &["finish", "join", "pcurves", "reduce"],
        delegates: &[],
    },
    TagEntry {
        function: "stale_declaration_tag",
        values: &["curve_locus", "patch", "vertex_on_face", "vertex_vertex"],
        delegates: &[],
    },
    TagEntry {
        function: "step_import_error_tag",
        values: &[
            "adoption",
            "assembly",
            "dangling_reference",
            "declaration_unresolved",
            "instance",
            "invalid_eps_override",
            "malformed_real",
            "malformed_record",
            "missing_uncertainty",
            "nothing_to_import",
            "pcurves",
            "placement",
            "recognition_ambiguous",
            "rim_off_wall_boundary",
            "structure",
            "syntax",
            "tier_invalid",
            "topology",
            "unsupported_entity",
            "unsupported_unit",
            "vertex_without_point",
            "wrong_entity_type",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "stl_error_tag",
        values: &[
            "degenerate_triangle",
            "index_out_of_range",
            "io",
            "too_many_triangles",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "structure_refusal_tag",
        values: &["flipped", "indeterminate"],
        delegates: &[],
    },
    TagEntry {
        function: "tessellate_error_tag",
        values: &[
            "certificate_exceeded",
            "empty_loop",
            "invalid_chordal_tolerance",
            "missing_entity",
            "null_scaffold_edge",
            "resolution_overflow",
            "ring_on_curved_face",
            "self_touching_trim_loop",
            "tolerance_band_unformable",
            "triangulation",
            "unsupported_curve",
            "unsupported_curved_domain",
            "unsupported_curved_shape",
            "unsupported_nurbs_face",
            "unsupported_surface",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "transform_error_tag",
        values: &[
            "approx_lane_unsupported",
            "approx_recertify",
            "band",
            "certify",
            "corrupt",
            "non_finite_map",
            "not_rigid",
            "null_scaffold",
            "nurbs_placeholder",
            "pcurve",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "tube_error_tag",
        values: &[
            "band",
            "degenerate_window",
            "escalated",
            "frame_not_orthogonal",
            "full_range_window",
            "non_unit_axis",
            "non_unit_u_ref",
            "nonpositive_wall",
            "revolve",
            "wall_exceeds_radius",
            "wall_gap_collapsed",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "update_error_tag",
        values: &["already_pinned", "no_such_reference"],
        delegates: &[],
    },
    TagEntry {
        function: "validation_error_tag",
        values: &[
            "approx_certification",
            "approx_lane_unsupported",
            "back_pointer_mismatch",
            "band",
            "census_escalated",
            "census_lane_unsupported",
            "census_undecidable",
            "census_unsupported",
            "component_euler_violation",
            "contact_contradicted",
            "curved_sense_inverted",
            "dangling_description",
            "dangling_geometry",
            "dangling_topology",
            "degenerate_torus",
            "degenerate_torus_escalated",
            "description_not_adjacent",
            "edge_across_shells",
            "edge_certification",
            "edge_halves_identical",
            "edge_not_antiparallel",
            "edge_slot_backpointer_mismatch",
            "emanating_start_mismatch",
            "empty_loop_vertex_with_emanating",
            "half_edge_multiply_claimed",
            "half_edge_unclaimed",
            "lamina_wedge",
            "leaked_null_face_record",
            "leaked_provenance",
            "lone_vertex_with_incidence",
            "loop_cycle_overrun",
            "loop_role_inverted",
            "missing_provenance",
            "multiply_owned",
            "negative_volume",
            "next_prev_mismatch",
            "nonpositive_torus_tube",
            "null_edge_at_rest",
            "null_face_at_rest",
            "null_scaffold_shared",
            "orbit_foreign_member",
            "orphan_entity",
            "orphan_geometry",
            "outer_listed_as_ring",
            "parent_loop_mismatch",
            "pcurve",
            "planar_boundary_escalated",
            "planar_boundary_residual",
            "planar_face_escalated",
            "planar_face_residual",
            "poisoned_surface_description",
            "ring_contact_escalated",
            "ring_meets_outer",
            "scaffold_at_rest",
            "scaffolding_empty_loop",
            "scaffolding_strut_vertex",
            "shell_disconnected",
            "shell_without_faces",
            "sliver_dihedral",
            "solid_without_shells",
            "split_vertex_orbit",
            "stale_contact_declaration",
            "stale_null_face_loop",
            "tangent_not_intrinsic",
            "transverse_not_intrinsic",
            "uncertifiable_surface",
            "undeclared_contact",
            "undeclared_cusp",
            "unreachable_half_edge",
            "vertex_orbit_overrun",
            "volume_uncomputable",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "workspace_error_tag",
        values: &[
            "duplicate_id",
            "header",
            "io",
            "load",
            "pin",
            "pin_mismatch",
            "randomness_unavailable",
            "save",
            "save_target_not_in_store",
            "save_would_duplicate_id",
            "unknown_id",
            "update",
        ],
        delegates: &[],
    },
];

/// The committed inventory of `src/tags.rs`'s `pub const` tag words —
/// the tags that are not behind a `match` at all.
///
/// One entry today. It exists because the evaluation door has no
/// kernel arm to match on and spelled the standing ladder's first rung
/// by hand; `the_evaluation_door_speaks_the_standing_ladder` pins the
/// COPY against the two doors that do match, and this row pins the
/// word itself, so the three cannot drift together in silence.
const TAG_CONSTS: &[(&str, &str)] = &[("NODE_NOT_EVALUATED", "node_not_evaluated")];

/// Everything [`read_tag_table`] recognised in `src/tags.rs`.
struct TagTable {
    /// Function name -> (its own literals, sorted; its delegates, sorted).
    functions: BTreeMap<String, (Vec<String>, Vec<String>)>,
    /// `pub const` name -> its literal.
    constants: BTreeMap<String, String>,
}

/// The contents of the string literal at `at`, and the offset one past
/// it — or `None` when `at` opens no literal, or opens one that never
/// closes.
///
/// **The extent comes from the CODE view, and that is the whole
/// technique.** [`code_only`] has blanked every literal to spaces,
/// prefix, escapes and closing delimiter included, so the first byte
/// that survived blanking is past the literal's end; the contents are
/// then read out of `text` at those offsets. Nothing here knows what an
/// escape is, which is the point of asking the shared lexer instead.
fn string_literal<'a>(text: &'a str, code: &str, at: usize) -> Option<(&'a str, usize)> {
    if !text[at..].starts_with('"') {
        return None;
    }
    let blanked = code.as_bytes();
    let mut end = at;
    while end < blanked.len() && blanked[end] == b' ' {
        end += 1;
    }
    // Trailing source whitespace — and any blanked comment — sits in
    // the same run of spaces, so the literal is what is left after it.
    let literal = text[at..end].trim_end();
    let value = literal.strip_prefix('"')?.strip_suffix('"')?;
    Some((value, at + literal.len()))
}

/// A cursor over ONE tag function's body, in the two views the shared
/// lexer supplies.
///
/// Both preserve byte offsets, so `at` indexes either — and the file's
/// text as well. What this file used to re-derive to get here (a
/// quote-aware line-comment stripper, string and escape lexing, three
/// brace-depth loops) is [`test_utils::source`]'s, and the precondition
/// it states is what makes the depth counting below a parse: in the
/// code view every bracket is a real bracket.
struct Cursor<'a> {
    /// Comments blanked, literals KEPT — the view tokens are read from.
    text: &'a str,
    /// Comments and literals both blanked — the view structure is read
    /// from, and the only one [`balanced_end`] may be run over.
    code: &'a str,
    /// The read position, a byte offset into both views.
    at: usize,
    /// One past the body's last byte.
    end: usize,
    /// The function the body belongs to — messages name it.
    what: &'a str,
}

impl<'a> Cursor<'a> {
    /// The unread remainder.
    fn rest(&self) -> &'a str {
        &self.text[self.at..self.end]
    }

    /// Advance past whitespace — and past any comment, which the view
    /// has already blanked to whitespace.
    fn skip_ws(&mut self) {
        let rest = self.rest();
        self.at += rest.len() - rest.trim_start().len();
    }

    /// Advance one CHARACTER. The views are blanked byte for byte and
    /// are valid UTF-8, so stepping by the character's own width keeps
    /// `at` on a boundary — which is what the reader's old `is_ascii()`
    /// refusal bought by forbidding the case outright.
    fn bump(&mut self) {
        self.at += self.code[self.at..]
            .chars()
            .next()
            .map_or(1, char::len_utf8);
    }

    /// Advance past `token` if it is next, and say whether it was.
    fn eat(&mut self, token: &str) -> bool {
        self.skip_ws();
        if self.rest().starts_with(token) {
            self.at += token.len();
            true
        } else {
            false
        }
    }

    /// Advance past `token`, or fail loud.
    fn expect(&mut self, token: &str) {
        assert!(
            self.eat(token),
            "tags.rs: in `{}`, expected `{token}` and found {:?} — \
             I do not understand this",
            self.what,
            self.rest().chars().take(60).collect::<String>()
        );
    }

    /// Read a `"..."` string starting at the cursor, returning its
    /// contents.
    fn take_string(&mut self) -> &'a str {
        assert!(
            self.rest().starts_with('"'),
            "tags.rs: in `{}`, not a string",
            self.what
        );
        let (value, end) = string_literal(self.text, self.code, self.at).unwrap_or_else(|| {
            panic!(
                "tags.rs: in `{}`, an unterminated string literal",
                self.what
            )
        });
        self.at = end;
        value
    }

    /// Consume one arm's PATTERN, up to the `=>` that ends it.
    ///
    /// Read over the code view, where a bracket inside a string literal
    /// is a space — so [`balanced_end`] steps over a struct pattern
    /// (`ReadbackError::Dangling { what: ... }`) whole, however many
    /// lines it spans, and a closer met here has no opener at all.
    fn skip_pattern(&mut self) {
        let blanked = self.code.as_bytes();
        while self.at < self.end {
            match blanked[self.at] {
                b'(' | b'[' | b'{' => {
                    let Some(close) = balanced_end(self.code, self.at) else {
                        break;
                    };
                    self.at = close + 1;
                }
                b')' | b']' | b'}' => panic!(
                    "tags.rs: in `{}`, a match arm closed before its `=>` — \
                     I do not understand this",
                    self.what
                ),
                b'=' if blanked.get(self.at + 1) == Some(&b'>') => return,
                _ => self.bump(),
            }
        }
        panic!(
            "tags.rs: in `{}`, a match arm ran off the end of the body",
            self.what
        );
    }

    /// Parse `match SCRUTINEE { ARMS }`, collecting what the arms mint.
    fn parse_match(&mut self, values: &mut Vec<String>, delegates: &mut Vec<String>) {
        self.expect("match");
        let blanked = self.code.as_bytes();
        loop {
            assert!(
                self.at < self.end,
                "tags.rs: in `{}`, a `match` with no `{{`",
                self.what
            );
            match blanked[self.at] {
                // The scrutinee's own brackets, stepped over whole, so
                // a `{` inside one is not mistaken for the block's.
                b'(' | b'[' => {
                    self.at = balanced_end(self.code, self.at).unwrap_or_else(|| {
                        panic!("tags.rs: in `{}`, a `match` with no `{{`", self.what)
                    }) + 1;
                }
                b'{' => break,
                _ => self.bump(),
            }
        }
        self.expect("{");
        loop {
            if self.eat("}") {
                return;
            }
            self.skip_pattern();
            self.expect("=>");
            self.parse_arm_body(values, delegates);
            let _ = self.eat(",");
        }
    }

    /// Parse the RIGHT of one arm's `=>`.
    ///
    /// Exactly six shapes are recognised, which is the enumerating
    /// claim this whole reader rests on: a string literal, a nested
    /// `match`, a `{ ... }` block around one of those, a call to
    /// another tag function, and — for the maps that answer
    /// `Option<&'static str>` — a bare `None` or a `Some(..)` around
    /// one of the others. Anything else — a `format!`, a `if`, a
    /// `const` reference, a method chain — fails here by name rather
    /// than being skipped, because a tag arrived at by a route this
    /// reader cannot follow is a tag the inventory silently stops
    /// covering.
    ///
    /// `None` contributes NOTHING: an arm that projects no word is a
    /// decision the reader records by the absence of a value, exactly
    /// as the source spells it. `Some` is not read as a delegation —
    /// it is the wrapper, and what it wraps is what reaches the
    /// inventory.
    fn parse_arm_body(&mut self, values: &mut Vec<String>, delegates: &mut Vec<String>) {
        self.skip_ws();
        let rest = self.rest();
        if rest.starts_with('"') {
            let value = self.take_string();
            assert!(
                !value.is_empty()
                    && value
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
                "tags.rs: in `{}`, the tag {value:?} is not lower snake case — \
                 every tag in this file is, and a reader that accepted \
                 anything would be guessing",
                self.what
            );
            values.push(value.to_owned());
            return;
        }
        if let Some(after) = rest.strip_prefix("match")
            && after.starts_with(|c: char| c.is_whitespace())
        {
            self.parse_match(values, delegates);
            return;
        }
        if rest.starts_with('{') {
            self.expect("{");
            self.parse_arm_body(values, delegates);
            self.expect("}");
            return;
        }
        // `None` and `Some(..)`, in that order: `Some` must be tested
        // before the delegation shape below, which would otherwise
        // read the wrapper as the called map and skip what it wraps.
        if let Some(after) = rest.strip_prefix("None")
            && !after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        {
            self.expect("None");
            return;
        }
        if rest.starts_with("Some(") {
            self.expect("Some");
            self.expect("(");
            self.parse_arm_body(values, delegates);
            self.expect(")");
            return;
        }
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if !name.is_empty() && rest[name.len()..].trim_start().starts_with('(') {
            self.at += name.len();
            self.skip_ws();
            let open = self.at;
            let close = balanced_end(self.code, open)
                .unwrap_or_else(|| panic!("tags.rs: in `{}`, a call that never closes", self.what));
            assert!(
                !self.text[open..=close].contains('"'),
                "tags.rs: in `{}`, a string literal inside a delegation's \
                 arguments — I do not understand this",
                self.what
            );
            self.at = close + 1;
            delegates.push(name);
            return;
        }
        panic!(
            "tags.rs: in `{}`, I do not understand this match arm's body: {:?}. \
             Teach this reader in the same diff — a shape it cannot follow is a \
             tag value the inventory stops covering, silently.",
            self.what,
            rest.chars().take(60).collect::<String>()
        );
    }
}

/// One tag function's body, read into (values, delegates).
fn parse_tag_body(
    name: &str,
    text: &str,
    code: &str,
    body: std::ops::Range<usize>,
) -> (Vec<String>, Vec<String>) {
    let mut cursor = Cursor {
        text,
        code,
        at: body.start,
        end: body.end,
        what: name,
    };
    // The `use ... as R;` shorthands some functions open with.
    loop {
        cursor.skip_ws();
        if !cursor.rest().starts_with("use ") {
            break;
        }
        let end = cursor.rest().find(';').unwrap_or_else(|| {
            panic!("tags.rs: in `{name}`, a `use` with no `;` — I do not understand this")
        });
        cursor.at += end + 1;
    }
    let mut values = Vec::new();
    let mut delegates = Vec::new();
    cursor.parse_match(&mut values, &mut delegates);
    cursor.skip_ws();
    assert!(
        cursor.rest().is_empty(),
        "tags.rs: `{name}`'s body has text after its `match`: {:?} — \
         I do not understand this",
        cursor.rest().chars().take(60).collect::<String>()
    );
    values.sort();
    delegates.sort();
    (values, delegates)
}

/// **Read `src/tags.rs` and enumerate its tag table.**
///
/// A RECOGNISER THAT ENUMERATES, in the house sense
/// (`scripts/check-ci-mirror-parity.py`'s header makes the argument at
/// length): every top-level line must match one of the forms below,
/// and anything else raises rather than being skipped. The failure
/// mode this rules out is the one that matters — a reader that quietly
/// matches nothing, reports a happy zero, and lets the pin pass
/// vacuously forever.
///
/// The recognised top-level forms, and nothing else: a blank line; any
/// `//` comment, doc comment or inner doc comment; a `use` item, one
/// line or a `{`-opened block closed by `};`; a
/// `pub fn NAME(..) -> &'static str {` whose body closes with a `}` in
/// column 0; and a `pub const NAME: &str = "value";`.
///
/// **Every one of those is matched over a view from the shared lexer**
/// ([`test_utils::source`]) rather than over the raw text: `code` is
/// the file with comments and literals blanked, `text` is the file with
/// comments alone blanked, and both preserve byte offsets, so a form
/// located structurally in `code` has its literal read out of `text` at
/// the same offsets. A comment is therefore already whitespace by the
/// time this loop sees a line, and the reader spells no comment
/// delimiter of its own.
fn read_tag_table(source: &str) -> TagTable {
    let text = code_and_literals(source);
    let code = code_only(source);
    let lines: Vec<&str> = source.lines().collect();
    let code_lines: Vec<&str> = code.lines().collect();
    // Both views preserve byte offsets AND line structure, so one table
    // of line starts serves the source and both of them.
    let starts: Vec<usize> = code
        .split_inclusive('\n')
        .scan(0usize, |at, line| {
            let start = *at;
            *at += line.len();
            Some(start)
        })
        .collect();
    let mut functions: BTreeMap<String, (Vec<String>, Vec<String>)> = BTreeMap::new();
    let mut constants: BTreeMap<String, String> = BTreeMap::new();
    let mut i = 0;
    while i < lines.len() {
        // Matched on the code view; REPORTED as the file spells it.
        let line = lines[i];
        let code_line = code_lines[i];
        let number = i + 1;
        // A blank line and a line that is nothing but comment are one
        // case here, because the lexer has already blanked the second.
        if code_line.trim().is_empty() {
            i += 1;
            continue;
        }
        if let Some(rest) = code_line.strip_prefix("use ") {
            if rest.ends_with('{') {
                i += 1;
                while i < lines.len() && code_lines[i] != "};" {
                    i += 1;
                }
                assert!(
                    i < lines.len(),
                    "tags.rs:{number}: a `use` block that never closes with `}};` — \
                     I do not understand this file"
                );
            } else {
                assert!(
                    rest.ends_with(';'),
                    "tags.rs:{number}: a `use` item that neither ends in `;` nor \
                     opens a block — I do not understand this: {line}"
                );
            }
            i += 1;
            continue;
        }
        if let Some(rest) = code_line.strip_prefix("pub fn ") {
            let (name, tail) = rest.split_once('(').unwrap_or_else(|| {
                panic!("tags.rs:{number}: a `pub fn` with no argument list: {line}")
            });
            assert!(
                tail.ends_with(") -> &'static str {")
                    || tail.ends_with(") -> Option<&'static str> {"),
                "tags.rs:{number}: a `pub fn` in the tag module whose signature is \
                 neither `(..) -> &'static str {{` nor \
                 `(..) -> Option<&'static str> {{` on one line — I do not \
                 understand this, and cannot say what it puts on the wire: {line}"
            );
            assert!(
                !name.is_empty()
                    && name
                        .bytes()
                        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_'),
                "tags.rs:{number}: {name:?} is not a lower snake-case name: {line}"
            );
            let mut j = i + 1;
            while j < lines.len() && code_lines[j] != "}" {
                j += 1;
            }
            assert!(
                j < lines.len(),
                "tags.rs:{number}: `{name}` has no closing `}}` in column 0 — \
                 I do not understand where its body ends"
            );
            let body = starts[i + 1]..starts[j];
            let previous =
                functions.insert(name.to_owned(), parse_tag_body(name, &text, &code, body));
            assert!(
                previous.is_none(),
                "tags.rs:{number}: `{name}` is defined twice"
            );
            i = j + 1;
            continue;
        }
        if let Some(rest) = code_line.strip_prefix("pub const ") {
            let (name, tail) = rest.split_once(": &str = ").unwrap_or_else(|| {
                panic!(
                    "tags.rs:{number}: a `pub const` in the tag module that is not a \
                     `&str` — I do not understand this: {line}"
                )
            });
            // The value is blanked in the code view, so it is located
            // by the structure around it and read from `text`.
            let at = starts[i] + code_line.len() - tail.len();
            let end_of_line = starts[i] + code_line.len();
            let value = string_literal(&text, &code, at)
                .filter(|&(_, after)| text[after..end_of_line].trim() == ";")
                .map(|(value, _)| value)
                .unwrap_or_else(|| {
                    panic!(
                        "tags.rs:{number}: a `&str` const whose value is not a bare \
                         literal — I do not understand this: {line}"
                    )
                });
            let previous = constants.insert(name.to_owned(), value.to_owned());
            assert!(
                previous.is_none(),
                "tags.rs:{number}: `{name}` is defined twice"
            );
            i += 1;
            continue;
        }
        panic!(
            "tags.rs:{number}: I do not understand this top-level line, so the tag \
             table cannot be enumerated past it. Teach this reader in the same diff \
             that adds the construct: {line}"
        );
    }
    TagTable {
        functions,
        constants,
    }
}

/// **Every tag value `src/tags.rs` can put on the wire is the one
/// Python was promised.**
///
/// The tag `match`es are exhaustive, so a NEW kernel arm stops this
/// crate compiling; the existence half of the contract is the
/// compiler's and wants nothing from a test. The VALUES have had no
/// guard at all. They are bare string literals, and renaming one —
/// `"unknown_node"` to `"node_unknown"` — compiles clean, satisfies
/// every exhaustiveness alarm in the file, and breaks every Python
/// caller branching on the string. **The tag values are a PUBLIC
/// PYTHON CONTRACT, not an implementation detail**: `pncad.pyi` names
/// them, `tests/*.py` branches on them, and a user's
/// `except PncadError as e: if e.tag == "..."` is written against
/// them.
///
/// So this reads `src/tags.rs` at test time, enumerates every tag
/// function and every literal each can return, and compares that
/// against [`TAG_INVENTORY`] — committed, in this file, where a
/// reviewer sees it move in the same diff as the value. It lives on
/// the Python-INDEPENDENT path on purpose: the pin fires on the
/// default no-interpreter CI row, which is the row that runs
/// everywhere.
///
/// **What it proves.** A renamed value reds, by name, saying which
/// function and which word. A value added to an existing map reds. A
/// value deleted reds. A whole new tag function reds, because its
/// vocabulary is new public surface that nobody has looked at. A
/// delegation flattened into a literal (or re-pointed at a different
/// map) reds, since that swaps one Python-visible word for another
/// map's whole set.
///
/// **What it does NOT prove, which is the more interesting half.** An
/// inventory pins the VOCABULARY, not the MAPPING. Swap two arms'
/// literals — `WouldCycle` returns `"delete_would_dangle"` and
/// `DeleteWouldDangle` returns `"would_cycle"` — and this test is
/// perfectly green: the set of words the file speaks did not change,
/// only which refusal says which. That failure is caught by the
/// CONSTRUCTION pins (`readback_refusal_tags_are_stable` and its
/// siblings above), which build or drive a real arm and assert the
/// word it answers with. The two guards are complements and neither
/// subsumes the other.
///
/// **And the construction pins are sampled, not total.** Some maps
/// have one and many have none; even where a map is pinned, the pin
/// builds a handful of its arms and the rest ride on the inventory
/// alone. For those the inventory below is the ONLY thing between a
/// rename and a broken caller. That is a large gain over nothing; it
/// is not the same claim as "the tag table is verified", and this
/// comment refuses to make the second one.
///
/// **No count and no roster is written here, deliberately.** This
/// paragraph used to carry three aggregate numbers and two lists of
/// function names, and every one of them went stale without anything
/// going red — which is the argument `tests/test_binding_census.py`
/// makes for refusing to write a count down, applied to the page that
/// was writing three. The lists rotted the same way and less visibly:
/// `node_error_tag` stood in the never-pinned roster while
/// [`shell_refusal_tags_are_stable`] had been constructing three of
/// its arms. What is checkable by machine is the table below; what is
/// checkable by reading is each pin, beside the map it pins.
///
/// **Out of scope, stated so it is not read as covered.**
/// `SelectRefusal::{InBand, PairInBand}` carry a `predicate: &'static
/// str` that `py/select.rs` surfaces as `SelectRefusal.predicate`, so
/// K predicate names reach Python too. Those names are minted in
/// `editor-core` and `topo`, not here, and neither arm is
/// constructible from this crate (see
/// `select_refusal_tags_are_stable`), so nothing on this page pins
/// them. `work/lib/` carries that as its own row.
///
/// **A second family is outside it too**: the `reason` words minted as
/// bare literals in `py/value.rs` rather than as a tag function here —
/// `"wrong_kind"` (five sites), `"empty_boolean"`, `"unknown_node"`,
/// `"poisoned"`, `"node_failed"` and `"mass_properties_failed"` — which
/// cross as an exception's `reason` attribute and so are as
/// Python-visible as anything in `tags.rs`. `pncad.pyi` documents the
/// first three and `node_failed`/`poisoned`; `"mass_properties_failed"`
/// is pinned nowhere in the tree. This inventory reads `src/tags.rs`
/// and nothing else, so none of them is covered by it.
#[test]
fn the_whole_tag_table_matches_its_committed_inventory() {
    // `crate_dir`, not the baked path alone: a nextest ARCHIVE replayed
    // on another runner has no such directory, and this crate's own
    // source is what the guard opens.
    let path = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tags.rs");
    let source = std::fs::read_to_string(&path).expect("this crate's own src/tags.rs");
    let table = read_tag_table(&source);

    // The floors: a reader that came back with nothing, or with a
    // plausible-looking handful, must red rather than pass vacuously.
    // They are set well under whatever the table currently holds, so
    // ordinary churn does not touch them — and they are ASSERTIONS,
    // which is why they may carry numbers where the prose above may
    // not.
    assert!(
        table.functions.len() >= 60,
        "the reader found only {} tag functions in src/tags.rs — it is \
         matching almost nothing, so this guard was about to pass \
         vacuously",
        table.functions.len()
    );
    let literals: usize = table.functions.values().map(|(v, _)| v.len()).sum();
    assert!(
        literals >= 500,
        "the reader found only {literals} tag literals in src/tags.rs — \
         it is matching almost nothing, so this guard was about to pass \
         vacuously"
    );
    assert!(
        !table.constants.is_empty(),
        "the reader found no `pub const` tag word, and there is at least \
         one (`NODE_NOT_EVALUATED`)"
    );

    let mut pinned: BTreeMap<&str, &TagEntry> = BTreeMap::new();
    for entry in TAG_INVENTORY {
        let previous = pinned.insert(entry.function, entry);
        assert!(
            previous.is_none(),
            "TAG_INVENTORY names `{}` twice",
            entry.function
        );
    }

    let mut complaints: Vec<String> = Vec::new();

    for (name, (values, delegates)) in &table.functions {
        let Some(entry) = pinned.get(name.as_str()) else {
            complaints.push(format!(
                "NEW tag function `{name}`, minting {values:?} — a new set of \
                 public Python words that no inventory has looked at"
            ));
            continue;
        };
        let want: Vec<String> = entry.values.iter().map(|v| (*v).to_owned()).collect();
        if *values != want {
            let added: Vec<&str> = values
                .iter()
                .filter(|v| !want.contains(v))
                .map(String::as_str)
                .collect();
            let gone: Vec<&str> = want
                .iter()
                .filter(|v| !values.contains(v))
                .map(String::as_str)
                .collect();
            if added.is_empty() && gone.is_empty() {
                complaints.push(format!(
                    "`{name}`: the same words in different multiplicities — \
                     now {values:?}, pinned {want:?}"
                ));
            } else {
                complaints.push(format!(
                    "`{name}`: value(s) ADDED {added:?}, value(s) GONE {gone:?} \
                     (a RENAME shows as one of each)"
                ));
            }
        }
        let want: Vec<String> = entry.delegates.iter().map(|d| (*d).to_owned()).collect();
        if *delegates != want {
            complaints.push(format!(
                "`{name}`: delegates to {delegates:?}, pinned as {want:?} — \
                 a forwarded arm carries the other map's whole vocabulary, so \
                 this moves Python-visible words even though no literal here \
                 changed"
            ));
        }
    }
    for entry in TAG_INVENTORY {
        if !table.functions.contains_key(entry.function) {
            complaints.push(format!(
                "tag function `{}` is GONE from src/tags.rs — the words {:?} \
                 no longer reach Python from it",
                entry.function, entry.values
            ));
        }
    }

    for (name, value) in &table.constants {
        match TAG_CONSTS.iter().find(|(n, _)| n == name) {
            None => complaints.push(format!("NEW `pub const` tag word `{name}` = {value:?}")),
            Some((_, want)) if want != value => complaints.push(format!(
                "`{name}`: the const's value is {value:?}, pinned as {want:?}"
            )),
            Some(_) => {}
        }
    }
    for (name, _) in TAG_CONSTS {
        if !table.constants.contains_key(*name) {
            complaints.push(format!("`pub const` tag word `{name}` is GONE"));
        }
    }

    assert!(
        complaints.is_empty(),
        "src/tags.rs has moved away from TAG_INVENTORY.\n\n\
         THE TAG VALUES ARE A PUBLIC PYTHON CONTRACT, not an implementation \
         detail: Python callers branch on these strings, so a renamed value \
         is a breaking change to the bindings and a new one is new public \
         surface. If the move is deliberate, update TAG_INVENTORY in this \
         same commit, and check `pncad.pyi` and `tests/*.py` for callers of \
         every word that moved.\n\n  {}",
        complaints.join("\n  ")
    );
}

/// **The committed roster of `Doc.node_kind`'s vocabulary.**
///
/// Sorted, and every word [`crate::node_kind::node_kind`] can answer.
/// The test below re-derives the set from that module's own literals
/// at test time and compares, so an added, renamed or deleted kind
/// reds by name.
const NODE_KIND_ROSTER: &[&str] = &[
    "assertion",
    "boolean_intersect",
    "boolean_subtract",
    "boolean_union",
    "chamfer",
    "datum",
    "declare",
    "extrude",
    "fillet",
    "hollow_tube",
    "instantiate_part",
    "loft",
    "mate",
    "measure",
    "part",
    "pattern",
    "placed_union",
    "profile",
    "revolve",
    "shell",
    "split",
    "sweep",
    "transform",
    "tube",
    "union",
];

/// **The node-kind words are a public Python contract**, pinned whole.
///
/// The `match` in `src/node_kind.rs` is exhaustive with no wildcard
/// arm, so a NEW kernel variant stops this crate compiling: the
/// EXISTENCE half of the contract is the compiler's and wants nothing
/// from a test. The VALUES have no such guard. They are bare string
/// literals, and renaming one — `"placed_union"` to `"group"` —
/// compiles clean and breaks every Python caller branching on the
/// string. That is the argument
/// [`the_whole_tag_table_matches_its_committed_inventory`] makes about
/// `src/tags.rs`, and this row is the same mechanism at the same
/// altitude: read the source, compare against a roster committed here,
/// where a reviewer sees it move in the same diff as the value.
///
/// **The reader is one loop because the FILE is disciplined.** Every
/// string literal in `src/node_kind.rs` is a kind word — the module
/// holds one function and nothing else that could carry a literal — so
/// "every literal in the literal view" is the whole extraction. A
/// helper added there that needed a literal of its own would red here,
/// which is the right outcome: it would also be a second thing in a
/// file whose one job is this vocabulary.
///
/// **And it reads the STUB too.** A word the Rust side speaks but
/// `pncad.pyi` never lists is a word no Python caller can discover, so
/// every roster entry must appear in backticks inside `node_kind`'s
/// own docstring — scoped to that docstring, because several of these
/// words are also method names elsewhere in the file and a whole-file
/// search would pass on those for the wrong reason.
///
/// **What it does NOT prove**, exactly as the tag inventory's does
/// not: this pins the vocabulary, not the MAPPING. Swap two arms'
/// literals and the set is unchanged. The mapping is executed by the
/// Python suite, which drives real documents through the door
/// (`tests/test_document.py`, `tests/test_placed_union.py`).
#[test]
fn the_node_kind_vocabulary_matches_its_committed_roster() {
    // `crate_dir`, not the baked path alone: a nextest ARCHIVE replayed
    // on another runner has no such directory.
    let path = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("node_kind.rs");
    let source = std::fs::read_to_string(&path).expect("this crate's own src/node_kind.rs");

    // The literal view of the shared lexer, so a word written in a doc
    // comment is prose and only a real literal is a word.
    let literals = test_utils::source::keeping(&source, &[test_utils::source::Region::Literal]);
    let mut found: Vec<String> = Vec::new();
    let mut rest: &str = &literals;
    while let Some(open) = rest.find('"') {
        rest = &rest[open + 1..];
        let close = rest
            .find('"')
            .expect("the lexer's literal view closes every quote it opens");
        found.push(rest[..close].to_owned());
        rest = &rest[close + 1..];
    }
    found.sort();

    // The floor: a reader that came back with nothing must red rather
    // than agree with an empty roster.
    assert!(
        found.len() >= 20,
        "the reader found only {} literals in src/node_kind.rs — it is \
         matching almost nothing, so this guard was about to pass \
         vacuously",
        found.len()
    );

    let want: Vec<String> = NODE_KIND_ROSTER.iter().map(|w| (*w).to_owned()).collect();
    assert!(
        want.windows(2).all(|p| p[0] < p[1]),
        "NODE_KIND_ROSTER must be sorted and free of duplicates"
    );
    let added: Vec<&str> = found
        .iter()
        .filter(|w| !want.contains(w))
        .map(String::as_str)
        .collect();
    let gone: Vec<&str> = want
        .iter()
        .filter(|w| !found.contains(w))
        .map(String::as_str)
        .collect();
    assert!(
        added.is_empty() && gone.is_empty(),
        "src/node_kind.rs has moved away from NODE_KIND_ROSTER: word(s) \
         ADDED {added:?}, word(s) GONE {gone:?} (a RENAME shows as one of \
         each).\n\nTHESE WORDS ARE A PUBLIC PYTHON CONTRACT: a Python caller \
         branches on `Doc.node_kind`, so a renamed word is a breaking change \
         and a new one is new public surface. If the move is deliberate, \
         update NODE_KIND_ROSTER in this same commit and check `pncad.pyi` \
         and `tests/*.py` for callers of every word that moved."
    );

    // The stub side: every word listed where a Python caller reads it.
    let stub = std::fs::read_to_string(
        test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("pncad.pyi"),
    )
    .expect("this crate's own pncad.pyi");
    let from = stub
        .find("def node_kind(")
        .expect("pncad.pyi declares Doc.node_kind");
    let docstring = &stub[from..];
    let docstring = &docstring[..docstring.find("\n    def ").unwrap_or(docstring.len())];
    let unlisted: Vec<&str> = NODE_KIND_ROSTER
        .iter()
        .copied()
        .filter(|w| !docstring.contains(&format!("`{w}`")))
        .collect();
    assert!(
        unlisted.is_empty(),
        "`Doc.node_kind`'s docstring in pncad.pyi does not list {} of its \
         own words: {unlisted:?} — a word a caller cannot discover is a word \
         that is not really bound",
        unlisted.len()
    );
}

// ---- the gathered product, memoized on an evaluation -----------------
//
// `crate::product_memo` is the behaviour behind four bound doors, and
// these are its acceptance rows. They live HERE, on the default build
// path, for the reason the module does: a `#[pyfunction]` needs an
// interpreter and this does not, so the count that matters — how many
// times the document was gathered — is pinned in the twelve `test`
// jobs of every code-tier run rather than only in the python suite.
//
// The witness is `pncad::document::gathers_on_this_thread`, the
// gather's own thread-local counter, read as a DIFFERENCE across the
// calls being asked about. It is `cfg(debug_assertions)`-gated, so
// these rows are too; every profile this workspace builds keeps it on.
mod product_memo_rows {
    use crate::product_memo::{self, ProductMemo};
    use pncad::document as d;
    use pncad::tolerance::Tol;

    /// The world xy frame, the plane the fixture sketches on.
    fn xy_frame() -> d::Node<d::ProfileProgram> {
        let len = |v: f64| d::Expr::literal(v, d::Dimension::Length).expect("a length literal");
        let scl = |v: f64| d::Expr::literal(v, d::Dimension::Scalar).expect("a scalar literal");
        d::Node::Datum(d::Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        })
    }

    /// A square `[0,s]²` on `plane`.
    fn square(plane: d::RecipeNodeId, s: f64) -> d::Node<d::ProfileProgram> {
        let lit = |v: f64| d::Expr::literal(v, d::Dimension::Length).expect("a length literal");
        d::Node::Profile(d::ProfileProgram {
            plane,
            loops: vec![d::LoopProgram::Chain(vec![
                d::ProgramStep::At([lit(0.0), lit(0.0)]),
                d::ProgramStep::LineTo(d::ProgramTarget::Point([lit(s), lit(0.0)])),
                d::ProgramStep::LineTo(d::ProgramTarget::Point([lit(s), lit(s)])),
                d::ProgramStep::LineTo(d::ProgramTarget::Point([lit(0.0), lit(s)])),
                d::ProgramStep::LineTo(d::ProgramTarget::Start),
            ])],
        })
    }

    fn insert(
        doc: d::ProfileDoc,
        node: d::Node<d::ProfileProgram>,
    ) -> (d::ProfileDoc, d::RecipeNodeId) {
        let applied = d::apply(&doc, &d::DocEdit::InsertNode { node }, Tol::witness())
            .expect("the edit is accepted");
        let minted = applied.record.minted.expect("an insert mints an id");
        (applied.doc, minted)
    }

    /// One box: square(2) extruded 1.5, under the id `label` derives.
    fn box_doc(label: &str) -> d::ProfileDoc {
        let lit = |v: f64| d::Expr::literal(v, d::Dimension::Length).expect("a length literal");
        let doc = d::ProfileDoc::empty_derived(label, Tol::witness());
        let (doc, plane) = insert(doc, xy_frame());
        let (doc, profile) = insert(doc, square(plane, 2.0));
        let (doc, _) = insert(
            doc,
            d::Node::Extrude {
                profile,
                distance: lit(1.5),
            },
        );
        doc
    }

    /// A document that draws nothing: a plane and a sketch, no solid.
    fn sketch_only(label: &str) -> d::ProfileDoc {
        let doc = d::ProfileDoc::empty_derived(label, Tol::witness());
        let (doc, plane) = insert(doc, xy_frame());
        let (doc, _) = insert(doc, square(plane, 2.0));
        doc
    }

    fn evaluated(doc: &d::ProfileDoc) -> d::Evaluation<f64> {
        d::evaluate::<f64>(
            doc,
            None,
            &d::CancelToken::new(),
            &d::EvalOptions::default(),
            Tol::witness(),
        )
    }

    /// Gathers performed while `body` ran.
    #[cfg(debug_assertions)]
    fn gathers(body: impl FnOnce()) -> u64 {
        let before = d::gathers_on_this_thread();
        body();
        d::gathers_on_this_thread() - before
    }

    /// The unit's whole point: a Python caller asking both questions
    /// of one evaluation pays for ONE gather, whichever order it asks
    /// in. Before the memo each wrapper gathered for itself.
    #[cfg(debug_assertions)]
    #[test]
    fn the_checks_registry_and_the_assembly_gate_share_one_gather() {
        let doc = box_doc("memo-both-orders");
        let ev = evaluated(&doc);
        let cfg = d::ChecksConfig::default();
        let tol = Tol::witness();

        let memo = ProductMemo::default();
        let checks_then_assembly = gathers(|| {
            product_memo::checks_report(&memo, &doc, &ev, &cfg, tol).expect("the registry runs");
            product_memo::assembly(&memo, &doc, &ev, tol).expect("the gate passes");
        });
        assert_eq!(checks_then_assembly, 1);

        let memo = ProductMemo::default();
        let assembly_then_checks = gathers(|| {
            product_memo::assembly(&memo, &doc, &ev, tol).expect("the gate passes");
            product_memo::checks_report(&memo, &doc, &ev, &cfg, tol).expect("the registry runs");
        });
        assert_eq!(assembly_then_checks, 1);
    }

    /// The sweep: every door that wants a product joins the memo, so
    /// all four together are still one gather.
    #[cfg(debug_assertions)]
    #[test]
    fn all_four_product_doors_share_one_gather() {
        let doc = box_doc("memo-four-doors");
        let ev = evaluated(&doc);
        let cfg = d::ChecksConfig::default();
        let tol = Tol::witness();
        let memo = ProductMemo::default();
        let count = gathers(|| {
            product_memo::body(&memo, &doc, &ev, tol).expect("the gather succeeds");
            product_memo::body_and_names(&memo, &doc, &ev, tol).expect("and names its entities");
            product_memo::checks_report(&memo, &doc, &ev, &cfg, tol).expect("the registry runs");
            product_memo::assembly(&memo, &doc, &ev, tol).expect("the gate passes");
        });
        assert_eq!(count, 1);
    }

    /// The clone the assembly gate is handed is a clone of the SAME
    /// product the registry read: the memo outlives the consuming
    /// door, so a later caller finds it rather than re-earning it.
    #[cfg(debug_assertions)]
    #[test]
    fn the_gate_consumes_a_copy_and_the_memo_survives_it() {
        let doc = box_doc("memo-survives-the-gate");
        let ev = evaluated(&doc);
        let tol = Tol::witness();
        let memo = ProductMemo::default();
        product_memo::assembly(&memo, &doc, &ev, tol).expect("the gate passes");
        let again = gathers(|| {
            product_memo::assembly(&memo, &doc, &ev, tol).expect("and passes again");
            product_memo::body(&memo, &doc, &ev, tol).expect("the gather succeeds");
        });
        assert_eq!(again, 0);
    }

    /// The registry's laziness survives the memo, and it is the point:
    /// a configuration whose enabled residents all read the evaluation
    /// has nothing to gather FOR, so nothing is gathered and the memo
    /// is not filled either.
    #[cfg(debug_assertions)]
    #[test]
    fn a_subject_free_configuration_gathers_nothing() {
        let doc = box_doc("memo-lazy");
        let ev = evaluated(&doc);
        let cfg = d::ChecksConfig {
            separation: d::Advisory::Off,
            ..d::ChecksConfig::default()
        };
        assert!(!cfg.needs_a_subject());
        let tol = Tol::witness();
        let memo = ProductMemo::default();
        let none = gathers(|| {
            product_memo::checks_report(&memo, &doc, &ev, &cfg, tol).expect("the registry runs");
        });
        assert_eq!(none, 0);
        let first = gathers(|| {
            product_memo::body(&memo, &doc, &ev, tol).expect("the gather succeeds");
        });
        assert_eq!(first, 1);
    }

    /// A gather that REFUSES carries no product to keep, so the memo
    /// stays empty and the next ask re-gathers — never worse than a
    /// façade that gathered every time, and never a cached refusal
    /// standing in for one the gather would re-derive.
    #[cfg(debug_assertions)]
    #[test]
    fn a_refusing_gather_is_not_memoized() {
        let doc = sketch_only("memo-refusal");
        let ev = evaluated(&doc);
        let tol = Tol::witness();
        let memo = ProductMemo::default();
        let count = gathers(|| {
            for _ in 0..2 {
                let refusal = product_memo::body(&memo, &doc, &ev, tol)
                    .expect_err("a document with no body root has no product");
                assert!(matches!(refusal, d::ProductError::NoBodyRoots));
            }
        });
        assert_eq!(count, 2);
    }

    /// The gather's refusal is a SUBJECT the residents report over,
    /// not an error — the posture `editor_core::run_checks` takes, kept
    /// by the memo path that replaces it.
    #[test]
    fn a_gather_refusal_reaches_the_registry_as_a_subject() {
        let doc = sketch_only("memo-refusal-subject");
        let ev = evaluated(&doc);
        let memo = ProductMemo::default();
        let report = product_memo::checks_report(
            &memo,
            &doc,
            &ev,
            &d::ChecksConfig::default(),
            Tol::witness(),
        )
        .expect("no body roots is a subject, not a refusal");
        assert!(report.findings.is_empty());
    }

    /// A product is the answer for the tolerance it was gathered at,
    /// and the memo says so: asked at another, it gathers again and
    /// keeps the entry it has. Driven through the keyed seam because a
    /// process commits exactly ONE tolerance and cannot offer a second
    /// (`Tol` is the witness of that commitment).
    #[cfg(debug_assertions)]
    #[test]
    fn a_different_tolerance_gathers_again() {
        let doc = box_doc("memo-tolerance-key");
        let ev = evaluated(&doc);
        let tol = Tol::witness();
        let at = tol.get();
        let other = pncad::tolerance::Tolerance {
            eps: at.eps * 2.0,
            ..at
        };
        let memo = ProductMemo::default();
        let count = gathers(|| {
            memo.with_at(&doc, &ev, at, tol, |_| ())
                .expect("the gather succeeds");
            memo.with_at(&doc, &ev, at, tol, |_| ())
                .expect("and is memoized");
            memo.with_at(&doc, &ev, other, tol, |_| ())
                .expect("a second tolerance is a second question");
            memo.with_at(&doc, &ev, at, tol, |_| ())
                .expect("and the held entry is still the held entry");
        });
        assert_eq!(count, 2);
    }

    /// The DI3 pairing gate, asked BEFORE the memo is consulted: a
    /// memo answers without reaching a gather, so the refusal a gather
    /// would have raised has to be raised here or not at all.
    #[test]
    fn a_mispaired_document_is_refused_before_the_memo_is_consulted() {
        let doc = box_doc("memo-paired");
        let other = box_doc("memo-paired-other");
        let ev = evaluated(&doc);
        assert!(ProductMemo::paired(&ev, doc.id()).is_ok());
        let mispaired = ProductMemo::paired(&ev, other.id()).expect_err("a foreign document");
        assert_eq!(mispaired.expected, other.id());
        assert_eq!(mispaired.found, doc.id());
    }
}
