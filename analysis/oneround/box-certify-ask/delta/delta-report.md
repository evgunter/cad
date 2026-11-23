# Delta read: PR 7272, `oneround/box-certify-ask`, after the fix pass

Reviewed head: **`1ff43e30`** (previous review: `34c966f6`; base
`oneround/box-region` at `2e9fa0b6`).

## Verdict: NOT-MERGEABLE-AS-IS, for one mechanical reason; the code is at APPROVE-WITH-FIXES

Every finding of the first review is closed in substance, my seven mutants
are all dead, and the fast set is green. But the frozen head **conflicts
with main and has no hosted CI run** (D1). Once main is merged the re-pins
have to be taken again, so the head that merges is not this one.

## What I executed and what I only read

Executed, at `1ff43e30`:

- **Fast set, workspace less `viewer` and `pncad-py`, ε = 1e-9**: 16 586
  run, 16 586 passed (binary carrying an inert env-gated logging patch,
  `instrumentation-delta.patch`). Judged against nextest's `ci` default
  filter.
- **A 211-row slice with the slow set** (every re-pinned ledger, count,
  digest and golden, `revalue_corpus`, `placed_balls`, the F2 fixtures, the
  rosters): ask on at 1e-9, 1e-6, 1e-12; ask off at 1e-9, same binary.
- **15 mutants** (my 7 re-planted, 8 new) against geom-brep's lib fast set
  (1 541 tests); the survivors and the topo-side ones against topo's whole
  fast set, lib and integration (4 042 tests).
- **A build of the base** at `2e9fa0b6`, `revalue_corpus` and the rosters.
- A probe of ten span-and-cover mixes through `EdgeCurve::certify`
  (`delta_probes.rs`).
- `git merge-tree` of the head with `origin/main`; the hosted run's job log.

Only read: the fix pass's new code in `session_box.rs`, `certify.rs`,
`extent.rs`, `finish.rs`; the PR body. Not done: the whole slow set, the
demos, the cost figures, the 1e-12-only and fuzz-seed reds.

## MAJOR

### D1. The head conflicts with main and no CI ran on it. DEMONSTRATED, `sure`

Run 45187025826 is not a CI run. It is the `merge ref` workflow, concluded
**failure**: "PR #7272 conflicts with its base 'main' (mergeable_state:
dirty) … CI created NO run for this push". No CI run exists for `1ff43e30`;
the newest is 44897731642 on `34c966f6`. Locally, merging `origin/main`
(`d23a330e`) into the head conflicts in `.config/nextest.toml`, in
`asm2a_instantiate`, `m10_6_certifying_keys.txt`, `m10_8`, `m10_9`,
`m10_bulge`, `m10_sym_profile`, `revalue_corpus`, `sym11_exact_channel`,
`sym_9_retry`, geom-brep's `recourse_roster` and
`a_sweep_keeps_the_frame…`: that is, in nearly every file this unit
re-pins. (It merges clean with `1329e397`, the main it last took.)

So: the PR body's "Hosted run on `1ff43e30`: 45187025826, queued" is
stale and now false; "hosted CI … covers every golden this PR re-pins" has
never been exercised on a head carrying these pins; and every re-pin in
claim 5 must be re-measured after the merge. This is the first review's M1
in a new form: the goldens are again vouched for by local runs only, mine
included.

## MINOR

### d1. Three mutants survive. DEMONSTRATED, `sure`

All seven of the first review's mutants are now red, on the rows the PR
body names. Of eight new ones, five are red. Three survive geom-brep's lib
fast set and topo's whole fast set:

| mutant | what it shows is unpinned |
|---|---|
| the span read before the box (`decide_turn` first, returning any `IntervalNotForward`) | claim 2's order. No row has an edge the box shows inside or past it with a span that is not forward. My probe confirms the code is right (coincident points 2 km out, and a far line over a zero span, both read `OutsideSessionBox`); nothing holds it there. |
| a point that is not finite hides the rest (the `is_finite_length` filter removed from `edge_points_reach`) | F6's sentence "a NaN end beside an end 5 km out is past the box by the far end". True in my probe, pinned by no row. |
| a boxed `EdgeCertification` is family `Certificate` again (the new arm in `finish.rs` removed) | F3's partition fix. 4 042 topo tests pass without it. |

Killed: ask dropped at `certify_carrying` (5 rows), inside read without the
carrier's points (2), cover not hulled with the ends (3), inside read
without the ends (1), a margin in the band refusing (1), the limit admitted
(1), a point reported as the limit (2), the span arm dropped (3), any span
refusal replacing the box's (1), an uncovered edge left to the checks (2),
an ellipse read by samples (1), every uncovered cause a limit (2 in
geom-brep, 1 in topo).

## NOTE

- **Claim 1.** F1: the red golden is green at this head at three ε. F2:
  `exact_enclosure_rows` and the zone fuzz carry per-ε floors and pass;
  `placed_balls` below. F3, F5, F6, F7: as the body says, by reading and by
  the mutants above. F4: the four new rows kill what they were written for.
  `a_splines_cover_alone_…_is_the_limit_at_every_door` asks three of the
  five doors (`.take(3)`); the name says every. `sure`.
- **Claim 2.** Not falsified. The order is as stated (the span is read only
  in the `Uncovered` arm). Two things the body's wording does not cover,
  both reachable only by a hand-built spec: (a) "decided to run no length"
  is in code "any `IntervalNotForward`", which includes an in-band span that
  is not zero; a NaN-direction line over a 5e-10 m span is refused with
  "tighten the tolerance below 5e-11 m", and tightening it only uncovers the
  defect; (b) a NaN end on a good line over a forward span ends "this is a
  kernel defect; report it" at a build door, where the point came from the
  caller (it blamed the kernel before the unit too). `likely`, small.
- **Claim 3.** In what I ran no test is red with the ask on and green with
  it off: the slice's 11 reds with the ask on are a subset of its 42 with it
  off (the other 31 are the re-pinned rows, as they must be). The 11 are the
  10 `revalue_corpus` rows and topo's `recourse_roster`; **the same 11 fail
  on my build of the base** at `2e9fa0b6`, so they are inherited, confirmed
  rather than taken on trust. Same 11 at 1e-6 and 1e-12. The 20 047-test
  differential on `83aaad76` is unverified by me; between it and the head,
  the fast set at 1e-9 is what I can vouch for. Not checked: the three
  1e-12 base reds and the `ellipse_torus` seed (outside my slice).
- **Claim 4.** Fast set at 1e-9: 119 `Point` in 32 tests, 32 `Uncovered` in
  14, 3 `Unshown` in 1 (the row built for it). Every `Point` is a test body
  past the box. The `Uncovered` are poison rows and null chords (a line over
  `(0, 0)` between coincident points) inside kill-operator tests on valid
  fixtures, which end as the span's refusal as before. No helper, scaffold
  or cutter. The demos and the suite-wide counts are unverified by me.
- **Claim 5.** Confirmed on the rows I could read with the ask off:
  `two_hole_plate` 1051, `r1_annulus` 550, `r2_link` 945 against main's
  1047, 548, 943 (main's move +4, +2, +2) and pinned 1099, 574, 981 (the
  box's +48, +24, +36, the same as at `34c966f6`); `asm2a` 1036 and `m10_8`
  504 with it off equal main's pins. No symbolic count moves. One move is
  not a count: `r2_link`'s FROZEN reads 2600 with the ask off and 2604 with
  it on. It is accounted in the row, and it is this unit's. All of it is
  to be re-taken after D1.
- **Claim 6.** Holds. The small-ball row is forced to read every radius ε
  resolves: at 1e-9 all five to `D/r = 1.7e9` are admitted and compared, at
  1e-6 only `r = 1e-2` is owed (`1.7e5`; three are read). The unit-ball row
  compares to 998 m. Its far legs never ran before, so nothing it caught is
  lost; what it can catch at 1e-6 is thin. `likely`.
- **Claim 7.** Not re-measured. My per-ask figures from the first review
  (1.7 to 186 µs by kind) are consistent with a 5 µs mean.

## Style

Exercised: Q1, Q2, Q3, Q4, Q5; Q8 on `geom-brep/src/session_box.rs`.

- **S1.** `extent.rs`: `cover_is_exact` is documented as "the one home of
  that split, which `edge_reach_bounds` and `edge_coordinate_reach_below`
  both read", and the fix pass widens it at one reader only (`let exact =
  cover_is_exact(carrier) || carrier.conic().is_some()`). The home now says
  an ellipse is not exact and one of its two readers says it is. `sure`.
- **S2.** `UncoveredCause::Carrier` reports the *first* unknown's kind and
  cause and a separate `unbounded_only` over all of them; a mixed refusal
  shows one cause and stands as another. `unsure`.
- **S3.** The PR body is now two measurements interleaved ("the tables
  below the figures are the first measurement's"), with counts from three
  different heads. A reader cannot tell which number is true of the head
  without the commit log. `sure`.
- **S4.** The first review's S9 (the sweep helper's interval) was argued,
  not changed. The argument is fair; recorded as open by choice. `sure`.

## Ratings

- Idiom and structure: 4 (one home each way now; the `cover_is_exact` split).
- Test quality: 4 (every earlier survivor has a row; three new gaps).
- Doc and comment honesty: 3 (the body's verification section describes a
  hosted run that does not exist).

## Files

`delta/mutants/*.patch` with `gb`, `topo`, `topofull` results;
`delta/delta_probes.rs` and `.patch`; `delta/instrumentation-delta.patch`;
`delta/box-refusals-fast-1e-9.log`.
