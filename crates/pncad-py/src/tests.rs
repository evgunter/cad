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
use pncad::topo::{FaceKey, SolidKey, VertexKey};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
// The shared Rust-source lexer: `src/tags.rs` is READ by the tag-table
// guard below, and a synthetic fixture beside it is read by the
// guard's own guard. This is the tree's one answer to "is this text
// code, prose or a literal"; `crates/test-utils/tests/reader_census.rs`
// carries the line that says so.
use test_utils::source::{
    ItemBody, balanced_end, boundary_after, boundary_before, code_and_literals, code_only,
    comments_only, ident, impl_head, item_body, line_start, plain_string_literal, skip_ws,
    type_base,
};

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
///
/// The list is the kernel's own [`Dimension::ALL`] and not a copy of
/// it here: a dimension added to the lattice is pinned by this row
/// without an edit, where a list written here would leave the row
/// green over the dimensions it happened to name.
#[test]
fn dimension_tags_match_the_kernel_prose() {
    for dim in Dimension::ALL {
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

/// **The two dimension alphabets are one list in two cases.**
///
/// `Measurement.dimension` answers the capitalized spelling and every
/// other door answers the lower-case one, so two Python attributes
/// with the same name carry two spellings of one four-word list. That
/// is allowed and is not free: the day one list gains a word or
/// renames one, the other has to move with it, and nothing but this
/// says so.
///
/// Over the kernel's own [`Dimension::ALL`], so a dimension added to
/// the lattice is pinned without an edit here.
#[test]
fn the_two_dimension_alphabets_are_one_list_in_two_cases() {
    use crate::errors::measurement_dimension_tag;

    for dim in Dimension::ALL {
        let mut capitalized = dimension_tag(dim).to_owned();
        capitalized[..1].make_ascii_uppercase();
        assert_eq!(
            measurement_dimension_tag(dim),
            capitalized,
            "the measurement spelling of a dimension is no longer the FFI tag \
             capitalized"
        );
    }
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
            ErrorClass::Evaluation(_) => "EvaluationError",
            ErrorClass::Validation(_) => "ValidationError",
            ErrorClass::QuantityOp => "QuantityOpMismatch",
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
        // The two classes with a payload: the word is the same for
        // every reason, and `eval_reason_tag` and
        // `validation_refusal_tag` are what pin the reasons themselves.
        ErrorClass::Evaluation(crate::errors::EvalReason::NodeFailed),
        ErrorClass::Validation(crate::errors::ValidationRefusal::Validate),
        ErrorClass::QuantityOp,
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
/// through a real document. `ClearanceRefusal` is the interval
/// engine's own, and its ONLY producer on the measure path is
/// `impl MinClearanceLane for geom_core::Interval`; no Python
/// evaluation reaches it, because the binding evaluates at `f64` and the
/// lane is what gates it. So this row is where the second one's tag and
/// prose are pinned at all.
#[test]
fn the_fourth_verbs_two_refusals_are_stable() {
    use crate::tags::{measure_unavailable_at_tag, node_error_tag};
    use pncad::document::{ClearanceRefusal, MeasureUnavailableAt, NodeErrorKind};

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

    // The four arms `clearance::min_separation` refuses with — the
    // only ones this carrier's producer can build — all cross as one
    // tag.
    let refused = |r| NodeErrorKind::MeasureClearanceRefused(r);
    let empty = refused(ClearanceRefusal::EmptyScope);
    let unpaired = refused(ClearanceRefusal::NoAdmittedPair);
    let unsupported = refused(ClearanceRefusal::Unsupported {
        carrier: "a free-form face",
        face: FaceKey::default(),
    });
    let poison = refused(ClearanceRefusal::PoisonEnclosure {
        a: FaceKey::default(),
        b: FaceKey::default(),
    });
    for e in [&empty, &unpaired, &unsupported, &poison] {
        assert_eq!(node_error_tag(e), "measure_clearance_refused", "{e}");
    }
    // The prose is pinned on the two arms whose rendering is a
    // sentence. `Unsupported` and `PoisonEnclosure` print their faces
    // as `FaceKey` `Debug` — the defect
    // work/props/props-refusal-prose-outgrows-the-viewer.md owns — and
    // `reads_as_prose` does not look for an arena key, so a pin there
    // would pass and prove nothing.
    for e in [&empty, &unpaired] {
        assert!(crate::errors::reads_as_prose(&e.to_string()), "{e}");
    }
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

    // DI3's pairing refusal, under the word the gather, the checks and
    // the name-level edit door already answer with: one fact, one tag,
    // whichever door a caller meets it at.
    assert_eq!(
        hit_test_error_tag(&H::EvaluationOfAnotherDocument {
            expected: pncad::document::DocumentId::derive("tag-expected"),
            found: pncad::document::DocumentId::derive("tag-found"),
        }),
        "evaluation_of_another_document"
    );

    // The certified tie between faces, under the word the name-level
    // interrogation already answers with for "this denotes more than
    // one thing".
    assert_eq!(
        hit_test_error_tag(&H::Ambiguous { hits: Vec::new() }),
        "ambiguous"
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
            node_pick_error_tag(&N::Standing(standing.clone())),
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

/// **Every `MateFault` arm's payload, built and read.**
///
/// The arm table, executable and TOTAL: all thirteen arms are built
/// here and every field each carries is read.
/// `crate::mate_payload::mate_payload` is the projection
/// `MateFault`'s Python attributes are read off, and this
/// pin says what each arm puts on the wire: the exact set it CARRIES,
/// in publication order, with the rest `None`.
///
/// Totality of the TABLE and totality of the PROJECTION are two
/// guarantees and this file holds both. The projection's is the
/// stronger and the older: `mate_payload`'s match is exhaustive with
/// no wildcard, so an arm that reached Python unprojected would not
/// compile. The table's is that every arm's field set is executed —
/// which needs every payload type nameable from this crate, and is
/// why `LeverRefusal` is curated at `pncad::document` beside the
/// refusal that holds it.
#[test]
fn every_mate_fault_arm_projects_the_payload_it_carries() {
    use crate::mate_payload::mate_payload;
    use pncad::document::{
        Clash, DocumentId, Lever, LeverRefusal, MateFault as F, MateSide, NodeErrorKind,
        NodeRefusal, RecipeNodeId, Subgroup,
    };
    use pncad::geom_core::{
        Band, BandError, BandField, FrameError, FrameInput, FrameVector, Indeterminate, MarginDiag,
        UnitVec3,
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
    // payload: the subject is not a mate, and both ids cross so a
    // caller compares them against `Doc.id` rather than reading them
    // out of the prose.
    carries(
        &F::PosesOfAnotherDocument {
            expected: DocumentId::derive("a"),
            found: DocumentId::derive("b"),
        },
        &["expected_document", "found_document"],
    );
    carries(&F::ClassNotAdmitted { mate: id(1) }, &["mate"]);
    // The four arms whose payload is a NESTED refusal. Each crosses
    // under the inner refusal's own word, with the numbers that word
    // qualifies beside it — the frame door's vocabulary, spelled the
    // same way here.
    let band = Band::new(1.0e-9, 1.0e-6).expect("a valid band");
    let escalated = |predicate| Indeterminate {
        margin: MarginDiag::Value(2.0e-9),
        band,
        predicate,
    };
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::A,
            error: FrameError::Degenerate {
                input: FrameInput::Aim,
                indeterminate: Some(escalated(Some("frame_aim_definite"))),
            },
        },
        &[
            "mate",
            "side",
            "predicate",
            "inner_variant",
            "margin",
            "zero",
            "escalate",
        ],
    );
    // A definite zero refused outright rather than landing in the
    // band, so there is no classification to publish.
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::B,
            error: FrameError::Degenerate {
                input: FrameInput::Tangent,
                indeterminate: None,
            },
        },
        &["mate", "side", "inner_variant"],
    );
    // The same empty payload for a non-finite length, and for a
    // sharper reason: nothing was CLASSIFIED, so there is no margin
    // to publish rather than a margin that happened to be definite.
    // `MateFrame::placement` calls `point_at` directly, so both of
    // that door's non-finite words reach Python through this arm —
    // the roll-reference one is pinned here because it is the site
    // the unit found silently returning a degenerate frame.
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::B,
            error: FrameError::NonFiniteLength {
                input: FrameVector::RollReference,
            },
        },
        &["mate", "side", "inner_variant"],
    );
    assert_eq!(
        mate_payload(&F::Frame {
            mate: id(1),
            side: MateSide::B,
            error: FrameError::NonFiniteLength {
                input: FrameVector::RollReference,
            },
        })
        .inner_variant,
        Some("non_finite_roll_reference"),
    );
    // An enclosure straddles rather than landing: two bounds and no
    // value, the `MarginDiag` fork the frame door publishes.
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::A,
            error: FrameError::Degenerate {
                input: FrameInput::RollReference,
                indeterminate: Some(Indeterminate {
                    margin: MarginDiag::Enclosure {
                        lo: -1.0e-9,
                        hi: 3.0e-9,
                    },
                    band,
                    predicate: None,
                }),
            },
        },
        &[
            "mate",
            "side",
            "inner_variant",
            "margin_low",
            "margin_high",
            "zero",
            "escalate",
        ],
    );
    // A poisoned margin is the absence of a number, not a number:
    // the band still crosses.
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::A,
            error: FrameError::Degenerate {
                input: FrameInput::MirrorNormal,
                indeterminate: Some(Indeterminate {
                    margin: MarginDiag::Invalid,
                    band,
                    predicate: None,
                }),
            },
        },
        &["mate", "side", "inner_variant", "zero", "escalate"],
    );
    // The frame ladder's own band refusal, two levels down:
    // `inner_variant` is one level in — the word `FrameError` crosses
    // under — and the band's payload is what the numbers say.
    carries(
        &F::Frame {
            mate: id(1),
            side: MateSide::B,
            error: FrameError::Band(BandError::InvalidValue {
                field: BandField::Escalate,
                value: f64::INFINITY,
            }),
        },
        &["mate", "side", "inner_variant", "field", "value"],
    );
    carries(
        &F::Band {
            error: BandError::InvalidValue {
                field: BandField::Zero,
                value: 0.0,
            },
        },
        &["inner_variant", "field", "value"],
    );
    carries(
        &F::Band {
            error: BandError::InvalidLeverArm { value: -1.0 },
        },
        &["inner_variant", "value"],
    );
    carries(
        &F::Band {
            error: BandError::Empty {
                zero: 1.0e-6,
                escalate: 1.0e-9,
            },
        },
        &["inner_variant", "zero", "escalate"],
    );
    // A struct, not an enum: the escalation has no inner WORD, and
    // its shape is which margin attribute is set.
    carries(
        &F::Indeterminate {
            mate: id(1),
            diag: Box::new(escalated(Some("mate_clocking_redundant"))),
        },
        &["mate", "predicate", "margin", "zero", "escalate"],
    );
    carries(
        &F::Unleverable {
            mate: id(1),
            refusal: LeverRefusal::PartUnresolved {
                instance: id(0),
                fault: pncad::document::PartFault::NoResolver,
            },
        },
        &["mate", "instance", "inner_variant"],
    );
    // A levered clash carries both halves of the lever beside it —
    // the tilt for an authored roll, the residual for a pure number
    // — and one measured without a lever carries none of the three.
    carries(
        &F::Contradictory {
            held: id(1),
            added: id(1),
            predicate: "mate_clocking_redundant",
            clash: Clash::Levered(Lever::Roll {
                radians: 0.25,
                arm: 2.0,
            }),
        },
        &[
            "held",
            "added",
            "predicate",
            "clash",
            "lever_tilt",
            "lever_arm",
        ],
    );
    carries(
        &F::Contradictory {
            held: id(1),
            added: id(2),
            predicate: "mate_member_axis_fixed",
            clash: Clash::Levered(Lever::Residual {
                value: 0.25,
                arm: 2.0,
            }),
        },
        &[
            "held",
            "added",
            "predicate",
            "clash",
            "lever_residual",
            "lever_arm",
        ],
    );
    // A length measured outright carries the clash and no lever; the
    // structural refusal measures nothing and carries no clash.
    carries(
        &F::Contradictory {
            held: id(1),
            added: id(2),
            predicate: "mate_member_translation_zero",
            clash: Clash::Length { metres: 0.01 },
        },
        &["held", "added", "predicate", "clash"],
    );
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
            clash: Clash::Structural,
        },
        &["held", "added", "predicate"],
    );
    carries(
        &F::Under {
            mate: id(1),
            parent: id(2),
            child: id(3),
            residual: Subgroup::Planar {
                normal: UnitVec3::new(
                    pncad::authoring::v3(0.0, 0.0, 1.0),
                    "pncad_py_test_normal",
                    Band::new(1e-9, 1e-8).expect("a band"),
                )
                .expect("a unit normal"),
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

    // The two ids answer the two ROLES, not one id twice: which
    // document was asked for and which one the solve is of is the
    // whole of what a mispaired read reports.
    let mispaired = F::PosesOfAnotherDocument {
        expected: DocumentId::derive("asked"),
        found: DocumentId::derive("solved"),
    };
    let payload = mate_payload(&mispaired);
    assert_eq!(payload.expected_document, Some(DocumentId::derive("asked")));
    assert_eq!(payload.found_document, Some(DocumentId::derive("solved")));

    // The nested refusals' words are the tag maps' own, so a caller
    // branches on one vocabulary whether the refusal reached them
    // from this door or from the door that owns it.
    let frame_band = F::Frame {
        mate: id(1),
        side: MateSide::A,
        error: FrameError::Band(BandError::InvalidLeverArm { value: 0.0 }),
    };
    assert_eq!(mate_payload(&frame_band).inner_variant, Some("band"));
    assert_eq!(mate_payload(&frame_band).value, Some(0.0));
    let degenerate = F::Frame {
        mate: id(1),
        side: MateSide::A,
        error: FrameError::Degenerate {
            input: FrameInput::Aim,
            indeterminate: None,
        },
    };
    assert_eq!(
        mate_payload(&degenerate).inner_variant,
        Some("degenerate_aim")
    );
    let unleverable = F::Unleverable {
        mate: id(1),
        refusal: LeverRefusal::PartUnresolved {
            instance: id(0),
            fault: pncad::document::PartFault::NoResolver,
        },
    };
    let payload = mate_payload(&unleverable);
    assert_eq!(payload.inner_variant, Some("part_unresolved"));
    assert_eq!(payload.instance, Some(id(0)));

    // The classifier's numbers are the ones the band and the margin
    // held, on the frame door's own names.
    let escalation = F::Indeterminate {
        mate: id(1),
        diag: Box::new(escalated(Some("mate_member_empty"))),
    };
    let payload = mate_payload(&escalation);
    assert_eq!(payload.margin, Some(2.0e-9));
    assert_eq!(
        (payload.zero, payload.escalate),
        (Some(1.0e-9), Some(1.0e-6))
    );
    assert_eq!(payload.predicate, Some("mate_member_empty"));
    assert_eq!((payload.margin_low, payload.margin_high), (None, None));

    // **`clash` IS the product of the lever's two halves.** The
    // kernel computes the deviation at the raising site and stores
    // it; the two halves ride beside it, and a caller multiplying
    // them gets the number it was handed. A roll sets the tilt and a
    // residual the residual; the other half is `None`, and which is
    // set is how the kind crosses.
    let levered = F::Contradictory {
        held: id(1),
        added: id(1),
        predicate: "mate_clocking_redundant",
        clash: Clash::Levered(Lever::Roll {
            radians: 0.25,
            arm: 2.0,
        }),
    };
    let payload = mate_payload(&levered);
    let (tilt, arm) = (
        payload
            .lever_tilt
            .expect("a levered clash carries its tilt"),
        payload.lever_arm.expect("and its arm"),
    );
    assert_eq!(payload.clash, Some(tilt * arm));
    assert_eq!(payload.lever_residual, None);
    let levered = F::Contradictory {
        held: id(1),
        added: id(2),
        predicate: "mate_rotation_two_axis_reachable",
        clash: Clash::Levered(Lever::Residual {
            value: 0.75,
            arm: 3.0,
        }),
    };
    let payload = mate_payload(&levered);
    let (residual, arm) = (
        payload
            .lever_residual
            .expect("a residual clash carries its pure number"),
        payload.lever_arm.expect("and its arm"),
    );
    assert_eq!(payload.clash, Some(residual * arm));
    assert_eq!(payload.lever_tilt, None);
    // The structural refusal measures nothing: no clash and none of
    // the three halves — `None` rather than zeroes claiming a
    // measurement nothing made.
    let structural = F::Contradictory {
        held: id(1),
        added: id(2),
        predicate: "mate_member_empty",
        clash: Clash::Structural,
    };
    let payload = mate_payload(&structural);
    assert_eq!(
        (
            payload.clash,
            payload.lever_tilt,
            payload.lever_residual,
            payload.lever_arm
        ),
        (None, None, None, None)
    );
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
/// this crate's own — [`crate::errors::EvalReason`], mapped by
/// [`crate::tags::eval_reason_tag`]. Three maps, one word between
/// them, and this is the pin that keeps them saying it.
///
/// It runs in BOTH directions on purpose: renaming the kernel arms'
/// tag fails here, and so does editing this door's arm away from
/// them. That is the property `picking_refusal_tags_are_stable`
/// protects for the pick, one door further out.
#[test]
fn the_evaluation_door_speaks_the_standing_ladder() {
    use crate::errors::EvalReason;
    use crate::tags::{eval_reason_tag, hit_test_error_tag, interrogate_error_tag};
    use pncad::document::RecipeNodeId;
    use pncad::select::{HitTestError as H, InterrogateError as I};

    let node = RecipeNodeId(0);
    let rung = eval_reason_tag(EvalReason::NodeNotEvaluated);
    assert_eq!(rung, hit_test_error_tag(&H::NodeNotEvaluated { node }));
    assert_eq!(rung, interrogate_error_tag(&I::NodeNotEvaluated { node }));

    // And it is NOT the other no-entry fact. "The document has no such
    // node" and "this run never reached it" are two states the door
    // kept collapsed while only one of them could arise, and the whole
    // of what B-CANCEL changed at this door is that both now can.
    assert_ne!(rung, eval_reason_tag(EvalReason::UnknownNode));
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
        let applied = apply(
            doc,
            &DocEdit::InsertNode { node },
            tol,
            &pncad::document::RefusingReach,
        )
        .expect("the node inserts");
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
    let pruned = apply(
        &doc,
        &DocEdit::DeleteNode { id: extrude },
        tol,
        &pncad::document::RefusingReach,
    )
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
    use pncad::geom_core::{BandError, BandField};
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
    // The band arm delegates: a Python caller branching on `reason`
    // gets the constructor's own word, so a collapsed band and an
    // overflowed one are two answers rather than one.
    assert_eq!(
        select_refusal_tag(&SelectRefusal::Band(BandError::Empty {
            zero: 5e-324,
            escalate: 5e-324,
        })),
        "empty"
    );
    assert_eq!(
        select_refusal_tag(&SelectRefusal::Band(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        })),
        "invalid_value"
    );
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
/// `the_load_door_reaches_dimension_mismatch_arms_as_a_typed_dimension_refusal`
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

    // The reachable set, exhaustively: every dimension the kernel
    // names (`Dimension::ALL`, so "exhaustively" is a claim about the
    // lattice and not about a list copied here), a finite and a
    // non-finite value each. Nothing here is a dimension MISMATCH,
    // which is what makes `LiteralError` the right class.
    let mut reachable = std::collections::BTreeSet::new();
    for dim in Dimension::ALL {
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
            &pncad::document::RefusingReach,
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
/// They arrive as `PersistError` with `variant == "dimension"` and the
/// failing check's own tag as `inner_variant`, which is what this pins:
/// the STRUCTURE crosses, not a sentence about it, and the word comes
/// from `expr_dimension_error_tag` — the same map the expression text
/// door draws `ParseError.kind` from.
///
/// The class names the DOOR rather than the type, as it does at the
/// other two: a save file's dimension mismatch is not a literal-value
/// refusal and it is not the quantity boundary's operator check, and
/// the branchable fact — which check refused — rides beside the stage
/// in one vocabulary instead of being split across three.
#[test]
fn the_load_door_reaches_dimension_mismatch_arms_as_a_typed_dimension_refusal() {
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
        &pncad::document::RefusingReach,
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
        &pncad::document::RefusingReach,
    )
    .expect("the profile inserts");
    let text = save(&applied.doc, &[], tol).expect("the document saves");
    let (header, body) = text.split_once("\n{").expect("a header line then the body");
    let body = format!("{{{body}");
    let saved: serde_json::Value = serde_json::from_str(&body).expect("the save body is JSON");

    // Every case replaces the FIRST literal in the document, so this
    // is driven by the wire SHAPE rather than by a node id.
    // The `unit` field is REQUIRED on the wire, and a literal written
    // without one refuses as a missing field before the rebuild runs at
    // all — which is a refusal about the schema, not about dimensions,
    // and would make every case below prove the wrong thing.
    let length = serde_json::json!({ "Literal": { "value": 1.0, "dim": "Length", "unit": "m" } });
    let angle = serde_json::json!({ "Literal": { "value": 1.0, "dim": "Angle", "unit": "rad" } });
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
            "dimension",
            "{arm}: the load path's dimension refusal must reach the door \
             as its own arm, not as vocabulary this build lacks"
        );
        let pncad::document::PersistError::Dimension { error, .. } = &err else {
            panic!("{arm}: the tag says dimension but the arm does not: {err:?}")
        };
        assert_eq!(
            expr_dimension_error_tag(error),
            arm,
            "{arm}: the structured refusal crosses whole, so WHICH check \
             failed is branchable from Python"
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
/// arm buildable without geometry: the op family at its f64 witness
/// and the lane refusal.
///
/// **Two arms are driven through a real document instead**, in
/// `tests/test_shell.py`, and for one reason in two forms: their
/// payloads are not this layer's to mint. `shell_open_resolve` carries
/// a `ResolveError`, whose constructors belong to the document layer;
/// `shell_open_kind` carries the entity door's `Found`, which has a
/// private field and is mintable only inside
/// `editor_core::eval::entity_door` — a refusal that says what an
/// entity turned out to be cannot be assembled by anything that did
/// not resolve one. `test_an_edge_in_the_open_list_refuses_typed`
/// reaches it through a real edge and asserts the same tag.
#[test]
fn shell_refusal_tags_are_stable() {
    use crate::tags::node_error_tag;
    use pncad::document::NodeErrorKind;
    use pncad::topo::ShellError;
    let op = NodeErrorKind::Shell(Box::new(ShellError::Thickness { thickness: -0.5 }));
    assert_eq!(node_error_tag(&op), "shell");
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
        pair(&NodeErrorKind::Tube(Box::new(TubeError::DegenerateWindow))),
        ("tube", Some("degenerate_window"))
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

/// **A frame's direction refusal, carried to the node that read it,
/// keeps the word the frame's own raise answers.**
///
/// `NodeErrorKind::FrameDirection` exists to add the frame's ID to a
/// refusal a reader raises about ANOTHER node — the fact is
/// unchanged, so the tag is unchanged, and a Python caller matching
/// `degenerate_direction` keeps matching after the locator lands.
/// That is a claim about the MAPPING, which the inventory cannot
/// make: swap this arm for a freshly minted word and the inventory
/// still has to be edited, but nothing would say the edit broke every
/// caller of the old one.
///
/// All four of the direction door's facts, because the arm delegates
/// and a delegation that answered one fixed word for all of them
/// would pass a one-row pin.
#[test]
fn a_carried_frame_direction_refusal_keeps_the_frames_own_tag() {
    use crate::tags::{node_error_tag, node_inner_kind_tag};
    use pncad::document::{DirectionRefusal, NodeErrorKind, RecipeNodeId};
    use pncad::geom_core::{Band, Indeterminate, MarginDiag, UnitVec3Error};

    let band = Band::new(1.0e-9, 1.0e-6).expect("a valid band");
    let carried = |error| NodeErrorKind::FrameDirection {
        profile: RecipeNodeId(7),
        frame: RecipeNodeId(3),
        refusal: DirectionRefusal {
            role: "datum frame x axis",
            error,
        },
    };
    // The WORD per fact, written down. Comparing the two sides alone
    // would stay green if `wire::refusal` mapped `Degenerate` onto
    // `NonFiniteDirection`: both sides move together, so only a
    // literal catches a re-pointed arm.
    for (error, word) in [
        (UnitVec3Error::Degenerate, "degenerate_direction"),
        (UnitVec3Error::NonFiniteLength, "non_finite_direction"),
        (UnitVec3Error::UnderflowedLength, "underflowed_direction"),
        (
            UnitVec3Error::Escalated(Indeterminate {
                margin: MarginDiag::Value(2.0e-9),
                band,
                predicate: Some("datum_unit_norm"),
            }),
            "escalated",
        ),
    ] {
        let direct = DirectionRefusal {
            role: "datum frame x axis",
            error,
        }
        .node_error();
        assert_eq!(
            node_error_tag(&carried(error)),
            word,
            "the carried refusal stopped answering the word this fact has \
             always answered, so every Python caller matching it breaks"
        );
        assert_eq!(
            node_error_tag(&carried(error)),
            node_error_tag(&direct),
            "the carried refusal and the frame's own raise have diverged"
        );
        // Compared, not pinned: today both are `None`, and if the
        // direction family ever projects an inner discriminant, the
        // carried road must project the same one rather than keeping
        // a `None` this row froze in.
        assert_eq!(
            node_inner_kind_tag(&carried(error)),
            node_inner_kind_tag(&direct),
            "the carried road projects a different inner tag from the frame's own"
        );
    }

    // And the two ids the arm exists for reach the prose.
    let shown = carried(UnitVec3Error::Degenerate).to_string();
    assert!(
        shown.contains("node 7") && shown.contains("node 3"),
        "the arm names the profile that read and the frame that refused: {shown}"
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
/// what each of the 68 arms puts on the wire: the exact set of
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
/// The pin is TOTAL over the enum: all 68 arms are built here, so an
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
        DocumentId, EditError as E, ExprPath, Frame, MeasureNodeFault, MetaVersionError, ParamName,
        ProvenanceFault, RecipeNodeId, RootFault, SlotId,
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
    carries(&E::SetProgramOnNonProfile { node: id(1) }, &["node"]);
    carries(
        &E::ProvenanceMalformed {
            node: id(1),
            fault: ProvenanceFault::LoopCount {
                loops: 1,
                provenance: 2,
            },
        },
        &["node"],
    );
    carries(&E::WitnessOnNonSketch { node: id(1) }, &["node"]);
    carries(&E::DuplicateWitnessEntry { node: id(1) }, &["node"]);
    carries(&E::PlacementOnNonInstance { node: id(1) }, &["node"]);
    carries(&E::PlacementRuleMismatch { node: id(1) }, &["node"]);
    carries(&E::EmptyPlacementList { node: id(1) }, &["node"]);
    carries(&E::NonFinitePlacement { node: id(1) }, &["node"]);
    carries(&E::NonFiniteAlignment { node: id(1) }, &["node"]);
    // The door's per-mate admission carries the solve's fault WHOLE
    // beside the mate: the one payload that crosses as a value.
    carries(
        &E::MateRefused {
            node: id(1),
            fault: Box::new(pncad::document::MateFault::TableLacks {
                mate: id(1),
                what: "a clocking rider on a planar rest",
            }),
        },
        &["node", "fault"],
    );
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
        &E::PayloadUnknownDocParam {
            name: param(),
            node: id(1),
        },
        &["node", "param"],
    );
    carries(
        &E::PayloadDocParamDimension {
            name: param(),
            node: id(1),
            declared: Dimension::Length,
            referenced: Dimension::Angle,
        },
        &["node", "param", "expected", "found"],
    );
    carries(
        &E::SlotUnknownDocParam {
            name: param(),
            node: id(1),
            slot: SlotId::Count,
        },
        &["node", "slot", "param"],
    );
    carries(
        &E::SlotDocParamDimension {
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
    carries(
        &E::DocParamNotDeclared {
            name: param(),
            door: pncad::document::CarryForwardDoor::Value,
        },
        &["param"],
    );
    carries(
        &E::NonFiniteDocParam {
            name: param(),
            field: pncad::document::DocParamField::Nominal,
        },
        &["param"],
    );
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
    // The sorted designation's fault reports ONE position, so it
    // carries `first` and not `again`.
    carries(
        &E::SelectionNotCanonical { node: id(1), at: 2 },
        &["node", "first"],
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
            refusal: Box::new(pncad::document::ProgramRefusal::Validate(
                pncad::profile::ProfileError::EmptyProfile,
            )),
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
    // Two DOCUMENTS, which no attribute of this record carries — the
    // `ProductError` arm's precedent one door over: the message states
    // both ids.
    carries(
        &E::EvaluationOfAnotherDocument {
            expected: DocumentId::derive("edited"),
            found: DocumentId::derive("handed"),
        },
        &[],
    );
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
    assert_eq!(promoted_curve_kind_tag(&PromotedCurveKind::Line), "line");
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
    // word, which no sentence is — underscored variant names included,
    // which is the one character the predicate's alphabet spells.
    assert!(!reads_as_prose("SeamRetrimsArcFirstSide"));
    assert!(!reads_as_prose("Seam_Retrims_Arc_First_Side"));
    // And the shapes prose legitimately carries: a quoted user string
    // (`Debug` on a `&str`, which the id doors use for its escaping),
    // and a sentence that opens on a capital.
    assert!(reads_as_prose(
        "not a document id: \"nope\" — an id is 32 hex digits"
    ));
    assert!(reads_as_prose("Tessellate refused"));
}

/// A blend escalation names its site in prose at every site.
///
/// `BlendSite::Link` and `::Joint` are struct variants, so rendering
/// the site through `Debug` puts the field-brace fingerprint in the
/// message and the refusal PANICS `crate::py::typed_err` instead of
/// raising. The escalation arm is the one an indeterminate predicate
/// is for, so that panic sits behind an ordinary fillet or chamfer
/// request; the site renders through its own `Display`, and this is
/// the rendering that says so.
#[test]
fn a_blend_escalation_reads_as_prose_at_every_site() {
    use pncad::prelude::{Band, BlendError, BlendSite, EdgeKey, Indeterminate};
    use pncad::prelude::{MarginDiag, VertexKey};

    let band = Band::new(1e-9, 1e-6).expect("a band");
    for site in [
        BlendSite::Link {
            edge: EdgeKey::default(),
        },
        BlendSite::Joint {
            vertex: VertexKey::default(),
        },
        BlendSite::Chain,
    ] {
        let refused = BlendError::Escalated {
            site,
            source: Indeterminate {
                margin: MarginDiag::Value(0.0),
                band,
                predicate: Some("fillet3_radius_headroom"),
            },
        };
        let text = refused.to_string();
        assert!(
            reads_as_prose(&text),
            "a fillet or chamfer escalation at {site:?} panics the binding \
             rather than raising: {text}"
        );
        assert!(
            text.starts_with("escalated at ") && !text.contains("Key("),
            "the site names itself after the preposition the sentence supplies, \
             and no arena key: {text}"
        );
    }
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
            kind: Some(pncad::document::ProductErrorKind::NoBodyRoots),
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
    carries(&unavailable, &["reason", "boolean_variant"]);

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
    // The boolean class beside the sentence, and the two words are
    // read off the same evidence: a consumer branching on this one
    // never parses the prose to learn which refusal it was.
    assert_eq!(
        check_payload(&unavailable).boolean_variant,
        Some("classification_invariant")
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
/// through five `Body` methods — the four rungs of the ladder and
/// `validate_geometric_measured`, whose gate half is the third rung —
/// so most of the enum is unreachable
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
    use pncad::topo::{
        CensusContact, CensusSubject, CensusUnsupportedCause, ContactRefusal, EntityId,
        ValidationError,
    };

    // The arm whose subject is ONE entity: the recourse is that
    // carrier's, so the kind of carrier is the word a caller acts on.
    assert_eq!(
        project(&ValidationError::CensusUnsupported {
            subject: CensusSubject::Entity(EntityId::Edge(Default::default())),
            // The cause is threaded, not read: this crate projects
            // the SUBJECT and has no word for the cause yet, which is
            // LIB's row. `what` is production's own string, from
            // `topo::boolean::contact_verify`'s Rest-ladder arm.
            cause: CensusUnsupportedCause::ContactLane(ContactRefusal::NotCertifiable {
                what: "a declared face's surface kind is outside the Rest ladder's \
                       inventory (plane, sphere, cylinder)",
            }),
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

    // An arm whose payload projects to no word at all, and the shape
    // every such arm takes: the variant alone, five `None`s beside it,
    // so `getattr` never raises on a finding a caller did not expect.
    // Check 7's subject is a solid, and a solid key is not a word.
    assert_eq!(
        project(&ValidationError::NegativeVolume {
            solid: SolidKey::default(),
        }),
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
    assert_eq!(
        project(&ValidationError::NegativeVolume {
            solid: SolidKey::default(),
        })
        .stale_kind,
        None
    );
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
    assert_eq!(
        word(RingContact::OuterVertexOnEdge {
            outer_vertex: VertexKey::default(),
            ring_edge: Default::default(),
        }),
        Some("vertex_on_ring_edge")
    );
    assert_eq!(
        word(RingContact::EdgesMeet {
            ring_edge: Default::default(),
            outer_edge: Default::default(),
        }),
        Some("edge_edge_point")
    );
    assert_eq!(
        word(RingContact::Circles {
            ring_loop: Default::default(),
            outer_loop: Default::default(),
        }),
        Some("circle_circle")
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

/// **`ValidationRefusal::ALL` is the enum, and the inventory is what
/// says so.**
///
/// The roster exists so the class's attribute set can be derived from
/// the enum rather than restated, which is worth nothing if the roster
/// can fall behind the enum. Nothing in Rust enumerates a plain enum's
/// variants, so this borrows the instrument that already reads the
/// tree: [`TAG_INVENTORY`]'s `validation_refusal_tag` row is compared
/// against `src/tags.rs` by
/// [`the_whole_tag_table_matches_its_committed_inventory`], so a sixth
/// refusal is an arm in that map (a compile error until it is
/// written), then a word in the inventory (a red row until it is
/// named), and then a member of the roster — a red row HERE until it
/// joins.
///
/// What it cannot see: a refusal added to the enum and to the map with
/// the inventory row updated in the same change and the roster left
/// alone would red here and nowhere else, which is the point; a
/// refusal added to the enum alone does not compile.
#[test]
fn validation_refusals_are_the_roster_the_inventory_reads() {
    let entry = TAG_INVENTORY
        .iter()
        .find(|entry| entry.function == "validation_refusal_tag")
        .expect("the inventory carries the validation class's vocabulary");
    let mut from_roster: Vec<&str> = crate::errors::ValidationRefusal::ALL
        .iter()
        .map(|refusal| crate::tags::validation_refusal_tag(*refusal))
        .collect();
    from_roster.sort_unstable();
    assert_eq!(
        from_roster, entry.values,
        "`ValidationRefusal::ALL` and the map's inventoried words have drifted \
         apart — the roster is missing a refusal the enum has, or names one it \
         does not"
    );
}

/// **The `ValidationError` class mints onto exactly two attributes.**
///
/// [`crate::errors::ValidationRefusal::ATTRIBUTES`] is what
/// `crate::py::typed_err` asserts a raise site did not spell, so it has
/// to be the whole image of `attribute()` — a third attribute missing
/// from it would be a Python-visible word a raise site could still pass
/// unnoticed, which is the gap the set replaced a single word to close.
/// Both directions, so a stale member cannot sit there either.
#[test]
fn the_validation_class_mints_exactly_these_attributes() {
    use crate::errors::ValidationRefusal;
    let written: BTreeSet<&str> = ValidationRefusal::ALL
        .iter()
        .map(|refusal| refusal.attribute())
        .collect();
    assert_eq!(
        written.into_iter().collect::<Vec<_>>(),
        ValidationRefusal::ATTRIBUTES,
        "the attributes the refusals actually write are not the set the raise \
         door is asserted against"
    );
}

/// **Which attribute each validation refusal's word lands on**,
/// committed.
///
/// `attribute()` is the one thing deciding whether a refusal's word
/// reaches Python as `ValidationError.door` or as
/// `ValidationError.reason`, and moving a variant between its arms
/// changes a published contract while every other guard on this page
/// stays green: the word is still minted from the map, still
/// inventoried, still spelled once. Two of the five are held from the
/// Python side (`tests/test_validate.py` reads `door` on
/// `validate_pseudomanifold` and `reason` on `mass_properties_failed`)
/// and three are reachable from no Python test, because a body the
/// kernel produces passes the first three rungs. So the routing is
/// pinned here instead, for all five.
///
/// **This pins; it does not close.** A sixth refusal routed to the
/// wrong attribute is caught by the row it has to add here, not by the
/// compiler.
const VALIDATION_ROUTING: &[(crate::errors::ValidationRefusal, &str, &str)] = &[
    (
        crate::errors::ValidationRefusal::Validate,
        "door",
        "validate",
    ),
    (
        crate::errors::ValidationRefusal::Closed,
        "door",
        "validate_closed",
    ),
    (
        crate::errors::ValidationRefusal::Geometric,
        "door",
        "validate_geometric",
    ),
    (
        crate::errors::ValidationRefusal::Pseudomanifold,
        "door",
        "validate_pseudomanifold",
    ),
    (
        crate::errors::ValidationRefusal::MassProperties,
        "reason",
        "mass_properties_failed",
    ),
];

/// Every refusal writes the attribute and the word [`VALIDATION_ROUTING`]
/// commits it to, and the table names no refusal the enum does not.
#[test]
fn every_validation_refusal_writes_the_attribute_it_is_committed_to() {
    use crate::errors::ValidationRefusal;
    for refusal in ValidationRefusal::ALL {
        let (_, attribute, word) = VALIDATION_ROUTING
            .iter()
            .find(|(committed, _, _)| committed == refusal)
            .unwrap_or_else(|| panic!("{refusal:?} has no committed routing"));
        assert_eq!(
            refusal.attribute(),
            *attribute,
            "{refusal:?} writes a different Python attribute than the one \
             committed for it"
        );
        assert_eq!(
            crate::tags::validation_refusal_tag(*refusal),
            *word,
            "{refusal:?}'s word is not the one committed for it"
        );
    }
    assert_eq!(
        VALIDATION_ROUTING.len(),
        ValidationRefusal::ALL.len(),
        "the committed routing names a refusal the enum does not have"
    );
}

/// **Two layers spell one entity the same way.**
///
/// [`crate::tags::entity_kind_tag`] is total over what a document
/// NAME can denote; [`crate::tags::entity_id_tag`] is total over what
/// the arena can hold. They are two vocabularies and neither
/// contains the other — but where both speak of one entity, a caller
/// resolving a name and a caller reading a census subject are reading
/// one concept, and learning two spellings for it would be a fact
/// about this crate rather than about the model.
///
/// The three shared words are pinned by CONSTRUCTION, which pins the
/// mapping and not just the vocabulary; the fourth is pinned as a
/// divergence, because `body` is a document node and `solid` is an
/// arena lump, and the day a kind is added to either side this row
/// asks whether the other gained one too.
#[test]
fn the_entity_kind_and_entity_id_maps_agree_where_both_speak() {
    use crate::tags::{entity_id_tag, entity_kind_tag};
    use pncad::select::EntityKind;
    use pncad::topo::{EdgeKey, EntityId};

    for (kind, entity) in [
        (EntityKind::Face, EntityId::Face(FaceKey::default())),
        (EntityKind::Edge, EntityId::Edge(EdgeKey::default())),
        (EntityKind::Vertex, EntityId::Vertex(VertexKey::default())),
    ] {
        assert_eq!(
            entity_kind_tag(kind),
            entity_id_tag(&entity),
            "the name layer and the arena layer have drifted apart on one entity"
        );
    }

    // Derived from the committed rows rather than from a roster
    // written here: whatever the two maps mint, `body` is the only
    // word the kind side speaks that the arena side does not.
    let kinds = TAG_INVENTORY
        .iter()
        .find(|entry| entry.function == "entity_kind_tag")
        .expect("the inventory carries the entity-kind alphabet");
    let ids = TAG_INVENTORY
        .iter()
        .find(|entry| entry.function == "entity_id_tag")
        .expect("the inventory carries the entity-id alphabet");
    let unshared: Vec<&str> = kinds
        .values
        .iter()
        .copied()
        .filter(|word| !ids.values.contains(word))
        .collect();
    assert_eq!(
        unshared,
        ["body"],
        "the two entity alphabets diverge somewhere new — a document `body` is not \
         an arena `solid`, and any other difference is a spelling to settle"
    );
}

/// **Two doors spell one param-table fault the same way.**
///
/// The kernel names the eight param-ref refusal arms under one
/// convention, stated once on `editor_core::EditError` and guarded
/// there; this is that convention's image on the wire. A caller that
/// branches on `EditError.variant` and one that branches on the
/// snapshot door's `variant` are reading ONE fault at ONE address, so
/// learning two words for it would be a fact about this crate rather
/// than about the kernel.
///
/// Pinned by CONSTRUCTION, so it pins the MAPPING and not just the
/// vocabulary: each of the four (address, fact) pairs is built at both
/// doors and the two words compared. A door that re-mints a word of
/// its own reds here by name. The four entries `SHARED_TAG_WORDS`
/// carries are the population half of the same fact; this row is why
/// they are one concept rather than a coincidence.
#[test]
fn the_edit_and_snapshot_maps_agree_on_the_four_param_ref_words() {
    use crate::tags::{edit_error_tag, snapshot_error_tag};
    use pncad::document::{Dimension, EditError, ParamName, RecipeNodeId, SlotId, SnapshotError};

    let node = RecipeNodeId(5);
    let name = || ParamName::new("width");

    let pairs: [(&str, &str, EditError, SnapshotError); 4] = [
        (
            "slot",
            "unknown",
            EditError::SlotUnknownDocParam {
                name: name(),
                node,
                slot: SlotId::Radius,
            },
            SnapshotError::SlotUnknownDocParam {
                node,
                slot: SlotId::Radius,
                name: name(),
            },
        ),
        (
            "slot",
            "dimension",
            EditError::SlotDocParamDimension {
                name: name(),
                node,
                slot: SlotId::Radius,
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            },
            SnapshotError::SlotDocParamDimension {
                node,
                slot: SlotId::Radius,
                name: name(),
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            },
        ),
        (
            "payload",
            "unknown",
            EditError::PayloadUnknownDocParam { name: name(), node },
            SnapshotError::PayloadUnknownDocParam { node, name: name() },
        ),
        (
            "payload",
            "dimension",
            EditError::PayloadDocParamDimension {
                name: name(),
                node,
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            },
            SnapshotError::PayloadDocParamDimension {
                node,
                name: name(),
                declared: Dimension::Length,
                referenced: Dimension::Angle,
            },
        ),
    ];

    for (address, fact, edit, snapshot) in &pairs {
        assert_eq!(
            edit_error_tag(edit),
            snapshot_error_tag(snapshot),
            "the edit and load doors have drifted apart on the {fact} fact at the {address} \
             address"
        );
    }

    // The words themselves, against the `{address} x {fact}` product
    // written once: the pairing above stays true if BOTH maps drift
    // together, and this is what catches that.
    let mut spoken: Vec<&str> = pairs
        .iter()
        .map(|(_, _, edit, _)| edit_error_tag(edit))
        .collect();
    let mut convention: Vec<String> = ["slot", "payload"]
        .into_iter()
        .flat_map(|address| {
            ["unknown_doc_param", "doc_param_dimension"]
                .into_iter()
                .map(move |fact| format!("{address}_{fact}"))
        })
        .collect();
    spoken.sort_unstable();
    convention.sort_unstable();
    assert_eq!(
        spoken, convention,
        "the four param-ref words have left the address-then-fact convention on the wire"
    );
}

/// **The class table's `no_at_rest_record` arm predicts the mint
/// door's refusal in the mint door's own word.**
///
/// `ClassAdmission::NoAtRestRecord` exists to answer, before a tool
/// commits an edit, the question `MintRefusal::NoAtRestRecord`
/// answers after one — the kernel says so at the arm itself. So a
/// caller that asks ahead and then reads that refusal is reading one
/// fact, and it stays one fact only while the two maps agree word for
/// word. That is what this row holds.
///
/// **What it does not hold, stated so the row is not read wider than
/// it is.** The prediction is the arm's, not the table's: the mint
/// door (`editor_core::assembly::mint`) refuses through
/// a wildcard `other =>`, so `ClassAdmission::NotAdmitted` ALSO
/// reaches the caller as `MintRefusal::NoAtRestRecord` — under a word
/// the table did not answer. A tool matching the answer it got
/// against the refusal it later sees is right here, silent on `mints`
/// (which refuses nothing) and wrong on `not_admitted`. Whether that
/// wildcard should split is a kernel question, not one this crate can
/// pin.
#[test]
fn the_class_table_predicts_the_mint_refusal_in_its_own_words() {
    use crate::tags::{class_admission_tag, mint_refusal_tag};
    use pncad::document::{ClassAdmission, MintRefusal, RecipeNodeId};
    use pncad::topo::ContactClass;

    assert_eq!(
        class_admission_tag(&ClassAdmission::NoAtRestRecord {
            why: "the table's own reason"
        }),
        mint_refusal_tag(&MintRefusal::NoAtRestRecord {
            mate: RecipeNodeId(0),
            class: ContactClass::Tangent,
            why: "the table's own reason",
        }),
        "the class table and the mint door have drifted apart on the refusal one \
         exists to predict"
    );
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
            "uncertified",
            "unminted_mates",
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
            "non_finite_sector_chord",
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
            "underflowed_sector_chord",
            "unrepresentable_result",
            "unsupported_declaration_class",
            "zip_correspondence",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "boundary_edit_tag",
        values: &["mate_head_not_a_face", "name_serialize"],
        delegates: &["declare_error_tag", "placement_rule_fault_tag"],
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
            "chart_coherence",
            "chart_coherence_unavailable",
            "chart_coherence_unexamined",
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
        function: "class_admission_tag",
        values: &["mints", "no_at_rest_record", "not_admitted"],
        delegates: &[],
    },
    TagEntry {
        function: "coherence_condition_tag",
        values: &[
            "meridian_closure",
            "meridian_continuation",
            "rim_continuation",
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
            "doc_param_count_has_no_distribution",
            "doc_param_count_has_no_unit",
            "doc_param_not_declared",
            "doc_param_unit_mismatch",
            "doc_param_value_kind_mismatch",
            "duplicate_input",
            "duplicate_witness_entry",
            "empty_placement_list",
            "empty_witness_bulk",
            "evaluation_of_another_document",
            "improper_placement",
            "invalid_distribution",
            "invalid_tolerance",
            "maintenance_refused",
            "maintenance_unrecorded",
            "mate_refused",
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
            "payload_doc_param_dimension",
            "payload_unknown_doc_param",
            "pin_unchanged",
            "placement_axis",
            "placement_on_non_instance",
            "placement_rule_mismatch",
            "profile_program_refused",
            "provenance_malformed",
            "read_site_missing_node",
            "rebind_appearance_collision",
            "rebind_identity",
            "rebind_kind_mismatch",
            "rebind_metadata_collision",
            "rebind_no_references",
            "rebind_target_missing_node",
            "rebind_unknown_name",
            "repeated_designation",
            "selection_not_canonical",
            "set_members_on_non_list",
            "set_program_on_non_profile",
            "slot_dimension_mismatch",
            "slot_doc_param_dimension",
            "slot_unknown_doc_param",
            "structural_slot_needs_structural_edit",
            "too_few_members",
            "unknown_node",
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
        // `mate_fault_tag` twice: the maintenance's refusal and the
        // per-mate admission's each forward the solve's fault whole.
        delegates: &[
            "distribution_fault_tag",
            "expr_dimension_error_tag",
            "mate_fault_tag",
            "mate_fault_tag",
            "measure_node_fault_tag",
            "meta_version_error_tag",
            "node_error_tag",
            "program_refusal_tag",
            "provenance_fault_tag",
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
        function: "entity_kind_tag",
        values: &["body", "edge", "face", "vertex"],
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
        function: "eval_reason_tag",
        values: &[
            "empty_boolean",
            "node_failed",
            "node_not_evaluated",
            "poisoned",
            "unknown_node",
            "wrong_kind",
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
            "non_finite_aim",
            "non_finite_mirror_normal",
            "non_finite_roll_reference",
            "non_finite_tangent",
            "underflowed_aim",
            "underflowed_mirror_normal",
            "underflowed_roll_reference",
            "underflowed_tangent",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "frame_fault_tag",
        values: &["improper", "non_finite"],
        delegates: &[],
    },
    TagEntry {
        function: "hit_test_error_tag",
        values: &[
            "ambiguous",
            "evaluation_of_another_document",
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
        function: "interface_crossing_tag",
        values: &["mate"],
        delegates: &[],
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
        function: "lever_refusal_tag",
        values: &[
            "face_unbounded",
            "malformed_body",
            "no_extent",
            "no_finite_bound",
            "not_an_instance",
            "part_unresolved",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "loft_error_tag",
        values: &[
            "band",
            "cap_plane",
            "degenerate_stacking",
            "euler",
            "pcurve",
            "reversed_stacking",
            "seam_structure",
            "section_structure",
            "skin",
            "stacking_escalated",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "maintenance_tag",
        values: &[
            "drop",
            "gauge_rewrite",
            "join",
            "orphaned_declare",
            "rebound",
            "split",
            "strand",
            "stranded_appearance",
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
            "mate_frame_degenerate",
            "mate_indeterminate",
            "mate_part_selects_another_copy",
            "mate_placer_refused",
            "mate_poses_of_another_document",
            "mate_self",
            "mate_table_lacks",
            "mate_under",
            "mate_unleverable",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "mate_primitive_tag",
        values: &["clocking", "coaxial", "frame_coincidence", "planar_rest"],
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
        function: "mint_refusal_tag",
        values: &["mate_reference_refused", "no_at_rest_record"],
        delegates: &[],
    },
    TagEntry {
        function: "naming_error_tag",
        values: &[
            "duplicate",
            "emission",
            "escalated",
            "fragment_lineage_cycle",
            "member_edge_tied",
            "merged_chord",
            "merged_chord_off_rim",
            "missing_upstream",
            "narrow_band",
            "seam_line_sides",
            "seam_vertex_parentage",
            "seam_vertex_partners",
            "split_lineage_cycle",
            "unnamed",
        ],
        delegates: &["band_error_tag", "rim_share_tag"],
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
            "declare_resolve",
            "declare_site_not_an_operand",
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
            "undeclarable_contact",
            "undeclared_contact",
            "underflowed_direction",
            "unschedulable_cycle",
            "verb_arity",
            "witness_bifurcation",
            "wrong_operand",
        ],
        delegates: &[
            "mate_fault_tag",
            "node_error_tag",
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
        function: "ortho_frame_error_tag",
        values: &[
            "degenerate_u_axis",
            "degenerate_v_axis",
            "escalated",
            "non_finite_u_axis",
            "non_finite_v_axis",
            "underflowed_u_axis",
            "underflowed_v_axis",
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
            "fillet_arc_flattened_in_storage",
            "fillet_carrier_below_scene_resolution",
            "fillet_offset_lever_too_short",
            "guided_structure",
            "junction_cusp",
            "junction_tangent",
            "no_corner_for_fillet",
            "no_corner_of_pair",
            "non_finite_direction",
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
            "underflowed_direction",
            "zero_direction",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "persist_error_tag",
        values: &[
            "dimension",
            "display_unit",
            "distribution",
            "edit_replay",
            "header_id",
            "id_mismatch",
            "maintenance_frame",
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
        values: &["lattice"],
        delegates: &[],
    },
    TagEntry {
        function: "program_refusal_tag",
        values: &["geometry", "record", "resolve", "transition", "validate"],
        delegates: &[],
    },
    TagEntry {
        function: "promoted_curve_kind_tag",
        values: &["circle", "line"],
        delegates: &[],
    },
    TagEntry {
        function: "promoted_kind_tag",
        values: &["cylinder", "plane"],
        delegates: &[],
    },
    TagEntry {
        function: "provenance_fault_tag",
        values: &[
            "loop_count",
            "no_such_old_loop",
            "no_such_old_step",
            "old_loop_continued_twice",
            "old_step_continued_twice",
            "step_count",
            "step_of_new_loop",
        ],
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
        values: &[
            "carrier_in_chain",
            "notation_before_any_step",
            "notation_off_program",
            "subdivision_count",
        ],
        delegates: &["expr_dimension_error_tag"],
    },
    TagEntry {
        function: "refused_ref_tag",
        values: &["ref_ambiguous", "ref_read_below_a_root", "ref_vanished"],
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
            "non_finite_axis",
            "non_manifold_axis_contact",
            "op",
            "pcurve",
            "sliver_axis_clearance",
            "sliver_join",
            "sliver_radius",
            "sliver_rim",
            "underflowed_axis",
            "unsupported_toroid",
            "vertex_crosses_axis",
            "void_insertion",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "rim_share_tag",
        values: &["shared_rim_not_adjacent", "shared_rim_several"],
        delegates: &[],
    },
    TagEntry {
        function: "ring_contact_tag",
        values: &[
            "circle_circle",
            "edge_along_edge",
            "edge_edge_point",
            "vertex_on_edge",
            "vertex_on_ring_edge",
            "vertex_vertex",
        ],
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
            "in_band",
            "not_a_datum",
            "not_a_length",
            "pair_in_band",
            "tied_disagrees",
            "unreadable",
        ],
        delegates: &["band_error_tag", "unmirrored_select_tag"],
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
            "assertion_target",
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
            "payload_doc_param_dimension",
            "payload_unknown_doc_param",
            "placement_improper",
            "placement_non_finite",
            "placement_not_gauge",
            "placement_rule",
            "placement_site",
            "slot_dimension",
            "slot_doc_param_dimension",
            "slot_unknown_doc_param",
            "witness_on_missing_node",
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
            "wall_column_structure",
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
        function: "stl_refusal_tag",
        values: &["not_utf8"],
        delegates: &[
            "binary_header_error_tag",
            "solid_name_error_tag",
            "stl_error_tag",
        ],
    },
    TagEntry {
        function: "structure_refusal_tag",
        values: &["flipped", "indeterminate"],
        delegates: &[],
    },
    TagEntry {
        function: "subgroup_tag",
        values: &[
            "cylindrical",
            "empty",
            "planar",
            "prismatic",
            "revolute",
            "se3",
            "trivial",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "tessellate_error_tag",
        values: &[
            "certificate_exceeded",
            "empty_loop",
            "invalid_chordal_tolerance",
            "meridian_free_curved_face",
            "missing_entity",
            "null_scaffold_edge",
            "resolution_overflow",
            "ring_on_curved_face",
            "self_touching_trim_loop",
            "single_column_curved_face",
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
            "full_range_window",
            "nonpositive_wall",
            "revolve",
            "wall_exceeds_radius",
            "wall_gap_collapsed",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "unexaminable_tag",
        values: &["corrupt", "non_iso_carrier", "null_scaffold_edge"],
        delegates: &[],
    },
    TagEntry {
        function: "unmirrored_select_tag",
        // Twice on purpose: one word for two arms, and the multiset is
        // what says the two are held equal here rather than by prose.
        values: &["unclassified", "unclassified"],
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
            "instance_interference",
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
            "poisoned_surface_datum",
            "poisoned_surface_description",
            "ring_contact_escalated",
            "ring_meets_outer",
            "ring_nesting_undecided",
            "ring_outside_outer",
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
            "unrepresentable_surface_datum",
            "vertex_orbit_overrun",
            "volume_uncomputable",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "validation_refusal_tag",
        values: &[
            "mass_properties_failed",
            "validate",
            "validate_closed",
            "validate_geometric",
            "validate_pseudomanifold",
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

/// **Every word that two tag maps both mint, and how many mint it.**
///
/// `src/tags.rs`'s header claims a tag word is scoped to the map that
/// mints it — a coincidence between two maps is not a collision, and
/// pinning every such pair would pin accidents. That claim was
/// decided over seven WORDS and asserted over the file; this is the
/// file, measured — the roster below IS the count, which is why no
/// number is written into this sentence: the population grows
/// whenever a map does, and a prose count of it has gone stale twice.
///
/// The row does not say which of the entries below are one concept and
/// which are coincidence — all but twelve are unread, and `work/census/`'s
/// `sixty-one-tag-words-are-minted-by-two-or-more-maps-and-seven-are-read`
/// is where that question lives. What it does is make the population
/// OBSERVED: a word that starts colliding, or stops, or picks up a
/// third map, arrives as a failure naming itself instead of as
/// silence under a rule nobody re-derived.
///
/// Derived from [`TAG_INVENTORY`] rather than from the source, so it
/// inherits that table's own guard and cannot drift from `tags.rs`
/// without `the_whole_tag_table_matches_its_committed_inventory`
/// reding first. Its own hand-written half is the roster below, and
/// the guard on THAT is this row's two directions: an entry no longer
/// shared fails exactly as a new sharing does.
const SHARED_TAG_WORDS: &[(&str, usize)] = &[
    ("ambiguous", 3),
    ("approx_lane_unsupported", 2),
    ("assertion_dimension", 2),
    ("assertion_target", 2),
    ("band", 16),
    ("cap_plane", 3),
    ("certify", 2),
    ("contact_contradicted", 2),
    ("corrupt", 3),
    ("cosurface_escalated", 2),
    ("dangling_geometry", 2),
    // Three, and ALL THREE are one fact: `parse_error_tag`,
    // `persist_error_tag` and `edit_error_tag` each mean "the document
    // layer's dimension checker refused", at the text door, the load
    // door and the edit door, and each carries that refusal's own tag
    // beside the word — `EditError::Dimension` holds the very same
    // `DimensionError` the other two do
    // (`crate::tags::edit_inner_variant_tag`). An earlier reading of
    // this row had `edit_error_tag`'s down as a DIFFERENT question — a
    // slot's declared dimension against the expression handed to it,
    // which is `SlotDimensionMismatch`, a different arm — and calling
    // them different is what made the three-door divergence in
    // `PersistError`'s `EditReplay` projection invisible.
    ("dimension", 3),
    ("edge", 2),
    ("empty", 2),
    ("empty_boolean", 2),
    ("empty_placement_list", 2),
    ("escalated", 11),
    ("euler", 2),
    ("evaluation_of_another_document", 4),
    ("face", 3),
    ("improper_placement", 2),
    ("indeterminate", 2),
    ("instance", 2),
    ("io", 2),
    ("join", 3),
    ("measure_malformed", 2),
    ("no_at_rest_record", 2),
    ("no_such_body", 2),
    ("node_failed", 4),
    ("node_not_evaluated", 3),
    ("node_poisoned", 2),
    ("non_finite", 5),
    ("non_finite_direction", 2),
    ("non_finite_placement", 2),
    ("not_a_body", 2),
    ("not_an_instance", 2),
    ("null_scaffold_edge", 2),
    ("op", 3),
    ("part_unresolved", 2),
    // ONE concept, and pinned as one: the param-ref convention
    // `editor_core::EditError`'s enum doc states. That the two maps
    // agree word for word is held by
    // `the_edit_and_snapshot_maps_agree_on_the_four_param_ref_words`;
    // these four rows say only that the sharing is deliberate.
    ("payload_doc_param_dimension", 2),
    ("payload_unknown_doc_param", 2),
    ("pcurve", 5),
    ("pcurves", 3),
    ("placement_rule_mismatch", 2),
    ("poisoned", 2),
    ("profile", 2),
    ("revolve", 2),
    ("shell", 2),
    ("skin", 2),
    ("sliver_join", 2),
    ("sliver_rim", 2),
    // The slot-addressed half of the four above, same pin.
    ("slot_doc_param_dimension", 2),
    ("slot_unknown_doc_param", 2),
    ("split", 2),
    ("structure", 3),
    ("tolerance_conflict", 2),
    ("transition", 2),
    ("undeclared_contact", 2),
    ("underflowed_direction", 2),
    ("unknown_node", 5),
    ("unknown_param", 4),
    ("unnamed", 2),
    ("unreadable", 2),
    ("validate", 2),
    ("vertex", 2),
    ("vertex_on_edge", 2),
    ("vertex_on_face", 2),
    ("vertex_vertex", 3),
    ("wrong_kind", 2),
];

#[test]
fn every_word_two_tag_maps_share_is_on_the_committed_roster() {
    let mut minters: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for entry in TAG_INVENTORY {
        for value in entry.values {
            minters.entry(value).or_default().insert(entry.function);
        }
    }
    let shared: Vec<(&str, usize)> = minters
        .iter()
        .filter(|(_, functions)| functions.len() > 1)
        .map(|(word, functions)| (*word, functions.len()))
        .collect();
    let pinned: Vec<(&str, usize)> = SHARED_TAG_WORDS.to_vec();
    if shared != pinned {
        let detail: Vec<String> = minters
            .iter()
            .filter(|(_, functions)| functions.len() > 1)
            .map(|(word, functions)| {
                format!(
                    "{word} ({}): {}",
                    functions.len(),
                    functions.iter().copied().collect::<Vec<_>>().join(", ")
                )
            })
            .collect();
        panic!(
            "the set of tag words minted by two or more maps has moved.\n\n\
             A word two maps share is either ONE FACT the two must keep \
             spelling the same way — which owes a pin in this file — or a \
             coincidence between two unrelated vocabularies, which owes \
             nothing but a decision that it is one. Decide which, then \
             update SHARED_TAG_WORDS from the list below.\n\n  {}",
            detail.join("\n  ")
        );
    }
}

/// The committed inventory of `src/tags.rs`'s `pub const` tag words —
/// the tags that are not behind a `match` at all.
///
/// **One row, and what a row here means is weaker than a map's.** A
/// word behind an exhaustive `match` is reached by naming a variant,
/// so a raise site cannot spell a new one and a kernel arm that
/// arrives without a word stops the build. A `pub const` gives neither:
/// it pins the TEXT where this guard reads it, and the next raise site
/// can still write a literal.
///
/// So the form is the fallback for a word that has no enum to hang
/// off. [`crate::tags::STEP_IMPORT_WIREFRAME`] is one: it names an arm
/// of the STEP importer's SUCCESS enum that the import door does not
/// adopt, while every other word on that attribute is the kernel
/// refusal's — `step_import_error_tag` is the sole other source of
/// `StepImportError.variant` — so there is no type the class could
/// carry that would not be a second spelling of the kernel's arm
/// list. That
/// door's growth is walled by the compiler instead — the enum is not
/// `#[non_exhaustive]` and the door matches it exhaustively.
const TAG_CONSTS: &[(&str, &str)] = &[("STEP_IMPORT_WIREFRAME", "wireframe")];

/// Everything [`read_tag_table`] recognised in the source it read.
struct TagTable {
    /// Function name -> (its own literals, sorted; its delegates, sorted).
    functions: BTreeMap<String, (Vec<String>, Vec<String>)>,
    /// `pub const` name -> its literal.
    constants: BTreeMap<String, String>,
    /// Which arm shapes the reader dispatched on, and which top-level
    /// forms it matched — the reader reporting on ITSELF, which is
    /// what [`the_tag_table_reader_recognises_every_form_it_claims`]
    /// compares against the two rosters.
    shapes: BTreeSet<ArmShape>,
    /// Which top-level forms were matched.
    forms: BTreeSet<TopForm>,
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

/// One shape of match-arm body [`Cursor::parse_arm_body`] recognises.
///
/// **The reader dispatches on this enum rather than on a chain of
/// `if`s over the text, and that is what ties the fixture in
/// [`the_tag_table_reader_recognises_every_form_it_claims`] to the
/// reader.** Three steps close the loop: recognition is
/// [`Self::matches`], an exhaustive `match`, so a seventh form needs
/// a seventh variant; the classifier walks [`Self::ALL`], so a
/// variant left out of that list is never returned and its form is
/// refused as unreadable the first time anything spells it; and the
/// fixture test asserts that every shape in `ALL` was dispatched on,
/// so a variant added to the list reds until the fixture holds an
/// instance. A form the reader can read is therefore a form that test
/// has seen.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum ArmShape {
    /// A bare string literal: the word itself.
    Literal,
    /// A nested `match`, whose own arms are read in turn.
    NestedMatch,
    /// A `{ ... }` block around one of the others.
    Block,
    /// A bare `None`, which contributes no word.
    NoneArm,
    /// A `Some(..)` wrapper around one of the others.
    SomeArm,
    /// A call to another tag function, whose words are that
    /// function's.
    Delegation,
}

impl ArmShape {
    /// Every shape, **in the order they are tested** — this list IS
    /// the classifier's loop, so a shape left out of it is not
    /// recognised at all rather than recognised and unwatched.
    ///
    /// The order is load-bearing in one place: `None` and `Some(..)`
    /// come before [`Self::Delegation`], which would otherwise read
    /// the wrapper as the called map and skip what it wraps.
    const ALL: &'static [Self] = &[
        Self::Literal,
        Self::NestedMatch,
        Self::Block,
        Self::NoneArm,
        Self::SomeArm,
        Self::Delegation,
    ];

    /// Whether an arm body starting `rest` is this shape. Decides
    /// nothing else: recognition is separated from parsing so that
    /// the first half is a list the compiler holds.
    fn matches(self, rest: &str) -> bool {
        match self {
            Self::Literal => rest.starts_with('"'),
            Self::NestedMatch => rest
                .strip_prefix("match")
                .is_some_and(|after| after.starts_with(|c: char| c.is_whitespace())),
            Self::Block => rest.starts_with('{'),
            Self::NoneArm => rest.strip_prefix("None").is_some_and(|after| {
                !after.starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
            }),
            Self::SomeArm => rest.starts_with("Some("),
            Self::Delegation => {
                let name = delegation_name(rest);
                !name.is_empty() && rest[name.len()..].trim_start().starts_with('(')
            }
        }
    }
}

/// The identifier a delegation arm opens with — empty where the arm
/// does not open with one.
fn delegation_name(rest: &str) -> String {
    rest.chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
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
    /// Every arm shape this cursor has dispatched on, for
    /// [`the_tag_table_reader_recognises_every_form_it_claims`].
    shapes: BTreeSet<ArmShape>,
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

    /// Which of [`ArmShape`]'s forms the arm body at the cursor is —
    /// decided without consuming anything, so that recognition and
    /// parsing are two steps and the first one is enumerable.
    ///
    /// Anything else — a `format!`, an `if`, a `const` reference, a
    /// method chain — fails here by name rather than being skipped,
    /// because a tag arrived at by a route this reader cannot follow
    /// is a tag the inventory silently stops covering.
    fn arm_shape(&self) -> ArmShape {
        let rest = self.rest();
        ArmShape::ALL
            .iter()
            .copied()
            .find(|shape| shape.matches(rest))
            .unwrap_or_else(|| {
                panic!(
                    "tags.rs: in `{}`, I do not understand this match arm's body: {:?}. \
                     Teach this reader in the same diff — a shape it cannot follow is a \
                     tag value the inventory stops covering, silently.",
                    self.what,
                    rest.chars().take(60).collect::<String>()
                )
            })
    }

    /// Parse the RIGHT of one arm's `=>`.
    ///
    /// Exactly six shapes are recognised, which is the enumerating
    /// claim this whole reader rests on: a string literal, a nested
    /// `match`, a `{ ... }` block around one of those, a call to
    /// another tag function, and — for the maps that answer
    /// `Option<&'static str>` — a bare `None` or a `Some(..)` around
    /// one of the others. They are [`ArmShape`]'s variants, and the
    /// dispatch below is a `match` over that enum rather than a chain
    /// of tests, so the six are a list the compiler holds rather than
    /// one this comment holds.
    ///
    /// `None` contributes NOTHING: an arm that projects no word is a
    /// decision the reader records by the absence of a value, exactly
    /// as the source spells it. `Some` is not read as a delegation —
    /// it is the wrapper, and what it wraps is what reaches the
    /// inventory.
    fn parse_arm_body(&mut self, values: &mut Vec<String>, delegates: &mut Vec<String>) {
        self.skip_ws();
        let shape = self.arm_shape();
        self.shapes.insert(shape);
        match shape {
            ArmShape::Literal => {
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
            }
            ArmShape::NestedMatch => self.parse_match(values, delegates),
            ArmShape::Block => {
                self.expect("{");
                self.parse_arm_body(values, delegates);
                self.expect("}");
            }
            ArmShape::NoneArm => self.expect("None"),
            ArmShape::SomeArm => {
                self.expect("Some");
                self.expect("(");
                self.parse_arm_body(values, delegates);
                self.expect(")");
            }
            ArmShape::Delegation => {
                let name = delegation_name(self.rest());
                self.at += name.len();
                self.skip_ws();
                let open = self.at;
                let close = balanced_end(self.code, open).unwrap_or_else(|| {
                    panic!("tags.rs: in `{}`, a call that never closes", self.what)
                });
                assert!(
                    !self.text[open..=close].contains('"'),
                    "tags.rs: in `{}`, a string literal inside a delegation's \
                     arguments — I do not understand this",
                    self.what
                );
                self.at = close + 1;
                delegates.push(name);
            }
        }
    }
}

/// **The tag-table reader's own guard.**
///
/// [`read_tag_table`] is a source-text reader, and a reader that stops
/// recognising a form does not fail — it reports agreement over the
/// set it can still see. Every other check on this page exercises it
/// against `src/tags.rs`, which covers only the forms that file
/// happens to hold, and holds a single instance of some: one
/// `pub const` tag word ([`TAG_CONSTS`]), and no
/// `Option<&'static str>` map with a bare `None` arm sits where an
/// eye would notice it going unread.
///
/// So this drives the reader over a source written to hold ONE of
/// every top-level form and arm shape its header claims, and pins what
/// each contributes. A branch that stops matching reds here by name
/// rather than going quiet.
///
/// **The roster below is not a hand census of the reader's branches,
/// and that is the whole point of [`ArmShape`] and [`TopForm`].** The
/// reader classifies before it parses, over those two enums, and each
/// enum's `ALL` IS its classifier's loop. So a form the reader can
/// read is a variant in `ALL` — a variant outside it is never
/// returned and its form is refused as unreadable — and every variant
/// in `ALL` must be exercised here or the assertions at the end of
/// this test red by name. A seventh form added to the reader without
/// an instance in the fixture below cannot be green — which is a
/// property of the dispatch shape and not of anyone remembering to
/// extend a list.
#[test]
fn the_tag_table_reader_recognises_every_form_it_claims() {
    // Not `src/tags.rs`: the subject is the reader, and a fixture it
    // cannot drift away from is the only way to assert a form the real
    // file does not currently spell.
    let source = r#"//! A module header.

// A line comment, and a blank line above it.
use crate::errors::EvalReason;
use pncad::document::{
    EvalError,
};

/// A doc comment carrying a "quoted" word the reader must not read.
pub fn first_tag(reason: EvalReason) -> &'static str {
    match reason {
        EvalReason::UnknownNode => "unknown_node",
        // A delegation inside a block and a bare one, so the two are
        // told apart rather than conflated; and two bracketed
        // patterns, which is what drives the pattern skipper's
        // balanced-bracket branch.
        EvalReason::WrongKind => { second_tag(reason) }
        EvalReason::NodeFailed(_) => third_tag(reason),
        EvalReason::Poisoned { through: _ } => "poisoned",
        EvalReason::EmptyBoolean => match reason.parts() {
            [_, ..] => "empty_boolean",
            [] => "no_parts",
        },
    }
}

pub fn second_tag(reason: EvalReason) -> &'static str {
    match reason {
        _ => "second",
    }
}

pub fn third_tag(reason: EvalReason) -> &'static str {
    match reason {
        _ => "third",
    }
}

pub fn maybe_tag(reason: EvalReason) -> Option<&'static str> {
    match reason {
        EvalReason::Poisoned => None,
        _ => Some("maybe"),
    }
}

pub const SAMPLE_WORD: &str = "sample_word";
"#;
    let table = read_tag_table(source);

    let names: Vec<&str> = table.functions.keys().map(String::as_str).collect();
    assert_eq!(names, ["first_tag", "maybe_tag", "second_tag", "third_tag"]);

    let (values, delegates) = &table.functions["first_tag"];
    // Sorted, so the literal, the nested `match`'s two words and the
    // two delegations all land where the inventory compares them —
    // and the doc comment's quoted word does NOT.
    assert_eq!(
        values,
        &["empty_boolean", "no_parts", "poisoned", "unknown_node"]
    );
    assert_eq!(delegates, &["second_tag", "third_tag"]);

    // The delegates' own words, which is the half a name-only
    // assertion leaves open: a reader that stopped collecting from a
    // wildcard-only `match` would still report both functions.
    assert_eq!(table.functions["second_tag"].0, ["second"]);
    assert_eq!(table.functions["third_tag"].0, ["third"]);

    // `None` contributes nothing; `Some` is the wrapper, not a
    // delegation, so what it wraps is what reaches the inventory.
    let (values, delegates) = &table.functions["maybe_tag"];
    assert_eq!(values, &["maybe"]);
    assert!(delegates.is_empty());

    // The `pub const` form, over the fixture's own instance: the
    // subject here is the reader, and `src/tags.rs`'s single const
    // would make this branch's coverage a fact about that file.
    assert_eq!(
        table.constants,
        BTreeMap::from([("SAMPLE_WORD".to_owned(), "sample_word".to_owned())])
    );

    // And the reader's report on ITSELF: every shape and every form it
    // can dispatch on was dispatched on above. This is the assertion
    // that makes the fixture a census rather than a sample — a form
    // added to the reader reds here until the source above holds one.
    assert_eq!(
        table.shapes.iter().copied().collect::<Vec<_>>(),
        ArmShape::ALL,
        "the fixture no longer exercises every arm shape the reader dispatches on"
    );
    assert_eq!(
        table.forms.iter().copied().collect::<Vec<_>>(),
        TopForm::ALL,
        "the fixture no longer exercises every top-level form the reader matches"
    );
}

/// The pattern skipper's refusal, which no well-formed source reaches.
///
/// [`Cursor::skip_pattern`] panics on a bracket that closes before its
/// arm's `=>`, and the fixture above cannot hold an instance: a source
/// that provokes it is not a source the rest of the reader can read.
/// So it gets its own two-line source, and the branch is exercised
/// rather than asserted about.
#[test]
#[should_panic(expected = "a match arm closed before its `=>`")]
fn the_tag_table_reader_refuses_a_pattern_that_closes_before_its_arrow() {
    read_tag_table(
        "pub fn broken_tag(reason: EvalReason) -> &'static str {\n    \
         match reason {\n        ) => \"never\",\n    }\n}\n",
    );
}

/// **A `pub const fn` map in this file is refused, and says why.**
///
/// `src/tags.rs` holds no `const` map today, so nothing else drives
/// this rung — and the form is not hypothetical: every map in
/// `src/errors.rs` has it, which is what makes "move the map here"
/// wrong as stated wherever that is proposed. `TopForm::Const` matches
/// on `pub const `, a PREFIX of `pub const fn `, so without the
/// refusal above the line is read as a malformed `&str` const and the
/// message names the wrong thing.
#[test]
#[should_panic(expected = "a `pub const fn` tag map")]
fn the_tag_table_reader_refuses_a_const_fn_map() {
    read_tag_table(
        "pub const fn dimension_tag(dim: Dimension) -> &'static str {\n    \
         match dim {\n        Dimension::Length => \"length\",\n    }\n}\n",
    );
}

/// One tag function's body, read into (values, delegates) — and the
/// arm shapes the reader dispatched on getting there.
fn parse_tag_body(
    name: &str,
    text: &str,
    code: &str,
    body: std::ops::Range<usize>,
) -> ((Vec<String>, Vec<String>), BTreeSet<ArmShape>) {
    let mut cursor = Cursor {
        text,
        code,
        at: body.start,
        end: body.end,
        what: name,
        shapes: BTreeSet::new(),
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
    ((values, delegates), cursor.shapes)
}

/// One top-level form [`read_tag_table`] recognises.
///
/// [`ArmShape`]'s argument one level out, and the same device: the
/// reader classifies a line into this enum first and dispatches on it
/// second, so a seventh top-level form cannot be read without a
/// seventh variant, and
/// [`the_tag_table_reader_recognises_every_form_it_claims`] requires
/// the fixture to hold an instance of each.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum TopForm {
    /// A blank line, or one that is nothing but comment — one case,
    /// because the shared lexer has already blanked the second.
    BlankOrComment,
    /// A one-line `use` item.
    UseItem,
    /// A `use` item that opens a `{` block, closed by `};`.
    UseBlock,
    /// A tag map: `pub fn NAME(..) -> &'static str {`.
    TagFn,
    /// A partial tag map: `pub fn NAME(..) -> Option<&'static str> {`.
    OptionTagFn,
    /// A `pub const NAME: &str = "..";` tag word.
    Const,
}

impl TopForm {
    /// Every form, **in the order they are tested** — this list IS
    /// the classifier's loop, exactly as [`ArmShape::ALL`] is, so a
    /// form left out of it is not recognised at all.
    const ALL: &'static [Self] = &[
        Self::BlankOrComment,
        Self::UseItem,
        Self::UseBlock,
        Self::TagFn,
        Self::OptionTagFn,
        Self::Const,
    ];

    /// Whether a top-level line (in the CODE view, so comments are
    /// already blank) is this form, and what follows its keyword.
    fn matches(self, code_line: &str) -> Option<&str> {
        let after = |keyword: &str| code_line.strip_prefix(keyword);
        match self {
            Self::BlankOrComment => code_line.trim().is_empty().then_some(""),
            Self::UseItem => after("use ").filter(|rest| rest.ends_with(';')),
            Self::UseBlock => after("use ").filter(|rest| rest.ends_with('{')),
            Self::TagFn => after("pub fn ").filter(|rest| rest.ends_with(") -> &'static str {")),
            Self::OptionTagFn => {
                after("pub fn ").filter(|rest| rest.ends_with(") -> Option<&'static str> {"))
            }
            // `pub const ` is a prefix of `pub const fn `, and the two
            // are different forms rather than a well-formed one and a
            // malformed one. Refusing the second here sends it to the
            // ladder below, which says what it is.
            Self::Const => after("pub const ").filter(|rest| !rest.starts_with("fn ")),
        }
    }
}

/// Which [`TopForm`] a top-level line is, and what follows its
/// keyword — decided in one place, so the loop below dispatches on an
/// enum rather than on a second chain of prefixes.
///
/// Fails loud on anything else, which is the enumerating claim: a
/// construct this reader cannot place stops the table being read past
/// it rather than being skipped.
fn top_form<'a>(code_line: &'a str, line: &str, number: usize) -> (TopForm, &'a str) {
    if let Some((form, rest)) = TopForm::ALL
        .iter()
        .find_map(|&form| form.matches(code_line).map(|rest| (form, rest)))
    {
        return (form, rest);
    }
    // A line that opens with a keyword this reader knows and then does
    // something it does not: the diagnostic ladder, so the refusal
    // names the part that was not understood rather than the line.
    assert!(
        !code_line.starts_with("use "),
        "tags.rs:{number}: a `use` item that neither ends in `;` nor \
         opens a block — I do not understand this: {line}"
    );
    assert!(
        !code_line.starts_with("pub fn "),
        "tags.rs:{number}: a `pub fn` in the tag module whose signature is \
         neither `(..) -> &'static str {{` nor \
         `(..) -> Option<&'static str> {{` on one line — I do not \
         understand this, and cannot say what it puts on the wire: {line}"
    );
    assert!(
        !code_line.starts_with("pub const fn "),
        "tags.rs:{number}: a `pub const fn` tag map. This reader's two function \
         forms strip `pub fn `, so a `const` one is a form it has never been \
         taught — every map in `src/errors.rs` has this shape, which is why \
         moving one here verbatim does not work. Teach this reader in the same \
         diff that adds the construct: {line}"
    );
    panic!(
        "tags.rs:{number}: I do not understand this top-level line, so the tag \
         table cannot be enumerated past it. Teach this reader in the same diff \
         that adds the construct: {line}"
    );
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
    let mut shapes: BTreeSet<ArmShape> = BTreeSet::new();
    let mut forms: BTreeSet<TopForm> = BTreeSet::new();
    let mut i = 0;
    while i < lines.len() {
        // Matched on the code view; REPORTED as the file spells it.
        let line = lines[i];
        let code_line = code_lines[i];
        let number = i + 1;
        let (form, rest) = top_form(code_line, line, number);
        forms.insert(form);
        match form {
            TopForm::BlankOrComment | TopForm::UseItem => {
                i += 1;
            }
            TopForm::UseBlock => {
                i += 1;
                while i < lines.len() && code_lines[i] != "};" {
                    i += 1;
                }
                assert!(
                    i < lines.len(),
                    "tags.rs:{number}: a `use` block that never closes with `}};` — \
                     I do not understand this file"
                );
                i += 1;
            }
            TopForm::TagFn | TopForm::OptionTagFn => {
                let (name, _) = rest.split_once('(').unwrap_or_else(|| {
                    panic!("tags.rs:{number}: a `pub fn` with no argument list: {line}")
                });
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
                let (read, arm_shapes) = parse_tag_body(name, &text, &code, body);
                shapes.extend(arm_shapes);
                let previous = functions.insert(name.to_owned(), read);
                assert!(
                    previous.is_none(),
                    "tags.rs:{number}: `{name}` is defined twice"
                );
                i = j + 1;
            }
            TopForm::Const => {
                let (name, tail) = rest.split_once(": &str = ").unwrap_or_else(|| {
                    panic!(
                        "tags.rs:{number}: a `pub const` in the tag module that is not a \
                         `&str` — I do not understand this: {line}"
                    )
                });
                // The value is blanked in the code view, so it is
                // located by the structure around it and read from
                // `text`.
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
            }
        }
    }
    TagTable {
        functions,
        constants,
        shapes,
        forms,
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
/// **The `reason` and `door` words are a second family, and no raise
/// site under `src/py/` MINTS one any more.** Such an attribute is as
/// Python-visible as a `variant`, and the words used to be spelled at
/// construction sites where an inventory reading `src/tags.rs` alone
/// could not see them. Minting is the claim and it is narrower than
/// "no such word is written there": `crate::py::measure` writes
/// `door`, `verb` and `scalar` from a kernel value's own `&str`
/// fields, which this crate does not choose and no inventory reads —
/// `work/census/`'s `DimensionError.op` row carries that population,
/// the `door` literal in `editor-core` included. Two classes now CARRY their discriminant, which
/// is the strongest of the arrangements below because it makes the
/// word unspellable rather than merely spelled elsewhere: no raise of
/// [`crate::errors::ErrorClass::Evaluation`] or
/// [`crate::errors::ErrorClass::Validation`] — in this crate or in a
/// file that does not exist yet — can be written without naming a
/// variant, and `crate::py::raise_typed` mints the word from the map.
///
/// **The other doors are closed one rung lower, and the difference is
/// worth reading.** `crate::py::doc`'s boundary raise takes a
/// [`crate::errors::BoundaryEdit`] and `crate::py::mesh`'s export raise
/// a [`crate::errors::StlRefusal`], so no CALL SITE of either can spell
/// a word — but `EditError.variant` and `StlError.variant` also carry
/// the kernel refusals' own words, so the class cannot carry a type
/// without the binding restating a kernel enum's arm list, and a fresh
/// raise of either class could still pass a `variant` of its own.
/// [`crate::tags::unmirrored_select_tag`] is lower again: it makes the
/// query door's wildcard and the contact-class crossing one word in one
/// place, and stops there.
///
/// **And one word is pinned by text alone**, which is the weakest
/// arrangement and is marked as such where it lives:
/// [`crate::tags::STEP_IMPORT_WIREFRAME`], in [`TAG_CONSTS`].
///
/// **What a sweep shape can and cannot see** is the standing lesson
/// here rather than any of those words. A sweep for a literal beside a
/// `reason`/`variant` key found three of the nine; the same words
/// minted in tuple position, passed as a `&'static str` argument, or
/// returned from a getter are the same class and were invisible to it.
/// `work/census/` carries what is measured and unfixed — including
/// `DimensionError.op`, whose twelve words still reach Python from
/// literals at call sites of two `&'static str` parameters.
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
    // No floor on `constants`, and it is not wanted: a floor exists to
    // catch a reader that matched almost nothing over a population too
    // big to enumerate, and `TAG_CONSTS` IS the enumeration — every
    // const is named there, so the loop below reports a missing one by
    // name rather than as a count that drifted.

    // The order [`TAG_INVENTORY`]'s own doc states, CHECKED rather
    // than restated. It is not a Python-visible fact and nothing below
    // depends on it — the comparison is keyed by name — but a table
    // this long is read by scrolling, and three entries had drifted out
    // of place before this ran. A stated invariant with no check is the
    // shape this whole page exists to close.
    let ordered: Vec<&str> = TAG_INVENTORY.iter().map(|entry| entry.function).collect();
    let mut by_name = ordered.clone();
    by_name.sort_unstable();
    assert_eq!(
        ordered, by_name,
        "TAG_INVENTORY is not in the order its doc claims — by function name"
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

/// **One item `src/errors.rs` declares**, and the check that holds
/// whatever words it puts on a Python wire.
struct MintingItem {
    /// The item, qualified by the `impl` block that holds it where it
    /// has one: `Type::name`, or `<Type as Trait>::name`.
    owner: &'static str,
    /// How many literals it spells, character literals included.
    /// Zero for an item that spells none — a map forwarding another
    /// file's word, a constructor taking a word from its caller, a
    /// roster of variants — which is a row here like any other,
    /// because what this census watches is ARRIVALS.
    literals: usize,
    /// What holds those WORDS. This roster holds the item's
    /// EXISTENCE; values are each row's own pin, named here so that a
    /// row cannot be added without someone answering the question.
    held_by: &'static [Holder],
}

/// What holds one item's WORDS — the answer to the question a row
/// cannot be added without answering.
///
/// A column of prose is a claim nothing re-derives: a renamed test
/// leaves it pointing at nothing and the census stays green, because
/// the census is about the item's EXISTENCE and not about what holds
/// its words. Naming the holders as data rather than as a sentence is
/// what lets [`every_test_this_census_names_is_one_this_file_declares`]
/// go and look.
enum Holder {
    /// A test in this file, by name, and what it holds. The name is
    /// re-derived against this file's own source; nothing re-derives
    /// `holds`, which is a description and not a fact about the tree.
    ///
    /// The name is the ONE spelling. Carrying the test item beside it
    /// — `fn()` in this same array — would pin the rename at compile
    /// time and would be a second spelling of the same thing held
    /// equal to the first by nothing, which is the defect this
    /// program exists to close.
    Test {
        /// The `#[test] fn`'s name, as this file declares it.
        name: &'static str,
        /// What that test holds about this item's words.
        holds: &'static str,
    },
    /// A holder outside Rust — `pncad.pyi`, the Python suite. Nothing
    /// here re-derives it, and saying so is the point: a row whose
    /// holders are all `Outside` is a row no Rust check covers.
    Outside(&'static str),
}

/// **The committed roster of everything `src/errors.rs` declares** —
/// the file's arrival alarm.
///
/// `src/tags.rs` is safe to add to because `TAG_INVENTORY` reads it:
/// a new map there is *"a new set of public Python words that no
/// inventory has looked at"* the moment it lands. This file had no
/// equivalent, and the cost was measured rather than predicted — a
/// further `-> &'static str` map arrived here unpinned, in a diff that
/// left the tracker row predicting it untouched.
///
/// **Which bytes are literals is the shared lexer's answer, not a
/// grammar of this reader's own, and that is the whole design.** Every
/// previous instrument over this crate's vocabulary was keyed on a
/// FORM — a top-level `pub fn`, a literal beside a key, a lowercase
/// word — and each went blind to the arrival that did not wear it:
/// this file's `-> &'static str` maps include inherent methods inside
/// `impl` blocks, which a top-level-keyed reader does not walk, and
/// every one of them is a `pub const fn`, which the tag reader's
/// `pub fn ` forms do not admit. [`read_minting_items`] asks
/// [`test_utils::source`] which bytes of the file are literals, so
/// there is no form a LITERAL can arrive in that this reader was not
/// taught.
///
/// **The population is a grammar, and saying otherwise was the last
/// short claim written here.** Which bytes are literals is the shared
/// lexer's answer; which item spells them is this reader's, and the
/// walk that answers it reads `fn`, `const` and `static` at the
/// keyword rather than at a line start, under an allow-list of what
/// may precede an item. Line-start keying is what that sentence used
/// to hide: `impl Subject { pub const ALL: &[u8] = &[]; }` read as no
/// item at all, and the same shape under a rostered row above it
/// inflated that row and cancelled to silence. What the walk can and
/// cannot read is [`SCOPE_WALK_BLIND_SPOTS`] and the tests it names,
/// not this paragraph.
///
/// **The rows are the file's DECLARATIONS and the literals are what
/// each one contributes.** Keying the roster on the literals instead
/// left an item spelling none outside the alarm entirely, which is an
/// arrival alarm blind to an arrival: three items here spell no
/// literal (`EvalReason::ATTRIBUTES`, `QuantityOpMismatch::new`,
/// `ValidationRefusal::ALL`), and the first of them is Python-visible
/// vocabulary — an attribute NAME. A map forwarding `crate::tags`'
/// word is the same shape and
/// `the_errors_mint_census_reds_by_name_on_a_map_that_spells_no_literal`
/// executes it.
///
/// **The key is `(self type, trait, item name)`, which is the sibling
/// census's key** — `crates/test-utils/tests/hand_written_impl_census.rs`
/// keys on `(path, trait, self type)` and says at the site why the
/// trait is in it. Written as Rust writes it, `<Type as Trait>::name`.
/// A key without the trait made `impl fmt::Debug for QuantityOpMismatch`
/// beside the existing `impl fmt::Display` a collision, and hard-stopped
/// the census on correct code with an instruction — qualify them apart —
/// that Rust gives no way to follow.
///
/// **What this roster does NOT do**, said here because a roster reads
/// as completeness: it is an ARRIVAL alarm, not a word inventory. The
/// `literals` count moves when a word is added to or dropped from a
/// rostered item, so growth is loud; a word RENAMED in place, or
/// swapped for another inside one item, leaves the count alone and is
/// the `held_by` column's business.
///
/// **Its population is the file's `fn`, `const` and `static`
/// declarations, and what is outside that is a word channel that is
/// not one of them.** `QuantityOpMismatch::op` is a struct FIELD: a
/// `&'static str` this file carries and does not spell, reaching
/// Python as `DimensionError.op` and interpolated into that class's
/// message. **Twelve words arrive that way today** (measured
/// 2026-09-15 over every literal passed to the two `&'static str` door
/// parameters under `src/py/`), every one minted at a call site and
/// read by no instrument —
/// `work/census/dimension-error-op-carries-twelve-words-minted-at-call-sites.md`
/// is that row, and it is not this one. A SECOND such field arrives
/// with this census silent, which
/// `the_errors_mint_census_cannot_see_a_word_channel_that_is_not_a_declaration`
/// executes rather than asserts. Widening this reader to name a field
/// would see the channel and still not see the words, which are in
/// another file.
///
/// **Why this census is in this file, weighed.** Against: this
/// program's other two instruments —
/// `crates/test-utils/tests/hand_written_impl_census.rs` and
/// `deny_unknown_fields_census.rs` — are each their own file, and this
/// one lands deep inside a `mod tests` whose size is an open row on
/// the same slate, which each unit that touches it has grown. That row
/// carries the measurement, with the command that re-derives it and
/// the SHA it was taken at; a copy of the number here would be a
/// second spelling of a count, which is this program's own subject.
/// For: the subject is
/// ONE sibling module of this crate, read through `crate_dir` like
/// `TAG_INVENTORY` above it reads `src/tags.rs`, and the `held_by`
/// column names tests that live here — a census in `test-utils` would
/// cite eleven tests it cannot reach and would be a second place to look
/// for "what holds a word in this crate". The `TAG_INVENTORY`
/// precedent decided it. **What it costs is real and is not paid
/// here**: `work/census/pncad-py-tests-rs-…` carries both sides, and
/// the growth, for whoever splits it.
///
/// **Where this reader's parts live, and why each is where it is.**
/// Four operations here are not this census's: reading an `impl` head
/// ([`test_utils::source::impl_head`]), taking a type's bare name
/// ([`type_base`]), lexing an identifier ([`ident`]) and finding a
/// line's start ([`line_start`]). Each had a second implementation in
/// this file or a sibling census by a different algorithm, and a core
/// hosted inside one of its consumers is the drift this program exists
/// for — so all four moved to `crates/test-utils/src/source.rs`, which
/// is where this crate's other readers already ask what a byte is.
///
/// **Three more are lexer operations and are still hosted here**, and
/// that is disclosed rather than argued away: [`balanced_open`] is
/// [`balanced_end`]'s inverse, [`strip_modifier`] is
/// [`boundary_before`]'s predicate at the other end of a word, and
/// [`item_start`] answers a question the sibling census has and
/// answers less well. They stay because `crates/test-utils/*` is
/// another program's territory and widening its grammar is a filed row
/// rather than one taken here —
/// `work/census/the-mint-reader-hosts-three-lexer-operations-of-its-own.md`.
///
/// What is about THIS census and nothing else is the
/// `(self type, trait, item name)` key, the attribution of a literal
/// to the item whose head is nearest above it, and the roster below.
///
/// Sorted by owner, and the test below checks that rather than
/// restating it.
const ERRORS_MINTING_ITEMS: &[MintingItem] = &[
    MintingItem {
        owner: "<QuantityOpMismatch as Display>::fmt",
        literals: 1,
        held_by: &[Holder::Test {
            name: "a_quantity_operator_mismatch_carries_structure_not_prose",
            holds: "the rendered message",
        }],
    },
    MintingItem {
        owner: "DIMENSION_DOORS",
        literals: 0,
        held_by: &[Holder::Outside(
            "nothing: it is a `#[doc(hidden)]` anchor whose whole content is a \
             six-row prose table, and the count that table fixes was written \
             three ways with two different numbers before it existed, which is \
             `work/lib/the-dimension-door-table-is-prose-nothing-re-derives.md`",
        )],
    },
    MintingItem {
        owner: "ErrorClass::class_name",
        literals: 35,
        held_by: &[Holder::Test {
            name: "error_classes_name_the_python_hierarchy",
            holds: "the 35 class names, against a SECOND exhaustive match, so a new \
                    class stops the build",
        }],
    },
    MintingItem {
        owner: "EvalReason::ATTRIBUTE",
        literals: 1,
        held_by: &[
            Holder::Test {
                name: "the_discriminant_attribute_names_are_declared_in_the_stub",
                holds: "the word against `pncad.pyi`'s `EvaluationError` declaration",
            },
            Holder::Outside("the Python suite, which reads `EvaluationError.reason`"),
        ],
    },
    MintingItem {
        owner: "EvalReason::ATTRIBUTES",
        literals: 0,
        held_by: &[Holder::Test {
            name: "the_discriminant_attribute_names_are_declared_in_the_stub",
            holds: "the one word it carries — `ATTRIBUTE`'s, spelled once and \
                    derived here",
        }],
    },
    MintingItem {
        owner: "QuantityOpMismatch::new",
        literals: 0,
        held_by: &[Holder::Outside(
            "nothing: its `op` parameter's twelve words are minted at call sites \
             under `src/py/` and no instrument reads them, which is \
             `work/census/dimension-error-op-carries-twelve-words-minted-at-call-sites.md`",
        )],
    },
    MintingItem {
        owner: "ValidationRefusal::ALL",
        literals: 0,
        held_by: &[Holder::Test {
            name: "validation_refusals_are_the_roster_the_inventory_reads",
            holds: "the roster against `crate::tags::validation_refusal_tag`'s words \
                    as `TAG_INVENTORY` reads them, so a sixth refusal reds",
        }],
    },
    MintingItem {
        owner: "ValidationRefusal::ATTRIBUTES",
        literals: 2,
        held_by: &[
            Holder::Test {
                name: "the_validation_class_mints_exactly_these_attributes",
                holds: "the set, equal to the image of `attribute` over `ALL` in both \
                        directions",
            },
            Holder::Test {
                name: "the_discriminant_attribute_names_are_declared_in_the_stub",
                holds: "`door` against `pncad.pyi`'s `ValidationError` declaration, and \
                        `reason` as a gap that stub has not closed",
            },
        ],
    },
    MintingItem {
        owner: "ValidationRefusal::attribute",
        literals: 2,
        held_by: &[
            Holder::Test {
                name: "the_validation_class_mints_exactly_these_attributes",
                holds: "the words",
            },
            Holder::Test {
                name: "every_validation_refusal_writes_the_attribute_it_is_committed_to",
                holds: "which refusal writes which",
            },
        ],
    },
    MintingItem {
        owner: "canonical_unit",
        literals: 2,
        held_by: &[Holder::Test {
            name: "canonical_units_match_the_gq5_ratification",
            holds: "both units",
        }],
    },
    MintingItem {
        owner: "dimension_tag",
        literals: 4,
        held_by: &[
            Holder::Test {
                name: "dimension_tags_are_stable",
                holds: "the four words",
            },
            Holder::Test {
                name: "dimension_tags_match_the_kernel_prose",
                holds: "them against the kernel's own rendering, over `Dimension::ALL`",
            },
        ],
    },
    MintingItem {
        owner: "is_bare_camel_token",
        literals: 1,
        held_by: &[Holder::Test {
            name: "the_prose_rule_separates_a_display_from_a_debug_dump",
            holds: "the predicate over an underscored bare token — and nothing here \
                    reaches Python, a `char` being an alphabet rather than a word",
        }],
    },
    MintingItem {
        owner: "measurement_dimension_tag",
        literals: 4,
        held_by: &[Holder::Test {
            name: "the_two_dimension_alphabets_are_one_list_in_two_cases",
            holds: "the four words as the capitalised spelling of `dimension_tag`'s, \
                    over `Dimension::ALL`",
        }],
    },
    MintingItem {
        owner: "reads_as_prose",
        literals: 1,
        held_by: &[Holder::Test {
            name: "the_prose_rule_separates_a_display_from_a_debug_dump",
            holds: "the predicate over both fingerprints",
        }],
    },
];

/// Every item `source` declares, with the literals it spells.
///
/// **Which bytes are literals is [`test_utils::source`]'s answer, not
/// this function's**, taken from three of its views at once: `code`
/// blanks comments and literals, `comments` keeps comments alone, and
/// `text` keeps literals alone, so a byte blank in the first two and
/// not the third is inside a literal. No grammar of this reader's
/// decides which bytes are a word. What is left is the POPULATION and
/// the ATTRIBUTION — which items there are and which of them spells a
/// given literal — and both are this reader's own walk over
/// [`DECLARATION_KEYWORDS`] and [`SCOPE_KEYWORDS`], which is where it
/// can be wrong. Two of its known wrong answers are refused rather
/// than reported: a literal it cannot place at all, and a literal in
/// an attribute whose item it cannot name. The rest are
/// [`SCOPE_WALK_BLIND_SPOTS`].
///
/// **Every literal counts, character literals included.** A `char` puts
/// no word on a Python wire and is not vocabulary; it is counted anyway
/// because the population is the file's literals and an item is only in
/// this census if it has one. Dropping them dropped the ITEM that
/// spelled nothing else — a `pub const fn sep(self) -> char` spliced
/// onto this file arrived with the roster silent — and made a `char`
/// added to a rostered item free. A character literal is carried as the
/// token AS WRITTEN, `'_'` rather than `_`, so a row's list cannot read
/// as a word it is not.
///
/// Fails loud on what it cannot place: a literal above every
/// declaration, a literal in an attribute on an item this reader does
/// not name, a literal form it cannot lex, a scope whose body does not
/// close, an attribute whose `[` does not close, two items that would
/// answer to one qualified name.
fn read_minting_items(source: &str) -> BTreeMap<String, Vec<String>> {
    let text = code_and_literals(source);
    let code = code_only(source);
    let comments = comments_only(source);
    let scopes = scope_spans(&code);
    let decls = declaration_heads(&code);
    // Every item the file declares, literal or not. The population is
    // the DECLARATIONS and the literals are what each one contributes:
    // keying the map on the literals instead would leave an item that
    // spells none out of the census entirely, and an arrival alarm
    // that cannot see an arrival which happens to spell no word is
    // not an arrival alarm.
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for &(start, ref name) in &decls.heads {
        let owner = qualified(&scopes, start, name);
        assert!(
            found.insert(owner.clone(), Vec::new()).is_none(),
            "errors.rs: two items answer to `{owner}` — this reader keys on the item \
             name qualified by every scope that holds it, trait and all, so two \
             spellings of one key merge into one roster row silently. **Rust permits \
             such a pair**: two `#[cfg]`-selected items of one path are one item after \
             configuration, and this reader has no configuration — so this is either a \
             reader that has mis-read one of them, or a file this reader cannot census \
             until it is taught to tell the two apart. Whichever it is, it is not \
             something the author can qualify away."
        );
    }
    let bytes = text.as_bytes();
    let blanked = code.as_bytes();
    let prose = comments.as_bytes();
    let mut at = 0usize;
    while at < bytes.len() {
        // Blank in the code view, blank in the comment view, and NOT
        // blank in the one that keeps literals: inside a literal.
        // Three views rather than two, because a comment adjoining a
        // literal is blank in the same two the literal is, and a walk
        // that cannot tell them apart reads across the boundary.
        if blanked[at] != b' ' || prose[at] != b' ' || bytes[at] == b' ' {
            at += 1;
            continue;
        }
        let mut end = at;
        while end < bytes.len() && blanked[end] == b' ' && prose[end] == b' ' {
            end += 1;
        }
        let raw = text[at..end].trim_end();
        let value = if let Some(word) = plain_string_literal(raw) {
            word.to_owned()
        } else if raw.starts_with('\'') || raw.starts_with("b'") {
            // A character literal, carried as written — see the doc.
            raw.to_owned()
        } else {
            // A raw string, a byte string, a C string, a multi-line
            // literal this walk saw one line of: a form it cannot read
            // is a form a word could arrive in unseen.
            panic!(
                "errors.rs:{}: a literal form I do not understand, so I cannot say \
                 what it puts on the wire: {raw:?}",
                test_utils::source::line(&text, at)
            );
        };
        found
            .entry(minting_owner(&text, &scopes, &decls, at))
            .or_default()
            .push(value);
        at += raw.len();
    }
    for literals in found.values_mut() {
        literals.sort();
    }
    found
}

/// One thing [`scope_spans`] cannot see, the probe that executes it,
/// and what that probe does not reach.
///
/// **Prose here was a claim nothing re-derived**, in a file whose
/// subject is exactly that. The entry for `impl` in type position
/// named a test that has never existed under the spelling it used —
/// the real one reads `recognises` where the entry read `reads` — and
/// the doc gate is green over a name in prose, link or no link,
/// because rustdoc does not build a `cfg(test)` module at all. A name
/// in this column is looked up in this file's own source instead.
struct BlindSpot {
    /// What the walk does not see, or sees wrongly.
    cannot_see: &'static str,
    /// The `#[test] fn` that executes it, re-derived against this
    /// file's own source. An entry with no probe is a claim, and a
    /// claim is what this column exists to refuse.
    probe: &'static str,
    /// What that probe does not reach. A probe inherits the fence of
    /// whoever wrote it, and this column is where that fence is
    /// written down.
    unreached: &'static str,
}

/// **What [`scope_spans`] cannot see, each with its probe and that
/// probe's own blind spot.**
///
/// Stated before the walk was changed rather than after, which is the
/// order this program's standing finding 2 asks for: the list, then
/// the probe against each entry, then the probe's own fence. It claims
/// no completeness — four such lists have been written about this
/// reader and every one was short.
const SCOPE_WALK_BLIND_SPOTS: &[BlindSpot] = &[
    BlindSpot {
        cannot_see: "a scope a macro expands to: a `macro_rules!` body spelling `impl` \
                     is text this walk reads out of its expansion context, and one that \
                     pastes the keyword together spells nothing for it to read",
        probe: "the_errors_mint_reader_reads_a_macro_body_as_text_and_says_so",
        unreached: "a derive or attribute macro, whose output no text in this file \
                    spells — no probe written here can reach it, and this census is \
                    silent on every item such a macro mints",
    },
    BlindSpot {
        cannot_see: "a `mod name;` whose body is another file: those items are outside \
                     this reader's population entirely",
        probe: "the_errors_mint_reader_does_not_read_a_file_module_as_a_scope",
        unreached: "it shows the walk does not MISATTRIBUTE the rest of the file into \
                    that module, and says nothing whatever about the words in the other \
                    file",
    },
    BlindSpot {
        cannot_see: "a keyword standing in a TYPE — `-> impl Display`, an `impl Trait` \
                     argument, a `fn(u8)` pointer, a `const` generic parameter, the \
                     `static` of every `&'static` lifetime. `item_start` refuses these, \
                     and the half that ends in a `;` is refused twice over by \
                     `ItemBody::Declaration`",
        probe: "the_errors_mint_reader_recognises_what_it_claims",
        unreached: "the fixture is read by the same walk it is checking, so a defect in \
                    `boundary_before`/`item_body` would pass both together. The check \
                    that is differently shaped is the roster itself — thirteen rows over \
                    the real `src/errors.rs`, a behavioural pin that moves if this walk \
                    starts or stops seeing a scope",
    },
    BlindSpot {
        cannot_see: "the prefix is the SYNTACTIC nesting and not the type's defining \
                     path: `mod a { impl crate::Taxonomy { … } }` keys under \
                     `a::Taxonomy`, and a top-level `impl Taxonomy` under `Taxonomy`, \
                     for one type",
        probe: "the_errors_mint_reader_keys_a_nested_impl_by_where_it_is_written",
        unreached: "it records what this reader answers, not that the answer is the one \
                    a future author expects; it is a disclosed property and not a repair",
    },
    BlindSpot {
        cannot_see: "an `impl` whose generic argument holds a brace — `impl Holds<{ N }> \
                     for Subject` — loses its body to that brace and stops being a \
                     scope, so its items are keyed bare",
        probe: "the_errors_mint_reader_loses_an_impl_whose_generic_argument_holds_a_brace",
        unreached: "it pins the wrong answer as the answer. The repair is in `item_body`, \
                    which is `test_utils::source`'s and shared with the sibling census \
                    that reads it the same way, so it is a filed row rather than a \
                    widening taken here",
    },
    BlindSpot {
        cannot_see: "a BLOCK is a scope in Rust and is not one here: two blocks in one \
                     function may each declare a `const` of the same name, and this \
                     reader keys both under the function. So may two `#[cfg]`-selected \
                     `mod`s of one path — one item after configuration, two to a reader \
                     that has none",
        probe: "the_errors_mint_reader_refuses_two_items_that_answer_to_one_name",
        unreached: "the refusal is a hard stop, so the case is not censused but \
                     abandoned: a file holding such a pair gets no roster at all until \
                     this walk is taught to tell the two apart",
    },
];

/// Every `impl`, `mod`, `trait` and `fn` block in the file: the
/// prefix a name declared inside it is qualified by, and the byte
/// range its body spans.
///
/// **The scan is free and the guard is the sibling census's**, not a
/// line-start rule of this reader's own.
/// `crates/test-utils/tests/hand_written_impl_census.rs` walks
/// `code[from..].find("impl")` over the blanked view and admits a hit
/// only where [`boundary_before`] and [`boundary_after`] make it a
/// keyword; [`item_body`] then says whether a body follows. An `impl`
/// inside a `mod` or a function body is ordinary Rust that a
/// line-start rule does not see, and its methods then answer to bare
/// names — loud, because a bare name is not on the roster, but loud
/// under an item that does not exist by that name.
///
/// **`mod` is in the walk because the qualifier is a PATH.** A scope
/// that contributes no prefix is a scope two types of the same bare
/// name can collide under, and the collision is the one direction this
/// census cannot be loud in — two names merging into one roster row.
/// With the module in the key, `named` below is refusing a name Rust
/// itself refuses, which is what its message claims.
///
/// **The key carries the TRAIT, and that is the sibling census's key
/// rather than this reader's invention.** That census keys on
/// `(path, trait, self type)` and says why: a key naming less than the
/// impl covers impls it was never written about. Here the third
/// element is the item name instead of the file, and dropping the
/// trait had the same cost in a sharper form — `impl fmt::Debug for
/// QuantityOpMismatch` beside the existing `impl fmt::Display` puts
/// two `fmt`s under one key, and ordinary correct Rust then hard-stops
/// the census on a demand no author can satisfy, because two trait
/// `fmt`s cannot be qualified apart. `<Type as Trait>::name` is Rust's
/// own spelling for the distinction and is what the roster carries; a
/// module prefix is written BEFORE it (`nested::<Type as Trait>::name`)
/// rather than inside the angle brackets, which is not how Rust spells
/// that path.
///
/// **A rendered string and not a tuple, weighed.** The key's job is to
/// be unique, and it is not quite: two `#[cfg]`-selected items of one
/// path, or two blocks in one function, answer to one string, which
/// [`read_minting_items`] refuses rather than merges. A tuple key
/// rendered only for messages would not carry that distinction either
/// — neither pair differs in any component a tuple would hold, the
/// `cfg` and the block being the distinguishing thing and no part of
/// the path. What it would cost is the roster:
/// [`ERRORS_MINTING_ITEMS`]'s `owner` is one committed string per row,
/// sorted and compared by it, and a tuple would make every row a
/// nested literal for no gain in what the census can tell apart.
///
/// **What this walk cannot see is [`SCOPE_WALK_BLIND_SPOTS`]**, which
/// is data rather than prose for the reason the `held_by` column is:
/// every entry names the probe that executes it, and
/// [`every_test_this_census_names_is_one_this_file_declares`] looks
/// each one up in this file's source. A list like this one has claimed
/// exclusivity and been short four times in this program, so it claims
/// none — these are the ones that have been run, and the walk is a
/// text walk, so there are others.
fn scope_spans(code: &str) -> Vec<(String, std::ops::Range<usize>)> {
    let mut spans = Vec::new();
    for keyword in SCOPE_KEYWORDS {
        let mut from = 0usize;
        while let Some(off) = code[from..].find(keyword) {
            let at = from + off;
            from = at + keyword.len();
            if !boundary_before(code, at)
                || !boundary_after(code, at + keyword.len())
                || item_start(code, at).is_none()
            {
                continue;
            }
            let body = match item_body(code, at) {
                ItemBody::Body(body) => body,
                // `impl Trait` in type position, `mod name;` and a
                // trait method with no default all end at a `;`, and
                // none of them opens a scope this file holds items in.
                ItemBody::Declaration(_) => continue,
                ItemBody::Unterminated => panic!(
                    "errors.rs:{}: an `{keyword}` whose body neither opens nor closes — \
                     I do not understand where its items end",
                    test_utils::source::line(code, at)
                ),
            };
            spans.push((scope_qualifier(code, keyword, at, body.start), body));
        }
    }
    spans
}

/// The prefix a name declared directly inside one scope is written
/// under: a module's own name, or an `impl`'s subject and trait.
fn scope_qualifier(code: &str, keyword: &str, at: usize, body_start: usize) -> String {
    if keyword != "impl" {
        let name = ident(code, skip_ws(code, at + keyword.len()));
        assert!(
            !name.is_empty(),
            "errors.rs:{}: a `{keyword}` whose name I cannot read",
            test_utils::source::line(code, at)
        );
        return name.to_owned();
    }
    let head = impl_head(code, at, body_start).unwrap_or_else(|| {
        panic!(
            "errors.rs:{}: an `impl` head I cannot read — its generic list does \
             not close before its body",
            test_utils::source::line(code, at)
        )
    });
    let subject = type_base(&head.self_type);
    head.trait_path.map_or_else(
        || subject.to_owned(),
        |named| format!("<{subject} as {}>", trait_key(&named)),
    )
}

/// The keywords that open a scope a declaration can be written
/// inside, and whose name the declaration is qualified by.
///
/// `fn` is one of them, and its absence was a phantom: a `const`
/// declared in a method's body answered to the ENCLOSING IMPL's name,
/// so `impl A { fn m() { const W … } }` beside `impl B { … }` put a
/// `W` under `A` that `A` does not declare and read `A::m` as
/// spelling nothing. Two free functions each holding a `const` of the
/// same name are ordinary Rust and collided outright.
const SCOPE_KEYWORDS: [&str; 4] = ["impl", "mod", "trait", "fn"];

/// The keywords that declare an item this census attributes literals
/// to, in the order their arms are read.
const DECLARATION_KEYWORDS: [&str; 3] = ["fn", "const", "static"];

/// What may be written between an item's attributes and its keyword.
///
/// `const` is here for `const fn`, whose head this reader takes at the
/// `fn` — so that the body is a scope like any other `fn`'s. The
/// `const` arm of [`declaration_name`] hands `const fn` on for exactly
/// that reason, and the two together give one head per item rather
/// than two.
const ITEM_MODIFIERS: [&str; 5] = ["pub", "unsafe", "async", "extern", "const"];

/// Where the item whose keyword is at `at` begins — its first
/// modifier, or `at` itself — and `None` where that keyword opens no
/// item at all.
///
/// `impl` is a type-position keyword — `-> impl Display`, an
/// `impl Trait` argument — and the body [`item_body`] answers for one
/// of those is the enclosing FUNCTION's, so a walk that took it would
/// qualify that function's whole contents under the trait's name.
/// [`ItemBody::Declaration`] catches only the half that ends in a `;`.
/// `fn` stands in a type too (`fn(u8) -> u8`), `const` stands in a
/// generic parameter list (`impl<const N: usize>`) and in a `const {}`
/// block, and `static` is the tail of every `&'static` lifetime in the
/// file.
///
/// **An allow-list of what may precede an item, not a deny-list of
/// type positions.** A deny-list of the places these keywords can
/// stand is a blind-spot list, and a blind-spot list written about
/// this reader has been short every time one has been written. What
/// may sit between one item and the next is whitespace — a comment is
/// whitespace in the code view — and [`ITEM_MODIFIERS`], with a
/// visibility's optional `(…)`. `default` is specialization's and
/// modifies none of these keywords, so it is not in the list. What may
/// sit before that is the end of another item (`}` or `;`), the
/// opening of the scope holding it (`{`), the close of an attribute
/// (`]`), or the start of the file.
///
/// **A modifier is stripped as a WHOLE token.** `head.strip_suffix`
/// alone reads the `pub` of `mypub` as a visibility, and what follows
/// is then a declaration this reader invents; the identifier boundary
/// is what keeps the allow-list from admitting a word it was not
/// written about.
///
/// **A wrong answer here is loud in one direction only, and that is
/// the safe one.** Rejecting a real item costs its declarations their
/// qualifier or drops them, and a bare name is not on the roster, so
/// each reports NEW. Accepting a type position is the quiet direction,
/// which is why the list admits rather than excludes.
fn item_start(code: &str, at: usize) -> Option<usize> {
    let mut head = code[..at].trim_end();
    let mut begin = at;
    loop {
        let Some(last) = head.as_bytes().last() else {
            return Some(begin);
        };
        if *last == b')'
            && let Some(open) = balanced_open(head)
            && let before = head[..open].trim_end()
            && let Some(cut) = strip_modifier(before, "pub")
        {
            begin = before.len() - "pub".len();
            head = cut;
            continue;
        }
        let Some((cut, word)) = ITEM_MODIFIERS
            .into_iter()
            .find_map(|word| strip_modifier(head, word).map(|cut| (cut, word)))
        else {
            return matches!(last, b'}' | b';' | b'{' | b']').then_some(begin);
        };
        begin = head.len() - word.len();
        head = cut;
    }
}

/// `text` with a trailing `word` removed, where that `word` is a whole
/// identifier — `None` where it is the tail of a longer one.
fn strip_modifier<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let cut = text.strip_suffix(word)?;
    cut.chars()
        .next_back()
        .is_none_or(|c| !c.is_alphanumeric() && c != '_')
        .then(|| cut.trim_end())
}

/// The offset of the `(` that opens the round bracket `text` ends
/// with, or `None` where it does not close.
///
/// `text` is a [`code_only`] view, so every bracket is a real bracket
/// — the same precondition [`balanced_end`] states, read the other way
/// round.
fn balanced_open(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (at, byte) in text.bytes().enumerate().rev() {
        match byte {
            b')' => depth += 1,
            b'(' => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// A trait spelling as a key: its path qualification dropped, its
/// generic arguments KEPT.
///
/// The path is noise — `fmt::Display` and `core::fmt::Display` are one
/// trait. The arguments are not: `PartialEq<Other>` and `PartialEq`
/// are two impls a type may carry at once, and a key that conflated
/// them would collide on exactly the case the trait was put in it for.
fn trait_key(path: &str) -> String {
    let (base, args) = path.split_once('<').map_or((path, ""), |(b, a)| (b, a));
    let base = base.rsplit("::").next().unwrap_or(base).trim();
    if args.is_empty() {
        base.to_owned()
    } else {
        format!("{base}<{args}")
    }
}

/// Every `fn`, `const` and `static` the file declares: the byte offset
/// its head starts at, and its name.
///
/// **`const` is two items in one keyword** — `const NAME:` declares
/// one and `const fn NAME(` modifies another — and reading the second
/// as the first is how a `pub const fn` map goes unread, which is
/// exactly the shape the tag reader's `pub fn ` forms miss. So the
/// keyword decides nothing on its own; what follows it does.
///
/// **A declaration's head starts at its first ATTRIBUTE, not at its
/// `fn` line**, and that is the difference between a loud census and a
/// silent one. Attribution is by the nearest declaration at or above a
/// literal, and an outer attribute sits ABOVE the item it decorates —
/// so `#[deprecated(note = "word")]` written over one item was read as
/// a literal of the item before it. That is not merely the wrong row:
/// it INFLATES the row above by one and, paired with a deletion in the
/// same item, cancels to no complaint at all. Absorbing the attribute
/// run into the head below puts the literal on the item that carries
/// it, where the count moves the way an arrival does.
fn declaration_heads(code: &str) -> Declarations {
    let attributes = attribute_spans(code);
    let mut absorbed = vec![false; attributes.len()];
    let mut found: Vec<(usize, String)> = Vec::new();
    for keyword in DECLARATION_KEYWORDS {
        let mut from = 0usize;
        while let Some(off) = code[from..].find(keyword) {
            let at = from + off;
            from = at + keyword.len();
            if boundary_before(code, at)
                && boundary_after(code, at + keyword.len())
                && let Some(begin) = item_start(code, at)
                && let Some(name) = declaration_name(code, keyword, at)
            {
                found.push((begin, name));
            }
        }
    }
    // File order, because attribution is by the nearest head at or
    // above a literal and the scan above is per keyword.
    found.sort_by_key(|head| head.0);
    let heads: Vec<(usize, String)> = found
        .into_iter()
        .map(|(begin, name)| (head_start(code, &attributes, &mut absorbed, begin), name))
        .collect();
    let stray = attributes
        .into_iter()
        .zip(absorbed)
        .filter_map(|(span, taken)| (!taken).then_some(span))
        .collect();
    Declarations {
        heads,
        stray_attributes: stray,
    }
}

/// What [`declaration_heads`] found: the items literals are attributed
/// to, and the outer attributes that decorate none of them.
struct Declarations {
    /// Each declaration's head start and its name, in file order.
    heads: Vec<(usize, String)>,
    /// Outer attributes this reader could not attach to a declaration
    /// it knows — one on a `struct`, an `enum` or a field. A literal
    /// inside one belongs to the decorated item and NOT to the
    /// declaration above it, and since this reader cannot name that
    /// item it refuses rather than attributing the literal wrongly.
    stray_attributes: Vec<std::ops::Range<usize>>,
}

/// Every OUTER attribute in the file, as the byte range from the `#`
/// to its closing `]`.
///
/// `#![…]` is excluded: an inner attribute decorates the module it is
/// written in, not any item below it, so the literal in one is above
/// every declaration — which [`minting_owner`] refuses rather than
/// attributing.
fn attribute_spans(code: &str) -> Vec<std::ops::Range<usize>> {
    let mut spans = Vec::new();
    let mut from = 0usize;
    while let Some(off) = code[from..].find("#[") {
        let start = from + off;
        let close = balanced_end(code, start + 1).unwrap_or_else(|| {
            panic!(
                "errors.rs:{}: an attribute whose `[` does not close — I do not \
                 understand where it ends",
                test_utils::source::line(code, start)
            )
        });
        spans.push(start..close + 1);
        from = close + 1;
    }
    spans
}

/// Where a declaration's head begins: the start of the line of the
/// first attribute in the unbroken run above it, or the declaration's
/// own line start where there is none.
///
/// A doc comment between an attribute and its item does not break the
/// run: `code` is a [`code_only`] view, in which a comment is
/// whitespace.
fn head_start(
    code: &str,
    attributes: &[std::ops::Range<usize>],
    absorbed: &mut [bool],
    mut start: usize,
) -> usize {
    while let Some(index) = attributes
        .iter()
        .rposition(|span| span.end <= start && code[span.end..start].trim().is_empty())
    {
        absorbed[index] = true;
        start = line_start(code, attributes[index].start);
    }
    start
}

/// The name the declaration keyword at `at` declares, or `None` where
/// it declares nothing this reader attributes literals to.
///
/// **`const` is two items in one keyword** — `const NAME:` declares
/// one and `const fn NAME(` modifies another. The second is handed on
/// rather than read here: its head is the `fn`, which is also where
/// [`scope_spans`] takes its body from, and reading it twice would put
/// two heads on one item and hard-stop the census on a name it had
/// itself doubled.
///
/// The lexing is [`test_utils::source::ident`]'s, shared with
/// `deny_unknown_fields_census.rs`: this file had written the
/// plain-alphanumeric half of it without the raw-identifier arm, which
/// is the same reader one keyword away from reading `r#fn` as `r`. An
/// empty answer is `None`, so `fn (` is not read as declaring
/// something unnamed.
fn declaration_name(code: &str, keyword: &str, at: usize) -> Option<String> {
    let mut cursor = skip_ws(code, at + keyword.len());
    match keyword {
        "const" if ident(code, cursor) == "fn" => return None,
        "static" if ident(code, cursor) == "mut" => {
            cursor = skip_ws(code, cursor + "mut".len());
        }
        _ => {}
    }
    let name = ident(code, cursor);
    (!name.is_empty()).then(|| name.to_owned())
}

/// A declaration's name written under EVERY scope whose body holds
/// it, outermost first — `Type::name`, `<Type as Trait>::name`,
/// `module::Type::name`.
///
/// Every enclosing scope and not just the innermost: a prefix dropped
/// is two items that can answer to one key, and merging two rows into
/// one is the direction this census has no way to be loud in.
/// Enclosure nests, so the containing scope is the one that starts
/// first.
fn qualified(scopes: &[(String, std::ops::Range<usize>)], at: usize, name: &str) -> String {
    let mut enclosing: Vec<&(String, std::ops::Range<usize>)> = scopes
        .iter()
        .filter(|(_, body)| body.contains(&at))
        .collect();
    enclosing.sort_by_key(|(_, body)| body.start);
    let mut out = String::new();
    for (qualifier, _) in enclosing {
        out.push_str(qualifier);
        out.push_str("::");
    }
    out.push_str(name);
    out
}

/// Which item spells the literal at `at` — the nearest declaration
/// head at or above it, qualified.
///
/// Fails loud rather than inventing an owner. A literal with no
/// declaration above it is in a construct this reader has not been
/// taught, and attributing it to nothing would drop it from the census
/// silently. A literal inside an attribute on an item this reader does
/// not name is the SHARPER case: the nearest declaration above it is
/// the item BEFORE the one the attribute decorates, so attributing it
/// there would inflate that row by one — and a deletion in the same
/// item cancels the inflation to no complaint at all.
fn minting_owner(
    text: &str,
    scopes: &[(String, std::ops::Range<usize>)],
    decls: &Declarations,
    at: usize,
) -> String {
    assert!(
        !decls.stray_attributes.iter().any(|span| span.contains(&at)),
        "errors.rs:{}: a literal in an attribute on an item I cannot name — a \
         `struct`, an `enum`, a variant or a field. It belongs to the item the \
         attribute decorates, and charging it to the declaration above would move \
         THAT row's count instead. Teach this reader the item in the same diff.",
        test_utils::source::line(text, at)
    );
    let (start, name) = decls
        .heads
        .iter()
        .rev()
        .find(|&&(start, _)| start <= at)
        .unwrap_or_else(|| {
            panic!(
                "errors.rs:{}: a string literal above every declaration in the file — \
                 I cannot say which item spells it",
                test_utils::source::line(text, at)
            )
        });
    qualified(scopes, *start, name)
}

/// Where [`ERRORS_MINTING_ITEMS`] and the file disagree.
///
/// A function rather than a block inside the test, so that the WHOLE
/// instrument — the comparison as well as the reader — is what the
/// guard below drives over fixtures. A guard that re-implements the
/// comparison it checks is asserting about a second copy.
fn minting_complaints(found: &BTreeMap<String, Vec<String>>) -> Vec<String> {
    let mut complaints = Vec::new();
    let mut pinned: BTreeMap<&str, &MintingItem> = BTreeMap::new();
    for item in ERRORS_MINTING_ITEMS {
        assert!(
            pinned.insert(item.owner, item).is_none(),
            "ERRORS_MINTING_ITEMS names `{}` twice",
            item.owner
        );
    }
    for (owner, literals) in found {
        match pinned.get(owner.as_str()) {
            None if literals.is_empty() => complaints.push(format!(
                "NEW item `{owner}`, which no roster has looked at. It spells no \
                 literal, so nothing here says whether it puts a word on a Python \
                 wire — a map forwarding `crate::tags`' word does not, and a \
                 `&'static str` this file hands on does. Add a row to \
                 ERRORS_MINTING_ITEMS naming what holds its words, and if nothing \
                 does, that is the finding."
            )),
            None => complaints.push(format!(
                "NEW item `{owner}` spells {} literal(s) — {literals:?} — that no \
                 roster has looked at. If any of them reaches Python, it is public \
                 vocabulary with no pin; add a row to ERRORS_MINTING_ITEMS naming what \
                 holds the words, and if nothing does, that is the finding.",
                literals.len()
            )),
            Some(item) if item.literals != literals.len() => complaints.push(format!(
                "`{owner}` spells {} literal(s) — {literals:?} — and \
                 ERRORS_MINTING_ITEMS says {}. A word added here is Python-visible \
                 vocabulary; check it against {} and move the count.",
                literals.len(),
                item.literals,
                held_by_prose(item)
            )),
            Some(_) => {}
        }
    }
    for item in ERRORS_MINTING_ITEMS {
        if !found.contains_key(item.owner) {
            complaints.push(format!(
                "ERRORS_MINTING_ITEMS names `{}`, and the reader did not find it. \
                 Either it is gone — drop the row — or the reader stopped seeing it, \
                 which is this guard going blind and is the serious case.",
                item.owner
            ));
        }
    }
    complaints
}

/// One item's holders as a sentence, so a complaint about a moved
/// count tells its reader where the words it is about are held.
fn held_by_prose(item: &MintingItem) -> String {
    item.held_by
        .iter()
        .map(|holder| match holder {
            Holder::Test { name, holds } => format!("`{name}`, which holds {holds}"),
            Holder::Outside(what) => (*what).to_owned(),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

/// This crate's own `src/errors.rs` — the census's subject, and what
/// the guards below drive it over with an arrival spliced on.
///
/// `crate_dir`, not the baked path alone: a nextest ARCHIVE replayed on
/// another runner has no such directory, and this crate's own source is
/// what the census opens.
fn errors_source() -> String {
    let path = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("errors.rs");
    std::fs::read_to_string(&path).expect("this crate's own src/errors.rs")
}

/// **Nothing enumerated `src/errors.rs`, and a fifth Python-visible
/// map arrived here unpinned while the tracker row predicting it sat
/// untouched in the same diff.** This is the arrival alarm that row
/// asked for.
///
/// The roster IS the enumeration, so there is no floor: every item is
/// named, and a reader that came back with nothing reports every row
/// missing BY NAME rather than a count that drifted.
#[test]
fn errors_rs_spells_literals_in_exactly_these_items() {
    let ordered: Vec<&str> = ERRORS_MINTING_ITEMS.iter().map(|item| item.owner).collect();
    let mut by_name = ordered.clone();
    by_name.sort_unstable();
    assert_eq!(
        ordered, by_name,
        "ERRORS_MINTING_ITEMS is not in the order its doc claims — by owner"
    );

    let complaints = minting_complaints(&read_minting_items(&errors_source()));
    assert!(
        complaints.is_empty(),
        "src/errors.rs and ERRORS_MINTING_ITEMS disagree. Every literal in that \
         file is a candidate for Python-visible vocabulary, and the roster is what \
         says one has arrived.\n\n  {}",
        complaints.join("\n  ")
    );
}

/// **The mint reader's own guard.**
///
/// [`read_minting_items`] is a source-text reader, and a reader that
/// stops recognising a form does not fail — it reports agreement over
/// the set it can still see. The census above exercises it against
/// `src/errors.rs`, which holds one of some forms and none of others:
/// no generic `impl`, no `static`, no restricted visibility, no
/// literal in an attribute, one character literal, no second trait
/// impl for a type that already has one, no `mod`, no `trait`, no
/// declaration sharing a line with its scope, no declaration inside a
/// function body, no `impl` at indentation and no `impl` in type
/// position.
///
/// So this drives it over a source written to hold one of each and
/// pins what every item contributes. The expectation is spelled out
/// whole rather than derived, because a fixture is the one place in
/// this file where a hand-written list is the subject rather than the
/// defect: it is what the reader is measured AGAINST.
///
/// **Five of these rows are the reader's key and its alphabet, and
/// all of them were arrived at by running it.** `Taxonomy` carries two
/// `fmt`s — `Display` and `Debug` — which is ordinary correct Rust and
/// which a key without the trait in it cannot tell apart; they answer
/// to `<Taxonomy as Display>::fmt` and `<Taxonomy as Debug>::fmt`. It
/// carries two `eq`s as well, `PartialEq` and `PartialEq<Other>`,
/// which is the case [`trait_key`] keeps the trait's ARGUMENTS for:
/// dropping them puts both under `<Taxonomy as PartialEq>::eq` and
/// hard-stops the census on correct code. That argument was written
/// down and pinned by nothing until these two rows.
/// `SEP` carries `'/'`, a character literal, counted as a literal and
/// carried as written. And `after_the_attribute` carries the
/// `#[deprecated]` literal ABOVE it, because an outer attribute is
/// part of the head of the item it decorates and not a literal of the
/// item before it. `foreign` carries `"C"`, its own ABI string: an ABI
/// is a literal like any other and this census counts literals, not
/// vocabulary, so a row's count is what the file spells and the
/// `held_by` column is where a word is said to reach Python or not.
///
/// **Eight more rows are [`scope_spans`]'s own**, and they are here
/// because the roster over `src/errors.rs` cannot reach them — that
/// file has no `mod`, no `trait`, and every `impl` in it starts a
/// line. `nested::Taxonomy::in_a_module` and `plain::IN_PLAIN` are an
/// `impl` at indentation and a module prefix in the key;
/// `<Taxonomy as Sealed>::MARK` is an `unsafe impl`, and the module
/// above it a `pub(crate)` one, which are two of the modifiers
/// [`item_start`] admits. `returns_impl::INSIDE` sits in the body of a
/// `fn` returning `impl Display`, so a walk reading that `impl` as an
/// item head would take the FUNCTION's body for the trait's scope and
/// answer `Display::INSIDE`.
///
/// The last four are what a line-start walk could not read at all.
/// `OneLine::ALL` shares its line with the `impl` that holds it, which
/// a reader keyed on the first token of a LINE answered `{}` to — and
/// answered by inflating the row above, where a deletion in the same
/// item cancels it to silence. `Declared::DECLARED_WORD` and
/// `Declared::declared` are a `trait`'s, which was no scope at all and
/// put both under bare names. `holder::HELD` is a `const` in a
/// `const fn`'s body: the head of that function is its `fn` and not
/// its `const`, which is what keeps one item from being read as two,
/// and its body is a scope, which is what keeps `HELD` off the
/// enclosing one.
#[test]
fn the_errors_mint_reader_recognises_what_it_claims() {
    let source = r#"//! A module header with a "quoted" word the reader must not read.

use pncad::document::Dimension;

/// A doc comment whose "quoted" word is prose, not vocabulary.
pub const fn top_level_map(d: Dimension) -> &'static str {
    match d {
        Dimension::Length => "length",
        Dimension::Angle => "angle",
    }
}

pub const TOP_WORD: &str = "top";

pub const SEP: char = '/';

pub static TOP_STATIC: &str = "static_word";

pub(crate) fn restricted() -> &'static str {
    "restricted"
}

pub unsafe extern "C" fn foreign() -> &'static str {
    "foreign"
}

impl Taxonomy {
    pub const ATTRIBUTE: &'static str = "attribute";

    pub const fn inherent(self) -> &'static str {
        match self {
            Self::One => "one",
            Self::Two => "two",
        }
    }
}

impl fmt::Display for Taxonomy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the display word")
    }
}

impl fmt::Debug for Taxonomy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the debug word")
    }
}

impl PartialEq for Taxonomy {
    fn eq(&self, other: &Self) -> bool {
        self.word("same") == other.word("same")
    }
}

impl PartialEq<Other> for Taxonomy {
    fn eq(&self, other: &Other) -> bool {
        self.word("other") == other.word("other")
    }
}

impl<'a> Borrowed<'a> {
    fn borrowed(&self) -> &'static str {
        "borrowed"
    }
}

impl<const N: usize> Fixed<N> {
    fn fixed() -> &'static str {
        "fixed"
    }
}

unsafe impl Sealed for Taxonomy {
    const MARK: &'static str = "marked";
}

pub(crate) mod nested {
    impl super::Taxonomy {
        pub const fn in_a_module() -> &'static str {
            "nested"
        }
    }
}

pub mod plain {
    pub const IN_PLAIN: &str = "plain";
}

pub trait Declared {
    const DECLARED_WORD: &'static str = "declared";
    fn declared(&self) -> impl fmt::Display;
}

impl OneLine { pub const ALL: &str = "one line"; }

pub const fn holder() -> &'static str {
    const HELD: &str = "held";
    HELD
}

pub fn returns_impl() -> impl fmt::Display {
    const INSIDE: &str = "inside";
    INSIDE
}

#[deprecated(note = "an attribute literal")]
/// A doc comment between the attribute and its item does not break the
/// run: over the code view it is whitespace.
#[must_use]
pub fn after_the_attribute() -> &'static str {
    "after"
}
"#;
    let found = read_minting_items(source);
    let expected: Vec<(&str, Vec<&str>)> = vec![
        ("<Taxonomy as Debug>::fmt", vec!["the debug word"]),
        ("<Taxonomy as Display>::fmt", vec!["the display word"]),
        ("<Taxonomy as PartialEq<Other>>::eq", vec!["other", "other"]),
        ("<Taxonomy as PartialEq>::eq", vec!["same", "same"]),
        ("<Taxonomy as Sealed>::MARK", vec!["marked"]),
        ("Borrowed::borrowed", vec!["borrowed"]),
        ("Declared::DECLARED_WORD", vec!["declared"]),
        ("Declared::declared", vec![]),
        ("Fixed::fixed", vec!["fixed"]),
        ("OneLine::ALL", vec!["one line"]),
        ("SEP", vec!["'/'"]),
        ("TOP_STATIC", vec!["static_word"]),
        ("TOP_WORD", vec!["top"]),
        ("Taxonomy::ATTRIBUTE", vec!["attribute"]),
        ("Taxonomy::inherent", vec!["one", "two"]),
        ("after_the_attribute", vec!["after", "an attribute literal"]),
        ("foreign", vec!["C", "foreign"]),
        ("holder", vec![]),
        ("holder::HELD", vec!["held"]),
        ("nested::Taxonomy::in_a_module", vec!["nested"]),
        ("plain::IN_PLAIN", vec!["plain"]),
        ("restricted", vec!["restricted"]),
        ("returns_impl", vec![]),
        ("returns_impl::INSIDE", vec!["inside"]),
        ("top_level_map", vec!["angle", "length"]),
    ];
    let read: Vec<(&str, Vec<&str>)> = found
        .iter()
        .map(|(owner, literals)| {
            (
                owner.as_str(),
                literals.iter().map(String::as_str).collect::<Vec<_>>(),
            )
        })
        .collect();
    assert_eq!(read, expected, "the mint reader read the fixture wrongly");
}

/// **What makes this census go blind, executed — over both halves of
/// the instrument.**
///
/// A reader that matched nothing must not report agreement, and
/// neither must a comparison that lost its missing-row loop. The two
/// fail separately, so one source cannot drive both:
/// [`read_minting_items`] answers empty over an empty source whether it
/// is sound or broken, which leaves the reader untouched by that case.
///
/// So this drives the pair over an empty source AND over a legible
/// file holding none of the roster's items. Over both, every row
/// reports missing BY NAME — the property that keeps a pinned count
/// from passing vacuously over a population that went missing — and
/// over the second the reader's own answer is named too, which a
/// reader that came back with nothing cannot do.
#[test]
fn the_errors_mint_census_reds_when_the_file_goes_quiet() {
    let elsewhere = "pub const fn elsewhere() -> &'static str {\n    \"word\"\n}\n";
    for (what, source) in [
        ("an empty source", ""),
        ("a file this roster is not about", elsewhere),
    ] {
        let complaints = minting_complaints(&read_minting_items(source));
        for item in ERRORS_MINTING_ITEMS {
            let named = format!("names `{}`", item.owner);
            assert!(
                complaints.iter().any(|c| c.contains(&named)),
                "{what} left `{}` unreported: {complaints:?}",
                item.owner
            );
        }
    }
    let complaints = minting_complaints(&read_minting_items(elsewhere));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("NEW item `elsewhere`")),
        "the reader's own answer went unnamed, so this guard drives only the \
         comparison: {complaints:?}"
    );
}

/// **The arrival this census exists for, executed against the real
/// file.**
///
/// A sixth map, in the position the census is weakest against — an
/// inherent method inside an `impl` block, which is where both of the
/// file's unseen maps already live and which a top-level-keyed reader
/// does not walk. It reds by name.
#[test]
fn the_errors_mint_census_reds_on_a_map_arriving_inside_an_impl() {
    let arrival = format!(
        "{}\nimpl ValidationRefusal {{\n    pub const fn sixth_map(self) -> &'static str \
         {{\n        match self {{\n            Self::MassProperties => \"sixth\",\n      \
         _ => \"other\",\n        }}\n    }}\n}}\n",
        errors_source()
    );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("NEW item `ValidationRefusal::sixth_map`")),
        "a sixth map inside an `impl` did not red by name: {complaints:?}"
    );
}

/// **A map arriving inside a `mod` arrives under its QUALIFIED name.**
///
/// The walk that finds scopes is free, so an `impl` at indentation is
/// an `impl`; the module is in the key, so the name the complaint
/// prints is one a reader can go and find. A bare `seventh` was what a
/// line-start walk reported — loud, but naming an item that does not
/// exist by that name, which sends its reader looking for the wrong
/// thing.
#[test]
fn the_errors_mint_reader_keys_a_nested_impl_by_where_it_is_written() {
    let arrival = format!(
        "{}\nmod nested {{\n    impl crate::errors::ValidationRefusal {{\n        pub \
         const fn seventh(self) -> &'static str {{\n            \"seventh\"\n        \
         }}\n    }}\n}}\n",
        errors_source()
    );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("NEW item `nested::ValidationRefusal::seventh`")),
        "a map inside a `mod` did not red under its qualified name: {complaints:?}"
    );
    assert!(
        !complaints.iter().any(|c| c.contains("NEW item `seventh`")),
        "the qualifier was lost as well as carried: {complaints:?}"
    );
}

/// This file's own source — what the roster's `held_by` names are
/// re-derived against.
///
/// `crate_dir` for [`errors_source`]'s reason: a nextest ARCHIVE
/// replayed on another runner has no such directory.
fn tests_source() -> String {
    let path = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests.rs");
    std::fs::read_to_string(&path).expect("this crate's own src/tests.rs")
}

/// Whether `code` declares a `#[test] fn` called `name`.
///
/// **A `fn` is not a test, and the column says test.** This asked only
/// whether some `fn` of that name existed, so a row naming
/// `scope_spans` — a reader in this file, not a check of anything —
/// passed green while claiming a test held its words. The `#[test]`
/// above the declaration is what makes the answer the one the column
/// is about.
///
/// A free scan, guarded at both ends. [`ident`] decides the name, so
/// `fn nameish` is not `fn name`; [`boundary_before`] and
/// [`boundary_after`] decide the keyword, and neither is optional — a
/// hit inside a longer word puts the scan mid-identifier, where
/// `fnx name` answered yes to `name`. `code` is a [`code_only`] view,
/// which is what keeps a name written in a comment or a literal from
/// answering yes.
///
/// **What this cannot tell apart:** a `#[test] fn` in a
/// `macro_rules!` body reads as a declaration, and a test declared in
/// another module of this crate reads as absent. Both are wrong
/// answers a row could earn; neither is reachable from a `#[test] fn`
/// in this file, which is what every row names.
fn declares_test(code: &str, name: &str) -> bool {
    let attributes = attribute_spans(code);
    let mut from = 0usize;
    while let Some(off) = code[from..].find("fn") {
        let at = from + off;
        from = at + "fn".len();
        if boundary_before(code, at)
            && boundary_after(code, at + "fn".len())
            && ident(code, skip_ws(code, at + "fn".len())) == name
            && let Some(begin) = item_start(code, at)
            && attribute_run(code, &attributes, begin).any(|span| code[span] == *"#[test]")
        {
            return true;
        }
    }
    false
}

/// The unbroken run of outer attributes written above the item
/// starting at `begin`, innermost first.
///
/// A doc comment between two of them does not break the run: `code` is
/// a [`code_only`] view, in which a comment is whitespace. This is
/// [`head_start`]'s walk without its absorption bookkeeping, which is
/// what that walk is for and this one is not.
fn attribute_run<'a>(
    code: &'a str,
    attributes: &'a [std::ops::Range<usize>],
    begin: usize,
) -> impl Iterator<Item = std::ops::Range<usize>> + 'a {
    let mut start = begin;
    std::iter::from_fn(move || {
        let index = attributes
            .iter()
            .rposition(|span| span.end <= start && code[span.end..start].trim().is_empty())?;
        start = line_start(code, attributes[index].start);
        Some(attributes[index].clone())
    })
}

/// **The test lookup's own guard**, over text written for it rather
/// than over this file.
///
/// [`declares_test`] is read by every row of two committed lists, and
/// a lookup that answered yes to everything would leave both asserting
/// nothing. The rows that answer no are the ones the lookup got wrong
/// or could: `fnprobe()` runs the keyword into the name and is
/// [`boundary_after`]'s row, `pubfn probe()` runs it into a modifier
/// the item walk strips and is [`boundary_before`]'s, and a `fn` with
/// no `#[test]` over it is not a hypothetical — it is what let a
/// roster row name a reader and pass.
#[test]
fn the_test_lookup_recognises_what_it_claims() {
    for (what, code, expected) in [
        ("a test", "#[test]\nfn probe() {}\n", true),
        (
            "a test under a second attribute",
            "#[test]\n#[should_panic]\nfn probe() {}\n",
            true,
        ),
        ("a plain function", "fn probe() {}\n", false),
        (
            "a keyword run into its name",
            "#[test]\nfnprobe() {}\n",
            false,
        ),
        (
            "a keyword run into a modifier",
            "#[test]\npubfn probe() {}\n",
            false,
        ),
        (
            "a test of another name",
            "#[test]\nfn probeish() {}\n",
            false,
        ),
    ] {
        assert_eq!(
            declares_test(code, "probe"),
            expected,
            "{what} was read as {}a test",
            if expected { "not " } else { "" }
        );
    }
}

/// **Every test name this census writes down, re-derived.**
///
/// The census holds an item's EXISTENCE and says so; what holds its
/// WORDS is the column, and a column of prose is a claim nothing
/// checks — a renamed test leaves it pointing at nothing and every
/// row here stays green, because a name is not a literal and no
/// reader in this file was looking at it.
///
/// So every [`Holder::Test`] is looked up in this file's own source,
/// and so is every [`BlindSpot`]'s probe. The lookup is differently
/// shaped from the lists it checks: they are committed data, and this
/// reads the file's text for a declaration.
///
/// **The blind-spot list is here because a name in prose is invisible
/// to it.** This reads [`code_only`], in which a doc comment is
/// whitespace, so a test cited in a sentence can never be looked up —
/// and the unit that turned the `held_by` column from prose into data
/// then named a test three screens away, in a doc comment, under a
/// spelling this file does not declare.
///
/// **The two directions this does not close**, disclosed rather than
/// claimed away. A row whose holders are all [`Holder::Outside`]
/// names no Rust check and is not made to — that is the statement
/// `Outside` exists to make. And `holds` is a description: nothing
/// re-derives that the named test holds what the row says it holds,
/// only that the test is there.
#[test]
fn every_test_this_census_names_is_one_this_file_declares() {
    let code = code_only(&tests_source());
    // The reader must be finding tests at all, must not be finding
    // them in a name this file does not declare, and must not answer
    // for a `fn` that is no test: a lookup that answered yes to
    // everything, or no to everything, would leave every row below
    // asserting nothing, and one that answered for any `fn` is how a
    // row naming a READER passed while claiming a check.
    assert!(
        declares_test(&code, "errors_rs_spells_literals_in_exactly_these_items"),
        "the lookup found no `errors_rs_spells_literals_in_exactly_these_items` in this \
         file, so nothing below is about this census"
    );
    assert!(
        !declares_test(&code, "a_test_that_nobody_wrote"),
        "the lookup answers yes to a name this file does not declare"
    );
    assert!(
        !declares_test(&code, "read_minting_items"),
        "the lookup answers yes for `read_minting_items`, which is a reader and not a \
         test — so every row below could name one and still pass"
    );
    for spot in SCOPE_WALK_BLIND_SPOTS {
        assert!(
            declares_test(&code, spot.probe),
            "the scope walk's blind-spot list says `{}` is executed by `{}`, and this \
             file declares no such test. An entry whose probe is gone is a disclosure \
             with nothing behind it.",
            spot.cannot_see,
            spot.probe
        );
        assert!(
            !spot.unreached.is_empty(),
            "the scope walk's blind-spot list carries `{}` with nothing said about what \
             its probe does NOT reach. A probe inherits the fence of whoever wrote it, \
             and that fence is what this column is.",
            spot.probe
        );
    }
    for item in ERRORS_MINTING_ITEMS {
        assert!(
            !item.held_by.is_empty(),
            "ERRORS_MINTING_ITEMS names `{}` and says nothing holds its words. Name \
             what does, or name what does not as `Outside` — a row cannot be added \
             without answering it.",
            item.owner
        );
        for holder in item.held_by {
            let Holder::Test { name, .. } = holder else {
                continue;
            };
            assert!(
                declares_test(&code, name),
                "ERRORS_MINTING_ITEMS says `{}`'s words are held by `{name}`, and this \
                 file declares no such test. Either it was renamed — move the name here \
                 in the same diff — or it is gone and those words are held by nothing.",
                item.owner
            );
        }
    }
}

/// Which classes carry a discriminant this crate mints, the Python
/// attribute names it writes, and the ones `pncad.pyi` does NOT
/// declare.
///
/// **The third column is the population's known gap, and a gap is a
/// filed row or it is a suppression.** `ValidationError` is raised
/// with `reason` on its one measurement refusal and with `door` on its
/// four validator refusals, and the stub declares only the second — so
/// the class's Python contract is short by a word. A reason left out
/// of this column would read as agreement; listed here, the stub
/// gaining the declaration reds this test.
///
/// **What it costs to ADD to this column, which is the half the
/// argument above is silent about.** Removing an entry is loud: the
/// test reds until the stub really does declare the word. Adding one
/// flipped a red to green with a one-line edit, and the only
/// constraint was that the name be one this crate mints — so the
/// repair for `pncad.pyi` losing a declaration was indistinguishable
/// from the disclosure of a real gap. Each entry therefore names the
/// tracker row that schedules it, and
/// [`the_discriminant_attribute_names_are_declared_in_the_stub`] goes
/// and looks for that row under `work/`. A suppression now costs a
/// filed row, which is what `work/README.md` asks of every disclosure:
/// *"disclosing a residue is not scheduling it"*.
///
/// **How a reader knows the two rows are all of them.** They are the
/// two `ErrorClass` variants that carry a discriminant, which is the
/// set `crate::py::typed_err` mints from. A third would be a third
/// `ATTRIBUTES` const in `src/errors.rs`, and [`ERRORS_MINTING_ITEMS`]
/// reds on one BY NAME whether or not it spells a literal —
/// `the_errors_mint_census_reds_by_name_on_a_map_that_spells_no_literal`
/// executes exactly that shape. So the arrival is loud one file over,
/// and this roster is hand-written for what no file states: which
/// Python CLASS each const's words are written onto.
const DISCRIMINANT_ATTRIBUTE_STUBS: &[DiscriminantStub] = &[
    DiscriminantStub {
        class: "EvaluationError",
        minted: crate::errors::EvalReason::ATTRIBUTES,
        undeclared: &[],
    },
    DiscriminantStub {
        class: "ValidationError",
        minted: crate::errors::ValidationRefusal::ATTRIBUTES,
        undeclared: &[(
            "reason",
            "validation-error-reason-is-raised-and-the-stub-declares-only-door",
        )],
    },
];

/// One class's row in [`DISCRIMINANT_ATTRIBUTE_STUBS`].
struct DiscriminantStub {
    /// The Python class a raise writes these attributes onto.
    class: &'static str,
    /// The attribute names this crate mints onto it, read from the
    /// crate's own const rather than spelled again here.
    minted: &'static [&'static str],
    /// The known gaps: each an attribute name `pncad.pyi` does not
    /// declare, with the tracker row that schedules closing it.
    undeclared: &'static [(&'static str, &'static str)],
}

/// Whether `work/` holds a tracker row with this id, on any program's
/// slate.
///
/// Any slate, because a row outlives the program that filed it: a
/// finding goes onto the slate of the program whose ground it lands
/// on, and moves when that program closes. What this asks is whether
/// the row EXISTS, which is the difference between a disclosure and a
/// suppression.
fn tracker_row_exists(id: &str) -> bool {
    let work = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR")).join("work");
    let file = format!("{id}.md");
    std::fs::read_dir(&work)
        .expect("the tracker directory")
        .filter_map(Result::ok)
        .any(|slate| slate.path().join(&file).is_file())
}

/// The instance attributes `pncad.pyi` declares in one class's body.
///
/// `tests/test_stubs.py` states this stub's convention and depends on
/// it: a `Final` annotation, with or without arguments, is a
/// class-level constant, and a bare `name: type` is an instance
/// attribute — which is how every refusal payload on every
/// `PncadError` subclass is declared. This reads the second.
///
/// **The two readers of that one convention are held equal by
/// nothing, and they had already drifted.** `stub_class_names` there
/// admits `Final` and `Final[…]`; this admitted only the second, so
/// `x: Final` was a class attribute in Python and an instance
/// attribute here. No line in `pncad.pyi` spells it today
/// (`rg ': Final$' crates/pncad-py/pncad.pyi` is empty, 2026-09-15),
/// so the divergence was latent and the fixture below now holds one.
/// That is the instance closed and not the class:
/// `work/census/one-stub-convention-has-two-readers-in-two-languages.md`
/// carries the pair. This check stays in Rust because its other half
/// is `crate::errors::EvalReason::ATTRIBUTES` — a Rust const read as a
/// const, which a Python copy would have to re-spell, and re-spelling
/// a word is the defect this census exists to find.
///
/// Docstrings are skipped by triple-quote parity and not by
/// indentation, because a line inside one is arbitrary prose and this
/// stub's prose names attributes constantly.
fn stub_instance_attributes(stub: &str, class: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut inside = false;
    let mut in_doc = false;
    for line in stub.lines() {
        if line.matches("\"\"\"").count() % 2 == 1 {
            in_doc = !in_doc;
            continue;
        }
        if in_doc {
            continue;
        }
        if let Some(rest) = line.strip_prefix("class ") {
            inside = ident(rest, 0) == class;
            continue;
        }
        let Some(body) = line.strip_prefix("    ") else {
            continue;
        };
        let name = ident(body, 0);
        if !inside || name.is_empty() || !body[name.len()..].starts_with(": ") {
            continue;
        }
        let annotation = body[name.len() + ": ".len()..].trim_end();
        if annotation != "Final" && !annotation.starts_with("Final[") {
            found.insert(name.to_owned());
        }
    }
    found
}

/// **The stub reader's own guard.**
///
/// [`stub_instance_attributes`] is a text reader over a file whose
/// PROSE names attributes constantly — `pncad.pyi`'s class docstrings
/// are where this surface argues its vocabulary — so a reader that
/// stopped skipping docstrings would report attributes the stub never
/// declares, and one that stopped finding class bodies would report
/// none and agree with everything.
///
/// The real stub happens to hold no annotation-shaped line inside a
/// docstring on either class the roster reads, so nothing there drives
/// the skip. This does: one of each form, and the expectation is one
/// name.
///
/// `Bare: Final` is the form the two readers of this convention
/// disagreed on. It is a class-level constant in `test_stubs.py` and
/// was an instance attribute here, so the row that carries it pins
/// which of the two answers this file gives.
#[test]
fn the_stub_attribute_reader_recognises_what_it_claims() {
    let stub = "class Other:\n    \
                elsewhere: str\n\n\
                class Subject(Base):\n    \
                \"\"\"Prose about this class.\n\n    \
                reason: a sentence about an attribute, not a declaration of one.\n    \
                \"\"\"\n\n    \
                door: str\n    \
                Host: Final[Kind]\n    \
                Bare: Final\n    \
                def method(self) -> None: ...\n\n\
                class After:\n    \
                after: str\n";
    assert_eq!(
        stub_instance_attributes(stub, "Subject")
            .into_iter()
            .collect::<Vec<_>>(),
        vec!["door".to_owned()],
        "the stub reader read the fixture wrongly"
    );
}

/// **The words this crate writes onto a Python attribute NAME, held
/// against the stub that declares that attribute.**
///
/// `EvalReason::ATTRIBUTE` is one of the thirteen items in
/// [`ERRORS_MINTING_ITEMS`] and was the one whose column said *no Rust
/// check names this word*: the word is an attribute name, not a tag,
/// so `TAG_INVENTORY`'s population cannot reach it however it grows,
/// and renaming it moved a Python contract with nothing in Rust
/// noticing. This is that check. It is not a second spelling of the
/// word — it reads the const and goes looking for it.
#[test]
fn the_discriminant_attribute_names_are_declared_in_the_stub() {
    let stub = std::fs::read_to_string(
        test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("pncad.pyi"),
    )
    .expect("this crate's own pncad.pyi");
    for row in DISCRIMINANT_ATTRIBUTE_STUBS {
        let (class, minted, undeclared) = (row.class, row.minted, row.undeclared);
        let declared = stub_instance_attributes(&stub, class);
        assert!(
            !declared.is_empty(),
            "read no instance attribute at all out of `{class}` — this reader has \
             stopped seeing the stub's class bodies, and every row below would agree \
             with it"
        );
        for name in minted {
            if let Some((_, filed)) = undeclared.iter().find(|(gap, _)| gap == name) {
                assert!(
                    !declared.contains(*name),
                    "`{class}.{name}` is declared in `pncad.pyi` now. Drop it from \
                     DISCRIMINANT_ATTRIBUTE_STUBS's third column and close \
                     `{filed}`, which carries the gap."
                );
                assert!(
                    tracker_row_exists(filed),
                    "`{class}.{name}` is suppressed here on the ground that `{filed}` \
                     carries the gap, and no such row is filed under `work/`. A \
                     suppression with no row behind it is this column growing silently \
                     — file it in the same diff, or let the check red."
                );
            } else {
                assert!(
                    declared.contains(*name),
                    "this crate writes `{name}` onto `{class}`, and `pncad.pyi` does \
                     not declare it. A Python caller reading the stub cannot discover \
                     an attribute a raise sets."
                );
            }
        }
        for (name, _) in undeclared {
            assert!(
                minted.contains(name),
                "`{class}`'s undeclared column names `{name}`, which this crate does \
                 not write onto it"
            );
        }
    }
}

/// **A `mod` whose body is another file opens no scope here**, and the
/// items after it are not written under its name.
///
/// `mod other;` and `mod other { … }` are the same three characters to
/// a text walk. Taking the first for the second would put every
/// remaining item in the file under a module it is not in, and since a
/// module prefix is part of the key, every one of those items would
/// report NEW under a name that does not exist.
#[test]
fn the_errors_mint_reader_does_not_read_a_file_module_as_a_scope() {
    let found = read_minting_items("mod other;\n\npub const WORD: &str = \"word\";\n");
    assert_eq!(
        found.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["WORD"],
        "a file module was read as a scope"
    );
}

/// **A keyword inside a longer word opens no scope and declares
/// nothing**, executed over each of the three guards that say so.
///
/// The walk finds its keywords with `str::find`, which is a substring
/// search: `module_guard!` holds `mod`, `pubmod!` holds `mod` after a
/// word the allow-list admits, and `unsafepub` ends in one. A macro
/// invocation with a brace body is ordinary Rust and the first two are
/// what it looks like to a substring search.
///
/// Each row below fails differently with its guard removed, which is
/// why there are five. On the scope walk: [`boundary_after`] is the
/// only refusal of `module_guard!` — [`item_start`] is TRUE there, the
/// `{` of the preceding item being what precedes it — and without it
/// the items inside answer to `ule_guard::…`; [`boundary_before`] is
/// the only refusal of `pubmod!`, whose `pub` the allow-list strips to
/// nothing; and [`strip_modifier`]'s identifier boundary is the only
/// refusal of `unsafepub`, which strips to `unsafe` and then to
/// nothing. On the declaration walk the same two guards answer for
/// declarations: a field named `constant` opens with `const` and,
/// without [`boundary_after`], declares an item called `ant` — an
/// ordinary field name and the reachable one of the pair. The last
/// row is a macro's token tree, where `pubfn later` is two identifier
/// tokens: without [`boundary_before`] the allow-list strips the `pub`
/// and declares `later`. Both are names no roster carries, so both are
/// loud — under items that do not exist.
///
/// **`boundary_before` on the declaration walk needs that token tree,
/// and that is worth saying.** On every ordinary declaration the other
/// two guards already refuse the same text, so the row that drives it
/// is the one shape a text walk cannot rule out — a macro body, which
/// this file reads as text and says so two tests down.
///
/// **The third row is not valid Rust and is here anyway.** `pub` and
/// `unsafe` are keywords, so no valid program writes one as the tail
/// of an identifier next to another keyword — and this is a text walk,
/// handed macro bodies and half-written files, whose allow-list has to
/// refuse a word it was not written about rather than rely on its
/// input being a compiling program. Its literal lands on the row above
/// it, which is that refusal's visible cost and is why the row above
/// it is there.
#[test]
fn the_errors_mint_reader_refuses_a_keyword_inside_a_longer_word() {
    for (what, source, expected) in [
        (
            "a macro invocation with a brace body",
            "pub const FIRST: &str = \"first\";\n\nmodule_guard! {\n    pub const W: &str \
             = \"w\";\n}\n",
            vec![("FIRST", vec!["first"]), ("W", vec!["w"])],
        ),
        (
            "a macro whose name ends in a scope keyword",
            "pubmod! {\n    pub const W: &str = \"w\";\n}\n",
            vec![("W", vec!["w"])],
        ),
        (
            "a modifier that is the tail of a longer word",
            "pub const FIRST: &str = \"first\";\nunsafepub const W: &str = \"w\";\n",
            vec![("FIRST", vec!["first", "w"])],
        ),
        (
            "a field whose name opens with a declaration keyword",
            "struct S {\n    constant: u8,\n}\n\npub const W: &str = \"w\";\n",
            vec![("W", vec!["w"])],
        ),
        (
            "a token tree holding a modifier glued to a keyword",
            "pub const FIRST: &str = \"first\";\n\nsoup! {\n    pubfn later\n}\n",
            vec![("FIRST", vec!["first"])],
        ),
    ] {
        let found = read_minting_items(source);
        let read: Vec<(&str, Vec<&str>)> = found
            .iter()
            .map(|(owner, literals)| {
                (
                    owner.as_str(),
                    literals.iter().map(String::as_str).collect::<Vec<_>>(),
                )
            })
            .collect();
        assert_eq!(read, expected, "{what} was read as an item");
    }
}

/// **A function body is a scope**, and two functions each holding a
/// `const` of one name are two rows rather than a hard stop.
///
/// `fn a() { const W … }` beside `fn b() { const W … }` is ordinary
/// Rust, and a walk whose scopes were `impl` and `mod` alone keyed
/// both under `W` and stopped the census on a demand no author can
/// satisfy. The same shape one level in is worse than loud: a `const`
/// in a method's body answered to the enclosing IMPL, so `A::m`'s word
/// was carried by a phantom `A::W` and `A::m` itself read as spelling
/// nothing.
#[test]
fn the_errors_mint_reader_keys_a_local_declaration_by_the_function_that_holds_it() {
    let found = read_minting_items(
        "fn a() {\n    const W: &str = \"a\";\n}\n\nfn b() {\n    const W: &str = \
         \"b\";\n}\n\nimpl Subject {\n    fn m() {\n        const W: &str = \
         \"m\";\n    }\n}\n",
    );
    assert_eq!(
        found
            .iter()
            .map(|(owner, literals)| (owner.as_str(), literals.len()))
            .collect::<Vec<_>>(),
        vec![
            ("Subject::m", 0),
            ("Subject::m::W", 1),
            ("a", 0),
            ("a::W", 1),
            ("b", 0),
            ("b::W", 1),
        ],
        "a local declaration was not keyed by the function holding it"
    );
}

/// **A `trait` body is a scope**, and the items in one answer to the
/// trait's name.
///
/// The walk's scopes were `impl` and `mod`, and a `trait` is neither:
/// an associated `const` or a defaulted method landed under its bare
/// name, which is the complaint that names an item that does not exist
/// by that name. It also collided outright with a free item of the
/// same name, which is ordinary Rust — a free `fn canonical_unit`
/// beside a `trait Door { fn canonical_unit(); }` is two items and was
/// one key.
#[test]
fn the_errors_mint_reader_keys_a_trait_item_by_its_trait() {
    let found = read_minting_items(
        "pub fn canonical_unit() -> &'static str {\n    \"free\"\n}\n\npub trait Door \
         {\n    const DOOR: &'static str = \"door\";\n    fn canonical_unit() -> \
         &'static str;\n}\n",
    );
    assert_eq!(
        found
            .iter()
            .map(|(owner, literals)| (owner.as_str(), literals.len()))
            .collect::<Vec<_>>(),
        vec![
            ("Door::DOOR", 1),
            ("Door::canonical_unit", 0),
            ("canonical_unit", 1),
        ],
        "a trait item was not keyed by its trait"
    );
}

/// **An `impl` whose generic argument holds a brace loses its body**,
/// disclosed and executed rather than repaired.
///
/// [`item_body`] takes the first `{` after the keyword for the start
/// of the body, and `impl Holds<{ N }> for Subject` spells one inside
/// its generic argument — so the census reads the const-generic
/// argument as the block and every item of the real body falls outside
/// every scope. The items are then keyed bare, which is loud (a bare
/// name is on no roster) under a name that does not exist.
///
/// It is not repaired here because [`item_body`] is
/// [`test_utils::source`]'s and shared: teaching it where an `impl`
/// head ends is a widening of that module's grammar, which is a filed
/// row (`work/census/item-body-takes-a-const-generic-brace-for-an-item-body.md`)
/// and not this census's to take. The sibling census
/// `crates/test-utils/tests/hand_written_impl_census.rs` reads the
/// same helper the same way and carries the same exposure.
///
/// **A comparison in the argument is not what does it.**
/// [`test_utils::source::angle_end`] discloses a `>` closing a generic
/// list early, and `impl Holds<{ 1 > 0 }> for Subject` was written
/// down here as an instance of that residue. It is not: the braced
/// form answers the same with no comparison in it at all, and the
/// unbraced form that residue is about — a bare `1 > 0` as a const
/// generic argument — is not something Rust accepts.
#[test]
fn the_errors_mint_reader_loses_an_impl_whose_generic_argument_holds_a_brace() {
    for argument in ["{ N }", "{ 1 > 0 }"] {
        let found = read_minting_items(&format!(
            "impl Holds<{argument}> for Subject {{\n    const W: &str = \"w\";\n}}\n"
        ));
        assert_eq!(
            found.keys().map(String::as_str).collect::<Vec<_>>(),
            vec!["W"],
            "the `impl` head `Holds<{argument}>` was read as a scope after all — this \
             disclosure is stale and the entry beside it should say so"
        );
    }
}

/// **A body that never closes is a refusal**, not a scope running to
/// the end of the file.
#[test]
#[should_panic(expected = "whose body neither opens nor closes")]
fn the_errors_mint_reader_refuses_an_impl_whose_body_does_not_close() {
    read_minting_items("impl Subject {\n    pub const W: &str = \"w\";\n");
}

/// **An attribute whose `[` never closes is a refusal**, for
/// [`attribute_spans`]'s reason: an attribute run is absorbed into the
/// head of the item below it, and a run whose end this reader cannot
/// find would take an arbitrary amount of the file with it.
#[test]
#[should_panic(expected = "an attribute whose `[` does not close")]
fn the_errors_mint_reader_refuses_an_attribute_that_does_not_close() {
    read_minting_items("#[cfg(\npub const W: &str = \"w\";\n");
}

/// **Two items answering to one key is a hard stop, and the code that
/// causes it can be correct.**
///
/// A `#[cfg(test)] mod pick` beside a `#[cfg(not(test))] mod pick` is
/// ordinary Rust — one item after configuration, two to a reader that
/// has none — and this census keys on the path. It refuses, because
/// the alternative is merging two rows into one and that is the one
/// direction this census cannot be loud in. What the refusal must not
/// do is tell its reader to qualify them apart, which is what it said
/// while its premise was that Rust rejects the pair.
#[test]
#[should_panic(expected = "it is not something the author can qualify away")]
fn the_errors_mint_reader_refuses_two_items_that_answer_to_one_name() {
    read_minting_items(
        "#[cfg(test)]\nmod pick {\n    pub const W: &str = \"a\";\n}\n\n\
         #[cfg(not(test))]\nmod pick {\n    pub const W: &str = \"b\";\n}\n",
    );
}

/// **A macro body is text, and this reader reads it once**, at the
/// definition — not once per expansion, and not at the scope the
/// expansion lands in.
///
/// That is the honest description of what a text walk can say about a
/// macro, and it is a disclosure rather than a repair: an item minted
/// twice is rostered once, and an item minted at a call site in
/// another module is rostered under the module the DEFINITION sits in.
/// What no test here can reach is a macro that pastes the keyword
/// together, or a proc macro, whose output this file spells nowhere.
#[test]
fn the_errors_mint_reader_reads_a_macro_body_as_text_and_says_so() {
    let found = read_minting_items(
        "macro_rules! mint {\n    () => {\n        impl Taxonomy {\n            pub const \
         fn from_macro() -> &'static str {\n                \"from_macro\"\n            \
         }\n        }\n    };\n}\n\nmint!();\nmint!();\n",
    );
    assert_eq!(
        found
            .iter()
            .map(|(owner, literals)| (owner.as_str(), literals.len()))
            .collect::<Vec<_>>(),
        vec![("Taxonomy::from_macro", 1)],
        "the macro body was not read once at its definition"
    );
}

/// **A map that forwards another file's word is an arrival like any
/// other**, executed against the real file.
///
/// It spells no literal, and while the census's population was its
/// LITERALS such a map added nothing for the reader to see and landed
/// silent. The population is the file's DECLARATIONS now and the
/// literals are what each one contributes, so the item reds by name
/// and its author is asked the question the roster exists to ask.
#[test]
fn the_errors_mint_census_reds_by_name_on_a_map_that_spells_no_literal() {
    let arrival = format!(
        "{}\nimpl ValidationRefusal {{\n    pub const fn forwarded(self) -> &'static str \
         {{\n        crate::tags::validation_refusal_tag(self)\n    }}\n}}\n",
        errors_source()
    );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    let named: Vec<&String> = complaints
        .iter()
        .filter(|c| c.contains("NEW item `ValidationRefusal::forwarded`"))
        .collect();
    assert_eq!(
        named.len(),
        1,
        "a map forwarding another file's word did not red by name: {complaints:?}"
    );
    // The arm, and not just the name. Both NEW arms print the owner,
    // so a test asserting on the name alone passes with the tailored
    // one unreachable — and that arm is the whole user-facing payload
    // of reading a zero-literal item at all: it is what tells the
    // author that nothing here can say whether the item puts a word
    // on a wire, which is the question this row exists to ask.
    assert!(
        named[0].contains("It spells no literal, so nothing here says whether it puts a word"),
        "the zero-literal complaint was the generic arm, which reports a count rather \
         than asking the question: {:?}",
        named[0]
    );
}

/// **What this census cannot see, executed: a word channel that is
/// not a declaration.**
///
/// The population is every `fn`, `const` and `static` the file
/// declares. A struct FIELD is none of those, and
/// `QuantityOpMismatch::op` is one — a `&'static str` this file
/// carries and does not spell, reaching Python as `DimensionError.op`
/// and interpolated into the class's message. Twelve words arrive that
/// way today, every one of them minted at a call site under `src/py/`
/// and read by no instrument
/// (`work/census/dimension-error-op-carries-twelve-words-minted-at-call-sites.md`,
/// measured 2026-09-15). A SECOND such field would arrive with this
/// census silent, and that is what this executes.
///
/// Widening the reader to name a field is not the repair it looks
/// like: the words are not in this file, so seeing the channel would
/// not see them, and what holds them is a question about `src/py/`.
#[test]
fn the_errors_mint_census_cannot_see_a_word_channel_that_is_not_a_declaration() {
    let arrival = errors_source().replace(
        "    pub op: &'static str,",
        "    pub op: &'static str,\n    /// A second word this file carries and does \
         not spell.\n    pub unit: &'static str,",
    );
    assert_ne!(arrival, errors_source(), "the field splice did not land");
    // Equal AND empty. Equality alone passes over two equal non-empty
    // sides, which is what this reads as whenever the census above is
    // red for some unrelated reason — a guard that goes quiet exactly
    // when the thing it guards is already broken.
    let baseline = minting_complaints(&read_minting_items(&errors_source()));
    assert!(
        baseline.is_empty(),
        "this file disagrees with its roster already, so nothing here is about the \
         spliced field: {baseline:?}"
    );
    assert_eq!(
        minting_complaints(&read_minting_items(&arrival)),
        baseline,
        "the blind spot this test records has closed — say so and delete it"
    );
}

/// A literal this reader cannot attribute stops the census rather than
/// being dropped from it.
#[test]
#[should_panic(expected = "a string literal above every declaration")]
fn the_errors_mint_reader_refuses_a_literal_it_cannot_attribute() {
    read_minting_items("#![doc = \"above every declaration\"]\n");
}

/// A literal form this reader does not lex stops the census too: it
/// cannot say what such a form puts on the wire, and skipping it is
/// how a word arrives unseen.
#[test]
#[should_panic(expected = "a literal form I do not understand")]
fn the_errors_mint_reader_refuses_a_literal_form_it_cannot_read() {
    read_minting_items("pub fn f() -> &'static str {\n    r\"raw\"\n}\n");
}

/// Two items under one qualified name would merge into one roster row
/// — the compensating blindness a census keyed by name has to refuse.
///
/// The key carries the trait, so a type's `Display::fmt` and its
/// `Debug::fmt` are two keys and not this case; what is left is a name
/// Rust itself rejects, which means the reader has mis-read one of the
/// two items rather than the file holding both.
#[test]
#[should_panic(expected = "two items answer to `Refusal::word`")]
fn the_errors_mint_reader_refuses_two_items_under_one_name() {
    read_minting_items(
        "impl Refusal {\n    fn word() -> &'static str {\n        \"one\"\n    }\n}\n\
         impl Refusal {\n    fn word() -> &'static str {\n        \"two\"\n    }\n}\n",
    );
}

/// **A second trait impl for a type that already has one is not a
/// collision**, which is the whole of why the trait is in the key.
///
/// `impl fmt::Debug for QuantityOpMismatch` beside the existing
/// `impl fmt::Display` is ordinary, correct, rustfmt-stable Rust. Under
/// a key of `SelfType::fn_name` it hard-stopped this census on a demand
/// its author could not satisfy — two trait `fmt`s cannot be qualified
/// apart in Rust. It arrives by name now, like any other arrival.
#[test]
fn the_errors_mint_census_reds_by_name_on_a_second_trait_impl() {
    let arrival = format!(
        "{}\nimpl fmt::Debug for QuantityOpMismatch {{\n    fn fmt(&self, f: &mut \
         fmt::Formatter<'_>) -> fmt::Result {{\n        write!(f, \"a debug word\")\n    \
         }}\n}}\n",
        errors_source()
    );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("NEW item `<QuantityOpMismatch as Debug>::fmt`")),
        "a second trait impl for a type that already has one did not red by name: \
         {complaints:?}"
    );
}

/// **An item spelling only a CHARACTER literal is an arrival too.**
///
/// A `char` puts no word on a Python wire, and a reader that read it
/// and dropped it dropped the ITEM with it: this splice — a whole new
/// `pub const fn` on a rostered type — left the census silent. The
/// population is the file's literals, so a character literal counts
/// toward its item's tally and the item appears.
#[test]
fn the_errors_mint_census_reds_by_name_on_an_item_spelling_only_a_char() {
    let arrival = format!(
        "{}\nimpl ValidationRefusal {{\n    pub const fn sep(self) -> char {{\n        \
         '/'\n    }}\n}}\n",
        errors_source()
    );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("NEW item `ValidationRefusal::sep`")),
        "an item spelling only a character literal did not red by name: {complaints:?}"
    );
}

/// **A literal in an attribute belongs to the item below it**, and the
/// census is loud about it even when a deletion would have cancelled
/// the count.
///
/// Attribution is by the nearest declaration head, and an outer
/// attribute sits above its item — so `#[deprecated(note = "…")]`
/// written over one item was charged to the item BEFORE it. That
/// inflates a rostered row rather than inventing an unrostered name,
/// which is the quiet direction: drop a word from that same row in the
/// same diff and the two cancel to no complaint. Both moves at once,
/// executed.
#[test]
fn the_errors_mint_census_reds_on_an_attribute_literal_that_would_cancel() {
    let arrival = errors_source()
        .replace(
            "pub const ATTRIBUTES: &'static [&'static str] = &[\"door\", \"reason\"];",
            "pub const ATTRIBUTES: &'static [&'static str] = &[\"door\"];",
        )
        .replace(
            "    pub const fn attribute(self) -> &'static str {",
            "    #[deprecated(note = \"probe_word\")]\n    pub const fn attribute(self) \
             -> &'static str {",
        );
    let complaints = minting_complaints(&read_minting_items(&arrival));
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("`ValidationRefusal::ATTRIBUTES` spells 1")),
        "the deletion from ATTRIBUTES went unreported: {complaints:?}"
    );
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("`ValidationRefusal::attribute` spells 3")),
        "the attribute literal was not charged to the item it decorates: {complaints:?}"
    );
}

/// A literal in an attribute on an item this reader cannot name stops
/// the census rather than moving the count of the declaration above it.
#[test]
#[should_panic(expected = "a literal in an attribute on an item I cannot name")]
fn the_errors_mint_reader_refuses_an_attribute_on_an_item_it_cannot_name() {
    read_minting_items(
        "pub fn f() -> &'static str {\n    \"word\"\n}\n\n\
         #[serde(rename = \"renamed\")]\nstruct Payload {\n    field: u8,\n}\n",
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
        let applied = d::apply(
            &doc,
            &d::DocEdit::InsertNode { node },
            Tol::witness(),
            &pncad::document::RefusingReach,
        )
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
