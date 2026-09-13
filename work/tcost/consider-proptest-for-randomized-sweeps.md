---
id: consider-proptest-for-randomized-sweeps
kind: issue
title: Consider migrating the hand-rolled randomized sweeps to proptest
status: closed
opened: 2026-08-13
github: 466
blocked_on: [tcost]
refs: [452, 462]
closed: 2026-09-11
---

## From GitHub issue 466

Opened 2026-08-13; 0 comments.

The kernel has a family of hand-rolled randomized sweeps — `ring_interval_fuzz`, `nurbs_cert::r1_random_rational_soundness_sweep`, `lt_r1_probes::r1_randomized_soundness`, `r2_lt_probes`, `review_m6_surgery_rider`, `review_m5_pr4_adversarial`, `probe_fuzz`, `spline_hull`, `review_m5_pr7b_tensor`, `review_m1_pr4::seqgen_*` and others — each carrying its own copy of an xorshift64\* `Rng` struct and its own hardcoded seed.

`proptest` is **already a workspace dev-dependency of every crate**. It ships, out of the box, three things this family currently hand-rolls or lacks:

| what | hand-rolled today | proptest |
|---|---|---|
| iteration dial | a literal count per test | `PROPTEST_CASES` env var |
| failing-case reproduction | nothing — seeds were hardcoded, so a failure was reproducible only because it was never new | `.proptest-regressions` file, committed, replayed on every subsequent run |
| minimisation | none — a failure reports the raw adversarial input | shrinking to a minimal counterexample |

The third is the interesting one for this codebase. When one of these sweeps fails it currently hands you a random NURBS net with degree 5, 13 interior knots and adversarial weights, and you work out by hand which feature of it matters. Shrinking is exactly the tool for that.

## The counter-argument, stated fairly

These are **bulk-oracle sweeps**, not per-case properties. A typical one runs N random configurations and, for each, compares a certified bound against a densely sampled truth — the cost is dominated by the oracle, not by case generation, and there is no cheap "is this case interesting" predicate for proptest to shrink against without re-running the oracle at every shrink step. Some (`ring_interval_fuzz`) are exhaustive-ish products over edge-value tables rather than random draws at all, and proptest is the wrong shape for those.

So this is a **consider**, not a plan. A plausible outcome is migrating the genuinely per-case properties and leaving the bulk sweeps as they are.

## Context

Found during the 2026-08-13 test-time audit. Related work already landed:

- #452 — the interval run legs execute only the tests that feature adds.
- #462 — duplicated work removed from the heaviest suites.

A follow-up PR gives this family a shared harness: one `Rng`, a per-run **random** seed (logged unconditionally and repeated in failure messages, overridable to replay), an `EFFORT` dial with every count expressed as a multiple of it, and CI gating so a sweep runs only when the code it was written to test is in the change closure. That PR deliberately does **not** migrate to proptest — it makes the family uniform first, which is also what would make a later migration mechanical rather than archaeological.

If we do migrate, the `EFFORT` dial becomes `PROPTEST_CASES` and the seed door becomes `.proptest-regressions`, so the harness is designed to be replaceable by them rather than to compete with them.

## Home

S-TCOST: `crates/*/tests/*` and `crates/test-utils/*` are the program's territory, and S-QA — which parked this issue on its own text scheduling the uniform-harness PR first — is closed. Parked on the program because that unnumbered harness PR is now this program's to schedule.

## Closed (Ev's ruling, 2026-09-11)

**Closed as not worth doing.** Ev, in chat: *"the fuzzers have found
things only rarely and it seems like understanding the problem found by
the fuzzer hasn't been too burdensome."*

Both halves of this issue's case fall to that, and it is the case's own
structure: the whole argument for migrating was **shrinking** — the
table above prices `PROPTEST_CASES` and `.proptest-regressions` as
conveniences and says outright that *"the third is the interesting one
for this codebase"*, because a failing sweep hands you a random NURBS net
with degree 5, 13 interior knots and adversarial weights and you work out
by hand which feature matters. Shrinking is a tool for making that
cheaper. If it has not been expensive, the tool has nothing to buy.

The frequency half is the evidence for the same conclusion and was
already on this program's record: the red-history census (2026-09-02,
`work/tcost/log.md`) classified 397 (test, run) reds over 162 distinct
tests, and of the 145 tests that ever appeared in a `Slowest 20` table
only 21 have ever gone red at all. A migration priced against a rare
event whose diagnosis is already cheap does not clear its own cost.

**And the counter-argument this issue states fairly was never answered.**
These are bulk-oracle sweeps: the cost is in the oracle, not in case
generation, and there is no cheap "is this case interesting" predicate to
shrink against without re-running the oracle at every shrink step. Some
(`ring_interval_fuzz`) are exhaustive-ish products over edge-value tables
and are not random draws at all, where proptest is the wrong shape
outright. Nothing since 2026-08-13 has moved that.

**What does NOT close with it**, so the ruling is not read wider than it
is:

- The uniform harness this issue was parked on **exists** —
  `test_utils::fuzz` supplies one RNG, a per-run varying seed logged
  unconditionally, and the EFFORT dial. The `blocked_on` trigger had
  effectively fired; nothing was waiting on the migration.
- `memories/test-suite-cost.md`'s fuzzing rules are untouched and bind as
  before: a fuzzer must not fix its seed, counts are multiples of the
  EFFORT dial, and the seed is logged unconditionally so a red run is
  reproducible. This ruling is about a LIBRARY, not about what a fuzzer
  owes.
- `r1-probe-seeds-are-not-on-the-fuzz-dial` stays open on its own terms:
  two rows draw from the clock under a private `R1_SEED` that
  `CAD_FUZZ_SEED` cannot pin, so a red there is unreproducible by ANY
  route. That is the reproducibility floor this ruling assumes is
  already met, and it is the one place it is not.
