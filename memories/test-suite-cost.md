---
name: test-suite-cost
description: Standing rules for what a test may cost the CI suite — randomized sweeps must be seed-varying, EFFORT-scaled and gated to the code they test; failure isolation is worth less than per-run cost; assertion-free tests are never gates
metadata:
  type: feedback
---

Rulings from the 2026-08-13 whole-suite test-time audit. The audit
itself is history; these are the rules that outlast it.

**A randomized sweep must be MARKED to run only on changes to the code
it was written to test (Evan, 2026-08-13).** Not on every PR. The
argument "the chance it turns up something new isn't technically zero"
does not justify paying for it on every run — depth is bought
deliberately by cranking its dial, not sprayed across unrelated
changes. A new sweep that is not gated is a defect in the sweep.

Three properties every sweep needs, together:

- **The seed VARIES per run and is logged unconditionally.** A
  hardcoded seed makes a sweep a replay corpus: it explores identical
  points forever and after its first green run can only fail because
  the code under test changed. That is a fine thing to be, but it must
  not be described or budgeted as fuzzing. Log the seed always (not
  only on failure) and repeat it in assertion messages, or a red run is
  unreproducible.
- **Counts are multiples of a shared EFFORT dial**, shipped at the
  level a gated run should cost, so depth is one env var away.
- **Gated** as above.

**Failure isolation is worth less than per-run cost (Evan,
2026-08-13).** When several tests rebuild the same expensive fixture,
merge them — nextest is process-per-test, so a `OnceLock` shares
nothing across tests and each pays in full. The dev cost of looking
closer at a failing merged test is negligible against something that
slows every run. Compensate by LABELLING each assertion so the failing
property is unambiguous from the message alone.

**A test that asserts nothing is never a gate.** It cannot fail, so it
cannot gate; it is evidence for a reviewer at the time. See
[[review-and-dependency-policy]] — reviewer suites are a seam to mine,
and this is the class to drop first.

**Silent skips are the escape-hatch shape.** A bare `return` at some ε
reports green having asserted nothing. Use the tree's NAMED loud-skip
idiom (`interval_lane_skipped_no_certified_coverage_here`) so the
absence is visible in the battery log.

**Measured facts that will mislead you if you assume otherwise:**

- The change filter's tier `all` fires on **75% of building merges**
  (demos/, .github/, scripts/ dominate), so gating anything on the
  filter's existing closure output barely reduces how often it runs —
  84–94% of building merges. A frequency gate has to key on *source*
  changes, ignoring demos/CI/scripts.
- The crates holding the sweeps are crystallized: own-source change
  rates of 3–16% per building merge.
- Cost concentrates savagely. At the audit, 20 tests were 55% of all
  test time and 2,603 tests were 1.7% of it. Profile before cutting;
  the long tail is free.
- Per-test CI timings are NOT comparable across legs without
  normalising — one leg in a sampled run was ~1.5x faster than its
  siblings, which manufactures apparent ε-sensitivity.
