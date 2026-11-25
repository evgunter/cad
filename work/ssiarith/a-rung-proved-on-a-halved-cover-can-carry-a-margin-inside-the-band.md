---
id: a-rung-proved-on-a-halved-cover-can-carry-a-margin-inside-the-band
kind: issue
title: limb 3's widest proving rung can carry a crossing bound inside the band, and the ladder escalates there instead of taking a narrower rung
status: open
opened: 2026-11-25
priority: P2
cost: M
---



Filed by SECTARMS's `sectarms/limb3-halves-cover` lane (PR 7776, the
build of Ev's decision on PR 7056).

## Measured

`sweep` `germ_torus_components::two_donuts_in_general_pose_pass_the_germ_mint`,
placement 2, at ε = 1e-6, under ∪ in both member orders:

| | main | PR 7776 |
|---|---|---|
| `A ∪ B` | the germ's march refuses `TubeStraddles` | `Join(Euler(DescriptionUncertified { edge: Minted, error: AnalyticRung3(Escalated { limb: Tube, margin 3.61e-6, band [1e-6, 1e-5], predicate ssi_tube_transversality }) }))` |
| `B ∪ A` | the same refusal | builds one seamed solid |

On the PR the march carries the pair (limb 3 halves its cover and
proves a tube), the germs mint, and the join mints an edge whose
at-rest certificate escalates: its tube is proved, a graph and one arc,
and the bound on the crossing angle over that rung's cover, levered by
the extent, is 3.6e-6 m, inside the band. At the default ε and at
1e-12 both orders build.

## Why

`ssi::certify::limb_three` takes the widest rung whose chain is a graph
and one arc, and decides that rung's margin once
(`tube_transversality`). A margin inside the band escalates, and no
narrower rung is asked. Before the cover was halved a wide rung was
either no graph or a graph by a margin of the pose's own size. A halved
cover stops as soon as every box's enclosure is zero-free
(`CoverBase::at`), so the widest rung that proves is often one that
proves by little: its least box is a graph by whatever the last halving
left. Whether a narrower rung of this edge's carrier, padded less,
reads a margin that clears the band was not measured.

The escalation is sound and typed. What it costs is that the answer
depends on which rung happens to prove first, and here on the member
order.

## What would answer it

Two readings, and choosing between them is a design question, since C2
has the ladder's one decision made "once, on the chosen radius":

- the ladder takes the widest rung whose margin **clears the band**,
  stepping to a narrower rung past one that proves inside it; or
- the cover is halved until each box's margin clears the band, which
  needs the band and the extent in the cover's inputs, and so in what a
  reader re-derives it from.

The row pins the pose as measured (`TUBE_IN_BAND`, then a body).
