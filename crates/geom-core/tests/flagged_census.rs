//! The clause-(i) debt lane's census count assertion (the standing
//! rule on [`geom_core::k_stats::decide_flagged`]; the lane is tracked
//! as issue #214): **no new `decide_flagged` site ships without a
//! ledger row** in `docs/predicate-dimension-audit.md`. This test pins
//! the number of SHIPPED call sites (crate `src/` trees — fixtures and
//! demos carry prose reasons instead of rows and are not counted) to
//! the ledger's inventory: F2 ×4, F6 ×3, F7 ×1, F10 ×1 (one loop over
//! seven rigidity residuals), F13 ×1, F14 ×1, F15 ×1 — **12 sites**.
//!
//! Adding a site without updating BOTH the ledger and this count fails
//! the suite; retiring a flagged family (its own unit) decrements it.
//! The count only moves together with the ledger — the rule enforces
//! itself.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::{Path, PathBuf};

/// The ledger's current shipped `decide_flagged` inventory.
const LEDGER_FLAGGED_SITES: usize = 12;

fn count_in_tree(dir: &Path, hits: &mut Vec<(PathBuf, usize)>) {
    for entry in std::fs::read_dir(dir).expect("readable src tree") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            count_in_tree(&path, hits);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).expect("readable source file");
            let n = text.matches("k_stats::decide_flagged(").count();
            if n > 0 {
                hits.push((path.clone(), n));
            }
        }
    }
}

#[test]
fn shipped_decide_flagged_sites_match_the_ledger() {
    // geom-core sits at <workspace>/crates/geom-core.
    let crates_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates dir")
        .to_path_buf();
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(&crates_root).expect("crates dir listing") {
        let src = entry.expect("dir entry").path().join("src");
        if src.is_dir() {
            count_in_tree(&src, &mut hits);
        }
    }
    let total: usize = hits.iter().map(|(_, n)| n).sum();
    assert_eq!(
        total, LEDGER_FLAGGED_SITES,
        "shipped decide_flagged sites diverged from the ledger's inventory \
         (docs/predicate-dimension-audit.md, clause-(i) section; issue #214). \
         A new site needs a ledger row FIRST, then this count moves with it. \
         Sites found: {hits:#?}"
    );
}
