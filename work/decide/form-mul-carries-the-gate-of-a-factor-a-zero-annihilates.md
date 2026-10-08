---
id: form-mul-carries-the-gate-of-a-factor-a-zero-annihilates
kind: issue
title: Form::mul carries the gate of a factor a zero annihilates
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [the-decision-read-answers-theorems-the-must-carry-stations-would-prove, DECIDE-9]
---


`Form::mul` (`crates/geom-core/src/sym/form.rs`) builds a product's
`gated` flag as `self.gated || other.gated`. Where one factor is the
zero form and ungated, the product is zero wherever the other factor
has a value, and the poison check above it guarantees a value. So the
product is an unconditional theorem, and the flag reports it as resting
on the other factor's read.

DECIDE-9 fixed the same shape at `combine`'s early zero arm in
`geom_core::sym`. That is the site every `Mul` NODE in the shipped early
walk reaches first, and the site the pad's 32 and the bracket's 16
`dihedral_wedge` decisions went through
(`the-decision-read-answers-theorems-the-must-carry-stations-would-prove`,
"What Phase 1 found"). `Form::mul` is the combinator below that arm. It
is reached with a zero factor only:
- where `combine`'s arm is off (`early_ab` and `trig_of_atan` both
  shut), or
- from inside a rule (`powi_form`, rule D's recurrence, the per-node
  reduction).

DECIDE-9's trial counter in `Form::mul` (an ungated zero times a gated
factor) printed nothing on R2's bracket replay at `certifies_at` or on
R2's pad leaf at `1e2·ε`. So no measured document reaches it today. It
is the conservative direction: a theorem reported as gated, never the
reverse.

The fix has the same shape as `combine`'s: a product with a zero factor
takes the AND of its zero factors' gates. It also moves `Form::digest`
for such a product, so a row should show that no atom key a measured
document mints moves.

**The class, whole.** DECIDE-9's review names these sites. Each builds
a product's gate as the OR of its factors' gates:
- `Form::mul` (`sym/form.rs`);
- `powi_form` (`sym.rs`), which multiplies through `Form::mul`;
- `algebra::apply` (`sym/algebra.rs`, the `gated: out.gated || f.gated`
  of a substitution step).

The rule they would take is `zero_factors_gate` in `sym.rs`: a product
is gated exactly when every zero factor is gated. That helper is the one
home of the rule, and these sites should call it rather than restate it.
