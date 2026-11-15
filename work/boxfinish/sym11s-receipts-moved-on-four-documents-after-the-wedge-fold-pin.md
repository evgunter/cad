---
id: sym11s-receipts-moved-on-four-documents-after-the-wedge-fold-pin
kind: issue
title: sym11's symbolic and numeric columns rose on four documents at PR 6963 (the profile's direction mint reads two signs per minted direction); pinned unjudged
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

**Which merges** (BOXFINISH's measurements, from its orchestrator's
comment on PR 6985, 2026-11-15; GREEN's own two-commit check agrees as
far as it reached: the pinned table still reads at `33828f00ed` and at
`967cca218`, the first-parent commit before PR 6963):

- **The symbolic and numeric columns are PR 6963's**
  (`boxfinish/profile-directions`, merge `4642f0594`): green on its
  first parent, red after. The profile's direction mint reads two signs
  per minted direction. That is this row.
- **The frozen column is PR 6953's** (`onehome/rim-span`, merge
  `380317144`): `r2_link` 3576 to 3575 and `r2_rounded_pad` 3989 to 4000
  match the stored table at PR 6963's merge and move there. Not this
  row: BOXFINISH's lane put that evidence on
  `work/onehome/sym11s-symbolic-column-fell-on-four-documents-with-the-rigid-motion-move.md`,
  and it is ONEHOME's to judge.

BOXFINISH's repair of PR 6963 is said to keep these two columns' new
readings (it changes which zeros mint an axis, not how many signs are
read), so the pin should stand after it.

**Owed.** Say that the added decisions (four more symbolic and four
more numeric on the plate, sixteen and sixteen on the pad) are the
direction mint's two sign readings and nothing else, and that they are
the change's intent.

Reproduce: `cargo nextest run -p editor-core -E 'test(sym11_the_exact_channel_never_contradicts_past_the_ceiling)'`
(the test prints every receipt).
