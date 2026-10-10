---
id: wedge-classes-reads-a-corner-flat-at-its-short-bounds-as-convex
kind: issue
title: wedge_classes reads a corner whose reflex edge is flat within the band at its short bounds as convex, and classes long edges wrong far out of band
status: dispatched
opened: 2026-10-08
priority: P1
cost: M
branch: cleave/wedge-classes-flat
---

Filed by PR 4289's fourth review (m1). Pre-existing: main (8d11c126)
reads every one of the witness's readings the same way.

## What

`crates/topo/src/boolean/sectors.rs` `wedge_classes` decides whether a
corner of three faces or more is convex by reading each bound against
every other face's plane (`plane_side_code`, the bound at its own
reach). A bound read `On` changes neither flag (`Ok(SideCode::On) => {}`
in the convexity loop), so a corner whose only reflex edge is flat
within the zero band at its bounds' reach reads as convex. The convex
branch then classes each partner edge by its side of every face's
plane, at that edge's own reach.

Where the bounds are short and the edges read against the corner are
long, that reading is wrong far outside the band: the reflex dent `g`
is within the band at 1 mm, but at 50 m or 1 km the region between the
reflex-adjacent planes is metres wide, and the convex reading puts it
on the wrong side. A silent wrong class: naming takes the rows as given.

## Witness

PR 4289's review-4 probes (`tang/non-convex-cone-review4-probes` @
31580144, `review4/dart_long.py`): a dart of four faces over the ring
`(1,0,0), (0,1,g), (−1,0,0), (0,−1,0)`, bounds 1 mm and 1 cm long,
`g = ±1e-5, ±1e-6, ±3e-7`, material or hollow, read by line edges 1 m,
50 m and 1 km long a hair above or below the flat.
- At `g = ±1e-6` and `±3e-7` with 1 mm bounds, `wedge_classes` reads
  133 classes the exact oracle contradicts, 678 to 233 000 ε from any
  boundary.
- Read directly, the polygon-cone reader (`cone_side`) reads those
  directions `None` or refuses. Inside `wedge_classes` it is never
  asked, because the convexity loop has already decided the corner is
  convex.

`sectors::cone_fuzz`'s long-probe family (`dart_long`) reads this
corner every run; its `wedge_classes` column there is counted as known
wrong against this item and not asserted.

## The shape to give

A bound read `On` against another face's plane leaves the corner's
convexity undecided, not convex: route such a corner to `cone_read`
(which reads it at each probe's own reach, and passes over what it
cannot decide), or refuse. Measure which cells of the suite move first:
every flat-within-the-band corner of three faces takes that branch
today.
