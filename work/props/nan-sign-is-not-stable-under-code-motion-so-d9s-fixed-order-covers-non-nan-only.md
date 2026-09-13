---
id: nan-sign-is-not-stable-under-code-motion-so-d9s-fixed-order-covers-non-nan-only
kind: issue
title: A Mat3/Affine3 product's NaN sign and payload differ between debug and release because LLVM commutes fadd across inline sites, so D9's fixed-order determinism holds for non-NaN outputs only
status: open
opened: 2026-09-11
refs: [2375]
---


## Finding

Measured by the full review of WIRE's PR 2375, which built a
differential harness over 200 000 random `Frame` pairs drawn from
`f64::from_bits` — NaNs, ±inf, subnormals, ±0, `f64::MAX`/`MIN` — to
falsify a bit-identity claim about `Frame::compose`. Confidence `sure`,
the reviewer's. The harness is at
`~/.local/share/cad-work/wire-r1-scratch/difftest.rs` if that lane's
scratch survives; it rebuilds from this description in minutes.

**In debug: zero divergence.** In `--release`: divergence in exactly one
place, the **sign bit of a generated NaN** — `fff8000000000000` against
`7ff8000000000000` — where one summand is a propagated input NaN and
another is x86's QNaN-indefinite from `(-0.0) * inf`.

**It is not a source-level difference.** Both spellings under test called
the *identical* `Mat3::mul`; this is LLVM commuting `fadd` across two
inline sites. Which makes it a standing fact about
`crates/geom-core/src/linalg/`, not about the PR that found it:

> **a `Mat3` or `Affine3` product's NaN payload and sign are not stable
> under code motion**, so D9's "fixed evaluation order" buys determinism
> for **non-NaN outputs only**.

## Why it is worth a row rather than a footnote

`docs/DESIGN.md`'s D9 is cited across this tree as the authority for
bit-level claims, and several of them are written without the
qualifier — `Mat3::determinant`'s "exact evaluation order (D9)" and
`Frame::compose`'s "D9-deterministic, not claimed exact" among them.
Each is **true as intended**: every one of those doors is reached with
finite inputs, and this kernel refuses non-finite geometry at its gates
rather than computing with it. So this is not a live bug and no output
is known to be wrong.

What it is, is a premise nobody has written down, on the one axis where
a reader would assume the opposite: "fixed order ⇒ same bits" is exactly
the kind of sentence a later lane will lean on, and it has an exception.

## What a taker owes, and what it does NOT

**Owed**: the qualifier stated once, at the home — `crates/geom-core/src/linalg/`'s
own docs, beside the operators — rather than at each of the sites that
cite D9. And, if a cheap one exists, a guard: a row asserting that two
spellings of one product agree bitwise on **finite** inputs, which is the
claim that actually holds and which nothing currently pins.

**Not owed**: a change to `docs/DESIGN.md` D9 unless the reading here is
wrong. D9 is a decision about arena order and evaluation order in the
kernel's own computations; it does not claim anything about NaN
propagation, and reading it as if it did is this row's own inference. If
a taker judges that the qualifier belongs in D9's text rather than in
linalg's, that IS a design-doc change and goes to Ev as one.

## Rider, from the same review (MINOR 2)

`crates/geom-core/src/linalg/mat.rs`'s `mod tests` has **no row pinning
`Mat3::Mul`'s own product**. The review's mutation M9 permuted the
product's columns inside `Mat3::mul` and every test in the consuming
crate stayed green, because the consumer's oracle was built from
`Mat3::Mul` itself. The nine multiply-adds every composed placement
rides on are unpinned at their home. That row belongs here, in
`geom-core`, and is smaller than the NaN question above — take it first.
