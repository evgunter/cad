---
id: a-held-pierces-charge-zero-refuses-with-no-second-order-reading
kind: issue
title: A held pierce whose face leaves the pierced face within the charge's Zero refuses CurvedSectorSideUnsupported with no second-order reading
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Filed by GERM's sector-order2 lane (2026-11-20), from its sweep for the
class of
`work/germ/a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side.md`.

## What

`vtxfac::faces_holding_pierce` (`crates/topo/src/boolean/vtxfac.rs`,
the `bool_held_pierce_curved` read) charges a held pierce's run with
`sectors::sagitta_charge(|n × n_pierced|, lever, arm)` and refuses
`BooleanError::CurvedSectorSideUnsupported` on any verdict
`Refused::of` returns, a Zero included. That is the shape the sector
bound's reading had before the GERM fix: a first order decided, a
charge decided Zero, and a typed refusal where the side is second order.
The GERM fix ties a sector bound on its charge's Zero and reads it by
its jet (`bound_side::TiedBy::Charge`, `Tie::descend`); the held pierce
has no bound to tie (its run is the face's half above the pierced face
from the germ `n × n_pierced`), so its second order is the two faces'
relative bend along their section tangent, the reading
`sectors::tangent_relative_side` takes for a declared-`Tangent` pair,
which nothing wires in here.

No row is known to reach it: a held pierce with a face a hair from
tangent to a curved pierced face. A probe would place a plane face
holding a pierce point at a slope inside `2·sqrt(zero/R)` of a
cylinder wall.

## Done when

A held pierce whose charge is decided Zero is read by the faces'
second-order side, or refuses on a margin that reading names, and a
row pins it.
