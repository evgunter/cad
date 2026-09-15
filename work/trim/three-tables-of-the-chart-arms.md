---
id: three-tables-of-the-chart-arms
kind: issue
title: Three tables of the chart metring arms in three crates, two of them copies
status: open
opened: 2026-09-14
refs: [clearance-window-tightening-needs-chart-boundary]
---



## What

The metring arms of a chart — metres per chart unit — are answered in
three places, and two of them are copies of the first:

- `topo::chart_region`'s `certified_arms` is the ONE home, and
  `docs/TRIM-3-SPEC.md` §2 (deleted at PR-2's merge; recoverable per `docs/DOC-LEDGER.md`) cites it as the authority for why a plane's
  `(1, 1)` and a cylinder's `(r, 1)` need no bound while every other
  carrier takes a certified inf with a gate. It is **private to that
  module**.
- `editor_core::clearance`'s `chart_arms` (TRIM-3 PR-2) spells those
  two arms character for character, and answers `None` for everything
  else. The reason for the copy is written at the site: the original is
  private and `chart_region.rs` is Track Q's, read-only for that unit.
- `geom_brep::chart_arms_at` is one word off in the name and answers
  the SUP direction, which is a different question with a similar
  shape.

## Why it matters

The hazard is DRIFT, not disagreement — the two tables agree today. A
carrier that gains a certified arm in `certified_arms` gains nothing in
`clearance`, and the clearance consumer's own residue
(`clearance-window-cone-sphere-torus`) predicts exactly that move: "one
more arm in `chart_arms`". Whoever lands it has to know there are two
tables, and nothing in either file says so except the paragraph PR-2
added at the copy site.

## Fix shape

Make `certified_arms`' two exact arms reachable — a `pub` door in
`topo` returning the exact pair for the carriers that have one, `None`
otherwise, with the gated inf staying private to `chart_region` — and
delete `clearance::chart_arms`. Small, and it is a `topo` edit, which
is why PR-2 could not take it: the fence ruled that file read-only for
the unit. Rename `geom_brep::chart_arms_at` to say `sup` while
someone is there.

## Home

TRIM filed it from the copy site; the edit is `topo`'s and
`chart_region.rs` is Track Q's ground.
