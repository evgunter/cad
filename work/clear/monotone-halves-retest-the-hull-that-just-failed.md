---
id: monotone-halves-retest-the-hull-that-just-failed
kind: issue
title: after a cell against itself fails the monotone arm, its two halves re-test the hull that just failed
status: open
opened: 2026-11-01
priority: P3
cost: E
---


Found by the PRICE designers (2026-11-01). When a cell against itself fails
the monotone arm, it splits, and each half re-tests a hull containing the
one that just failed. Measure, and skip the re-test where it cannot pass.
