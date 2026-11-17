---
id: a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it
kind: issue
title: T1's cone joined to a box whose edge crosses its axis meshes a cone triangle that reach_probes' geometry reads as wound against the face
status: dispatched
opened: 2026-11-16
priority: P1
cost: M
branch: tesslane/cone-axis-winding
---


## Finding

`reach_probes`' T1 pose (the cone of half-angle `π/6` and radius `1.2`,
turned a quarter about `y`; the box `(0,6)×(0,6)×(−3,3)` turned `−50°` about
`z`) with the box's edge at `(xf·s, height − 1.5·s, 0)`, `xf = 0`: the edge
crosses the cone's axis. For `s = 0.6` and `0.3`, the ops `A ∪ B`, `B ∪ A`
and `A ∖ B` (A the cone): `mesh::tessellate` answers `Ok`, `check_mesh`
passes, and `reach_probes::geometry` refuses the mesh: *a Cone face's
triangle winds against the face*.

| s | op | δ `2e-2` | δ `5e-3` | δ `1e-3` |
|---|---|---|---|---|
| 0.6 | `A ∪ B`, `B ∪ A`, `A ∖ B` | passes | wound against | wound against |
| 0.3 | the same three | passes | passes | wound against |

Each named triangle has a corner at a low shared mesh id (`0`, `9`, `11`): a
vertex of the body, by its id the cone's apex or a corner on the axis. The
other three ops at these poses, and every op at `xf = 0.5, 0.8, −0.5`, pass.
Measured 2026-11-16 on `tesslane/cone-near-apex`, default ε, debug.

## Not this branch's

The same nine cells answer the same way with the cone arm set back to the
chart-box certificate, rows uniform in `v` and the ear rule (main's three,
emulated on that branch; main itself not rebuilt). So neither the cone
certificate, the graded rows nor the shared-side rule makes them.

## Not measured

Which lane emits the triangle; whether it is a fold the lane's winding
census misses (it reads the kernel's normal at the centroid's foot,
`TrimmedFace::outward`) or a reading of the probe's own (`geometry` takes
the surface's closed-form normal at the centroid, and a triangle with a
corner on the axis has its centroid's foot near where that normal turns).
The mesh encloses the body's volume to the chordal bound, by the same
check. No row pins it: `geometry` is a probe-file check.

- 2026-11-16 — **The nine cells refuse `UnresolvedWinding` on the cone face** (the clearance and winding unit, on main with PR 6965 merged). With `[mesh.winding]` read where faces join the mesh, the nine cells of the table that read "wound against" are exactly the nine `tessellate` refuses, each on its cone face, at the default ε, `10⁻⁶` and `10⁻¹²`; the nine that pass mesh, and `validate::check_winding` and `geometry` pass on them. So the triangle the probe named is one whose winding is inside the rounding of its own positions: its normal is noise, and the probe read the noise as inward. It is not shown to be a fold, and the probe's normal at the axis is not what failed. Pinned: `reach_probes::a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta`. Still unmeasured: which triangle it is (its corners, its height against the allowance) and what in the cone arm lays it; a body that meshed with a noise normal now refuses typed, and the cure is this row's.

## Unit (2026-11-17, TESSLANE)

**Letter M. Tier: single review, FULL.** One arm's emission near its singular vertex: narrow, but a cure that keeps a triangle it should not, or drops one it should keep, is a mesh with a hole or a fold, so believing it takes claims to falsify.

Branch `tesslane/cone-axis-winding`, cut from `tesslane/lattice-clearance` (PR 7043, in review): that PR is what turns these nine cells from a mesh with a noise normal into the typed refusal `UnresolvedWinding`. Merge that branch as its fix pass lands, and main after it merges. Unit 4 (PR 7041) and the cylinder certificate unit are in `tessellate.rs` and `cert.rs`.

**What is known.** Nine cells (s = 0.6 at δ 5·10⁻³ and 10⁻³, s = 0.3 at δ 10⁻³; A ∪ B, B ∪ A and A ∖ B, A the cone) mesh on main with one cone triangle whose winding is inside the rounding of its own positions, and refuse on PR 7043 as `[mesh.winding]` rules. Each such triangle has a corner at a low shared mesh id: the cone's apex or a body vertex. The same nine under main's older certificate, rows and ear rule. Nobody has looked at the triangle.

**Owed by the unit:**

- **Measure before building.** Get the refused face's triangles (a test-only hook, or the lane called below the join): for each unresolved triangle its three corners in the chart, in the development and in space, what each corner is (an apex copy, a boundary chord point with its edge, a lattice node, an ear or shared-side point, a refinement point), its cross product against the allowance, and which step of the lane made it (the development's triangulation, the kept-side rule's insertion, the ear cure, the refinement). Say whether it is a triangle of no area by construction (two corners one point in space; three corners on one ruling) or a thin one that a coarser or finer δ would resolve, and why the nine coarser cells mesh. Then look for the same shape off this pose: a cone face through its apex cut by one plane through the axis, by two; the same on a sphere face through a pole and on the spindle's apex, on both curved lanes. The PR body leads with what the triangle is.
- **Cure it where it is made, in the lane's own terms.** If the lane already has the rule for this shape on another arm or at another step (the singular-row rule; the development's "three points of one ruling cannot be a triangle's corners"; the kept-side rule), the cure is that rule reaching this case, and the PR says why it did not. The verdict at the join is the backstop and is not loosened, scaled or skipped for a face; no triangle is dropped from a mesh after the fact; nothing reads ε.
- **If the triangle cannot be avoided** (the body's own vertices put three boundary points within rounding of a line), that is the page's refusal standing, and it is a stop: report the body and the measurement, do not build.
- **Done when** the nine cells mesh at every ε where the body builds and pass `check_mesh`, `validate::check_winding` and `reach_probes::geometry`; the row PR 7043 pinned (`a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta`) is re-pinned as a mesh; a mutant that removes the cure reds it.
- **What moves, argued.** Name every patch whose bytes move (a digest of every patch of the mesh suite on the base and on the head) and why each new mesh is right. A face that meshed on the base and now refuses, or a rectangle-lane or planar byte that moves, is a stop.
- **Run what CI runs before every push**, and push at least hourly, work in progress included: `bash scripts/run-gates.sh`; `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit`; fmt; clippy for mesh, geom-brep, `demos/tour` and the viewer job (`cargo clippy -p viewer --features app --all-targets -- -D warnings`); `cargo nextest run -p mesh` whole and the viewer's tests; the steps of the `mesh budget meter + certificate falsifier` job and of the `tools` job from `.github/workflows/ci.yml`; sweep's cone rows; the rows you add or change at `CAD_TOLERANCE_EPS` 1e-6 and 1e-12 as well, each stating what it expects at each. A test costing 0.5 s or more on the hosted runner goes in the slow set with its measured time.
