---
id: a-left-profiles-full-revolve-keeps-the-reversed-builds-anchors-and-rims
kind: issue
title: A full revolve of a profile to its axis's left anchors its walls, runs its rims and splits its bands by the reversed axis it builds about
status: open
opened: 2026-11-23
priority: P2
cost: H
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
---

Found by LATHE's one-side-of-the-axis lane, in the fix pass of PR 7425.

A profile to the left of its revolve axis builds as the same turn about
the reversed axis (`revolve::axis::AxisFrame::reflected`, read in
`revolve_construction`). What the body hands out reads back the
authored axis:
- the wall charts' axis (`AxisFrame::chart`);
- a plane wall's sense;
- the gaps' rank.

For a partial revolve, the left body is then its mirror's body turned
half a turn about the axis, face for face. The pin is
`a_revolve_takes_a_profile_on_either_side_of_its_axis::a_profile_and_its_mirror_across_the_axis_revolve_alike`.

A FULL revolve still shows the reversed build in three places:
- **Wall anchors.** A cylinder's or plane wall's frame origin is
  anchored at its swept segment's start (`revolve::surfaces::wall_surface`,
  `foot3(seg.a)` / `world(seg.a)`). A full revolve traverses reversed
  in either frame, so a left profile's start is its mirror's end.
  Measured: the washer `[1, 2] × [0, 1]` and its mirror, heading `+y`.
  The right inner cylinder is anchored at `y = 0` and the left at `y = 1`.
- **Rim direction.** Measured (PR 7425's re-review): `topo::readback::edge_pose`'s
  axis for a washer's rims under `Full` is `+heading` for a profile right of
  the axis and `−heading` for one to its left. Partial rims agree with
  their mirror's. `full.rs` mints every rim with `axis_c = Positive`,
  sweeping `+2π` about the axis it builds about. The likely site is
  `AxisFrame::latitude` (`crates/sweep/src/revolve/axis.rs`), which carries the raw
  (reflected) azimuth rather than the authored chart. **A consumer reads
  it:** editor-core's edge Axis read (`eval/wire/pose.rs`, the
  edge-pose arm) hands that axis out as a directed axis. So a user can
  read a wrong directed answer, the same class as PR 7425's MAJOR 2 on
  wall charts. Hence P2.
- **Band labels.** The wire case's `walls`/`rims` are the `0…π` band
  about the build axis, so the authored `π…2π` band for a left profile.
  `lathe/full-revolve-one-band` retires the π band, which removes this one.

The body (point set, volume, outward normals, wall chart axes) is the
mirror's turned in every case. The fix is a full builder that sweeps
toward `+n` (forward traversal, `axis_c` negative) for a reflected
frame, in place of the reversed-traversal convention it fixes today.
That is `full.rs`'s ground and coupled to the one-band rewrite and to
`a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote`,
which is rewriting `full.rs`.
