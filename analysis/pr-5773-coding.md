# PR 5773 dual review — blind coding inputs

Byte 99 (drawn 2026-10-26T13:10:58Z), odd, so A = R2 and B = R1. Do not read this mapping before coding: it lives here only so the orchestrator can un-blind at merge.

Frozen head: ee528ff3057f52caf95c70f0a3c73b2b1da07195. Arm: concurrent, letter H. Both reviewers were dispatched at 12:24Z with one brief. Neither was interrupted, and both report that lane isolation held.

## Review A

NOT-MERGEABLE-AS-IS. 1 MAJOR, 4 MINOR, 4 NOTE.

- **MAJOR M1. `Link::lowest_on` refuses typed in f64 for edges in general position.**
  - Mechanism: the pairwise roots are double roots. A ray's line form against a plane form whose plane contains the ray differ by a perfect square. `sqrt` turns roundoff into a ~1e-8 shift, which lands the candidate inside another plane's band sliver, so `offset` refuses InBand.
  - Executed: 573/1200 random segments refuse. The crossing ridges turned 1/7/45/61° refuse 5/21. The PR body's "f64 reads every row" is wrong, and so is the P3 row's scope (it says Interval-only).
  - Completeness holds.
  - Sure.
- **MINOR m1. `offset` reads a false 0 beside a short ray** (`Link::distance` / `touches`). `wedge_side` draws q in to |r−c|, so "beyond the ray" is in band out to band·|q−c|/|r−c|. Executed: a corner with a 1e-3 −x edge reads 0 at d = 5e-7. That breaks "nearer by at most the band" and "zero exactly at contact". Sure.
- **MINOR m2. The Dual tangent is NaN at contact.** Executed: vertex on the ridge line, and a segment across the ridge. Stackup reads `TangentDegraded`, where the old fold was finite. Sure / likely.
- **MINOR m3. The doors test misses `validate::definitely_positive(...)` and `enclosure_probe().is_some_and(hi < 0.0)`**; both mutants stay green. The `< T::zero()` needles are dead. Executed. Sure.
- **MINOR m4. The P3 row and the PR body misdescribe the three Interval refusals.** Two of them are at h = ±0.01, not at contact. The interval test stays green if every reading refuses. Executed. Sure.
- **NOTE N1.** Stale prose: `cone_fuzz.rs:3`, `gap_reads…:462` (`gap_wedge_crossing`), `K-REPORT:938`.
- **NOTE N2.** The narrowing note on `gap-reads-a-cells-carrier-beyond-the-cell` overstates; sectors run on without end. Likely.
- **NOTE N3.** The body builders never run `Link::closes`. `in_face` has no caller and no test. No row checks `at_point(body)` against the census snapshot. Sure.
- **NOTE N4.** `lowest_on` evaluates about 4·C(n,2) candidates, each with a full `offset`, and one refusal anywhere refuses the whole segment. Likely.
- **Claims:**
  1. holds: the census text is byte-identical, and failures are identical on head and base;
  2. holds for the named needles;
  3. partly holds (m1);
  4. holds in exact arithmetic, fails through M1;
  5. holds;
  6. scoped wrongly (M1, m4);
  7. deviations (b) and (c) are right;
  8. holds;
  9. mostly holds.
- **Style:**
  - Q1: `o.max(-o)` is a second spelling of abs;
  - Q1: two builder families that share only `of_face`;
  - Q2: `on_edge`'s precondition is not enforced;
  - Q2: `distance`'s doc is false;
  - Q3: the interval and doors needles cannot fail;
  - Q5: `lowest_on`'s and the module's docs over-promise;
  - Q6: the Dual regression is unscheduled;
  - Q7: don't refuse the whole segment on one candidate; take a double root as −b/2a;
  - Q8: read end to end.

## Review B

NOT-MERGEABLE-AS-IS. 2 MAJOR, 4 MINOR, 6 NOTE.

- **MAJOR M1. `Link::lowest_on` refuses most generic edge–edge Gaps in f64 at ε = 1e-9, and base reads them.**
  - Mechanism: the tie families apex-form = ray-line-form, (u·w)², and plane-form = ray-line-form for a ray in that plane, (n·w)², are double roots. `sqrt(max(disc, 0))` gives an error of ~1e-8, which lands in the sliver; `reads` refuses, and one refused candidate sinks the segment. These ties are C1 transitions, never extrema.
  - Executed: random segments refuse at block 831/1500, hole 856, saddle 844, ridge 530, trough 525 (0–3 at 1e-6, 0 at 1e-12). 1638/2000 refuse at exact contact on a ridge. Real bodies turned about 24 axes: head reads 11/48, base 48/48.
  - Fixtures are axis-aligned. The P3 row misdiagnoses it.
  - Sure.
- **MAJOR M2. The Interval offset is not an enclosure over a box, where the base fold was.**
  - `Link::distance` counts the plane distance when an in-band `wedge_side` is read as "not beyond".
  - Executed at link level: a block corner with q = (s, −0.5, 0). s ∈ [0, 0.1] reads [0, 0]; [−0.1, 0.1] reads [−0.1, 0], the wrong sign. `lowest_on` gives [−0.3, 0] against f64 0.1. The base fold gives [0, 0.1].
  - Sure; reaching it through the box driver is likely.
- **MINOR m1. `link_reads` maps InBand/PieceInBand to `MeasureUnsupported`** where the deleted predicate escalated (`MeasureEscalated`). `drive.rs` then bisects to Budget instead of reading `indeterminate()`. Likely.
- **MINOR m2. The doors row cannot see raw comparisons that compile.** Executed: the `enclosure_probe().is_some_and(hi < 0.0)` mutant stays green, and a `copysign` mutant goes red. The `< T::zero()` needles cannot occur. Sure.
- **MINOR m3. `interval_lane_encloses_the_f64_gap` cannot fail as refusals grow**, and cannot catch M2. Sure.
- **MINOR m4. `docs/INTENT-STAGE5-SPEC.md:132` still lists "a saddle vertex"** among the cells whose Gap refuses. Sure.
- **NOTEs:**
  - claim 1 holds (diffed and byte-identical; inherited reds named);
  - claim 3 holds in f64 (20k points × 5 shapes, 0 mismatches);
  - claim 4 is complete apart from M1;
  - claim 5 holds;
  - deviations (a), (b) and (c) are right;
  - claims 8 and 9 hold;
  - stale citations: `cone_fuzz.rs:3`, `K-REPORT:938`;
  - unexecuted: `wedge_side`'s `within()` undercut may scale as band·|q−p|/|r−p|. Unsure.
- **Style:**
  - Q1: `of_body_face` re-walks what the census snapshot does, with two incidence spellings;
  - Q1: `distance` and `lowest_on` spell the piece set twice;
  - Q7/Q3: `CellOf::Bare` is built only by tests;
  - Q7: two `unreachable!` arms;
  - Q3: every fixture is axis-aligned;
  - Q5: `lowest_on`'s doc ("only reads higher");
  - Q6: the P3 row has the wrong cause;
  - class: perfect-square ties wherever a ray lies in a plane.

## Correspondence pre-note (orchestrator, before coding)

- **Bilateral:**
  - M1, with no severity divergence;
  - the doors-test escapes and dead needles;
  - the interval row being monotone in the wrong direction;
  - stale `cone_fuzz` / `K-REPORT` prose;
  - deviations right;
  - two builder families (style);
  - the docs over-promising (Q5).
- **Candidate bilateral at divergent severity:** B's M2 (MAJOR, executed) and A's m1 (MINOR, executed). Both stem from `Link::distance` counting the plane when `wedge_side` reads in band. B's unexecuted NOTE on `within()` scaling is the same site. If the coder rules them one defect, the finding is bilateral and not tallied. If it rules them two, B's M2 is a tally candidate: unilateral, MAJOR, code, executed.
- **Unilateral A:** the Dual NaN tangent (MINOR); `closes`, `in_face` and the at_point–snapshot row (NOTE); cost (NOTE); `o.max(-o)` (style).
- **Unilateral B:** Escalated vs Unsupported (MINOR); the stale stage 5 spec line (MINOR); `CellOf::Bare` and `unreachable!` (style); the piece set spelled twice (style).
