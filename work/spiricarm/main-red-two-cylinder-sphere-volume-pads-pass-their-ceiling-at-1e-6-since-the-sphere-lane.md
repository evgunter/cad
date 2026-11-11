---
id: main-red-two-cylinder-sphere-volume-pads-pass-their-ceiling-at-1e-6-since-the-sphere-lane
kind: issue
title: Main red at eps 1e-6: since PR 6509, two cylinder x sphere volume pads pass their 1e-7 ceiling (a_ringed_wall, verbs_cylsph_opening)
status: dispatched
branch: spiricarm/sphere-pad-ceiling
opened: 2026-11-11
priority: P1
cost: M
---

Found by PCERT's nightly-reds lane (`pcert/nightly-slowset-reds`), in nightly 42965378060 (main `b2916558d9`), at ε 1e-6 only.

| row | message |
|---|---|
| `sweep::all a_ringed_wall_divided_by_a_carried_chord::a_wall_holding_one_balls_ring_is_divided_by_the_others_chord` | `A ∪ B: t2=true t3p=true cert=true operand=AtTheSphere v=1.642414632012235 pad=1.0257e-7 want=1.6424146317032864 ceiling=1e-7` (and `A ∖ B` alike, pad 1.0247e-7) |
| `sweep::all verbs_cylsph_opening::a_transversal_pose_builds_in_both_poses` | `direct: … operand=AtTheSphere v=19.64403305320636 pad=5.29e-6 want=19.644033044270536 ceiling=1e-7` |

**Bisected, first-parent, at ε 1e-6:**
- `a_ringed_wall`'s row passes at `fc00197c` (PR 6518's merge; `343823b7`, PR 6803, adds only a work file). It fails at `e2435632`, the merge of PR 6509 (`spiricarm/rotational-lane-sphere`), at `62884aa1`, and on main `019bed88`.
- `verbs_cylsph_opening::a_transversal_pose_builds_in_both_poses` arrived in PR 6509 (`fb09d050`, "cylinder x sphere rows move to their next stop: built at their volume …"). It has been red at ε 1e-6 since it arrived, at `e2435632` and on main.

Both bodies build, certify and read their volume within `want` (the values agree to 1e-9 and 1e-8). What fails is the volume pad against a 1e-7 ceiling. At ε 1e-6 the sphere's rotational lane pads by 1e-7 to 5e-6, so the pad seems to scale with ε. A ceiling of 1e-7 is not one an ε-1e-6 pad can meet.

Decide which side is wrong. If the lane's pad is ε-scaled by construction, the rows' ceiling should scale with the row's ε, and that should be argued from the lane's pad term. If not, the pad is too wide. Do not move the ceiling to fit the number.
