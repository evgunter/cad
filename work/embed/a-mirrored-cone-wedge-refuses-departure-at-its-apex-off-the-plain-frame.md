---
id: a-mirrored-cone-wedge-refuses-departure-at-its-apex-off-the-plain-frame
kind: issue
title: A cone wedge revolved by a negative angle past a half turn refuses Departure at its wedge plane and cone off the plain frame, at one scalar or both, where the same wedge by the positive angle is embedded in every pose
status: open
opened: 2026-11-22
priority: P1
cost: M
---


## Finding

Filed by lane `embed/apex-separator` (2026-11-22, the PR 7315 fix pass),
on its head after merging main `7669fcfdd` (PRs 7120 and 7321).

The triangle with a leg on the axis, revolved about `y` through the
verdict door (`topo::clearance::embedded`, the default budget). By `+θ`
every pose below is `Embedded` at both scalars, and
`self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
pins it. By `−θ` only the plain frame is:

| `−θ`, pose | f64 | `Interval` |
|---|---|---|
| −4.7224, plain | Embedded | Embedded |
| −4.7224, turned (axis (1,2,3), 0.7) | Embedded | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` |
| −4.7224, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −4.7224, moved (640, −570, 520) | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` (381 discharged) | `Budget(Pairs)` |
| −4.7224, a hundred times the size | `Uncertified`, the same pair | `Budget(Pairs)` |
| −4.7224, a needle (r 0.05) | `Uncertified`, the same pair | Embedded |
| −4.7224, flat (h 0.05) | `Uncertified`, the same pair | Embedded |
| −5.5, plain | Embedded | Embedded |
| −5.5, turned | `Budget(Depth)` | `Budget(Depth)` |
| −5.5, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −5.5, moved (640, −570, 520) | Embedded | `Budget(Pairs)` |
| −5.5, a hundred times | `Uncertified`, the same pair | `Budget(Pairs)` |
| −5.5, a needle | `Uncertified`, the same pair | Embedded |
| −5.5, flat | Embedded | Embedded |

The pair is the wedge's plane `1v1` (the start plane, normal ≈ x) and
the cone `4v1`, across the ruling `6v1`, near the apex vertex `3v1`.
Several poses read one class at f64 and another at `Interval`; that is
part of the finding, not tuned away.

**Traced** (−4.7224 moved (640, −570, 520), f64, on PR 7315's head with
prints, 2026-11-22; nothing committed). It is the depth budget, not a
reading of the plane through the apex: all 8 refused splits are
`CellBudget::Depth` at depth 40, plane `1v1` (and `2v1`) against cone
`4v1`, the cone's cells `u ∈ (4.7001, 4.7124)`,
`v ∈ (−1.38e-3, 1.6e-13)`, reaching the apex vertex `3v1`. There the
cone is not presented at all: `present_chart` returns `Local` at
`sides_are_the_faces` (the cover), because the shared ruling `6v1`'s
image on `XLo` (`u = 4.70239`) covers `v ∈ [−1.414213562373102,
−7.105427357601002e-15]` and stops `7.1e-15` short of the apex, the
rounding of its stored end 640 units out, so it does not cover the
cell's side `(−1.38e-3, 0)`. No separator is tried on those cells, the
plane through the apex included.

**It is PR 7115's case**: that PR runs an edge whose image ends an ulp
short of a collapse on to the collapsed side (`ends_on_collapse`).
Measured on PR 7115's head `e88481264`: with PR 7315's `injective.rs`
applied the pose is `Embedded` (173 discharged); with PR 7115's head
alone it refuses `Departure` on `1v1`/`4v1` at `6v1` (260 discharged).
So the mirrored poses need both: PR 7115 to present the cone at the
apex, then PR 7315's plane through the apex to sign it. The review of
PR 7315 found no false certificate across 960 mirrored and re-charted
variants of the hand-built wedge
(`injective::presentation_rows::mirrored_and_recharted_wedges_never_certify_a_meeting`).

**Unchecked:** only this pose is traced; the table's other mirrored
poses are taken to be the same shape, not measured. Re-measure them and
pin the holding ones once PR 7115 lands.
