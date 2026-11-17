# The problem

You are a designer on a Rust B-rep CAD kernel (repository evgunter/cad). Read `docs/prompts/designer.md` IN FULL first: it binds you, and it says what your report is. Then `CLAUDE.md`. What follows is the problem as the orchestrator understands it. It is a hypothesis about the tree, not a finding: check it, and correct it first if it is wrong. No solution is proposed here on purpose; the options are yours to find.

## Terms

- The mesher (`crates/mesh`) turns each face of a solid into triangles within a tolerance δ of the surface. Its ratified page is `crates/mesh/README.md`.
- A curved face is meshed in its **chart** `(u, v)`. On a cone, `u` is the angle round the axis and `v` the slant distance from the apex, so the whole row `v = 0` is ONE point in space, the apex, and every line `u = const` is a straight **ruling** through it.
- Two **lanes** mesh curved faces. The **rectangle lane** (`mesh::curved`) takes a face whose domain is a rectangle of its chart. It walks the boundary, lays a lattice inside, triangulates in the raw chart `(u, v)`, and keeps every triangle but those it knows are collapsed at a pole. The **chart lane** (`mesh::trimmed`) takes every other curved face; on a cone it triangulates in the cone's development (the cone unrolled flat about its apex), where the apex is one point and a ruling is a straight ray from it. `mesh::tessellate::lane_of` chooses.
- A **split side**: a side of the rectangle (here a ruling from the apex down to the rim) that is two edges of the body, with a vertex part-way along. Booleans make them whenever a cut ends on a ruling.
- `[mesh.winding]` (ratified 2026-11-16, PR 6966): every emitted triangle's winding is outward and resolved above the rounding of its own positions, and a face that would emit one f64 cannot resolve refuses typed. PR 7043 (`tesslane/lattice-clearance`, in review, the tree you are given) builds it: the refusal is `TessellateError::UnresolvedWinding`.

## What was measured (2026-11-17, by the lane that was sent to cure it and stopped)

1. A cone face that holds its apex and has a split side is meshed by the rectangle lane. With a bare (unsplit) side that lane lays no interior lattice; with a split side `curved::grid_counts`' cone arm keeps its lattice (`bare_sides` is false). The same arm is the subject of `work/tesslane/a-cone-face-with-a-split-side-has-no-worst-triangle-theorem.md`.
2. The triangulation then emits triangles whose corners are an apex copy, `(u_side, 0)`, and two lattice nodes of the first interior column, `(u₁, −j·hv)` and `(u₁, −(j+1)·hv)`. In the chart that is a real triangle. In space the three points lie on one ruling: it has no area by construction, at any δ. The side from the apex copy down to the split vertex is one long constraint with no point on it, and the triangulation fans the apex copy down the column beside it. A coarse δ escapes only because the fan then reaches one node of that column.
3. On main these meshes are returned and pass `check_mesh`; those triangles' normals are rounding noise. On PR 7043 the face refuses `UnresolvedWinding`.
4. How wide: over plain cones and cone wedges (quarter, half, three-quarter and full turn; half-angles 45° and about 80°), each with a ruling split at one or two points, placed obliquely, δ from 0.1 to 0.001: **every cone face through its apex with a split ruling refuses `UnresolvedWinding` from δ 0.01 down, 84 cells**, the plainest being a full revolved cone whose seam ruling is split at its midpoint. In the mesh suite it is `reach_probes`' T1 pose (a cone joined to a box whose edge crosses the cone's axis), nine cells, pinned on PR 7043 as `a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta`.
5. The same arm also refuses `CertificateExceeded` on the 80° cone at many δ, and on a face spanning both nappes at every δ when split.
6. A sphere face through its pole does not make the shape (a meridian is an arc, so the triangle has area). In the chart lane's development the apex is one vertex and a ruling an exact line, so that triangulation cannot build it.
7. Measured, not proposed: with every cone face sent to the chart lane by a temporary route, all of those cells mesh and pass `check_mesh` and the probes' geometry check, including the ones that refuse `CertificateExceeded` today.

## What needs deciding

What is the code missing, such that a lane can make a triangle of no area by construction, and whose defect is it: the lane's triangulation, its lattice, what the boundary hands it, the route, or the division of faces between two lanes? What should mesh a cone face through its apex whose side is split, and from what facts? What should the kernel answer for these faces until that is built, given that main returns a mesh whose normals are noise and PR 7043 refuses a plain cone at δ 0.01? Say what the final state is, and what in it would need the project owner's word.

## What is ratified or planned around it

- `crates/mesh/README.md`: `[mesh.winding]`; `[mesh.sizing]` and its list of lanes and constants (which name the rectangle lane's cone arm and the headroom the chart lane keeps).
- `work/tesslane/plan.md`, section **The chart lane**: the plan's final state is one lane for every non-planar face. Its unit 6, `rectangle-faces-move-to-the-chart-lane`, retires the rectangle lane and is marked as the project owner's call, with a density measurement (the chart lane sizes its lattice at half of δ on an analytic chart until a theorem covers its boundary triangles, so a face moved there carries about twice the triangles today). Unit 4 (PR 7041, in review) makes `lane_of` route by the face's domain; unit 4b lays lattice lines through boundary runs on iso lines.
- `memories/output-stability-as-justification.md`: an unmoved output is never an argument, and a moved one is not a cost in your ranking (`designer.md` §4).

## Where to look

`crates/mesh/src/curved.rs` (the module header's five questions, `grid_counts`, `bare_sides`, the pole handling, `tessellate_curved`), `walk.rs`, `chords.rs` (which edges the rectangle lane's chord pass sizes against its lattice, and which it skips), `trimmed.rs` (the development, `CdtPlane`, the singular-row rule), `chart_table.rs`, `tessellate.rs` (`lane_of`), `validate.rs`; `crates/mesh/tests/reach_probes.rs`; the rows `work/tesslane/a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md`, `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem.md`, `rectangle-faces-move-to-the-chart-lane.md`, `a-boundary-run-on-a-chart-iso-line-gets-a-lattice-line.md`.

## How to work

- Your tree is a git worktree at PR 7043's head; the path and your own build directory are in your dispatch. Build and probe there freely (`cargo ... -p mesh`, with `CARGO_TARGET_DIR` set as given); change nothing outside it, commit nothing, push nothing. Disk is short: keep one build profile and delete build output when you are done.
- Read no pull request's comment thread and no branch under `analysis/`. Other designers are weighing this same statement now; you will not see their reports until yours is in.
- Your report is your final message, in the two sections `designer.md` §5 asks for, at most 150 lines, naming no model and no lane.
