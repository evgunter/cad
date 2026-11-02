---
id: seat7-sweep-digest-drifted-on-main
kind: issue
title: seat7_sweep_lowering's committed corner_table digest no longer matches on main
status: open
opened: 2026-11-01
---


`crates/editor-core/tests/seat7_sweep_lowering.rs`
`the_sweep_documents_evaluate_to_their_committed_digests` is red on main at
8509df2510 (reproduced on that base by the WIRE `wire-carries` lane,
2026-11-01): `corner_table` gives `0xfa506ba6d180fc96` where the test pins
`0x8078f0efe3f11dd2`. The document has no split or shell, so PR 6268 is not
the cause. Not bisected; the cause is unknown. The fix is to find the commit
that moved it and either restore the reading or re-pin the digest with that
commit's justification, not to re-pin blind.
