# Review of PR #3845, frozen head 3f98dbb948

Lane `reach-dual3845-r2`. Wall clock 13:59–14:42 UTC, 2026-10-02. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 3 · NOTE 5.
No glimpse: I read the PR body (`get`) only and fetched no other `analysis/reach-dual/*` branch. The CI gate was checked, not assumed: run 37016247882 is green on 3f98dbb9 (test, lint, corrupt-input).
Oracle: closed form πr²h per rod, and `point_in_solid` on a 245-point lattice per built body against an analytic inside test.
Probes: `probes/reach-dual-3845-r2/` (the suite, the env-switched mutant/census patch, and outcome tables for PR and main).

## Claims
1. **Arc pass pairs the right germs: FALSIFIED in part (MINOR-1).** On a single circle it holds. Across several coaxial circles it pairs wrong germs. The bodies refused; I found no wrong body.
2. **Incidence filter: HOLDS.** Mutants "keep first" and "keep last" each red 3 rows. In the cross-circle case the filter is what refuses the wrong arc (held = 0).
3. **Bit-identical for chord-completed seams: HOLDS.** Census across all of topo and sweep (3896 tests): the new code is reached only by the PR's rows and five torus rows. The torus rows decline as on main. No probe row that main builds is refused on the PR, at any ε.
4. **Table and widening: HOLDS.** At ε 1e-9 / 1e-6 / 1e-12 the probes built 191 / 191 / 187 bodies. Every one matches the closed form (rel ≤ 3e-16), passes tier 3 and 3′, and has 0 `point_in_solid` mismatches. The rows cover θ ∈ {0, 1e-7, 0.7, π/2, π−1e-7, π}, 2-, 3- and 4-piece walls, unequal arcs (δ ∈ {1e-7, 1e-3, 0.3, 1} past a half turn), scales ×1e-3/×1/×1e3, both operand orders, and third and fourth rods on the results. The dumbbell control matches its closed form 2(π·0.3²·0.5 + π·1.5²) at all three ε.
5. **Mutants: FALSIFIED in part (MINOR-2).** Arc pass removed: 3 rows red. Filter dropped: 3 rows red. Chord-nearest in the arc pass: **nothing reds**. φ ∈ [0, 2π): **nothing reds**.
6. **`join-ranks-conic-facing-germs-by-chord`: confirmed by inspection, with one omission (NOTE-1).** No fixture reaches it.

## Findings
**MINOR-1. The arc pass pairs germs across coaxial circles.** `rest.rs:546` checks only that j *has* a frame, never that it is i's frame. `join.rs:1343` returns φ = 0 for a site at the germ's own angle: with sin = +0, `atan2(-0, −c)` = −π. The documented range is (0, 2π] (`join.rs:1324`); for the opposite sense the same site gives φ = 2π. Both are distances exactly 0 or 4π.
- *Demonstrated by execution* with `R2DBG` logging of each chosen pair:
  - Stacked split tubes (r 2 / 1), θ = 0 and π: the four segments are outer-site to inner-site radials, (2,0,1)–(1,0,1). The union refuses `Join(UnpairedLooseEnds{8})`.
  - Rotated (0.3, 0.3): same wrong pairing, refusing `RestZipUnsupported{ParallelSeamEdges}` with held = 0.
  - C-section half-tubes: diagonal pairs (1,0)–(−2,0) at an arc tie of π.
  - A two-solid stack around a middle rod: the pairs are vertical, (1,0,1)–(1,0,2).
- Every case refuses, in both orders and at all three ε. The correct pairing exists in each, and the PR's own argument would build it.
- Safety rests on downstream realization declining (`fan_edge_between` / `mint_chord`), which nothing argues. The surfaced refusal names the join, not the cause. JOIN-2 inherits this as contract.

**MINOR-2. Two mutants survive.**
- Chord-nearest in the arc pass and φ ∈ [0, 2π) both pass every PR row, the topo unit row, and all my rod rows.
- The PR's stated reason for `germ_separation` ("chord can rank back-to-back germs first") has no row.
- `germ_separation_rows` (`join.rs:2888`) pins the function, not its use in the pass, and has no row at φ → 0 / 2π. Within the PR's fixtures the two rankings are indistinguishable: on one circle at most one ≥ π segment survives the chord pass, except the exact two-half case, where everything ties.

**MINOR-3. The `ParallelSeamEdges` doc (`refusal_routes.rs:1068`) now says both edges "bound the faces the segment's end germs lie on".** `rest.rs:1035` also raises it when *no* edge bounds them (held = 0). *Demonstrated* on the rotated tube. The refusal's text misstates its cause.

**NOTE-1. Claim 6.** The mechanism holds by inspection (`join.rs` `find_match`/`loose_partners`: conic facing, chord nearest).
- The item omits `find_match`'s `is_up` filter (`join.rs:700`), which may exclude some back-to-back pairs. `loose_partners` has no such filter.
- I instrumented `find_match` over all topo and sweep tests: 228 multi-candidate rankings, chord argmin = arc argmin in all 228. So no fixture reaches it. `loose_partners` was not instrumented.

**NOTE-2. A premise of the brief is wrong.** Unequal radii (1/1.5, 1.5/1, 1/1+1e-7) **build** correct bodies (0 point-sample mismatches) rather than refusing typed. On main, 1/1+1e-7 refused at ε 1e-6.

**NOTE-3. Rotated poses never reach the zip.** Tilted-frame stacks refuse `CurvedPierceUnsupported` identically on main and on the PR, at every θ. Rotated coverage of the new code is therefore nil.

**NOTE-4. Two refusals at ε 1e-12, scale ×1e3, on the third-rod row.**
- θ = 0: newly reachable, refuses `Escalated(SectorSide)` (margin −1.08e-12, typed).
- θ = π/2: refuses `JoinDesync{"stale arc description failed re-certification"}`, identical on main, so not this PR's.

**NOTE-5. Ellipse sections were not exercised.** I built no oblique-section REST fixture.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7; Q8 partly)
- `rest.rs:534`: the chord pass inlines `germs_face_each_other`'s straight arm verbatim. The arc pass calls the helper. That makes two spellings in one loop, on top of the zip/join duplication the PR body names. Confidence: sure.
- `rest.rs:510`: the pass switch is data, `Option<Vec<Option<Frame>>>`, re-entered by `continue` from the `best == None` arm. I would have written two explicit passes. Confidence: likely.
- `join.rs:769` says "`None` is a claim". `conic_frames` folds NoArm and CylinderPinch into the same `None` as "straight". This is harmless here only because the arc pass ignores `None`. Confidence: likely.
- The global minimum compares `r·φ` across germs on different radii (the r at i's site), so cross-segment ranking mixes units of arc. This is the root of MINOR-1. Confidence: likely.
- Q3: the `(0, 2π]` endpoint and the chord-vs-arc choice have no red-able row (MINOR-2). Confidence: sure.
- The `face_of` closure at `rest.rs:1011` re-spells `Body::face_of_half_edge`. That helper's doc sanctions keeping a walk for distinct refusals. Confidence: unsure.
- Q8: `rest.rs` is 2006 lines. I read its structure and all touched and adjacent functions (`incident_faces`, `halves_at`, `mint_chord` header), not the whole file end to end.
