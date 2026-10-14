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
