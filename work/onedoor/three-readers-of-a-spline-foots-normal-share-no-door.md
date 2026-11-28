---
id: three-readers-of-a-spline-foots-normal-share-no-door
kind: issue
title: Three readers take a spline placement's in-trim foot to spline_outward_normal_at, each with its own error policy, and share no door
status: open
opened: 2026-11-27
priority: P3
cost: M
---

Filed by the LATHE teapot lane on PR 7824's review (R1, R2, 2026-11-27).

## What

One reading, three spellings. Each of these takes the in-trim foot a
spline placement found and reads
`face_normal::spline_outward_normal_at` there:

- **`boolean::slit::spline_floor`**:
  - places with `spline_region::spline_face_placement`;
  - a placement or normal escalation becomes `BooleanError::Escalated`;
  - a query-keeping refusal, and `NotSpline` or `PlaceholderChart`, become `Ok(None)` (no floor);
  - an aborting refusal becomes `ClassificationInvariant`.
- **`boolean::vtxfac`, the pierced spline face** (`classify_vertex_on_face`'s `n_pierced`):
  - takes the sweep's recorded chart point;
  - `At(refusal)` becomes `BooleanError::of_pierced_normal`;
  - `PlaceholderChart` becomes `CurvedBooleanUnsupported`;
  - `StaleFace` and `NotSpline` become `ClassificationInvariant`.
- **`boolean::join::spline_held_normal`**, through `holding_face_normal`:
  - places with `contain::face_placement_charted`;
  - `At(why)` becomes `HoldingUnread::Normal`, ranked by `NormalAtError::refusal`;
  - `StaleFace`, `NotSpline` and `PlaceholderChart` become `JoinDesync`.

So the same `SplineNormalError` arm is read in three ways: a face that
is no spline after its placement read a chart point is no floor, a
classification invariant, or a desync. A fourth reader would write a
fourth policy.

## What it needs

One door, "a spline face's outward normal at a point placed in it":
placement and normal together, returning the normal or one typed
refusal. Each caller then maps that one refusal to its own error once.
The uniqueness premise
(`splice/the-spline-placement-read-certifies-no-second-in-trim-foot`)
would then have one reader to certify for, not three.
