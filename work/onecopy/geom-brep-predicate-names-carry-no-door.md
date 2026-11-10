---
id: geom-brep-predicate-names-carry-no-door
kind: issue
title: geom-brep's roster reads every name but says no door for any: topo's prefix-keyed door table has no geom-brep twin
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by ONECOPY's roster-reach unit (phase A.2 of `identity-and-fixture-names-outside-topos-roster`).

`crates/geom-brep/tests/recourse_roster.rs` reads every name geom-brep decides (324 and more) and lists every site it cannot read, but it says nothing about which door decides a name. topo's roster does: a name carries its door's prefix (`bool_`, `split_`, …), each door is typed on the error its escalations raise, and a name under none is an `EXCEPTION` with its door. geom-brep's names carry prefixes (`pcurve_`, `props_`, `ssi_`, `curve_pair_`, `section_`, `cs_`, `interval_`, …), but no table says which error type each prefix reports through, and some names carry none.

**Done** when geom-brep's roster holds a door table on topo's shape: prefixes declared once, each door typed on its error, exceptions listed with their door, and a prefix with no name red.
