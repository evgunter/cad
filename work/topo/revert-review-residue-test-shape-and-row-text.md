---
id: revert-review-residue-test-shape-and-row-text
kind: issue
title: PR 4154's review residue: a vacuous unchanged-assert, refusal-named counters, no require_halves row through revert, stale row text
status: open
opened: 2026-10-06
priority: P4
cost: E
---


## What

Findings from the full review of PR 4154 (`topo: revert's torn reads panic; RevertError::Corrupt retires`), which merged without them. None is a soundness defect.

- `crates/topo/src/review_d18.rs:3704-3707`: `assert!(deep_snapshot(&trial) == snapshot, "revert wrote to the torn body it read")` cannot go red. `Body::revert` takes `&self`, so the type already guarantees it. Drop it, or state why it stays beside the mints' `&mut` version, which can fail.
- `review_d18.rs:2457-2471, 2585-2590`: `cells[1]`, `RenameCells::refused` and `refused_per_tear` now count premise *panics*. Rename them.
- No `revert` row tears the mate's edge (`require_halves`, `euler.rs:4671`). The "dead mate slot" row (`revert.rs:500-516`) pins only `{edge:?}` plus the row-four text, so it cannot tell the slot hop from the claim hop from `require_halves`. `revert_reads_whole` (`review_d18.rs:3813-3851`) also omits `require_halves` and `face_of_linked`'s loop-listing check. It is a hand-kept mirror of `revert`'s read set.
- Three hops `revert` now reaches name only "on a tier-1-valid body", without `OPERATORS_KEEP_LINKS`: `mate_of`'s unclaimed `unreachable!` (`euler.rs:4454`), `require_halves` (`euler.rs:4671`), and `face_of_linked`'s `assert!` (`body.rs:1647`). Three of `revert`'s five callers are mid-op.
- `proven_mate` / `mate_of` / `resolve_half_edge` sit in a new `impl<T: Real>` block at the bottom of `euler.rs` (`:4401-4403`), while `live.rs` hosts the same family of plan-phase link reads.
- Row text: `work/hone/kernel-bug-refusals-end-without-the-shared-ending.md:55-57` still lists `topo::RevertError`'s link arms as a live instance. Mark it retired, as its neighbouring `topo::pcurves` bullet does. `work/topo/torn-body-refusal-families-beyond-the-six-doors.md`'s `RevertError::Corrupt` cell names a branch.
