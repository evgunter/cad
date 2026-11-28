---
id: an-unproven-plane-pairs-recourse-follows-operand-order-under-the-term-budget
kind: issue
title: An unproven plane pair's recourse follows the order its two faces are read in: the one-plane offset holds its second operand's normal twice in one product, so one order passes the term budget and the other fits it
status: closed
opened: 2026-11-24
priority: P2
cost: M
closed: 2026-11-28
pr: 7710
branch: boxfinish/one-plane
---



Found by the one-plane unit
(`the-one-plane-reading-multiplies-a-normal-by-a-decided-sign`,
`boxfinish/one-plane`, 2026-11-24) while writing its operand-order row.

**What is there.** `geom_brep::plane_eq::plane_ladder`'s offset is
`d₁ − (n₁·n₂)·d₂`, with `dᵢ = nᵢ·(oᵢ − c)`, and the face-pair door's
bound holds the same product. Where a union's plane pair is not one
plane as a formula, the coincidence door reports why the replay left
it: `coincide::replay::prove_row` discharges the margin the union
RECORDED for the pair (`replayed.margin`), and names `Budget` where a
form of it passes the door's term budget (256 terms) and
`OneConstruction` where the forms fit and do not reduce to zero.

**Measured** (the unit's head, default ε, the gate's profile):
`coincidence_door::two_blocks_resting_on_one_floor_are_one_plane_at_their_bases`.
- In the corpus scene's order the abutting `SameOpposite PlaneLadder`
  pair reads `unproven/Numeric/Budget`; with the union's two members
  named in the other order
  (`corpus::blocks_on_one_floor::members_exchanged`) it reads
  `unproven/Numeric/OneConstruction`. The same rows are proven in both
  orders, and every ring seat of `g_ring_closure_measure` proves alike
  with its two cells exchanged. On main's form (`d₁ − σ·d₂`) all four
  of the scene's unproven plane rungs read `OneConstruction` in the
  stated order; the exchanged order was not run on main.
- **Asking the exchanged form where the first ends on `Budget` buys
  nothing at the door's own reading** (a mutant of
  `coincide::replay::carrier_verdict`, not kept: on
  `Undischarged::OverTheBudget` it reads the pair again with the two
  seats exchanged). In each scene two pairs end over the budget, the
  exchanged ask is over the budget for both too, three forms a margin
  each time (so six more forms a scene), and no label moves. The door's
  own ladder reading is over the budget in BOTH operand orders for the
  pair whose label follows the scene's order, so the label is not that
  reading's: it is the recorded margin's, which the union took over its
  own extent in its own order.
- **It is the budget.** With the door's budget at 1024 terms (a mutant
  of `replay::BUDGET`, not kept) all four unproven plane rungs read
  `OneConstruction` in both orders, as on main, and no direct reading
  ends over the budget. The row cost 5.8 s there against 1.2 s at 256.
  So every form of these margins lies under 1024 terms, and the
  recorded margin of the abutting pair straddles 256 between the two
  orders.

**The cause is not traced into the tier.** Which form of the recorded
margin passes 256 terms in one order and not the other was not looked
at. The unit first read the cause off the formula (the offset holds
its second operand's normal twice in one product, so it is larger with
the heavier normal second); the exchanged-ask mutant does not support
that as the whole account, since the direct reading is over the budget
either way.

**What it cost: more than a label.** This row first said no verdict
moves. That was false, and the review of PR 7710 showed it on the
frozen head `a99cefc81`: the one offset `d₁ − (n₁·n₂)·d₂` is linear in
the first normal's sign factor and quadratic in the second's, so
- whether a pair is PROVEN one plane followed the operand order (of ten
  spellings of a sign factor, three proved in one order only, at the
  ladder and at the pair door, one of them still at 4,096 terms), and
- at `f64` and `Interval` the verdict itself followed the order at the
  band's edge (121 of 400k near-band poses at the pair door, 151 at the
  ladder, 34 of 20k at `Interval`; main's form: none).

The label on the floor row was the same defect seen from below the
proof level.

## Closed (2026-11-28, PR 7710's fix pass)

The reading is a symmetric function of the pair
(`geom_brep::plane_eq::pair_offset`): the ladder and the pair door read
the geometric mean of the two mirror offsets, `√|e₁₂·e₂₁|`, which is
the same number with the planes exchanged and is zero as an expression
where either mirror offset is. Measured on the fix pass:
- the ten-spelling matrix proves every spelling in both orders, at 256
  and at 4,096 terms
  (`plane_eq::tests::a_sign_factor_spelled_ten_ways_proves_alike_in_either_order`);
- 20,000 near-band poses read alike to the bit in both orders at `f64`
  and, every twentieth, at `Interval`
  (`plane_eq::order_fuzz::a_plane_pair_reads_alike_in_either_order`);
- the floor row reads the same labels with the union's members
  exchanged, and asserts it: in both orders one flush pair reads
  `OneConstruction` and the other three unproven plane pairs read
  `Budget` (one more rung than the one-order form named the budget
  on; that it is the product of the two mirror offsets passing 256
  terms is read from the pins, not traced in the tier).

**What is left, and where it is.** Nothing of the order dependence.
The cost of the symmetric form is a larger residual on a pair that is
NOT one plane as a formula: three of the floor scene's four unproven
plane pairs now name the budget where main named one construction.
That is the door's term budget against a form's size, the same
question as any other `Budget` reason, and no row is opened for it
here. The measurement above at 1,024 terms was taken on the one-order
form; it was not re-taken on the symmetric one.

Signed: (BOXFINISH one-plane lane)
