---
id: shell-clearance-gate-reads-operand-footprints-not-offset-footprints
kind: issue
title: wall_clearance tests the OPERAND's footprints, but an inward offset grows past a concave edge by t: an S-bend column (pre-existing) and two diagonally offset voids (common after SHELL-5) shell silently with crossing twins
status: closed
opened: 2026-09-08
closed: 2026-09-08
---


Found by SHELL-5's R1 review (PR 2159), by execution
(`crates/sweep/tests/shell5_r1_probes.rs`, the two `r1p3_*` rows;
branch `shell/5-r1-probes`).

`wall_clearance` (`crates/topo/src/shell.rs`, `footprints_may_overlap`)
skips an antiparallel non-adjacent planar pair whose projected boxes
are definitely separated — read off the OPERAND. But the inward offset
of a face EXTENDS past each of its concave edges by `t` (the offset
planes meet further out), so two faces whose operand footprints are
disjoint by `δ < 2t` across a gap `g < 2t` have offsets that cross,
and no gate reads them. Every edge of a box void is concave from the
material's side, so after SHELL-5 the class is reachable on any two
voids offset diagonally.

Measured:

- **Pre-existing, outer shell.** The S-bend prism
  `(0,0),(1,0),(1,0.2),(2,0.2),(2,0.5),(0.8,0.5),(0.8,0.3),(0,0.3)`
  extruded 1, at `t = 0.12`: the two risers (x = 1, x = 0.8; gap 0.2,
  footprints disjoint by 0.1 in y) and the two shelves (y = 0.3,
  y = 0.2; gap 0.1, disjoint by 0.2 in x) all cross; builds, tier 3
  green, riser twins at x = 0.88 (from x = 1) and x = 0.92 (from
  x = 0.8). Identical at the merge base `c39a904e`.
- **Made common by SHELL-5.** A `6×4×4` box with voids
  `[1,2.2]×[1,2]×[1,3]` and `[2.4,3.6]×[2.1,3.1]×[1,3]` (facing walls
  0.2 apart, footprints disjoint by 0.1 in y) at `t = 0.15`: builds
  3 solids, tier 3 green, void 1's dilated wall at x = 2.35 and void
  2's at x = 2.25 overlapping in y on [1.95, 2.15]; the reported
  volume is the sum of the three walls with the crossing counted twice.

The module docs' "that is what makes the carried evidence sound on
planar operands" therefore overstates the planar gate: it is sound for
faces whose footprints overlap in the operand, not for the diagonal
class. A gate that grows each footprint by `t` before the separation
test (or reads the moved clone's footprints after the offset, which is
SHELL-4's certificate one dimension down) closes it; the probe rows
above are pinned to the current silent build and go red when it does.

## Closed

Closed in PR 2159's fix pass (2026-09-08). `footprints_may_overlap`
(`crates/topo/src/shell.rs`) now grows each face's projected footprint
box by `t` on every side before the separation decide — the footprint
an inward offset has past a concave edge, and an over-read at a convex
one (#571 direction). Rows: `shell5_r1_probes::r1p3_diagonal_voids_refuse_at_the_grown_footprint_gate`
and `r1p3_outer_shell_s_bend_refuses_above_the_wall_and_builds_below_it`
(the S-bend at `t = 0.09` still builds, twins uncrossed — the growth is
exactly `t`, not a box inflation), `shell5_r2_probes::r2_diagonal_voids_refuse_at_the_grown_footprint_gate`
and `r2_the_same_gate_hole_is_closed_on_a_single_shell_notched_operand`
(which also pins the notched body's closed form at `t = 0.1`). Every
existing shell fixture and the tour's shelled scenes still build.
