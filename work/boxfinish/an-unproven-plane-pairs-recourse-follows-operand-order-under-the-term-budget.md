---
id: an-unproven-plane-pairs-recourse-follows-operand-order-under-the-term-budget
kind: issue
title: An unproven plane pair's recourse follows the order its two faces are read in: the one-plane offset holds its second operand's normal twice in one product, so one order passes the term budget and the other fits it
status: open
opened: 2026-11-24
priority: P2
cost: M
---



Found by the one-plane unit
(`the-one-plane-reading-multiplies-a-normal-by-a-decided-sign`,
`boxfinish/one-plane`, 2026-11-24) while writing its operand-order row.

**What is there.** `geom_brep::plane_eq::plane_ladder`'s offset is
`d₁ − (n₁·n₂)·d₂`, with `dᵢ = nᵢ·(oᵢ − c)`. The second normal enters
`n₁·n₂` and `d₂`, which are multiplied; the first enters `d₁` and
`n₁·n₂`, which are not. So the expression's size is quadratic in the
second operand's normal and linear in the first's, and the mirror form
`(n₁·n₂)·d₁ − d₂` is the other way round. Where the pair is not one
plane as a formula, the coincidence door reports why the replay left
it, and that reason reads `Budget` where the form passes the term
budget (256 terms) and `OneConstruction` where it fits.

**Measured** (the unit's head, default ε, the gate's profile):
`coincidence_door::two_blocks_resting_on_one_floor_are_one_plane_at_their_bases`.
In the corpus scene's order the abutting `SameOpposite PlaneLadder`
pair reads `unproven/Numeric/Budget`; with the union's two members
named in the other order
(`corpus::blocks_on_one_floor::members_exchanged`) it reads
`unproven/Numeric/OneConstruction`. The same rows are proven in both
orders, and every ring seat of `g_ring_closure_measure` proves alike
with its two cells exchanged. On main's form (`d₁ − σ·d₂`, linear in
both normals) all four of the scene's unproven plane rungs read
`OneConstruction` in the stated order; the exchanged order was not run
on main.

Why the size differs is read from the formula, not traced into the
tier: which sub-form freezes was not looked at.

**What it costs.** No verdict moves: the pair is unproven either way
and glues numerically. The recourse a finding names for it follows the
order the document lists the union's members in
(`[intrinsic.contingent]`: how a value was spelled, the order things
were listed in).

**Options, not weighed.**
- Two symmetric claims (each plane holds the foot of the region's
  centre on the other): both forms are asked in either order, so the
  reason is order-free, at a second margin per ladder and `Budget`
  wherever either form passes it. The designers' round 2 chose one
  offset without this measurement.
- The caller hands the ladder a point of each plane that lies in the
  region and is on the plane as a formula; the offset is then linear
  in one normal. The one-plane unit kept this out of scope absent a
  lost proof.
- Leave it: a reason that follows form size under a budget is already
  how the door's `Budget` behaves for any two spellings of one value.

**Done when** the abutting pair's reason is the same in both orders of
the scene's members, or the row above is ruled to stand and this row
closed with that.

Signed: (BOXFINISH one-plane lane)
