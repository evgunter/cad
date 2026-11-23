tesslane/collapsed-side — STAGE 2 REPORT (part 2 of 2): PR body

=====
**A spline face with a collapsed side meshes at every δ.** Closes `work/tesslane/a-spline-face-with-a-collapsed-side-meshes-only-at-a-coarse-chordal.md`.

## The defect
A loft with a pinned corner, a hinge or a lune builds walls whose chart has a collapsed side: a whole `u` column is one point P (`geom_brep::chart_singularity::singular_points`, `Singularity::Side`). The trimmed lane's spline arm read no singular vertex there, so `flat_points` held P once, at (0, 0). The polygon's side then ran diagonally from the top rim's last chord point (h, 1) to (0, 0), across the column. Every triangle beside it fanned from P@(0, 0) and spanned the column in the chart, and `NurbsCellGrid::cert` bounds a triangle by its chart box.
- Example: the wedge at δ = 1e-2 gave bound 0.01649. The sampled parametric deviation was 0.00825, while the wall is planar, so the true distance to the surface is about 0.
- The refinement retry could not cure it: the worst triangle's long edge was a constrained side of the polygon, and the bound stayed at 0.01649 for all six retries.
- The loft's three bodies refused at δ ≤ 1e-2. The wall pinned through three sections at v-degree 2 refused at every δ, 0.3 included.

## The cure (`crates/mesh/src/trimmed.rs`)
- **Read the sides from the home.** `CollapsedSides::of(surface)` reads them. A loop vertex is singular where its stored point IS the side's stored point (`one_stored_point`, the identity `collapsed_sides` itself uses). No band is read, so the mesh stays a function of (body, δ). A vertex a hair off the side keeps the one-copy shape: sound, and refused typed where δ is not met.
- **Keep the copy.** With the vertex singular, `flat_points` keeps its copy as it does at a sphere's pole, so the column is a side of the polygon. This is the analytic arms' own spelling, not a second one.
- **Walk the side.** `SideWalk` walks the side between the two copies at ⌈span/h⌉ pieces, with h the face's chord step along the side (`row.steps(face_target)`). This is `[mesh.sizing]`'s rim-chord rule applied to the one boundary side that has no edge. Every point is a copy of P lying on the side, where S(u, v) = P, so the chart-box certificate is sound for any triangle cornered there.
  - A triangle on two copies has two corners on one vertex, and the existing emit rule drops it.
  - The triangles that remain fan about P. The identified-vertex census re-derives that on every class body in debug.
  - Analytic arms walk nothing: the sphere's and spindle's certificates read space, and the cone's reads no azimuth at its apex.
  - A vertex on two collapsed sides (a corner) is not walked, since the path between its copies turns there. Filed as `work/tesslane/a-spline-vertex-on-two-collapsed-sides-keeps-its-polygons-copies-unwalked.md`; no door builds one.
- The certificate is unchanged, and README needs no edit.

## What moved
Only walls with a collapsed side. Over the class (9 lofts, below) at δ ∈ {0.1, 1e-2, 1e-3}, per-face coordinate digests, base against head:
- 16 of the 27 runs refused on base; head meshes all 27;
- in the 11 runs both mesh, 22 collapsed-side walls move;
- the 56 other faces (caps, quads, the struts' walls) are bit-identical.

Where base already meshed, the counts change by a few percent (wedge at 0.1: 240 → 248 triangles). No golden or digest row of the suite moves: no corpus body has a collapsed side. No pin moved.

## Rows (`crates/mesh/src/trimmed_collapsed_rows.rs`)
- `every_collapsed_side_the_loft_builds_meshes_at_a_fine_chordal` (slow set): the class at δ 0.1, 1e-2 and 1e-3 meshes, is closed and resolves its winding. The wedge also encloses 4·sin 0.5 within δ·area. The class:
  - the wedge;
  - one corner pinned;
  - a corner pinned through three sections, at v-degree 1 and at v-degree 2;
  - a hinge in the first, middle or last slab;
  - the lune, and the lune on a straight slab.
- `a_collapsed_side_walls_samples_stay_under_their_certificates` (`budget` feature): every collapsed-side wall's samples stay within their own triangle's certificate at 1e-3.
- `a_collapsed_side_is_walked_as_its_unpinned_twins_strut_is_chorded` (slow set): the RELATION. At δ 0.1, 1e-2 and 1e-3, the pinned wall's side holds exactly as many pieces as its twin's strut has chords. The twin is the same corner lifted 1e-4, which builds a strut edge where the side was; its strut count is the larger of its two walls' asks. Every copy also lies on the side's column.
- `a_collapsed_sides_vertex_is_its_stored_point_and_nothing_near_it`: the side's own point is held; a point an ulp off is not.
- `a_side_is_walked_along_its_axis_and_a_corner_is_not`: a `u` side walks along `v`, a `v` side along `u`, and a vertex on two sides is not walked.
- `a_walked_side_holds_its_pieces_between_its_copies`: ⌈1/0.3⌉ = 4 pieces, on the segment, in walk order, beside the right edge, the loop's closing pair included; any other pair is left as it is.

All rows pass at ε 1e-9, 1e-6 and 1e-12.

## Mutants (each caught)
1. Sides never read: the class row and the relation row.
2. Copies kept but no walk: the class row and the relation row.
3. Walk axis swapped: the axis row, the relation row and the class row.
4. Corner walked: the axis row.
5. A 1e-9 distance band in place of exact identity: the exact-identity row.
6. One piece too many: the pieces row and the relation row.
7. Walked at twice the step: the pieces row and the relation row.

## §5 sweep
Pattern over `crates/mesh/src`: `singular_points()|is_singular(|singular_row_shape(|pole_side_columns(|\.poles()|pole_columns(|SingularPoint`. Hits and dispositions:
- trimmed `tessellate_trimmed`'s singular read: fixed (`face_polygon`).
- trimmed `singular_row_shape`: not this unit. PastPole and BothNappes are analytic row-end shapes, and a collapsed side bounds its chart. SeamTwice on a spline chart is the separate "read as walked" seam case.
- trimmed `pole_side_columns`: not this unit. It guards #678's single-column shape on the analytic uniform lattice, and a spline lattice has no `nu`; the emit census re-derives the conclusion, and it passes on the class.
- trimmed `CdtPlane::of`, `nappe`, `developed_runs`, `Side::of`, `cdt_point` and `inside_point`: they read `singular` only in the cone's development; a spline chart is `Lattice`.
- trimmed emit `corners.is_singular` → `ChartRow::cert`: the spline certificate ignores it, and that is sound because every copy is on the side.
- chords `FaceLattice` `Reads::ChartRows { singular }`: analytic only (the chord budget for an arc near a pole); a collapsed side has no edge.
- curved `pole_columns` and `poles()`, walk `chart.poles()`: the curved lane, where no spline face routes.

Blind spot: readers keyed on a repeated id rather than on a singular symbol. Second pass over `ids_at_two_uvs|one_location_per_branch|id_repeats_apart|overused_identified_edge_in`:
- `one_location_per_branch` returns at once on a spline chart (no periods);
- `id_repeats_apart` is the cylinder's debug check;
- the census (`ids_at_two_uvs`, `overused_identified_edge_in`) now sees the spline copies and passes on every class body.

The doors: only the loft builds a collapsed side, as `u` columns. Step import has no fixture with one, so a `v` side is covered by the unit rows alone.

## Still refuses
Nothing in the class, at any δ down to 3e-4 (scratch sweep).
=====
