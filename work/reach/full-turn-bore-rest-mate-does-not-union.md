---
id: full-turn-bore-rest-mate-does-not-union
kind: issue
title: A declared cylindrical Rest on a full-turn bore does not union - a ruling fragment with both ends past the bore keeps the frontier, the zip cannot pair one bore face with three shaft walls, and vtxfac has no curved pierced carrier
status: review
opened: 2026-10-01
refs: [full-period-wall-has-no-containment-verdict]
branch: reach/fullturn-bore-mate
pr: 3814
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
  `(Zero, Zero)` arm records nothing and keeps the door (the rule that
  is now `Placement::declared` with an unseen interior). The filer read
  the ruling's crossings with the bore's rim circles as recorded by
  nobody; measured since, the ruling also crosses the collar's flat caps
  there, and the planar sweep records those crossings (see the cause
  below). On `origin/main` the same mate refuses at the peg's
  `EdgeKey(4v1)`; with the band class, at `EdgeKey(5v1)`.
- **Full engagement** (`peg(1.0, 1.0)`): the reduction passes and the
  join refuses `Join(UnpairedLooseEnds { count: 12 })`.

The partial case was filed as the same class as `mate2_r1_probes.rs`'s
misaligned-azimuth probe (now `probe_misaligned_azimuth_split_unions`):
a rim arc crossing the partner's seam ruling in its interior. That probe
too unions with the crossing recorded at the collar's cap.

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

## The cause, as measured (PR 3814, after its dual review)

The title's reading was not the binding one. A shaft ruling through the
one-face bore does cross the bore's rims between its ends, but each rim
also bounds the collar's flat cap, which the ruling crosses
transversally — the planar sweep splits the ruling there and records the
crossing (forcing the new crossing layer to answer "clear" leaves every
one-face-bore row green). What refused the partial mate was the
declared-cover rule's nothing-recorded guard: a ruling fragment with both
ends past the face's window (`Elsewhere`) kept the frontier door, though
nothing of it lies on the face. The fix reads that pair as no event, on
the certificate that the fragment's interior meets the face's boundary
nowhere — and that certificate is the crossing layer, whose only sighting
is a boundary curve with the shared carrier on both sides (a bore split
by a circle, `full_turn_bore_mate::a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth`,
which refuses `UnpairedLooseEnds` without it). The flush mate's door was
the REST zip's patch pairing (one bore face against three shaft walls),
and an off-seam azimuth's was `vtxfac`'s coplanar lump on a curved
pierced face.
