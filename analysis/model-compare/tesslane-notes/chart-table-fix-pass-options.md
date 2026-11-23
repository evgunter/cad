CHART-TABLE fix pass — A done (unpushed, in progress), B needs your pick.
    
    A. "On the row" is now a fact about the body, decided once per face, where the kernel decides it.
    - `AnalyticRow::is_singular(p, band)` = `geom_brep::chart_singularity::vertex_at_singular_point(p, pole, band)` against each of the chart's poles.
      - The poles now come from `topo::chart::Chart::poles` (item 5); the row adds only `RowEnd`.
      - It is the same decision `topo::pcurves::singular_at` takes to turn the lift at that vertex. A 3-D distance at the run's band does not move with scale or pose.
    - The trimmed lane computes the set of loop-vertex mesh ids that are singular (`trimmed::singular_vertices`, read off each edge's end chord ids, which are the vertices' points bitwise).
    - `flat_points`, `pole_side_columns`, `singular_row_shape` (PoleInside, SeamTwice) and the certificate's per-corner flags read that set, by id.
    - `ON_ROW` is gone. The only `v` reading left is PastPole's `4ε`-relative past a domain end, on angles only (sphere and spindle; the cone's apex has no "past").
    - An escalation reads off, as `walk`'s wrap strip reads it. A vertex the lift could not decide leaves the loop with no lift, and the face refuses first.
    - Measured, funnel below the cut, δ relative to scale (δ·s at 5e-3, 1e-3, 2e-4):
      - ×1, ×100 and ×1e3 mesh at all three δ;
      - the posed funnel (your rotation and translation) meshes at all three δ;
      - ×300 at 2e-4 refuses `FoldedTriangle`: that is B.
    - The coarse-δ refusals are item 3's and identical at every scale and pose:
      - ×1 at 2e-2 bound 0.02922; ×1 at 1.5e-2 bound 0.015033 (×1e3: 15.0326);
      - the posed funnel at 2e-2 bound 0.029225.
    - A new row reads the decision itself: cone, sphere and spindle charts posed at (700.3, −577.7, 123.4) on axis (1,2,3), at scales 1e-2, 1 and 1e3. In each, the point and a point half the band's zero away are singular; a point twice its escalation away is not.
    
    B. Mechanism and count.
    - The sphere has no such triangle at all. Over the tilted ball pair at δ 2e-2, 1e-2, 5e-3, 1e-3 and the z-poled balls (r 1 and 0.3), zero triangles have 2A/L² &lt; 1e-9; the worst ratio is 0.030.
    - Three distinct points of a sphere are never collinear, because its "column" is a meridian arc. A cone's column is a ruling, a straight line through the apex. So an apex copy (u_a, 0) and two lattice points of one column (u, v1), (u, v2) make a proper chart triangle and a collinear 3-D one.
    - `pole_side_columns` is about #678's non-manifold fan, not this. Nothing in the sphere's mechanism covers the cone.
    - Measured on the funnel below the cut: 2 such triangles at δ 1e-3, 4 at 2e-4 (2A/L² ≈ 1e-17), 0 at 5e-3.
    - So the cure is not forced. Options, as final states:
    
    (1) RECOMMENDED: the cone's CDT runs in its development.
    - `cdt_point` maps (u, v) to the unrolled sector (r = v, angle = u·sin α), the cone's isometric development, instead of (u/k, v). A ruling is a straight line through ONE apex point there, so a 3-D collinear triple is a planar collinear triple and the exact CDT cannot form it: impossible by construction.
    - The two apex copies of the polygon map to one CDT vertex with one id. Their side has zero length (the constraint is skipped as a == b).
    - The lattice is still laid in (u, v), at the row's steps, and every certificate, fold and ear test still reads UV. Only the CDT's coordinates change, as `metric` changes them today. The cone row's metric becomes the development, so it answers "the metric the triangulation runs in" per chart.
    - No sphere, cylinder, torus or spline byte moves; the trimmed cone arm is this unit's.
    - Cost: a polar map per cone point, and a sector of u-span ≤ 2π·sin α &lt; 2π, so no self-overlap.
    
    (2) Keep the chart CDT and constrain every lattice column as a polyline from its own apex copy (u_i, 0), inserting the apex at every column on the apex row. Fewer degenerates, but not impossible by construction: an apex copy can still see two points of another, constrained column.
    
    (3) Emit the apex fan explicitly (apex to the first row, per column) and CDT only the face above the first row. By construction, but a second triangulation at a singular point, and the first row must be a polygon constraint.
    
    I will build (1) unless you say otherwise. It is the only one that is impossible by construction while still being "the metric the triangulation runs in", a row fact. Building the remaining items meanwhile.
</cross-session-message>

0 notifications remain queued.
