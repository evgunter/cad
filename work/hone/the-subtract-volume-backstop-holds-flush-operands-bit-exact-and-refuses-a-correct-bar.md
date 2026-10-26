---
id: the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar
kind: issue
title: vol(A ∖ B) ≥ vol(A) − vol(B) charges no rounding for the operands' flush faces, so a correct turned bar refuses ResultVolumeImplausible
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Found by FLUSH's needle-cavity lane (`flush/needle-cavity-section-loop`,
2026-10-25). Measured on main (merge base `28e9e447`) and on the branch,
with the same result on both.

**What happens.** Take `slab ∖ fill`, with the slab
`(0,1)×(0.5,0.8)×(0.5,0.5+w)` and the fill
`(0,1)×(0.5+w,0.8)×(0.5,0.5+w)`, at w = 30ε and ε 1e-6, built with
`mapped_cube` and turned about `(1, 2, 3)` by 1.1 rad. It refuses
`ResultVolumeImplausible { which: "vol(A ∖ B) ≥ vol(A) − vol(B)",
got: 9.000000034156605e-10, bound: 9.000000140051221e-10 }`. The result
is the `w × w × 1` bar (`w² = 9e-10`), and it is right: the
axis-aligned pose builds it, and so does the turned
`slab ∩ (brick ∖ fill)`, which has no such backstop arm. The probe is
`crates/topo/tests/needle_cavity_probe.rs` (`#[ignore]`d).

**Why.** `boolean/ops.rs` `bound_holds` re-derives the violation in
interval arithmetic, so each body's volume is certified for the
geometry it stores. But the fill is flush with the slab on five faces,
and each operand's corners were rounded separately. `vol(A) − vol(B)`
therefore integrates two copies of the flush faces that differ by
rounding, about `1e-17` here: `δp ≈ u/2` over the faces' area. The
result holds neither copy. The inequality holds for the geometry the
glue decided Zero, not for the stored bits. The deficit is
`1.06e-17`, against a `vol(A) ≈ 9e-6`.

**What is owed.** Charge the bound for the operands' positional
rounding over the area the result glued or dropped, or skip the arm
where the margin lies within that charge (the backstop is a kernel
self-check, so an uncharged margin it calls certain is a false
`ResultVolumeImplausible`). Then pin the turned ε 1e-6 30ε bar.

## Covered by the glue's widening (CLEAVE, 2026-10-26)

`cleave/backstop-admits-the-glues-zero-band` (PR 5757) widens each
backstop bound by what the op's one-carrier glue admits: the band's
zero times each group of glued faces' area less its smallest face's
(`glue_admits`, `crates/topo/src/boolean/ops.rs`). The fill's five
flush faces glue on one carrier at Zero, so the 1.06e-17 deficit lies
far inside that widening. The turned 30ε `slab ∖ fill` now builds at
ε 1e-6, both upright and with the fill face leaning, and
`shell_role_reader.rs` pins it as built (`Want::Backstop` is gone).
The widening is the zero over the glued area, not the operands'
positional rounding this row asks for, so a flush pair that does not
glue is not covered by it.
