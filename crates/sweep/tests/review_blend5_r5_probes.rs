//! BLEND unit 5 review probes (lane r5, PR 2155): a doc comment in
//! `crates/sweep/src/**` that names a test row as its evidence is a
//! citation, and these rows RESOLVE every one of them.
//!
//! **Two spellings are live in the tree, and each row here reads one.**
//! The path spelling `crates/sweep/tests/<file>.rs::<row>` is the one
//! unit 5 normalised to; the module spelling `<file>::<row>` (no
//! `.rs`, no path) is what `cargo test -p sweep --test all --
//! <file>::<row>` actually filters on, because `tests/all.rs` mounts
//! every suite as a module of one binary. A rename or a deletion of a
//! cited row goes red here in either spelling; a row that still exists
//! and asserts less than the sentence says is the half no resolver can
//! see, and is not claimed.
//!
//! Prose is read through `test_utils::source::comments_only`, so a
//! citation inside a string literal or a `#[doc = "…"]` attribute is
//! not in the corpus; the suite's `#[test]` is read through
//! `code_only`, so a commented-out row does not answer for a name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::{Path, PathBuf};

use test_utils::source::{code_only, comments_only, crate_dir, rust_sources};

/// A citation as found: where it sits, and the suite and row it names.
#[derive(Debug)]
struct Citation {
    at: String,
    suite: String,
    row: String,
}

fn ident_end(s: &str, from: usize) -> usize {
    s[from..]
        .find(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
        .map_or(s.len(), |n| from + n)
}

/// The sweep crate's `src/` files as `(path relative to `src/`, prose-only view)`.
fn sweep_prose() -> Vec<(String, String)> {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out: Vec<(String, String)> = rust_sources(&src)
        .into_iter()
        .map(|p: PathBuf| {
            let rel = p
                .strip_prefix(&src)
                .expect("a sweep source lies under src/")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("readable {}: {e}", p.display()));
            (rel, comments_only(&text))
        })
        .collect();
    out.sort();
    out
}

/// The line number (1-based) of byte offset `at` in `text`.
fn line_of(text: &str, at: usize) -> usize {
    text[..at].matches('\n').count() + 1
}

/// Every `crates/sweep/tests/<file>.rs::<row>` in the prose.
fn path_citations() -> Vec<Citation> {
    const NEEDLE: &str = "crates/sweep/tests/";
    let mut out = Vec::new();
    for (rel, prose) in sweep_prose() {
        let mut from = 0;
        while let Some(i) = prose[from..].find(NEEDLE) {
            let start = from + i + NEEDLE.len();
            let file_end = ident_end(&prose, start);
            from = file_end;
            if !prose[file_end..].starts_with(".rs::") {
                continue;
            }
            let row_start = file_end + ".rs::".len();
            let row_end = ident_end(&prose, row_start);
            if row_end == row_start {
                continue;
            }
            out.push(Citation {
                at: format!("src/{rel}:{}", line_of(&prose, start)),
                suite: prose[start..file_end].to_owned(),
                row: prose[row_start..row_end].to_owned(),
            });
            from = row_end;
        }
    }
    out
}

/// Every backticked `<module>::<row>` in the prose whose `<module>.rs`
/// is a suite file under `tests/` — the spelling `cargo test` filters on.
fn module_citations(tests: &Path) -> Vec<Citation> {
    let mut out = Vec::new();
    for (rel, prose) in sweep_prose() {
        let mut from = 0;
        while let Some(i) = prose[from..].find('`') {
            let start = from + i + 1;
            let module_end = ident_end(&prose, start);
            from = start;
            if module_end == start || !prose[module_end..].starts_with("::") {
                continue;
            }
            let row_start = module_end + 2;
            let row_end = ident_end(&prose, row_start);
            if row_end == row_start || !prose[row_end..].starts_with('`') {
                continue;
            }
            let module = &prose[start..module_end];
            if !tests.join(format!("{module}.rs")).is_file() {
                continue;
            }
            out.push(Citation {
                at: format!("src/{rel}:{}", line_of(&prose, start)),
                suite: module.to_owned(),
                row: prose[row_start..row_end].to_owned(),
            });
            from = row_end;
        }
    }
    out
}

/// Whether `code` (a code-only view) declares `#[test] fn <row>(`, with any
/// other attributes between the two allowed.
fn declares_test_row(code: &str, row: &str) -> bool {
    let needle = format!("fn {row}(");
    let mut from = 0;
    while let Some(i) = code[from..].find(&needle) {
        let at = from + i;
        from = at + needle.len();
        if at > 0 && !code.as_bytes()[at - 1].is_ascii_whitespace() {
            continue;
        }
        // Walk back over the attribute block immediately above the fn.
        let mut head = code[..at].trim_end();
        while head.ends_with(']') {
            let Some(open) = head.rfind("#[") else { break };
            if head[open..] == *"#[test]" {
                return true;
            }
            head = head[..open].trim_end();
        }
    }
    false
}

/// Whether `code` declares `fn <row>(` at all — a module path may name a
/// helper (`review_blend6_r1_probes::seeds`), which is a citation of a
/// FUNCTION and resolves as one.
fn declares_fn(code: &str, row: &str) -> bool {
    let needle = format!("fn {row}(");
    let mut from = 0;
    while let Some(i) = code[from..].find(&needle) {
        let at = from + i;
        from = at + needle.len();
        if at == 0 || code.as_bytes()[at - 1].is_ascii_whitespace() {
            return true;
        }
    }
    false
}

fn resolve(tests: &Path, cites: &[Citation], must_be_test: bool) -> Vec<String> {
    let mut misses = Vec::new();
    for c in cites {
        let suite = tests.join(format!("{}.rs", c.suite));
        let Ok(text) = std::fs::read_to_string(&suite) else {
            misses.push(format!("{}: no suite file tests/{}.rs", c.at, c.suite));
            continue;
        };
        let code = code_only(&text);
        let ok = if must_be_test {
            declares_test_row(&code, &c.row)
        } else {
            declares_fn(&code, &c.row)
        };
        if !ok {
            misses.push(format!(
                "{}: tests/{}.rs declares no {}`fn {}`",
                c.at,
                c.suite,
                if must_be_test { "`#[test]` " } else { "" },
                c.row
            ));
        }
    }
    misses
}

/// **Every path-spelled citation resolves.** The corpus is the one
/// grep PR 2155 hands to the gate; a rename or a deletion of a cited
/// row is what goes red.
#[test]
fn every_path_spelled_test_citation_in_the_sweep_docs_resolves_to_a_test_row() {
    let tests = crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let cites = path_citations();
    assert!(
        cites.len() >= 8,
        "the corpus is not empty: unit 5 left one citation per touched file at least, \
         found {}",
        cites.len()
    );
    let misses = resolve(&tests, &cites, true);
    assert!(
        misses.is_empty(),
        "{} of {} path-spelled citations do not resolve:\n{}",
        misses.len(),
        cites.len(),
        misses.join("\n")
    );
}

/// **Every module-spelled citation resolves too.** This spelling is
/// the `cargo test` filter, and it is what unit 5's sweeps did not
/// see: the corpus here is the count of the OTHER spelling still live.
#[test]
fn every_module_spelled_test_citation_in_the_sweep_docs_resolves_to_a_test_row() {
    let tests = crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let cites = module_citations(&tests);
    let misses = resolve(&tests, &cites, false);
    assert!(
        misses.is_empty(),
        "{} of {} module-spelled citations do not resolve:\n{}",
        misses.len(),
        cites.len(),
        misses.join("\n")
    );
}
