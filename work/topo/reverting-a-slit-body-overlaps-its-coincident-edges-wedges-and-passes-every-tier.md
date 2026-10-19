---
id: reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier
kind: issue
title: Body::revert of a body with coincident distinct edges on one line yields overlapping material wedges, and every tier passes it
status: closed
opened: 2026-10-14
priority: P1
cost: M
branch: cleave/revert-repairs-coincident-edges
closed: 2026-10-15
---

Found by the radial-edges design fork on CLEAVE's `[ev]` PR 4795, by
reading only; not yet measured.

## What

Where more than two face sheets meet on a line, the material forces
which sheets pair into edges: one edge per creased material sector.
Complementing the material **re-pairs** the sheets. It is not a sign
flip. `Body::revert` (`crates/topo/src/revert.rs`) flips every face's
side and re-pairs nothing.

Take a slit body: the split's one-end trench, PR 4622's form, already
on main, or the island ring that 4795 rules for. Its two coincident
edges a (floor | wall a) and b (wall b | floor) each bound a ~73°
wedge. After revert, each bounds ~287°. The two wedges overlap, so
the line's material is counted twice.

- Tier 3's check 4 reads one edge at a time.
- The census licenses the coincident pair with no record, because
  its bounds are shared vertices.

So the reverted body passes every tier. It is reachable today:
`subtract` reverts its tool's kept part (`boolean/finish.rs`
`b_kept.revert()`), and the split's slit body is a legal operand.

D1's "wedge 0 and wedge 2π … legal together or not at all" argues
from revert, and holds only for a single cusp. The revert of two
kissing cusps is two seams, because the sheets re-pair.

## Owed

1. Measure. Revert the 4622 one-end slit body and validate it. Then
   subtract a slit-bearing tool, and validate that result too.
2. Re-pair at revert. Where coincident edges share a line, revert
   re-pairs the sheets by the new material sectors. Until that is
   built, refuse it typed on an operand whose coincident edges say
   such a line exists.
3. The tier-3′ side check over every group of coincident edges on one
   line: their material sectors must be disjoint. Its home is
   restread's `census-passes-a-face-inside-an-edges-material-wedge`.
   Without the check, this body passes the gate.

## Built (branch cleave/revert-repairs-coincident-edges)

Measured on `origin/main` `ebf6c05239` at ε = default, 1e-6 and 1e-12
(the same at each):
- The split's one-end slit body passes tiers 2, 3 and 3′. Its keel is
  two edges from (1.5, 1, 1) to two vertices at (4, 1, 1) on one point
  key, each a `Line` on its own curve key.
- `revert` of it: tier 2 passes, tiers 3 and 3′ fail only
  `NegativeVolume` (a reverted body bounds the complement); the wedge
  check reads each 286.7° wedge alone and passes it.
- `subtract(slab [−1,5]×[−2,4]×[0,1.5], slit body)`: one solid, no
  records, volume 46.1875 (the closed form), tiers 3 and 3′ pass. The
  floor carries the two overlapping 286.7° edges. Confirmed. The same
  with the slab ending at `x = 3` (the slit cut across): 30.1125, every
  tier passes.

Landed: refusals, not the re-pairing.
- `Body::revert` refuses `RevertError::CoincidentEdges` structurally,
  deciding nothing: two line edges on the same two point keys
  (`census::shared_segments`). The void door and ∖'s fallbacks (B
  whole, or a cavity nothing joins) refuse where the operand holds
  such a line; the fallbacks first read every collinear overlap.
- ∖'s seamed finish reads the collinear overlaps among B's edges that
  the result keeps (through the split lineage), and refuses
  `BooleanError::Revert` (`RevertError::WedgesOverlap`) only where the
  result's material wedges on the line overlap (`census::line_wedges`,
  membership by tier 3's `validate::wedge_holds`). A reading in band
  escalates as an input reading, `BooleanDecision::LineEdges` or
  `LineWedges`, with its lever. Two edges whose parallel reading is in
  band but whose lines stand definitely apart are not read as a
  near-coincidence: the prism with near-parallel diagonals 0.7 apart
  builds, as on main (`a_tool_with_near_parallel_edges_standing_apart_builds`). Where
  the other operand's faces meet the line the join can re-pair the
  sheets, and two rows show it does: `a_vertex_crossing_both_sides_of_a_pinch`
  (`cutter ∖ pinch`, one 90° edge) and the staircase's `P − X` (two
  disjoint 90° edges) build as on main.
- Pinned refused: the slit at every pose, the bare and reflex pinch
  (`B − pinch`), the unequal-heights pinch, and nine
  `union_flush_onto_edge_contact` rows whose main answers overlapped
  (90°, 13.6° or more). The table is in PR 5106's body.
- The re-pairing is filed as
  `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`;
  the same overlaps built by the join from inputs that hold no line,
  as CLEAVE's P1
  `boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`.
