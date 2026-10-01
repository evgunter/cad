---
id: kev-merge-rebases-carriers-names-a-door-and-no-geometric-lever
kind: issue
title: MergeRebasesCarriers names the describing kill door and no geometric lever (D4 ¶1 (i))
status: open
opened: 2026-10-01
priority: P4
cost: E
refs: [kevs-fan-merge-needs-a-re-describing-kill-door, set-face-surface-passes-a-swap-off-the-faces-own-boundary, 3598]
---

## What

Found by the PR 3598 fix pass's sweep for keys-only refusals whose text
names only the describing sibling. D4 ¶1 (i) asks every refusal for
*move the geometry, phrased as the decision's own lever, always*.
PR 3598 gave `RechartStrandsDescriptions` and `RechartUnvouched` theirs
(the chart and the descriptions). The kill family's
`EulerOpError::MergeRebasesCarriers` (`crates/topo/src/euler.rs`,
`EulerOpError::render`) still ends in a door alone: "(kev_describing
takes one, and their re-descriptions)".

## Fix

End its text in its own lever, along the lines of "Recourse: kill an
edge whose surviving fan has no certified member, or re-describe the
members at the merged vertex (kev_describing ...)". Word it to fit
the decision, and pin the render on a real raise, as
`attach::tests::the_keys_only_rechart_refusals_end_in_the_chart_as_their_lever`
does.
