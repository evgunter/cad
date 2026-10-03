# Review of PR #3985, frozen head 7e33abf087

Lane `reach-dual3985-r2`. Start 2026-10-03 18:51 UTC, end 20:48 UTC. Merge base `0770bfaa`.
**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 5. No glimpse: I read the PR body only (via `get`), my own brief file, and no `analysis/reach-dual/*` branch other than this one.

## Claims
1. **No surviving chord moved. Holds (execution).** I added an env-gated dump of every `chord_spec` result (carrier, params, endpoints, bits) to both the head and the merge base (`probes/chord_dump_patch.py`) and compared per test (`probes/chord_dump_compare.py`):
   - topo+sweep+step-import: 626 of 638 common tests are identical sequences. In the other 12, every base spec reappears bit-identical on head (base-only = 0); head only adds chords where base refused `SectionNotPolar`/`SectionArcWindow`.
   - editor-core+mesh: 358 of 360 are identical. `reach_slab_cut_sector_side` changes 4 chords, but only in order `[1,2,0]`, which stops at the ringed-wall door on both base and head, so no built body changes.
   - Tour (release): 3744 vs 3744 specs, identical multisets, and every STL/STEP/uv output byte-identical.
2. **New bodies are correct. Holds (execution).** `probes/review_reach_dual3985_r2_probes.rs` runs every op in both orders at ×1e-3, ×1 and ×1e3. Volumes are checked against my own closed forms (lens, cap, sector + fan areas) or my own slice integral, plus all three tiers, plus `point_in_solid` against my own SDF:
   - Built and correct: 69 tilted sphere pairs, 72 tilted plane×sphere, 18 bar/L-notch pierces, 288 four-crossing star plates.
   - Reused operands (union ∖ slab, against a Monte Carlo volume of my predicate) also build correctly.
   - **0 wrong bodies.** Base builds 0 of the sphere pairs and pierces, and 54 of the plane×sphere rows. At ε 1e-6 every result was still right; one probe reported 3 "wrong" points that lie 8.4e-7 m from the surface, inside the 1e-6 band, where `OnBoundary` is the correct answer (my probe's skip distance was too tight).
3. **Walk order beats chord length. Holds by mutant; my fixture doesn't discriminate.** I built 0°/100°/200°/300° crossings, on two half-turn faces and on one full-turn revolved face, over 6 rotations, 2 radii pairs and 3 scales. All 288 bodies are correct on head, but base gets them right too, so the hazard isn't reached by this fixture (likely `find_match`'s `is_up` filter, unmeasured). Mutant M1 (walk_passes disabled) turns `reach_slab_cut_sector_side::a_slab_across…` and `tilted_sphere_pair::a_pierce_off_the_seam_plane…` red. **The insert.rs strut-facing swap is not in this PR** (see N1).
4. **The new rows go red without the fix. Holds (execution).** `probes/mutants.py`, run over topo, sweep and mesh plus the PR's editor-core rows:
   - M1 walk_passes disabled: 2 red.
   - M2 boolean germ dir flipped: 147 red; it builds 9 bodies that tier 3 catches.
   - M3 split dir flipped (chord only): 61 red.
   - M4 arc always ccw: 262 red.
   - Old selector restored (head's rows run against base's kernel): 8 of the touched rows go red.
   - The collar row is in the CI slow set and is green under M1/M3, but goes red under M2 and M4 (`SectionLoopMixed`).
   - Two refusal pins never go red under any mutant: `a_pip_with_its_seam…` and `the_die_pips_shape_stops_typed…`.

Suites, run by me since **no CI run exists on 7e33abf0** (0 check runs): topo+sweep+step-import 4422/4422 and editor-core+mesh 2939/2939, each at ε 1e-9, 1e-6 and 1e-12. `work.py lint` and clippy are clean. At 1e-12 my probes' `ball_poled` fixture door fails pcurve certification (`pcurve_envelope`) before any boolean runs; that's not this PR.

## Findings
- **MINOR M1 · demos/README.md:62 (inspection; the contradiction is executed)** says the z-moved head "refuses `SectionArcSide { NoCertifiedRun }`", that the spun head's chords "read the face's window", and that the x-moved head goes "through the run-side arc rule". The PR's own `snowman.rs` row (the z-moved head) now builds. The PR updated `demos/tour/src/snowman.rs` but not this README row.
- **MINOR M2 · crates/topo/src/boolean/reduce.rs:3336-3342 (execution).** `Placement::declared` argues `Undecided` is untested end-to-end because the collar ∖ wedge "refuses before any mate: `SectionArcWindow{BothContained}`". Base does refuse there, but head builds that pose (`every_wedge_through_a_full_turn_collar…`). The premise is gone. Whether the `Undecided` arm is now reachable is unmeasured; this may be code drifting from the invariant, not just a stale doc.
- **MINOR M3 · work/reach/arc-side-rule-has-two-predicates.md:77-79 (execution: `pull_request_read get`).** It says "The PR body has the counts" of the phase-1 cross-check. The body is a WIP placeholder, so the measurement the retirement rests on is recorded nowhere I could read.
- **MINOR M4 · merge state (execution: `git merge-tree`).** The head conflicts with main in `chord_join.rs`, `boolean/join.rs`, `splitting/join.rs`, `snowman.rs`, `tilted_sphere_pair.rs` and the join work item (main carries JOIN-3's chord plan). What merges isn't what I reviewed; claim 1's differential and the mutants are owed again on the merge.
- **NOTE N1 · brief premise.** No `insert.rs` change is in the diff: `strut_facing` lives on main (`boolean/insert.rs:911`) and isn't reachable from this head. Claim 3b can't be checked here; it belongs with M4.
- **NOTE N2 · `bool_join_nearest` (execution).** At ×1e-3, bar-through-ball pierces refuse `Escalated(bool_join_nearest, margin −9.7e-9)`; at ×1 and ×1e3 the same poses refuse `RingOffCylinderChart`. That's a pre-existing absolute-metre chord ranking, newly reached, so the refusal door depends on scale.
- **NOTE N3 (execution).** `point_in_solid` refuses at every sampled point (5220) on every sphere-pair and pierce result. The new bodies can't be classified, which matches filed items (`tang/a-ring-on-a-sphere-face…`, `reach/carved-sphere-body…`). My set-membership oracle therefore covered only the plane×sphere and plate bodies.
- **NOTE N4.** The collar row is slow-set only (`.config/nextest.toml:20`), so the PR gate never runs it; only nightly does.
- **NOTE N5.** Every `RingOffCylinderChart` my probes hit (60 refusals) is the filed sphere-ring winding item. Those refusals are typed, not wrong bodies.

## Style (Q1, Q2, Q3, Q4, Q5, Q6, Q7 exercised; Q8 partial: chord_join.rs header + section map + 900-1220 + 2290-2480, not every line)
- **S1 · chord_join.rs:1263-2030 vs :1-80 (likely).** About 770 lines of azimuth-window/chart machinery remain in the "chord-join core". `face_azimuth_window`'s readers are now only `boolean/solid_contain.rs:988,3206` (the rest of that span still serves the ring lane). The module doc no longer mentions any of it: a core hosted beside a consumer that no longer needs it.
- **S2 · boolean/join.rs:948-953,1542 vs splitting/join.rs:454 (likely).** Walk order is spelled twice: an atan2 sweep about the centre in `walk_passes`, and a parameter walk in `conic_pairs`. The prose ("the order `conic_pairs` pairs by") is the only thing tying them together.
- **S3 · chord_join.rs:998 (likely).** `chord_arc_leave` decides two different quantities under one name: a cosine (unit germ dir) on the boolean and sine×cosine (`±n_plane×n_out`) on the split. The audit row admits as much. K-REPORT's precedent (PR #55 MINOR-1) split margins like that.
- **S4 · chord_join.rs:1015 vs splitting/join.rs:483 (unsure).** The same sine is decided on two levers: `curvature_lever_arm` in the pairing and `conic.sa` in the chord. On a sphere the chord's margin would be ρ/R of the pairing's, and its `Zero` maps to `SectionInvariant` ("malformed") rather than a grazing refusal. It's unreachable today because the split refuses spheres at its reduce.
- **S5 · boolean/join.rs:631 (unsure).** `leave` relies on `choose_roles` returning the germs' own halves, held by convention; anything else falls through to a desync.
- **S6 · germ_coplanar_conic.rs every_op… (likely, Q3).** The row accepts a refusal or a build and lets `point_in_solid` refuse, so it passes on base and on head and can't go red on this change.
- **S7 · boolean/join.rs:948 (unsure, Q7).** `walk_passes` is a linear scan inside `partners`, which itself runs pairwise, so cubic in open germs. Tour wall time was unchanged (69 s vs 70 s).
- **S8 · Q6.** The new issues are filed and scheduled (`work/contact/a-notched-full-turn-wall…`, `work/tang/a-ring-on-a-sphere-face…`). I found no unscheduled deviation.

## Probes (`probes/`)
`review_reach_dual3985_r2_probes.rs` (drop into `crates/sweep/tests/` + one `all.rs` line), `chord_dump_patch.py`, `chord_dump_compare.py`, `mutants.py`.
