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
    expr_dimension_error_tag, path_error_tag, persist_error_tag, step_import_error_tag,
    workspace_error_tag,
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
    ] {
        assert_eq!(class.class_name(), expected(class));
    }
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
/// Two arms have no façade constructor and so no line here — the
/// `select_refusal_tags_are_stable` caveat, for a different reason.
/// `HitTestError::Unnamed`'s payload is an `EntityRef`, an arena key
/// beside a body index, and the façade deliberately does not name that
/// type; `NodePickError::Index`'s payload is `MeshPickError`, which
/// CUR3 recorded DECIDED absent. Both tags are covered by the matches
/// themselves, which are exhaustive and would stop compiling if an arm
/// moved.
#[test]
fn picking_refusal_tags_are_stable() {
    use crate::tags::{hit_test_error_tag, node_pick_error_tag};
    use pncad::document::RecipeNodeId;
    use pncad::mesh::TessellateError;
    use pncad::select::{HitTestError as H, NodePickError as N};

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
/// and filling its fields. That is unavailable here: `Resolved`,
/// `ResolutionFailure` and `ResolveIndeterminate` are decided absent
/// from the façade (`crates/pncad/tests/all.rs`'s `NOT_CARRIED`), so a
/// `Resolution` cannot be assembled at all through `pncad` — it can
/// only be OBTAINED, by resolving a real name against a real run. So
/// this test builds a document, and the three states are three things
/// that happen to it.
///
/// That is a stronger pin than the literal one it replaces, and worth
/// naming as such: it asserts that each state is REACHABLE by the
/// route a caller reaches it, not merely that a match arm returns a
/// string. It runs on the default no-Python path, so hosted CI checks
/// the words a Python caller branches on without an interpreter.
///
/// The `ambiguous` and `vanished` failures are not separately reached
/// and do not need to be: they are the same `failed` word by the same
/// arm of the same match, and what distinguishes them does not cross
/// (this function's own doc comment says why).
#[test]
fn resolution_status_tags_are_stable() {
    use crate::tags::resolution_status_tag;
    use pncad::document::{
        CancelToken, Datum, DocEdit, EvalOptions, Expr, LoopProgram, Node, ProfileDoc,
        ProfileProgram, apply, evaluate,
    };
    use pncad::prelude::Dimension;
    use pncad::select::{RunCtx, all_faces, resolve};

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
    assert_eq!(
        resolution_status_tag(&resolve(
            RunCtx {
                doc: &pruned,
                eval: &after
            },
            &stored
        )),
        "failed"
    );

    // INDETERMINATE: the node is still there and the RUN did not reach
    // it — a canceled run's suffix, which is the one arm of this state
    // reachable without breaking a feature. The name is unharmed and
    // the repair is to evaluate again, which is exactly why this must
    // not answer `failed`.
    let canceled = CancelToken::new();
    canceled.cancel();
    let partial = run(&doc, &canceled);
    assert_eq!(
        resolution_status_tag(&resolve(
            RunCtx {
                doc: &doc,
                eval: &partial
            },
            &stored
        )),
        "indeterminate"
    );
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

#[test]
fn path_error_tags_are_stable() {
    use pncad::prelude::{Open, Start, circle, p2};

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
/// its tag. Two arms of each map carry kernel internals with no public
/// constructor — [`ChecksError::Band`]'s `BandError`, and the shell
/// door's refusal behind `Escalated`/`Unsupported` — so their tags are
/// covered by the exhaustive match alone, which is the real alarm
/// here: neither enum is `#[non_exhaustive]`, so a kernel arm added
/// without a tag stops this crate compiling.
///
/// What the pin adds over the match is the STRINGS. A tag is the
/// branchable half of a typed refusal, so renaming one is a surface
/// break the compiler cannot see.
#[test]
fn check_registry_tags_are_stable() {
    use crate::tags::{check_evidence_tag, checks_error_tag};
    use pncad::document::{CheckEvidence, ChecksError, RecipeNodeId};

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
            reason: "boxes refused".into()
        }),
        "separation_unavailable"
    );
}

/// **The tier-3′ census findings do not read as prose, and that is a
/// KERNEL rendering, not a binding one.**
///
/// `crate::py::typed_err` asserts every message it raises satisfies
/// [`reads_as_prose`], and every door in this crate obeys the rule the
/// assertion stands for — the binding never authors a `Debug` dump.
/// Three `ValidationError` arms are worded by the kernel out of
/// `Debug` anyway: `UndeclaredContact` renders its `CensusContact` as
/// `{contact:?}` and carries a `witness` the kernel builds with
/// `format!("{p:?}")`, and `StaleContactDeclaration` renders its
/// `DeclaredContact` the same way. Only tier 3′ produces those arms,
/// so `Body::validate_pseudomanifold` is the first door to reach them
/// — and reach them it does, on an ordinary call: two touching solids
/// gathered by `product`, which declares nothing.
///
/// This pin is the reason `run_validator` raises through
/// `typed_err_kernel_authored`. It is deliberately an assertion that
/// the message is NOT prose, so the day the kernel renders these arms
/// through `Display` (filed at
/// `work/lib/tier-3-prime-findings-render-through-debug.md`) this row
/// goes red and the exemption can go with it.
#[test]
fn the_census_findings_are_not_prose_by_this_crate_s_own_rule() {
    use pncad::topo::{CensusContact, StaleDeclaration, ValidationError};

    // Both arms are built here rather than by evaluating a document:
    // this row is about the RENDERING, and a default arena key is
    // enough to render one — nothing dereferences it.
    let census = ValidationError::UndeclaredContact {
        contact: CensusContact::VertexOnFace {
            vertex: VertexKey::default(),
            face: FaceKey::default(),
        },
        // The kernel builds this field with `format!("{p:?}")`
        // (`census::witness`), so it carries braces of its own.
        witness: format!("{:?}", pncad::geom_core::Point3::<f64>::origin()),
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
            !reads_as_prose(&message),
            "a tier-3′ census finding reads as prose now — the kernel \
             rendering was fixed. Drop `typed_err_kernel_authored` and \
             its exemption, let `run_validator` raise through \
             `typed_err` again, and close the filed item. Message: \
             {message}"
        );
        assert!(
            message.contains(" { "),
            "the struct-brace fingerprint is exactly what \
             `reads_as_prose` rejects; without it the arm was \
             reworded: {message}"
        );
    }

    // The recourse survives the Debug guts: what a Python caller
    // reads is unusable as a TAG but is still the kernel's whole
    // diagnosis, which is why the binding pastes it rather than
    // inventing a second wording.
    assert!(
        census.to_string().contains("never blessed from discovery"),
        "the undeclared-contact recourse is the actionable half"
    );
}

// ---------------------------------------------------------------
// The tag table's VALUES, pinned as a set.
// ---------------------------------------------------------------

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
        function: "assembly_error_tag",
        values: &[
            "at_rest",
            "mate_reference_refused",
            "no_at_rest_record",
            "uncertified",
        ],
        delegates: &["product_error_tag"],
    },
    TagEntry {
        function: "binary_header_error_tag",
        values: &["binary_header_sniffs_ascii", "binary_header_too_long"],
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
        values: &["band", "product_unavailable", "root_without_value"],
        delegates: &[],
    },
    TagEntry {
        function: "declare_error_tag",
        values: &["no_findings", "no_minted_id"],
        delegates: &["edit_error_tag"],
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
            "declare_names_missing_node",
            "delete_would_dangle",
            "dimension",
            "doc_param_dimension_mismatch",
            "doc_param_not_declared",
            "doc_param_value_kind_mismatch",
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
            "placement_on_non_instance",
            "placement_rule_mismatch",
            "profile_program_refused",
            "rebind_appearance_collision",
            "rebind_identity",
            "rebind_kind_mismatch",
            "rebind_metadata_collision",
            "rebind_no_references",
            "rebind_target_missing_node",
            "rebind_unknown_name",
            "slot_dimension_mismatch",
            "structural_slot_needs_structural_edit",
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
        function: "mate_fault_tag",
        values: &[
            "mate_band",
            "mate_class_not_admitted",
            "mate_contradictory",
            "mate_dangling_head",
            "mate_datum_too_small_to_lever",
            "mate_frame_degenerate",
            "mate_indeterminate",
            "mate_self",
            "mate_table_lacks",
            "mate_under",
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
            "empty_operand",
            "escalated",
            "expr",
            "extrude",
            "fillet",
            "fillet_selection_empty",
            "fillet_selection_kind",
            "fillet_selection_resolve",
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
            "payload_expr",
            "placements_uncertified",
            "profile",
            "profile_anchor",
            "profile_lane_replay",
            "profile_replay",
            "revolve",
            "seed",
            "seed_pinned_section",
            "skin",
            "split",
            "tolerance_conflict",
            "transform",
            "tube",
            "undeclared_contact",
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
        function: "node_pick_error_tag",
        values: &["mesh_index", "no_such_body", "not_a_body"],
        delegates: &["hit_test_error_tag", "tessellate_error_tag"],
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
            "anchor_outside_trimmed_extent",
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
            "fillet_encloses_leg_carrier",
            "fillet_offset_lever_too_short",
            "guided_structure",
            "junction_cusp",
            "junction_tangent",
            "no_corner_for_fillet",
            "nonpositive_circle_radius",
            "nonpositive_fillet_radius",
            "nonpositive_leg",
            "overdetermined_junction",
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
            "ref_node_gone",
            "ref_not_a_face",
            "ref_vanished",
        ],
        delegates: &[],
    },
    TagEntry {
        function: "resolution_status_tag",
        values: &["failed", "indeterminate", "resolved"],
        delegates: &[],
    },
    TagEntry {
        function: "resolve_fault_tag",
        values: &["part_epsilon_seam", "part_pin_mismatch", "part_unresolved"],
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
        function: "update_error_tag",
        values: &["already_pinned", "no_such_reference"],
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
    /// Exactly four shapes are recognised, which is the enumerating
    /// claim this whole reader rests on: a string literal, a nested
    /// `match`, a `{ ... }` block around one of those, or a call to
    /// another tag function. Anything else — a `format!`, a `if`, a
    /// `const` reference, a method chain — fails here by name rather
    /// than being skipped, because a tag arrived at by a route this
    /// reader cannot follow is a tag the inventory silently stops
    /// covering.
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
                tail.ends_with(") -> &'static str {"),
                "tags.rs:{number}: a `pub fn` in the tag module whose signature is \
                 not `(..) -> &'static str {{` on one line — I do not understand \
                 this, and cannot say what it puts on the wire: {line}"
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
/// **And the construction pins are sampled, not total.** Eighteen of
/// the thirty-seven functions carry at least one — `interrogate`,
/// `readback`, `hit_test`, `node_pick`, `tessellate`,
/// `resolution_status`, `select_refusal`, `declare_error`,
/// `expr_dimension`, `fmt_quantity`, `parse_error`, `eval_error`,
/// `persist_error`, `workspace_error`, `step_import_error`,
/// `path_error`, `checks_error`, `check_evidence` — and even those
/// are samples (`persist_error_tag`: two of thirteen arms;
/// `step_import_error_tag`: two of twenty-two; `path_error_tag`: three
/// of thirty). The other nineteen — `assembly`, `binary_header`,
/// `edit`, `export`, `frame`, `inline`, `mate_fault`, `node_error`,
/// `part_fault`, `placement_rule_fault`, `product`,
/// `recorded_program`, `refused_ref`, `resolve_fault`, `root_fault`,
/// `solid_name`, `split`, `stl`, `update` — have none, and between
/// them hold 192 of the table's 354 literals, `edit_error_tag`'s
/// fifty and `node_error_tag`'s fifty-four included. For those the
/// inventory below is the ONLY thing between a rename and a broken
/// caller. That is a large gain over nothing; it is not the same claim
/// as "the tag table is verified", and this comment refuses to make
/// the second one.
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
    // They are set well under the real numbers (37 functions, 354
    // literal occurrences) so ordinary churn does not touch them.
    assert!(
        table.functions.len() >= 30,
        "the reader found only {} tag functions in src/tags.rs — it is \
         matching almost nothing, so this guard was about to pass \
         vacuously",
        table.functions.len()
    );
    let literals: usize = table.functions.values().map(|(v, _)| v.len()).sum();
    assert!(
        literals >= 250,
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
