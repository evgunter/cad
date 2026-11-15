---
id: main-red-sym11s-numeric-column-fell-on-five-documents-after-the-least-turn-mint
kind: issue
title: Main red: sym11's numeric column fell by one or two on five documents after PR 6988 (the decided least-turn mint), a slow row the PR gate did not run
status: open
opened: 2026-11-15
priority: P0
cost: E
---


Filed by BOXFINISH (2026-11-15, 22:35 UTC), found by a local run of the
row before merging PR 6990 onto main.

**Measured** at ε = 1e-9, CI's profile, on main `0661cffb`
(`cargo nextest run -p editor-core --ignore-default-filter -E
'test(/sym11_exact_channel_rows/)'`):
`sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`
fails. A receipt is `[symbolic, numeric, frozen]`:

| document | as read | stored |
|---|---|---|
| `two_hole_plate` | `[979, 1046, 54]` | `[979, 1048, 54]` |
| `r1_annulus` | `[511, 548, 54]` | `[511, 549, 54]` |
| `r2_link` | `[786, 943, 2738]` | `[786, 944, 2738]` |
| `r2_filleted_bracket` | `[1367, 1545, 84]` | `[1367, 1547, 84]` |
| `r2_rounded_pad` | `[1483, 1709, 2773]` | `[1483, 1710, 2773]` |

**Attribution.** The row was green on PR 6990's head `30f7e77f`, which
had main up to PR 7003 (the lane, the reviewer and BOXFINISH each ran
it there). Since then main gained PR 6975 and PR 7003 (docs only),
tracker PRs, PR 7004 (two `work/boxes` files) and PR 6988
(`onehome/least-turn-mint`, 35 files under `crates/`). PR 6990's
merged head reads the same five receipts as main alone, so PR 6990 does
not move them. PR 6988 is the only code among them. Not bisected
further: which of its changes drops the numeric decisions is not
traced.

**Why the gate missed it.** The row is in the slow set, and a PR's
slow-set step runs only slow rows whose names sit on added lines
(`work/ciw/a-stored-data-change-runs-none-of-the-slow-rows-that-read-it.md`).

**The fix** is the owning lane's: say which decisions the least-turn
mint no longer takes on these five documents (the row's own message:
"a number that moved is a decision that moved and is a finding, not a
table to refresh"), then re-pin with that account.
(BOXFINISH orchestrator)
