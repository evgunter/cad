---
id: a-carried-fold-off-the-line-escalates-its-lineage-past-a-tenth-of-epsilon
kind: issue
title: The carried fold of three prisms moved 0.5ε or 0.9ε off one line escalates SelfCheck(CarriedLineage) on pm_census_ve_line_gap; the span-first lineage is pinned at 0.1ε only
status: closed
opened: 2026-10-11
priority: P2
cost: M
refs: [the-boolean-records-no-line-touch-and-the-census-does-not-demand-one]
closed: 2026-10-12
pr: 4820
branch: contacthold/one-record-carry-boolean
---

## What

`ops::ee_lineage` (`crates/topo/src/boolean/ops.rs`) places a carried
edge-edge row on the reduction's split pieces by reading each pair's
span first (`census::past_segment`, `census::segments_apart`), then the
segment question on either piece's line (PR 4740). The only row that
reaches it is
`three_solids_on_one_line::the_carried_fold_passes_three_prime_in_every_member_order`,
over `three_off(0.1ε)`.

Measured by PR 4740's review on its head: the same carried fold over
`three_off(kε)` still escalates `SelfCheck(CarriedLineage)` on
`pm_census_ve_line_gap`:

- k = 0.5: member orders [3, 1, 2, 0] and [1, 3, 2, 0];
- k = 0.9: 8 member orders.

It is identical on the merge-base, so not a regression of that PR, and
it is loud (a typed escalation, no body shipped).

`ops::ledger`'s vertex-on-edge split arm (the `stays` read, on
`census::on_segment_interior` alone) did not get the span-first read:
a vertex past the parent piece's span is read on that piece's infinite
line, the reading `ee_lineage` was fixed for. `pm_census_ve_line_gap`
is that arm's question as well as `ee_lineage`'s vertex-on-piece one,
so which of the two escalates is the first thing to measure.

## Done when

The carried fold over `three_off(kε)` for k ∈ {0.5, 0.9} builds and
passes 3′ in every member order, or refuses typed for a reason the row
states; the ledger's vertex-on-edge arm reads the span first or says
why it need not.


## Closed on PR 4820 (2026-10-12)

`ops::ee_lineage` and `ops::ledger`'s vertex-on-edge split arm are
gone: the boolean carries an operand's records through the one carry
(`ops::carried` over `records::landings`), which places each pair by the
census's record confirm, so neither segment question is asked. Measured
on the PR's head, the carried fold over `three_off(kε)`, every member
order:

- k = 0.5: all 24 orders build and pass 3′. The row
  `three_solids_on_one_line::the_carried_fold_passes_three_prime_in_every_member_order`
  now runs this fixture too.
- k = 0.9: 6 orders refuse typed at step 3,
  `Escalated { Coincidence(EdgeOnEdge, Moot) }` on `bool_ee_collinear`
  (margin 1.04e-9, in band); the other 18 build, 9 of them passing 3′
  and 9 refused at 3′ by `CensusEscalated` on `pm_census_ef_cut_gap` /
  `pm_census_ef_cross_side` (margins 1.04e-9 and 1.29e-9, in band). In the
  two of those orders read in full ([3, 2, 1, 0] and [2, 3, 1, 0]) the
  uncarried fold is refused at 3′ too, with 11 findings. No order escalates `SelfCheck(CarriedLineage)`: every
  refusal is the geometry sitting in the escalation band, which is the
  reason the done-when allows.
