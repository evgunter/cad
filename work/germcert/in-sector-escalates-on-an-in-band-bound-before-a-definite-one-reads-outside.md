---
id: in-sector-escalates-on-an-in-band-bound-before-a-definite-one-reads-outside
kind: issue
title: in_sector escalates on an in-band bound reading before a later reading decides the direction outside
status: open
opened: 2026-10-13
priority: P3
---



## What

`crates/topo/src/boolean/sectors.rs` `in_sector` answers `false` on the
first reading decided Negative (`bool_cone_within` ×2, then
`bool_cone_facing`). It returns an in-band reading through `?` before a
later reading can decide the direction outside, so the sector escalates
`Coincidence(Sectors)` where a definite answer stands. The function's own
doc already notes an order dependence ("at a zero flip its answer can
depend on which of two sectors over one plane is read first, as an
in-band reading may").

The identity ladder's rule (any definite reading answers; an in-band one
refuses only where none does) has one home since PR 4918:
`topo::boolean::carrier_eq::identity_data`. A conjunction of "outside"
readings is the same shape with the signs read the other way.

Found by PR 4918's second-review sweep (2026-10-13). Not measured.
