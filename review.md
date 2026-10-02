# Review of PR #3814, frozen head 08708983c7

Lane `reach-dual3814-r1`. Wall clock 05:59–07:20 UTC, 2026-10-02. **Verdict: APPROVE-WITH-FIXES** — MAJOR 0 · MINOR 5 · NOTE 3.
No wrong body anywhere: **~700 boolean outcomes**, every body checked against my oracle. That is closed-form membership sampled through `point_in_solid` (no samples wrong, none unanswered), plus volume against closed form, tier 3 + census, and shell count. Bodies off the closed form (misfit shafts) are judged on membership only.
Local (debug, `--profile ci`), topo + sweep at ε 1e-9 / 1e-6 / 1e-12: 3843/3843 green each. `demos/tour` (lily) was not run. The branch moved past the freeze (a main merge, `5fc57be9`, `a4a11a5b`); not reviewed.

## Exercise (probes/review_3814_r1.rs; release build)
- **Widened grid**: 7 azimuths (0, 1e-7°, 7.3, 60, 119.9, 180, 243.7) × 4 spans (through, flush, flush-lo `y∈[1,2.5]`, flush-hi `[0.5,2]`) × 6 poses × both operand orders = 336 rows. Result: 186 bodies, all right; 34 band escalations at 1e-7° (`Coincidence(Sectors)`, margin 1.5e-9; correct); 102 `Merge(Pcurve LoopDiscontinuity)` (MINOR 1).
- **Radii × scales**: (r, R) ∈ {(.5,1.5), (.3,1), (1.7,2.5), (.05,.06)} × s ∈ {1e-3, 1, 1e3} × 3 poses × 3 azimuths × 2 spans = 216/216 right.
- **Misfit shafts, declared Rest**:
  - radius ±1e-4, offset 1e-4, tilt 1e-5 rad: `ContactContradicted`;
  - offset / tilt 1e-9: typed `Escalated`;
  - radius −1e-12, offset / tilt 1e-13: unions, right to the oracle.
- **Blind shafts and other ops**: blind shafts (4 spans) refuse typed. ∩ and ∖ (both orders) refuse `FallbackExtentUnsupported`, as filed.
- **Results reused**: (collar ∪ shaft) ∪ a second collar threaded on the protruding top is right at 0° and 60°, except order-dependent refusals (MINOR 1).
- **Mutants**: whole topo + sweep suites per mutant; reduce/carrier_cross/vtxfac restored after each.

## Findings
1. **MINOR — `shaft ∪ collar` refuses where `collar ∪ shaft` unions** (`crates/topo/src/boolean/rest.rs:361`). DEMONSTRATED (`r1_grid`, `r1_widened_unions`).
   - At every off-seam azimuth, a protruding shaft (through, flush-lo, flush-hi) as operand A refuses `Merge(Pcurve { LoopDiscontinuity })`; at 0° the flush-lo/-hi spans do too.
   - The output stage is rejecting a result loop the zip built, and its recourse text ("repair the face's boundary") points the user at the operands.
   - The same happens in the reuse case (`u ∪ c2` at 60° refuses; `c2 ∪ u` unions).
   - On the merge base these rows refused earlier (CurvedPierce / CurvedBoolean). The PR's rows test only `collar ∪ shaft`, and the title claims "unions at any azimuth". Unfiled.
2. **MINOR — the crossing layer's interior checks have no committed row that goes red** (`reduce.rs:1414`, `reduce.rs:1541`, `carrier_cross.rs:97`). DEMONSTRATED by mutants:
   - M7 (line arm skips `on_carrier_crossing`), M6 (circle arm skips it), M6+M7 together, and M3 (lone-ring-vertex candidate dropped) each turn **0** suite rows red.
   - The layer is load-bearing elsewhere: my `r1_split_bore` (bore = two full-turn faces split at y=1.5; no plane touches that circle) unions right at 0° flush / flush-lo, and under M6+M7 refuses `UnpairedLooseEnds`.
   - Only `meetings_rows` guard the closed forms (M2: coplanar circles dropped → red).
3. **MINOR — the stated root cause 1 is not what the fixtures exercise** (`crates/sweep/tests/mate2_r1_probes.rs:30-33`, PR body "Root causes", the work item). DEMONSTRATED.
   - The new doc says the crossing "is interior to both edges: only an on-carrier crossing step sees it". Under M6+M7, probe 1 and all of `full_turn_bore_mate` still union right (probe 1: additive to 2.7e-15, tier 3, census clean).
   - Those pierces are recorded at the collar's cap planes, whose inner rims are the bore's rims. What unblocked these fixtures is `Placement::declared`'s all-`Elsewhere` → no-event (`reduce.rs:2293`).
   - The sentence enshrines an unchecked causal story (`memories/review-and-dependency-policy.md`).
4. **MINOR — `vtxfac`'s `declared_rest` gate is unguarded** (`crates/topo/src/boolean/vtxfac.rs:389`). DEMONSTRATED (M5: gate dropped → 0 rows red).
   - My undeclared probe (box face tangent to a cylinder, corners on it; ∪ ∩ ∖, 3 azimuths) refuses `CurvedPierceUnsupported` before reaching it.
   - The C4 gating is therefore correct by inspection only.
5. **MINOR (Q4) — an `Undecided` endpoint keeping the door now has only a truth-table guard** (`mate2_r2_probes.rs:210`). DEMONSTRATED.
   - The removed comment called this row "the unit's only `cargo test` guard that `Undecided` keeps the typed frontier", and it was re-pinned to a union.
   - M8 (`Undecided` → `Recorded` in `declared`) turns red only `undeclared_rule_rows::the_declared_rule_reads_all_elsewhere_by_the_interior_certificate`. No reachable fixture guards it.
6. **NOTE — a second blind-shaft payload is unfiled.** A shaft starting inside the bore at 0° (`y0=1.5,h=1`; `y0=1.2,h=0.5`) refuses `RestZipUnsupported(ChordBetweenIsolatedPierces)`. The filed `work/zip/blind-shaft-…` names only `ChordEndpointRevisited`. Typed. DEMONSTRATED.
7. **NOTE — `Placement::declared(…, false)`'s all-`Elsewhere` door is reached by the unit row only** (M4 → only that row red). Sound by inspection: no-crossing + both ends out ⇒ wholly out; `Undecided` is checked first.
8. **NOTE — claims 1 and 3, by inspection** (`carrier_cross.rs:70`, `rest.rs:964`):
   - The candidate set is complete for the pairs it answers: line×circle at the plane point, the closest point of skew lines, the common line of crossing circle planes, coplanar circles, plus every boundary vertex for shared stretches. `Unread` covers a line parallel to a circle's plane, non-line/circle carriers and uncertified curves.
   - `mirror_edges` mutates `red` by value, so `Ok(None)` cannot leak chords. Every body I sampled is tier 3 + census clean.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q7; Q8 partly — `carrier_cross.rs` read whole, `reduce.rs`/`rest.rs` not)
- `carrier_cross.rs:299,351,352`: one row name, `bool_carrier_cross_disc`, decides three different quantities (a discriminant, `r1+r2−d`, `d−|r1−r2|`). Two spellings of one concept. *likely*
- `carrier_cross.rs:179,199,310,333`: `Escalated{Crossing(OnEdge)}` is spelled four times, one of them as a local closure. *sure*
- `carrier_cross.rs:261`: line×line is levered at a unit metre ("a sine over a metre"), while the circle pairs use the radius. That is scale-dependent next to its siblings. *unsure*
- `rest.rs:241,274`: `patch_faces` runs twice per operand (before and after the mirror), and the first result is shadowed. *likely*
- `offer_rows.rs` `SITES`: four more hand-written census rows. A hand-kept list, the class the census exists to catch. *unsure*
- Q3:
  - `mate2_r1_probes::probe_misaligned…` still asserts nothing although it now unions. *sure*
  - `full_turn_bore_mate.rs:159` can't go red on any refusal (by design), and runs identity pose only. *likely*
  - `full_turn_bore_mate.rs:106` never swaps operands (see 1). *sure*

## Fixes asked
Before merge, either fix or file 1 (operand order). For 2/4/5, add red-able rows: the split bore, the swapped order, and an `Undecided` fixture. Correct the sentence in 3 (probe doc, work item). File 6.

No glimpse: I fetched only the PR branch and this lane's own pre-created branch (which equalled the frozen head). Read the PR body via `get` only.
