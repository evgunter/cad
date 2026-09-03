---
id: reader-census-selftest-vacuous-in-worktree
kind: issue
title: reader census selftest vacuous-fails in any dot-component worktree checkout (repo_root + dot-skip walk)
status: open
opened: 2026-08-29
github: 1274
refs: [1231, 1234]
---

## From GitHub issue 1274

opened 2026-08-29, 0 comments.

Found during M10-3's fix pass (PR 1231), reported as a pushback rather than fixed there since it is repo tooling, not that unit's scope.

`every_site_that_reads_rust_source_is_in_the_ledger` (the reader census) cannot pass in any `.claude/worktrees/…` checkout: `repo_root()` canonicalizes to a path containing a dot component, and the census walk skips every dot-component path — so `found` comes back empty and all 34 ledger entries read as stale. It passes on CI's checkout and on a plain clone.

Consequence: every worktree-based lane that runs the census locally gets a false red (or learns to skip it), which is exactly the shape the census exists to prevent. Fix direction: make the walk's dot-skip relative to the repo root rather than absolute (skip dot components *below* the root, not in the root's own path), or have `repo_root()` hand the walk a root-relative iterator.

## Home

`work/issues/` — the same census machinery as issue 1234: track W / S-QA-shaped repo tooling with no open program's territory over it.
