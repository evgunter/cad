---
id: the-pose-screens-clear-and-parallel-record-minus-k-eps-and-land-in-band-under-k-lint
kind: issue
title: k-lint (dev-probe): curve_pair_pose_clear and _parallel record -Keps plus rounding, so 279 (1e-6) and 318 (1e-9) of them land in band; _tilt records exactly 2Keps
status: open
opened: 2026-11-10
priority: P2
cost: M
refs: [k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs, bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint]
---


Found by INSTR's large-K lint lane (`instr/klint-large-k`), reading the
uncapped split of
`work/instr/k-lint-dev-probe-large-k-gate-fails-on-51730-margins-once-the-sweep-runs.md`.

## What

`scripts/k_probe_sweep.sh`'s dump half on main `9fe94da35` (2026-11-10),
linted by `tools/k-lint` as the nightly does. Three of the pose's
screens (`crates/geom-brep/src/curve_pair/pose.rs`) record margins that
are the band's own escalation, not a distance:

| row | site | recorded \|m\| | outcome |
|---|---|---|---|
| `curve_pair_pose_clear` | `Polish::closest` (~885), `clear − reach` | `Kε − δ` or exactly `Kε` | in band, or negative |
| `curve_pair_pose_parallel` | `route` (~980), `offset − rate·(length/2 + e)` | `Kε − δ`, exactly `Kε`, and `0.342·Kε` on lily_walls | in band, or negative |
| `curve_pair_pose_tilt` | `route` (~987), `tilt` levered by `length + 2e` | exactly `2Kε` | positive |

`δ` is rounding, from 7e-18 m to 6e-17 m. Rule 1 (in band), per ε:

| ε | `_parallel` | `_clear` | shapes |
|---|--:|--:|---|
| 1e-6 | 195 | 84 | demo/lily, demo/lily_walls, demo/projectbox_cutaway, corpus/axis_projection |
| 1e-9 | 234 | 84 | the same, plus corpus/revolves_on_one_axis |
| 1e-12 | 0 | 0 | the lily is not recorded there (pinned stem stop) |

The definite twins at exactly `Kε` and `2Kε` flag rules 2 and 3 at
every row: 623 rows at 1e-6, 584 at 1e-9 and 392 at 1e-12.

**The same sample changes rule from one ε to the next by its last
bits.** `corpus/axis_projection`'s `_parallel` lands in band on 13
rows and definite on 40 at 1e-6. At 1e-9 the split is 40 and 13, and
at 1e-12 all 53 are definite.

## Reading (not yet the owner's)

When the pair's geometric term is zero (no clearance, zero offset, zero
overlap length), each margin is the pad `e = Kε` it subtracts, or the
lever `2e` it reads. A pad of exactly `Kε` puts the recorded margin on
the band's edge, so rounding decides between `indeterminate` and
`negative`. None of the three screens refuses: an undecided `_clear`
goes on to the polish, and an undecided `_parallel` goes on to the gap
and tilt arms. So rule 1 is reading screens that fall through, and
rules 2 and 3 are reading ε-coupled margins. This is the
`bool_pair_gap` shape (`work/instr/bool-pair-gap-is-eps-coupled-and-flags-74-rows-per-eps-under-k-lint.md`),
where the margin is `2(K+2)·ε`.

## Owed

Say whether a screen whose margin is its own pad when the geometry
term is zero should record that margin, and whether the pad belongs at
the band's edge. If the margin is right as recorded, say so on the
`bool_pair_gap` row: these three names then join its rule (4) ruling.
The in-band rows still need an answer here, because rule 1 is not
demotable and no threshold reaches it.
