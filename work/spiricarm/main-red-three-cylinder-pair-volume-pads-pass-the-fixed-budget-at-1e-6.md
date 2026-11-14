---
id: main-red-three-cylinder-pair-volume-pads-pass-the-fixed-budget-at-1e-6
kind: issue
title: Main red at eps 1e-6: three cylinder x cylinder rows hold their closed form in an enclosure wider than the fixed 1e-7 pad budget (verbs_germarms2 x2, verbs_cylcyl_r1_review_probes)
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by GREEN's ε 1e-6 lane (`green/eps-1e-6`), from nightly 43371638371 (main `e36675c1`). Red at ε 1e-6 only.

**What fails** (`CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --test all -E 'test(=<name>)'`, on main `b363f5fa1`):

| row (`sweep::all`) | first message |
|---|---|
| `verbs_germarms2::a_rod_through_a_drum_wall_builds_at_its_sliced_volume` | `drum ∪ rod: OK BAD t2=true t3p=true cert=true operand=true v=1.667873349 pad=3.7e-7 want=1.667873349` |
| `verbs_germarms2::the_fenced_poses_keep_their_own_outcomes` | `unequal radii, direct: OK BAD t2=true t3p=true cert=true operand=true v=14.935301415 pad=7.6e-6 want=14.935301415` |
| `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly` | `perpendicular unequal radii/union: OK BAD t2=true t3p=true cert=true operand=true v=81.651534547 pad=6.7e-6 want=81.651534544` |

No volume is wrong: each body passes its four checks and its enclosure holds the closed form. The line reads `BAD` because the enclosure's half-width is over `VOLUME_PAD_BUDGET = 1e-7` (`crates/sweep/tests/common/differential.rs`, `Budget::Fixed`, through `outcome`), a fixed number at every ε. Each row stops at its first `BAD` line, so its other poses and ops are unread at 1e-6.

**When.** The nightly's 1e-6 row passed the second and third on 11-03 (main `2d9dc809`) and failed all three on 11-04 (main `63cbd7c3`); the first is not in the 11-03 run and was red the first night it ran. The pads have widened since: `1.6e-7`, `3.8e-6`, `3.4e-6` on 11-04.

**Same shape as a closed row.** `main-red-two-cylinder-sphere-volume-pads-pass-their-ceiling-at-1e-6-since-the-sphere-lane` (PR 6840) found the cylinder × sphere rows holding an ε-scaled pad to a fixed `1e-7`, and moved them to the resolution ceiling with measured guards. `cylinder-sphere-pad-ceilings-speak-four-vocabularies` is the class.

**Owed.** Decide which side is wrong, as that row did: if the rotational lane's pad on these chorded walls is ε-scaled by construction, the three rows read a ceiling stated from the resolution (`judged_at_resolution`) with a measured guard; if not, the pad is too wide. Do not move the budget to fit the number. Say why the pads doubled between 11-04 and 11-13.

**Pinned** (`green/eps-1e-6`): each row is under `test_utils::pinned_red::row` at ε 1e-6, on its message less the pad's digits. Take the pins off with the fix.
