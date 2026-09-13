---
id: band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps
kind: issue
title: Band::linear's AND Band::angular_at's # Errors both say BandError arises only on K-epsilon overflow; BandError::Empty is reachable from a validated tolerance with no overflow, and angular_at reaches it at an ORDINARY epsilon
status: open
opened: 2026-09-11
refs: [2378]
---


## Finding

Measured by WIRE's `names-refusal-carries-cause` lane (PR 2378), which
needed to know whether `Band::linear`'s failure has a unique cause
before it could decide whether a refusal must carry it. It does not.
**Re-verified independently by the WIRE orchestrator** before filing;
the arithmetic is below so no one has to take either party's word.

`crates/geom-core/src/predicate.rs:355-366`, `Band::linear`'s `# Errors`:

> [`BandError`] only when K·ε is not a valid escalation threshold —
> i.e. the run's ε is within a factor K of `f64::MAX`, so the product
> overflows to infinity. Unreachable for any physically meaningful
> tolerance …

**A second arm is reachable, at the opposite end of the range, with no
overflow.** `Tolerance::validate`
(`crates/geom-core/src/tolerance.rs:479-487`) admits **any** finite
ε > 0 and **any** finite K > 1. Take the minimum subnormal
ε = 5e-324 and K = 1 + 2⁻⁵² (the smallest double above 1, so a valid K):

```
eps      = 5e-324           # f64::MIN_POSITIVE subnormal
K        = 1.0000000000000002
K * eps == eps              # True
```

The increment `K·ε − ε` is below half of 2⁻¹⁰⁷⁴ and rounds away, so
`K·ε` **is** `ε`, `Band::new(ε, K·ε)` has `zero == escalate`, and the
band is rejected as `BandError::Empty` — the *"a zero-or-negative-width
band is a different design"* arm. Nothing overflowed.

For any **normal** ε it genuinely cannot happen: K ≥ 1 + 2⁻⁵² forces the
product to exceed ε by at least one ulp. That is presumably the reading
the sentence was written from, and it is true of every tolerance anyone
would set. It is not true of the set the validator admits, which is what
the doc's word *"only"* claims.

## Why it mattered to a caller, which is how it was found

`SelectRefusal::Band` was a **unit variant** — the `BandError` was
discarded — and the defence for that was this doc: one cause, so naming
it adds nothing. With two reachable arms the defence fails, and **the
two ends want opposite repairs** (ε ≈ `f64::MAX`: lower ε; subnormal ε
with K ≈ 1: raise one of them). A refusal that names neither sends half
its readers the wrong way. PR 2378 carries the payload at all four
sites on exactly this argument.

## What a taker owes

The `# Errors` sentence made true — both arms named, with the honest
"unreachable for any physically meaningful tolerance" kept for the
overflow one, since that part is still right. Optionally a validator
row pinning the two reachable arms; PR 2378 pins them at the
`editor-core` end (`crates/editor-core/tests/display_contract.rs`,
`band_refusals_name_which_band_failure_they_caught`) as assertions over
the validator's invariants, and the home for a `geom-core` version is
`predicate.rs`'s own `mod tests`.

**Not owed**: a change to `Tolerance::validate`. Admitting subnormal ε
may or may not be right, but that is a separate question and this row
does not ask it — a doc that describes the validator it has is the fix
here.


## Corrected and widened by PR 2378's full review (2026-09-11)

Two amendments, both from the review that attacked the lane's numbers
rather than accepting them. **The finding stands; two of its sentences
were wrong.**

### 1. The collapse boundary is much wider than "K within an ulp of 1"

The original filing (and PR 2378's own rustdoc) framed the reachable set
as ε subnormal with K = 1 + 2⁻⁵². That is one point in it, not its edge.
Measured (`num.py`, `num2.py` in the review lane's scratch):

> At ε = 5e-324, **every K < 1.5 collapses the band** — 1.1, 1.25 and 1.4
> all give `K·ε == ε`. 1.5 is the first that does not (ties-to-even).

The true condition: **`Empty` is reachable iff ε is subnormal with
ε < 2⁻¹⁰²³ ≈ 1.11e-308, AND K < 1 + 1/(2n), where ε = n·2⁻¹⁰⁷⁴** — or,
as PR 2378's fix pass states it exactly, iff `fl(K·n) == n`. The ulp
framing made the hazard sound like a knife-edge; it is a region.

**And the region needs TWO knobs turned, which this file said wrongly
the first time.** An earlier revision here claimed the default K = 10
*"sits outside it only because ε does"* — backwards. At **K = 10 the
collapse arm is unreachable at every ε**, subnormal included
(10 · 5e-324 = 5e-323 ≠ 5e-324). Reaching it needs `CAD_AMBIGUITY_K`
below 1.5 **as well as** a subnormal `CAD_TOLERANCE_EPS`. That bounds
the hazard considerably and it is the honest framing: K = 10 is the
permanent ratified default (`docs/K-REPORT.md`, #89 closed), so no
default run can meet this. The two arms are also disjoint — no tolerance
reaches both.

### 2. `Band::angular_at` has the same false sentence, and it is WORSE

`crates/geom-core/src/predicate.rs:393-402`. Same *"only when … overflows"*
claim, and `BandError::Empty` is reachable there from a **physically
ordinary** tolerance:

> ε = 1e-9 (the session-box default order) with `lever_arm = 1e300` gives
> `zero = ε/lever_arm = 1e-309` — subnormal — and any K in
> `[1 + 2⁻⁵², 1 + 2.5e-15)` collapses it.

No absurd tolerance is required. What is extreme is the **lever arm**,
and a lever arm is a **caller argument** supplied per predicate, not a
run configuration an operator sets once. `Band::angular_at`'s own doc
tells callers to name the arm the decision turns on and lists the
session-box extent as the conservative universal choice — so a large arm
is the documented road, not a misuse.

**Class-not-instance: a fix pass that corrects only `Band::linear` ships
a half-fix and should be labelled one.** The instrument is every
`# Errors` section in `predicate.rs`, not the one line PR 2378 happened
to need.

### Who else read the wrong sentence

`grep -rn "BandError" crates/ --include=*.rs | grep -v geom-core` shows
**15+ enums** carrying a `BandError` across `topo`, `geom-brep`,
`profile` and `mesh`. Every one of those authors read this `# Errors`
section to decide what their variant owes. That is the reach of the
defect, and it is the argument for fixing the sentence at its home
rather than at the sites that cite it.
