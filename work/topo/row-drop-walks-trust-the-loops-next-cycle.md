---
id: row-drop-walks-trust-the-loops-next-cycle
kind: issue
title: drop_loop_rows, drop_face_rows and the site-row plans take a loop's rows from its next walk, which no plan proves claims the loop: a torn next drops or re-mints another loop's rows
status: open
opened: 2026-09-30
refs: [mef-and-mekr-move-a-walked-run-they-never-prove-is-the-loops]
---

## What

Found by the second pass of the walk-proofs unit (PR 3511), which made
every plan that moves or removes half-edges by a loop walk prove the
walk claims the loop, and then looked at the walks that decide which
pcurve rows a door drops or re-mints.

- `Body::drop_loop_rows` (`crates/topo/src/euler_ring.rs`), called by
  `mfkrh`, `kfmrh` and `ring_move` through
  `drop_rows_on_chart_change`, and `Body::drop_face_rows`, take a
  loop's rows from `pcurves::loop_rows`, which walks `next` from
  `first` and reads no `parent_loop`, in the mutation phase.
- The site-row plans (`Body::site_cycle` and `site_cycle_from` in
  `crates/topo/src/euler.rs`, read by `mev_fan_site`, `mef_chords`'
  rows plan and `mekr`'s target side) walk the same way, and the plan
  re-mints rows for every half-edge the walk lists.

A torn `next` diverted through another loop makes the door drop, or
re-mint in this face's chart, rows of that loop's members; one closed
past a member leaves that member's row on the chart the loop left.
Neither is a tier-1 fault, so the pcurve pass rather than tier 1 would
see it. Not measured.

## The shape to give

Each walk proves its members claim the loop (the shape of
`Body::require_run_of`) in its plan, and the mutation-phase drops take
their member list from the plan instead of walking again.
