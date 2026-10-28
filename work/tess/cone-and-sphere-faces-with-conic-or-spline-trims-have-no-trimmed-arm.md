---
id: cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm
kind: issue
title: a cone face bounded by a conic section, or a sphere face bounded by a spline, has no trimmed-lane arm
status: open
opened: 2026-10-28
priority: P2
cost: H
---


Filed by TESS's sphere-arm unit
(`sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`),
from its sweep for faces of the analytic charts a non-iso trim bounds.

**Where it refuses.** `mesh::trimmed::trim_frontier`, the chart
roster's fallback: a face whose loop carries an ellipse, B-spline or
spiric carrier (`trimmed::has_trim_carrier`) routes to the trimmed lane,
which has a cylinder arm, a NURBS arm and, since this unit, a sphere
arm for faces bounded by circles only (`trimmed::takes_sphere_arm`).
Every other such face refuses
`UnsupportedCurve { note: "conic/B-spline/spiric trim on a
plane/cone/sphere/torus chart — …" }`.

- **Cone.** A plane cutting a cone off its axis leaves a conic section
  on the cone face. `crates/sweep/tests/cone_join_lane.rs` accepts this
  refusal as one of its outcomes ("the mesh refuses only the conic
  trim"), so those bodies build and do not mesh.
- **Sphere bounded by a spline.** A sphere face whose loop carries a
  B-spline carrier (a general section, a projected `Net` image) still
  takes `trim_frontier`; the sphere arm's UV speed bound is a closed
  form for a circle's image (`chords::sphere_uv_speeds`) and refuses a
  projected spline image typed.
- The torus and plane cases are
  `work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`.

**What a fix owes.** For the cone, an arm like the cylinder's (its
certificate `cert::cert_cone` is UV-based, so the arm needs the conic's
UV image and a speed bound for it); for the sphere, a speed bound for
a projected spline image (the `General` arm's differenced-net hull is
the model).
