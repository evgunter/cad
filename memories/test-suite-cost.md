---
name: test-suite-cost
description: Standing rules for what a test may cost the suite — all fuzzing varies its seed, scales on an EFFORT dial, and runs every time at EFFORT=1 with its DEPTH gated to the code it tests; assertion-free tests never gate
metadata:
  type: feedback
---

# Fuzzing

These bind **ALL fuzzing in this repo** — every randomized sweep,
property sweep, adversarial sweep and fuzz row, wherever it lives.

**A fuzzer MUST NOT FIX ITS SEED (Ev, 2026-08-13).** No hardcoded
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

- **A coverage/witness claim of the third shape.** Condition (Ev): K
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
- **Counts as multiples of a shared EFFORT dial**, shipped at the smoke
  level EVERY run pays, so depth is one env var away.
- **ALWAYS RUN, at EFFORT = 1. The marker buys DEPTH, not existence**
  (Ev, 2026-09-11). Every fuzzer runs on every run at the shipped smoke
  level; the marker naming the code it was written to test selects which
  ones then run DEEPER — **on its NAMED PATHS against the diff, never on
  the crate closure** (Ev, 2026-09-11). A closure reaches every ancestor
  of a file, so depth keyed on one is bought by changes that cannot
  affect the sweep; the marker exists to say which few files it is
  actually about, and that is the set that buys depth. This also means
  the fail-open direction INVERTS for depth: a run that cannot resolve
  the diff runs everything at EFFORT = 1 and nothing deep, because
  failing open on existence means running more and failing open on depth
  would mean running everything deep on the tier-`all` runs that are
  most merges. "The chance it turns up something new isn't
  technically zero" still does not justify paying for DEPTH on every run
  — this is adversarially reviewed code with good suites and no
  safety-critical exposure — but it does not justify paying nothing
  either, and at EFFORT = 1 a sweep costs about what its process costs.
  A fuzzer whose depth is not keyed to the code it tests is a defect in
  the fuzzer.

  **Why the smoke level is not skipped.** What a skip saves is wall
  clock on the test legs, and only that: the test binaries are compiled
  into the archive whether or not they execute, so the build — the run's
  longest job — does not move. And a skip fails SILENTLY: a marker that
  resolves to nothing, omits a helper, or sits on a `#[path]`-mounted
  file leaves the suite not running while the tree reports a green gate.
  At EFFORT = 1 the same broken marker costs depth instead of existence,
  and the row still compiles, still executes, and still catches a panic
  on every run. **The failure mode is the argument**, not the seconds.

  **EFFORT = 1 is a COUNT, never a timeout.** A time-based cutoff makes
  what the test explored depend on the machine, so it differs per leg,
  cannot be reproduced from the logged seed, and manufactures apparent
  ε-sensitivity (see the last bullet of this file). A wall-clock ceiling
  over the whole EFFORT = 1 population is a fine TRIPWIRE — it reds when
  the smoke level stops being one — but it is never the dial.

  **The premise is that the binary is compiled ANYWAY, and there is one
  place it is not.** A kernel fuzz row costs only execution, because
  `build + archive` compiles it into the nextest archive whether or not
  it runs. A sweep in a SEPARATE CARGO ROOT that a pull request does not
  otherwise build costs its whole compile — `interval-transcendentals/`
  is ~234 s of build to buy ~7 s of cases at EFFORT = 1. There the
  job-level gate stands as it is, and the depth argument applies to the
  LANE rather than to the row: that job already runs at EFFORT = 8 on the
  changes that reach it, which is this rule's shape and its precedent.

  **This does not reach a shape-3 row.** Where the count IS the coverage
  claim (*at least K of class C*, C not concisely constructible), it is
  anti-monotone and EFFORT must not scale it below its floor. That is the
  mixing trap named above, and it is why the floor's witness is static or
  the test is split.

# Everything else

**Failure isolation is worth less than per-run cost (Ev).** When
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
