# Review of PR #3846, frozen head 00085e001d

Lane `reach-dual3846-r1`. **Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 4. No glimpse: I read only the PR body (`get`) and
check runs/job log of the `test` job; no comments, reviews or other `analysis/reach-dual/*` branch. Wall clock 16:38–17:33 UTC 2026-10-02.

## Claims, by execution (oracle: closed form + analytic point membership, `probes/probe_r1_3846.rs`)
- **C2 order-independence — holds.** Plate and L poses plus radii 0.01, 0.25, 0.5, 1, 1.5, 1.99 (sharp/rounded); mismatched 0.3/0.5, 0.5/0.3,
  0.25/1, 1/0.25, 1.9/0.2; scales ×1e-3, ×1e3; three rigid poses (30° about z, 0.7 rad about (1,2,3), 90° about x, so the stack is on its
  side); and results reused as operands under a third plate. Each ran both orders × ∪, A∖B, B∖A, ∩. Every cell builds, with volume within
  7e-16 relative, tier 3 and 3′ green, ∩ empty, and 120 `point_in_solid` samples per body agreeing with the oracle. Main differential: the
  same cells refuse `CurvedPierceUnsupported` in exactly the PR's table pattern, plus rotated, scaled and reused stacks.
- **C1 soundness — no wrong body found.** The short box beside the fillet (z∈[.25,.75], [.25,1], [.5,1.5], [−.5,.5]; declared `Tangent`) is
  a held pair with no vertex under the touch. It refuses typed in both orders under every op. In-band shifts δ=±1e-10/1e-12 build exact.
  Shifts of ±1e-7 and ±1e-3 give `Join(UnpairedLooseEnds)` in BOTH orders; main splits them by order (`CurvedPierceUnsupported` / Join).
- **C3 the hold only widens** — I did not find a counterexample. Uncovered pairs cannot reach `Interior` (every hold arm is `covered`-gated).
  The undeclared box graze refuses in both orders. Near-equal radii 0.5/0.500001 refuse in both orders, every op, on main and the PR alike.
- **Suites:** `topo`+`sweep` nextest at ε 1e-9: 3964/3964; 1e-6: green apart from my probe's own artifacts; 1e-12: only the known
  `rigid_map_near_eps_plane_nurbs` red. The probe reds were typed `Escalated` samples inside my near-boundary skip, and `find_flush_candidates`
  `PairInBand` at δ/radius ≈ band. That is the finder, not this PR. CI `test` on `00085e00` is red; the log tail I read shows nothing
  failing, and I did not confirm the PR's account of it (`pncad` north-star row).

## Findings
1. **MINOR: `settle_held` has no row that can fail.** DEMONSTRATED by mutants (`probes/mutants_r1_3846.py`). The mutant where settle accepts
   every held pair unread (`reduce.rs:1449`, `held.into_iter().take(0)`) passes **all 3966 `topo`+`sweep` tests**. So do settle ignoring the
   fragment's verdict (`reduce.rs:1467`) and settle reading only the leading fragment (`reduce.rs:1471`), on the PR's rows and my probes. Only
   "hold disabled" (`reduce.rs:1066`) and the line×wall hold (`reduce.rs:2305`) red rows. My short-box probe tells settle-accepts-all apart:
   that mutant BUILDS those poses, with correct volumes. The PR refuses them. Settle's refusal is the soundness half of claim 1 and is
   unpinned. Fix: add the short-box both-order refusal as a row, plus a pose where the touch sits in a non-leading fragment.
2. **MINOR: two hold arms are unreached and the PR body under-states one.** DEMONSTRATED: mutating out the circle-arm hold (`reduce.rs:1885`)
   or the `(Pos,Pos)` arm at `reduce.rs:2188` reds nothing. The body says the second arm is "torus, no fixture reaches it". Its guard is
   `!on_line || Torus`, so it also holds covered ARCS against every curved kind. The circle-arm hold is listed as live with no fixture saying so.
3. **NOTE (inspection, likely):** the covered `(Zero,Pos)`/`(Pos,Zero)` arms (`reduce.rs:1981-1995`) never look inside the fragment.
   That is sound for line×cylinder/sphere: a convex nonnegative residual has an interval zero set. A torus line or an arc can touch twice.
   Settle now routes held torus and arc pairs into exactly these arms, so a second interior touch would be accepted unrecorded. The gap is
   pre-existing (the reverse order reached it on main), but the hold widens its reach. Not built: no torus fixture.
4. **NOTE: README clause** (`crates/topo/README.md:210-212`). `git log --all -S'recorded at its endpoints only'` → only `62af8f98` ([ev]).
   The new sentence does not alter "recorded at its endpoints only". It describes a mechanism that brings interior touches to endpoints.
   It does add an order-independence commitment, scoped wider than the code: it is untrue for the unreached torus and arc arms (finding 2),
   and "recorded" reads oddly when the unsplit case refuses. My call is that it is description, not re-decision (likely).
5. **NOTE: filed items measured honestly.** DEMONSTRATED: the short-armed L at r=1 refuses `CurvedPierceUnsupported{face 4v1, edge 2v1}`
   with operand A or B per order, as filed. The box corner on the ruling (both senses) refuses `CurvedBooleanUnsupported{face 6v1, Plane}`
   in both orders, ∪ and ∩, as filed.
6. **NOTE:** the mismatched-radii item flips to `review`. Its new section correctly records that the "both orders" title was false.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q7; Q8 partially, read the `reduce.rs` header plus the touched arms, not all 4924 lines)
- `reduce.rs:1401` `HeldPair`/`settle_held`/`held` and `boolean/discard.rs:96` `HeldEdge`/`HeldInto`/`DiscardRow::held` are two unrelated
  meanings of "held" in one module tree (Q1, likely). Also look for other "hold" vocabularies in `boolean/` (`finish.rs`).
- `reduce.rs:1471-1491` finds a split's trailing fragment by walking `he_plus.next`, while `requeue` (`reduce.rs:3520`) finds it via
  `emanating` and finish/discard read `Provenance`/lineage rows. That makes three ways to enumerate one split's fragments (Q1, likely).
- `reduce.rs:1053`: `Interior` is excluded from `trace.accepted`, and `settle_held` never writes the trace, so a held pair settled
  `Recorded` is invisible to the idealized/realized accepted-pair channel. The module header (`reduce.rs:14-18`) and the `Recorded`
  variant's doc rely on that channel seeing every accepted pair (Q4, likely).
- `reduce.rs:47` module doc: "Sweep order (D9): direction A→B fully, then B→A". The settle stage after both directions is not mentioned
  (Q5, sure).
- `reduce.rs:1438-1440` "A pair nothing split reads exactly as it was held" is asserted, not enforced. Direction 2 may have split the
  held face's boundary edges, which changes the containment walk's input (Q2, unsure).
- Q3: `a_tangency_in_the_middle…` cannot fail on any settle defect (finding 1). `a_declared_tangent_beside…` is the only row reaching the
  line×wall hold with a distinct geometry (sure).
- Q7: holding then re-reading through the full arm is heavier than splitting at structural tangent points before the sweep, which the
  unit item itself offered. Taste only (unsure).
- The mutant pass found that the *recording* half of settle is redundant on every measured pose: the other direction already records
  the touch vertex. Worth one sentence at `reduce.rs:1424` on what settle adds beyond the refusal (unsure).

## Probes (`probes/`)
`probe_r1_3846.rs` (drop into `crates/sweep/tests/` and add a `mod` line to `all.rs`): `probe_radii_and_mismatch`, `probe_scales`,
`probe_rotated`, `probe_reuse`, `probe_graze_shift`, `probe_short_box_beside_fillet`, `probe_filed_short_l`, `probe_filed_box_corner`.
`mutants_r1_3846.py`: the seven text mutants of `reduce.rs`, run against the full `topo`+`sweep` nextest.
