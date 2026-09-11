---
id: band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps
kind: issue
title: Band::linear's # Errors says BandError arises only on K-epsilon overflow; BandError::Empty is reachable from a VALIDATED tolerance at subnormal epsilon with no overflow
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
