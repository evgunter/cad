---
id: editor-core-suites-carry-eleven-part-resolver-stubs
kind: issue
title: Eleven editor-core suites carry their own PartResolver stub, in_part and PART_BODY; MSOLVE-4 hoists one into tests/fixture and the ten siblings owe a migration
status: open
opened: 2026-09-06
---


Found by MSOLVE-4's style review (PR 1960); filed by the MSOLVE
orchestrator. Test infrastructure, no obvious program owner (TCOST or
CIW by shape).

`impl PartResolver for StubStore` with an `insert` that pins a document
and a `resolve` that checks the pin, plus `in_part(instance, cap)` and
a `PART_BODY` constant, are copied across `mate1_member_vocab.rs`
("as in the sibling suites"), `mate1_r1_probes.rs`, `mate1r2_probes.rs`,
`mate6_gather_mints.rs`, `mate6r1_shared.rs`, `mate6r2_probes.rs`,
`asm_r2b_assembly.rs`, `fix_pattern_mate_crossing.rs`,
`rev_fix_xsplit_unreachable.rs`, `msolve1_transform_aware.rs` and (at
its review) `msolve4_mate_memo.rs`. MSOLVE-4's fix pass hoists one into
`crates/editor-core/tests/fixture/` and uses it from its own suite; the
ten siblings keep their copies until a migration lands. A copy that
drifts (a pin check dropped, a different failure text) is a suite
testing a different resolver than its neighbours believe.

## Same class, second instance (MSOLVE-2's style review, 2026-09-06)

`in_copy(pattern, i, master)` has seven private copies across the
mate suites (`fix_pattern_mate_crossing`, `mate1_member_vocab`,
`mate1_r1_probes`, `mate1r2_probes`, `msolve1_transform_aware`,
`rev_fix_xsplit_unreachable`, `msolve2_member_chain`), and `run`,
`gate`, `xform` travel with it. MSOLVE-2's fix pass hoists one set
into `tests/fixture/` beside the seat oracle for its own two suites;
the rest migrate with the resolver stub.
