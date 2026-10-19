---
id: tier-3-could-check-each-joints-deck-element-is-at-most-one-period
kind: issue
title: Once every image is placed in its chart's principal window, tier 3 could check that each joint's stored deck element is at most one period; no check reads it today
status: open
opened: 2026-10-15
priority: P3
cost: E
design: true
---


Filed with PR 5126 (unit A of the `[ev]` PR 5142 answer).

## What

The walk now reads each joint's deck element from the half-period
marks at zero, with no cap (`geom_brep::Period::decide`), so a stored
`Deck` (`crates/topo/src/joint.rs:69`) can carry any whole number of
periods in `u` and `v`. Once unit B places every image over its edge's
span in the chart's principal window, two neighbouring images sit
within one period of each other, and a joint whose element is more than
one period out names a row that was not placed. Nothing reads that
today: `validate_pcurves` (`crates/topo/src/pcurves.rs:5002`) re-walks
and compares the elements but states no bound on them.

## The design question

Whether "|k| ≤ 1 on each channel" is a C4 invariant of a placed row
that tier 3 checks, or only a fact about today's producers. Unit B
carries the bound itself as a row; this item is whether tier 3 then
enforces it on what a body stores.
