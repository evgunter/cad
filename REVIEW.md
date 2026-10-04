# Delta review: PR 4026 fix pass, frozen head `f88ad91e`

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 3 · NOTE 4

The fix pass does what it says. Every battery count in its Validation table reproduces exactly, main → head. Head is line-identical to `93ecd145` on all five pierce batteries. Every asserting row goes red under a mutant. No refusal→BAD line is a definite wrong body by my census-free geometry. One thing is new: an in-band edge pair that neither the census nor the pinned row sees (m1). It is not shown wrong, but it is unfiled, and it makes the pinned row's doc claim false for its own pose.

**Method.** I used release builds of head `f88ad91e`, main `c860806e` (current origin/main; the merge base is `095ea377`, and nothing in `crates/topo` differs) and `93ecd145`, each in its own target dir. I ran r1's `r1_pierce_probes` (`cube`, plus `R1_NT_D=±{1e-2,1e-4,1e-5,3e-6,1e-7,1e-8}`), r2's `r2_shapes_battery` / `r2_holed_battery` and `pierce_runs_battery`. Every line is `differential::outcome`.

My own check is `review-delta/delta_inband.rs`, the `DR_IND=1` harness in `harness.patch`. On every built body it lists the vertex–vertex, vertex–edge and edge–edge pairs within 3·(εK), by plain segment distance, and faces that meet two vertices on one point. It uses no kernel predicate.

Other probes:
- `delta_probe.rs` builds the shallow200 pose at any ε and tilt.
- The mutants are env switches in `mutants.patch`.
- All outputs are in `review-delta/out/`.

## Claims

1. **The false-positive verdict: HOLDS for the census's pair. The broader "no pair in band" claim is FALSIFIED (m1).**
   - At `shallow200 nt e0 a3 d1e-7`, ε = 1e-9, pc ∪/∖ raise `EdgeEdgeOverlap{13v1,25v5}`.
   - That pair (the top-edge piece near (4,1) and the section edge from `v`) is not within 3 bands; the far ends are 2.04e-7 apart. So the census's witness is false, as filed.
   - The row's pose (its association order) reproduces the same finding at ε = 1e-6 (tilt 1e-4) and 1e-12 (tilt 1e-10).
   - At ε = 1e-6 with the fixed 1e-7 tilt, every in-band pair is the recorded `vv` contact or ends on it (`shallow200-pose.txt`).
   - **Converse: cannot ship silently through this lane.** A real collinear overlap has each bound at a vertex of one edge, and that vertex lies on the other edge, or on its vertex.
     - Passes 1–2 (`census.rs:1295` `pair_vertex_vertex`, `census.rs:1349` `pair_vertex_edge`) read each such vertex's own offset. They apply the same backing rungs as `ee_bound_backed`.
     - So a collinear-lane miss is always caught by a vertex pass.
     - Executed: no head body that tier 3′ passed has an interior in-band edge pair, an unrecorded in-band vertex–vertex pair, or an in-band vertex–edge pair that main does not also ship.
2. **No definite wrong body ships: HOLDS.**
   - Main → head refusal→BAD lines: 6 (r1 cube), 65 (r1 near-tangent) and 3 (r2 shapes), 74 in all.
   - I re-read each one's tier-3′ error (`head-refusal-to-bad-t3p.txt`): 72 are `CensusEscalated` only (`ee_span` / `ee_parallel` / `ee_overlap`), and 2 are the false positive.
   - The 26 escalated poses match the P0 row's list exactly.
   - Main's own 138 + 31 BAD lines are unmoved.
3. **The new refusals: HOLD.**
   - 0 SOUND→refusal against main, and 0 against `93ecd145` (base→head moves 0 lines in all five batteries).
   - With the guard lowered to `> 1` (`M_RUNS1`), every two-run pose refuses typed `PierceRunsUnordered{runs: 2}`, so the variant and its plumbing are live.
   - No pose I can build reaches 3 runs: degree-3 vertices only, as r2 N3 says.
4. **The rows can go red: HOLDS.** The 0-BAD half of `the_sweep_subset_ships_no_bad_body` goes red under no-`kfmrh` (24 lines `OK BAD … cert=false`). Its vertex-count half goes red under no-weld (24 lines "runs through two vertices at v"), as does `two_out_runs_…` at `join_pierce_strut_facing.rs:219`. The near-tangent row goes red under no-twin and under `M_RUNS1`. See N2 for the subset row's blind spot.
5. **Welds: HOLDS.** Across every built head body, `FACE2V` (a face meeting two vertices on one point) never fires, and every head body passes tier 2. `weld_pair` (`finish.rs:563`) is the old post-check, moved verbatim. The new `one_vertex` check never fires in any battery.
6. **Filed rows: HOLD, with one correction (m2).**
   - The escalated P0 row: poses and counts reproduce, including main's 139 by prism.
   - The ∩ row: its evidence is r1's numbers.
   - The weld-residue, hole and kissing-corner rows name the right next door.
   - The CONTACT row's converse paragraph is wrong (m2).

## Findings

**m1 (MINOR, executed, likely). Head certifies bodies in which two vertices on one point each carry an edge, and the two edges run within the band for up to ~0.4 of their length. Main refuses every such pose; the class is unfiled.**
- At the pinned pose, pc ∪ holds two vertices at `v`, and no face meets both (copies left apart, PR 3813).
- The top-edge piece leaves one, and the section edge leaves the other. They are 3.7e-7 rad apart, so they lie within ε = 1e-9 for about 2.7e-3 from `v`, and within εK for about 2.7e-2.
- Across the batteries: 116 tier-3′-SOUND lines and 17 BAD lines at 89 poses, all main refusals. At `shallow200 nt e0 a1 d1e-8` the pair stays within 3 bands for 1.25 of its 2.0 length. On main there are 0 such lines; on `93ecd145` they are identical to head.
- The census passes the pair (`census.rs:1891`): it levers parallelism at the full arm (2.0), and the shared point is endpoint territory.
- Under D3, a collinear overlap whose far bound carries no vertex is unbacked. It is excused here only because the arm reads "not parallel". This is the same arm question the CONTACT row's ref `boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm` raises.
- The pinned row's doc (`join_pierce_runs_sweep.rs:442`, "no two edges of the body come within the band") is false for its own pose. Its check excuses any pair that shares a *point* (`:508`), not only a pair that shares a vertex.
- Fix: file the class with these poses (on CONTACT, or on the escalated P0 row), and correct the row's doc.

**m2 (MINOR, inspection + executed). The CONTACT row's "converse" (`work/contact/…long-edges-start.md:52`) is a risk that cannot occur.** A collinear-lane miss is caught by passes 1–2 (claim 1). The row's next door, "read the offset at the short edge's endpoints", fixes the false positive. It would not see m1, whose pair diverges from a shared point. The row should say both.

**m3 (MINOR, executed). `boolean_pinch_copies.rs:6-11` names two places copies stay apart on one point.** The near-tangent ∪/∖ of m1 is a third, at 89 poses. Its "That is not universal" list is an instance list, while the class is "any two-run pierce whose copies no face meets". This is a half-fix of r1 S5 / r2 S4.

**N1.** The fix-pass kernel changes are inert on every battery: head equals `93ecd145` line for line. The ≥3-run guard and the `one_vertex` check are unreached, as their docs say.

**N2.** The 0-BAD subset row passes refusals by design. So it stays green under `M_RUNS1` (every two-run pose refuses) and under ∩-follows-the-walk. `join_pierce_strut_facing` is what catches those. The subset row alone does not guard reach.

**N3.** I did not re-run the other 14 JOIN batteries. Spot check, base vs head, line-identical: `rc_wide_battery` (40 320), `join1_r1_reflex_battery` (1 152) and `rw_battery` (2 250).

**N4 (dispatch premise).** "Origin/main" is `c860806e`; the PR merged `095ea377`, and the two differ only outside `crates/topo` and `crates/sweep`. The brief's r1 repro equals the row's pose bit for bit at ε = 1e-9: `DP_ROWASSOC` gives identical keys and witness. The row's "ten bands" is 1e-4 at ε = 1e-6, not 1e-5: I first ran 1e-5, where every op refuses `Escalated(SectorSide)`.

## Style (exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8 partial on `finish.rs`, 957 lines, read whole)

- **S1 (Q2, sure)** `vtxfac.rs:762`. "where the walk's facing refuses `SelfLoopEdge` and this one builds" overclaims. By the filed ∩ row's own numbers, the start facing builds 69 and refuses 100 `JoinDesync` of those 169.
- **S2 (Q7, likely)** `vtxfac.rs:771-781`. The `match runs.len()` keeps a general `k =>` arm and `(i + 1) % k`, although `vtxfac.rs:551` now refuses k > 2. The walk is written for k runs and reachable for two. That is the shape the P3 row warned of, kept as dead generality.
- **S3 (Q1, likely)** `finish.rs:538` vs `:546`. The pierce weld refuses `Chord` before it checks `one_vertex`. So two off-point copies that a chord would join refuse as "divide a face", not "not on one point". `weld_pinches` checks the point first. The two welds still order their guards differently.
- **S4 (Q3, likely)** `join_pierce_runs_sweep.rs:508`. The shared-endpoint test is point equality, so the row cannot fail on m1's shape. Its premise excludes the failing mode.
- **S5 (Q6, unsure)** `.config/nextest.toml:21`. The PR's one 0-BAD guard goes straight into the slow set. The file's rule ("no record of catching a bug the fast set would miss") reads against a fresh guard. It still runs per-PR where `topo` seeds.
- **S6 (Q4, likely)** `work/contact/…long-edges-start.md:44`: "Every edge pair of the body parts by more than 2.0e-7" is false (m1). The same sentence appears in the PR body's Fix pass §1.
- **S7 (Q5, unsure)** `mod.rs` `PierceRunsUnordered` Display: "crosses a face of the other {runs} times at one point". A run is a sector arc of Out, not a crossing; the user-facing count may read oddly.

No lane-isolation glimpse beyond the two review reports the brief named.

REVIEW COMPLETE
