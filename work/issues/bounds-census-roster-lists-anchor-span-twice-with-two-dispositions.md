---
id: bounds-census-roster-lists-anchor-span-twice-with-two-dispositions
kind: issue
title: bounds_census's roster lists profile's anchor_span twice, HandedOff and Payload, and the census accepts both
status: open
opened: 2026-09-05
---


Reported by MSOLVE-1's implementer lane (PR 1929) while measuring an
inherited red on main; filed by the MSOLVE orchestrator, no owner
obvious (the census is `geom-core`'s test, the site is `profile`'s,
the disposition names Track V).

`crates/geom-core/tests/bounds_census.rs` carries two `Site` rows for
`crates/profile/src/path/arc_fillet.rs` / `anchor_span`: one
`HandedOff("Track V's, as map_refusal below; ...")` (added by
`020ad54d4`, the roster line that closed main's red) and one
`Payload("the corner-outcome PRESENTATION sort key ...")` (added by
`413f435a5`, FILLET-H7's fix pass, which closed the same red
independently). Two commits fixed one red in parallel and both landed;
the census is green with both rows, so it does not refuse a subject
with two dispositions. Two things to decide: which disposition is the
true one (they argue the same thing under different labels), and
whether a roster with one subject twice should lint red rather than
pass — a census that accepts contradictory rows is a weaker instrument
than its name says.
