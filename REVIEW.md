# Review r2 — PR #4397, the annular one-segment tube through a plate

Frozen head `06d1e2e0`, base `db51132f`. Base = head with `stands.rs` reverted (the only `src` file the PR touches).
Probes: `crates/sweep/tests/annular_tube_review_r2_probes.rs` (10 rows). Every op runs in both orders and goes through
`differential::outcome` **with the closed form** (not the measured volume), plus a `volume ± pad` check, `check_mesh` and a solid count.
Mutants: one instrumented build of `stands.rs` (`R2_MUT` / `R2_K` / `R2_TRACE`), not committed.
Lane isolation: I read no other review branch or session. Nothing glimpsed.

**Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 4 · NOTE 6. No wrong body and no wrongly decided role were found.

## Claims

1. **Bodies right — HOLDS (executed).** 300+ op lines on head, all at their closed form: |dv| ≤ 7e-15, `volume_pad` 0, meshed and `check_mesh` clean. Poses:
   - a thin ring (1 / 0.999) at four azimuth pairs, sunk and through;
   - a tangent-close inner circle (gaps 1e-4 and 1e-6, the vertices near or far);
   - 3, 4, 5 and 6 nested one-segment circles, sunk and through;
   - a square with a circle hole, a circle with a square hole, a circle with circle and triangle holes;
   - a stadium with a circle hole, and a stadium ring;
   - the tube turned over;
   - rings 0.9995 / 0.9999.

   Not SOUND: only `t3p=false` on multi-solid results (2–6 solids, each one in another's bore). Their volume and the other flags are right, and the cause is `CensusUndecidable` (`r2_t3p_cause_of_the_bore_results`), the PR's (*) class.
   Base → head: every base `SectionLoopUndecided` on these poses became a SOUND or T3′-class body; no other line moved. Re-derived area forms: Σ±πr², the stadium 4ar+πr², the polygons by shoelace.
   Flush coaxial cylinders and caps flush on the plate refuse `CurvedPierceUnsupported` / `UndeclaredCoincidence` on both trees, so they never reach the ladder.
2. **A new witness is never wrong — HOLDS (inspection, backed by the bodies above).** Each `across_edges` candidate is used only if `point_in_face` certifies it strictly inside (`stands.rs:283-287`).
   - Both signs of `normal × m′` are tried; the side is never assumed.
   - A sliver, reflex corner or hole near the edge can only make a candidate fail certification, never pass wrongly.
   - On a conic edge a poor tangent costs reach, not soundness.

   The residue is a NaN path and a new error surface (N4, N5).
3. **Every reader still right — HOLDS for the join; the other readers were not reached (executed trace).** Traced with `R2ACROSS` over all 5,223 topo and sweep tests: the new rung is taken only via `complex_side` ← `resolve_roles_geometric`. `shell_verdict`, `witness_insides` (check 10, pieces sort) and the census never take a step-across witness (M4).
   - No reader relies on rung 3 failing. `on_verdict` reads only an all-`On` tally, and a disc point on a coincident shell reads `On` (inspection).
   - `loop_roles` gains cross-checking: two decided loops that agree now raise `SectionLoopMixed` (`join.rs:3165`).
4. **Nothing else moved — HOLDS (executed).** Same binary build, base vs head:
   - `pinch_runs_battery`: 3,024 lines, 0 moved;
   - `rc_wide` shards 5, 20, 34, 48, 62, 77 of 84 (not the PR's): 2,880 lines, 0 moved (5,770 SOUND, 133 EMPTY ok);
   - full `-p topo -p sweep`, default profile (the slow set included, so every CLEAVE / tier-3 / pieces / `shell_witness` row, 328 of them): 5,223 / 5,223 pass;
   - editor-core `on_verdict_rows` and `refusal_concision*`: 23 / 23 pass.
5. **Mutants are real — PARTLY (executed, full topo + sweep runs).**

   | mutant | result |
   |---|---|
   | drop `across_edges` | the PR's 5 witness rows red (plus my T3′ row) |
   | `+` side only | the same 5 red |
   | `k ≤ 2` and `k ≤ 3` | only `two_nested_annuli_build` red |
   | `−` side only | **survives**, 5,223 / 5,223 (M1) |
   | `k ≤ 4` | **survives**, the PR's whole suite green (M2) |
   | edge steps before vertices | **survives**, and moves 0 lines on pinch + 3 rc shards |

   The last is expected under the ladder's premise, but no row pins the claim that "faces whose vertex candidates certified keep their witness".
6. **The sweep is complete — HOLDS for the sample; the count is wrong (N1).**

## Findings

- **M1 MINOR test-gap (executed)** `stands.rs:330`.
  - The `m + step` half of every candidate pair never decides anything. Minus-only passes all 5,223 tests and the turned-over tube (`r2_turned_over_tube`); plus-only reddens the PR's rows.
  - So either the half-edge / curve sense makes `−` structurally the interior side here (then `+` is dead work), or a pose needing `+` exists and no row has it. I could not build one.
- **M2 MINOR test-gap (executed)** `stands.rs:292`.
  - `ACROSS_HALVINGS = 12` is pinned only to k ≥ 4 by the PR's suite (k = 4 keeps it all green).
  - `r2_thin_nested_rings` at w = 1e-3 needs exactly k = 12: SOUND at 12, `SectionLoopUndecided` at 11. Adopt it or an equivalent row.
- **M3 MINOR doc / claim (executed)** `join.rs:3148-3152`.
  - "Neither deciding … cannot both do unless their faces are all curved" is false. Three nested planar rings of width and gap 9e-4 (`r2_thin_nested_rings`) refuse `SectionLoopUndecided` on head, every region face planar.
  - The cliff is the fixed ratio `L/4096` beside every edge, independent of the band, and it is stated only in the work row and the PR body, not at the refusal site. The refusal is honest and typed: not a wrong body.
- **M4 MINOR test-gap (executed trace)** The PR says the shell witness, check 10 and the pieces sort "all get the same reach", but no test anywhere drives a step-across witness through those readers (claim 3). Their correctness at the new rung rests on inspection.
- **N1 NOTE (executed grep)** PR body §Sweep: "45 `lerp(…, 0.5)` sites in topo" does not reproduce.
  - Topo `src` has 9 `lerp(` calls, 5 of them with 0.5, and 38 `from_f64(0.5)`.
  - I read 8: `reduce.rs:3179`, `chord_join.rs:2673/2347/2465`, `solid_contain.rs:2991/3290`, `classify.rs:638`, `splitting/finish.rs:1000`. None builds a face-interior witness.
  - `finish.rs:967` `whole_body_side` is a vertex-plus-curved-edge witness with no face rung. Its premise (an uncut closed body cannot have all of them ON) is argued in its doc; not this defect.
- **N2 NOTE (executed)** Newly built bodies hit a tessellator limit.
  - Rings 0.9995 / 0.9999 with opposed vertices: `mesh::tessellate` refuses `Triangulation { face }` on all 12 ops. Aligned vertices mesh.
  - Base refused these at the join. Typed, outside this PR's code; worth a row in `mesh`.
- **N3 NOTE (inspection)** `an_annular_tube_through_a_plate.rs:130` hands `outcome` the *measured* volume, so its 1e-7 check is vacuous. The adjacent pad assert carries the closed form, and the pad measured 0, so it is tight. Readers of the suite should not take `SOUND` as volume evidence.
- **N4 NOTE (inspection)** `stands.rs:323`: a zero midpoint tangent (a spline with a cusp, a degenerate parametrisation) makes `normalize()` NaN. The NaN candidate goes to `point_in_face` unguarded and relies on the walk refusing in band.
- **N5 NOTE (inspection)** New error surface.
  - Before the PR, a one-vertex face never reached `certified_in_face`; now it does.
  - A non-inconclusive refusal there (`CorruptLoop`: a whole-turn scaffold circle, a conic wound past a period) now propagates out of the ladder as `LadderRefusal::Containment`, where it used to return `None`.
  - No pose found.
- **N6 NOTE (executed)** 18 pre-existing tests now decide through the new rung (`a_plane_across_a_one_face_wall` ×6, `planar_ring_arc_closure` ×2, `axis_lap`, `run_walls_built`, `snowman`, …). All stay green, but the claim that "the only change is a face that offered none may now offer one" reaches further than the annulus.

## Style

- **Q1 (likely)** `across_edges` re-walks the loops and re-spells rung 2's edge → curve → `certified()` lookup with the same desync string (`stands.rs:310-318` vs `:226-234`). It also re-spells the edge midpoint as `ders1(mid_param)` beside rung 2's `curve.mid_point()`: two spellings of one point.
- **Q2 (sure)** `L` drifts between texts: the code doc says "the **sum** of m's distances to the two ends" (`stands.rs:296`); the PR body and the work row say "m's distance to the edge's two ends".
- **Q3 (sure)** M1, M2: two halves of the new code no row can turn red. The PR's own rows also pass when the candidates come before the vertex ones.
- **Q4 (sure)** M3: `join.rs:3148` cites a premise that this PR narrows but does not make true. In the doc-rotted vs code-drifted sense, the doc rotted.
- **Q5 (sure, clean)** The module doc (`stands.rs:1-51`) matches the code after the edit. "Curved faces offer no interior witness" is still true.
- **Q6 (likely)** The `L/4096` reach is a measured, quotable limit (the work row's "Any planar face wider than L/4096") with no guard and no reason for 12 at the claim site. It needs M2's row or a stated reason.
- **Q7 (unsure)** I would not have used a halving search. A scan line through the edge midpoint, cut against the face's own loops, gives a constructive interior point with no 4096 cliff. Taste only.
- **Q8 (sure)** Read `stands.rs` end to end (512 lines). No accumulation beyond Q1.
- **Fix minting its own defect (likely)** Closing "rung 3 reads vertices only" adds a new fixed-depth limit (M3) and a new unpinned branch (M1): a fresh instance of "a witness source with a blind spot no row shows".

REVIEW COMPLETE
