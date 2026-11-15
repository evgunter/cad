---
id: sym11s-receipts-moved-on-four-documents-after-the-wedge-fold-pin
kind: issue
title: sym11's receipts moved on four documents after PR 6956 pinned them: symbolic and numeric up on all four, frozen up on the pad and down by one on the link; pinned unjudged
status: open
opened: 2026-11-15
priority: P2
cost: E
---


Filed by GREEN's `green/nightly-1115` lane, from nightly 43680833019
(main `dbb21b4a`), where
`editor-core::all sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`
is red at every ε row.

The row reads a receipt `[symbolic, numeric, frozen]` per document past
its ceiling. Against the table PR 6956 pinned (the readings PR 6869 left,
`work/onedoor/sym11s-numeric-and-frozen-columns-rose-at-6869-the-wedge-fold-stage-2.md`):

| document | pinned by PR 6956 | main `3e7a8c93b` |
|---|---|---|
| `two_hole_plate` | `[1015, 1050, 299]` | `[1019, 1054, 299]` |
| `r1_annulus` | `[535, 553, 278]` | unchanged |
| `r2_link` | `[813, 947, 3576]` | `[814, 948, 3575]` |
| `r2_filleted_bracket` | `[1425, 1545, 969]` | `[1433, 1553, 969]` |
| `r2_rounded_pad` | `[1515, 1698, 3989]` | `[1531, 1714, 4000]` |

The same at ε 1e-6, 1e-9 and 1e-12, and the same in a build at the
nightly's `opt-level = 2` and at the default dev profile. No
contradiction fired and both dispute columns are zero on every document.

GREEN re-pinned `PAST_THE_CEILING` at main's readings, so the row is
green and reds on the next move. The pin is not a judgment.

**Which merge: not bisected.** It is on BOXFINISH's slate on the
strength of BOXFINISH's own log line (`work/green/log.md`, 2026-11-14):
PR 6963 (`boxfinish/profile-directions`, merge `4642f0594`) "moves
`sym11_exact_channel_rows`' symbolic call counts too, by the same mint
readings it re-pins in five other files". That claim covers the
symbolic column; the numeric column rose by the same amounts and the
frozen column moved on two documents, which it does not speak to.
GREEN's check at `967cca218` and `4642f0594` did not complete (its box
ran out of memory building three targets at once). If the move is not
PR 6963's, this row moves to the slate of the merge it is.

**Owed.** Say which decisions were added (four more symbolic and four
more numeric on the plate, sixteen and sixteen on the pad), why the
pad's plain walks freeze eleven more nodes, and whether that is the
intent of the change that moved them.

Reproduce: `cargo nextest run -p editor-core -E 'test(sym11_the_exact_channel_never_contradicts_past_the_ceiling)'`
(the test prints every receipt).
