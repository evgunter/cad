tesslane/collapsed-side — FIX PASS REPORT (part 2 of 2): corrected PR body

=====
**A spline face with a collapsed side meshes at every δ.** Closes `work/tesslane/a-spline-face-with-a-collapsed-side-meshes-only-at-a-coarse-chordal.md`.

## The defect
A loft with a pinned corner, a hinge or a lune builds walls whose chart has a collapsed side: a whole `u` column is one point P (`geom_brep::chart_singularity::singular_points`, `Singularity::Side`). The spline arm of the trimmed lane read no singular vertex there.
- The polygon held P ONCE, at (0, 0), not twice as the issue supposed. Its side ran diagonally from the top rim's last chord point (h, 1) to (0, 0), across the column, and the chart triangle (0,0), (0,1), (h,1) lay outside the triangulation.
- Every triangle beside the column fanned from P@(0, 0) and spanned the column in the chart, and `NurbsCellGrid::cert` bounds a triangle by its chart box.
- Example: on the wedge at δ = 1e-2 the bound was 0.01649, against a sampled 0.00825 on a planar wall.
- Refinement could not cure it: the worst triangle's long edge was a constrained side of the polygon, and the bound stayed at 0.01649 for all six retries.
- The loft's walls refused at δ ≤ 1e-2. The wall pinned through three sections at v-degree 2 refused at every δ, 0.3 included.

## The cure (`crates/mesh/src/trimmed.rs`)
- **Read the sides from the home, decided where the kernel decides them.** `CollapsedSides` reads a spline chart's sides from `singular_points`. A loop vertex is on a side where `chart_table::at_a_singular_point` decides it at the run's band, the same function (`vertex_at_singular_point`) topo's lift calls where it turns the face. So the polygon keeps a copy exactly where the lift turns.
  - The mesh reads the incidence again because the lift does not store it: a joint at a singular point stores an ordinary deck element, like every joint.
  - An undecided vertex refuses as the analytic arms' do: `SphereTrimShape { SingularUndecided }`. The variant's name is a sphere's, and a rename is filed with the orchestrator.
- **Keep the copy.** `flat_points` keeps the vertex's copy as it does at a pole, so the column is a side of the polygon. This is the analytic arms' own spelling.
- **Walk the side.** `SidePoints` puts ⌈span/h⌉ pieces between the two copies, with h the face's chord step along the side (`row.steps(face_target)`).
  - This is `[mesh.sizing]`'s rim-chord rule applied to the one boundary side that has no edge: as fine as a strut on that column would be chorded, which is finer than the certificate alone needs.
  - Each point holds the side's fixed axis at the knot rectangle's own end, so it lies on the side, where S = P, and the chart-box certificate is sound for any triangle cornered there.
  - A triangle on two copies has two corners on one vertex and the emit pass drops it, as at a sphere's pole side. Its 3-D image is a segment, an edge of kept, certified triangles, so it adds no mesh point and the promise holds. What the surface does over the dropped chart region against that segment is measured, not proved: within 0.061 δ over the 2,184 drops of the class at four δ (this PR's review).
  - The census of identified vertices re-derives the fan about each walked vertex.
  - Analytic arms walk nothing: the sphere's and spindle's certificates read space, and the cone's reads no azimuth at its apex.
  - A vertex on two collapsed sides, or undecided against one, keeps its copies with no point between. That shape is sound and coarse; it is filed as `work/tesslane/a-spline-vertex-on-two-collapsed-sides-keeps-its-polygons-copies-unwalked.md`, and no door builds one.
- The certificate is unchanged, and README needs no edit.

## What moved
Only walls with a collapsed side.
- Over the loft's class at δ ∈ {0.1, 1e-2, 1e-3}, per-face coordinate digests, base against head: base refused 16 of 27 runs, head meshes all; in the runs both mesh, 22 collapsed-side walls move and the 56 other faces are bit-identical. This is the review's and the lane's measurement; no committed row shows the faces WITHOUT a collapsed side unmoved, beyond the suite's goldens staying green.
- `d9_mesh_goldens` gains the pinned wedge and the corner pinned through three at v-degree 2 (both refused at 1e-2 on base), at δ 1e-2 and 3e-2. No existing golden moved.
- Its two rows join the slow set (1.48/1.58 s → 1.91/2.12 s, one local run).

## Rows (`crates/mesh/src/trimmed_collapsed_rows.rs`)
The class is 11 lofts:
- the wedge;
- hinges in the first, middle and last slab;
- a corner pinned in a lone slab, and in the first and in the last slab of two;
- a corner pinned through three sections at v-degree 1 and at v-degree 2;
- the lune, and the lune on a straight slab.

The rows:
- `every_collapsed_side_the_loft_builds_meshes_at_a_fine_chordal`: the class meshes at δ 0.1, 1e-2 and 1e-3, closed and resolved, and the wedge encloses 4·sin 0.5 within δ·area. It does NOT pin the count: twice the step still meshes.
- `a_collapsed_side_is_walked_as_its_unpinned_twins_strut_is_chorded`: the count relation. The pinned wall's side holds as many pieces as its twin's strut has chords (the corner lifted 1e-4), at three δ.
- `a_walked_side_holds_its_pieces_between_its_copies`: ⌈1/0.3⌉ pieces, at the side's held value even from a copy an ulp off it, in walk order, beside their edge, the closing pair included.
- `a_side_is_walked_along_its_axis_and_a_corner_is_not`: a `u` side walks along `v` and a `v` side along `u`, with their held values; a vertex on two sides is not walked.
- `a_collapsed_sides_vertex_is_decided_at_the_band`: the side's point, an ulp off and half the band's zero are on; twice the escalation is off; between is undecided.
- `a_vertex_in_a_sides_band_is_on_it_and_meshes_as_the_vertex_on_its_point`: the wedge with its vertex points restated an ulp up meshes face for face as the wedge at three δ.
- `the_wall_pinned_through_three_sections_meshes_only_with_its_side_walked`: the witness; with the copy alone its bound sits near 0.06 at every δ.
- `a_lune_walks_both_its_collapsed_sides`: two sides on one wall, each walked down its own column.
- `the_class_meshes_to_one_digest_at_every_eps`: 11 FNV digests at δ 1e-2, identical at ε 1e-9, 1e-6 and 1e-12.
- `a_collapsed_side_walls_samples_stay_under_their_certificates` (`budget` feature): every sample within its own triangle's certificate at 1e-3.

All pass at the three ε. A collapsed `v` side is covered by the unit rows only: the loft builds none, and no import fixture holds a collapsed side.

## Mutants (each caught)
1. Sides never read: 6 rows.
2. Copy kept, no side points: 6 rows, the v-degree-2 witness among them.
3. Free axis swapped: 7 rows.
4. Corner walked: the axis row.
5. An exact read in place of the band: the band row and the in-band row.
6. One piece more: the pieces row, the twin row, the lune row and the digest row.
7. Twice the step: the same four.
8. Fixed coordinate taken from a copy: the pieces row.

## §5 sweep
Pattern over `crates/mesh/src`: `singular_points()|is_singular(|singular_row_shape(|pole_side_columns(|\.poles()|pole_columns(|SingularPoint`. Hits and dispositions:
- trimmed's singular read: fixed (`face_polygon`).
- `singular_row_shape`: not this unit. Its shapes are analytic row ends, and a collapsed side bounds its chart; SeamTwice on a spline chart is the separate seam case.
- `pole_side_columns`: not this unit. It guards #678's single column on the analytic uniform lattice; a spline lattice has no `nu`, and the census re-derives the fan.
- `CdtPlane::of`, `nappe`, `developed_runs`, `Side::of`, `cdt_point` and `inside_point`: they read `singular` only in the cone's development.
- emit `corners.is_singular` → `ChartRow::cert`: the spline certificate ignores it, and that is sound because every point it reads is on the side.
- chords `Reads::ChartRows { singular }`: analytic only.
- curved `pole_columns`/`poles()`, walk `chart.poles()`: the curved lane, where no spline face routes.

Second pass, over readers keyed on a repeated id: `ids_at_two_uvs|one_location_per_branch|id_repeats_apart|overused_identified_edge_in`.
- `one_location_per_branch`: returns at once on a spline chart.
- `id_repeats_apart`: the cylinder's.
- The census sees the spline copies and passes on the class.

## Still refuses
Nothing in the class at any δ down to 3e-4. The open rows:
- the corner shape (filed, P3);
- the ε guard owed by both readers of pole identification (`work/tesslane/pole-identification-lets-eps-choose-between-two-meshes-of-a-cone-face.md`, which this PR's dated note extends);
- the twelve chord-pass copies (filed, P4).
=====
