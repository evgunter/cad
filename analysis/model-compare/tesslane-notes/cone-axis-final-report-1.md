cone-axis-winding — FINAL REPORT, part 1 of 3. Pushed head: 4f1bb5c44c30b992a863c3847499dad5d6a5569c (confirmed with ls-remote). It includes 7043's fix pass through 9391ee0177. 7041 is not merged, as you said.

1. MEASUREMENT (unchanged from earlier): the triangle is the rectangle lane's. Its corners are an apex copy plus two lattice nodes of the column beside a split side ruling: three points of one ruling, so zero area by construction. The chart CDT's fan from the apex copy down that column made it. The class is every cylinder or cone face whose straight side holds a point. Measured on the base: cylinder walls refuse CertificateExceeded at δ 0.01 and 0.001; cones through their apex refuse UnresolvedWinding (the 5° cone at every δ); the 80° cone split at two points refuses CertificateExceeded at every δ; the frustum meshes on the fallback lattice.
Where the spec was wrong:
(a) step-import's split_iso_side is not in the class. The import's join merges the two collinear EDGE_CURVEs before the mesh sees the body, so nothing moves there.
(b) A one-face full-turn wall with a split seam does not refuse SeamTwice. It routes to the chart lane and meshes; it is in the new row.
(c) No body reaches BothNappes. The only body on both nappes, the hand-built apex_crossing_bowtie, is refused by the shape door (props_rim_side) before the route is asked, whether or not its generator is split at the apex.
(d) The walk's own run rule (same kind across a separated junction) is the wrong route test: it caught the slit (two D9 digests moved) and the one-generator rimless loops. The route reads the loop's v order instead and reads no ε.

2. WHAT CHANGED:
- 4d2a10746a: the route; the fallback lattice and bare_sides deleted; the curved lane refuses SPLIT_SIDE_ROUTED_HERE (router defect); `lane_of`, `face_target`, `arm_steps` and `compute_chords` take the run's tolerances, and SizingTols is minted before the chord pass; T1 re-pinned; split-side sweep totals 250/0; new class row; slow set.
- dea0aeb9de: the route test by v order (it replaced the walk's band rule); `lane_of` takes `Band`; the class row checks volume against the unsplit body; the split+placed wedge count 28 → 40; doc fixes in tessellate.rs and sizing.rs; tracker (one row filed and closed, three re-pointed).
- f0de9a0f0b: two call sites the 7043 fix pass added.
- Merges of origin/tesslane/lattice-clearance.
Why the existing rules missed this case: the development rule lives on the chart lane, and `lane_of` routed by carrier kinds, so a split ruling (two iso carriers) stayed on the rectangle lane. `bare_sides` was read only after routing, and its answer was to lay a lattice rather than refuse.

4. RAN LOCALLY on the final head unless noted:
- mesh whole suite: 494/494.
- Rows I added or changed at ε 1e-6 and 1e-12: green (run on the commit before the last 7043 merge).
- Pre-push list on 769d01310f (7043's 85cdf9df36 merged): fmt, run-gates, doc-gate, and clippy for mesh, geom-brep and the tour were green; the mesh --features budget clippy and test were green; tools (k-lint, tess-lint, tess-meter: fmt, clippy, test) green; step-import 329/329; sweep's 63 meshing modules 584/584.
- Viewer clippy is green on the final head; at 769d01310f it was red with large_err, which 7043's 9391ee0177 fixed.
- Viewer tests: two reds that are 7043's, not this branch's. `display_budget::the_budget_commits_the_delta_it_always_has` and `review_pick_r2::the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus` fail identically on origin/tesslane/lattice-clearance at 9391ee0177: same tally (451512,147856,16596,8142 vs pinned 451800,147952,16824,8244), and the same 68-line budget table byte for byte. Please route them to 7043.
- Left to CI: everything else; I did not wait for it.

6. WHAT SLOWED ME: the disk allowance filled once (three target dirs) and one compile was OOM-killed, so some runs were repeated. The first route test needed the walk's ε band, which threaded ε to five signatures and moved the ε inventory; replacing it with the v-order test removed both. The PR body follows in parts 2 and 3.
