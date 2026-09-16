# S-MESH exit walk — criteria vs evidence

**STATUS: PROPOSED (design conversation — awaiting Ev's
ratification; on sign-off this document is S-MESH's done-state of
record, the program closes, and the successor TESS opened in the same
PR stands).**
S-MESH = mesh honesty and budget (`work/mesh/plan.md` /
`work/mesh/log.md`; opened 2026-08-31 from the ratified stream cut in
`docs/WORK-STREAMS-2026-08.md` §S-MESH by the same orchestrator as
S-BOOL; A/B band 1200–1299). The plan's "Exit shape (proposed)"
paragraph supplies the criteria, quoted verbatim, one commitment per
row, dispositions per the S-MATE/S-CERT convention: MET /
MET-WITH-RECORDED-HONESTY / CARRIED (named owner).

Eleven units merged: ordinals **1200–1210**, samples **#76, #82, #88,
#92, #96, #101, #106, #110, #112, #113, #157**. Three rulings ratified
in-program (Ev, in-chat, 2026-09-01: Q1 S65 stays compiled out; Q2
option (d), the detectors relocate body-side; Q3 explicit doors, no
transitive floor). No tally candidate in eleven duals.

## The walk

| # | Criterion (verbatim from `work/mesh/plan.md` §Exit shape) | Disposition | Evidence / honesty note |
|---|---|---|---|
| 1 | "the walk anchors are placement-honest (#1362)" | MET | **PR #1389** (MESH-1, ordinal 1200, sample #76): the walk's `band_u` loop-area fold re-anchored at the loop's own bbox centre (`loop_area` extracted — the disclosed deviation that made the conditioning claim testable), the red-first drift table 1e2..1e8 with whole-2π branch flips at 1e6 and 1e8 under the origin anchor; issue 1362 closed at merge. |
| 2 | "the sub-floor engineered zeros mesh (#555 — the Klein wall-7 entry narrates a closed case)" | MET | **PR #1421** (MESH-2, ordinal 1201, sample #82): the class closed in two layers — the chart frame writes the far point's structurally-zero v at the projection site (a repeated-row determinant, bit-keyed so a slit seam's twin traversals stay one chart point) and spade's `mitigate_underflow` floors the insert feed for the systematic residue; issue 555 closed at merge; the Klein wall-7 entry re-recorded as closed. |
| 3 | "pole classification is guarded against undeclared poles (#896)" | MET | **PR #1460** (MESH-3, ordinal 1202, sample #88): `walk::loop_polygon`'s D2-row-5 undeclared-pole guard — no junction the emission carries as `pole:false` lies within ε of a chart pole, the identified side deliberately not asserted (never-infer-from-values); the pole find given one home (`pole_index`); issue 896 closed at merge. Sample #88 was taken under merge contention (MESH-3's row landed after another program's); main's merge order ruled, no renumber. |
| 4 | "ε's consumers are the methods on a type (#881 closed on both halves)" | MET | **PR #1517** (MESH-4, ordinal 1204, sample #96): ε's terminal reads became named operations on a mesh-local `Eps` newtype (separates / coincident / dominates / pad), all six ported; the two-argument form disclosed at the fix pass as unadoptable without moving bytes; the MESH-4 two-build digest at three ε rows is the D9 instrument every later unit in both programs ran; issue 881 closed at merge on both halves. |
| 5 | "the `nu == 1` schedule says what it does (#685)" | MET | **PR #1507** (MESH-5, ordinal 1203, sample #92): the sizing-intent decision made by measurement — the π/6 cone-wedge δ-sweep at two builds showed honouring the v-schedule multiplies the patch 5–9× at bitwise-identical sampled deviation, so one strip is right and the schedule says so; the S29 instance retired at the site; issue 685 closed at merge. The one-element-grid class the unit did not fund is `one-element-grid-axes-drop-schedule` (issue 1513), carried to TESS. |
| 6 | "S65 is ruled with all three prices measured and the ruling implemented (#897 → Q1)" | MET | Q1 RULED (S65 stays compiled out; no unconditional shipped guard). **PR #1545** (MESH-6, ordinal 1205, sample #101): the full-2π seam and the cross-face identification each given a mechanical `cfg(debug_assertions)` census (the emit pass widened to identified edges; the unpaired-chord use-count; the trimmed lane's NURBS arm covered in the fix pass); issue 897 closed at merge. |
| 7 | "the iso-rectangle premise has one home and every door cites it (#727/#726 under Q3)" | MET | Q3 RULED (explicit doors, no transitive floor). **PR #1565** (MESH-7, ordinal 1206, sample #106): the public shape-only predicate door `props::require_iso_rectangle` beside the flux lane, every consumer citing it; SMELL §D row C11 retired in the record; issues 727 and 726 closed at merge. **PR #1599** (MESH-11, ordinal 1209, sample #113) completed the family — the walk's arc premise VERIFIED rather than inherited, at the door, as the separate named predicate `require_one_chart_branch` beside it (CERT-1's pole rows admit pole-crossing meridian arcs on purpose); issue 1571 closed at merge. |
| 8 | "input-quality detectors speak through the ratified warning channel (#868 under Q2)" | MET-WITH-RECORDED-HONESTY | Q2 RULED as option (d), RELOCATION — not the warning channel the criterion's wording anticipated when the plan was written. **PR #1585** (MESH-8, ordinal 1207, sample #110): the three `mesh::walk` input-quality `debug_assert!`s DELETED and their conditions re-derived body-side as `topo::coherence::examine_chart_coherence(body, tol) -> CoherenceReport` — non-gating, deterministic, each relocated condition shown firing on the witness the mesh assert would have caught; issue 868 closed at merge. The honesty: the criterion is met by the ruling that superseded it, and `topo/src/coherence.rs` passes to TESS with `crates/mesh`. |
| 9 | "#950 stands parked with its typed trigger" | MET (carried parked to TESS) | MESH-9 stayed PARKED on issue 950 for the whole program: neither fix is needed until a body presents the configuration, and the failure is a `CertificateExceeded` refusal that names it. The unit and its trigger item (`rim-chords-exceed-snapped-column-count`) move to TESS together, the park and its `blocked_on` intact. |
| 10 | "Track R is empty in §D" | CARRIED (TESS ×7, PROPS ×2) | No Track R row ran as a lane: the lane budget went to the defect cluster (MESH-1–8) and the three units the substrate produced (MESH-10, 11, 12), and Ev's 2026-09-16 direction was to close the programs with their residue rolled over where it coheres. MESH-R dissolved at this exit and its rows re-homed by FILE: S28, S236, S237, D300, D303, D304, C23 to TESS (their files are TESS's); C3 and D30 to PROPS (`props/quad.rs` is PROPS' certify ground, the S-CERT sequencing they waited on discharged). S26 closed under that sequencing before the exit. |
| 11 | "Every unit merged on its own green hosted head; the walk convention applies at exit" | MET | Eleven unit merges, each on a hosted green run verified before the merge, the A/B row riding the unit branch as its last commit; MESH-12's landing was executed by PROPS on top of S-MESH's own state-sync commit (the two programs shared the lane budget and the merge window) and its row records BOTH duals. This document is the walk. |
| 12 | Process (§Process, verbatim in substance): "v6 … ordinals claimed on main at review dispatch from band 1200–1299; record-at-merge with per-phase tokens/wall-clock; blinding discipline verbatim; hosted CI the only gate; the #1356 ε-trailer practice adopted from the first dispatch" | MET-WITH-RECORDED-HONESTY | Eleven v6 duals, ordinals 1200–1210 all claimed on main at dispatch, rows at merge, briefs symmetric, blocks MESH-B1–B4 drawn branch-side. **The honesty, four notes**: (1) the band was renumbered twice at the joint opening (900 lost to GAUTH, 1000 to SEAT) and fixed at 1200 on main; (2) MESH-B3's slot-2 arm was MISASSIGNED at one dispatch and disclosed in the ledger at MESH-12's claim (ordinal 1210) rather than corrected silently; (3) MESH-12's dual ran TWICE — S-MESH's on its frozen head and PROPS's on the head PROPS landed — and the row records both, with the instrument note that the second found the reversed span the first's forward-only ladders could not; (4) the ε-trailer practice was retired repo-wide on 2026-09-04 (the `CI-Config` spelling deleted), so units from MESH-12 on carried none, as the ledger records. |

## Walk evidence beyond the criteria

- **MESH-10** (PR #1595, ordinal 1208, sample #112): `torus_parse` folds
  the pieces of a split meridian into the one meridian they carry
  before `torus_ends` reads the span — keyed on split LINEAGE
  (`Provenance::SplitEdge` chased to the root edge), a granted
  topo-ground extension; issue 1562 closed at merge. This is the
  lineage fold BOOL-5's reviewers later named as the precedent the
  sphere wedge arm lacks (filed on PROPS).
- **MESH-12** (PR #1617, ordinal 1210, sample #157): a sphere meridian
  span past certification's per-edge winding bound REFUSES typed at
  the parse under one name on every consumer
  (`props_meridian_span_winding`); issues 1601 and 1588 closed at
  merge, 1615 un-parked (carried to TESS).
- **The doctrine this program leaves behind**: ε as named operations on
  a type; the never-infer guard shape (structural rungs only, the
  identified side never asserted from values); shape predicates as
  explicit doors each consumer cites; body-data coherence examined
  body-side, non-gating; the MESH-4 digest as the cross-program D9
  instrument.
- **Residue re-homed at this exit, by file** (`work/README.md`'s
  closing rule): fourteen mesh findings (one of them the trigger item
  of the parked MESH-9, one VIEW's rider on the degenerate-normal row,
  filed into `work/mesh/` after this walk was cut and moved beside its
  parent), MESH-9 itself, and seven Track R rows to the successor **TESS**
  (`work/tess/`, band 5100–5199, opened in this PR); the stored-span
  reads (issue 1618) and the two `props/quad.rs` rows (C3, D30) to
  PROPS; the seed-varying cert10 gate and the grep-only sentinel
  markers to S-TINT. MESH-R closed as dissolved. Nothing was left in
  `work/mesh/` but the program files.
- **Filed forward on other slates during the program** (each with its
  home): the cross-program follow-ons 1587, 1597, 1598 and 1602 on
  their owners' ground; DOCM — the GUI-1 seeded fuzz guard that
  reddened MESH-12's landing (proven not the PR's).
- **What opens with the sweep**: `work/mesh/` is deleted at
  ratification with the walk, recorded in `docs/DOC-LEDGER.md` with
  the SHA; `crates/mesh/*` and `crates/topo/src/coherence.rs` are
  TESS's.
- **Open with Ev at exit** (nothing blocks ratification): this walk's
  sign-off and, with it, the TESS opening; the S65/D283-class
  questions the charter reserved as Ev's stand unasked beyond Q1.
