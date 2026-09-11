//! BLEND unit 5 review probes (lane r5, PR 2155): a doc comment in
//! `crates/sweep/src/**` that names a test row as its evidence is a
//! citation, and these rows RESOLVE every one of them.
//!
//! **One spelling, `<module>::<row>`** — no `.rs`, no path. It is what
//! `cargo test -p sweep --test all -- <module>::<row>` filters on,
//! because `tests/all.rs` mounts every suite as a module of one
//! binary; a path spelling reads as a filter that selects nothing. A
//! rename or a deletion of a cited row goes red here. A row that still
//! exists and asserts less than the sentence says is the half no
//! resolver can see, and is not claimed.
//!
//! The corpus is `crates/sweep/src/**` citing `crates/sweep/tests/**`.
//! A citation to another crate's suite — `admit.rs`'s
//! `reader_census::…`, in `test-utils` — is outside it: this row can
//! only read its own crate's `tests/` directory.
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

/// Every citation resolved, one message per miss. A citation must name
/// a `#[test]`: a module path can also name a helper
/// (`review_blend6_r1_probes::seeds`), and a helper is not evidence.
fn resolve(tests: &Path, cites: &[Citation]) -> Vec<String> {
    let mut misses = Vec::new();
    for c in cites {
        let suite = tests.join(format!("{}.rs", c.suite));
        let Ok(text) = std::fs::read_to_string(&suite) else {
            misses.push(format!("{}: no suite file tests/{}.rs", c.at, c.suite));
            continue;
        };
        if !declares_test_row(&code_only(&text), &c.row) {
            misses.push(format!(
                "{}: tests/{}.rs declares no `#[test] fn {}`",
                c.at, c.suite, c.row
            ));
        }
    }
    misses
}

/// **Every test citation in the `sweep` docs resolves to a `#[test]`.**
///
/// A doc or code comment under `crates/sweep/src/**` that names a row
/// as its evidence writes it `<module>::<row>`, and this row walks all
/// of them: the suite file must exist under `crates/sweep/tests/` and
/// must declare `<row>` as a `#[test]`. A rename or a deletion of a
/// cited row goes red here, which is the half of the class that can be
/// mechanised; whether the row still ASSERTS what the sentence says is
/// the half that cannot, and is not claimed.
///
/// The floor keeps it non-vacuous: the corpus was 45 citations when
/// this row was written, so a reader that silently stopped matching
/// would fail here rather than pass over an empty list.
#[test]
fn every_test_citation_in_the_sweep_docs_resolves_to_a_test_row() {
    let tests = crate_dir(env!("CARGO_MANIFEST_DIR")).join("tests");
    let cites = module_citations(&tests);
    assert!(
        cites.len() >= 40,
        "the corpus is not empty and the reader still matches: found {} citations",
        cites.len()
    );
    let misses = resolve(&tests, &cites);
    assert!(
        misses.is_empty(),
        "{} of {} citations do not resolve:\n{}",
        misses.len(),
        cites.len(),
        misses.join("\n")
    );
}
