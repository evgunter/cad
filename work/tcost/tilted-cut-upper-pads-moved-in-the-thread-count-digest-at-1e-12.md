---
id: tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12
kind: issue
title: sweep mass_props thread-count digest: tilted_cut_upper's vpad/apad moved at CAD_TOLERANCE_EPS=1e-12 after PR 4952 landed
status: open
opened: 2026-10-13
priority: P3
cost: E
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
