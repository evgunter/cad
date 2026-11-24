---
id: the-channel-contradiction-check-walks-the-tiers-ladder-on-every-definite-decision
kind: issue
title: The check that the two channels do not contradict walks the tier's first attempt on every definite decision, and on a curved body's ray decisions that is 75 to 150 ms each
status: closed
opened: 2026-11-15
priority: P1
cost: H
branch: boxfinish/decide-path-check
pr: 7394
closed: 2026-11-24
---


Filed by BOXFINISH (2026-11-15) from its M5 measurement
(`boxfinish/probe-switch-off` at `2f37154e`, `BOXFINISH_TRACE=3`; the
box-scalar switch's box arm off, so every sweep door finishes its leaf
at `Sym<Interval>`). It holds BOXFINISH's cost row
(`work/boxfinish/tier-3-at-every-box-replay-leaf-multiplies-the-decisions.md`),
which the switch's deletion waits on; hence its band.

**What is measured.** A finish that refuses nothing costs 57 ms on a
six-face slab and 842 ms on a four-face body with a cylinder wall
(2 to 4.6 ms at `f64`), 2.76 s on a three-face revolved circle. Almost
all of it is inside `sign_within`, not in building margins (799 of
842 ms; 2711 of 2763), and in a handful of decisions:

- the cylinder-walled body: `side_at_infinity_winding`, 6 decisions,
  453 ms (75 ms each) and `side_at_infinity_denom`, 2 decisions, 239 to
  305 ms; together 82% of the finish. On the slab the same two are 6
  decisions in 7.4 ms and 6 in 6.1 ms;
- the revolved circle: `curve_pair_span_end_chord` 100 decisions
  559 ms, `bool_torus_region_at` 12 decisions 354 ms,
  `azimuth_near_cut_side` 60 decisions 284 ms,
  `side_at_infinity_winding` 2 decisions 276 ms, and smaller.

**Each of those decisions is decided numerically, every time**
(per drive: `side_at_infinity_winding` 24 of 24, `_denom` 12 of 12,
`bool_wall_turn_true` 48 of 48, `curve_carrier_clear` 60 of 60). The
symbolic work on them buys no verdict. Stack samples of the slow row
(four usable) all sit in `sym::contradicts` → `ladder` → `rungs` →
`walk` → `form_in`, two of them under `side_reader::aimed_ray_inner`.

**Where that is in the code** (read, `crates/geom-core/src/sym.rs`,
`Decide::sign_within` for `Sym`): after the numeric channel answers a
definite sign, the exact-witness arm runs
`debug_assert!(!contradicts(self.node))` and the inexact arm
`if contradicts(self.node) { count_theorem_disputed() }`.
`contradicts` "RUNS THE WALKS (`discharge`), which is the cost each arm
pays", and the workspace ships `debug-assertions = true` in release, so
both arms walk in every profile. The walks freeze on the ring's guards,
not the term budget: per slab finish 96 nodes in
`side_at_infinity_winding` (80 `Mul`, 16 `Add`, every one
`Refusal::Ring`); on the cylinder-walled row over two drives, 1084
nodes in `side_at_infinity_winding` and 228 in `_denom`.

**Inferred, not measured:** that the curved cases cost more per climb
because the margin's DAG is larger (a ray against a cylinder or a torus
carries roots and turns); form sizes and rungs per decision were not
captured; whether a repeated decision re-walks its form or is served
from the walk's memo.

**The question.** What the check that the two channels do not
contradict is for, and what it may cost: whether it runs on every
definite decision, once per form, under a budget of its own, or in a
run that asks for it; and what a run that skips it can no longer say.
It is a soundness self-check and a receipt column
(`SymCounts::checks_expanded` / `checks_frozen`, the disputed and
contradicted counts), so the answer is a design one.

**What would say it is closed** (BOXFINISH's side): with the switch's
box arm off, a finish of a cylinder-walled or revolved body that
refuses nothing costs within a small multiple of the slab's per
decision, and the seven fast rows M4 lists run inside the 1.0 s budget.
(BOXFINISH orchestrator)

## Weighed (2026-11-22, fork-log row 301): the check leaves the decision path

Two designers weighed this, BOXFINISH running the fork because SYM
had no orchestrator and its cost row waits here. They agree after two
rounds (`docs/ERROR-DESIGN.md` E12, "The two channels are checked
where an answer is returned", is the ratified statement).

**Corrections to this row's premise, both designers'.**

- The check does not walk the whole ladder: it is `Path::Contradiction`,
  the first attempt (plain form, rule form, top reduction), with no
  retry and no door. A second walk runs beside it that this row did not
  name: `door_zero` walks the registered form (its own memo) of every
  definite margin whenever the run holds a registration, in every
  profile, and before `profile::set_origin(Origin::Assertion)`, so the
  cost profile's past tables charge it to the decision.
- A repeated decision does not re-walk: forms are memoized per node
  per session, freezes included. The cost is first-time forms of
  margins nothing else asked about.
- The check is on the wrong population. It runs only where the numeric
  channel was definite, which is where the tier's answer is thrown
  away. Where the tier's answer is used, nothing checks it: a retry's
  zero, and an in-band enclosure that excludes 0 stamped proven.
- It writes the receipt (`checks_expanded`, `checks_frozen`, and its
  plain-walk freezes in `frozen` outside a drive) from inside a
  `debug_assert!`, against `[failure]` row 5.
- No record was found of the exact-witness assertion ever catching a
  defect; the recorded firings were at `Sym<f64>`, where the theorem
  was right and the rounded number wrong. Tracker and logs were
  searched, PR bodies and CI logs were not.

**The final state.**

1. A definite numeric sign returns at once. `contradicts` and
   `door_zero` leave `decide_sign`'s definite arm.
2. Where the tier answers `Zero` at `Sym<Interval>` (any rung, a
   retry, the registry door), a `debug_assert!` that the enclosure
   contains 0 (`[failure.taxonomy]` row 5; Ev on PR 7337: a check for
   bugs is a `debug_assert`, never a plain `assert!`). It leaves out
   `Decide::free_identity`
   margins, recognised on the failing path only: when the enclosure
   excludes 0, the node's DAG is walked for a `SymOp::Free` before
   panicking. Nothing is stored. Its message names the rung and the
   enclosure; the soundness argument is stated at the site (the number
   is the interval evaluation of the DAG the form was taken from; every
   leaf, `Param`, `Opaque`, `Hull`, `Lit`, stands for a point of its own
   interval; the decision read takes an arm only on a certified
   one-sided enclosure over the parameter brackets).
   It is not a test of the algebra: an enclosure clear of the band was
   already definite, so the assertion can fire only on one that
   excludes 0 inside or across the band (a window nothing checks
   today). A wrong algebraic zero over an enclosure that straddles 0
   is seen by no check in a single run; the instrument looks for those.
3. The old cross-check is a test instrument: all three events
   (algebra against a certified enclosure; algebra against a rounded
   number; a registration against a definite sign) and the retries,
   with `Path::Decision`. It is feature-compiled AND installed at run
   time, because a feature enabled through a dev-dependency is unified
   across the workspace's test build. The re-valuation recorder
   (`sym/revalue.rs`, `sym-revalue-testing`) already records every
   `Sym` decision's node and sign through a thread-local; the
   instrument can read that record inside the session.
4. It runs in small rows on every PR that plant a contradiction and
   see it caught, over the document corpus on any PR that touches the
   tier, and nightly.
5. `theorems_disputed`, `checks_expanded`, `checks_frozen` and
   `registrations_contradicted` leave `SymCounts`.

**For the unit.**

- Check that nothing outside `sym.rs` writes `Sym::value`, and make
  the field private behind a reader, so "the number is its
  expression's" holds by construction.
- `predicate::tests::a_proven_zero_is_offered_no_tolerance` stamps an
  excluding enclosure proven; it becomes a straddling one.
- A row at `geom_brep::edge_nurbs::limb2_enclosure_as` with
  `floor > 0` and a discharging identity pins the exception.
- Run the whole suite and the nightly drives before trusting the
  assertion; a firing on a registered zero is a registrant finding.
- Readers to move or delete: `editor-core` `drive.rs`'s two report
  lines; the two thread-count digests' coverage guards; `m10_9_pins_interval`,
  `sym11_exact_channel_rows`, `revalue_corpus`; `geom-core`'s
  `sym11_witness_kind{,_interval}_rows`; `sweep`'s
  `sym11_far_placement_rows`; `sym.rs` "Cost" and the `SymCounts`
  docs.
- Whether `frozen` on pinned receipts moves is measured, not assumed
  (only plain-walk freezes count there, and inside a drive the column
  is the leaf's NEED).
- Do not key the change on `k_stats`' `Role::Check`: the role is the
  caller's use of the outcome, and the slab's 95% is on choices.

**Closes or re-states, when built:**
`a-retrys-zero-has-no-cross-check-against-the-numeric-channel`,
`an-in-band-enclosure-excluding-zero-is-stamped-proven-zero-unchecked`,
`a-dispute-names-no-predicate-on-the-receipt`,
`far-placement-zero-dispute-cells-can-be-satisfied-by-freezes`;
NURBS's `disp-zero-can-be-satisfied-by-freezes` and
`the-thread-count-digest-cannot-see-dispute-coverage`. TIER's
`symbolic-tier-costs-95-percent-of-the-m10-3-drive` is re-measured
after it.

**Still worth measuring whatever is built:** the time to the first
`Refusal::Ring` freeze on the ray margins, which the decision path
pays, with the retry ladder, the day such a margin is indeterminate;
and how many of the revolved circle's `curve_pair_span_end_chord`
decisions are numeric.

## Closed

PR 7394 (2026-11-24), built by BOXFINISH as approved on `[ev]` PR 7337.
A decision the numeric channel has settled asks the tier nothing; a
proven zero, from any rung, a retry, the door or a deferred discharge,
is checked against the enclosure the decision read
(`discharge_checked`, a `debug_assert!`); the two-channel cross-check
is a test instrument behind `sym-revalue-testing`
(`geom_core::sym::contradiction`), asked last.

Not closed by this, each with its own file:

- The instrument reaches the slab's drives leaf by leaf and no other
  drive, and no `Sym<f64>` lane:
  `the-contradiction-instrument-asks-no-corpus-drive-and-no-point-lane`.
- Its zero can still be satisfied by a frozen walk:
  `far-placement-zero-dispute-cells-can-be-satisfied-by-freezes`.
- An opaque minted before a session shares a name with the session's
  first: `an-opaque-minted-before-a-session-shares-a-name-with-the-sessions-first`
  (PR 7470's review took it up).
- Six debug-only deciding sites outside the tier: CHARTTAIL's row, as
  re-titled by the lane.
