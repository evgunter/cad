---
id: a-charge-ties-jet-at-an-arm-near-r-reads-past-a-barely-decided-parting
kind: issue
title: A charge tie's jet stands on any decided parting, so at an arm near R a parting barely off the zero band leaves the side to the first order while unbounded higher orders can outweigh it
status: open
opened: 2026-11-20
priority: P2
cost: M
refs: [a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side]
---


Filed by GERM's orchestrator from PR 7174's delta review (2026-11-20).

## What

`bound_side`'s `read_down` lets a tie that the charge made stand on its
jet only when the parting (the relative second order) is decided off
the zero band. The case it refuses is a parting *inside* the zero band.
A parting just barely decided still leaves the jet's sign to the first
order (`s·arm`) wherever `|rel|·arm/2 ≪ s`. At an arm near R, the orders
past the second that the jet does not read are unbounded. On a sphere
the fourth-order term is about `l⁴/8R³`, which is 0.125 at `l = R`.

The review's unit-level probe: R=1, s=1e-6 and rel=−3e-8 read
`Leaves(Above)`, with a parting of −1.5e-8 and a jet of about +9.85e-7.
The fix pass's own row (`sectors::a_charge_tie_reads_its_jet_on_a_decided_parting`,
the case "a parting below, a jet above" → `Out`) pins this regime. No
pose is known that reaches a wrong body.

Also, from the same review (N2): the parting gate maps every `Err(_)`
to the charge's refusal. An in-band parting outside the gap therefore
quotes the charge's Zero (about 5e-17) where it used to escalate as
`ReadFault::Parting` with its own `SectorCurving` lever. The lever
offered is still right, but the diagnosis is lost.

## Owed

Bound the next order on the bracket, or require the parting to dominate
the rise rather than merely be decided. A pose that reaches a wrong
body would make this P0. Restore the parting's own escalation where it
is the reading that decided nothing.
