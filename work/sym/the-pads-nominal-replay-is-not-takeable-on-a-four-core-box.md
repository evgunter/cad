---
id: the-pads-nominal-replay-is-not-takeable-on-a-four-core-box
kind: issue
title: R2's rounded pad's nominal replay does not finish in a lane's budget on a four-core box, so SYM-9's Phase 1 tables have five documents and not six
status: open
opened: 2026-09-22
---



## What was measured (SYM-9's Phase 1, 2026-09-22)

SYM-9's two Phase 1 tables are taken by replaying each measured
document at its nominal box, once per retry shape
(`editor-core/tests/sym_9_retry_interval.rs`). On a four-core box with
~15 GB, a dev build:

| document | one nominal replay |
| --- | --- |
| R1's segment boss | 1.5 s |
| two-hole plate | 2.2 s |
| R1's annulus | 2.5 s |
| R2's filleted bracket | 17.4 s |
| R2's link | 59.3 s |
| **R2's rounded pad** | **> 30 min, ONE replay, not finished** |

Two attempts were made. The first ran with the shape report installed
and was abandoned at 30 minutes without the first replay printing; the
second ran with `CAD_SYM_9_NO_REPORT=1`, which is the row's own door
for the pad (the report renders the plain and early forms of every
blocked residual, and the pad has ~991 of them), and was abandoned at
the same point. **Memory was never the wall**: 1.0 GB resident at 17
minutes with 11 GB free, and the process was using ~1.4 cores. It is
wall time in the walks, not the shape report and not the box's RAM.

So this is not the OOM the SYM-9 dispatch warned about — that one is
real and the `NO_REPORT` door answers it — but a SECOND wall behind it,
and the dispatch's warning hid it: a lane told the report OOMs takes
the report off and finds the replay still does not return.

## Why it matters

The pad is one of the six documents every SYM unit measures against,
and it is the document SYM-8's rule F moved 24 decisions on — the
mechanism SYM-9 is about, realised on a real document. SYM-9's Phase 1
tables therefore have five rows where the spec asks for six, and the
unit's claim that the retry ladder recovers nothing outside R2's link
and R2's filleted bracket is a claim about five documents.

## What would answer it

1. **Time it properly first.** Nothing here is a profile: run
   `m10_sym_profile_interval`'s instrument over one pad replay and read
   which walk the time is in, rather than guessing from the rule
   table.
2. A release-profile replay. Every number above is dev; the tier's
   release/dev ratio on the slab is 2.7-3.05x
   (`geom_core::sym`'s `# Cost`), which would not on its own bring 30
   minutes under a lane's budget but would say whether the shape is
   the same.
3. Failing both, the pad's row belongs on a box with more cores and
   the measurement belongs in a dispatched run rather than inside a
   lane's turn.

## Home

`crates/editor-core/tests/m10_8_r2_probes_interval.rs` (the `pad`
door), `crates/editor-core/tests/sym_9_retry_interval.rs` (the rows
that could not take it). Filed by SYM-9's lane with the readings above.
