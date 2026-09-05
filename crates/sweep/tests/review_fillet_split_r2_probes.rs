//! FILLET-SPLIT review probes (lane r2, PR 1964).
//!
//! Two mechanical guards for claims the unit leaves to prose. The split
//! itself is pinned well: an edit to a moved carve moves bits, and the
//! bit dump reds (measured — swapping `chord_site`'s `back`/`fwd`
//! reddens `bitdump_{die,pip_rims,chamfered_cube,ruled_band}`). What is
//! NOT pinned is the SHAPE the split established, and both rows below
//! are about the shape rather than the arithmetic.
//!
//! **Row 1 — the `kef` census's roster is the directory, not a list.**
//! `review_fillet_t_r1_probes`'s census reads four files named by hand
//! in a `CARVE_FILES` array. The spec asked to "widen the census to the
//! directory"; a hand list is the narrower thing, and it is narrow in
//! the GREEN direction — a fifth file under `blend/open/` carrying a
//! `kef` site is not read by the census at all, so the count stays 8
//! and the row stays green. This row walks `src/blend/` and asserts
//! (a) `blend/open/` holds exactly the files the roster names, and
//! (b) no file under `blend/` outside the roster touches `kef` in
//! code. Either way round, a new carve file forces the census to be
//! re-taken rather than silently escaping it.
//!
//! **Row 2 — nothing in the seam or the open bands is bare `pub`.**
//! `docs/FILLET-SPLIT-SPEC.md` Phase 2 clause 3 binds "no item becomes
//! `pub`", and the PR body lists the twelve `pub(super)` and the
//! `pub(in crate::blend)` widenings that the move forced. Nothing
//! computes with that list: the visibility of a moved item is exactly
//! the kind of thing a later edit relaxes one keyword at a time, and
//! neither the dump nor the census can see it. This row pins the one
//! bare `pub` the seam has (`ring_clearance_for_tests`, re-exported by
//! `test_support`) and reds on a second.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::{Path, PathBuf};

use test_utils::source::{code_only, crate_dir, rust_sources};

/// The files `review_fillet_t_r1_probes::CARVE_FILES` names, in its
/// order: the seam first, then the open bands' directory.
const CENSUS_ROSTER: [&str; 4] = [
    "blend/surgery.rs",
    "blend/open/mod.rs",
    "blend/open/planar.rs",
    "blend/open/ruled.rs",
];

fn blend_dir() -> PathBuf {
    crate_dir(env!("CARGO_MANIFEST_DIR")).join("src/blend")
}

/// `path` relative to `src/`, `/`-separated — the spelling
/// `CARVE_FILES` uses.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("a walked file lies under src/blend")
        .to_string_lossy()
        .replace('\\', "/")
}

/// **The census's roster is the carve's directory.**
///
/// Red when a file arrives under `blend/open/` (the roster no longer
/// spans the open bands), and red when any other file under `blend/`
/// gains a `kef` site (the roster no longer spans the carve). Both are
/// the same failure — a census that reads a hand list cannot notice a
/// file it was never handed — and both leave
/// `review_fillet_t_r1_probes` green while it is no longer censusing
/// what its own header says it censuses.
#[test]
fn the_kef_census_roster_is_the_carves_directory() {
    let blend = blend_dir();
    let files = rust_sources(&blend);
    assert!(
        files.len() >= 4,
        "the walk of {} found {} .rs files; blend/ holds the surgery, the two open bands \
         and their mod.rs at the very least",
        blend.display(),
        files.len()
    );

    let mut under_open: Vec<String> = files
        .iter()
        .map(|p| relative(&blend, p))
        .filter(|r| r.starts_with("open/"))
        .map(|r| format!("blend/{r}"))
        .collect();
    under_open.sort();
    let mut rostered_open: Vec<String> = CENSUS_ROSTER
        .iter()
        .filter(|r| r.starts_with("blend/open/"))
        .map(|r| (*r).to_string())
        .collect();
    rostered_open.sort();
    assert_eq!(
        under_open, rostered_open,
        "`review_fillet_t_r1_probes::CARVE_FILES` names the open bands by hand. The \
         directory and the list have diverged, so the one-`kef`-door census is no longer \
         taken over every file the carve spans — re-take it, and widen the list with it"
    );

    let strays: Vec<(String, usize, usize)> = files
        .iter()
        .map(|p| {
            (
                relative(&blend, p),
                code_only(&std::fs::read_to_string(p).unwrap()),
            )
        })
        .filter(|(r, _)| !CENSUS_ROSTER.contains(&format!("blend/{r}").as_str()))
        .map(|(r, code)| {
            (
                r,
                code.matches(".kef(").count(),
                code.matches("kef_minted(").count(),
            )
        })
        .filter(|(_, direct, door)| *direct > 0 || *door > 0)
        .collect();
    assert!(
        strays.is_empty(),
        "a file under blend/ that the `kef` census does not read touches `kef` in code \
         (file, `.kef(` sites, `kef_minted(` sites): {strays:?}. The census's site count \
         is summed over its roster, so a site outside the roster is invisible to it — add \
         the file to `CARVE_FILES` and re-take the count"
    );
}

/// **The move widened visibility by exactly the items the PR lists, and
/// nothing became `pub`.**
///
/// The spec's Phase 2 clause 3 is a claim about keywords and only a
/// human reading of the PR body checks it. This row makes the bare-`pub`
/// half mechanical over the seam and both open bands: exactly one item
/// is `pub`, and it is the test-support door that was already `pub` at
/// the merge base. A moved item promoted to `pub` — the easy accident,
/// since `pub(in crate::blend)` is the unfamiliar spelling and `pub`
/// compiles — reds here.
#[test]
fn the_seam_and_the_open_bands_export_one_public_item() {
    let blend = blend_dir();
    let mut sites: Vec<String> = Vec::new();
    for path in rust_sources(&blend) {
        let rel = relative(&blend, &path);
        if rel != "surgery.rs" && !rel.starts_with("open/") {
            continue;
        }
        let code = code_only(&std::fs::read_to_string(&path).unwrap());
        for line in code.lines() {
            let t = line.trim_start();
            // `pub(` is a restricted form; a bare `pub ` is not. No
            // line number is carried: a pin on one would red for every
            // edit above it, which is not what this row is about.
            if t.starts_with("pub ") {
                sites.push(format!("blend/{rel}: {}", t.trim_end()));
            }
        }
    }
    sites.sort();
    let expected = ["blend/surgery.rs: pub fn ring_clearance_for_tests<T: Decide + Bounds>("];
    assert_eq!(
        sites, expected,
        "`docs/FILLET-SPLIT-SPEC.md` Phase 2 clause 3: no item the move touches becomes \
         `pub`. The seam's one public item is `ring_clearance_for_tests`, which \
         `sweep::test_support` re-exports and which was already `pub` at the merge base; \
         everything else crossing the `open/` boundary is `pub(in crate::blend)` and \
         everything the open bands read back out of the seam is `pub(super)`. A second \
         entry here is a widening the PR body does not list"
    );
}
