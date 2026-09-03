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
use std::collections::BTreeMap;
use std::path::Path;

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
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
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
