---
id: a-fillets-run-out-rides-by-segment-kind-only
kind: issue
title: A fillet's pending run out is claimed by the next emission of the same segment kind; that it lies on the arrival carrier is guaranteed only by the chain lattice, not checked
status: open
opened: 2026-09-25
priority: P3
cost: E
---


## The finding

This comes from the re-review of PR 3223 (S1). `PendingRunOut::rides`
(`crates/profile/src/path.rs`) compares only segment kind: a straight
emission rides a ray run out, and an arc rides a circle run out. That
is correct today only because the chain lattice refuses any straight
emission from a directed tip that leaves the arrival ray. Every such
continuation refuses as "not a legal chain-lattice walk".

`common::pinned`'s `assert_runs_ride_their_carriers` has the same
limit: it checks straight against arc, not collinearity or the same
circle.

If the lattice ever admitted `turn` or `line_to` after a directed `at`,
the fillet would silently claim a segment on another carrier. That is
PR 3223's B1 alias again.

## Fix direction

Make `rides` geometric: test that the emission lies on the arrival
carrier (collinear with the ray, or the same circle), and refuse
otherwise. Tighten the corpus pin the same way.
