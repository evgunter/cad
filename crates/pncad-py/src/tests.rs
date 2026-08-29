//! Tests for the Python-independent half of the crate.
//!
//! These run on the DEFAULT build path — no `python` feature, no
//! interpreter — which is the point: hosted CI executes them without a
//! Python toolchain present.

// Per the workspace convention recorded in the root Cargo.toml: test
// code may allow the panic family, because panicking IS a test's
// failure mechanism.
#![allow(clippy::expect_used, clippy::panic)]

use crate::errors::{ErrorClass, QuantityOpMismatch, canonical_unit, dimension_tag};
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
        }
    }
    for class in [
        ErrorClass::Edit,
        ErrorClass::Evaluation,
        ErrorClass::Validation,
        ErrorClass::Dimension,
        ErrorClass::Literal,
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
    ] {
        assert_eq!(class.class_name(), expected(class));
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
/// `the_load_door_reaches_dimension_mismatch_arms_as_an_untyped_parse_refusal`
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

/// **The second door.** `WireExpr::rebuild` (the load path) re-runs
/// every dimension check through `Expr`'s OPERATOR builders, so a
/// hand-edited save file reaches the genuine dimension-mismatch arms
/// with no new binding at all — six of them, executed here.
///
/// Today they arrive in Python as `PersistError` with `variant ==
/// "parse"`, because the deserializer `Debug`-formats the structured
/// refusal into a serde message. That is a real misrouting and it is
/// **issue #694**, not this crate's to fix: a dimension mismatch is
/// not a parse failure, and a `format!("{err:?}")` message is not the
/// "typed exception carrying the structured error" this crate's
/// taxonomy promises.
///
/// What this test is for is the DECISION the fix will force. When
/// #694 gives these a typed class, this assertion goes red, and
/// whoever changes it has to answer the question the three names make
/// easy to get wrong: a dimension mismatch from the load path is not
/// a `LiteralError` (nothing about it is a literal) and it is not the
/// quantity boundary's `DimensionError` either.
#[test]
fn the_load_door_reaches_dimension_mismatch_arms_as_an_untyped_parse_refusal() {
    let tol = Tol::witness();
    use pncad::document::{DocEdit, LoopProgram, Node, ProfileDoc, ProfileProgram, apply, save};
    use pncad::prelude::SketchPlane;

    let doc: ProfileDoc = crate::identity::derived("dimension-routing-probe", tol);
    let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
        .expect("finite corners");
    let applied = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(ProfileProgram {
                plane: SketchPlane::xy(),
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
            "parse",
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
    assert_eq!(persist_error_tag(&header), "header");
    let unknown = pncad::document::load("schema: 9999\n{}", Tol::witness())
        .expect_err("a future schema refuses");
    assert_eq!(persist_error_tag(&unknown), "unknown_schema");
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

    let overdetermined = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.0, 0.0), Tol::witness())
        .expect("a leg east")
        .tangent()
        .tangent_arc_to(Start, Tol::witness())
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
