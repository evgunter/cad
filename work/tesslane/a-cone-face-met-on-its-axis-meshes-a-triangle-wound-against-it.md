---
id: a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it
kind: issue
title: T1's cone joined to a box whose edge crosses its axis meshes a cone triangle that reach_probes' geometry reads as wound against the face
status: dispatched
opened: 2026-11-16
priority: P1
cost: M
design: true
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

## Measured, and stopped (2026-11-17)

The lane dispatched on the section above measured the triangle and stopped, as the section asks, because the cure has more than one defensible home. The section's premise was wrong: the face is not the chart lane's.

- **The face is the rectangle lane's.** T1's refusing cone face is half the cone, chart box `u ∈ [π, 2π]`, `v ∈ [−2.4, 0]`, the apex row at `v = 0`, each side ruling split once at slant 1.039. A split side makes `bare_sides` false, so `curved::grid_counts`' cone arm keeps its lattice (17 columns by 11 rows at δ 5·10⁻³, 36 by 23 at 10⁻³): the arm of `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`. The paragraph "Not this branch's" above emulated the chart lane's cone arm, which never sees this face.
- **The triangle.** Its corners are an apex copy, `(π, 0)` or `(2π, 0)`, and two lattice nodes of the first interior column, `(u₁, −j·hv)` and `(u₁, −(j+1)·hv)`. A triangle in the chart; in space three points of one ruling (the nodes' offsets from the apex are exactly one and two times one vector). No area by construction, at any δ.
- **What makes it.** The lane triangulates in the raw chart. The side from the apex copy down to the split vertex is one constraint about 1.04 long with no point on it; the triangulation fans the apex copy down the column beside it to about mid-side, and every fan triangle that takes two nodes of that column is the one above. One a side at 17 columns, three or four at 36. A coarse δ meshes because the fan then reaches one node of the column.
- **Not on a sphere, and not in the development.** A sphere's meridian is an arc, so the like triangle at a pole has area. In the chart lane's development the apex is one vertex and a ruling an exact line.
- **How wide.** Over the whole mesh suite instrumented at the join, unresolved triangles occur only in T1's row. Off the suite: plain cones and cone wedges (quarter, half, three-quarter and full turn; half-angles 45° and about 80°), each ruling split at `{0.5}`, `{0.3129, 0.15645}` or `{0.9}`, placed obliquely, δ 0.1 to 0.001: **every cone face through its apex with a split ruling refuses `UnresolvedWinding` from δ 0.01 down, 84 cells**, the plainest a full revolved cone whose seam ruling is split at its midpoint. The 80° cone also refuses `CertificateExceeded` at many δ, and a face spanning both nappes at every δ when split. A ball and a sphere band: clean.
- **Measured, with a temporary route of every cone face to the chart lane:** all 18 T1 cells mesh and pass `check_mesh` and the probes' geometry check, and so does every body of the second pass at every δ, the ones the rectangle lane refuses `CertificateExceeded` included.

Where a class of rectangle faces is meshed, and what the kernel answers for it meanwhile, is a design question: three designers are weighing it (`docs/DESIGN-FORK-PROTOCOL.md`), and the unit resumes on their answer.

## The designers' answer, and what this unit builds (2026-11-17)

Three designers weighed it, two rounds (`docs/DESIGN-FORK-PROTOCOL.md`; the reports are on `analysis/design-fork/tesslane-cone-apex-split-side`). They agree on all of this:

- **The class is wider than this row.** It is every *ruled face* (a face on a cylinder or a cone, whose lines `u = const` are straight rulings) with a *split side* (a straight side carried by two or more edges). A cylinder wall with a split ruling refuses `CertificateExceeded` from δ 0.01 down, at every δ on some; a frustum with one meshes on a lattice no theorem sizes; slender cones (half-angle 5°, 1°) refuse `UnresolvedWinding` at every δ.
- **The cause** is the rectangle lane's triangulation, not its lattice: it triangulates the raw chart, its theorem for a ruled face (every triangle between two neighbouring rulings) is a premise nothing holds, and a vertex on a straight side is the one boundary point off the grid. The fallback lattice (`curved::grid_counts`' arm for a side that is not bare, from PR 5717's fix pass) was laid to paper over that and is what makes the triangles of no area.
- **The final state** is not this unit: such a face meshes in the chart lane as the same face without the split plus one triangle per vertex, with no interior rows. It is its own unit, after this one.
- **Until then** these faces are the chart lane's, which meshes every one of them today, and `UnresolvedWinding` stays as the backstop on every lane.

**This unit builds the "until then".** Same letter and tier as above (M, single FULL review).

- **The route.** A cylinder or cone face that the doors decide is an iso-rectangle, and whose straight side is carried by more than one edge, goes to the chart lane. The test is read from the face's own outer loop, before the chord pass (`chords` and `face_target` read the lane), in the place the face's lane is decided and nowhere else. It is the rectangle lane's own admission test (`bare_sides`) moved ahead of the route. It must not catch a keyway, a notch, a slit or a ringed face: those are no rectangle and are routed already. One designer's probe used "more than two line-carrier traversals" and moved seven pins for that reason alone.
- **The deletion.** The rectangle lane's fallback lattice and `bare_sides` go; the lane takes a ruled face only with bare sides, and a ruled face that reaches it otherwise is a kernel defect that fails loudly and typed, never a mesh.
- **Base.** This branch carries PR 7043. Unit 4 (PR 7041, `tesslane/route-by-domain`) rewrites where the lane is decided (`tessellate::Routes`, `curved::iso_rectangle_shape`) and is in its fix pass; merge that branch when the orchestrator says its fix head is pushed, and put the test where that unit decides the lane. Until then build against `lane_of` as it stands here, the test as one function of the face's loop so the move is a call site.
- **Done when:**
  - all 18 cells of T1's row mesh and pass `check_mesh`, `validate::check_winding` and `reach_probes::geometry`, at the default ε, 10⁻⁶ and 10⁻¹² where the body builds; `a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta` is re-pinned as a mesh;
  - `curved::tests`' split-side sweep has no refusal (246 meshed and 4 refused today; the four are the mirror-nappe fixture's cylinder wall, and `TOTAL_REFUSED`'s comment blames its cones);
  - a row meshes, at δ 0.1, 0.01 and 0.001 and obliquely placed, each of: a full and a half cylinder wall, a cone through its apex at 45° and at 80°, a frustum, and a slender cone (5°), each with a ruling split at one point and at two;
  - a mutant that removes the route reds a row at body level, and one that widens it to every ruled face reds another (D9, or a count pin).
- **What moves, argued.** A digest of every patch of the mesh suite, `sweep`'s meshing rows and `step-import`'s on the base and on the head. Only faces of the class may move. For those that meshed on the base (the frustum, coarse δ cones and cylinders, `step-import`'s `split_iso_side`), table the triangle counts before and after: the chart lane lays rows these faces do not need and sizes at its headroom, and that price is known and is the next unit's to remove. A face outside the class whose bytes move, or any face that meshed on the base and refuses on the head (a hand-built face of the class that stores no chart image would answer `DomainUnread`), is a stop: report the body.
- **Named refusals this leaves**, each pinned if a door builds it and said so if none does: a one-face full-turn wall with a split seam (`SeamTwice`); a face on both nappes (`BothNappes`).
- **Tracker, on this branch.** File the cylinder wall's refusal as its own issue row and close it here; re-point `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` (its arm is deleted; the face is the chart lane's at the named headroom until the strip unit); add to the closed row `curved-lane-triangulates-in-raw-chart-coordinates` that its closing premise ("no point off the lattice reaches this lane") was false for a split vertex, citing this row.
- **Run before every push** what the section above lists, and `cargo nextest run -p step-import` and sweep's meshing rows with it.

