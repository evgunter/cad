//! Reading Rust source as TEXT — the shared lexer, and the three views
//! of a file it supports, for the guards that pin a claim about the
//! code against the code itself.
//!
//! # Why this is here rather than in each guard
//!
//! A guard that greps its own crate's sources needs an answer to *is
//! this text code, prose, or a literal*, and the tree's answer used to
//! be one hand-rolled reader per guard — five of them, no two lexing
//! the same language, most knowing only a line-leading `//`. The
//! direction that costs is the silent one: a real site commented out
//! leaves its text in the file, so a count does not move and the guard
//! stays green over exactly the change it exists to catch.
//!
//! This crate is where the answer lives because it is a
//! zero-dependency leaf every crate in the tree already takes as a
//! dev-dependency, so a guard anywhere can share one lexer instead of
//! minting the next copy of it.
//!
//! # One lexer, three views, and why the count is three
//!
//! [`keeping`] is the only thing here that knows Rust's lexical
//! grammar. Everything else is a SELECTION over its output, which is
//! the structural rule: a guard whose needle is a shape the three
//! named views do not cover asks [`keeping`] for the region set it
//! wants, and does not write a second lexer to get it.
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
//!
//! # What it does not model
//!
//! It is a lexer, not a parser. An identifier assembled by a macro
//! (`concat_idents!`, `paste!`) is invisible to any textual walk, a
//! `pub fn` inside a `macro_rules!` body is text like any other, and
//! an `include!`d file is not seen at all. Nested block comments,
//! every string prefix (`b`, `c`, `r`, `br`, `cr`) and the
//! lifetime-versus-char-literal distinction ARE modelled, each with a
//! row in [`self::tests`] that reds if it stops being.

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
/// **This is the tree's only Rust lexer for source-text guards, and
/// the region set is the knob.** The three named views below are
/// selections over it; a needle wanting a fourth combination passes
/// the combination, and does not fork the lexer to get it.
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
#[must_use]
pub fn code_and_literals(text: &str) -> String {
    keeping(text, &[Region::Code, Region::Literal])
}

/// A file's PROSE alone, with code and literals blanked — the inverse
/// view, for a guard whose subject is a doc comment.
///
/// The comment markers themselves survive, so a caller that means
/// `///` rather than `//` can still say so with a line test.
#[must_use]
pub fn comments_only(text: &str) -> String {
    keeping(text, &[Region::Comment])
}

/// Copy `bytes` through, or blank them keeping newlines.
fn emit(out: &mut Vec<u8>, bytes: &[u8], kept: bool) {
    out.extend(bytes.iter().map(|&c| {
        if kept || c == b'\n' {
            c
        } else {
            b' '
        }
    }));
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
            let end = b[i..].iter().position(|&c| c == b'\n').map_or(b.len(), |r| i + r);
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
/// why every prefix has a row in [`self::tests`].
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
        if b[k] == b'"' && b[k + 1..].iter().take(hashes).filter(|c| **c == b'#').count() == hashes {
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    // These rows touch no process-global state and are safe to run in
    // parallel with anything. `vacuity`'s do not — its `caught` helper
    // swaps the process-wide panic hook from parallel test threads and
    // flakes about one run in thirteen (15/200; **issue #882**). Do not
    // copy that shape here.
    use super::{Region, code_and_literals, code_only, comments_only, keeping};

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
    const CODE_READS: [&str; 5] = [
        "gap * lever < eps",
        "let eps = tol.eps();",
        "f(a, b, eps)",
        "struct T<'a> { eps: &'a f64 }",
        "let c = 'e'; let d = eps;",
    ];

    /// S13's ratified shape for a text-matching guard: **a clean
    /// fixture must pass and every planted violation must fire.** Here
    /// the "violation" is a needle that survives blanking when it
    /// should not, or is lost when it should not be.
    #[test]
    fn every_hiding_place_is_blanked_and_no_code_read_is_lost() {
        for row in HIDING_PLACES {
            assert!(row.contains("eps"), "a row cannot hide what it lacks: {row}");
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
        assert!(!code_only(live).contains("e4_dual_door"), "code view blanks it");
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
        for view in [code_only(multi), comments_only(multi), code_and_literals(multi)] {
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
        assert!(code_only("let ab = ar\"x\";").contains("ar"), "not a raw string");
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
}
