---
id: main-red-the-mass-props-thread-golden-since-4938
kind: issue
title: Main red (sweep slow set, unseen by per-PR CI): mass_props_are_thread_count_invariant's serial golden moved for tilted_cut_upper; suspected PR 4938
status: review
opened: 2026-10-14
priority: P0
cost: E
branch: flux/main-reds-golden-and-split-rim
---


## The finding

On FUSE's PR 5030 (head 0fa506959), the FUSE lane hit a red row that reproduces on a pure `origin/main` worktree at `4533c4b12`.
- **Failing row:** `sweep` slow set `mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`.
- **What moved:** the `vpad`/`apad` bits of tilted_cut_upper differ from the serial golden.
- **Why main never ran it:** the change filter seeds a crate's slow set only from the files a diff touches.

The cause is not bisected. The suspect is PR 4938 (`flux/periodic-closure-one-decide`, merged 2026-10-13 15:05). It includes `104839ab4` (props: decide the axis-inside premise at the axis class's own threshold) and `571b7030d` (props: pin the torus carries). Both change the props walk that this golden digests. PR 4984 (nurbs) is the other candidate.

To reproduce: `cargo nextest run -p sweep -E 'test(the_walk_matches_the_serial_golden_at_four_threads)'`.

## What it needs

1. Bisect.
2. If the pads moved for a correct reason, re-baseline the golden and say what moved (`work/nurbs/main-slow-set-reds-on-two-rows-since-4553.md` is the precedent).
3. If they moved for any other reason, fix the source.

## Resolution (FLUX, `flux/main-reds-golden-and-split-rim`)

Not FLUX's, and already fixed on main; this row was left open.

- **What moved it:** PR #4952 (`pcert/envelope-radial-pair-whole`),
  which reads the pcurve envelope's radial pair whole. tilted_cut_upper's
  `vpad`/`apad` shrank at all three ε, and its value, area and verdict
  channel did not move: a correct move.
- **Who re-baselined it:** NURBS's PR #5055
  (`nurbs/recut-thread-count-digest-after-4952`, commit `cd00022655`,
  merged 2026-10-14 02:55Z), after a clean bisect: `5f0d1cf5` (the #4952
  merge) is the first failing merge, and `b6ef0ab7` passes. The sighting
  above, at `4533c4b12` (2026-10-13 22:41Z), predates that merge.
  #4938's merge (`63740ca8`) is an ancestor of the passing `b6ef0ab7`,
  so it did not move the pads. #4984 is not the first failing merge
  either, and current main, which carries it, passes.
- **On current main** (`e0f270aa`, which carries #5038 and #5333): the
  row passes at 1e-6, 1e-9 and 1e-12, and sweep's whole slow set
  (`--profile ci --ignore-default-filter`, the slow-set expression,
  default ε) runs 384/384 green. No other slow-set row is red, for this
  cause or any other.

Nothing to re-baseline; the row closes on this record.
