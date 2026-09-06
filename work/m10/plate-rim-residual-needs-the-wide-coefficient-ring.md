---
id: plate-rim-residual-needs-the-wide-coefficient-ring
kind: issue
title: the plate's rim residual (carrier_endpoint_start) needs a coefficient ring wider than the shipped 256 bits, and rule C alone does not certify it
status: closed
opened: 2026-09-05
closed: 2026-09-06
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

## Closed by measurement (M10-9, 2026-09-06): the ring table

M10-9's §4 asked for the table this row is owed, on the two documents
the trade was argued over, at the DEFAULT ε with the shipped rule set
and the registered-identity door open. One leaf, timed; the ceiling
answered as a bracket (half the 256-bit ceiling must certify whole,
twice it must refuse) rather than as a digit
(`editor-core/tests/m10_9_evidence_interval::m10_9_ring_table`, run
three times against three edited values of
`geom_core::sym::COEFF_BITS`).

| `COEFF_BITS` | plate, one leaf | plate ceiling | R2 bracket, one leaf | bracket ceiling |
| --- | --- | --- | --- | --- |
| **256 (shipped)** | 0.21 s | `[7.787e2, 7.817e2] · ε` | 0.65 s | `[3.865e2, 3.880e2] · ε` |
| 1024 | 0.20 s | unmoved | 24.74 s (**38×**) | unmoved |
| 4096 | 0.22 s | unmoved | 34.01 s (**52×**) | unmoved |

("unmoved" is both ends asserted: half the 256-bit ceiling still
certifies whole and twice it still refuses, at every bound.)

**No bound change ships.** M10-9's rule was that one ships only if it
MOVES a ceiling at no more than 2× the bracket's leaf cost; neither
1024 nor 4096 moves a ceiling at all, and both cost the bracket 38×
and 52×. The wide ring was the alternative to the door, and it was
never affordable: what the ~640-bit expansion would have bought on the
plate's rim residual, the door buys for nothing — and, as it turns
out, the plate's ceiling does not move either way, because what bounds
it is a different identity
(`work/m10/plate-ceiling-is-now-the-arc-span-identity`).

The first two bullets of "what is owed" above are therefore answered
in the negative and by measurement: a small-integer fast path already
exists (`Int::Small` inline, `BigInt` on demand — M10-8), the bound
was raised twice with it in place, and the cost is coefficient GROWTH
rather than heap traffic. The third — the tour's stop 1 as the
certified study — is not reachable at this ceiling; the stop's caption
now names the bounding predicate and its enclosure instead.
