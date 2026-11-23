---
id: assertion-rows-spell-at-least-two-ways
kind: issue
title: Assertion rows spell 'at least' two ways: value rows >= and <=, clearance rows ≥ and ≤
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by the style review of PR 6357 (finding 1).

The feature tree's assertion rows spell one relation two ways:
a value assertion's comparison reads `0.0125 m >= 0.01 m`
(`AssertionRelation::symbol`, `crates/editor-core/src/measure.rs`, read
by `viewer::tree::Asserted::comparison`), and a clearance row's equation
reads `clearance(a, b) ≥ 2 mm` (`ClearanceBounds::equation`, the same
file, read by `viewer::tree::Cleared::equation`).

## Repair shape

One notation for every assertion row — the mathematical `≥ ≤ =` the
clearance equation uses (Ev on PR 6261: "display like it is an
equation"), or ASCII for both — chosen once and written by the kernel's
one door for each claim.
