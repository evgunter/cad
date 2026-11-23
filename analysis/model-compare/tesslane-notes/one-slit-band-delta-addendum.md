DELTA ADDENDUM — PR 7498 @ 12614f22f1 (the sizing.rs docs-only commit).

**Verdict unchanged: APPROVE-WITH-FIXES.** It is still conditional on hosted CI. The PR's check runs show "change filter" queued and "prime the build cache" skipped. No test job has run.

## The two new sentences
1. **`sphere_grid_steps`: "the argument is Delaunay's, so it is read in that plane; what it concludes, a triangle inside one cell, is the same in every plane, since the scale between them is axis-aligned and affine". TRUE.** An axis-aligned positive scale maps lattice cells to lattice cells, so "this triangle's UV box lies in one cell" is plane-invariant. Which triangles Delaunay MAKES is not, and the sentence correctly places the premise (aspect under 2√2) in the CDT's plane.
2. **`SPHERE_CELL_ASPECT`: "the trimmed lane's sphere arm lays these steps and triangulates in the chart's metric at mid latitude ... so there the raw aspect still bounds the lemma's premise". FALSE.** That lane's CDT does not run in raw UV, so the raw aspect bounds nothing there. The other half ("past 45° it sets the steps themselves") is true.

## The trimmed lane's sphere arm has the same gap, on main (a finding against main, for you to file)
Its lattice is sized by `sphere_grid_steps`: raw aspect at most √2 nominally, 0.94–2.65 realized. It is triangulated in `trimmed_metric`'s sphere plane, k = 2^round(log₂(1/cos mid-lat)). Measured, not inferred:

- **Mesh suite at 12614f22f1** (local trace on the trimmed lane's metric line, reverted since): 153 trimmed sphere faces, all near the equator (raw aspect 0.94–1.00). 0 of 152 with interior rows and columns fall outside the premise. So the suite never reaches the case.
- **My own bodies:** a ball posed at tilts 0.03, 0.1 and 0.2 rad about z, intersected (`topo::intersect`) with a slab y > sin(lat) for lat 55, 65, 72, 78, 82 and 85°. That gives polar caps trimmed by an oblique circle, which go to the trimmed lane. Meshed at 5 δ from 3e-2 to 3e-4: 90 meshes.
  - The trimmed lane's CDT-plane aspect reaches **0.093** (k = 16, raw 1.49, v from 1.45 to 1.57). That is cells 10.7× taller than wide, against the lemma's bound of 2.83.
  - Every cap at lat ≥ 78° sits at k = 8 or 16, with CDT aspect 0.09 to 0.18. `cell_metric` would have chosen k = 1 or 2 there.
- **Consequence on those 90:** none visible. All 90 mesh and are watertight, with sampled worst 0.54 δ under main's metric.
  - With `cell_metric` swapped into the trimmed lane (a local toggle, reverted), they also all mesh, worst 0.82 δ. That is not better: the shared-side cure is tuned to the chart metric, as its doc says.
  - So this is a latent premise gap, not a demonstrated defect. Severity: P3 or P4 as a doc and premise row.
  - The trimmed lane's oblique trims are off-lattice everywhere anyway, and what holds them is its certificate, under the open `one-containment-lemma-for-an-oblique-boundary`. The sphere lemma is not load-bearing there.

**Suggested filing (against main, not this unit):** "the trimmed lane's sphere arm lays a lattice sized for a raw-UV Delaunay lemma (`SPHERE_CELL_ASPECT`) and triangulates it in the chart metric at mid latitude, where cells reach 10.7:1 at lat ≥ 78° (k 8–16); no refusal or deviation found on 90 oblique-trimmed polar caps". Cite `sizing.rs` `SPHERE_CELL_ASPECT` and `chart_table.rs` `trimmed_metric`. On this PR, the cheap fix is to delete the false clause "so there the raw aspect still bounds the lemma's premise", or replace it with what is true: the trimmed lane reads the cap only for its steps, and its CDT plane is not the lemma's.

**Executed:** the trimmed-lane trace over the whole mesh suite (309 pass), the polar-cap probe under two metrics, and the check runs. Everything was reverted and the tree is clean.
