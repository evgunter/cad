//! FILLET-SPLIT review probes (PR 1964): the ONE visibility row, folded
//! from both review lanes' probes at the fix pass.
//!
//! The split itself is pinned by the bit dump (an edit to a moved carve
//! moves bits — measured: swapping `chord_site`'s `back`/`fwd` reddens
//! `bitdump_{die,pip_rims,chamfered_cube,ruled_band}`) and the one-`kef`
//! door by `review_fillet_t_r1_probes`, which walks the whole `blend/`
//! directory. What neither can see is the SHAPE the move established at
//! the `open/` boundary: `docs/FILLET-SPLIT-SPEC.md` Phase 2 clause 3
//! binds "no item becomes `pub`", and the visibility of a moved item is
//! exactly the kind of thing a later edit relaxes one keyword at a time
//! — `pub` compiles, passes every carve row and shows in no dump.
//!
//! One row, three facts about `surgery.rs` and every file under
//! `blend/open/`, walked ([`rust_sources`]) rather than listed:
//!
//! 1. **The exact bare-`pub` set** (lane r2): exactly one item is `pub`,
//!    the test-support door `ring_clearance_for_tests`, which
//!    `sweep::test_support` re-exports and which was `pub` at the merge
//!    base. A second entry is a widening the PR body does not list.
//! 2. **Everything under `open/` is `pub(in crate::blend)` or
//!    `pub(super)`** (lane r1): the exact scope `blend::surgery` and
//!    `blend::open::*` share, or the `mod` lines. `pub(crate)` — which
//!    fact 1 cannot see — is a design change, not a move.
//! 3. **`surgery.rs` carries exactly two items wider than `pub(super)`**
//!    (lane r1): the ring-clearance pair (`pub(crate) fn ring_clearance`
//!    and its bare-`pub` door), both predating the split, named so a
//!    third is a re-count rather than a silent widening.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::PathBuf;

use test_utils::source::{code_only, crate_dir, rust_sources};

/// `surgery.rs` and every `.rs` under `blend/open/`, as `(path relative
/// to `src/`, code-only text)`.
fn seam_and_open_bands() -> Vec<(String, String)> {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut out: Vec<(String, String)> = rust_sources(&src.join("blend"))
        .into_iter()
        .map(|p: PathBuf| {
            let rel = p
                .strip_prefix(&src)
                .expect("a blend source lies under src/")
                .to_string_lossy()
                .replace('\\', "/");
            (rel, p)
        })
        .filter(|(rel, _)| rel == "blend/surgery.rs" || rel.starts_with("blend/open/"))
        .map(|(rel, p)| {
            let text = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("readable {}: {e}", p.display()));
            (rel, code_only(&text))
        })
        .collect();
    out.sort();
    out
}

/// Every `pub` token in `code` as `(scope, the code that follows)`: the
/// scope is the text inside `pub(...)`, or empty for a bare `pub`; the
/// tail is the next 48 bytes of code, whitespace-collapsed, enough to
/// name the item.
fn pub_tokens(code: &str) -> Vec<(String, String)> {
    let bytes = code.as_bytes();
    let is_ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut out = Vec::new();
    for (i, _) in code.match_indices("pub") {
        if i > 0 && is_ident(bytes[i - 1]) {
            continue;
        }
        let mut j = i + 3;
        if j < bytes.len() && is_ident(bytes[j]) {
            continue;
        }
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let scope = if j < bytes.len() && bytes[j] == b'(' {
            let close = code[j..].find(')').expect("a closed visibility scope") + j;
            let s = code[j + 1..close]
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            j = close + 1;
            s
        } else {
            String::new()
        };
        let end = code
            .char_indices()
            .map(|(k, _)| k)
            .find(|&k| k >= j + 48)
            .unwrap_or(code.len());
        let tail = code[j..end]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        out.push((scope, tail));
    }
    out
}

#[test]
fn the_seam_and_the_open_bands_export_one_public_item_and_widen_nothing_else() {
    let sources = seam_and_open_bands();
    let open: Vec<&(String, String)> = sources
        .iter()
        .filter(|(rel, _)| rel.starts_with("blend/open/"))
        .collect();
    assert!(
        open.len() >= 3,
        "the open bands are `blend/open/{{mod,planar,ruled}}.rs`; found {:?}",
        open.iter().map(|(r, _)| r).collect::<Vec<_>>()
    );

    // Fact 1 — the exact bare-`pub` set over the seam and the open bands.
    let mut bare: Vec<String> = sources
        .iter()
        .flat_map(|(rel, code)| {
            pub_tokens(code)
                .into_iter()
                .filter(|(scope, _)| scope.is_empty())
                .map(move |(_, tail)| format!("{rel}: {tail}"))
        })
        .collect();
    bare.sort();
    assert_eq!(
        bare.len(),
        1,
        "`docs/FILLET-SPLIT-SPEC.md` Phase 2 clause 3: no item the move touches becomes \
         `pub`. The seam's one public item is `ring_clearance_for_tests`, which \
         `sweep::test_support` re-exports and which was already `pub` at the merge base; \
         found {bare:?}"
    );
    assert!(
        bare[0].starts_with("blend/surgery.rs: fn ring_clearance_for_tests<"),
        "the bare `pub` item is the test-support door `ring_clearance_for_tests`: {bare:?}"
    );

    // Fact 2 — everything under `open/` is `pub(in crate::blend)` or `pub(super)`.
    let mut crossing = 0usize;
    for (rel, code) in &open {
        for (scope, tail) in pub_tokens(code) {
            match scope.as_str() {
                "in crate::blend" => crossing += 1,
                "super" => {}
                other => panic!(
                    "{rel}: `pub({other})` on `{tail}` — the open bands' items are \
                     `pub(in crate::blend)` (the scope both `surgery` and `open::*` sit \
                     in) or `pub(super)` (the `mod` lines); a wider spelling is a design \
                     change, not a move"
                ),
            }
        }
    }
    assert!(
        crossing >= 8,
        "the corner and blank plans, the ruled plan and the two carves cross into \
         `open/` as `pub(in crate::blend)`; only {crossing} such items were found"
    );

    // Fact 3 — `surgery.rs`'s two items wider than `pub(super)`, named.
    let (_, surgery) = sources
        .iter()
        .find(|(rel, _)| rel == "blend/surgery.rs")
        .expect("blend/surgery.rs is the seam");
    let wider: Vec<(String, String)> = pub_tokens(surgery)
        .into_iter()
        .filter(|(scope, _)| scope.is_empty() || scope == "crate")
        .collect();
    assert_eq!(
        wider.len(),
        2,
        "surgery.rs carries exactly two items wider than `pub(super)` — the \
         ring-clearance pair, which predates the split; found {wider:?}"
    );
    assert!(
        wider
            .iter()
            .any(|(s, t)| s == "crate" && t.starts_with("fn ring_clearance<")),
        "the `pub(crate)` item is `ring_clearance`: {wider:?}"
    );
}
