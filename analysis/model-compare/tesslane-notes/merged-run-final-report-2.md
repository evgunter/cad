MERGED-RUN lane — FINAL REPORT, part 2 of 2: 3. THE PR BODY, ready to paste, between the two lines of =====.

=====
# The chart lane's cone arm meshes a run of ruling points its development merges

## The limit

A cone face on the chart lane is triangulated in the cone's development, which cuts a point's slant radius to 27 significant bits so that every point of one ruling lies on one exact line through the apex. Two points of one ruling within one 27-bit bin of the slant radius are therefore ONE point of the development. The bin is a length, up to `2⁻²⁷·r / sin α` along the ruling: `8.5·10⁻⁵` on a cone of half-angle 0.01° and radius 2. The face refuses `SphereTrimShape { DevelopedMerge }`.

Since every cone face with a split ruling is the chart lane's, that is a refusal at the default ε for any draft-angle cone whose ruling is split near an end, or twice close together.

## The cure

A **run** is a set of consecutive points of one loop of the trim polygon, along one ruling, that the development puts at one point (`trimmed::Run`, `trimmed::developed_runs`).

- **The triangulation holds one vertex for the run.** `insert_points` enters the run in the boundary walk once, so its vertex is not read as a pinch; `constrain_boundary` already skips a side of no length.
- **The emit pass tells the points apart again.** The run is anchored by its neighbour along the ruling. In the CDT triangle on the polygon side to that neighbour the run's vertex is the run's point next to the neighbour; in every other triangle of the vertex's fan it is the run's far end. The triangles between the run's points are added: each on two consecutive points of the run and the third corner of that anchored triangle, wound as the triangle it is cut from. The fan about the one vertex becomes a fan about each point, and the side between two points of the run is a side of exactly one triangle, as a boundary side is.
- **Every triangle, the added ones included, goes through the same code as before**: the duplicate-id drop, the shared-side rule, `resolved_normal`, the outward test, the certificate. An added triangle has no inside where the CDT runs, so one that the shared-side rule flags queues the point of the CDT triangle it is cut from, once.
- **A run is read from the polygon's own order and its sides' carriers, never from a distance**: every polygon point at that CDT point, consecutive on one loop and not all of it; no singular (apex) vertex and no mesh vertex twice; each side between two of them carried by a line (on a cone, a ruling), `v` strictly monotone along it; its two neighbours at two CDT points; and the side to at least one neighbour carried by a line too. Any other merge refuses `DevelopedMerge` as before.
- **The rule "all points of one ruling on one exact line" is untouched**: `developed`, `significant` and `cdt_point` do not change.

No face without a merged run enters the new path: `developed_runs` answers an empty list, `insert_points` and `emit` read `runs.is_empty()`.

### Where this differs from the unit's spec

The spec's quoted analysis was made by reading. Three things in it did not hold on the tree, and one case it did not have:

1. **"a, P, b are collinear in the development."** Not for the commonest cell. A split near an END of a ruling merges with the ruling's end vertex, which is a corner of the polygon: one neighbour is on the ruling, the other on a rim. So the anchor is chosen: the neighbour along the ruling, the first of the two inside a ruling side. On that side the anchored triangle has two corners on the ruling, so its third is off it.
2. **"The run is one chart column."** A ruling's far end vertex is read through its RIM's image: measured `u = 0.0` against the ruling's `u = −1.2246467991473532·10⁻¹⁶`. Recognising the neighbour by column left 153 of the 1296 default-ε cells refusing. The run and its anchor are read from the polygon sides' carriers instead (`trimmed::carried_by_a_line`). Filed: `a-rulings-end-vertex-read-through-its-rims-image-is-off-the-rulings-exact-line`.
3. **The run's vertex is a pinch to `planar::Pinches`** (one handle, two mesh ids) and would refuse `PinchWedge`: a run's later points are counted by the walk and recorded at no handle (`Walk::skip`).
4. **Two runs anchored by the apex in one triangle** (a thin apex wedge with both rulings split by their rim ends) have no one third corner. The triangle is cut along both: the first run's triangles fan about the other run's far end, the other's about the first's point next to the apex.

Where the spec's literal rule (the anchor always the neighbour before) is put back as a mutant, no integration cell at the default ε reds; the unit rows and the apex wedge do. The anchor choice is held by exactness and by those rows, not by a measured failure of the literal rule at the default ε.

## What moves

**Faces that refused `DevelopedMerge` and now mesh.** Measured on the spec's bodies (the wall `(1, 0), (2 + tan α, 0), (2, 1), (1, 1)` and the solid `(0, 0), (2 + tan α, 0), (2, 1), (0, 1)`; full and half turns; α 0.01°, 0.1°, 1°, 3°, 26.6°; each line edge split in turn beside its start, beside its end, twice and three times at its middle, gaps `10⁻⁴` to `10⁻¹⁰`; placed obliquely; δ 0.1, 0.01, 0.001), cell by cell, base against head:

| ε | cells | `DevelopedMerge` → `Meshed` | `Meshed` → `Meshed` | Plane `UnresolvedWinding` → the same | not built |
|---|---|---|---|---|---|
| default | 7480 | 1296 | 5337 | 27 | 820 (`split_edge` refuses the split) |
| `10⁻¹²` | 7480 | 1479 (963 through the whole oracle; 516 on the 0.1° bodies, whose volume is not computable at that ε, tessellate) | 5517 (1248 with the volume unread) | 300 | 4 (the 0.01° bodies do not revolve) |

`Meshed` is `reach_probes::got_enclosing`: `check_mesh`, `check_winding`, every triangle of every analytic face outward and sampled within δ, and the body's volume to the chordal deficit. No cell goes from meshing to anything else. At ε `10⁻¹²` with the gap `10⁻¹¹` added (the smallest `split_edge` admits there): 769 mesh, 143 are the planar neighbour's refusal, none is the cone's.

**Faces that meshed: no byte moved.** Shown by a digest of every patch (FNV over each triangle's corner position bits, per face, as a multiset), base `0700268f2a` against head, with a temporary hook at the join in `tessellate`:

| suite | base patches | missing on head | new on head |
|---|---|---|---|
| `mesh`, whole, default profile with the slow set | 49,728 | 0 | 1,084 (the new rows' cells) |
| `sweep`, the 64 modules that name `mesh::` or `tessellate` (584 tests) | 31,646 | 0 of 31,350 | 0 |
| `step-import`, whole | 108 | 0 | 0 |

`sweep`'s other 296 patches (Plane and Cylinder faces, same triangle counts) differ between the base run and the head run, and differ in exactly the same way between two runs of the identical head: a row that varies its seed per run, on lanes this change does not touch. They are not evidence either way and are stated as such.

**How many faces took the new path** (a temporary counter beside `developed_runs`): `mesh` suite 268, all in the rows this PR adds; `sweep`'s modules 0; `step-import` 0; `demos/tour` (its 106 tests, the tour at three ε among them) 0, and 0 refusing. On the base, the `mesh` suite at the default ε, `sweep`'s modules and `step-import` had no face refusing `DevelopedMerge` at all: the class was reached by the review's bodies, not by a row.

## The thin triangle

A triangle between two points of a run has the run's gap for its base. Measured over the spec's bodies at the smallest gap each ε admits (every added triangle, a temporary probe in `emit`):

| ε | cone, gap | added triangles | certificate | winding: least height over allowance | outward |
|---|---|---|---|---|---|
| default | 0.01° and 1°, `10⁻⁸`; 0.01°, `10⁻⁵` | 423 | all `Certified`, at most 0.496 δ | 1.6·10⁵ (`1.74·10⁻⁹` against `1.07·10⁻¹⁴`) | all |
| `10⁻⁶` | 0.01°, `10⁻⁵` | 141 | all `Certified`, at most 0.496 δ | 1.6·10⁸ | all |
| `10⁻¹²` | 0.1°, 1°, 26.6°, `10⁻¹¹` | 423 | all `Certified`, at most 0.497 δ | 163 (`1.74·10⁻¹²` against `1.07·10⁻¹⁴`) | all |

None refuses `UnresolvedWinding` or `FoldedTriangle` on those bodies. One does elsewhere, at ε `10⁻¹²` only, and is pinned and filed (below).

## What still refuses

- **`DevelopedMerge`, a merge that is no run.** A point merged with the apex, and points of two sides or of two loops: unit rows (`a_run_is_read_from_the_polygons_order`); no body builds either (a point reaches the apex's CDT point only under spade's floor, `2⁻¹⁴²`). A ruling side wholly inside the cut: `split_rulings::a_cone_band_shorter_than_the_developments_resolution_refuses` (ε `10⁻¹²`; the band is not built at a coarser ε).
- **`UnresolvedWinding` on the cone, ε `10⁻¹²` only: a run anchored on a needle.** The cone `(0, 0), (1, 0), (0, 1)` through its apex, a ruling split `10⁻¹⁰` from its rim end, δ 0.001: the run's neighbour along the ruling is the apex, the triangle on that side takes a lattice node hugging the apex for its third corner, and the added triangle reads height `5.15·10⁻¹⁵` against allowance `9.14·10⁻¹⁵`. 11 of 12 such cells probed at δ 0.001; δ 0.1 and 0.01 mesh. Typed, never a wrong mesh; before this PR the same cells refused `DevelopedMerge` at every δ. Pinned (`a_run_anchored_on_a_needle_triangle_refuses_unresolved`) and filed with the cure seen (cut the fan along the edge standing most squarely on the ruling, a choice read from developed coordinates): `a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind`.
- **`UnresolvedWinding` on a PLANAR neighbour**, on the base and the head alike: three splits of one straight edge `10⁻⁸` apart, and the planar face holding the edge makes a triangle of the three. 27 cells at the default ε, 300 at `10⁻¹²`. The planar lane's; filed, not touched: `a-planar-face-makes-a-triangle-of-three-close-points-of-one-straight-edge`.
- `split_rulings::a_ruling_split_a_hair_from_its_end_folds_a_triangle` (the neighbour row) answers as before: its split is `3·10⁻⁷` on a 20° cone, which does not merge.

## Rows

`crates/mesh/tests/split_rulings.rs` (three ε; each says what its ε builds):

- `a_run_of_ruling_points_the_development_merges_meshes`: the wall a full turn (one face, the seam walked on both rays), 0.01° and 1°, five split shapes (beside the start, beside the end, two, THREE, both ends), δ 0.01.
- `every_run_of_a_draft_cones_rulings_meshes` (slow set, 4.2 s): the grid, four bodies, three δ, 210 cells at the default ε.
- `a_draft_cone_ruling_split_within_the_developments_resolution_meshes` and `a_frustum_seam_split_within_the_developments_resolution_meshes`: PR 7049's two pins, as meshing rows.
- `two_runs_anchored_by_the_apex_mesh` (ε `10⁻¹²`).
- `a_cone_band_shorter_than_the_developments_resolution_refuses`, `a_run_anchored_on_a_needle_triangle_refuses_unresolved`: the two pins above.

`crates/mesh/src/trimmed.rs`, `development_tests`:

- `a_run_is_read_from_the_polygons_order`: hand polygons, six runs (inside a side, of three, across the polygon's first point, starting and ending at a corner, from the apex) and nine refusals.
- `a_split_seams_close_points_are_a_run_on_each_ray`: the runs read off a real split seam: one run of three on each ray with the same vertices in opposite orders; at a corner, one anchored before and one after.

The spec's row "a run next to the apex that must still refuse (a split merged with the apex)" has no body: `split_edge` admits no split within `2⁻¹⁴²` of an apex. The unit row holds it.

## Mutants

Each put in, run against the fast rows at the default ε and `10⁻¹²`, and taken out:

| mutant | red |
|---|---|
| the re-split removed (every corner the run's first point, nothing added) | `a_run_of_ruling_points_…_meshes`, `a_draft_cone_…_meshes` (default); those, the frustum row, the needle pin and `two_runs_…` (`10⁻¹²`) |
| the added triangles dropped | the same |
| every fan triangle given the anchored point | `a_run_of_ruling_points_…_meshes`, `a_draft_cone_…_meshes`; the frustum row and the needle pin at `10⁻¹²` |
| the apex exclusion removed | `a_run_is_read_from_the_polygons_order` |
| a non-run merge admitted: a run with no neighbour along its ruling | `a_run_is_read_from_the_polygons_order` |
| … sides between its points needing no line | the same |
| … order along the ruling not read | the same |
| … its two neighbours at one CDT point | the same |
| the anchor always the neighbour before | `a_run_is_read_…`, `a_split_seams_…`; `two_runs_…` at `10⁻¹²` |
| two runs in one triangle fanned as one | `two_runs_anchored_by_the_apex_mesh` (`10⁻¹²`) |
| a run's later points entered in the walk | every meshing row (`PinchWedge`) |

## The sweep (implementer-discipline §5)

**The class**: a place where a lane's CDT holds two mesh vertices at one location and refuses or drops.

**Shape grepped**, over `crates/mesh/src`: the duplicate-id drop (`ids[0] == ids[1]`), position-keyed dedup (`to_bits()` keys, `index_of`, `cdt.insert`, `bulk_load_cdt`), `ids_at_two_uvs` and its readers, the walk and pinch tables (`walk.enter`, `Pinches::of`, `id_in`), and the zero-length side skip (`if a == b`).

| hit | disposition |
|---|---|
| `trimmed::TrimmedFace::insert_points`, the development | fixed: a run is one vertex and is re-split; any other merge refuses |
| the same, in the chart (lattice metric) | not this class: the map is injective on floored points, so two ids at one vertex are one UV location, a pinch, which `Pinches` reads |
| `trimmed::TrimmedFace::emit`'s duplicate-id drop (the apex row's and a pole side's fan; a seam's two copies in a narrow development) | not this unit: one mesh vertex at two locations, the inverse shape; the added triangles go through the same drop, and the seam rows mesh through it |
| `trimmed::singular_row_shape`'s `SeamTwice` (the sphere arm's pole, a seam twice on a lattice arm) | not this unit: one vertex, two locations |
| `curved`'s `insert` (two ids at one handle refuse `PinchWedge`) | not this unit: its chart does not round; typed refusal for truly coincident points |
| `planar::triangulate` with `Pinches`, and its `a == b` skip | not this unit: the chart frame is an isometry in f64 with no cut; two ids share a handle only within rounding of each other, below any gap `split_edge` admits |
| `ids_at_two_uvs` readers (`curved`'s and `trimmed`'s census, `id_repeats_apart`) | unchanged: they read ids at two UV locations, which a run's points are not (a seam's are, as before) |

**What the pattern could not match**: a non-injective map applied to coordinates on their way into a CDT that is not spelled as a dedup; and a CDT outside `crates/mesh`.

**Second pass, at that gap**: every map applied before a CDT insert (`cdt::spade_point`, `trimmed::cdt_point`, `developed`, `significant`, the lattice metric's scale) and every `spade::` user in the repository. One more non-injective map: `cdt::spade_point`'s floor, which sends `(0, 2⁻¹⁴²)` to `0.0` on all three lanes. Two ids it collapses are one location by the lanes' own definition (`ids_at_two_uvs` reads the same floor): `curved` refuses `PinchWedge`, `planar` and the chart read a pinch. Not measured further: no door builds two vertices that close. No CDT outside `mesh`.

## Filed

- `a-run-anchored-on-a-needle-triangle-makes-triangles-f64-cannot-wind` (P3)
- `a-planar-face-makes-a-triangle-of-three-close-points-of-one-straight-edge` (P3)
- `a-rulings-end-vertex-read-through-its-rims-image-is-off-the-rulings-exact-line` (P3)
- closed: `a-ruling-split-within-the-developments-resolution-merges-two-points`, with what the cure reaches and leaves.

`work/tesslane/a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it.md` still lists the merge among the residuals that unit "found and filed"; that sentence is that unit's record and is left.
=====
