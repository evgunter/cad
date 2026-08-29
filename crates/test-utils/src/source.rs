//! Reading Rust source as TEXT — the shared lexer, and the three views
//! of a file it supports, for the guards that pin a claim about the
//! code against the code itself.
//!
//! # Why this is here rather than in each guard
//!
//! A guard that greps its own crate's sources needs an answer to *is
//! this text code, prose, or a literal*, and the answer each guard
//! reached for was its own — no two lexing the same language, most
//! knowing only a line-leading `//`. The direction that costs is the
//! silent one: a real site commented out leaves its text in the file,
//! so a count does not move and the guard stays green over exactly the
//! change it exists to catch.
//!
//! **This is the shared answer, not the only one in the tree.**
//! `crates/test-utils/tests/reader_census.rs` enumerates every site
//! that reads Rust source as text and says which reader each uses; the
//! readers still outside this module are named there with the track
//! that owns each, and the shell gates under `scripts/gates/` have a
//! second home of their own in awk. **Read the ledger for the count —
//! this paragraph deliberately carries none**, because a number here
//! is a copy that goes stale in the direction that matters.
//!
//! This crate is where the shared answer lives because it is a
//! zero-dependency leaf that can sit below everything. Most crates
//! already dev-depend on it; `pncad`, `pncad-py` and `quantity` do
//! not, and that is not a detail — `pncad/tests/all.rs` holds the
//! class's largest unconverted reader, and adding the dev-dependency
//! is the first step of converting it.
//!
//! # One lexer, three views, and why the count is three
//!
//! [`keeping`] is the only thing in this module that knows Rust's
//! lexical grammar. Everything else is a SELECTION over its output, or
//! an operation that reads its output as balanced text, which is the
//! structural rule: a guard whose needle is a shape the three named
//! views do not cover asks [`keeping`] for the region set it wants,
//! and does not write a second lexer to get it.
//!
//! The three named views are the three a needle can want:
//!
//! - [`code_only`] — the needle is a **code fragment** (a call, an
//!   operator, an item head). Comments and literals both blanked:
//!   nothing an identifier read can hide inside survives, so blanking
//!   removes false positives and can lose no real site.
//! - [`code_and_literals`] — the needle **contains a string literal**
//!   (`#[path = "x.rs"]`, `decide("split_arc_window"`). Blanking the
//!   literal would make such a guard vacuous; blanking the comment is
//!   still the whole point.
//! - [`comments_only`] — the needle is **prose** (a doc-comment
//!   heading that is itself the ledger being pinned). The inverse
//!   view: code and literals blanked, so a heading spelled in a
//!   `format!` cannot satisfy a guard that means to read the docs.
//!   **A `#[doc = "…"]` ATTRIBUTE is not in this view**: it is a
//!   literal to the lexer, as it is to `rustc`, so a ledger written
//!   in attributes rather than in `///` reads as absent. That is the
//!   fail-red direction, and it is stated because it is the one place
//!   *"the docs"* and *"the comments"* are not the same set.
//!
//! # Three operations over a blanked view
//!
//! A blanked view has a property raw source does not: **every bracket
//! in it is a real bracket**, because one inside a literal or a comment
//! is a space. Three operations depend on exactly that precondition,
//! which is why they live beside the lexer rather than at each call
//! site — [`balanced_end`], [`top_level_split`] and the traversals
//! [`rust_sources`] and [`suite_files`].
//!
//! # What it does not model
//!
//! It is a lexer, not a parser. An identifier assembled by a macro
//! (`concat_idents!`, `paste!`) is invisible to any textual walk, a
//! `pub fn` inside a `macro_rules!` body is text like any other, and
//! an `include!`d file is not seen at all. Nested block comments,
//! every string prefix (`b`, `c`, `r`, `br`, `cr`) and the
//! lifetime-versus-char-literal distinction ARE modelled, each with a
//! row in this module's tests that reds if it stops being.

// This module PANICS and `expect`s, deliberately, for `fuzz`'s reason
// one file over: a guard whose source walk cannot read the tree, or
// finds nothing in it, must go RED rather than quietly assert over an
// empty set — that vacuity is the failure this crate exists to
// prevent. The workspace's no-panic rule is about production code and
// nothing on a shipped build path can reach here: no production
// manifest names this crate at all (see the crate docs).
#![allow(clippy::panic, clippy::expect_used)]

/// The three regions every byte of a Rust file lies in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Region {
    /// Everything that is neither a comment nor a literal.
    Code,
    /// `//` line comments and nested `/* … */` blocks, doc comments
    /// (`///`, `//!`, `/** */`) included — they are comments to the
    /// lexer and prose to a reader, which is what [`comments_only`]
    /// exists to search.
    Comment,
    /// String, byte-string, C-string, raw-string and char literals,
    /// **prefix and delimiters included**. A lifetime (`&'a str`) is
    /// code: a `'` opens a literal only when it closes within one
    /// escape or one scalar.
    Literal,
}

/// `text` with every byte OUTSIDE `keep` blanked to a space.
///
/// Newlines survive and every other removed byte becomes one space, so
/// byte offsets, line numbers and line structure are all preserved and
/// a caller may report a position into the ORIGINAL text from a match
/// in this one. A multi-byte character is blanked byte for byte, so it
/// is never half-erased.
///
/// **This is the only Rust lexer in this module, and the region set is
/// the knob.** The three named views below are selections over it; a
/// needle wanting a fourth combination passes the combination, and
/// does not fork the lexer to get it. Other Rust lexers do exist in
/// the tree — `crates/test-utils/tests/reader_census.rs` names each
/// with the track that owes its conversion.
#[must_use]
pub fn keeping(text: &str, keep: &[Region]) -> String {
    let b = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        let (end, region) = span(b, i);
        emit(&mut out, &b[i..end], keep.contains(&region));
        i = end;
    }
    String::from_utf8(out).expect("blanking never splits a character")
}

/// Rust source with every comment and every literal blanked — the view
/// for a needle that is a **code fragment**.
#[must_use]
pub fn code_only(text: &str) -> String {
    keeping(text, &[Region::Code])
}

/// Rust source with every comment blanked and literals KEPT — the view
/// for a needle that **contains a string literal**, which
/// [`code_only`] would erase and leave the guard vacuous.
///
/// The tree's largest caller is the `every_suite_file_is_aggregated`
/// row each crate's `tests/all.rs` carries, whose needle is the mount
/// `#[path = "<suite>.rs"]`. **The argument is stated here so it is
/// stated once**: a mount that has been commented out must not answer
/// for the file it names, because `autotests = false` then drops a
/// whole suite from the build with the guard still green — the silent
/// direction, at its largest.
#[must_use]
pub fn code_and_literals(text: &str) -> String {
    keeping(text, &[Region::Code, Region::Literal])
}

/// A file's PROSE alone, with code and literals blanked — the inverse
/// view, for a guard whose subject is a doc comment.
///
/// The comment markers themselves survive, so a caller that means
/// `///` rather than `//` can still say so with a line test.
///
/// **`#[doc = "…"]` is NOT here.** A doc attribute is a string
/// literal, and this view blanks literals; a guard reading a ledger
/// that is written in attributes finds nothing and goes red, which is
/// the safe direction but is a surprise worth having in writing.
#[must_use]
pub fn comments_only(text: &str) -> String {
    keeping(text, &[Region::Comment])
}

/// Copy `bytes` through, or blank them keeping newlines.
fn emit(out: &mut Vec<u8>, bytes: &[u8], kept: bool) {
    out.extend(
        bytes
            .iter()
            .map(|&c| if kept || c == b'\n' { c } else { b' ' }),
    );
}

/// The end offset and region of the span starting at `i`.
///
/// Code is returned one byte at a time; every comment and literal is
/// returned whole, which is what makes a needle unable to straddle the
/// boundary.
fn span(b: &[u8], i: usize) -> (usize, Region) {
    if let Some(n) = raw_string_len(b, i) {
        return (i + n, Region::Literal);
    }
    match b[i] {
        b'/' if b.get(i + 1) == Some(&b'/') => {
            let end = b[i..]
                .iter()
                .position(|&c| c == b'\n')
                .map_or(b.len(), |r| i + r);
            (end, Region::Comment)
        }
        b'/' if b.get(i + 1) == Some(&b'*') => {
            // Rust's block comments NEST: the first `*/` does not
            // necessarily close the one that opened here.
            let (mut depth, mut j) = (1usize, i + 2);
            while j < b.len() && depth > 0 {
                if b[j] == b'/' && b.get(j + 1) == Some(&b'*') {
                    depth += 1;
                    j += 2;
                } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            (j.min(b.len()), Region::Comment)
        }
        b'"' => (i + quoted_len(b, i), Region::Literal),
        b'b' | b'c' if b.get(i + 1) == Some(&b'"') && token_start(b, i) => {
            (i + 1 + quoted_len(b, i + 1), Region::Literal)
        }
        // `b'x'` — the byte-CHAR literal. Only `b` takes this prefix;
        // there is no `c'…'` in Rust. Written out because the string
        // arm above does not cover it, and a prefix left behind as
        // code is [`Region::Literal`]'s contract broken rather than
        // its partition: the stray `b` is a token no guard means.
        b'b' if b.get(i + 1) == Some(&b'\'') && token_start(b, i) => {
            match char_literal_len(b, i + 1) {
                Some(n) => (i + 1 + n, Region::Literal),
                None => (i + 1, Region::Code),
            }
        }
        b'\'' => match char_literal_len(b, i) {
            Some(n) => (i + n, Region::Literal),
            // A lifetime. One byte of code, so the scan resumes on the
            // name rather than swallowing the rest of the file.
            None => (i + 1, Region::Code),
        },
        _ => (i + 1, Region::Code),
    }
}

/// Whether the token starting at `i` starts a token — nothing
/// identifier-shaped immediately before it.
fn token_start(b: &[u8], i: usize) -> bool {
    !i.checked_sub(1)
        .is_some_and(|p| b[p].is_ascii_alphanumeric() || b[p] == b'_')
}

/// The byte length of the `"…"` literal opening at `i`, escapes
/// honoured. An unterminated literal runs to end of input.
fn quoted_len(b: &[u8], i: usize) -> usize {
    let mut j = i + 1;
    while j < b.len() && b[j] != b'"' {
        j += usize::from(b[j] == b'\\') + 1;
    }
    (j + 1).min(b.len()) - i
}

/// The byte length of the RAW string literal opening at `i` — `r"…"`,
/// `r#"…"#`, `br"…"`, `cr#"…"#` and so on — or `None`.
///
/// Raw strings are lexed apart from the plain arm rather than falling
/// through to it because that arm honours `\` escapes, which a raw
/// string does not have: `br"x\"` read as an escaped quote is an
/// unclosed string that blanks the rest of the file. That exact
/// spelling is the one this tree has got wrong three times, which is
/// why every prefix has a row in this module's tests.
fn raw_string_len(b: &[u8], i: usize) -> Option<usize> {
    if !token_start(b, i) {
        return None;
    }
    let mut j = i;
    if matches!(b.get(j), Some(b'b' | b'c')) {
        j += 1;
    }
    if b.get(j) != Some(&b'r') {
        return None;
    }
    j += 1;
    let mut hashes = 0usize;
    while b.get(j + hashes) == Some(&b'#') {
        hashes += 1;
    }
    if b.get(j + hashes) != Some(&b'"') {
        return None;
    }
    let mut k = j + hashes + 1;
    loop {
        if k >= b.len() {
            return Some(b.len() - i);
        }
        if b[k] == b'"'
            && b[k + 1..]
                .iter()
                .take(hashes)
                .filter(|c| **c == b'#')
                .count()
                == hashes
        {
            return Some((k + 1 + hashes).min(b.len()) - i);
        }
        k += 1;
    }
}

/// The byte length of the char literal at `i`, or `None` when the
/// quote opens a LIFETIME instead. Mis-reading `'a` as an opening
/// quote would swallow the rest of the file.
fn char_literal_len(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i + 1) == Some(&b'\\') {
        // `'\n'`, `'\''`, `'\\'`, `'\u{1F600}'`. The scan starts AT the
        // backslash, so the escape skip applies to it — starting one
        // byte later reads `'\''` as ending early and `'\\'` as
        // unterminated.
        let mut j = i + 1;
        while j < b.len() && b[j] != b'\'' {
            j += usize::from(b[j] == b'\\') + 1;
        }
        return (j < b.len()).then_some(j + 1 - i);
    }
    // One character, then a closing quote. A multi-byte char is one
    // char and several bytes, so step by the character's own width.
    let rest = std::str::from_utf8(b.get(i + 1..)?).ok()?;
    let c = rest.chars().next()?;
    let w = c.len_utf8();
    (b.get(i + 1 + w) == Some(&b'\'')).then_some(w + 2)
}

/// Every `.rs` file under `dir`, **recursively**, sorted.
///
/// Here rather than at each caller because a flat `read_dir` is the
/// other half of the copied walk: sharing the *predicate*
/// ([`keeping`]) and re-forking the *traversal* leaves each guard free
/// to miss a subdirectory, silently and in the green direction.
///
/// **Panics** if the walk finds nothing: a guard built on an empty
/// traversal passes by finding no sites, which is the vacuity this
/// crate exists to forbid. A caller wanting a stronger floor (a
/// minimum count, a required file) should assert it on the result.
#[must_use]
pub fn rust_sources(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let entries = std::fs::read_dir(dir)
            .unwrap_or_else(|e| panic!("a readable source directory {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                collect(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    collect(dir, &mut out);
    assert!(
        !out.is_empty(),
        "the walk of {} found no .rs file — every guard built on it would pass by \
         finding nothing",
        dir.display()
    );
    out.sort();
    out
}

/// The byte offset of the delimiter closing the bracket that opens at
/// `open`, or `None` if it never closes.
///
/// **`blanked` must be a view from [`keeping`] that drops literals and
/// comments** — that is the precondition the whole operation rests on:
/// in such a view every bracket is a real bracket, so depth counting
/// IS the parse and no literal tracker is needed. Run over raw source
/// it is wrong, silently.
///
/// `(`, `[` and `{` all count, because a call's argument list can
/// contain either of the others and a guard that carves one wants the
/// region, not the kind.
#[must_use]
pub fn balanced_end(blanked: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (off, c) in blanked[open..].char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => {
                // `checked_sub`, so an `open` that is not an opener
                // answers `None` rather than underflowing — a panic in
                // debug and a wrong answer in release is the one
                // outcome a shared helper must not have. Where
                // [`top_level_split`] clamps instead, the difference is
                // deliberate and stated there.
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(open + off);
                }
            }
            _ => {}
        }
    }
    None
}

/// The byte ranges of the `sep`-separated items of `blanked` at bracket
/// depth zero.
///
/// Same precondition as [`balanced_end`], and the same reason it is
/// here rather than at its call site: over a blanked view a comma
/// inside a string literal is a space, so splitting an argument list
/// needs no literal tracking — and a copy of this loop at the call
/// site is a copy of a lexer's postcondition, which is how the tree
/// got its readers in the first place.
///
/// `<` and `>` count as brackets: an argument list is the caller, and
/// a turbofish or a generic argument must not split one.
///
/// **Depth CLAMPS at zero here, where [`balanced_end`] refuses.** The
/// input is a fragment carved out of a larger expression, so a closer
/// with no opener inside it is ordinary — `>` in `a -> b` is one —
/// and the operation is "split at depth zero", which a clamp answers
/// and an underflow does not.
#[must_use]
pub fn top_level_split(blanked: &str, sep: char) -> Vec<std::ops::Range<usize>> {
    let mut depth = 0usize;
    let mut out = Vec::new();
    let mut start = 0usize;
    for (off, c) in blanked.char_indices() {
        match c {
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth = depth.saturating_sub(1),
            c if c == sep && depth == 0 => {
                out.push(start..off);
                start = off + c.len_utf8();
            }
            _ => {}
        }
    }
    out.push(start..blanked.len());
    out
}

/// The directory of the crate whose `CARGO_MANIFEST_DIR` is `baked`.
///
/// **Both ways a suite runs.** A plain `cargo test` resolves the path
/// baked in at compile time; a nextest ARCHIVE replayed on another
/// runner has no such directory, and `--workspace-remap` has instead
/// pointed the per-test cwd at the crate root. Every guard that opens
/// a file relative to its own crate needs both, and each one that
/// worked it out again wrote the same paragraph next to the same six
/// lines.
///
/// **Panics** when neither candidate holds a `Cargo.toml`: a guard
/// that silently resolved to the wrong root would read the wrong
/// tree.
#[must_use]
pub fn crate_dir(baked: &str) -> std::path::PathBuf {
    let baked = std::path::PathBuf::from(baked);
    if baked.join("Cargo.toml").is_file() {
        return baked;
    }
    let cwd = std::env::current_dir().expect("a working directory");
    assert!(
        cwd.join("Cargo.toml").is_file(),
        "neither {} nor {} is a crate root",
        baked.display(),
        cwd.display()
    );
    cwd
}

/// Every SUITE file under a crate's `tests/` directory, relative to it,
/// `/`-separated and sorted, with `all.rs` itself excluded.
///
/// Recursive, and shared for the reason [`rust_sources`] is: thirteen
/// crates carry a row asserting every suite is mounted in `tests/all.rs`,
/// and while they each walked `tests/` themselves twelve used a FLAT
/// `read_dir` and one recursed — so twelve of them could not see a
/// suite in a group directory at all, and nothing recorded that they
/// were the weaker variant.
///
/// **A suite and a shared HELPER are told apart by Rust's own module
/// rule, not by a list.** A subdirectory holding a `mod.rs` is a module
/// directory: its files are reached through `mod <name>;` inside
/// whatever suite declares it, so they are not test targets and owe no
/// `#[path]` line. A subdirectory without one is a group of suites,
/// each of which does. That is why `tests/common/mod.rs` is not
/// reported and `tests/curves/boxes.rs` is.
#[must_use]
pub fn suite_files(tests_dir: &std::path::Path) -> Vec<String> {
    rust_sources(tests_dir)
        .iter()
        .filter_map(|path| {
            let rel = path
                .strip_prefix(tests_dir)
                .expect("a walked file lies under tests/")
                .to_string_lossy()
                .replace('\\', "/");
            if rel == "all.rs" {
                return None;
            }
            // A module directory anywhere above it: its files are
            // reached through `mod <name>;`, never through `#[path]`.
            let mut dir = path.parent();
            while let Some(d) = dir {
                if d == tests_dir {
                    break;
                }
                if d.join("mod.rs").is_file() {
                    return None;
                }
                dir = d.parent();
            }
            Some(rel)
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    // These rows touch no process-global state and are safe to run in
    // parallel with anything. `vacuity`'s do not — its `caught` helper
    // swaps the process-wide panic hook from parallel test threads and
    // flakes about one run in thirteen (15/200; **issue #882**). Do not
    // copy that shape here.
    use super::{
        Region, balanced_end, code_and_literals, code_only, comments_only, keeping, top_level_split,
    };

    /// Every construct a needle can hide in, and the code read each one
    /// must not cost. Written once and asserted from three directions
    /// below, so a construct added here is exercised in all of them.
    const HIDING_PLACES: [&str; 12] = [
        "let x = 1; // eps",
        "/// eps",
        "//! eps",
        "/* eps */",
        "/* outer /* eps */ inner */",
        "let s = \"eps\";",
        "let s = \"he said \\\"eps\\\"\";",
        "#[doc = \"eps\"]",
        "panic!(\"over eps {eps_x}\");",
        "let s = r\"eps\";",
        "let s = br#\"eps\"#;",
        "let s = cr\"eps\";",
    ];

    /// Genuine code reads of `eps`, which no view of the code may lose.
    const CODE_READS: [&str; 6] = [
        "gap * lever < eps",
        "let eps = tol.eps();",
        "f(a, b, eps)",
        "struct T<'a> { eps: &'a f64 }",
        "let c = 'e'; let d = eps;",
        "let c = b'e'; let d = eps;",
    ];

    /// S13's ratified shape for a text-matching guard: **a clean
    /// fixture must pass and every planted violation must fire.** Here
    /// the "violation" is a needle that survives blanking when it
    /// should not, or is lost when it should not be.
    #[test]
    fn every_hiding_place_is_blanked_and_no_code_read_is_lost() {
        for row in HIDING_PLACES {
            assert!(
                row.contains("eps"),
                "a row cannot hide what it lacks: {row}"
            );
            assert_eq!(
                code_only(row).matches("eps").count(),
                0,
                "survived blanking: {row}"
            );
        }
        for row in CODE_READS {
            assert_eq!(
                code_only(row).matches("eps").count(),
                row.matches("eps").count(),
                "lost a code read: {row}"
            );
        }
    }

    /// **The needle that IS a string literal.** `code_and_literals`
    /// blanks the comment and keeps the literal, so a guard reading for
    /// `#[path = "…"]` sees the live mount and not the commented-out
    /// one. Under [`code_only`] the same guard would be vacuous, which
    /// is why the two views both exist.
    #[test]
    fn the_literal_view_keeps_the_needle_and_still_drops_the_comment() {
        let live = "#[path = \"e4_dual_door.rs\"]\nmod e4;";
        let dead = "// #[path = \"e4_dual_door.rs\"]\n";
        assert!(code_and_literals(live).contains("#[path = \"e4_dual_door.rs\"]"));
        assert!(!code_and_literals(dead).contains("#[path = \"e4_dual_door.rs\"]"));
        assert!(
            !code_only(live).contains("e4_dual_door"),
            "code view blanks it"
        );
        // And a literal is a literal wherever it sits: a needle inside
        // a doc comment is prose, not a site.
        assert!(!code_and_literals("/// #[path = \"x.rs\"]").contains("x.rs"));
    }

    /// **The inverse view.** A guard whose subject is a doc-comment
    /// ledger must not be satisfiable by code, and must still see the
    /// prose that `code_only` would erase.
    #[test]
    fn the_prose_view_reads_docs_and_refuses_to_be_satisfied_by_code() {
        assert!(comments_only("/// Version 7 is a break.").contains("Version 7 is"));
        assert!(!comments_only("f(format!(\"Version 7 is\"));").contains("Version 7 is"));
        assert!(!comments_only("let version_7_is = 1;").contains("version_7_is"));
    }

    /// **The three views partition the file**, byte for byte: every
    /// byte is code, comment or literal and never two of them, and
    /// keeping all three returns the input unchanged. This is what
    /// makes a fourth view a SELECTION rather than a second lexer —
    /// there is nothing outside the three to build one from.
    #[test]
    fn the_three_views_partition_every_byte_of_a_file() {
        let text = include_str!("source.rs");
        assert_eq!(
            keeping(text, &[Region::Code, Region::Comment, Region::Literal]),
            text,
            "keeping every region is the identity"
        );
        let views = [
            code_only(text),
            comments_only(text),
            keeping(text, &[Region::Literal]),
        ];
        for v in &views {
            assert_eq!(v.len(), text.len(), "byte offsets must survive");
        }
        for (i, &c) in text.as_bytes().iter().enumerate() {
            let kept = views.iter().filter(|v| v.as_bytes()[i] == c).count();
            if c == b' ' || c == b'\n' {
                continue; // Indistinguishable from its own blanking.
            }
            assert_eq!(kept, 1, "byte {i} ({:?}) is in {kept} regions", c as char);
        }
    }

    #[test]
    fn line_structure_survives_so_a_caller_can_report_a_line_number() {
        let multi = "a\n// eps\nb\n";
        for view in [
            code_only(multi),
            comments_only(multi),
            code_and_literals(multi),
        ] {
            assert_eq!(view.lines().count(), multi.lines().count());
        }
        assert_eq!(code_only(multi).lines().nth(1).unwrap().trim(), "");
        assert_eq!(comments_only(multi).lines().next().unwrap().trim(), "");
    }

    #[test]
    fn a_lifetime_is_not_an_opening_quote() {
        // If `'a` opened a literal, everything after it would blank and
        // the trailing read would be lost.
        let row = "fn f<'a>(x: &'a f64, eps: f64) -> bool { x < &eps }";
        assert_eq!(code_only(row).matches("eps").count(), 2, "{row}");
    }

    /// **All four raw-string prefixes, and the byte one is the point.**
    /// `br"x\"` is a CLOSED raw string whose body ends in a backslash;
    /// read through the escape-honouring plain-string arm it is an
    /// unclosed literal that blanks the rest of the file, losing every
    /// code read after it. That exact spelling is the one this tree got
    /// wrong three times before the lexer was shared.
    #[test]
    fn a_raw_string_closes_at_its_own_delimiter_and_loses_no_following_code() {
        let row = "let s = br\"a\\\"; let y = eps;";
        assert_eq!(
            code_only(row).matches("eps").count(),
            1,
            "the read after a `br\"x\\\"` must survive: {row}"
        );
        // Hashes, and a quote inside the body that does not close it.
        let hashed = "let s = r#\"a \"quoted\" b\"#; let y = eps;";
        assert_eq!(code_only(hashed).matches("eps").count(), 1);
        assert!(!code_only(hashed).contains("quoted"), "body is a literal");
        // An identifier ENDING in b/c/r before a quote is not a prefix.
        assert!(
            code_only("let ab = ar\"x\";").contains("ar"),
            "not a raw string"
        );
    }

    #[test]
    fn the_walk_is_recursive_and_refuses_to_be_empty() {
        let dir = std::env::temp_dir().join(format!("tu-src-{}", std::process::id()));
        let sub = dir.join("deep").join("deeper");
        std::fs::create_dir_all(&sub).expect("temp dirs");
        std::fs::write(dir.join("top.rs"), "// eps\n").expect("write");
        std::fs::write(sub.join("hidden.rs"), "let eps = 1;\n").expect("write");
        std::fs::write(dir.join("notrust.txt"), "eps").expect("write");
        let found = super::rust_sources(&dir);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().any(|p| p.ends_with("hidden.rs")), "{found:?}");
        std::fs::remove_dir_all(&dir).expect("cleanup");
    }

    /// **The precondition is the operation.** Over a blanked view every
    /// bracket is a real bracket, so depth counting is the whole parse
    /// — including when a literal holds an unbalanced one, which is the
    /// case a call-site copy gets wrong.
    #[test]
    fn the_balanced_region_is_carved_over_the_blanked_view() {
        let raw = "f(a, g(b), \"(((\", [c]) tail";
        let code = code_only(raw);
        let open = code.find('(').expect("an opener");
        let end = balanced_end(&code, open).expect("it closes");
        assert_eq!(&raw[open..=end], "(a, g(b), \"(((\", [c])", "{code}");
        // Raw text is the direction this exists to stop: the literal's
        // three unbalanced parens run the scan off the end.
        assert_eq!(balanced_end(raw, open), None, "raw source must not be used");
        assert_eq!(balanced_end(&code_only("f(a"), 1), None, "never closes");
    }

    /// Splitting an argument list: a comma inside a nested bracket, a
    /// generic argument or a blanked literal is not a separator.
    #[test]
    fn a_top_level_split_ignores_nested_and_generic_commas() {
        let raw = "name, Probe<A, B>, f(x, y), \"a,b\"";
        let code = code_only(raw);
        let parts: Vec<&str> = top_level_split(&code, ',')
            .into_iter()
            .map(|r| raw[r].trim())
            .collect();
        assert_eq!(
            parts,
            ["name", "Probe<A, B>", "f(x, y)", "\"a,b\""],
            "{code}"
        );
    }

    /// A suite in a group directory is a suite; a file under a `mod.rs`
    /// directory is a helper, reached through `mod <name>;`. Twelve of
    /// the thirteen mount guards could see neither.
    #[test]
    fn the_suite_walk_recurses_and_skips_module_directories() {
        let root = std::env::temp_dir().join(format!("tu-suites-{}", std::process::id()));
        let group = root.join("curves");
        let module = root.join("common");
        std::fs::create_dir_all(&group).expect("dirs");
        std::fs::create_dir_all(module.join("deep")).expect("dirs");
        for (path, text) in [
            (root.join("all.rs"), "// aggregator"),
            (root.join("flat.rs"), "// suite"),
            (group.join("boxes.rs"), "// suite in a group"),
            (module.join("mod.rs"), "// shared helper"),
            (module.join("deep").join("more.rs"), "// under the helper"),
        ] {
            std::fs::write(path, text).expect("write");
        }
        let mut found = super::suite_files(&root);
        found.sort();
        assert_eq!(found, ["curves/boxes.rs", "flat.rs"], "{found:?}");
        std::fs::remove_dir_all(&root).expect("cleanup");
    }

    #[test]
    fn a_crate_dir_is_the_one_holding_a_manifest() {
        let here = super::crate_dir(env!("CARGO_MANIFEST_DIR"));
        assert!(here.join("Cargo.toml").is_file(), "{here:?}");
        assert!(here.ends_with("test-utils"), "{here:?}");
    }

    /// **A literal is blanked PREFIX AND ALL**, which is
    /// [`Region::Literal`]'s stated contract and not merely its
    /// partition. `b'x'` is the construct that had no row: the
    /// partition holds either way — the `b` is *some* region — so
    /// neither the partition test nor 2,125 externally generated
    /// snippets could tell that the prefix was surviving as code.
    #[test]
    fn every_literal_prefix_is_blanked_with_its_literal() {
        for literal in [
            "b'x'",
            "b\"x\"",
            "c\"x\"",
            "br\"x\"",
            "cr#\"x\"#",
            "'x'",
            "b'\\''",
        ] {
            let row = format!("let z = {literal};");
            let want = format!("let z = {};", " ".repeat(literal.len()));
            assert_eq!(code_only(&row), want, "{row}");
        }
        // A lifetime after `b` is not a byte-char literal: it never
        // closes, so the `b` stays code and so does the read after it.
        let row = "fn f<'a>(b: &'a f64) -> f64 { *b }";
        assert_eq!(code_only(row).matches('b').count(), 2, "{row}");
    }

    /// A doc ATTRIBUTE is a literal, not a comment — `rustc` agrees,
    /// and a guard reading a ledger through [`comments_only`] must
    /// find nothing rather than half of it.
    #[test]
    fn a_doc_attribute_is_a_literal_and_not_prose() {
        let row = "#[doc = \"Version 7 is a break.\"]\nstruct S;";
        assert!(!comments_only(row).contains("Version 7 is"), "not prose");
        assert!(code_and_literals(row).contains("Version 7 is"), "a literal");
        assert!(!code_only(row).contains("Version 7 is"), "blanked as one");
        // The `///` spelling, which IS prose, for contrast.
        assert!(comments_only("/// Version 7 is a break.").contains("Version 7 is"));
    }
}
