//! Reading a crate's own source as TEXT — the shared answer to *"is
//! this text code?"* for the guards that pin a claim about the code
//! against the code itself.
//!
//! # Why this is here rather than in each guard
//!
//! `docs/SMELL-SCAN-2026-08.md`'s **S117** counts twelve source-text
//! guards in this tree behind five hand-rolled readers, no two of
//! which lex the same language, and names the way out: *a
//! test-support crate*. This is that crate — a zero-dependency leaf
//! every crate in the tree already takes as a dev-dependency, so a
//! guard in any of them can share one predicate instead of minting the
//! next copy of it.
//!
//! [`code_only`] is the predicate, ported unchanged in semantics from
//! `topo`'s `fixtures::code_only` (the walk #834 established) so that
//! collapsing that copy onto this one is a deletion rather than a
//! redesign. **`topo`'s copy is still live**; retiring it is S117's
//! row, not this module's.

// This module PANICS and `expect`s, deliberately, for `fuzz`'s reason
// one file over: a guard whose source walk cannot read the tree, or
// finds nothing in it, must go RED rather than quietly assert over an
// empty set — that vacuity is the failure this crate exists to
// prevent. The workspace's no-panic rule is about production code and
// nothing on a shipped build path can reach here: no production
// manifest names this crate at all (see the crate docs).
#![allow(clippy::panic, clippy::expect_used)]

/// `text` with every **comment** and every **literal body** blanked.
///
/// Removed: `//` line comments (anywhere on the line, not only at its
/// start), `/* … */` blocks (nested), and the CONTENTS of string, byte
/// and char literals — nothing that a read of an identifier can hide
/// inside, so blanking them can only remove false positives, never
/// create a false negative. Byte offsets and line structure are
/// preserved (newlines survive; removed bytes become spaces), so a
/// caller may still count lines or report a line number.
///
/// A `'` is treated as a char literal only when it closes within one
/// scalar (or one escape); otherwise it is a LIFETIME and stays code.
/// Mis-reading `'a` as an opening quote would swallow the rest of the
/// file.
///
/// **What it does not model**, because a guard that silently
/// mis-parses is worse than one that says so: **raw strings** in any
/// of their four spellings (`r"…"`, `r#"…"#`, `br…`, `cr…`) are
/// treated as an ordinary string opened at the quote, which is correct
/// unless the body contains a backslash-quote pair; and an identifier
/// assembled by a macro (`concat_idents!`, `paste!`) is invisible to
/// any textual walk. A caller whose tree may contain either owes its
/// own check that it does not — [`mentions_raw_string`] is that check,
/// and `mesh`'s ε inventory runs it rather than assuming.
///
/// **This is the WEAKER of `topo`'s two blankers, deliberately.**
/// `topo::source_walk::CodeOnly` does model raw strings; this is
/// `fixtures::code_only`, ported so that collapsing *that* copy onto
/// this one is a deletion. Collapsing `CodeOnly` onto it would be a
/// **downgrade** — S117's row says so, and says that closing the gap
/// here (porting `raw_string_open`) is the prerequisite, not an
/// optional extra.
#[must_use]
pub fn code_only(text: &str) -> String {
    let b = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    // Blank a byte range, keeping newlines so line numbers survive.
    let blank = |out: &mut Vec<u8>, b: &[u8], from: usize, to: usize| {
        for &c in &b[from..to.min(b.len())] {
            out.push(if c == b'\n' { b'\n' } else { b' ' });
        }
    };
    let mut i = 0usize;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'/') => {
                let start = i;
                while i < b.len() && b[i] != b'\n' {
                    i += 1;
                }
                blank(&mut out, b, start, i);
            }
            b'/' if b.get(i + 1) == Some(&b'*') => {
                let (start, mut depth) = (i, 1usize);
                i += 2;
                while i + 1 < b.len() && depth > 0 {
                    if b[i] == b'/' && b[i + 1] == b'*' {
                        depth += 1;
                        i += 2;
                    } else if b[i] == b'*' && b[i + 1] == b'/' {
                        depth -= 1;
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                if depth > 0 {
                    i = b.len();
                }
                blank(&mut out, b, start, i);
            }
            b'"' => {
                let start = i;
                i += 1;
                while i < b.len() && b[i] != b'"' {
                    i += usize::from(b[i] == b'\\') + 1;
                }
                i = (i + 1).min(b.len());
                blank(&mut out, b, start, i);
            }
            b'\'' => {
                let close = if b.get(i + 1) == Some(&b'\\') {
                    (i + 2..b.len().min(i + 8)).find(|&k| b[k] == b'\'')
                } else {
                    let mut k = i + 2;
                    // One UTF-8 scalar: 1 byte plus its continuations.
                    while k < b.len() && (b[k] & 0b1100_0000) == 0b1000_0000 {
                        k += 1;
                    }
                    (b.get(k) == Some(&b'\'')).then_some(k)
                };
                match close {
                    Some(k) => {
                        blank(&mut out, b, i, k + 1);
                        i = k + 1;
                    }
                    None => {
                        out.push(b'\'');
                        i += 1;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8(out).unwrap_or_default()
}

/// Whether `text` contains a **raw string** opener outside a comment.
///
/// The one construct [`code_only`] does not model, exposed so a caller
/// can assert its own tree is free of it rather than inheriting a
/// silent mis-parse. Deliberately conservative: it works on the
/// ORIGINAL text, so a raw string mentioned inside a comment counts as
/// a hit. A caller that trips this is being told to check, not that it
/// is wrong.
///
/// **All four prefixes**: `r"`, `r#`, and the BYTE and C-string forms
/// `br"`/`br#`, `cr"`/`cr#`. The byte prefix is not decoration — the
/// exact spelling `br"x\"` is the one this tree has now got wrong
/// three times (`D61`: #788 fixed it in `topo::source_walk::CodeOnly`,
/// G-g re-introduced it in `topo::fixtures::code_only`, and the first
/// version of THIS function suppressed detection whenever the opener
/// carried a `b`/`c` prefix, because it rejected any alphanumeric
/// predecessor). It cannot bite a tree with no `br"` in it, which is
/// exactly why it is fixed before there is a caller who trusts it.
#[must_use]
pub fn mentions_raw_string(text: &str) -> bool {
    let b = text.as_bytes();
    (0..b.len()).any(|i| {
        // `r`, optionally preceded by one `b` or `c`, then `"` or `#`,
        // with the whole prefix starting a token.
        if b[i] != b'r' || !matches!(b.get(i + 1), Some(&b'"') | Some(&b'#')) {
            return false;
        }
        let start = match i.checked_sub(1) {
            Some(p) if matches!(b[p], b'b' | b'c') => p,
            _ => i,
        };
        !start
            .checked_sub(1)
            .is_some_and(|p| b[p].is_ascii_alphanumeric() || b[p] == b'_')
    })
}

/// Every `.rs` file under `dir`, **recursively**, sorted.
///
/// Here rather than at each caller because a flat `read_dir` is the
/// other half of the copied walk: sharing the *predicate*
/// ([`code_only`]) and re-forking the *traversal* leaves each guard
/// free to miss a subdirectory, silently and in the green direction.
/// `topo`'s `source_walk::crate_sources` is the same walk and is
/// `pub(crate)` — the identical obstacle [`code_only`] is here to
/// remove.
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
        "the walk of {} found no .rs file — every guard built on it would pass by          finding nothing",
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
    // flakes about one run in seven (**issue #882**). Do not copy that
    // shape here.
    use super::{code_only, mentions_raw_string};

    /// S13's ratified shape for a text-matching guard: **a clean
    /// fixture must pass and every planted violation must fire.** Here
    /// the "violation" is a needle that survives blanking when it
    /// should not, or is lost when it should not be.
    #[test]
    fn every_hiding_place_is_blanked_and_no_code_read_is_lost() {
        let needle = "eps";
        // Each row hides the needle somewhere a naive line test misses.
        for row in [
            "let x = 1; // eps",
            "/// eps",
            "//! eps",
            "/* eps */",
            "/* outer /* eps */ inner */",
            "let s = \"eps\";",
            "let s = \"he said \\\"eps\\\"\";",
            "#[doc = \"eps\"]",
            "panic!(\"over eps {eps_x}\");",
        ] {
            assert_eq!(
                code_only(row).matches(needle).count(),
                0,
                "survived blanking: {row}"
            );
        }
        // And each row is a genuine code read that must SURVIVE.
        for row in [
            "gap * lever < eps",
            "let eps = Tolerance::get().eps;",
            "f(a, b, eps)",
            "struct T<'a> { eps: &'a f64 }",
            "let c = 'e'; let d = eps;",
        ] {
            assert_eq!(
                code_only(row).matches(needle).count(),
                row.matches(needle).count(),
                "lost a code read: {row}"
            );
        }
    }

    #[test]
    fn line_structure_survives_so_a_caller_can_report_a_line_number() {
        let multi = "a\n// eps\nb\n";
        assert_eq!(code_only(multi).lines().count(), multi.lines().count());
        assert_eq!(code_only(multi).lines().nth(1).unwrap().trim(), "");
    }

    #[test]
    fn a_lifetime_is_not_an_opening_quote() {
        // If `'a` opened a literal, everything after it would blank and
        // the trailing read would be lost.
        let row = "fn f<'a>(x: &'a f64, eps: f64) -> bool { x < &eps }";
        assert_eq!(code_only(row).matches("eps").count(), 2, "{row}");
    }

    /// **All four prefixes, and the byte one is the point.** `br\"x\\\"`
    /// is the exact spelling D61 records this tree getting wrong twice
    /// before; the first version of `mentions_raw_string` made it three
    /// by rejecting any alphanumeric predecessor, which the `b`/`c`
    /// prefix satisfies.
    #[test]
    fn the_unmodelled_construct_is_detectable_under_every_prefix() {
        for row in [
            "let s = r\"eps\";",
            "let s = r#\"eps\"#;",
            "let s = br\"eps\";",
            "let s = br#\"eps\"#;",
            "let s = cr\"eps\";",
            "let s = cr#\"eps\"#;",
        ] {
            assert!(mentions_raw_string(row), "missed an opener: {row}");
        }
        for row in [
            "for (name, r) in v",
            "let r = d / (bound + eps);",
            "check(\"eps\")",
            "let bar = b\"eps\";",
            // An identifier ENDING in b/c/r before a quote is not an
            // opener: the prefix must start a token.
            "let ab = ar\"x\";",
        ] {
            assert!(!mentions_raw_string(row), "false opener: {row}");
        }
    }

    /// The blanking rows above assert that the needle does not survive.
    /// **They do not assert that it was there to begin with**, so an
    /// edit that dropped `eps` from a row would make that row silently
    /// vacuous — the shape [`crate::vacuity`] exists to forbid. This
    /// row is the floor: every hide-row must actually contain the
    /// needle before blanking.
    #[test]
    fn no_hide_row_is_vacuous() {
        let rows = [
            "let x = 1; // eps",
            "/// eps",
            "//! eps",
            "/* eps */",
            "/* outer /* eps */ inner */",
            "let s = \"eps\";",
            "let s = \"he said \\\"eps\\\"\";",
            "#[doc = \"eps\"]",
            "panic!(\"over eps {eps_x}\");",
        ];
        for row in rows {
            assert!(row.contains("eps"), "row cannot hide what it lacks: {row}");
        }
        assert_eq!(rows.len(), 9, "a hide-row was added or removed");
    }

    /// The orchestrator's demonstration, kept as a row: with the `b`
    /// prefix suppressing detection, `code_only` swallowed to end of
    /// line and LOST the `eps` read while the guard said the tree was
    /// clean. Both halves are asserted — the blanker still loses it
    /// (it does not model raw strings, by design) and the guard now
    /// says so.
    #[test]
    fn the_br_spelling_is_caught_before_the_blanker_loses_a_read() {
        let row = "let s = br\"a\\\"; let y = eps;";
        assert!(row.contains("eps"), "fixture must carry the read");
        assert_eq!(
            code_only(row).matches("eps").count(),
            0,
            "blanker still loses it"
        );
        assert!(mentions_raw_string(row), "and the guard must say so");
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
