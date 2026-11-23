---
id: the-lily-bud-is-an-ovoid-and-wall-4-is-deleted
kind: unit
title: The lily's bud is authored as an ovoid with ellipse_to, and wall 4 is deleted, not re-aimed
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: an-ovoid-cannot-be-authored
blocked_on: [ellipse-to-draws-the-stretched-circle-from-a-point-tip, a-spheroid-face-tessellates-within-its-own-sizing-theorem, a-spheroid-writes-exact-step-as-a-surface-of-revolution]
---



Unit 10 of `an-ovoid-cannot-be-authored` (Ev, PR 7289): "No non-uniform
scale of a body; the lily's wall 4 is deleted, not re-aimed." The bud is
three partial revolves of one meridian on a tilted tripod of axes
(`demos/tour/src/lily.rs`, `bud` and `meridian`), so stretching each
meridian's belly along its own axis is the ovoid.

## What is true when it is done

- `lily::meridian` writes the belly as
  `ellipse_to(Stretched { c, along, factor, winding, p })` about the
  globe centre, stretched along the bud's axis, in place of
  `arc_to(Center { .. })`; the other legs keep their shape, their
  points recomputed on the stretched belly. The bud is written as a
  user would write it (implementer discipline §3).
- Wall 4 (`lily.rs`, the `Affine3` stretch refused by
  `RigidMotion::check_rigid` with `transform_rigid_col2_unit`) is
  deleted, not re-aimed: `RigidMotion` refusing a stretch is right.
- The tour renders the new bud (a `[render]` commit) and its frames
  move; the PR says what moved and why, and D9's byte identity for the
  lily is re-baselined on the new geometry.
- `docs/KERNEL-VERBS.md`'s "spheroid / ellipsoid primitive" row says
  the spheroid is a kind (PR 7289) and the lily's bud is built from
  it, and no longer cites wall 4. `work/meta/m6-carried-items-register.md`'s
  "Lily walls 4 ..." item drops wall 4.
- `an-ovoid-cannot-be-authored`'s parts 1 and 2 are done; the row
  stays open on SPLINEWALL's part 3.

## Check first

Whether any later lily scene reads the bud's faces as spheres by kind
(a `Surface::Sphere` match in `lily.rs` or its probes, such as
`review_probes::the_buds_three_axes_form_the_authored_tripod`). A
pinned probe that held the old kind moves with the bud; one that held
something the ovoid breaks is a finding to report, not to re-pin.

## Rows and ground

- Closes the part 1–2 bar's demo; changes `work/meta/m6-carried-items-register.md`.
- Crosses the tour (`demos/tour/src/lily.rs`) and `docs/KERNEL-VERBS.md`.
- Review: **single (STYLE)**, with the render frames looked at before
  merge. Difficulty **M**.
