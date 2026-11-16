---
id: main-red-sym11s-numeric-column-fell-on-five-documents-after-the-least-turn-mint
kind: issue
title: Main red: sym11's numeric column fell by one or two on five documents after PR 6988 (the decided least-turn mint), a slow row the PR gate did not run
status: closed
opened: 2026-11-15
priority: P0
cost: E
closed: 2026-11-16
pr: 7016
branch: onehome/sym11-least-turn
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

## Closed (2026-11-16): the identity charts, measured

The decisions the least-turn mint no longer takes are the `mate_chart_pole`
verdicts of the identity charts. A pair that is one direction exactly turns by
the identity with no pole verdict (its chord is 2), so each mate chart whose
mated directions agree exactly logs no pole row. Numeric only: the plate −2,
the annulus −1, the link −1, the bracket −2 and the pad −1. This is the same
pattern PR 6988 measured on `sym_9_retry_interval` and `m10_9_pins_interval`.

Measured by one variant: the mint deciding the pole at the identity again
(`least_turn_minted`'s identity branch). With it, the row reads every stored
receipt and passes; without it, it reads the five as found. The row is
re-pinned with that account in its doc.
