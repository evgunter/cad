---
id: plate-rim-residual-needs-the-wide-coefficient-ring
kind: issue
title: the plate's rim residual (carrier_endpoint_start) needs a coefficient ring wider than the shipped 256 bits, and rule C alone does not certify it
status: open
opened: 2026-09-05
refs: [M10-8]
---

**Measured by M10-8's fix pass**, on the two-hole plate's real study
(±0.05 mm spacing, σ = 0.01 mm radii; `demos/tour` stop 1). The plate
is bounded by `carrier_endpoint_start` — the arc rim's `‖q − c‖ = r` —
whose plain normal form is a `sqrt` whose argument is a degree-12
polynomial in the hole radius (products of `sqrt(…)^12`, `abs(…)`,
`sqrt(…)^3`, every radius factor a `sqrt` of the expanded square
`(a + 2r)²`, `a` and `r`'s nominal both `f64` literals with 53-bit
mantissas).

## What each mechanism reached

| ring | rule set | at the real study | frozen (nominal) |
| --- | --- | --- | --- |
| `i128` (M10-7) | any | refuses on `carrier_endpoint_start` | 1,056 |
| BigInt, 4096-bit bound | A0 | refuses on `carrier_endpoint_start` | 0 |
| BigInt, 4096-bit bound | A0 + rule C in the early walk | **2 decisions `sign_gated`**, still refuses on `carrier_endpoint_start` | 0 |
| BigInt, 256-bit bound (SHIPPED) | A0 + rule C | refuses on `carrier_endpoint_start` | see the fix-pass logs |

So rule C in the candidate shape (`sqrt(X) − R` with `NF(X) = NF(R)²`,
`crates/geom-core/src/sym/signed.rs`) DOES fold on the plate — twice
at the whole box — and the ceiling does not move: the rim residual's
outer `sqrt` is not discharged by folding its inner roots. What bounds
it next is recorded by
`m10_8_arc_family_interval::m10_8_what_bounds_each_document_past_its_ceiling`
(the rendered residual under the shipped set, in the fix-pass notes and
the PR body's §1).

## Why 256 bits ships

At 4096 bits nothing on R2's bracket froze and one leaf replay took
229 s against M10-7's 5.9 s (`m10_8_leaf_cost_per_rule_set`): with the
constant fold on, every coefficient is a product of dimensions' 53-bit
mantissas, and BigInt arithmetic on thousand-bit coefficients at the
term budget is the whole cost. At 256 bits the bracket and annulus
ceilings keep the factors measured at `i128` (10.4× and 39×) at
2.1–4.5 s per leaf, and the worst forms freeze again. The plate's rim
residual needs ~640 bits and more, and does not fit — the measured
trade, taken on cost (`geom_core::sym::COEFF_BITS`).

## What is owed

- A coefficient representation whose cost does not scale with the bit
  width where the width is not needed: a small-integer fast path
  (`i128` inline, BigInt on demand) so the bound can be raised without
  the bracket paying for the plate.
- With the bound raised, the plate re-measured under A0 + C: whether
  the outer `sqrt` reduces once its inner roots are folded (the fix
  pass saw `sign_gated: 2` and no ceiling move at 4096 bits), and if
  not, which residual bounds it — with its rendered form.
- The tour's stop 1 re-cut as the certified study if it certifies.
