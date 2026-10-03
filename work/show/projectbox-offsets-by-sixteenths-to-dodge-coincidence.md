---
id: projectbox-offsets-by-sixteenths-to-dodge-coincidence
kind: issue
title: projectbox sinks each boss 1/16 into the floor to dodge a cylinder-cap-on-floor contact the declared door could state
status: closed
opened: 2026-10-02
priority: P3
cost: M
closed: 2026-10-03
pr: 3899
---

## What

Found in the sweep for `letterforms-flush-declared`, which retired the
letterforms' 1/16 decoupling and with it the module doc that stated
"#91's design rule" (operands never share coincident planes).
`demos/tour/src/projectbox.rs` still cites that rule.

Read against PR 3811 (open; it reworks projectbox: the bosses become
round cylinders and the pilots through-bores), the one coincidence
dodge left is the bosses' 1/16 sink INTO the floor: `BOSS_Z.0 = 0.1875`
against the floor top at 0.25, commented there as "flush contact would
refuse". A boss standing on the floor is a cylinder CAP resting on the
floor plane — a planar contact the declared door states
(`booleans::flush_declarations` into `pncad::topo::union_with`;
`Doc.declare_all` in Python). The vent and cavity overshoots are
ordinary through-cuts, not dodges.

`heatsink`'s 1/16 fin overlap is the same shape and is already tracked
(`heatsink-placedunion-base-union-unfinished`, #1344); `twopeg` cites
the rule only as the reason a dodge was removed.

## Do

Once #3811 lands: stand each boss on the floor (`BOSS_Z.0` = the floor
top) and union it with the cap-on-floor contact declared; retire the
citation of the rule in the module doc, the caption and
`TestProjectbox`'s docstring (`crates/pncad-py/tests/test_north_star.py`).
Measure the chain first and pin any refusal with `walls::wall`.

## Closed

By #3899. The bosses stand on the floor and each union declares the
flush detector's findings. Six of those are continuations between
disjoint boss tops that the union does not need:
`work/tang/flush-detector-offers-disjoint-coplanar-pairs-as-continuations.md`.
The drifted copy in crates/sweep is
`work/tint/split-cylindrical-feature-box-copy-of-projectbox-has-drifted.md`.
