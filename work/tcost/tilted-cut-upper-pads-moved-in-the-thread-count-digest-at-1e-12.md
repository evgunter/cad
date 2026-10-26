---
id: tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12
kind: issue
title: sweep mass_props thread-count digest: tilted_cut_upper's vpad/apad moved at CAD_TOLERANCE_EPS=1e-12 after PR 4952 landed
status: closed
opened: 2026-10-13
priority: P3
cost: E
closed: 2026-10-26
---


## Finding

`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`, the
golden comparison) is red in the 1e-12 row of the PR gate. Only one row of
the digest differs:

    tilted_cut_upper  vpad 3e812c48b1faaaab -> 3e812c48b1eaaaab
                      apad 3ea9c26d0af80000 -> 3ea9c26d0ae00000

The values `v`/`a` and the verdict hash are unchanged. Only the pads
tightened, in their low bits.

PR 5021 (blend refusal text only) ran this row green on one main merge
and red on the next. The only kernel code that came in between was main's
merge of PR 4952 (`geom-brep/src/pcurve_cache.rs`, the envelope's radial
pair). That makes it main's row. The test's own message applies: if the
tighter pads are right, re-cut the table on main and say what moved.

**Also red at the default ε** (found by PCERT's `pcert/near-pole-read-tri-state`,
2026-10-14): on `origin/main` at `fe3da01f63`,
`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
fails at `eps=1e-9` with the same `tilted_cut_upper` pad move
(`vpad …b1faaaab → …b1eaaaab`, `apad …0af80000 → …0ae00000`), every
other line unchanged. Reproduced with `crates/topo/src/pcurves.rs` at
`origin/main`'s, so it is not that branch's. The row is in the slow
set, so per-PR CI runs it only when a diff seeds sweep's slow tests.

## Closed (2026-10-26, tracker sweep)

**Superseded.** The re-cuts named on
`the-thread-count-digests-sym-frozen-counts-moved-on-main`'s closure
(`8bfc8e2319`, `0da1cd59e7`, `1cfffd8748`) re-baselined
`tilted_cut_upper` too. `crates/sweep/tests/thread-count-digest/eps-1e-9.txt:3`
now reads `vpad=3e812c48b2000000 apad=3ea9c26d0ae00000`, which matches
neither side of this row's delta, and `eps-1e-12.txt:3` has moved as
well. The move this row reports is no longer in the tree.
