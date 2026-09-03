---
id: consider-proptest-for-randomized-sweeps
kind: issue
title: Consider migrating the hand-rolled randomized sweeps to proptest
status: parked
opened: 2026-08-13
github: 466
blocked_on: [tcost]
refs: [452, 462]
---

## From GitHub issue 466

opened 2026-08-13, 0 comments.

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
