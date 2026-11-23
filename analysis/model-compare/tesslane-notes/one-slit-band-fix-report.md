TESSLANE one-slit-band — FIX REPORT. Head a5741dec87 (confirmed with ls-remote).

COMMITS SINCE THE FROZEN HEAD 2152cd3463
- d0e59643b8: item 1 (the MAJOR).
- 2c8769ed04: items 2–7, 9 and 10; the filed and corrected rows.
- c9505dda6c: merge of main. It does not compile on its own: main made `Circle::frame` a method. My chain pushed it before reading the check.
- a5741dec87: fixes that one call site.
None of my commits carries a trailer.

LOCAL RUNS ON a5741dec87
- mesh, whole suite with the slow set (`--ignore-default-filter`): 551/551 at ε 1e-9, 1e-6 and 1e-12. 2c8769ed04 was likewise 536/536 at all three.
- clippy (mesh, --all-features) clean; fmt clean; `scripts/run-gates.sh` clean; `work.py lint` ok.
- viewer and sweep: not re-run after d0e59643b8. Nothing since then changes a mesh: the later commits are docs, tests and the main merge. At d0e59643b8: review_pick_r2 read 147,844 with the same two documents, display_budget was green, and sweep's 65 modules were 601/602 (the red is main's bisector_nodes).
- CI is left to the gate.

ITEMS
1. Done (d0e59643b8; see my earlier message). Your premise "k = 1 on spheres" is false, as measured. The rule is implemented as ruled.

2. Done, with one disagreement.
   Whole-mesh-suite differential: every curved patch is logged as (kind, nu, nv, triangle count, FNV of its corners), base = metric 1, head = the merged-equivalent head. 17,584 patches.
   - torus: 2,880 of 2,916 move;
   - spindle: 16 of 41 move (k = 2; includes the one-face spindle);
   - sphere: 238 of 3,081 move (k = 2 or ½);
   - cone: 0 of 3,976;
   - cylinder: 0 of 7,570.
   Every moved patch keeps its triangle count. The base logs 31 fewer patches only because the rows that pin the cure panic there.
   **Cylinders:** I cannot reproduce "25 cylinder patches move". Separately I ran the whole suite with ONLY the cylinder metric reverted, so every body meshes on both sides: all 7,570 curved cylinder patches are byte-identical, across k from 2⁻⁹ to 2¹⁴, including the filleted plates' hole walls (93 patches in the rows and goldens, k 1/16 to ½). If the reviewer has a body where one moves, I need it. Why a moved strip would still be right: `sizing::sagitta_step`'s strip argument reads the far row only and holds in any axis scaling (the module doc now says so).

3. Done.
   - `a_revolved_band_meshes_with_its_meridian_points_between_rows` covers a whole turn plus partial turns of π/2 and 2 radians, at hole 0.6 / r 5e-3, δ 3e-3 and 1e-3. All three build at every ε and refuse in raw UV at those δ (also measured at θ = π and 1). A partial turn has two meridian edges and no slit.
   - The module doc and the test-file header now name both shapes.
   - The knife-edge row says the review saw no trigger on a meridian arc of a turn other than a quarter.

4. Done.
   - `assert_band` pins `u ≤ 1` column and `v ≤ max(1, nv/m)` rows, with nv and m read from each band's own meridian polylines and lattice.
   - The containment row is corrected: 1.5 = nv/m for 2 chords on 3 rows, and the review's synthetic check is cited.

5. Done.
   - Module doc: "7.7 δ on PROFILE's filleted plate (R 0.305, r 0.005, δ 1e-3)".
   - Containment row: this unit's sweep (498 bands: 42 refused raw, up to 83 δ; ≤ 0.89 δ sampled), and the review's (720: 32 refused, up to 281 δ; certificate 0.99 δ, sampled 0.927 δ), each named.
   - Knife-edge row: 7.7 δ (PROFILE) and 83 δ (this unit's sweep), with their corpora.

6. Done.
   - The sphere-zone row is deleted. `curved::tests::the_cdt_cell_keeps_the_containment_lemmas_aspect_on_every_chart` is the sphere's row.
   - The two-arcs row's doc now says it witnesses that the mechanism exists on the short axis and is harmless there (raw UV meshes it too, 60 chords against 59 columns at δ 1e-3), and names what reds it: reach past one column, or deviation past δ.

7. Done.
   - The KernelLimit path is gone. The revolved bodies build at all three ε.
   - The ε byte-identity is now two corpus entries in `d9_mesh_goldens` (`filleted_hole_three_arcs`, `filleted_hole_five_arcs`): its one table is read at every ε row, and no new FNV is added.
   - The shell row keeps the r = 5e-4 refusal.

8. Kept. The `work/green` file is ON main now (7096 merged), so the citation resolves.

9. For the body (text below).

10. Done.
   (a) Names: `cell_metric` (the lattice's own, every chart, the curved lane's) and `trimmed_metric` (the trimmed lane's choice, the sphere at mid latitude), with docs saying which. Lemma sweep:
   - `sphere_grid_steps`' lemma is restated "in the plane the CDT runs in";
   - `SPHERE_CELL_ASPECT`'s doc says the cap was for raw UV, filed P4 below;
   - `sagitta_step`'s strip argument already says "any axis scaling", and holds;
   - `cone_grid_step` cites that strip argument;
   - `pole_columns`: order and equidistance along u, kept by a per-axis scale, and re-derived by the emit census.
   The curved module doc says all this in one paragraph.
   (b) My FNV is removed. Filed P4 `mesh-tests-carry-eight-private-fnv-digests` (the eight files).
   (c) The literals are now `CHART_READ` (named and argued). The slit is read from the mesh's boundary polylines (meridian = a polyline on the band whose v varies), not `u ≈ 0`.

FILED
- New: `the-sphere-cell-aspect-cap-was-chosen-for-a-raw-uv-cdt` (P4), `mesh-tests-carry-eight-private-fnv-digests` (P4).
- Corrected: `the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound`, `a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge`.

PR BODY (replaces the earlier one)
=====
**The curved lane triangulates in its own lattice's metric, so a thin fillet band meshes where a meridian chord point sits between its rows.**

## The defect

A hole's rim rounded on a plate gives a band of thin torus, one face. Revolved a whole turn, or filleted, the face has one slit; revolved part of a turn, it has two meridian edges and no slit. The curved lane meshes it on a lattice of `nu × nv` cells and certifies every triangle, sizing at δ on its theorem about lattice half cells (`[mesh.sizing]`). Two facts break that:

1. **A meridian's chord points can fall between the band's rows.** A meridian is a quarter arc of the tube. Its own chord count, `ceil(span / sagitta_step)`, uses a step capped at `π/4` wherever δ/r ≳ 0.17, so the count sits on the ceiling's edge. On PROFILE's tree the slit's stored span is π/2 + 5 ulp: 3 chords where the face lays 2 rows. With main's fillet door on a three-arc hole it is the mirror: 2 chords, 3 rows.
2. **The lane's CDT ran in raw UV.** There the thin tube's cells are about √(R/r) times taller than wide. A point a fraction t up a cell had an empty circle √(t(1−t))·a columns wide, and joined rim points four columns away: 7.7 δ on PROFILE's filleted plate (R 0.305, r 0.005, δ 1e-3).

## The cure

`tessellate_curved` inserts every point at `crate::cdt::lattice_point(u, v, k)`, with `k = curved::cdt_metric(..) = AnalyticRow::cell_metric(..)`: the lattice's realized cell aspect `(Δu/nu)/(Δv/nv)` to the nearest power of two, on every chart. The exact growing-axis scale is the one `trimmed::cdt_point` also calls. `AnalyticRow::metric` is split in two:
- `cell_metric`, the lattice's own, which the curved lane runs in;
- `trimmed_metric`, the trimmed lane's choice (the sphere's chart metric at mid latitude, else `cell_metric`), unchanged.

The curved lane never reads the trimmed lane's sphere choice: that is no lattice's spacing, 8 to 32 times one near a pole. In the curved lane's plane every cell's aspect is within √2 of one, inside the premise of `sphere_grid_steps`' containment lemma (under 2√2 on either axis). No ε is read; steps, counts and certificates are unchanged.

The lane's other arguments hold in that plane:
- `sizing::sagitta_step`'s strip argument (ruled charts) is stated for any axis scaling;
- `pole_columns`' separation of the two pole entries rests on order and equidistance along u, which a per-axis scale keeps, and the emit census re-derives it on every patch;
- `sphere_grid_steps`' lemma is restated in the CDT's plane.

## The class

Every curved torus face whose meridian chord count differs from its row count: a whole turn's slit, or a partial turn's two meridians. The measured trigger is the capped step's ceiling on a quarter arc. A rim carried by several edges (a two-arc hole's fillet foot, 60 chords on 59 columns) puts points between columns on the short axis, where raw UV did not bite either. The review's sweep found no trigger on a meridian arc of a turn other than a quarter.

The spec named `a-quarter-arc-swept-annulus-exceeds-its-triangle-certificate` as possibly the same cause. It is not. That row's refusing faces are NURBS walls of a swept annulus. They go to the trimmed lane, which already ran in its lattice metric before this change; this cause is the curved lane's raw-UV CDT. Checked: the row's faces are spline charts, which `tessellate::route_of` sends to the trimmed lane unconditionally, and the trimmed lane's `cdt_point` scaled every arm but the cone's already.

## What is measured and not proved

A triangle beside an off-lattice chord point is no half cell, and no closed form bounds it; the certificate holds it. Measured, it spans at most one column and at most `max(1, nv/m)` row gaps, nv the rows and m the meridian's chords. A meridian chord itself spans nv/m gaps (the review checked the rule on a synthetic CDT, m 1–10, nv 1–8). The two sweeps:

| sweep | bands | raw UV refuses | worst raw | metric refuses | worst certificate | worst sampled |
|---|---|---|---|---|---|---|
| this unit's | 498 | 42 | 83 δ | 0 | — | 0.89 δ |
| the review's | 720 | 32 | 281 δ | 0 | 0.99 δ | 0.927 δ |

On PROFILE's plate (161 cells), the triangles beside off-row points tie the worst half cell to within 2 ulp. Filed: `the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound`. No headroom is added for it.

## Bytes that move

Whole `mesh` suite, every curved patch digested (corners' bits), base (metric 1) against head, 17,584 patches:

| family | patches | moved | triangle counts |
|---|---|---|---|
| torus | 2,916 | 2,880 | equal |
| spindle (incl. `the_one_face_ball`'s spindle) | 41 | 16 (k = 2) | equal |
| sphere | 3,081 | 238 (k = 2 or ½) | equal |
| cone | 3,976 | 0 | — |
| cylinder | 7,570 | 0 (also checked with only the cylinder metric reverted, k 2⁻⁹ to 2¹⁴: all byte-identical) | — |

Only diagonals move. The lattice and boundary points are the same; a lattice cell's four corners are cocircular, so either diagonal is Delaunay, and which one is taken follows the metric. Every moved patch certifies at δ. Sphere faces move where the counts' ceilings leave the realized aspect at √2 or past it, so k rounds to 2 or ½. Their raw cells already met the lemma's premise (aspect ≤ 2.4 with rows, measured), and in the CDT's plane they are nearer square.

Re-pinned, each confirmed by a base run that reads exactly the old constant:
- `d9_mesh_goldens`: donut δ 0.05 `10814893347bbbdd` → `acbaea8ec424d559`; δ 0.15 `7628fd943b85d278` → `0a73f71aa4fa1618`; sphere_wedge δ 0.15 `70f42f79af650fee` → `d0e17c1a5ff99fee`.
- `route_rows` donut: `0x10e7f0a246fd3c81` → `0x72785776b24f0ce5`.
- viewer `review_pick_r2`, aimed-vertex count: base 147,850, head 147,844. This change moves six: `tube_arc` after its first edit 11901 → 11898, and `hollow_tube_elbow` open 7052 → 7049, both torus patches. Pinned to 147,844, and the comment names both parts. The earlier six (147,856 → 147,850) are main's and still unexplained. They stay owed on `work/green/main-red-the-viewers-pick-tally-reads-six-grazes-short-of-its-pin.md` (landed with PR 7096), though this re-pin turns the test green.

Unmoved: viewer `display_budget`; sweep's 65 tessellating modules (601/602 green; the red is main's `bisector_nodes::the_near_node_poses_at_the_witness_band`).

## Rows

`crates/mesh/tests/the_one_slit_band.rs`. Each row asserts that the band meshes and is watertight, samples within δ of the torus, reaches at most one column and `max(1, nv/m)` rows (read from the band's own counts), and carries the off-lattice point it is there for.
- `a_revolved_band_meshes_with_its_meridian_points_between_rows`: a whole turn, a quarter turn and two radians of a quarter arc of r 5e-3 on a hole of 0.6, at δ 3e-3 and 1e-3 (raw UV refused all six). Builds at every ε.
- `a_filleted_holes_one_slit_band_meshes_with_its_slit_points_between_rows`: `sweep::blend::fillet_edges` on holes of three arcs (r 1e-3; δ 1e-3, 3e-4) and five (r 5e-3; δ 3e-3, 1e-3).
- `a_band_whose_foot_is_two_arcs_meshes_with_its_rim_points_between_columns`: the rim source on the short axis, a witness that it is harmless there (raw UV meshes it too).

Also:
- `curved::tests::the_cdt_cell_keeps_the_containment_lemmas_aspect_on_every_chart`: through the lane's own `grid_counts` and `cdt_metric`, over sphere zones and caps off the equator, the whole sphere and a thin torus band, at δ 1e-1 to 1e-4. Every cell enters the CDT at aspect in (1/2√2, 2√2).
- `d9_mesh_goldens` gains the two filleted holes. Its one table is read at every ε row, which pins their bytes identical at ε 1e-9, 1e-6 and 1e-12.

## Mutants

| mutant | caught by |
|---|---|
| curved metric = 1 (raw UV) | the revolved and filleted-holes rows, d9's filleted holes, the aspect row |
| curved reads `trimmed_metric` | the aspect row |
| curved metric = 1/k | the revolved and filleted-holes rows, d9 |
| `lattice_point` ignores k (both lanes) | those, plus `trimmed::tests::cdt_point_floors_then_scales_the_growing_axis` |
| `lattice_point` always scales u by 1/k | `cdt_point_floors_then_scales_the_growing_axis` |

## §5 sweep

Pattern: every CDT insertion in `crates/mesh/src` (`cdt.insert(`, `bulk_load_cdt`, `spade_point(`).
- curved: fixed here.
- trimmed: through `cdt_point` (lattice metric, or the cone's development).
- planar: the plane's own orthonormal chart, with no lattice, so not this class.
- The rest are test modules.

The pattern cannot see a chart whose metric varies across a face (a sphere's 1/cos v). Sphere zones and caps up to 89° latitude are covered by the aspect row and the whole-suite differential. On the other charts: a curved cylinder or cone lays no rows and the walk refuses a chord point on a ruled side, so neither can hold an off-row point. A sphere or spindle can, and is in the differential above.

## Filed

- `the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound` (P2).
- `a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge` (P3).
- `the-sphere-cell-aspect-cap-was-chosen-for-a-raw-uv-cdt` (P4): the curved lane's plane holds the lemma's premise at any raw aspect, so the √2 cap may lift.
- `mesh-tests-carry-eight-private-fnv-digests` (P4).
- Evidence on `work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`: the r = 5e-4 revolve's KernelLimit at ε 1e-12.
=====
