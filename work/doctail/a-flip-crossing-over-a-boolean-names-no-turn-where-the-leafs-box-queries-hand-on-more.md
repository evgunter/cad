---
id: a-flip-crossing-over-a-boolean-names-no-turn-where-the-leafs-box-queries-hand-on-more
kind: issue
title: drive's FlipCrossing can name no turn at a boolean or a blend: an Interval leaf's box queries and blend thresholds answer otherwise than its f64 witness's, so the two paths part on a mark before any choice
status: open
opened: 2026-11-18
priority: P3
cost: M
refs: [verdict-log-tells-no-validation-reading-from-a-building-decision, 7114]
---

Filed by the DOCTAIL first-turn lane, on PR 7114's review.

`drive::classify_replay` refuses a leaf whose certifying vector differs from the f64 witness's, and names why with `diff_verdicts(witness, leaf)` (`RefusalReason::FlipCrossing`'s `FlipEvidence`). The gate is unchanged: it compares whole logs, and the candidate marks are not in it (`resolve::vdiff::VerdictVector`'s note).

The naming changed, and the change is quiet. Since PR 7114, a node's path carries the candidates its `bvh` queries and its marked pruning tests hand on (`NodeValue::marks`). An Interval leaf's boxes cover the leaf's whole parameter box, so at a boolean its queries can hand on a candidate the f64 witness, at one point of that box, prunes. The two paths then part at that mark (`FirstTurn::Parted`) before any choice differs. The node names no flip, and a `FlipCrossing` over a boolean can carry no turn at all, even where the first differing choice is the one that turned.

**Blends under an Interval leaf, worse.** The blend's reach and surgery mark every outcome of their `escalate` thresholds (`sweep::blend::reach::{sure, sure_below, fine}`, and the edge refinement's settled test). At an Interval leaf every reading's `lo()` and `hi()` widen with the leaf's box, so `sure` answers `false` where the f64 witness's point reading answers `true`, and a cell is subdivided where the witness's is not. A blend node under an Interval leaf will part from its witness almost always, at its first metered cell, and name no turn. The review of PR 7114 counted 24,261 marks against about 1k choices in one strip-cup blend node at f64, so the first such mark comes early.

The cost is evidence, not soundness: the leaf still refuses, and nothing is certified that was not. Profile nodes record no marks, so the drive and range rows that name turns there (`docm9_range`, `m10_3_driver_interval`) are unmoved.

**What would close it:**
- a drive row over a boolean, and one over a blend, that shows the evidence empty, or shows it is not;
- if it is empty, a mixed-scalar reading that compares marks only where both runs are point runs. It must say why an Interval run's extra candidates, which it then decides nothing about, are not a parting, which the log alone cannot show.

Alternatively, name where the paths parted (the query and the candidate), which is the PR 7114 question raised to Ev.
