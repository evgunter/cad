---
id: sym11s-symbolic-column-fell-on-four-documents-with-the-rigid-motion-move
kind: issue
title: sym11's symbolic column fell by 7 to 14 on four documents with PR 6717's rigid-motion move, a decision that moved
status: open
opened: 2026-11-11
priority: P2
cost: E
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`).

`editor-core::all sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling` was already red before this window, on its numeric and frozen columns (nightly 42761779270: `two_hole_plate: [1203, 824, 259], stored [1203, 824, 229]`). In this window its symbolic column moved too, at ε 1e-9 and 1e-6:

| document | `50cfcf499d` | `b2916558d9` |
|---|---|---|
| `two_hole_plate` | 1203 | 1189 |
| `r1_annulus` | 720 | 713 |
| `r2_link` | 998 | 991 |
| `r2_filleted_bracket` | 1619 | 1605 |

**Bisected, first-parent, at ε 1e-9:**
- 1203 at `2e4c44de`, `10e131e7`, `e2df00ef` and `61844c4a`, which is `08f0298b^1`;
- 1189 at `08f0298b`, the merge of PR 6717 (`onehome/rigid-motion`), and on main `019bed88`.

The row's own text: "a number that moved is a decision that moved and is a finding, not a table to refresh". So: name the decisions PR 6717's rigid-motion move dropped from the symbolic tier (the rigidity loop in `admitted_for_a_suite`, which the geom-core roster also followed), and argue that the drop is right. Then re-cut the row's table together with whoever owns its numeric and frozen columns' older move.
