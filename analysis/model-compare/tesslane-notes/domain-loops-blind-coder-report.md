1. FINDINGS (one row per distinct defect; A/B columns give label, severity, and DEM = demonstrated by execution / INSP = by inspection)

# | Finding | Coding | A | B | Class
1 | Empty ring on a curved face panics (`unreachable` in pcurves_rings) where base gave a typed refusal | BILATERAL | m6 MINOR, DEM | MINOR-1 MINOR, DEM | code
2 | Filed row says STL export "would refuse DegenerateTriangle"; it does not (cross 4.17e-18) | BILATERAL | m7 MINOR, DEM | MINOR-2 MINOR, DEM | doc/claim
3 | `DomainUnread` Display, `Gap` text ("finished body stores every one") untrue | BILATERAL | m5 (part) MINOR, INSP | claim 2 + style Q2 `unsure`; Gap reached by probe, text judged INSP | doc
4 | Door read twice per trimmed face (memo gather + lane); one read swallows the miss | BILATERAL | claim 2 correction + S1 style `sure`, INSP | style Q1 `likely`, INSP | code (structure)
5 | `TrimPolygon` unpacked into parallel slices; unit rows pass `next: &[]` with non-empty polygon | BILATERAL | S3 style `sure`, INSP | style Q7 `likely` + rubric, INSP | code (structure)
6 | reach_probes `NoArea` handling loses information (A: cannot say how many; B: keeps first kind only) | BILATERAL | S8 (part) style `likely`, INSP | style Q3 `unsure`, INSP | test-gap
7 | CI job `viewer app feature (wasm32)` red: 62 clippy `result_large_err`; cause not established | UNILATERAL-A | m1 MINOR (gating), DEM (red CI row) | - (saw checks queued/skipped) | code
8 | Ring near cone apex refuses (`CertificateExceeded` / `DomainUnread Undecided`); unreported | UNILATERAL-A | m2 MINOR, DEM | - ("ring near the apex was not built") | code
9 | Valid ringed torus face (spiric outer loop) refuses `DomainUnread{Undecided}`; PR account omits it | UNILATERAL-A | m3 MINOR, DEM | - | code
10 | `TrimLoopOrientation` Display text not true of the new ring cause | UNILATERAL-A | m4 MINOR, INSP + mutant | - | doc
11 | `DomainUnread` Display: drops `LiftGap` third cause; `Undecided` reads as nearness (split from row 3) | UNILATERAL-A | m5 (rest) MINOR, INSP | - | doc
12 | "Placed, not stored" witnessed by one body only (ball minus tube); moved rings on torus/cylinder unpinned | UNILATERAL-A | m8 MINOR (tests), DEM (mutant red on 1 body only) | - | test-gap
13 | Ringless wrong meshes (`NonManifoldEdge`) on base and head alike; not this unit | UNILATERAL-A | N1 NOTE, DEM | - | code (pre-existing)
14 | Sphere ring 0.006 rad from pole: `FoldedTriangle` at delta 2e-2 only | UNILATERAL-A | N2 NOTE, DEM | - | code
15 | memo mints its own band beside tessellate's (split from row 4) | UNILATERAL-A | S1 (part) style `sure`, INSP | - | code (structure)
16 | Curved lane ring guard removed; safety rests on `lane_of` arm order | UNILATERAL-A | S2 style `likely`, INSP | - | code (structure)
17 | `trim_orientation` "no outer loop" arm unreachable; no-area ring is `MissingEntity` | UNILATERAL-A | S4 style `likely`, INSP | - | code
18 | Three spellings of `RingPlacementError` variants; no `name()` | UNILATERAL-A | S5 style `sure`, INSP | - | contract-API
19 | types.rs:613 falsifier paragraph: `PoleInside` swapped for `SeamTwice` mechanically | UNILATERAL-A | S6 style `unsure`, INSP | - | doc
20 | trimmed.rs module docs / `TrimLoopOrientation` doc predate rings ("cylinder or sphere") | UNILATERAL-A | S7 style `likely`, INSP | - | doc
21 | reach_probes `Got::Unread`: no row expects it (split from row 6) | UNILATERAL-A | S8 (part) style `likely`, INSP | - | test-gap
22 | lily.rs retired wall kept as numbered panic-armed code | UNILATERAL-A | S9 style `unsure`, INSP | - | code
23 | trimmed.rs:530 "VACUOUS TODAY" comment census stale | UNILATERAL-A | S10 style `unsure`, INSP | - | doc
24 | Memo key changes on Planar/Curved lanes (`pcurve: None`); PR text false; undisclosed | UNILATERAL-B | - (claim 9 "held") | NOTE-1 NOTE, INSP; also "SILENT" deviation | claim (code effect stated harmless)
25 | Dropped-ring / filled-hole watertightness enforced by debug-only census; pre-existing | UNILATERAL-B | - (see section 6) | NOTE-2 NOTE, INSP | code (pre-existing)
26 | Outer-loop-only second readers (`outer_loop_curves`, `has_trim_carrier`, `has_tilted_circle`) | UNILATERAL-B | - (claim 13: "same readers the PR lists") | style Q1 `likely`, INSP | code (structure)
27 | `trim_orientation` doc overstates: ring orientation check is validity only, no mesh depends on it | UNILATERAL-B | - | style Q2/Q5 `sure`, DEM (check-skipped mutant passes all 4 rows) | doc
28 | `singular_row_shape` doc: prose replaces a check, no assertion | UNILATERAL-B | - | style Q2 `unsure`, INSP | doc
29 | Open work row still lists `PoleInside` among causes | UNILATERAL-B | - | style Q4 `sure`, INSP | doc
30 | errors.rs Display row pins only `OuterOpen` text; six cause texts unpinned | UNILATERAL-B | - | style Q3 `likely`, INSP | test-gap
31 | errors.rs `two_faces_refusing_differently...` narrowed to ringless; ringed poisoned face unexercised | UNILATERAL-B | - | style Q3 `unsure`, INSP | test-gap
32 | lift_probes.rs doc says singular point "inside" the face; it is in the ring's hole | UNILATERAL-B | - | style Q5 `unsure`, INSP | doc

2. COUNTS
- Distinct findings: 32. Bilateral: 6 (rows 1-6).
- Unilateral-A: 17 = MAJOR 0 / MINOR 6 (rows 7-12) / NOTE 2 (13-14) / style 9 (15-23).
- Unilateral-B: 9 = MAJOR 0 / MINOR 0 / NOTE 2 (24-25) / style 7 (26-32).
- Severity divergences on bilateral findings: 1 (row 3: A MINOR vs B style item tagged `unsure`). Rows 1 and 2 are MINOR/MINOR; rows 4-6 are style/style with only confidence tags differing (sure vs likely, sure vs likely, likely vs unsure). On row 4, A additionally lists it as a claim-2 correction.
- Three splits were made: rows 3/11 (A's m5), 4/15 (A's S1), 6/21 (A's S8).

3. CLAIMS
1. A: held (1602-line base/head differential). B: held (armed panic, 461/462). AGREE.
2. A: held, two corrections (door read twice; two untrue refusal texts). B: held, one exception (empty-ring panic; Gap text). AGREE on held; corrections overlap only on the Gap text.
3. A: held on what builds. B: held for what could be built. AGREE (neither built a ring touching another loop).
4. A: held (six mutants, 5 bodies x 3 delta). B: held (six mutants, 4 rows). AGREE. Mutant sets differ: A alone ran "ring read where stored" (row 12); B alone ran "reversed with check skipped" (row 27).
5. A: held (executed, 441 meshes). B: held by inspection. AGREE.
6. A: held by inspection. B: held. AGREE.
7. A: held by inspection. B: held by argument. AGREE.
8. A: partly (near-apex ring built and refuses, row 8). B: partly exercised (near-apex not built). AGREE on ruling.
9. A: held by inspection. B: corrected (NOTE-1, row 24). DISAGREE.
10. A: corrected (by rows 7 and 1). B: held, with MINOR-1 (row 1). DISAGREE on label; both cite the empty-ring panic, A adds the red CI job.
11. A: corrected (STL claim, row 2). B: held, with a corrected claim in the filed issue (row 2). Same substance, different label.
12. A: held by reading (gate run). B: held (gate run). AGREE.
13. Neither gives a held/falsified label. Both list the same PR-reported deviations (torus at 1e-3, notch to unit 4, measurement). Silent deviations: A lists rows 8, 9, 1; B lists rows 24, 1. Overlap on row 1 only.

4. VERDICTS AND RUBRIC
- Verdict: A APPROVE-WITH-FIXES ("No MAJOR"), conditional on CI, which A saw red on one job. B APPROVE-WITH-FIXES, conditional on a green that did not yet exist (B saw checks queued/skipped). B raised no MAJOR.
- Idiom/structure: A 4, B 3.
- Test quality: A 3, B 4.
- Doc/comment honesty: A 3, B 3.

5. TALLY CANDIDATES
None. Criterion (a) fails for every finding: neither reviewer raised anything as MAJOR, so there is no unilateral MAJOR to test against (b)-(d).
For the record, the unilateral findings that would meet (b) and (d) but are MINOR or NOTE are rows 7, 8, 9, 12 (A, MINOR) and 13, 14 (A, NOTE). B has no unilateral finding that is both code/test-gap/contract-API and demonstrated; row 27 is demonstrated but is a doc finding raised as style.

6. NOT CODED CONFIDENTLY
- Row 25 (B NOTE-2): A's claim 4 says "a misplaced ring is a mesh tessellate returns Ok; only check_mesh's winding sees it". That is adjacent (reliance on a caller's `check_mesh`) but concerns misplaced rings, not the debug-only census for dropped/filled rings, and A does not raise it as a finding. Coded UNILATERAL-B; a looser dedup would make it bilateral (NOTE vs in-claim remark).
- Row 24 vs claim 9: A writes "a ringless closing face's key is unchanged", which matches B's statement about ringless trimmed faces. A is silent on Planar/Curved lanes, so this is not coded as an affirmative contradiction, only as a differing ruling.
- Rows 16 and 26: both are "an outer-loop-only reader survives", at different sites (A: curved.rs; B: sizing.rs and trimmed.rs). A says its own sweep found "the same readers the PR lists, plus S1/S2", i.e. it saw B's sites and did not raise them. Coded as two unilateral findings; could be argued one class.
- Rows 19 and 29: both are stale `PoleInside` references, in different files (types.rs paragraph vs an open work row). Coded separately.
- Rows 10/20 and 27: all concern orientation text, but A's complaint is that the Display/doc wording predates rings, and B's is that the function doc implies a meshing hazard that does not exist. Coded separately.
- Rows 18 and 30: both touch `RingPlacementError` variant texts (A: three spellings; B: texts unpinned by tests). Coded separately.
- Row 1 construction differs: A reaches the panic after `mint_pcurves`; B says the panic needs the saved joint and pcurve rows restored, and without the restore the body refuses `Gap`. Treated as one defect.
- Row 3 demonstration status: B reached `Gap` by execution, but the judgement that the text is untrue is by reading, and B tags it `unsure`. Recorded INSP for both.
- Row 7: demonstrated as a red CI row, but A states the cause is not established and suggests checking the same job on main, so its attribution to this change is open. B's "queued/skipped" is a different observation time, not a contradiction.
- Row 8: B's 180-mesh census reports the cone ring body meshing at 12 delta; A's refusals are at poses moved toward the apex that B did not build. Not a contradiction.
- Class assignment for rows 8 and 9 is "code" (a valid body refuses), though both reviewers also frame them as an omission from the PR's account, which is claim-class.
- Review-coverage statements (ring-touching poses not built, torus v-moved ring not stored, spindle not built, ringed spline face not built) were not coded as findings.