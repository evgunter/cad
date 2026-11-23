---
id: a-side-the-join-could-split-has-a-piece-that-would-fold-or-fall-under-winding-resolution
kind: issue
title: A side the join could split has a piece that would fold or fall under winding resolution
status: open
priority: P3
opened: 2026-11-23
---


Where two patches both have a side inside, the join splits it in each, and a piece that folds against its face or falls under winding resolution refuses that face, typed (`mesh::join::split`; `crates/mesh/README.md` `[mesh.winding]`). Nothing measured reaches that refusal: every side the join actually split over the `mesh` suite at ε 1e-9, 1e-6 and 1e-12 and over `sweep`'s meshing modules has pieces that certify, wind and do not fold. But the split is not safe on every side by construction.

Measured over every side a lane could be asked to split (every side between two shared ids that two of the patch's triangles have, 2.8 M over the `mesh` suite's 13,697 meshes at the default ε, whether or not any other patch makes it):

| lane, surface | sides | a piece folds | a piece under winding resolution |
|---|---|---|---|
| planar | 1,636,310 | 8 | 9,937 |
| walked cone (development's midpoint; chart midpoint) | 41,863 | 685 (1,675) | 0 |
| rectangle cone (development's; chart) | 162,090 | 10 (57) | 0 |
| trimmed cone (development's; chart) | 227 | 0 (6) | 0 |
| trimmed sphere | 1,849 | 297 | 0 |
| rectangle torus | 32,405 | 32 | 0 |
| rectangle cylinder | 956,971 | 6 | 0 |

No piece exceeds or leaves unresolved its certificate. A planar piece under resolution is half of a thin triangle (`validate::resolved_normal` holds the height against an allowance that does not halve); a curved fold is a wide side whose split point stands far off the straight side, so one piece turns over (most also turn against the parent triangle).

**What would make one of these refuse:** another patch having the same side inside, which takes a face across holding both of its ends and making the same straight side in its own triangulation: two faces sharing two chains with no point of either between them (a shell thinner than δ, a sliver), or an ear of one chain against a face that lays no point near it (a plane cap). Each such side above sits in a patch whose face across does not make it, in the bodies measured.

**What to do when one is met:** a refusal is the answer the unit decided (`work/tesslane/the-mesh-cures-a-side-two-faces-can-both-make-three-ways.md`); a body that meets it is the witness to weigh a second split point (a different chart point, or a split of the far side), not a reason to read the face across.

## 2026-11-23 — the split's point is the foot (the review of 24ee9a28fb)

Ruled: the split's point is the foot of the side's straight midpoint on every analytic surface, one rule in place of the chart midpoint, the cone's development and the singular-end rule (`join::point_at`); a spindle sheet and a spline face keep the chart midpoint. The table above, taken again on the branch's head (the shared-side rules retired, so the trimmed sphere's sides differ from stage 1's), each side split both ways, each piece read for a fold against the analytic outward normal at its centroid in every lane, at the default ε over the `mesh` suite:

| surface, lane | sides | a piece folds: chart midpoint (the development's on a cone) | a piece folds: the foot |
|---|---|---|---|
| cone, the walk and the trimmed lane, no apex end | 37,968 | 687 | 0 |
| cone, the rectangle lane, no apex end | 136,812 | 10 | 0 |
| cone, an apex end (both) | 26,493 | 0 | 0 |
| sphere, the trimmed lane, no pole end | 95 | 75 | 0 |
| sphere, the rectangle lane, no pole end | 3,494 | 8 | 0 |
| sphere, a pole end | 1,589 | 0 | 0 |
| torus, the rectangle lane | 13,622 | 46 | 16 |
| torus, the trimmed lane | 717 | 0 | 0 |
| cylinder, the rectangle lane | 961,197 | 6 | 6 (the same point) |
| cylinder, the trimmed lane | 12,423 | 0 | 0 |
| spindle sheet (no foot in closed form) | 44 | 0 | the chart midpoint's |

No piece falls under winding resolution under either rule. The torus's 16 and the cylinder's 6 stay what this row says: a piece that would fold refuses its face, typed, if a second patch ever makes the side (`join::tests::a_split_piece_that_folds_refuses_its_face` holds that through `split` on a torus face).

The rectangle lane's apex sides (listed above as splittable) now split: the rectangle lane holds the apex as one chart vertex per column, and an end at a singular point is one end whatever `u` its copies carry (`join::tests::a_side_from_a_cones_apex_splits_on_the_rectangle_lane`).
