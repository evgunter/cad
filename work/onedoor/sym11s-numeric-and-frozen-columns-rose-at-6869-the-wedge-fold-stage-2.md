---
id: sym11s-numeric-and-frozen-columns-rose-at-6869-the-wedge-fold-stage-2
kind: issue
title: sym11's numeric and frozen columns rose on all five documents at PR 6869 (the wedge fold, stage 2): decisions that moved, pinned unjudged
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Filed by GREEN's `green/goldens` lane (PR 6956), from nightly 43371638371.

`crates/editor-core/tests/sym11_exact_channel_rows.rs` `sym11_the_exact_channel_never_contradicts_past_the_ceiling` reads a receipt `[symbolic, numeric, frozen]` per document. Its own text: "a number that moved is a decision that moved and is a finding, not a table to refresh".

**Bisected, first-parent at ε 1e-9:** at `e545801e0` and at `3b59315dc`, the merge of PR 6869 (`onedoor/wedge-fold-2`, "every edge reader of the dihedral class reads the whole-edge fold"), which is the next first-parent commit:

| document | `e545801e0` | `3b59315dc` |
|---|---|---|
| `two_hole_plate` | `[1015, 670, 191]` | `[1015, 1050, 299]` |
| `r1_annulus` | `[535, 401, 174]` | `[535, 553, 278]` |
| `r2_link` | `[817, 667, 3299]` | `[813, 947, 3576]` |
| `r2_filleted_bracket` | `[1427, 1006, 741]` | `[1425, 1545, 969]` |
| `r2_rounded_pad` | `[1523, 1214, 3683]` | `[1515, 1698, 3989]` |

Numeric and frozen rose on every document; symbolic fell by 2 to 8 on three. The readings are the same at 1e-6 and 1e-12 on main, and unchanged from `3b59315dc` to main. No contradiction fired and both dispute columns are zero.

GREEN pinned `PAST_THE_CEILING` at main's readings, so the row is green and reds on the next move. PR 6820's move in the same table is ONEHOME's, on `work/onehome/sym11s-symbolic-column-fell-on-four-documents-with-the-rigid-motion-move.md`.

**Owed.** Say which decisions the whole-edge fold moved from the symbolic tier to the numeric one (380 more numeric decisions on the plate) and whether that is the fold's intent or a cost.

Reproduce: `cargo nextest run -p editor-core -E 'test(sym11_the_exact_channel_never_contradicts_past_the_ceiling)'` at `e545801e0` and at `3b59315dc` (the assertion prints every receipt).
