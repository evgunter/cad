---
id: full-turn-bore-rest-mate-does-not-union
kind: issue
title: A declared cylindrical Rest on a full-turn bore does not union - an on-carrier ruling crossing a rim's interior is recorded by nobody
status: open
opened: 2026-10-01
refs: [full-period-wall-has-no-containment-verdict]
---

Found by the `reach-fullperiod` lane. Giving the face door a verdict on
a full-turn cylinder wall (the full-turn band class) places every
endpoint on such a bore, but the minimal shaft-in-a-full-revolve-bore
mate still does not union, at two doors further on.

## Fixture

Collar: the rectangle `ρ ∈ [0.5, 1.5]`, `y ∈ [1, 2]` revolved a full
turn about `y` (`sweep::revolve`, `Revolution::Full`) — its bore is ONE
full-turn face. Peg: `mate2_common::peg(z0, h)` (three 120° arcs,
radius 0.5) turned `-π/2` about `x` so its axis is `y`. Declarations:
`mate2_common::wall_decls` (3 pairs, the bore against each peg wall).

## Measured (same on `origin/main` and after the band class)

- **Partial engagement** (`peg(0.5, 2.0)`):
  `CurvedPierceUnsupported { operand: B, face: <collar bore>, edge: <peg seam ruling> }`.
  The ruling spans `y ∈ [0.5, 2.5]`, through the bore's whole height
  window. Both endpoints are on the shared carrier and the door now
  places both `Out` (past each rim), so the declared-cover rung's
  `(Zero, Zero)` arm records nothing and keeps the door
  (`Placement::records_the_pair`). The ruling's crossings with the
  bore's two rim CIRCLES are interior to both edges: nobody records
  them. On `origin/main` the same mate refuses at the peg's
  `EdgeKey(4v1)`; with the band class, at `EdgeKey(5v1)`.
- **Full engagement** (`peg(1.0, 1.0)`): the reduction passes and the
  join refuses `Join(UnpairedLooseEnds { count: 12 })`.

The partial case is the same class as
`crates/sweep/tests/mate2_r1_probes.rs`'s
`probe_misaligned_azimuth_split_reports_its_outcome` (a rim arc
crossing the partner's seam ruling in its interior, "recorded by
NOBODY"): on a full-turn bore every rim is one circle with its vertex
at the collar's seam, so every partner ruling not at that azimuth is
misaligned.

## Evidence (2026-10-01, `reach-snowman`): lily wall 12 now stops here

With a line × sphere root lane in the crossing layer, lily wall probe 12
(corm ∪ stem foot, declared cylindrical `Rest`) gets past the corm's
sphere zone and stops at this row's door:
`CurvedPierceUnsupported { operand: B, face: FaceKey(3v1), edge: EdgeKey(5v1) }`.
Measured at `curved_face_arm`: edge `5v1` is the foot's seam ruling at
azimuth 120°, `(-0.03, 0.052, z)`, `z ∈ [-0.92, 0]`; face `3v1` is the
corm's bore wall (axis `-z`, radius 0.06); the pair is declared-covered,
both residuals are zero, and both ends lie past the bore's height
window, so the `(Zero, Zero) if covered` arm records nothing and keeps
the door — the partial-engagement case above, on the plant.
