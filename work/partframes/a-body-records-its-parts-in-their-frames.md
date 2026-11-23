---
id: a-body-records-its-parts-in-their-frames
kind: issue
title: Stage 2 of fork 288: the body gets a frame table and geometry reads go through one aligning door
status: open
opened: 2026-11-18
priority: P2
refs: [7079, 7099, the-graft-holds-a-placed-copy-twice, vertex-on-edge-is-answered-from-the-claim-not-from-coordinates]
cost: X
points: 48
design: true
---

Filed by the ONEHOME orchestrator: stage 2 of fork 288 (PR 7079,
approved 2026-11-18). `[placement.frame]` now reads "a body records
each of its parts in the frame it was built in, and that record is
the part's data".

**Precondition (Ev's, in PR 7079).** Stage 1 lands first: every
kernel door over two bodies takes placed operands, and one aggregate
door (PR 7099 is its first). Then the whole-Boolean numbers at boxed
motions go back to Ev before this starts. The cross-placement probe
(`onehome/cross-placement-probe`, not merged) could only show the
readings; it showed carrier readings bit-identical, classification
8/8 against 0/8 at wide boxes, and the whole Boolean refusing first
in data the up-front move wrote.

**The unit.** The body gets a frame table (a tree; relative motions
through the nearest shared frame). About 630–860 geometry reads move
behind one aligning door, module by module, behind a temporary
accessor. A Boolean's result keeps each operand's survivors bit for
bit under its own frame. Estimated 9–30 PRs depending on staging.

## The whole-Boolean numbers after stage 1

Stage 1 landed: PR 7088 (pairwise readers through `topo::Seat`), PR 7099
(the one aggregate door, `graft_placed`) and PR 7206 (the Boolean's
doors take placed operands). The cross-placement probe of PR 7079
(`onehome/cross-placement-probe`, `35c730475`) was rerun end to end
through the placed door, placed against moved, over 2 scalars × 3 cases
× 10 boxed motions (turns 30° ± 1e-8°…1°, shifts ± 1.7e-10…1.7e-2 m).
The full table, with each row's first refusal, its escalations and both
times, is in PR 7206's body.

- Placed and moved agree on all 60 rows in outcome, result bits and
  escalations. Stage 1's door still moves the operand up front, inside
  itself, so this holds by construction.
- 7 of 60 build:

  | scalar | turned disc ∪ clear copy | base − placed pocket | stacked on the top cap |
  |---|---|---|---|
  | `Interval` | 0 / 10 | 0 / 10 | 1 / 10 (turn ± 1e-8°) |
  | `Sym<Interval>` | 1 / 10 (turn ± 1e-8°) | 0 / 10 | 5 / 10 (every turn) |

- The smallest boxes already refuse: a shift of ± 1.7e-10 m refuses in
  every case at both scalars.
- The first refusals sit in readings of data the up-front move wrote:
  carrier endpoints (`carrier_endpoint_start`/`_end`), a pcurve's chart
  incidence (`pcurve_cylinder_chart_incident`, `pcurve_envelope`), a
  vertex's face side (`bool_vertex_face_side`), and at `Sym` the result's
  junction and volume (`props_chart_junction_closed`,
  `props_cylinder_circle_off_axis`).

## Moved to PARTFRAMES (2026-11-21)

Moved from `work/onehome/` by `git mv`, id and body unchanged, when Ev chose to split stage 2 off as its own program (PR 7282, "splitting it off sounds good!"). Priced `X`, 48 points: the 9–30 PR estimate above, read as about six hard rows' work, since most of the 630–860 reads move behind the door mechanically, module by module, and the hard part is the door, the frame table and the result's survivors. `design: true`: how the door is staged, and whether the fused aggregate body is a cache or goes (`the-graft-holds-a-placed-copy-twice`), are open until stage 2 is planned.
