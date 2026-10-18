---
id: the-slide-charts-indeterminate-arm-is-unreachable
kind: issue
title: MateFault::Indeterminate and PoseDegenerate{Chart} in Translations::slide are unreachable since F: no case split remains
status: open
opened: 2026-10-18
priority: P3
cost: E
---

Found by the review of INTENT stage 3 F (PR #5299, finding 10).

F retired the measured fold's case splits. The only remaining producer of `MateFault::Indeterminate` and `NodeErrorKind::PoseDegenerate { Chart }` in the value solve is `Translations::slide`'s `coset::across` (`crates/editor-core/src/mate/value.rs:287-309`). That call crosses the chart's `x` with its normal, and the two are perpendicular by construction, so it cannot fire.

`MateFault::Indeterminate`'s doc and Display text in `crates/editor-core/src/mate.rs` still read "a case-split predicate" and "a case split could not be decided", but no case split remains.

**The work.**
1. Mint the slide's left line structurally from the chart, and retire the arm.
2. Then retire `MateFault::Indeterminate` if nothing else produces it, restating its tag, class and Python binding with it.
3. Re-baseline any kstats that move.
