---
id: reverting-a-slit-body-overlaps-its-coincident-edges-wedges-and-passes-every-tier
kind: issue
title: Body::revert of a body with coincident distinct edges on one line yields overlapping material wedges, and every tier passes it
status: open
opened: 2026-10-14
priority: P1
cost: M
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

Landed: the minimal sound form, not the re-pairing.
- `Body::revert` returns `Result<Self, RevertError>` and refuses
  `RevertError::CoincidentEdges { edges, ends }` where two distinct
  edges run between the same two point keys on two lines or one stored
  curve.
- ∖ refuses `BooleanError::Revert { operand: B, source }`: the seamed
  finish reads the input's coincident edges through the reduction's
  split lineage (the slit cut across has its pieces' ends on new point
  keys); the single-operand fallback and the void door
  (`VoidInsertError::Revert`) refuse through `revert`.
- The seamed finish skips a pair the classification read `On` the
  other operand's boundary: there the join pairs the sheets with that
  operand's faces.
- The pinch (two bricks touching along a line, its ends on shared point
  keys since PR 4793) is the same class. On main, `B − pinch` in
  `pierce_strut_at_a_pinch` (bare and reflex) and `cutter ∖ pinch` in
  `union_flush_onto_edge_contact` answered with each brick's own two
  faces still paired on the line (measured: each result edge on the
  z-axis bounds faces centred in one quadrant), the complement's
  wedges overlapping, and those rows pinned it. They now pin
  `BooleanError::Revert`.
- Pins: `crates/topo/tests/a_doubled_line_does_not_revert.rs` at every
  pose (default, 1e-6, 1e-12 measured locally). A single cusp still
  reverts to the legal knife slit
  (`tier3_tests::revert_maps_the_legal_cusp_to_the_legal_slit`).
- The re-pairing is filed as
  `revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet`.
