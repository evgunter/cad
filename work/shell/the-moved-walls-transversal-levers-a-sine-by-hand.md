---
id: the-moved-walls-transversal-levers-a-sine-by-hand
kind: issue
title: The shell's moved-walls transversal levers a sine by hand, Margin::of(cross.norm() * lever)
status: open
opened: 2026-11-11
priority: P3
cost: E
refs: [a-dimensionless-reading-is-levered-by-hand-at-the-seam-germ-side]
---

Found by the full review of FLUSH's PR 6812 (2026-11-11), which took the
same pattern out of `reduce::held_at_corners` before it landed. Read, not
measured.

**What.** `crates/topo/src/shell.rs`, the `shell_moved_walls_transversal`
decide, hands `Margin::of(cross.norm() * lever)` to the funnel: the sine
between two walls' normals times the extent `shell_moved_walls_extent`
measured. `Margin::of`'s doc (`geom_core` `predicate.rs`) rules that
out: an angular reading is levered through `Margin::levered` with a
`geom_core::Lever` that names its source. `scripts/gates/lever-sources.sh`
names `Margin::of(x * arm)` as a shape only review holds.

**What is owed.** Re-spell it through `Margin::levered` with a sourced
`Lever` (the walls' measured extent), or say why the arm fits no source
and spell it `Lever::unsourced` with its `// Unsourced lever:` line.
