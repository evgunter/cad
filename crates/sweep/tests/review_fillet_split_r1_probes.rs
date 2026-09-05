//! FILLET-SPLIT review probes (lane r1, PR 1964).
//!
//! **The move's two textual claims, pinned over the DIRECTORY.** Both
//! are statements about `crates/sweep/src/blend/` as a whole, so both
//! rows walk it ([`rust_sources`]) rather than naming files: a file
//! added under `blend/` later is inside the claim, not beside it.
//!
//! - **Visibility.** Everything that crossed into `open/` is
//!   `pub(in crate::blend)` (the exact scope `blend::surgery` and
//!   `blend::open::*` share) or `pub(super)` (the `mod` lines); the
//!   seam items `surgery.rs` exports to the open bands are
//!   `pub(super)`; nothing became `pub` or `pub(crate)`. Nothing else
//!   holds that — a `pub` slipped onto a moved item compiles, passes
//!   every carve row and shows in no dump. The two wider spellings in
//!   `surgery.rs` are the ring-clearance pair (`pub(crate) fn
//!   ring_clearance`, its `pub fn ring_clearance_for_tests` door),
//!   which predate the split, and the row names them so a third is a
//!   re-count rather than a silent widening.
//! - **The one `kef` door, directory-wide.** `review_fillet_t_r1_probes`
//!   reads the four files the carve spans; the PR's deviation 2 says
//!   widening to all of `blend/` would also hold. Pinned here: exactly
//!   one `.kef(` in the code of every file under `blend/`, in
//!   `surgery.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::{Path, PathBuf};

use test_utils::source::{code_only, crate_dir, rust_sources};

/// Every `.rs` under `src/blend/`, as `(path relative to `src/`, code-only text)`.
fn blend_sources() -> Vec<(String, String)> {
    let src = crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    rust_sources(&src.join("blend"))
        .into_iter()
        .map(|p: PathBuf| {
            let rel = p
                .strip_prefix(&src)
                .expect("a blend source lies under src/")
                .to_string_lossy()
                .replace('\\', "/");
            let text = std::fs::read_to_string(&p)
                .unwrap_or_else(|e| panic!("readable {}: {e}", p.display()));
            (rel, code_only(&text))
        })
        .collect()
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
fn nothing_under_blend_open_is_wider_than_its_one_caller_needs() {
    let sources = blend_sources();
    let open: Vec<&(String, String)> = sources
        .iter()
        .filter(|(rel, _)| Path::new(rel).starts_with("blend/open"))
        .collect();
    assert!(
        open.len() >= 3,
        "the open bands are `blend/open/{{mod,planar,ruled}}.rs`; found {:?}",
        open.iter().map(|(r, _)| r).collect::<Vec<_>>()
    );
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
    assert!(
        wider
            .iter()
            .any(|(s, t)| s.is_empty() && t.starts_with("fn ring_clearance_for_tests<")),
        "the bare `pub` item is the test-support door `ring_clearance_for_tests`: {wider:?}"
    );
}

#[test]
fn the_one_kef_door_holds_over_the_whole_blend_directory() {
    let sources = blend_sources();
    let kefs: Vec<(&str, usize)> = sources
        .iter()
        .flat_map(|(rel, code)| {
            code.match_indices(".kef(")
                .map(move |(i, _)| (rel.as_str(), i))
        })
        .collect();
    assert_eq!(
        kefs,
        vec![("blend/surgery.rs", kefs.first().map_or(0, |k| k.1))],
        "every file under blend/ together calls `Body::kef` at exactly one site, in \
         surgery.rs (`kef_minted`); found {kefs:?}"
    );
}
