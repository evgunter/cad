---
id: cone-and-sphere-faces-with-conic-or-spline-trims-have-no-trimmed-arm
kind: issue
title: a sphere face bounded by a spline has no trimmed-lane arm (its cone half is carried by the torus-and-cone row)
status: open
opened: 2026-10-28
priority: P0
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
  takes `trim_frontier`; the sphere arm's chord pass reads a conic's
  projected image only (`chords::sphere_image`) and refuses a projected
  spline image typed.
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
- **Sphere × spline: `trim_frontier` is never reached.** A cylinder r 0.5 against a ball r 0.3 at `(0.5, 0, 0)` builds in all six ops, and all six refuse `UnsupportedCurve` ("B-spline carrier with a C⁰ kink (interior multiplicity = degree)") from `chords::nurbs_chord_count`, in the chord pass, before any face is dispatched. Three more poses gave the same. That is `a-quadric-pairs-section-spline-carries-a-c0-knot-the-chord-pass-refuses`, which this row is parked on. No public construction was found that puts a C¹ spline on a sphere face, so the cell this row names has no witness yet. Read, not run: with a C¹ carrier the face would reach `trim_frontier`, since `takes_sphere_arm` excludes a face with a trim carrier; what the sphere arm then lacks is admission of a projected net (`chords::sphere_image` admits a conic's image only).

## 2026-11-14 — what the one chord rule changes here (TESSLANE, `one-chord-rule-over-each-boundary-images-speed-bound`)

The fix's sphere half no longer owes a speed bound: `geom_brep::Pcurve::speed_bound` answers a projected net's on every analytic chart (the carrier's certified `sup|C′|` over its control box's lever arm), and a focal section's on a cone. What the sphere arm owes is to admit the net: the chord pass refuses a projected spline or line image on a sphere-arm face (`chords::sphere_image`), as the arm meshes circle-bounded faces, and the arm itself does not walk one. The cone half's speed bound is the focal section's, `(1 + |β|)/(1 − |β|)` on the azimuth, attained; what it owes is the arm.

## Measured with the closed-loop carrier in (2026-11-17, SECTARMS, branch `sectarms/periodic-loop-carrier`)

A closed fitted section loop is now a periodic cubic, and no edge on it
reads an interior knot past multiplicity one, so `chords::nurbs_chord_count`
takes every carrier. What then answers, at δ 2e-2, 5e-3 and 1e-3
(`crates/mesh/tests/quadric_pair_sections.rs`):

| pair | ops | `mesh::tessellate` |
|---|---|---|
| cylinder × cylinder (a rod r 0.2 across a rod r 0.5, two loops) | all six | meshes, `check_mesh` passes |
| cylinder × sphere (the measured pose) | all six | `UnsupportedCurve` from `chords::chart_image`, the sphere face's projected spline image |
| sphere × torus | ∪, ∩ | the same refusal |
| cylinder × torus (a rod through a donut) | ∪, ∩, donut ∖ rod | the same refusal, on the torus face (`a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound`) |

So this row's sphere half is now the first stop of every cylinder ×
sphere boolean in a general pose.

## Re-banded P0 (2026-11-25)

The teapot's handle union builds, and the joined pot does not mesh: the
section edge between the vessel's sphere and the handle's torus refuses
`UnsupportedCurve` from `mesh::chords::chart_image` ("trimmed analytic
face bounded by the projected image of a spline or line carrier — the
trimmed lane bounds the UV speed of a conic's image only"), measured on
the tour's `tess-budget` sweep at δ 2e-4 (`teapot/teapotvessel`). Teapot
wall 2 (`demos/tour/src/teapot.rs`, `wall_probes`) pins it, so this row
is a stop of the P0 umbrella `splice/curved-demo-joins-build` and
carries its band. The joined pot needs both this row and its torus half, `a-torus-face-bounded-by-a-projected-spline-image-has-no-chord-bound`. The sphere's half meets `a-projected-spline-images-speed-bound-on-a-sphere-is-eleven-orders-loose` next: lifting the conic-only rule alone refuses at the chord budget.
