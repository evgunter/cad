---
id: a-step-placement-whose-reference-lies-nearly-along-its-axis-mints-a-frame-out-of-square
kind: issue
title: A STEP placement whose reference direction lies within about 1e-6 rad of its axis is admitted and mints a frame out of square, so a conic's stored points stand off the extent read from it
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found by a reviewer of ONEROUND's PR 7510 (2026-11-23; `analysis/oneround/conic-readers/review-report-delta.md` on `oneround/conic-readers-review-r1`, `b7435b27bf`, its m2, with a probe by mint under `review-r1-delta/`). **Demonstrated at the mint, not end to end: no STEP file was pushed through the importer.**

`geom_core::RigidMotion::from_axis_and_reference`, as `step-import` calls it for every `AXIS2_PLACEMENT_3D` (`entities.rs` `placement_built`, the site `"step_placement_axes"`, decided in `Band::least()`), refuses only an exactly parallel axis and reference, as its doc says. A reference a small angle off the axis is admitted, and the frame it mints is out of square by about 5e-16 over that angle. The reviewer's draws (4,000 a row; reference off the axis → worst `|u_ref·axis|` → the size at which a stored point of a conic stands 1e-9 m off the extent read from that frame):

| angle | refused | worst skew | size |
|---|---|---|---|
| 1e-2 rad | 0 | 4.6e-14 | 2.2e4 m |
| 1e-4 rad | 0 | 4.8e-12 | 210 m |
| 1e-6 rad | 0 | 4.4e-10 | 2.3 m |
| 1e-7 rad | 0 | 5.0e-9 | 0.2 m |

`Frame::new` reads the same, and `Curve3::ellipse_anchored` accepts a seam 1e-7 rad off the axis (skew 1.9e-9). Every reading of a conic that takes its frame as square (`Conic::along`, the extent, a height over its plane: `work/onecopy/a-conics-harmonic-takes-its-stored-seam-as-square-to-its-axis.md`, arriving with PR 7510) is then off by the skew times the size. A re-seam adds nothing, and `plane_cone_section`'s own frames are square to 1.7e-16.

**What it is.** The angle between the axis and the reference is a quantity the completion divides by, and `crates/geom-core/README.md` ("Float rounding", re-posing): a quantity a formula divides by that can vanish is a pose decided first, the formula dividing only on its definite arm. Here the pose is decided in the least band, so "nearly parallel" passes as definite. A file with such a placement is ill-posed input (the format's reference is meant to be projected square to the axis from a direction well away from it), so this is a wrong admission of bad input, not a wrong reading of good input.

**Owed:** the pose (reference against axis) decided in a band that covers the completion's conditioning, refusing as the caller's input where the frame cannot be minted square to the run's tolerance at the sizes the file holds; a row that pushes such a file through the importer and reads one conic's point against its extent.
