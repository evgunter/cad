---
id: reader-census-blind-in-dot-path-clone
kind: issue
title: reader_census walks nothing in a clone whose path has a dot-prefixed component — every ledger row then reads as stale
status: open
opened: 2026-08-29
github: 1234
refs: [1222]
---

## From GitHub issue 1234

Opened 2026-08-29; 0 comments.

Surfaced during BLEND-1 verification ([PR #1222](https://github.com/evgunter/cad/pull/1222), "Verification" section) and verified there against a pristine `origin/main` worktree, where it fails identically.

**The defect.** `crates/test-utils`'s `reader_census::every_site_that_reads_rust_source_is_in_the_ledger` skips path components starting with `.` during its walk. In any clone whose absolute path contains a dot-prefixed directory (e.g. a worktree under a `.claude/` or other dot-dir), the walk finds **no** Rust sources at all, so every ledger line reads as stale and the suite reds — a false red about the environment, not the tree. Hosted CI checks out to a path with no dot component and stays green, so the gate never sees it; the people who see it are exactly the local lanes the census is supposed to serve.

**Class note.** The skip rule presumably exists to avoid `.git`/target-adjacent noise; the fix wants the rule scoped to components *below the walk root* rather than applied to the root's own ancestors. Any other in-repo walker with the same ancestor-blind skip is the same class — worth one sweep when this is taken.

Not claimed by S-BLEND — filed for whichever lane owns `crates/test-utils`' census machinery (track W territory in the SMELL partition).

## Home

`work/issues/` — the census machinery in `crates/test-utils` is track W / S-QA-shaped ground, and S-QA is closed, so no open program's territory or charter claims it.
