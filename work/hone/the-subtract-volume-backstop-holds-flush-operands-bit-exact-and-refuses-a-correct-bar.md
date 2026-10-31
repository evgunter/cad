---
id: the-subtract-volume-backstop-holds-flush-operands-bit-exact-and-refuses-a-correct-bar
kind: issue
title: vol(A ∖ B) ≥ vol(A) − vol(B) charges no rounding for the operands' flush faces, so a correct turned bar refuses ResultVolumeImplausible
status: closed
opened: 2026-10-25
priority: P2
cost: M
pr: 5842
closed: 2026-10-27
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

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. The turned 30ε bar `slab ∖ fill` at ε 1e-6 builds: `shell_role_reader`'s `a_needle_cavity_builds_at_every_definite_width_and_holds_it_as_a_void` and `a_needle_channel_whose_fill_face_leans_reads_its_side_off_its_seam` take it as a cell that builds (tier 3′, one bar, its channel's closed-form volume, its corners) where they pinned the refusal.
