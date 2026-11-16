---
id: a-dimensionless-reading-is-levered-by-hand-at-the-seam-germ-side
kind: issue
title: The join's seam-germ side levers a cosine by hand, Margin::of(into_region.dot(n_f) * len)
status: open
opened: 2026-11-11
priority: P3
cost: E
refs: [main-red-a-needle-cavity-12eps-channel-escalates-at-bool-contact-vertex, the-moved-walls-transversal-levers-a-sine-by-hand]
---

Found by the full review of FLUSH's PR 6812 (2026-11-11), which took the
same pattern out of `reduce::held_at_corners` before it landed. Read, not
measured.

**What.** `boolean::join::germ_side` (`crates/topo/src/boolean/join.rs`,
read by `seam_germ_side`, PR 5694) hands
`Margin::of(into_region.dot(n_f) * len)` to the funnel: a cosine times
the chord's extent. `Margin::of`'s doc (`geom_core` `predicate.rs`)
rules that out: an angular reading is levered through `Margin::levered`
with a `geom_core::Lever` that names its source.
`scripts/gates/lever-sources.sh` names `Margin::of(x * arm)` as a shape
only review holds. The same shape at `shell.rs`'s moved walls is SHELL's
row `work/shell/the-moved-walls-transversal-levers-a-sine-by-hand.md`.

**What is owed.** Re-spell it through `Margin::levered` with a sourced
`Lever` (the chord's consumed extent), or say why the arm fits no source
and spell it `Lever::unsourced` with its `// Unsourced lever:` line.
