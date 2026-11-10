---
id: dome-clearance-screen-escalates-an-invalid-margin
kind: issue
title: sweep: the dome's equator at r >= 0.6 escalates fillet3_face_clearance with MarginDiag::Invalid
status: closed
opened: 2026-09-13
priority: P0
cost: H
closed: 2026-11-10
branch: bandreach/dome-invalid-margin
pr: 6779
---



## Finding

Filleting the dome's equator rim (`test_support::dome(1.0)`, the
plane–sphere rim of radius 1, one closed edge) across a ladder of blend
radii reads, at the default band:

| r | outcome |
|---|---|
| 0.1, 0.25 | builds, both contact edges jet-determinate at the closed form |
| 0.4, 0.5 | `FaceClearanceUncertified` (definite) |
| 0.6, 0.75, 0.9, 0.95, 0.99 | `Escalated { predicate: Some("fillet3_face_clearance"), margin: MarginDiag::Invalid }` |

A NON-VALUE margin reaches a user-facing refusal: the clearance screen
(`crates/sweep/src/blend/battery.rs`, `fillet3_face_clearance`, the
`gap − setback_here − setback_there` margin) is handed a poisoned
reading past `r = 0.6` and escalates it as if it were in band. The
rendered sentence then quotes a margin that is not a number, and the
recourse ("reduce the blend size, or enlarge the support face") is the
in-band sentence for a reading that was never classified. Measured by
BLEND-14's second reviewer and pinned as a table (not an assertion) in
`review_contact_edge_must_carry_r2_probes::r2_a_sphere_support_toward_osculation_never_reaches_the_rule`;
the likely cause is a setback the arm cannot derive once the ball's
centre passes the sphere's centre (`r > R/2`, the spine collapsing),
which the arm should refuse typed before the screen reads it.

## Disposition

Outside BLEND-14's fence (the battery's screen and the arms' setbacks);
filed for whoever owns the clearance screen. Not acted on.

Moved from `work/issues/` to `work/bandreach/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

## Closed

Re-measured on `origin/main` at `b0acc5e1e9`: r = 0.1, 0.25 build; 0.4,
0.5 refuse `FaceClearanceUncertified`; 0.6 to 0.99 escalate
`fillet3_face_clearance` with `MarginDiag::Invalid`, the table as filed.

**Cause.** Both setbacks the screen subtracts were NaN, the sphere's
first. `arms::plane_sphere_blend` takes the spine radius as
`s = √((R − r)² − h²)`, and on the dome `h = r`, so `s² = R(R − 2r)`.
The ball must sit `r` above the plane and `R − r` from the sphere's
centre, and both hold only while `R − r ≥ r`. At `r = R/2` the spine
shrinks to a point on the axis, and past it no ball of radius `r` rests
on both supports. The arm gated nothing and minted NaN trimlines,
which the clearance screen classified. The ball's centre does not pass
the sphere's centre, as the filing guessed: the spine circle closes on
the axis. The lentil's sphere × sphere equator had the same defect
through the sheet reduction (`arms::sheet_center`'s circle × circle
root), past `r = 0.4`.

**Outcome.** A typed refusal at the source. When the link resolves,
`fillet3_ball_room` decides that the offsets cross, on their secant
margin in metres (`arms::plane_sphere_room`, `arms::rim_room`). The
door is `battery::room_at`, shared with `fillet3_knife_room`. A radius
past it refuses `BlendError::BallDoesNotFit` with
`FILLET3_BALL_ROOM_RECOURSE`. A smaller radius always has room, so the
recourse holds wherever the refusal fires. The dome now builds at 0.1
and 0.25, refuses the clearance screen at 0.4, and refuses
`BallDoesNotFit` from 0.5: Zero at 0.5, room `1 − 2r` past it.

**Pinned.**
- `review_contact_edge_must_carry_r2_probes::r2_a_sphere_support_toward_osculation_never_reaches_the_rule`:
  the whole ladder, asserted one class per stretch.
- `a_ball_with_no_room_at_its_rim_refuses_typed`: dome, hemisphere,
  pocket floor, lentil, lantern neck and shoulder, laddered through
  their room. No refusal carries a non-value margin, and the first
  radius without room is the closed form's.
- `blend_recourse_followability::the_ball_room_recourse_is_followed_by_a_ball_that_fits`.
- `verbs_arms1_r1_probes::near_limit_radii_refuse_typed`, at the
  closed-form room.

