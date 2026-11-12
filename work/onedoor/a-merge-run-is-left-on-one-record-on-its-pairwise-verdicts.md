---
id: a-merge-run-is-left-on-one-record-on-its-pairwise-verdicts
kind: issue
title: A merge run is left on one record on its pairwise margins, not on one verdict read over every face the act moves
status: open
opened: 2026-11-11
---


Found building the one-record act (PR 6505, fix pass 2). The merge
decides a run's faces one carrier pair by pair across its edges
(`Body::faces_continue`, each pair read over its two faces'
`pair_extent`). Its act (`Body::one_record_of_run`,
`crates/topo/src/one_record.rs`) then leaves the whole run on one
record. `[coincidence.door]` reads a verdict once over the decided
faces and every face it moves. A chain of pairs each Zero over its own
two faces can stand up to the band times its length apart end to end,
and no reading covers the run's whole extent.

Today the act's write gate certifies every moved boundary on the kept
chart, so a run whose far member stands off the kept record refuses
rather than shipping wrong. Wanted: the run's verdict read once over
the run (the merge's token's extent), refusing with the coincidence
decision's lever where it is in band, or a ruling that pairwise reading
suffices for a run.
