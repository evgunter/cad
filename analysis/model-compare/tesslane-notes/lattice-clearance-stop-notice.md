IMPLEMENTER tesslane/lattice-clearance — EARLY STOP NOTICE (measure-first result; not the final report; nothing pushed yet beyond the base).

The brief's stop condition is met: faces that mesh on the branch's base newly refuse `CertificateExceeded` under the clearance, at every value of the constant. I am not choosing; I keep building only what does not depend on the choice (rule 2 whole; rule 1's mechanism; tests), leave the affected rows unpinned and the goldens unregenerated, and will push and send the full report. A ruling in the meantime would be used.

HOW MEASURED. The clearance filter is built (box of c·own step per chart axis against every side of every loop and every crease line; on the cone the box is the image of the chart box in the development, where the sides are straight). An in-process A/B ran every trimmed face the whole `mesh` suite builds, plus a 42-body corpus of mine (frusta/funnel cuts, hollowed vessel and elbow, C1, donut×B2, tilted pair, rod×bar, tube×ball, rods cut at tilt 0.5 and 1.1, six δ each), once with main's lattice and once under the clearance, same polygon, same retry loop. 548–550 distinct analytic trimmed faces, 826k–842k lattice nodes; 703 distinct spline faces, 3.6M nodes.

THE TABLE (analytic faces; "worst" = worst certificate over faces meshing both ways, in cell bounds, headroom allows 2):
- none: worst 1.807 (NOT the log's 1.06: the suite's own faces reach 1.81 on main — the funnel cut near its apex 1.81, T1 ringed cone 1.75, the r=5 tilted cylinder 1.52, the holed pipe 1.30).
- 1/16: 2,123 nodes not laid on 304 faces; 236 faces change triangle count; worst 1.807; NEWLY REFUSE 2.
- 1/8: 4,074 on 387 faces; 289 change; worst 1.927; NEWLY REFUSE 2 (the same two).
- 1/4: 7,984 on 400 faces; 312 change; worst 1.975; NEWLY REFUSE 7.
- (first census, suite only, tests truncating at their first red: 3/8 and 1/2 add refusals on five and six more reach rows — funnel through its apex, T1 cone ring, the turned bar's holed pipe, narrowing frustum.)
- Spline arm: 0 nodes not laid at 1/16, 1/8, 1/4 and 3/8 on all 703 faces (2,007 at 1/2); no mesh moves, no refusal. The arm takes the rule at 1/4 without a byte moving in this suite (so `loft_prism`/`swept_elbow` goldens do not move).

THE NEWLY REFUSING FACES (all `CertificateExceeded`, all mesh on the base):
1. At 1/16, 1/8 AND 1/4: `reach_probes::a_cone_face_ending_near_its_apex_refuses_at_fine_delta`, the funnel `revolved[(0,0),(1,1),(0,1)]` cut at h = 2e-2 by the plane of normal (sin 0.3, cos 0.3, 0), the half above, δ 5e-5, both cone faces: main 1.31 cell bounds; 2.33 at 1/16 (15 nodes not laid of 15,252), 5.53 at 1/8 (33), 11.28 at 1/4 (58). This is PR 6965's ground (the chart certificate reads the full azimuth span of a triangle the development makes short near the apex), so its answer may change when 6965 lands; today it refuses at any clearance.
2. At 1/4 only: `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta`, r = 5, δ 1.5e-3 (one of its 60 cells): main 1.52 → 2.04. The same face is `trimmed::tests::an_ear_the_budget_leaves_refuses_typed`'s.
3. At 1/4 only: my corpus's rod of radius 0.5 over z∈(−3,3) split through the origin by the plane of normal (sin 1.1, 0, cos 1.1), both halves' wall: δ 2e-2, main 1.11 → 2.04 (6 of 133 nodes not laid); δ 5e-4, main 1.37 → 2.14 (52 of 6,125).
So the log's "nothing refuses at 1/4 in either designer's run" does not hold on plain tilted-cut cylinders: a steep cut passes the headroom by 2–7%.

WHAT REPRODUCED. The cut frustum (r = 20ε) at δ 1e-4: 226 certified triangles with a lattice corner at height ≤ ε, least 5.56e-10 m, none unresolved (least height 4.4e3 allowances) — exact. The whole frustum at δ 1e-8: 81,792 triangles, 6,780 unresolved, all in the plane cap; none at 2e-8, 1e-7, 1e-6 — exact. Differences: at δ 3e-9 the base MESHES the whole frustum (149,324 triangles, 22,756 unresolved in the cap); the log says the cone face refuses `CertificateExceeded` there. Moved a metre off its axis, with the scale spelled as `cert::position_scale` spells it (Euclidean norm of the corner): 9,359 unresolved at 1e-8 and 4,465 at 2e-8, not the log's 6,610 — I take the designers' count to have used the largest |coordinate|; I am keeping cert's one spelling of the scale and will say so.

RETIREMENTS, measured: with any clearance on, the exact-duplicate drop fires 0 times and the node-on-a-side retry 0 times over the whole suite (6 and 60 on main's lattice). A narrow oblique band (rod wall between two parallel tilted cuts 0.2 … 0.002 apart, δ 1e-2…1e-4) meshes at every width under 1/4, worst 1.96 cell bounds; no new refusal there.

QUESTION THAT IS NOT MINE: what the unit does with faces 1–3 (a lower constant clears 2 and 3 but not 1; 1 may be 6965's to clear). Until told, rows 1 and 2 stay red on the branch and I name them in the report.
