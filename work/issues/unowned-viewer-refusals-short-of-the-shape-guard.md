---
id: unowned-viewer-refusals-short-of-the-shape-guard
kind: issue
title: editor-core/geom-brep: refusals the viewer draws on unowned ground that state no recourse or open with a label, by the shape guard's census
status: open
opened: 2026-09-29
priority: P2
cost: M
---


(CHROME `refusal-residue`, from the shape guard's structural prefix
and zero-recourse checks.)

## What

`work.py territory` names no owner for `crates/editor-core/src/checks.rs`,
`crates/editor-core/src/names/` or `crates/geom-brep/src/certify.rs`.
The shape guard (`crates/test-utils/src/refusal.rs`) found three things
on that ground.

1. **Check findings with no recourse.** Eight checks-window rows state
   none: `Check/ChartCoherence(meridian closure)`,
   `Check/ChartCoherence(rim)`, the three
   `Check/ChartCoherenceUnexamined(…)` rows, and
   `Check/SeparationUnavailable/Containment(CorruptFace | NoSuchSolid |
   ZeroVolumeBody)`. They are admitted by exact id in
   `crates/editor-core/tests/refusal_concision_chains.rs`
   `FILED_NO_RECOURSE`, under the comment naming this file.
2. **Name lineage refusals open with a label.** `Naming/SplitLineage`
   and `Naming/FragmentLineage` render `… does not hold: split lineage of
   edge EdgeKey(…) cycles: …`, which the guard reads as a label (it has
   no word only a sentence has). Both rows are kernel-keyed, so the key
   stands. They are admitted by `FILED`.
3. **A reversed interval ends on a short-edge lever.**
   `CertifyError::IntervalNotForward` with a definite `Negative`
   verdict, read at a build, renders "the stored parameter interval runs
   backwards … Recourse: move the geometry so this edge is not
   vanishingly short" (`Transform/Certify` in the chain test). The
   `ParamSpan` decision's own comment (`CertCheck::ending`) says no
   kernel construction mints a reversed interval. So the lever looks
   wrong for this arm: a definitely reversed interval is not a short
   edge. This is unmeasured beyond that one rendering; the routing is
   `geom_brep::certify`'s `recourse` table.

## Repair shape

For 1 and 2, rewrite at the source to the standard and drop the entry;
the must-fire check turns a stale entry red. For 3, read the raise
sites of `IntervalNotForward` and decide whether a definite reversal at
a build should end in `KERNEL_DEFECT_ENDING`.
