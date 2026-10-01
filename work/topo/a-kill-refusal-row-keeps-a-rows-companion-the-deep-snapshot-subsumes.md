---
id: a-kill-refusal-row-keeps-a-rows-companion-the-deep-snapshot-subsumes
kind: issue
title: euler_ring.rs's torn-loop refusal row compares a pcurve-rows companion beside deep_snapshot, which walks the pcurve table
status: open
opened: 2026-09-30
priority: P4
cost: E
refs: [refusal-rows-that-count-instead-of-snapshot]
---

## What

Found by PR 3580's sweep for the shape
`refusal-rows-that-count-instead-of-snapshot` closed in `null.rs`: a
`rows` closure over `body.pcurves()` compared beside
`fixtures::deep_snapshot`. `deep_snapshot` walks the pcurve table, so
the companion asserts nothing the snapshot does not.

One instance is left, in `crates/topo/src/euler_ring.rs`'s tests (the
`refuses` closure, ~`:3686`: `let (before, rows_before) =
(deep_snapshot(body), rows(body))` and its `"{door}: every row is where
it was"` assertion). It sits in a file under a live TOPO lane's diff,
so PR 3580 left it.

The sweep's other hits are not this shape:
`crates/sweep/tests/euler_site_row_frontiers.rs` compares rows alone
across a successful operation, with no snapshot beside it.

## Shape

Drop the companion and its assertion, and fold "every row included"
into the snapshot assertion's message, as `null.rs` did.
