---
id: main-red-the-mass-props-thread-golden-since-4938
kind: issue
title: Main red (sweep slow set, unseen by per-PR CI): mass_props_are_thread_count_invariant's serial golden moved for tilted_cut_upper; suspected PR 4938
status: open
opened: 2026-10-14
priority: P0
cost: E
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
