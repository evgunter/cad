---
id: a-glued-rod-escalates-a-section-straddle-at-1e-12-since-the-cylinder-node-listing
kind: issue
title: Main red at ε 1e-12: the rod glued in its bore escalates Coincidence(Section, Moot) on curve_carrier_straddle in every pose but one, both operations and both orders, since PR 6466
status: open
opened: 2026-11-14
priority: P2
cost: M
---



Filed by GREEN (`green/eps-1e-12`) from nightly 43371638371 (main
`e36675c1`, 2026-11-13); reproduced on main through `2c843a2e7`
(2026-11-14).

## What fails

`crates/sweep/tests/bisector_nodes.rs`
`a_rod_glued_in_its_bore_answers_alike_in_either_order` fails at
`CAD_TOLERANCE_EPS=1e-12` only. Of the row's 24 evaluations (six poses,
∩ and ∪, both operand orders), 20 escalate

    Coincidence(Section, Moot) Indeterminate { margin: Enclosure { .. },
      band: { zero: 1e-12, escalate: 1e-11 }, predicate: "curve_carrier_straddle" }

(`geom::curves::on_carrier`'s `straddle`). That is every pose but the rod
along `y` at `1e-9°`, in both operations and both orders, and the
enclosures differ between orders (for one, `[5.67e-16, 4.39e-6]`). The rod along `y` at `1e-9°` escalates
`Coincidence(SectorSide, Moot)` on `enters_material` instead, alike in
both orders, on a value margin (−3.47e-12).

- **The row stops there:** it compares margins as values, and an
  enclosure has none, so its own `expect` panics.
- **The pin** names the escalation (`Coincidence(Section, Moot)`,
  `curve_carrier_straddle`, `Enclosure`).
- **The probe**
  `bisector_nodes::at_1e_12_the_slanted_bores_refuse_every_cell_and_the_glued_rod_escalates_in_both_orders`
  asserts the escalation for the rod along `y` at `1e-8°`, ∩ and ∪, both
  orders.
- **Superseded doc claim:** the row's doc used to say that at 1e-12 both
  orders refuse at a pcurve certificate of the long bisector ellipse; no
  pose does on main.

## The merge

A first-parent bisect of main at 1e-12 names `c053873aa`, the merge of
PR 6466 (`sectarms/cylinder-node-listing`); its first parent `6775be78c`
passes. The same merge moved the 3° slanted bores at 1e-12
(`work/nearpoint/a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12.md`).

    CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
      -E 'test(a_rod_glued_in_its_bore_answers_alike_in_either_order)'

Related, not this row: the order dependence at the default ε
(`work/tangtail/a-glued-rod-answers-differently-by-order-off-the-pinned-angles.md`)
and the long ellipse's representability
(`work/germ/a-sliver-crossings-long-bisector-ellipse-is-served-past-representability.md`).

## Who cites this row

- the row's pin;
- the probe above.

## Done when

The row runs at 1e-12 to its comparison, both orders answering alike
with the straddle decided rather than the band loosened, and the pin
comes out.
