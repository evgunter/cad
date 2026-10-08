---
id: sharing-the-carriers-centre-node-costs-four-iso-boundary-theorems
kind: issue
title: the pushforward sharing the carrier's centre node costs sym_thin_strip four pcurve_iso_boundary theorems (Theorem -> NumericZero, outcome unchanged)
status: closed
opened: 2026-09-25
closed: 2026-09-29
---


Found by PATHS' `geom-brep-sketch-segment-full-turn` (#3254), on the
coordinator's ruling that the loss be attributed before it is
re-baselined.

## What stands

`sweep`'s thread-count digest, `sym_thin_strip validate_geometric`
(the thin arc strip lofted at `Sym<f64>`, `mass_props_are_thread_count_invariant`),
at every ε row: `symbolic_zero` 26 → 22, `numeric` 634 → 638, and a
few frozen counts. The four are `pcurve_iso_boundary`'s domain-end test
(`geom-brep` `pcurve_cache.rs` `side_of`, reached from
`run_iso_arc_checks`), shape-report indices 304, 340, 484 and 520. All
four decide Zero both ways; they were theorems and are now numeric. The
early form is `c·stretch_u·?#9f8d2cc0`, where `#9f8d2cc0` is a FROZEN
`Sub(Lit 0, …)` node.

## Attribution (toggles, measured at a 4096-term budget)

- Putting the span back to `4·atan|b|`: still 22.
- Putting the apex back to the sagitta form: still 22.
- Rebuilding the description's centre and sweep in `sketch_segment` from
  the chord and the bulge: still 22. This produces the same node
  contents as the stored ones.
- Rebuilding the centre and the sweep **inside `SketchSegment::eval`**
  (the old evaluation): **26**. This toggle alone restores all four.

So the cause is where the pushforward's centre node is built, not its
content. With the node shared, nodes that the walk used to meet first at
decision 5 (`carrier_matches_mapped_source`) are met in another state
(`[unrecorded]` in that decision's explain tree). The frozen `0 − x`
node then reaches the iso-boundary test. Across the same change, the
pushforward residuals get simpler, as the plate's walk ledger in
`m10_sym_profile_interval` records.

## What is owed

A reading of why a node's frozen state depends on which walk first meets
it, and whether the early memo should re-attempt a node frozen under
another decision's walk. Rebuilding the centre inside `eval` would take
the four back, but at the cost of the unit's point: the pushforward
reads the stored carrier.

## A traversal-order dependence (from #3254's review, R1 S8): for DECIDE's triage

A decision's classification (theorem or numeric) depends on the state
in which the walk first meets a frozen `0 − x` node. With identical
node contents, only the order of construction differs. So the tier's
answer depends on traversal order, not only on the forms: two walks
over the same DAG, meeting its nodes in a different order, can classify
the same decision differently. D9 (determinism) holds, because the
order is fixed per build. Invariance under a re-ordering that leaves
the forms unchanged does not hold. Marked for DECIDE's triage: whether
a node frozen under one decision's walk should be re-attempted under
another's, and whether a row should pin order-invariance on a fixture.

## Closed by the merge onto main (2026-09-29)

Merged onto main at 48b767651, where tier 3's check 1 reads every edge
carrier's datums inside the session, the four come back: at every ε the
regenerated digest reads `sym_thin_strip validate_geometric` `sz=26
num=634`, main's own counts. What still moves against main is the
`frozen` column alone, down on both sym-session rows (`sym_arc_loft`
685 → 656 / 599 → 570 / 670 → 641, `sym_thin_strip` 860 → 803 /
825 → 768 / 858 → 801 at ε = 1e-6 / 1e-9 / 1e-12). The loss was a
traversal-order effect, as the triage note guessed, and the carrier pass
now interns the shared node before the domain-end test meets it. Nothing
is left to triage.
