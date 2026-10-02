# Review of PR #3845, frozen head 3f98dbb948

Lane `reach-dual3845-r1` · started 13:59 UTC, ended 14:56 UTC, 2026-10-02 · glimpse: none (PR read with `get` and `get_check_runs` only; no other lane's branch, comments or reviews read).

**Verdict: NOT-MERGEABLE-AS-IS** (MAJOR 1 · MINOR 3 · NOTE 4). No wrong body anywhere: 0 of ~330 built probe rows disagreed with the closed form or with `point_in_solid` samples. The MAJOR is a new false kernel-bug verdict, raised on valid input that main refuses honestly. Its cause is local: the arc pass never checks that the two germs share a locus. CI on the PR head (run 37016247882: `test`, lint, gate) is green.

## Findings

**MAJOR 1: the arc pass reads germ j's sense in germ i's frame, so valid input refuses `JoinDesync` "malformed germ data"** (`rest.rs:546`, `join.rs:1311-1316`). DEMONSTRATED BY EXECUTION.
- `rest.rs:546` takes `frames[i]`. It only checks that `frames[j]` is `Some(_)`, never that it is the same conic.
- Fixture: two rod columns in ONE op (`probe_two_columns`). A is a base plate plus two rods r=1 on z 0..1, centred (±1.25, 0). B is a top plate plus two rods on z 1..2. Every finding is declared.
- Circle 2 is split at φ = atan2(√(1−1/d²), −1/d), d = 2.5 (`RADIAL`). There its tangent is radial about circle 1's centre, so `s2` decides Zero.
- Head: `JoinDesync { "a conic germ has no rotational sense (radial germ direction — malformed germ data)" }`. Main (cd49025f): `Join(UnpairedLooseEnds { count: 8 })`. Both operand orders, and the mirrored layout too.
- So a typed frontier refusal becomes a kernel-bug class with a false cause (the [[refusal-text-is-not-cause]] shape). Graded MAJOR as a wrong verdict, not a wrong body; the adjudicator may recalibrate.

**MINOR 1: the arc pass pairs germs across loci; claim 1's "pairs the right germs" fails off the single-circle fixtures** (`rest.rs:546-559`, `join.rs:1329-1344`). DEMONSTRATED BY EXECUTION (pairing log).
- Two stacked tubes r 1..2, all four rims split in two and aligned. The arc pass pairs (2,0) outer with (1,0) inner. Both sit at the same angle, so `germ_separation` ≈ 1e-16 ranks them first. Also (−1,0) with (−2,0).
- Two aligned columns: it pairs (−1,0) on one circle with (3,0) on the other.
- Every case ends in a typed refusal (`UnpairedLooseEnds`, `ChordEndpointRevisited`), and none builds. The same as main, except MAJOR 1.
- The chord pass also pairs across circles. That is on main already: e.g. tubes (0,0)/(0.7,0), where (1,0) pairs with (−1.53,−1.29). It is why that pose still refuses while (0,0)/(0,0.7) now builds.
- r·φ uses the radius at p1, so it is not comparable across loci of different radius.
- Not filed: the PR's sweep (body, "Sweep for the class") looked for chord facings by name. It did not look for a frame that is never compared.

**MINOR 2: two of the four mutants survive every row, including mine** (claim 5). DEMONSTRATED BY EXECUTION (`probes/mutants.py`).
- M2, chord-nearest in the arc pass: 81/81 green.
- M4, φ ∈ [0,2π) instead of (0,2π]: 81/81 green.
- `germ_separation_rows` pins the function, not its use. On one circle the arc pass only ever sees the half-turn case: at most 4 leftover germs, one facing partner each, where chord and arc agree.
- So the PR's "chord-nearest is not usable in the arc pass" has no reachable row. Its arc measure matters only across loci, where MINOR 1 shows it is wrong anyway.
- M1 (arc pass removed) and M3a (filter → refuse) turn 3 PR rows red. M3b/M3c (filter → first or last parallel edge) turn the same 3 red, and also build a WRONG body in my two-column probe: (−1.1, 0, 1) reads `OnBoundary` instead of `In`. So the incidence filter is load-bearing and pinned.

**MINOR 3: the dumbbell and two-half rods are fixed, but not two half-turn circles in one op** (aligned tube stack, two aligned columns). DEMONSTRATED BY EXECUTION. They refuse as on main. Scope, not regression, but the unit's title class ("aligned seams") is only half closed, and nothing is scheduled.

**NOTE 1 (claim 3, bit-identity): holds.** DEMONSTRATED BY EXECUTION.
- Method: `probes/segment_log_instr.py` logs every `enumerate_segments` result and every REST outcome (faces, edges, FNV of the point coordinates) on main and head. Full topo+sweep suites at 1e-9.
- All 268 REST bodies main builds (47 tests: crosslap, peg/collar, m5_s1, m9_3, mate2, contact8, …) come out identical on head.
- Head adds 43 new REST bodies (aligned poses), all correct. The hash covers points and counts, not key-level topology.

**NOTE 2 (claim 4): every table row reproduced** against πr²h and the dumbbell's 2π(0.3²·0.5 + 1.5²), my own derivation from the revolve profile.
- Widened, all correct or typed: piece pairs (2,2) (2,3) (3,3) (2,4) (4,4) (3,4), θ ∈ {0, 1e-7, 0.7, π/2, π−1e-7, π}, scales 1e-3/1/1e3, both orders. That is 216 builds at 1e-9 and 1e-6.
- At 1e-12 the 1e3-scale fixtures fail to author in `extruded`: a profile escalation, not this PR.
- Four-rod reuse chains; unequal radii 2/1, 1/2, 1/1.001; box on rod with x0 ∈ {0.3, 0, −0.3, −0.7} (arcs < π, = π, > π), n ∈ {2,3,4}; plates on 2 or 3 rods. All correct.
- A rotated pose (skew axis, 0.9 rad) refuses `CurvedPierceUnsupported`, identically on main.

**NOTE 3 (claim 6): the filed join item holds by inspection; no fixture reaches it.** Measured with `probes/join_instr.py` over the full topo+sweep suites.
- 2,336 conic `find_match` choices and 29,614 `loose_partners` choices.
- Chord-nearest ≠ arc-nearest exactly once: the edge-straddling pip in `review_d2_adv_probes::corpus`. There, equal chords (0.2078) tie across two face pairs with different frames (φ = π vs 2π/3). That is a tie-break, not a back-to-back pair.
- φ > π choices occur (axis_lap blind D: 4.43), but they are legitimate long arcs.
- The item omits `find_match`'s `is_up` filter (`join.rs` find_match), which may exclude its example (unsure).

**NOTE 4:** `conic_frames` (`rest.rs:613-633`) now lets the REST lane return `Err` from `germ_section_frame`, where main returned `Ok(None)` and surfaced the join's refusal. That is the same door as MAJOR 1. The REST-err count is unchanged on the corpus (8 = 8).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7, Q8 partial)
- Q1 · `rest.rs:531-542` vs `join.rs:1284-1296`: the straight chord-facing test is still a verbatim copy, re-commented "as the join", beside a call to the join's own `germs_face_each_other`. The PR widens the second spelling it says JOIN-2 deletes. **sure**
- Q2/Q5 · `rest.rs:29`: "the SAME mutual-facing/nearest tests as the join" is now false. The join ranks conic germs by chord, the REST arc pass by arc. **sure**
- Q5 · `rest.rs:447`: "paired along their conic locus" promises a same-locus pairing that nothing enforces (MAJOR 1, MINOR 1). **sure**
- Q5 · `join.rs:1324`: φ ∈ (0, 2π] is not what `T::pi() + (-sin).atan2(-cos)` gives at sin = +0.0, cos > 0. atan2(−0.0, −c) = −π, so φ = 0. The unit row has no φ ≈ 0 case. **likely**
- Q3 · `join.rs:2888`: the unit row cannot go red when `enumerate_segments` stops using the measure (M2). **sure**
- Q4 · the module doc's "no new numeric predicate": the swept-arc atan2 is a new measure decided through the old `bool_join_nearest` funnel at the length band. **unsure**
- Q7 · `frames: Option<Vec<Option<Frame>>>` doubles as the pass-mode flag, lazily set inside the `best == None` arm. Two explicit passes would read more plainly. **unsure**
- Q8 · read `rest.rs` 420-1045 (segments through `fan_edge_between`) end to end, not all 1998 lines. **sure**

## Probes (committed under `probes/`)
- `review_3845_r1_probes.rs`: drop it into `crates/sweep/tests/` and add it to `all.rs`. It covers the stack matrix, rotated/reuse, unequal radii, box on rod, plates on rods, two columns (incl. `RADIAL`) and tube stacks.
- `mutants.py` (M1, M2, M3a/b/c, M4).
- `segment_log_instr.py` + `segment_log_cmp.py` (main/head differential).
- `join_instr.py` (claim 6).
- Suites I ran: full topo+sweep at 1e-9 on head (green, three times instrumented) and on main. The PR rows, `germ_torus_doors`, my probes and the REST corpus subset at 1e-6 and 1e-12: green, no wrong body.
