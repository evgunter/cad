---
id: the-cylinder-pose-levers-a-tilt-from-the-axis-foot-of-the-region
kind: issue
title: The cylinder pair's pose levers a tilt from the axis foot of the region's centre, so a tilt that slides a wall along itself reads crossing
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Found by SECTARMS stage 2 (branch `sectarms/cylinder-pair-readers`), when
the guard's retired cylinder reading
(`section_cert::cylinder_cylinder`) was replaced by the value's pose.

## Measured

`topo::boolean::section_cert_axis_pose_rows::wide_walls_tilted_in_the_band_read_crossing_over_a_region_on_the_wall`:
- two walls of radius `r` ∈ {100, 1000} on one axis, `20` to `200`
  bands apart in radius;
- the second tilted `0.45` bands per metre;
- read over a unit region on the wall, `r` from the axis.

The pose (`CylinderCylinderPose::decide`, `cc_axes_parallel`, through
`cylinder_axes_parallel_reported` at `Reach::Ball(region)`) levers the
tilt by `Reach::lever_between`. That is the region's far side from the
axis's foot of its centre, `r + 1`. So it reads the axes crossing, and
the near-parallel march over the region refuses (R-march). The retired
reading levered at the region's diameter with a swing of
`sin·radius + r₂·sin²`, crediting that a turn about the pivot slides the
wall along itself. It served the rulings.

## What a fix has to decide

Whether the pose's parallelism reading can take the slide credit (the
displacement of the walls inside the region, not of every point within
the lever) without understating the lever on a pose where the section
does move by the full `θ·(r + 1)`. The decided text levers parallelism
`at the radius L of the operation's region`, which this reading
overstates for a region off the axis: errs safe, but it loses a served
case.

## 2026-11-02: since the ruling sweep

The row now reads the pose's crossing verdict on the ruling sweep, not on
the march (branch `sectarms/cylinder-ruling-sweep`). At `r = 1000` the
sweep decides there is no component in the region. At `r = 100` the
rulings near the region meet the second wall far off, levered by
`1/sin` of the tilt, and the sweep escalates on `ruling_cut` (R-tan).
The lever question stands as filed.

