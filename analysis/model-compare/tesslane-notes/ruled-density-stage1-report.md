UNIT 4c STAGE-1 REPORT (tesslane/ruled-density, head ca8d1f67217a2286da3bdb2482043ae63be91547; both commits carry no trailer)

WHAT I BUILT (throwaway; mesh::ruled_probe holds the switch CURE, read in trimmed.rs; stage 2 deletes it all)
- All cures: a face reaching the chart lane for a split side alone (no ring, no trim carrier) is sized at δ, not at the headroom (face_target), so its edges are chorded as the rectangle lane chords them.
- c1: no lattice rows; the lane's CDT as is (cylinder in the lane's own metric, cone in its development); refinement on Exceeded (the spline arm's centroid insert) switched on for the face; pass budget 400 so convergence is visible.
- c4 (extra): c1 with the cylinder triangulated in its isometric metric (r·u, v), to see how much c1 leans on the metric.
- c2: no rows, plus one held edge per split side. The spec's wording ("from each mid-side vertex across the face") does not confine the vertex: one edge from m closes one rim side only, and Delaunay still fans m across columns on the other. So I held the edge between the first rim points off the side (a1–b1; apex–a1 on a cone through its apex). That is one edge per side, and it closes the side's column.
- c3, the walk: the face's rim runs and side runs read from the polygon's edge carriers; rows A and B (or the apex) merged by u; each split side fanned from the corner opposite it; certified per triangle as the lane would (row.cert, verdict at δ). No CDT and no development.
- Harness: crates/mesh/tests/ruled_cures_probe.rs (#[ignore], release). Per cell: tessellate; check_mesh; reach_probes::geometry_enclosing (winding resolved and outward, triangles sampled within δ, volume); per routed face: triangles against the same face of the unsplit body (rectangle lane), worst cert/δ, passes, refinement points, wall time, rim chord points against the unsplit.

CORPUS: split_rulings' bodies plus a 0.1° draft wall; cones through the apex at 0.01°, 0.1°, 1°, 5°, 20°, 45° and 80° (full turn = two half faces) and a 20° cone turned 2π−0.3; frustums, full and half turn; your two slender cones; T1's four ops. Each line edge split at 1, 2 and 5 points and at 1e-3, 1e-6, 1e-8 and 1−1e-8 of its range from an end. δ 0.1, 0.03, 0.01, 0.003, 0.001 (T1 2e-2, 5e-3, 1e-3; your bodies at your δ and one coarser). 1,657 cells at the default ε, 1,040 at 1e-6, 1,714 at 1e-12. Cells change by ε because split_edge and revolve admit different bodies; cone 1000 does not build at 1e-12.

TABLE, default ε (meshed/cells; refusals; worst ratio to the unsplit face; worst added triangles per face; worst cert/δ; worst passes; Σ wall time)
walls   c0 530/590 DevMerge55 Triang5 | ×196 +9836 | .498 | 2 | 1.1s
        c1 501/590 DevMerge55 CertExc18 Fold16 | ×24 +3458 | 1.000 | 22 (needle wall: 400 passes, no convergence) | 5.8s
        c2 535/590 DevMerge55 | ×2.4 +18 | .998 | 2 | 0.5s
        c3 590/590 | ×1.6 +10 | .998 | 1 | 0.4s
cones   c0 615/635 DevMerge10 Fold10 | ×17618 +70469 (0.01° cone) | .506 | 6 | 36s
        c1 568/635 Triang15 DevMerge10 Fold42 | ×102 +2929 | 1.000 | 15 | 0.9s
        c2 625/635 DevMerge10 | ×2.25 +5 | .990 | 1 | 0.4s
        c3 635/635 | ×2.25 +5 | .990 | 1 | 0.3s
frustums c0 295/315 DevMerge20 | ×21 | c1 295/315 DevMerge20 ×20 14 passes | c2 295/315 DevMerge20 ×1.5 +5 | c3 315/315 ×1.5 +5 cert .698
slender (your bodies, "1e-8 from the rim end" read both as 1e-8 of the range and as 1e-8 in length):
        relative split: every cure meshes, today's lane at ×280–1440, c1/c2/c3 at unsplit+1.
        absolute split (the one that reproduces): c0, c1 and c2 all refuse DevelopedMerge on base; c3 meshes at +1 (37/36, 113/112, 8/7, 22/21), cert/δ ≤ .976.
T1      c0 96/96 ×163 | c1 96/96 ×83, 14 passes | c2, c3 96/96 +1 (on the one half cone face the ops leave untrimmed)
1e-6 and 1e-12 give the same picture: c3 meshes every cell at both (1,040 and 1,714); c2 refuses only DevelopedMerge (none at 1e-6, 85 at 1e-12); c1 adds Fold, CertificateExceeded and Triangulation as above.
- Across all ε, every meshing cell passes check_mesh, the winding check and sampled deviation ≤ δ for every cure (0 BAD). At 1e-12 the volume is not read on the bodies outside the session box; their winding is.
- c4 is far worse than c1 on cylinders (up to 1,620 refinement points against 4), so c1's cylinder result rests on the lane's square-cell metric.
- c1's passes: at the shipped MAX_GRID_RETRIES (6), 183 of c1's default-ε meshing cells would refuse. Its median ratio at δ 0.001 is ×20 on cones and ×10 on frustums: refinement on the certificate converges to roughly today's density. On the needle wall fold-drop and refinement ping-pong for 400 passes (4.6 s), and the fold drops refinement points.
- Count relation: with no interior points, any triangulation of the boundary has a fixed count (boundary points − 2), so c2 and c3 count alike: unsplit + 1 per side vertex (+2 per vertex on a one-face wall, whose split seam is both of its sides). The 4 extra interior points in c2's cylinder cells come from the ear cure's rebuild. Every triangle c1 adds over that is a refinement point.

THE RESIDUALS (default ε)
1. Planar neighbour narrower than δ (slender wall, δ 0.1, Triangulation): gone under all three cures, and the cause is sizing the routed face at δ, not the triangulation. Rim chord points equal the unsplit's under every cure (today ×1.21–1.43). So a planar neighbour's chord count returns to the base's under all three.
2. DevelopedMerge: c1 and c2 keep all of it (55 walls, 10 cones, 20 frustums, plus your absolute bodies); c3 has none (it leaves the development).
3. A ruling split a hair from its end folds (cone 20° 2π−0.3): today 10 cells fold; c1 makes it worse (42); c2 0; c3 0.
4. Yours (the 7054 needle beside a merged run near an apex): c3 never passes through the development, so it has neither the merge nor the needle; c1 and c2 keep the development and with it this residual. On base your bodies answer DevelopedMerge under c0, c1 and c2 when the split is 1e-8 in length (not of the range).

WHAT THE SPEC HAD WRONG OR LEFT OPEN
- Cure 2 as worded (one edge from the vertex) confines nothing; see above.
- "The chart lane's density" is in large part the headroom: dropping it is what cures residual 1, under any cure.
- The designers' "c1 meshes an unsplit cone at 1.25–1.45×" does not carry over to split faces: refinement converges on the walls (needle aside) but to ×10–100 on cones and frustums at fine δ, and it adds Fold and Triangulation refusals.
- Not measured: a face on both nappes; the four-use-edge question was not probed directly, though check_mesh is clean on every c3 cell (the walk skips the ear and shared-side rule).

WHAT IT WOULD TAKE TO SHIP
- c1: a pass budget about four times MAX_GRID_RETRIES, a cure for the fold/refinement ping-pong, and a cure for the Triangulation refusals; it keeps residuals 2 and 4 and leaves density near today's on cones. I would not ship it.
- c2: the row reader (the same front end as c3), held edges, sizing at δ; it keeps residuals 2 and 4 (it waits on 7054 and on a fix for the needle), and nothing proves the CDT beyond the held column (cert ≤ .998 measured).
- c3: about 200 lines (row reader, merge walk, side fan, per-triangle certificate), sizing at δ, the convex-hull proof the spec owes written beside the step functions, and an argument or a row for the ear question. It removes the face from the development and so cures residuals 2, 3 and 4 for this class.

RECOMMENDATION: c3, the walk. It meshes every cell at all three ε, with the fewest triangles, at the lowest cost, and is the only cure that clears every residual. The certificate still checks every triangle at δ, so it is "a division into triangles that never deviates by more than δ" in Ev's sense, not a constraint laid on the CDT. c2 is the fallback if Ev wants every face to go through the one CDT: it is identical in count and certificate, and differs only by the development residuals. c1 does not survive measurement.
What would change this: (a) Ev ruling that a second way of writing triangles in the chart lane is unacceptable (then c2, after 7054 and a needle fix); (b) a face of the class whose loop is not rim/side/rim/side or rim/side/apex/side (the probe would refuse it with MissingEntity; splits_a_straight_side admits iso rectangles only, so I expect none, and T1 found none); (c) a four-use edge the walk makes (none in about 4,400 c3 cells across the three ε).

Stopping here until you name the cure.
