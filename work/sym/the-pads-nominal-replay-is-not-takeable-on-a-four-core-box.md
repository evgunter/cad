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

## What the OTHER instrument does take (SYM-9's fix pass, 2026-09-24)

The pad is NOT unmeasurable on this box: the affordability line's own
instrument, one whole-box leaf at `1e2·ε`
(`m10_10_leaf_cost_with_and_without_the_algebra`), runs in RELEASE in
about two minutes a take, and it reads the leaf's receipt. With the
shipped rules the pad's leaf is `[890, 6, 150, 907]` (A1's pad row, to
the decision), in 131.3 s at one attempt per rung and 147.7 s with
`SymRetry::kept_atom`, which recovers nothing there (`retried` 0 with
either mask alone, either order, and both together). So SYM-9's claim
about the ladder covers six documents on the leaf and five at the
nominal, and what is still missing is the NOMINAL replay in dev — the
instrument Phase 1's rung funnel and per-shape table are taken on.

The pad's leaf itself moved a long way since SYM-8's reading of the
same instrument (18.8 s at rule F on); the decision read is not why —
with it shut (`SymRules::without_the_reads`) the leaf is 132.8 s — and
this row does not say what is.

## Who takes it

No unit does yet. It sits on SYM's slate unscheduled; the next unit that
needs a claim about the pad at its nominal owes this row first, and a
unit that states a six-document result owes it before stating it.

## Home

`crates/editor-core/tests/m10_8_r2_probes_interval.rs` (the `pad`
door), `crates/editor-core/tests/sym_9_retry_interval.rs` (the rows
that could not take it). Filed by SYM-9's lane with the readings above.
