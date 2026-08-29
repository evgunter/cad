---
name: test-suite-cost
description: Standing rules for what a test may cost the suite — all fuzzing varies its seed, scales on an EFFORT dial and is gated to the code it tests; assertion-free tests never gate
metadata:
  type: feedback
---

# Fuzzing

These bind **ALL fuzzing in this repo** — every randomized sweep,
property sweep, adversarial sweep and fuzz row, wherever it lives.

**A fuzzer MUST NOT FIX ITS SEED (Evan, 2026-08-13).** No hardcoded
literal, no `const SEED`, no seed derived from a loop counter. A fixed
seed does not make a weak fuzzer, it makes something that is not a
fuzzer at all: it explores the same points on every run for the rest of
the project's life, so after its first green run it can only fail
because the code under test changed.

**FIRST, ask which SHAPE the test is** — "a fuzzer must not fix its
seed" read as "everything needs a random seed" is how a coverage test
becomes flaky. Three shapes, and only one wants a varying seed:

- **Counterexample search** (*for all sampled x, P(x)*) — vary the
  seed. Monotone in the safe direction: cutting the count loses
  detection power, never correctness.
- **A witness you can WRITE DOWN** (*at least K of class C*, C
  concisely constructible) — do not search at all. Build it as a static
  fixture and assert it every run.
- **A witness you CANNOT write down** (C not concisely specifiable —
  "a walk that reaches every op kind") — FIX THE SEED. It is a fixture
  identifier, not a sampling strategy, and it cannot flake.

**The trap is mixing the first and third in one test.** A property test
with an anti-vacuity floor bolted on is both at once, and its sample
count then feeds two obligations of which only one is safe to cut. Make
the floor's witness static, or split the test.

**When a fixed seed IS right** — each case must say in-file which one
it is; an unexplained literal seed is the failure mode:

- **A coverage/witness claim of the third shape.** Condition (Evan): K
  large enough, or the simultaneous conditions numerous enough, that
  the row is VERY UNLIKELY TO PASS BY ACCIDENT on a lucky seed. K = 1
  against a 1-in-1000 class is the shape to avoid.
- **A pinned counterexample.** Prefer writing the input OUT as an
  explicit fixture; a seed is acceptable only as compression when the
  input is genuinely too big to write, and then the doc says "this seed
  reproduces #N".
- **Cross-PROCESS or cross-BUILD differential comparison**, where both
  sides must see byte-identical inputs. This does NOT cover the common
  in-process case (an f64 lane against an interval lane, bit-identical
  replay across repeats): that draws once and feeds both sides, so a
  varying seed serves it perfectly.

A third case masquerades as legitimate: a sweep whose real content is
an edge-value table or a product of boundary cases, the RNG only
filling gaps. That is an ENUMERATION — write it as one and let the
filler vary. A deliberate replay corpus must never be called,
described, or budgeted as fuzzing.

Three properties every fuzzer needs, together:

- **A varying seed, logged UNCONDITIONALLY** — always, not only on
  failure, and repeated in assertion messages, or a red run is
  unreproducible. Provide an env override for exact replay, and pin a
  genuine counterexample as an ordinary deterministic test alongside
  its fix.
- **Counts as multiples of a shared EFFORT dial**, shipped at the level
  a gated run should cost, so depth is one env var away.
- **MARKED to run only on changes to the code it was written to test.**
  "The chance it turns up something new isn't technically zero" does
  not justify paying for it on every run — this is adversarially
  reviewed code with good suites and no safety-critical exposure, so
  depth is bought deliberately. A fuzzer that is not gated is a defect
  in the fuzzer.

# Everything else

**Failure isolation is worth less than per-run cost (Evan).** When
several tests rebuild the same expensive fixture, merge them — nextest
is process-per-test, so a `OnceLock` shares nothing and each pays in
full. Compensate by LABELLING each assertion so the failing property is
unambiguous from the message alone.

**A vacuous assertion standing beside a real one is invisible to the
obvious detector.** A rule of the form *"a test whose EVERY assertion is
weak"* cannot see it. The narrowest shape is an assertion whose condition
is the value's own **codomain** — `assert!(sup >= 0.0 || sup.is_nan())` on
a fold of nonnegative magnitudes, `prop_assert!(r >= 0.0)` on
`sqrt(x)` for positive finite `x` — which can only ever change a panic
message, and which typically sits one line from the ceiling that does the
work. It reads like a soundness check, which is why a reader walks past
it. Anyone sweeping for this must key on the **assertion**, not on the
test; the fix is a deletion, not a repair, and the surviving message is
then unambiguous.

**A test that asserts nothing is never a gate.** It cannot fail, so it
cannot gate; it is evidence for a reviewer at the time. See
[[review-and-dependency-policy]] — this is the class to drop first.

**A one-shot comparison artefact expires with its comparison.** A probe
written to be diffed between two revisions is a permanent cost with no
consumer once that diff has been taken. Delete it, or name in-file the
future comparison that schedules it.

**Silent skips are the escape-hatch shape.** A bare `return` at some ε
reports green having asserted nothing. Use the tree's NAMED loud-skip
idiom (`interval_lane_skipped_no_certified_coverage_here`) so the
absence is visible in the battery log.

**Two things that will mislead you if you assume otherwise.** Cost
concentrates savagely — a handful of tests hold most of the test time
and the long tail is free, so profile before cutting. And per-test CI
timings are NOT comparable across legs without normalising; legs differ
enough to manufacture apparent ε-sensitivity. A frequency gate must key
on *source* changes: the change filter's `all` tier fires on most
merges, since demos/, .github/ and scripts/ dominate.
