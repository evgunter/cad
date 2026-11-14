---
id: a-glued-rod-escalates-a-section-straddle-at-1e-12-since-the-cylinder-node-listing
kind: issue
title: Main red at ε 1e-12: the rod glued in its bore escalates Coincidence(Section, Moot) on curve_carrier_straddle since PR 6466, where its row reads both orders refusing at a pcurve certificate
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by GREEN (`green/eps-1e-12`) from nightly 43371638371 (main
`e36675c1`, 2026-11-13); reproduces on main `2c843a2e7` (2026-11-14).

## What fails

`crates/sweep/tests/bisector_nodes.rs`
`a_rod_glued_in_its_bore_answers_alike_in_either_order` at
`CAD_TOLERANCE_EPS=1e-12` only. The row's doc says that at 1e-12 both
operand orders refuse at a pcurve certificate of the long bisector
ellipse. On main one operation instead escalates

    Coincidence(Section, Moot) Indeterminate { margin: Enclosure { lo: 5.67e-16, hi: 4.39e-6 },
      band: { zero: 1e-12, escalate: 1e-11 }, predicate: "curve_carrier_straddle" }

(`geom::curves::on_carrier`'s `straddle`), after four
`Coincidence(SectorSide, Moot)` escalations on `enters_material`
(margin −3.47e-12) that the row reads as before. The margin is an
enclosure, which the row's comparison cannot read, so it stops there;
the row is pinned at 1e-12 on that stop and goes red the day it moves.
Which order and operation raise it is unmeasured.

## The merge

A first-parent bisect of main at 1e-12 names `c053873aa`, the merge of
PR 6466 (`sectarms/cylinder-node-listing`); its first parent `6775be78c`
passes. The same merge moved
`bisector_nodes::slanted_bores_build_every_operation_one_difference_pinched_to_itself`
at 1e-12 (`work/nearpoint/a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12.md`).

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(a_rod_glued_in_its_bore_answers_alike_in_either_order)'

Related, not this row: the order-dependence at default ε
(`work/tangtail/a-glued-rod-answers-differently-by-order-off-the-pinned-angles.md`)
and the long ellipse's representability
(`work/germ/a-sliver-crossings-long-bisector-ellipse-is-served-past-representability.md`).

## Done when

The row runs at 1e-12 to its comparison: both orders refuse alike, or
build alike, with the section's straddle decided rather than the band
loosened, and the pin comes out.
