//! Tests for the Python-independent half of the crate.
//!
//! These run on the DEFAULT build path — no `python` feature, no
//! interpreter — which is the point: hosted CI executes them without a
//! Python toolchain present.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use crate::errors::{DimensionError, ErrorClass, canonical_unit, dimension_tag};
use crate::tags::{expr_dimension_error_tag, path_error_tag, persist_error_tag};
use pncad::document::Dimension;
use std::collections::BTreeMap;
use std::path::Path;

#[test]
fn dimension_tags_are_stable() {
    assert_eq!(dimension_tag(Dimension::Length), "length");
    assert_eq!(dimension_tag(Dimension::Angle), "angle");
    assert_eq!(dimension_tag(Dimension::Count), "count");
    assert_eq!(dimension_tag(Dimension::Scalar), "scalar");
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
fn dimension_error_carries_structure_not_prose() {
    let err = DimensionError::new("+", Dimension::Length, Dimension::Angle);
    assert_eq!(err.op, "+");
    assert_eq!(err.left, Dimension::Length);
    assert_eq!(err.right, Dimension::Angle);
    // The message exists for humans, but the fields above are the
    // contract (§L4: never strings).
    assert_eq!(err.to_string(), "cannot apply `+` to length and angle");
}

#[test]
fn error_classes_name_the_python_hierarchy() {
    assert_eq!(ErrorClass::Edit.class_name(), "EditError");
    assert_eq!(ErrorClass::Evaluation.class_name(), "EvaluationError");
    assert_eq!(ErrorClass::Validation.class_name(), "ValidationError");
    assert_eq!(ErrorClass::Dimension.class_name(), "DimensionError");
    assert_eq!(ErrorClass::Literal.class_name(), "LiteralError");
    assert_eq!(ErrorClass::Persist.class_name(), "PersistError");
    assert_eq!(ErrorClass::Export.class_name(), "ExportError");
    assert_eq!(ErrorClass::StepImport.class_name(), "StepImportError");
    assert_eq!(ErrorClass::Path.class_name(), "PathError");
    assert_eq!(ErrorClass::Select.class_name(), "SelectRefusal");
}

/// LIB-PYSEL: `SelectRefusal` is `#[non_exhaustive]`, so the tag
/// match cannot be the compile-time drift alarm the other tag
/// functions are — this pin is the replacement. It constructs every
/// arm whose payload the curated surface can build and asserts its
/// tag, so a kernel arm arriving untagged surfaces as `unclassified`
/// HERE rather than silently in Python. (`InBand`/`PairInBand`/
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

/// LIB-DOORS F5: the binding matches `Expr::literal`'s OWN refusals
/// (the `check_literal` pre-check is gone), and the tags Python sees
/// for the two literal-reachable arms are unchanged from LIB-U9S.
#[test]
fn literal_refusals_come_from_the_kernel_with_stable_tags() {
    use pncad::document::Expr;
    let non_finite = Expr::literal(f64::NAN, Dimension::Length).expect_err("NaN refuses");
    assert_eq!(expr_dimension_error_tag(&non_finite), "non_finite");
    let count = Expr::literal(3.0, Dimension::Count).expect_err("a continuous count refuses");
    assert_eq!(expr_dimension_error_tag(&count), "count_is_integer");
    assert!(Expr::literal(1.5, Dimension::Length).is_ok());
}

/// LIB-DOORS F1: a load refusal's tag, exercised through the real
/// door (the exhaustive match itself is the drift alarm; this pins
/// two tags' spellings against the wire).
#[test]
fn persist_error_tags_are_stable() {
    let header = pncad::document::load("not a header").expect_err("garbage refuses");
    assert_eq!(persist_error_tag(&header), "header");
    let unknown = pncad::document::load("schema: 9999\n{}").expect_err("a future schema refuses");
    assert_eq!(persist_error_tag(&unknown), "unknown_schema");
}

#[test]
fn path_error_tags_are_stable() {
    use pncad::prelude::{Open, Start, circle, p2};

    let zero = circle(p2(0.0, 0.0), 0.0).expect_err("a zero radius refuses");
    assert_eq!(path_error_tag(&zero), "nonpositive_circle_radius");

    let tangent = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0))
        .expect("a leg east")
        .angle(0.0)
        .expect_err("a corner tangent to its incoming leg refuses");
    assert_eq!(path_error_tag(&tangent), "junction_tangent");

    let overdetermined = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0))
        .expect("a leg east")
        .tangent()
        .tangent_arc_to(Start)
        .expect_err("a tangent LINE close refuses always");
    assert_eq!(path_error_tag(&overdetermined), "tangent_line_close");
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
/// `unsafe impl`), so the table is restated by hand — and an earlier
/// revision of this file silently dropped four clippy lints while
/// claiming it had kept them all. This test turns that claim into an
/// enforced invariant: adding a lint to `[workspace.lints]` breaks
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
