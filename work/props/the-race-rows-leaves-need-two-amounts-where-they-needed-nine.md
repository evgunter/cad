---
id: the-race-rows-leaves-need-two-amounts-where-they-needed-nine
kind: issue
title: The memo race row's chamber lost its far branch to the positive-depth rule, so its leaves need two distinct amounts where the reviewer's needed nine
status: open
opened: 2026-10-02
priority: P3
cost: M
---


`m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` drives `m10_3_r1_probes_interval::bounded_chamber(60ε, 30ε, 100ε)`: two extrudes, depths `q` and `c − q`. While a negative depth built on the other branch, every leaf replayed a body and the reviewer's 12-leaf race needed nine distinct amounts of the frozen set. Since an extrude's depth is a size (Ev, #3551; `work/recipe/extrude-distance-is-a-depth-and-a-side.md`), the box's two ends refuse `NegativeDepth`, the leaves there freeze nothing new, and the needs collapse to two or three values (`[2110, 866 ×6, 2110]` on the 8-leaf race, `[834, 1619, 1401]` on the 12-leaf one, at the default ε).

The row's non-vacuity floor asked for three distinct needs; RECIPE's PR lowered it to two, which is what the row's own words guard ("a column that collapsed to one value for every leaf"). What was lost is the richer witness. Re-planting the chamber on a profile vertex through collinear (built on both sides) was tried and gives two classes as well, and moves `m10_sym_profile_interval`'s per-ε form pins, which share the document, so it was not taken.

The fix is a chamber whose leaves need visibly different amounts again — a document with more than two independently flipping nodes, or a far branch that builds — re-pinning `m10_sym_profile_interval` with it, and the floor back at three.
