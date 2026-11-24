Excerpts of one reader's review of PR 7666 at `273f765de5` (attribution stripped): its answers on the run and on the sector sliver.

2. The run: held for line, circle and vertex. Each station is a boundary point, so the distance is attained, apart from m3. L-shaped face, hole, wrap, the line re-touched: the sup definition reads the far arm. For a planar face that is the right lever, since tilting the plane about the edge moves the far arm, so I find no defect by inspection. I built none of these as bodies. Spline: m5. Vertex with no kept run: `extent_of` → None → `parts` false. That is fail-safe, never a certification. I did not re-take the lane's count of 0.
3. The sliver: held. Probe `review_probe_sliver_neighbours`, contact9; runs in metres, ε = 1e-9, K = 10:
   - As pinned: the tip edge 17v1 is `Crease(Convex)` in tier 3's verdict. Both runs are 9.6097 (the farthest each face stands from the tip edge's LINE; the long direction (10, 1) is not square to the edge), so the runs ARE measured across the 1 mm edge. Result: `Embedded`, receipt (15, 24, 0, 0, 0, 9, 13). Check 4 and the question agree.
   - 1 m at the same angle (50ε at the far end): `Embedded`, the same receipt.
   - 10 m with 20ε at the far end (sum × 9.6 ≈ 19ε, just past the band): `Refused(Budget Depth 40)` at faces 8v7/9v3. Not certified, so the question is conservative near the band.
   - 100 mm at the same angle (5ε) and 10 m with 2ε: the `∩` itself refuses (vertex-on-face margin −5e-9 and −2e-9, inside (1e-9, 1e-8)). The in-band neighbours never reach the question.
   - Not built: the neighbour where the narrower face is the 1 mm one. The flat-strip unit row stands in.
   Reading: the "20 mm within ε" stretch exists beside every crease near its edge. I do not read the clause's last sentence as reaching it, and "known wrong" was unsupported.
