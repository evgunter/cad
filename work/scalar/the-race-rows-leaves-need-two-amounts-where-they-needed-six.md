---
id: the-race-rows-leaves-need-two-amounts-where-they-needed-six
kind: issue
title: The memo race row's chamber lost its far branch to the positive-depth rule, so its leaves need two distinct amounts where the reviewer's needed six
status: open
opened: 2026-10-02
priority: P3
cost: M
---


`m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` drives `m10_3_r1_probes_interval::bounded_chamber(60ε, 30ε, 100ε)`: two extrudes, depths `q` and `c − q`. While a negative depth built on the other branch, every leaf replayed a body and the reviewer's 12-leaf race needed six distinct amounts of the frozen set (measured at the merge base). Since an extrude's depth is a size (Ev, #3551; `work/recipe/extrude-distance-is-a-depth-and-a-side.md`), the box's two ends refuse `NegativeDepth`, the leaves there freeze nothing new, and the needs collapse to two or three values (`[2110, 866 ×6, 2110]` on the 8-leaf race, three values `834`, `1619`, `1401` on the 12-leaf one, at the default ε).

The row's non-vacuity floor asked for three distinct needs on every arm; RECIPE's PR made the floor per arm, two on the 8-leaf race (which is what the row's own words guard, "a column that collapsed to one value for every leaf") and three on the other two. What was lost is the richer witness. Re-planting the chamber on a profile vertex through collinear (built on both sides) was tried and gives two classes as well, and moves `m10_sym_profile_interval`'s per-ε form pins, which share the document, so it was not taken.

The fix is a chamber whose leaves need visibly different amounts again — a document with more than two independently flipping nodes, or a far branch that builds — re-pinning `m10_sym_profile_interval` with it, and the 8-leaf race's floor back at three.

## Re-homed from FLUX to SCALAR (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. SCALAR collects the scalar doors and the readers over them: the certified-door family, the margin and recourse readers, the unit-direction doors, and the box driver's readings. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## The faces' lever collapsed the 8-leaf race to one value (2026-10-15)

(TANG, U-lever, PR 5204) Since an edge's wedge is levered at its faces' extent (`topo::wedge_reach`), every leaf's replay reads every face's reach, and on the slab a refused leaf's need no longer depends on which door refused it: the 8-leaf race reads `[2419 ×8]`, the 12-leaf one `[3298, 1870 ×11]` (and now certifies that one leaf), the certifying drive `[237 ×4, 120 ×60]`, identical at ε 1e-6, 1e-9 and 1e-12. No budget measured (8 leaves at 4/2 through 128/24) gives the 8-leaf race a second value, and every slab drive that shows two certifies. The row's floors are now one on the 8-leaf race and two on the others, and the 12-leaf race says it certifies. The fix above stands, with "more than two independently flipping nodes" now needing nodes whose reach reads differ per leaf.

## A negative depth builds again (DOCTAIL, 2026-11-18)

(DOCTAIL, `work/doctail/a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face.md`) An extrude's depth is signed now (DESIGN.md `[placement]`, Ev's ruling on `[ev]` #7073): `wire_extrude` answers the sweep's `NegativeDepth` verdict by sweeping `|d|` and lifting the body `d` along the profile's `z`. So the chamber's two ends no longer refuse `NegativeDepth`; they build a body on the far branch, which is the second fix this row names. Only the coincidence zone and the band about zero still refuse. The race's needs were not re-measured in that PR. Whether the faces' lever (the section above) still collapses them, and whether the floors can go back up, is what this row now asks.

## The 8-leaf race now certifies, red on main (ONEHOME, 2026-11-19)

(ONEHOME's PR 7088 lane, a local run) Since the signed depth, the 8-leaf race certifies a leaf. On main at `8e7e7b5f4` the row is red at `m10_sym_drive_memo_interval.rs:539`. The arm `race` expects `certifies: false`, and its drive reads `Receipt { certified: 1, refused: 7, splits: 7 }`, with leaves' need `[36 ×8]` and frozen 36. The receipt is identical on PR 7088's merge of that main (`5b8bf47eb`) and with that PR's placement asks switched off. What re-pinning it means is this row's question: whether the arm's `certifies` flag moves to `true`, and whether its distinct-needs floor still has anything to read when every leaf needs 36.

## The same red at every ε (GREEN, 2026-11-22)

(GREEN U3, `green/editor-1e-12`) On the scheduled nightly 44870651337 (main `f5ef8f2b`) this row is red at the default ε, 1e-6 and 1e-12 with one message, `race: the row's certified arm is vacuous unless this drive certifies: Receipt { certified: 1, refused: 7, splits: 7 }` at `m10_sym_drive_memo_interval.rs:539`. So the re-pin this row owes greens the 1e-6 and 1e-12 rows too, and GREEN's 1e-12 row defers to it.

## Not PR 7434's; every slab drive certifies more (DOCTAIL, 2026-11-23)

(DOCTAIL, `work/doctail/main-red-the-identity-turns-decisions-moved-the-m10-slow-rows.md`) The row was reported as moved by PR 7434 (the least turn's identity shortcut removed). It was not. The receipt is `Receipt { certified: 1, refused: 7, splits: 7 }` on 7434's first parent `fa42c60` and on `787aed5`. It is the same with 7434's geom-core files restored on main, and with the Rodrigues arm's former spelling. It is the same at the default ε, 1e-6 and 1e-12.

This is a gain in certification, not a loss. With the `race` arm's `certifies` set to `true`, the whole row passes on main. Every leaf reports one column under both parallel pools, the sequential drive and both memo dials, and the certified lists agree. All three drives now certify more than this row's text says:

- the 8-leaf race certifies one leaf, `[-1.25e-8, 0]`, where the text says nothing;
- the 12-leaf race certifies two, `[-1.25e-8, 0]` and `[0, 1.25e-8]`, where the text says one;
- the 64-leaf drive certifies eight, symmetric about 0, where the text says four.

The certified leaves are the ones beside the variable's zero (why there was not traced). Each needs `[36 ×8]` on the 8-leaf race, and `203` against the refused leaves' `139` on the other two. So the `race` arm's distinct-needs floor of one still holds, and the other two arms' floor of two holds. The re-pin is `certifies: true` on `race`, plus the counts in `RACING`'s doc. Measured at the default ε, the row re-pinned that way is green on main. Taking the re-pin is this row's call.
