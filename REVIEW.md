# Review r2: PR 4038 at frozen head `e2e114be1`

**Verdict: APPROVE-WITH-FIXES** · MAJOR 0 · MINOR 3 · NOTE 5 (+ style)

Nothing I built ships a definite wrong body, and the pre-pass never left two vertices at a point. Every newly built body holds one vertex at `v`. The five PR mutants I re-ran reproduce exactly, and the sampled batteries do not move except as the PR says. The fixes are text: the new refusal's user-facing message is false for most of its lines, and the filed residue row's "right body" premise is not established (m1). Two rows also need the reach I measured (m2, m3).

**Method.** I used release builds in separate target dirs:
- head `e2e114be1`;
- base = its merge parent with main, `8793177bc` (the brief's `81dde823` is older; main moved, see N4);
- an instrumented copy of head with env-switched mutants and a split trace (`review-r2/instr-mutants.patch`).

My battery is `crates/sweep/examples/r2_pinch_probes.rs`. Every line is `differential::outcome`: t2, t3′, the certificate, the legal-operand union, and the volume to 1e-7. The volume comes from my own oracle:
- polytopes: each convex prism piece clipped by the cube's half-spaces, divergence theorem;
- cylinder: pieces mapped into the cylinder frame, slice polygon ∩ disk exact, adaptive Gauss–Kronrod.

Every built body also gets `pts=k` (points holding ≥2 vertices) and `FACE2V` (one face's loops meet two vertices at one point). Outputs, base/head pairs and `cmp.py` are in `review-r2/`. Lane isolation kept: I read only PR 4026's three review branches the brief named, and no other branch or session.

## Claims

1. **No wrong body ships: HOLDS (executed).**
   - The battery:
     - 8 corners: Ltop, Lbot, notch307, notchbot, shallow200, and new vee300 (≈300°), vee224bot (≈224°, bottom) and asym;
     - × 120 Fibonacci directions;
     - × `v` on the cube's face, on an edge (3 turns) and at a corner (3 turns);
     - × every op, both orders: 40 320 lines.
   - Plus the same corners at near-tangent tilts (±1e-3, ±1e-6, 1e-8 off each corner edge, all placements, 40 320), and a **curved pierced face**: a cylinder with `v` on its side, 60 directions × 3 axis turns × arc junction off/on `v` (17 280).
   - Base→head:

     | battery | refusal→SOUND | refusal→refusal | refusal→BAD | SOUND→refusal |
     |---|---|---|---|---|
     | cube | 397 | 210 | 0 | 0 |
     | near-tangent | 406 | 182 | 19 | 0 |
     | cylinder | 218 | 135 | 8 | 0 |

   - All 1 048 bodies main refused and head builds (BAD lines included) have `pts=0`: the crossed pinch is one vertex everywhere.
   - The 19 near-tangent BAD are all d = 1e-8 ∩, with exact volume, t2 and the certificate. Every t3′ finding is `CensusEscalated`. I re-read each error list (`head-nt-t3.txt.gz`): none is definite (m3).
   - The 8 cylinder BAD pass t2, t3′ and the certificate, and miss only by |dv| 1.1–2.8e-7 on volumes near 230. That is the curved-mass floor: base has 80 identical-class lines among bodies it already builds.
   - "First pair the orbit offers" is never the wrong pair, on two tests:
     - **choice:** my mutant `R2M_LAST` takes the *last* qualifying corner pair. It is line-identical to head on all 40 320 cube lines.
     - **surface check:** `R2M_ANYSURF` crosses any two outer faces. On the rows, it refuses loud ("minted-edge description failed certification") rather than build BAD.
   - Two vertices at one point with a face meeting both: never new (N1).
2. **The pre-pass is sound and complete: HOLDS for what is reachable (executed + inspection).**
   - The trace (`trace-summary.txt`) shows 657 collisions: 447 splits (374 two-face on side a, 44 two-face on side b, 29 one-ring) and 210 `PinchUncrossed`.
   - Every collision has exactly one earlier fusion on each side. No chain was reached. `R2M_MOVEONE` (move only the first earlier corner) is line-identical on all 40 320 lines.
   - A cycle in the union-find needs both operands to hold two vertices at one point, crossed. `NonManifoldResult` / `SharedVertexCrossings` refuse that upstream (inspection, `mod.rs` variants).
   - The `ZipCorrespondence` exhaustion (`zip.rs:276`) is typed but unreachable while A and B keys are disjoint. Moving every earlier corner of `v` isolates `v`, so a pair never collides twice. The collisions are bounded by `pairs`, and the loop runs `pairs + 1` passes (`zip.rs:218`).
   - `rest.rs:1630,1675` `zip_seam` callers: the refactor keeps the error strings and their order (inspection of the diff). The JOIN-1 declared battery is byte-identical (claim 5).
3. **Removing the ∩ rule is right: HOLDS (executed; derived only as far as the PR's argument).**
   - Every face-placement two-run ∩ builds `SOUND` on head: 350/350 lines over all 8 corners. Base built 226 and refused 124 `JoinDesync` ("derived ring role order" / "every chord arc").
   - Near-tangent face ∩ likewise. No pose needed the start rule.
   - The PR's ∩-start mutant reddens 2 rows.
   - The geometric reason I accept: the facing is a property of the arrangement on the pierced face, not of the kept side. The op only picks halves, and the rule was only masking the self-fusion that `cross_pinches` now crosses.
4. **`PinchUncrossed` is correct, not an over-refusal: PARTLY.**
   - It is never wrong: the R2M_OUTERLOOP variant (an outer loop may cross) turns all 210 of my lines into gate refusals. 184 are `LoopRoleInverted` (a bow-tie outer loop) and 26 are `RingMeetsOuter` (nested). None is BAD (`instr-outer.txt.gz`).
   - But the row's "right body needs a shell divided at a vertex" is not established, and the Display text is false for these lines (m1).
5. **Nothing else moved: HOLDS (executed, base vs head).**
   - `rc_wide_battery` at 84 shards (one (profile, turn) pair each): 37 024 of 40 320 lines print on both trees. The other 8 shards hit main's `orbit_step_at` panic on both trees. The printed lines are byte-identical.
   - `join1_r1_declared_battery` 27 000: identical.
   - `join1_r1_reflex_battery`: 980 identical (main panics at the same pose on both trees).
   - `pierce_runs_battery`: 604 lines before main's panic on both trees, 17 moved, all refusal→SOUND. They are exactly the two P0 families: 8 ∩ at `i=0 j=3..5`, `i=2 j=5`, and 9 cube ∖ prism at `i=6..8 j=0..2` (`pierce_runs_battery.diff`).
6. **The mutants are real: HOLDS (executed).**
   - Clean head: 8/8 rows green.
   - The PR's five I re-ran give exactly the red counts the PR states: no `cross_pinches` 5, no two-face 3, no one-ring 1, ∩-start 2, outer-loop-crosses 1.
   - Mine: `R2M_ANYSURF` reddens 4 rows. `R2M_LAST` and `R2M_MOVEONE` survive every row and every battery: equivalent on all reachable poses (S4).

## Findings

**m1 (MINOR, executed + inspection; claim/contract). `PinchUncrossed`'s message, and the residue row's premise, misdescribe most of its lines.**
- The Display text (`mod.rs:3228-3230`) says "no face of either solid runs through that point on both sides".
- In 210/210 of my lines, the outer-loop variant finds a kept face whose *outer loop* runs through `v` with one corner on each side of the pre-pass's own partition. The PR's row reports the same for 491 of its 509 lines.
- For the bow-tie faces (184 of mine, 19 of the PR's), the right body is a single vertex whose face's outer loop meets itself at `v`. That is the very shape the two-face `kef` crossing already ships and the gate accepts.
- What is missing is an Euler sequence that splices a face's two loops back into one at a shared vertex, not a shell divided at a vertex.
- So the premise at `mod.rs:2011-2012` ("needs the pinched shell divided before the zips") and in `work/join/a-pinch-no-kept-face-can-cross-refuses.md:55-60` is not established.
- Fix: correct the message and doc, and restate the row's shape-to-give per sub-family: bow-tie, nested, and the holed 18 with no face twice. Confidence: likely.

**m2 (MINOR, executed). The `PinchUncrossed` residue reaches beyond the filed row's instance list.**
- It also reaches face placements: cube ∖ prism on vee300 (11), asym (10) and vee224bot (5).
- It also refuses **unions**: vee300 at a cube corner (pc ∪, cp ∪) and notch307 on an edge (pc ∪).
- It also reaches near-tangent tilts (182) and the cylinder (35).
- The row lists edge / 315° / 225° / holed instances and names no ∪. Add the class statement and these poses.

**m3 (MINOR, executed). 19 more refusal→BAD near-tangent ∩ lines (all escalated), at corners the P0 row does not list.**
- Corners: notch307, notchbot, shallow200, vee300 and asym, at d = 1e-8.
- Three vee300 lines escalate on **edge-face** predicates (`pm_census_ef_residual`, `pm_census_ef_cut_gap`). The P0 row `near-tangent-boolean-results-ship-with-an-escalated-tier-3-census.md:20-23` says every finding is edge-edge.
- Add the poses and widen the predicate statement.

**N1 (pre-existing, executed).** Main ships `SOUND` bodies in which one face's outer loop passes two distinct vertices at exactly `v`. Example: `notch307 fib117 edge psi=0.3 pc S`, with vertices 33v1 and 34v1 at (2,1,1) on one outer loop. That is 5 cube + 6 near-tangent + 2 cylinder lines, identical on both trees. `outcome` and t3′ cannot see it, and it breaks the shared-point ruling. It is not this PR's, but it is unfiled as far as I found.

**N2.** On the cylinder, 76 cylinder ∖ prism lines move `SelfLoopEdge` → `ResultInvalid{VolumeUncomputable{RingOnCurvedFace}}`: the one-ring crossing on the curved wall meets a pre-existing volume-lane limit. This is refusal→refusal, but it is a crossing the pre-pass makes that then cannot be measured. Worth one line in the residue row.

**N3.** Curved reach is real. 218 cylinder lines go refusal→SOUND (t2, t3′, the certificate and the oracle volume). The operand check fails on every cylinder line on both trees, which is why I graded the cylinder without it.

**N4 (dispatch premise).** The brief's main is `81dde823`. The head's merge parent is `8793177bc`, where `orbit_step_at` panics in `rc_wide_battery` (8 of 84 shards), in `join1_r1_reflex_battery` and in `pierce_runs_battery`. All my comparisons are against `8793177bc`.

**N5.** `R2M_LAST` and `R2M_MOVEONE` are equivalent mutants on every reachable pose, and the exhaustion guard is dead (claim 2). The multi-fusion `moving` path has no pose that reaches it.

## Style (exercised Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 on `zip.rs`, read whole, 546 lines; `ops.rs` hunks only)

- **S1 (Q1, likely)** `zip.rs:237` vs `zip.rs:447-454`. The zips' fusion order (pair 0, then `n−1`…1) is written twice: once in `cross_pinches`' replay, once in `zip_seam`. Only a doc sentence ties them. The PR factored `align`/`section_cycle` so both read the same pairs, but not the order they are fused in. If either order changes, the pre-pass silently stops predicting the zips. Look also at `splitting/reassembly.rs:127`, a third copy.
- **S2 (Q7, unsure)** `zip.rs:343-348`. The two-face crossing tests `surface` key equality but not `sense`, and `kef` does not check sense either. Two back-to-back faces of one surface would merge. I found no reach.
- **S3 (Q2/Q6, likely)** `mod.rs:2005-2012`. The variant's doc asserts the residue's cure ("needs the pinched shell divided") at the claim site, in code, before anyone has measured it. That is a deferral written as a derivation (see m1).
- **S4 (Q3, sure).** No row pins *which* crossing site is chosen, or that every earlier corner moves. LAST and MOVEONE survive all 8 rows. That is fine while they are equivalent, but nothing goes red when a pose arrives where they are not.
- **S5 (Q5, likely)** `zip.rs:1-28`. The module doc still describes only the zip. It does not say the module now also rewrites kept faces' topology before the zip (`mev`+`kemr`/`kef` on non-section faces), or that `Descendants` gains face rows from it.
- **S6 (Q7, unsure)** `zip.rs:218-275`. Each split restarts the whole replay, with a union-find plus a parallel per-vertex `fused` corner map. A single pass that splits and continues would read more simply. Taste.
- **S7 (Q4, likely)** `vtxfac.rs:749-754` is updated. The two rows the PR says it answers (`the-intersection-ring-facing-is-measured-not-derived`, the notch/shallow "every chord arc" row) stay open with their old premise. The PR leaves this to the orchestrator; Every two-run face ∩ in my battery builds, notch307 and shallow200 included (claim 3), but I did not re-run r1's 124 lines.

REVIEW COMPLETE
