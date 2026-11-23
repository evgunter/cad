**The curved lane triangulates in the lattice metric, so a one-slit fillet band meshes where a chord point sits between its rows.**

## The defect

A hole's rim filleted on a plate gives a band of torus that is one face, a whole turn round with one slit (the revolve's seam, or the fillet's join of the hole's arcs). The curved lane meshes it on a lattice of `nu × nv` cells and certifies every triangle against δ, sizing at δ on its theorem about lattice half cells (`[mesh.sizing]`). Two facts combine to break that:

1. **The slit's chord points can fall between the band's rows.** The slit is a quarter arc of the tube. Its own chord count, `ceil(span / sagitta_step)`, uses a step capped at `MAX_ANGULAR_STEP = π/4` wherever δ/r ≳ 0.17, so the count sits on the ceiling's edge. On PROFILE's tree the slit's stored span is π/2 + 5 ulp, giving 3 chords where the face's box lays 2 rows. With main's fillet door on a three-arc hole it is the mirror case: 2 chords and 3 rows.
2. **The lane's CDT ran in raw UV.** A thin tube's lattice is about √(R/r) times taller than wide there. A point a fraction t up a cell had an empty circle √(t(1−t))·a columns wide, so it joined rim points up to four columns away. On PROFILE's plate (R 0.305, r 0.005, δ 1e-3) that read 7.72e-3; over the bands below, up to 83 × δ.

## The cure

`tessellate_curved` inserts every point at `crate::cdt::lattice_point(u, v, k)`, with `k = AnalyticRow::metric(..)` (a power of two near the lattice's aspect). These are the same function and the same exact growing-axis scale the trimmed lane's analytic arms use; the scaling now has one home, which `trimmed::cdt_point` calls too. No CDT in the crate runs in raw UV. Nothing reads ε, and the steps, counts and certificates are unchanged.

## What is measured and not proved

A triangle beside an off-lattice chord point is no lattice half cell, and no closed form bounds it; the certificate holds it. Measured on main over 498 one-slit bands, 104 of which hold a slit point between rows:
- every band triangle spans at most 1 column in u and 1.5 row gaps in v (1.0000000000000981 and 1.5000000000001397);
- every band triangle samples within 0.89 δ of the torus;
- none refuses. In raw UV, 42 refuse.

The bands: a revolved section, plus the fillet door on holes of 2, 3 and 5 arcs; holes 0.05, 0.3, 0.9; r from 5e-4 to 5e-2; δ from 3e-2 to 1e-4.

On PROFILE's plate (161 cells), the worst certificate among triangles beside the off-row points ties the worst lattice half cell to within 2 ulp and never exceeds it. The gap is filed (below); no headroom is added for it.

## Bytes that move, and why the new mesh is right

Patch digests (FNV over each triangle's corner bits), base against head: 22 bodies × 6 δ (0.15 to 3e-4); 520 patches at base, 528 at head. By family:

| family | patches | moved | triangle counts |
|---|---|---|---|
| curved torus: `donut` (2 patches), revolved one-slit bands r 0.0005/0.002/0.02/0.05 | 28 | all 28 | equal in every one |
| revolved band r = 0.0005 at δ 1e-3, 3e-4 | — | refused `CertificateExceeded` at base, meshes at head | — |
| planes, cylinders, cones (`l_prism`, `holed_prism`, `rounded_prism`, `washer`, `cone`, `cone_wedge`, `wedge`, `axis_wedge`) | all | none | — |
| spheres with symmetric windows (`ball`, `sphere_wedge` ×3; metric 1) | all | none | — |
| five witness bodies that refuse the same way at base and head (`keyway` and `oblique_lens` DomainUnread, `slit` UnsupportedCurvedDomain, ...) | — | unchanged | — |

Off-equator curved sphere faces move too. In the sphere zones 50°–80°, −80°–−50° and 60°–88° (metric 2), every patch at δ ≥ 1e-2 moves with equal triangle counts and an equal sampled worst deviation (0.46–0.94 δ). The zone at 20°–70° moves at two δ only.

Only diagonals move: the lattice and the boundary are the same points, and a rectangle's four corners are cocircular, so either diagonal is Delaunay. Which one is taken now follows the metric. Every moved patch still certifies at δ.

Re-pinned, each confirmed by a base run (this change's metric line reverted, every other byte the same) that reads exactly the old constant:
- `d9_mesh_goldens` donut: δ 0.05 `10814893347bbbdd` → `acbaea8ec424d559`; δ 0.15 `7628fd943b85d278` → `0a73f71aa4fa1618`.
- `route_rows` donut: `0x10e7f0a246fd3c81` → `0x72785776b24f0ce5`.
- viewer `review_pick_r2` aimed-vertex count: base 147,850, head 147,844. The six this change moves are three in `tube_arc` after its first edit (11901 → 11898) and three in `hollow_tube_elbow` open (7052 → 7049), both torus patches. The 147,856 → 147,850 before it is main's (`work/green/main-red-the-viewers-pick-tally-reads-six-grazes-short-of-its-pin.md`). Pinned to 147,844 with both parts named.

Unmoved: viewer `display_budget`; sweep's 65 tessellating modules (601/602 green; the red is main's `bisector_nodes::the_near_node_poses_at_the_witness_band`).

## Rows (`crates/mesh/tests/the_one_slit_band.rs`)

Each row pins a relation read from the mesh alone. The band meshes, it is watertight, every band triangle samples within δ of the torus, a triangle reaches at most 1 column and 1.5 rows, and the witness carries the off-lattice point it is there for.
- `a_revolved_one_slit_band_meshes_with_its_slit_points_between_rows`: revolve, hole 0.3, r 5e-4, δ 1e-3, 3e-4, 1e-4 (the three δ raw UV refused). At ε 1e-12 the revolve itself refuses `KernelLimit` upstream; the row says so and asserts that refusal.
- `a_filleted_holes_one_slit_band_meshes_with_its_slit_points_between_rows`: `sweep::blend::fillet_edges` on a holed plate; 3 arcs r 1e-3 δ 1e-3, 3e-4; 5 arcs r 5e-3 δ 3e-3, 1e-3.
- `a_band_whose_foot_is_two_arcs_meshes_with_its_rim_points_between_columns`: an off-lattice point from a source other than the knife edge. The foot is two edges, each chorded on its own, so it holds more chords than columns (60 against 59). This sits on the short axis, where raw UV did not refuse either.
- `the_filleted_holes_meshes_are_byte_identical_at_every_eps`: one pinned digest `0x527e_a70e_d171_1f85`, the same at ε 1e-9, 1e-6 and 1e-12.
- `an_off_equator_sphere_zone_meshes_within_delta_in_its_metric`: the 50°–80° zone, δ 1e-2 and 1e-3.

On main, the one-slit band is reached by the fillet door on a 3-, 5- or 6-arc hole, and by a revolve.

## Mutants

| mutant | caught by |
|---|---|
| curved `metric = 1` (raw UV) | the revolved, filleted-holes and digest rows; 42 of the 498 sweep bands refuse |
| curved `metric = 1/k` | the same three rows; 83 sweep bands refuse |
| `lattice_point` ignores `k` (both lanes) | those three plus `trimmed::tests::cdt_point_floors_then_scales_the_growing_axis` |
| `lattice_point` always scales u by 1/k | `cdt_point_floors_then_scales_the_growing_axis` |

## §5 sweep

The pattern is every CDT insertion in `crates/mesh/src` (`cdt.insert(`, `bulk_load_cdt`, `spade_point(`):
- curved: fixed here;
- trimmed: `cdt_point`, already in the metric or the cone's development;
- planar: the plane's own orthonormal chart, which is isometric and has no lattice, so not this class;
- the rest (`nurbs_cert.rs`, `curved.rs`, `planar.rs`, `trimmed.rs`, `cdt.rs`): test modules only.

What the pattern cannot see: a chart whose metric is not one scalar per face. The sphere's 1/cos v changes across a zone; I checked zones up to 88° latitude by sampled deviation. The curved spindle shares the torus metric; no door built one here, so it is not measured.

Other charts: a curved cylinder or cone lays no rows (`nv = 1`), and the walk refuses a chord point on a ruled side (`SIDE_CHORD_POINT`), so a one-slit cylinder or cone ring cannot hold an off-row point. Its patches did not move. A curved sphere ring: the zones above.

## Still refuses

The revolved r = 5e-4 section at ε 1e-12 refuses `KernelLimit` in the torus certification, upstream of the mesh.

## Filed

- `work/tesslane/the-curved-lanes-triangles-beside-an-off-lattice-chord-point-have-no-containment-bound.md` (P2): the measurement above; it cites `one-containment-lemma-for-an-oblique-boundary`.
- `work/tesslane/a-quarter-arcs-chord-count-sits-on-the-capped-steps-ceiling-edge.md` (P3): the 5-ulp span, the two counts, and both sides of the edge.
- Evidence added to `work/shell/the-torus-composite-bounds-a-thin-torus-far-above-its-carriers-rounding.md`: the r = 5e-4 revolve's KernelLimit at ε 1e-12.
