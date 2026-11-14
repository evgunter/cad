---
id: cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm
kind: issue
title: a sphere face bounded by a spline has no trimmed-lane arm (its cone half is carried by the torus-and-cone row)
status: parked
opened: 2026-10-28
priority: P2
cost: H
blocked_on: [a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses]
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
  `work/tesslane/trimmed-tessellation-lacks-torus-and-plane-arms.md`.

**What a fix owes.** For the cone, an arm like the cylinder's (its
certificate `cert::cert_cone` is UV-based, so the arm needs the conic's
UV image and a speed bound for it); for the sphere, a speed bound for
a projected spline image (the `General` arm's differenced-net hull is
the model).

- 2026-11-14 — moved from TESS to TESSLANE by `git mv` when TESS and TESSSIZE merged into their successor TESSLANE (`docs/doc-ledger/tess-and-tesssize-leave-the-tracker.md`), id and body unchanged.

## Measured on main (2026-11-14, TESSLANE) — the cone half moved, the sphere half is blocked earlier

- **Cone × ellipse** refuses at `trimmed::trim_frontier`, as filed. It is the same cell as `trimmed-tessellation-lacks-torus-and-plane-arms`'s cone case and is carried there at P0.
- **Sphere × spline: `trim_frontier` is never reached.** A cylinder r 0.5 against a ball r 0.3 at `(0.5, 0, 0)` builds in all six ops, and all six refuse `UnsupportedCurve` ("B-spline carrier with a C⁰ kink (interior multiplicity = degree)") from `chords::nurbs_chord_count`, in the chord pass, before any face is dispatched. Three more poses gave the same. That is `a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses`, which this row is parked on. No public construction was found that puts a C¹ spline on a sphere face, so the cell this row names has no witness yet. Read, not run: with a C¹ carrier the face would reach `trim_frontier`, since `takes_sphere_arm` excludes a face with a trim carrier; what the sphere arm then lacks is a UV speed bound for a projected net (`chords::sphere_uv_speeds` bounds a conic only).
