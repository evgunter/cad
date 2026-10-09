# Review r1 — PR #4397, annular one-segment tube through a plate

Frozen head `06d1e2e0`; base `db51132f` (the main merged at `8b3116d0`). Release builds, a separate
target dir for each of base, head and an env-switched mutant tree (`CAD_R1_MUT`). Probes:
`crates/sweep/tests/annular_tube_review_r1_probes.rs`; outputs and the mutant patch in `review-r1/`.
Lane isolation held: I read no other review branch, session or PR comment.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 4. No wrong body or wrongly decided role
found in 552 probe runs plus 108 declared runs. The fixes are test pins and prose.

## Claims

1. **Holds (executed).** I re-derived every closed form in the probe file: area × height, with
   the plate overlap `area·(min(z₁,1) − max(z₀,0))`; slot area `s/2·(ro²−ri²)` plus `2h²(π/2−1)`
   for the bulged ends. Families, each op in both orders: thin rings ri ∈ {0.99 … 0.99999} at three
   azimuth pairs, sunk and through; an inner circle tangent-close with gap 1e-2 … 1e-7; three and
   four nested rings; six circle/polygon mixes; C-slots (square and bulged ends, four sweeps).
   - Base → head moved 210 lines, all from `ERR Join(SectionLoopUndecided)`: 190 to `OK SOUND`, 20
     to `OK BAD` with only `t3p=false` (the known two-solids-in-bore class,
     `work/restread/census-cross-solid-…`). Every volume is its closed form; nothing else moved.
2. **Holds (inspection + executed).** `across_edges` only proposes candidates.
   - `certified_in_face` (`stands.rs:283`) still certifies each one through `point_in_face`. That
     call projects (`containment.rs:1797`, never `OffPlane`), so off-plane rounding at `m` is moot.
   - Both signs of `normal × m′` are tried and the certifier decides, so no side is assumed.
   - Under the ladder's no-crossing premise, any certified interior point is as valid as a vertex
     chord. A reflex corner or a step that crosses a hole is either certified (then inside) or
     dropped. A sliver thinner than `L/4096` offers none, which is a refusal, not a wrong answer.
   - Conic edges: the step runs along the curve normal at `m`; the certifier guards a poor
     tangent. Unexamined (unsure): a NURBS edge with `m′ = 0` normalizes to NaN (`stands.rs:327`).
3. **Holds for the shell witness (executed); check 10 and the pieces sort are unexercised.** The
   declared plug-in-bore probes (`r1_probe_declared_flush`) reach `shell_verdict`:
   - The one-segment plug at az (0,0) in three heights, and the ring plug at both az pairs:
     base `CoincidentShell{Mixed | Unpaired}` → head `EMPTY ok` (∩), and ∖ `SOUND` at the closed
     form. That is 12 runs (`review-r1/declared-*.txt`), all decided rightly.
   - A logging mutant over the sweep suite (2479 tests), topo unit (1715) and topo integration
     (984) shows the new rung certifying only in sweep tests. No topo row reaches it, so no
     check-10 or pieces-sort row reads a new witness.
   - No reader relies on rung 3 failing. `shell_verdict` reaches `on_verdict` only on an
     all-`On` tally, and a truly coincident planar face reads `On` at its new witness too. I
     could not reach that pose: identical shells refuse `FallbackExtentUnsupported` earlier.
4. **Holds, but the evidence is vacuous (executed; m2).** Base vs head: `pinch_runs_battery` 3024
   lines and `rc_wide` shards 5, 20, 34, 48, 62, 77 (6 × 480) moved 0. Sweep `all` (CLEAVE/HONE
   included) and topo unit are green on head; topo integration too, but for
   `sphere_twin_rows_interval::…tight_k…`, which needs nextest's per-process K and passes alone.
5. **Partly falsified (executed).** Over the PR suite, the sweep suite and every probe: `drop`
   and `+step only` go red on 5 PR rows; `k ≤ 2` on `two_nested_annuli_build` only;
   **`−step only`** and **across before vertex** survive everything (m1).
6. **Holds on my sample (inspection).** My own grep, shaped differently from the PR's
   (`lerp(|from_f64(0.5)|midpoint(|mid_point()` over non-test topo src), returned ~50 hits. I
   read the point-building ones; none is a face-interior witness built from vertices:
   `reduce.rs:3179` (a segment point, off-circle test), `chord_join.rs:2673` (a closing chord's
   midpoint), `finish.rs:1000` (a carrier midpoint, plane side), `shell.rs:3645`,
   `ring_path.rs:314`, `containment.rs:1583` (parameter midpoints), `reduce.rs:5630`,
   `attach.rs:2909` (test intersection witnesses).

## Findings

- **m1 MINOR (test-gap, executed)** `stands.rs:330`, `:278-287`. Mutant `out.push(m - step)`
  alone survives the PR suite, all 2479 sweep tests and every probe: the `+` half of "either way"
  has no row. Moving `across_edges` before the vertex candidates survives everything too (pinch,
  rc5 included), so "faces whose vertex candidates certified keep their witness" is unguarded.
  `k ≤ 2` is caught by one row.
- **m2 MINOR (claim/test-gap, executed)** PR body, "Batteries". The logging mutant prints 0
  `across_edges` calls over `pierce_runs_battery`, `pinch_runs_battery` and rc shard 5, and with
  `across_edges` dropped pinch and rc5 still move 0 lines: that evidence could not have gone red
  for this change, and the PR body should say so rather than cite it as support.
- **m3 MINOR (test-gap, executed)** `an_annular_tube_through_a_plate.rs` pins only the join's
  `complex_side`, though the PR says every reader "gets the same reach". The shell-witness change
  (base `CoincidentShell` → head decided, above) is real and unpinned; the CLEAVE row's "No
  fixture reaches it yet" (`work/cleave/the-uncut-shell-…:25`) is met for planar faces by the
  declared plug on base (`Unpaired{face}` on a one-vertex disc). No row reaches check 10 or the
  pieces sort through the new rung.
- **m4 MINOR (code, pre-existing, unfiled, executed)** `mesh::tessellate`: 118 newly built
  results (thin rings at (0,π)/(1,4), the tangent-close circle) fail at 5e-3 with
  `Triangulation{face}`. It depends on azimuth (aligned ri = 0.999 meshes clean), and base fails
  the same on the close (π,π) poses it already built: the mesher's defect, not this diff's. No
  issue for it in `work/`.
- **n1 NOTE (doc, inspection)** The JOIN row (`…section-loop-undecided.md:72`) and PR body say
  `L` is "the midpoint's distance to the edge's two ends"; the code sums them (`stands.rs:297`).
  "Any planar face wider than `L/4096` beside one of its certified edges" (`:75`) overclaims: the
  reach is only along the normal at the parameter midpoint, at 12 dyadic depths.
- **n2 NOTE (inspection)** "The one change in behaviour" is understated. A new non-decisive
  witness moves the `Tally` ranking from `Neither` to `InBand`/`Blocked`: `on_verdict` becomes
  `ShellWitnessExhausted`/`Containment` (`shell_witness.rs:196-205`), `Touching` becomes
  `Refused` (`stands.rs:503-509`), `SectionLoopUndecided` becomes `Containment` (`join.rs:3175`).
  A ring that newly decides can turn a built `(Side, Undecided)` into `SectionLoopMixed`
  (`join.rs:3165`). None was observed.
- **n3 NOTE (executed)** The thin-ring probes build because the disc's centre (k=2) decides the
  join, not the thin annulus. At ri = 0.9995 the annulus offers no witness (width < `L/4096`).
  No row sits at that boundary.
- **n4 NOTE (executed)** Undeclared coincident runs refuse `UndeclaredCoincidence` (42) and
  `CurvedPierceUnsupported` (24), identically on base and head; none is this PR's.

## Style lane (questions exercised: Q1–Q8)

- **Q1** `stands.rs:224-233` vs `stands.rs:311-321`: rung 2 and `across_edges` each spell the
  same half-edge → edge → curve → `certified()` chain, down to the desync string "witnessed edge
  has no curve". That is a second copy minted in this diff. — sure
- **Q1** `chart_region.rs:2207` `candidate_points` (vertex centroid plus neighbour midpoints) is a
  third face-interior candidate builder beside rung 3's triples and chords. The CLEAVE note
  proposes porting `across_edges` into charts as well, which would make a fourth spelling with
  no shared home. — likely
- **Q2** `stands.rs:291-301`: the doc argues the step order ("no shorter step is tried while a
  longer one is left") and the candidate order, and nothing pins either (m1). That is prose
  doing a test's work. — likely
- **Q3** m1 and m2 are this question's answers: the battery rows cannot fail for this change,
  and both-sides and the order cannot go red. — sure
- **Q4** `work/cleave/the-uncut-shell-…:44-47` still says the join refuses `JoinDesync`
  ("neither section loop's regions…"), but the code refuses `SectionLoopUndecided`. The rot
  predates this PR, but the PR appended to the row without fixing it. — likely
- **Q5** The `stands.rs` module doc (lines 1–52) matches the code, rung 3 text included. Nothing
  found. — likely
- **Q6** `ACROSS_HALVINGS = 12` (`stands.rs:292`): no reason is given at the site, it is never
  varied, and no row sits at its refusal boundary (n3). — likely
- **Q7** `stands.rs:330`: the inward side is knowable from the half-edge's sense against the
  carrier's direction, yet both are tried. That doubles the certifier calls, and the surviving
  `−only` mutant suggests the `+` arm is dead on every pose in the tree. — unsure
- **Q8** I read `stands.rs` end to end (512 lines). Its parts: the ladder, the candidate
  builders, `ShellRead` and `witness_insides`. No accretion beyond the Q1 copy. — sure

REVIEW COMPLETE
