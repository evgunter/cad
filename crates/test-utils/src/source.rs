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
/// mis-parses is worse than one that says so: **raw strings**
/// (`r"…"`, `r#"…"#`) are treated as an ordinary string opened at the
/// quote, which is correct unless the body contains a
/// backslash-quote pair; and an identifier assembled by a macro
/// (`concat_idents!`, `paste!`) is invisible to any textual walk. A
/// caller whose tree may contain either owes its own check that it
/// does not — see `mesh`'s ε inventory, which asserts the absence of
/// raw strings in the tree it walks rather than assuming it.
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
/// silent mis-parse. Deliberately conservative: it looks for `r"` or
/// `r#` preceded by a non-identifier byte, in the ORIGINAL text, so a
/// raw string mentioned inside a comment counts as a hit. A caller
/// that trips this is being told to check, not that it is wrong.
#[must_use]
pub fn mentions_raw_string(text: &str) -> bool {
    let b = text.as_bytes();
    (0..b.len()).any(|i| {
        b[i] == b'r'
            && matches!(b.get(i + 1), Some(&b'"') | Some(&b'#'))
            && !i
                .checked_sub(1)
                .is_some_and(|p| b[p].is_ascii_alphanumeric() || b[p] == b'_')
    })
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

    #[test]
    fn the_unmodelled_construct_is_detectable() {
        assert!(mentions_raw_string("let s = r\"eps\";"));
        assert!(mentions_raw_string("let s = r#\"eps\"#;"));
        assert!(!mentions_raw_string("for (name, r) in v"));
        assert!(!mentions_raw_string("let r = d / (bound + eps);"));
        // An identifier ending in `r` before a quote is not an opener.
        assert!(!mentions_raw_string("check(\"eps\")"));
    }
}
