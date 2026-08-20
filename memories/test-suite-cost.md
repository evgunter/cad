---
name: test-suite-cost
description: Standing rules for what a test may cost the CI suite — ALL fuzzing must vary its seed (never hardcode one), scale by an EFFORT dial, and be gated to the code it tests; failure isolation is worth less than per-run cost; assertion-free tests are never gates
metadata:
  type: feedback
---

Rulings from the 2026-08-13 whole-suite test-time audit. The audit
itself is history; these are the rules that outlast it.

# Fuzzing

These apply to **ALL fuzzing in this repo** — every randomized sweep,
property sweep, adversarial sweep and fuzz row, existing or new,
wherever it lives (`tests/` or an inline `#[cfg(test)]` module). Not
just the ones the 2026-08-13 audit happened to touch.

**A fuzzer MUST NOT FIX ITS SEED (Evan, 2026-08-13).** No hardcoded
literal, no `const SEED`, no seed derived from a loop counter. Draw it
fresh per run.

A hardcoded seed does not make a weak fuzzer — it makes something that
is **not a fuzzer at all**. It explores exactly the same points on
every run for the rest of the project's life, so after its first green
run it can only ever fail because the code under test changed. At the
audit, *every* seed in the workspace was a hardcoded literal across 44
RNG-driven tests, which is why the whole family was re-deriving known
answers at full price on every PR.

**FIRST, ask which SHAPE the test is** — because "a fuzzer must not fix
its seed" read as "everything needs a random seed" is how a coverage
test becomes flaky. Three shapes, and only one wants a varying seed:

- **Counterexample search** (*for all sampled x, P(x)*) — vary the seed.
  Monotone in the safe direction: cutting the count loses detection
  power, never correctness.
- **A witness you can WRITE DOWN** (*at least K of class C*, C concisely
  constructible) — do not search at all. Build it as a static fixture
  and assert it every run. Hunting for something you could construct
  buys it on ~99% of runs instead of 100%.
- **A witness you CANNOT write down** (same, but C is not concisely
  specifiable — "a walk that reaches every op kind") — FIX THE SEED. It
  is a fixture identifier, not a sampling strategy, and it cannot flake.

**The trap is mixing the first and third in one test.** A property test
with an anti-vacuity floor bolted on is both at once, and its sample
count then feeds two obligations of which only one is safe to cut. Make
the floor's witness static, or split the test.

**When a fixed seed IS right** — narrower than it sounds, and each case
must say in-file which one it is. An unexplained literal seed is the
failure mode; these are the only excuses:

- **A coverage/witness claim of the third shape above.** Condition
  (Evan, 2026-08-13): K must be large enough — or the simultaneous
  conditions numerous enough — that the row is VERY UNLIKELY TO PASS BY
  ACCIDENT on a lucky seed. K = 1 against a 1-in-1000 class is the shape
  to avoid: the single seed that happens to work then carries the whole
  claim. `topo`'s `seqgen` coverage rows clear it comfortably:
  they must reach every op kind AND every site shape at once, so a seed
  that satisfies them is a good seed rather than a lucky one.

- **A pinned counterexample.** A fuzzer found a real defect and that
  exact case is now a permanent regression row. Prefer writing the
  input OUT as an explicit fixture — a seed is an obscure way to name
  an input. A seed is acceptable only as compression when the input is
  genuinely too big to write, and then the doc must say "this seed
  reproduces #N": it is a fixture identifier, not a sampling strategy.
- **Cross-PROCESS or cross-BUILD differential comparison**, where both
  sides must see byte-identical inputs — merge-base vs tip, debug vs
  release, machine vs machine. There the shared seed is the mechanism.
  This does NOT cover the common in-process case: a test comparing an
  f64 lane against an interval lane, or asserting bit-identical replay
  across repeats, draws once and feeds both sides, so a varying seed
  serves it perfectly. Almost every "differential" test in this repo is
  the in-process kind.

A third case masquerades as legitimate: a sweep whose real content is
an edge-value table or a product of boundary cases, with the RNG only
filling gaps. That is not a fuzzer with a fixed seed, it is an
ENUMERATION — write it as one, and let any filler vary.

Whichever applies, name it in the test's own docs. A deliberate replay
corpus must never be called, described, or budgeted as fuzzing.

Three properties every fuzzer needs, together:

- **A varying seed, logged UNCONDITIONALLY.** Always, not only on
  failure, and repeated in assertion messages — otherwise a red run is
  unreproducible, which is the one failure mode that matters most.
  Provide an env override so a failure can be replayed exactly, and
  pin a genuine counterexample as an ordinary deterministic test
  alongside its fix. Finding the case is the fuzzer's job; being the
  regression gate for it afterwards is not.
- **Counts as multiples of a shared EFFORT dial**, shipped at the level
  a gated run should cost, so depth is one env var away.
- **MARKED to run only on changes to the code it was written to test.**
  Not on every PR. "The chance it turns up something new isn't
  technically zero" does not justify paying for it on every run — this
  is thoroughly adversarially reviewed code with good suites and no
  safety-critical exposure, so depth is bought deliberately, not
  sprayed across unrelated changes. A fuzzer that is not gated is a
  defect in the fuzzer.

# Everything else

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

**A one-shot comparison artefact expires with its comparison.** A probe
written to be diffed between two revisions — a printed hash, a recorded
stream, a pinned draw feeding a cross-build differential — is a
permanent cost with no consumer once that diff has been taken. Delete
it, or name in-file the future comparison that schedules it. Re-scoping
its doc to keep it is how one becomes permanent.

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
