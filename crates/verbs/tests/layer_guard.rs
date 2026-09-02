//! **The layering line, enforced rather than asserted in a comment.**
//!
//! `verbs` sits above the op crates and below the document layer, and
//! the rule is that the recipe vocabulary never crosses down into it:
//! no serde, no `Expr`, no `StableName`, no `RecipeNodeId`, no name
//! table. What may sit here is lowered pure data compared for identity
//! — the `topo::source` precedent.
//!
//! Two guards, because they fail differently. The MANIFEST guard is the
//! strong one: serde reaches a crate only through a dependency edge, so
//! a manifest with no serde-family entry cannot carry a derive whatever
//! the source says. (`scripts/gates/kernel-serde-free.sh` covers this
//! crate too — it scans every `crates/*/Cargo.toml` outside its
//! two-crate allowlist — and this row is the in-crate half that fails
//! at the same time rather than a mile away.) The SOURCE guard is the
//! weaker one: it catches the document-layer type names arriving by any
//! route a reader would recognize.
//!
//! **What the source guard cannot see**, stated so it is not
//! over-trusted: a document-layer type reached under an alias
//! (`use editor_core::names::StableName as Frozen`), or one named only
//! through an associated type or a generic parameter's bound. What
//! makes those unreachable in practice is not this scan but the
//! dependency graph — `editor-core` is ABOVE this crate, so naming any
//! of its types would need a cyclic dependency cargo refuses to build.
//! The scan's job is the case cargo permits: a serde or expression
//! vocabulary arriving from somewhere else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// Every file of this crate's own source, name and text. The guards
/// below read this one list; `the_guard_scans_every_source_file` pins
/// it against the directory, so a new module cannot arrive unguarded.
const SOURCES: [(&str, &str); 4] = [
    ("lib.rs", include_str!("../src/lib.rs")),
    ("verb.rs", include_str!("../src/verb.rs")),
    ("run.rs", include_str!("../src/run.rs")),
    ("flow.rs", include_str!("../src/flow.rs")),
];

/// The manifest, read as text for the dependency-name scan below.
const MANIFEST: &str = include_str!("../Cargo.toml");

/// **No serde reaches this crate**, checked where it would have to
/// enter: the dependency tables.
#[test]
fn the_manifest_names_no_serde_dependency() {
    // Assembled rather than written contiguously: this file is itself
    // read by no scanner today, but the sibling test below reads THIS
    // crate's sources, and a literal here would be a first match if the
    // list ever widened to `tests/`.
    let serde = ["ser", "de"].concat();
    let hits: Vec<&str> = MANIFEST
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter(|l| l.contains(&serde))
        .collect();
    assert!(
        hits.is_empty(),
        "the verbs manifest names serde: {hits:?}. Persistence is the document layer's job — \
         describe the bytes above the boundary."
    );
}

/// **No document-layer vocabulary is named in this crate's source.**
///
/// The names are the recipe layer's, one per thing the §0 line keeps
/// above: an authored expression, a persisted name, a typed node id, a
/// name table, and serde's own two derives.
#[test]
fn no_document_layer_type_is_named_in_the_source() {
    let forbidden: [String; 6] = [
        ["Serial", "ize"].concat(),
        ["Deserial", "ize"].concat(),
        ["Stable", "Name"].concat(),
        ["Recipe", "NodeId"].concat(),
        ["Name", "Table"].concat(),
        ["editor", "_core"].concat(),
    ];
    let mut violations: Vec<String> = Vec::new();
    for (name, src) in SOURCES {
        for line in src.lines() {
            // Prose may DISCUSS what is excluded — that is what the
            // module docs are for — so the scan reads code only. A
            // doc line is a LEADING `///`, `//!` or `//`; a TRAILING
            // comment still counts as code, which errs toward firing
            // rather than missing and is the same direction the LB13
            // façade guard takes.
            let trimmed = line.trim_start();
            if trimmed.starts_with("//") {
                continue;
            }
            for bad in &forbidden {
                if line.contains(bad.as_str()) {
                    violations.push(format!("{name}: {bad} in `{}`", line.trim()));
                }
            }
        }
    }
    assert!(
        violations.is_empty(),
        "document-layer vocabulary named in the verbs crate: {violations:#?}"
    );
}

/// The guards above scan a hand-written list; this pins the list
/// against the directory, so a module added without a row here fails
/// rather than going unscanned.
#[test]
fn the_guard_scans_every_source_file() {
    let src_dir = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found: Vec<String> = std::fs::read_dir(&src_dir)
        .expect("the crate has a src/")
        .map(|e| e.expect("a readable entry").file_name())
        .filter_map(|n| n.to_str().map(str::to_owned))
        .filter(|n| n.ends_with(".rs"))
        .collect();
    found.sort();
    let mut listed: Vec<String> = SOURCES.iter().map(|(n, _)| (*n).to_owned()).collect();
    listed.sort();
    assert_eq!(
        listed, found,
        "the verbs layer guard's file list has drifted from src/; every source file must be \
         scanned or the guard reports a hollow pass"
    );
    // A nested module directory would be missed by the flat read above,
    // which is a hole rather than a violation only while none exists.
    let nested: Vec<String> = std::fs::read_dir(&src_dir)
        .expect("the crate has a src/")
        .map(|e| e.expect("a readable entry"))
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().to_str().map(str::to_owned))
        .collect();
    assert!(
        nested.is_empty(),
        "src/ has grown subdirectories {nested:?}; the guard's flat scan no longer covers the \
         crate — widen it before adding one"
    );
}
