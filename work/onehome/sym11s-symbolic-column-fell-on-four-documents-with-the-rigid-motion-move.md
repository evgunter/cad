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

## Pinned, and two more moves bisected (GREEN, `green/goldens`, 2026-11-14)

From nightly 43371638371 (main `e36675c1`). The row read, identically at ε 1e-6, 1e-9 and 1e-12 and on main `b363f5fa1`:

| document | stored (PR 5889's table) | main |
|---|---|---|
| `two_hole_plate` | `[1203, 824, 229]` | `[1015, 1050, 299]` |
| `r1_annulus` | `[720, 555, 212]` | `[535, 553, 278]` |
| `r2_link` | `[998, 821, 5161]` | `[813, 947, 3576]` |
| `r2_filleted_bracket` | `[1618, 1159, 1313]` | `[1425, 1545, 969]` |
| `r2_rounded_pad` | `[1684, 1262, 3268]` | `[1515, 1698, 3989]` |

**GREEN pinned `PAST_THE_CEILING` at main's column**, with this row named in the file, so the suite is green and the table reds on its next move. No contradiction fired and both dispute columns are zero, which is the row's soundness claim; what is unjudged is which decisions moved.

**Two moves bisected, first-parent at ε 1e-9, over `9a256348..e36675c1`** (the plate's receipt; every document moves at both):

- `[1173, 824, 259]` at `f162f739d` became `[1015, 670, 191]` at `3d3fe2311`, the merge of PR 6820 (`onehome/rigid-motion`), the next first-parent commit. All three columns fell.
- `[1015, 670, 191]` at `e545801e0` became `[1015, 1050, 299]` at `3b59315dc`, the merge of PR 6869 (`onedoor/wedge-fold-2`, "every edge reader of the dihedral class reads the whole-edge fold"), the next first-parent commit. Numeric and frozen rose; that one is ONEDOOR's to account for.

Unchanged from `3b59315dc` to main. Not bisected: how the plate got from the `1189` this row records at PR 6717 to the `[1173, 824, 259]` read at `9a256348` and `f162f739d`, and the frozen column's older `229 -> 259`.

Reproduce: `cargo nextest run -p editor-core -E 'test(sym11_the_exact_channel_never_contradicts_past_the_ceiling)'` at each commit named (the assertion prints every receipt).
