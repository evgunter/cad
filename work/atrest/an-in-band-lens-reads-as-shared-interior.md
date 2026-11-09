---
id: an-in-band-lens-reads-as-shared-interior
kind: issue
title: A lens of non-circular arcs shallower than the zero may read as shared interior: the witness search takes crossings the meeting reader parts within the band, unmeasured since circular lenses read touching
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by the holdout review of `aligned-arc-or-curved-faces-that-only-abut-are-never-read-flush`
(both reviewers, executed at three ε), pinned in `census.rs`
`arc_interiors::a_lens_across_the_band_is_never_read_apart`.

## What

Two aligned arc-bounded faces whose arcs cross twice bound a lens.
`census.rs` `face_interiors_meet` runs the witness search
(`shared_interior_witness`) before the reading of pieces
(`arc_interiors_meet`). On arcs of radius 20, 100 and 1000 escalation
widths, turned off the axes and the origin, at ε = 1e-6, 1e-9 and 1e-12
alike:

| lens depth | witness search (`face_interiors_meet`) | pieces |
|---|---|---|
| 0.05ε | `Ok(true)` (20 widths: unread) | unread |
| 0.3ε, 0.9ε | `Ok(true)` | unread |
| 1.1ε … 9ε | unread | unread |
| 11ε, 50ε | `Ok(true)` | `Ok(true)` |

The reviewers saw the same at 10 … 1e5 widths, 0.05–0.9ε, and at 100
widths 0.056–0.22·zero. At 20 widths it is not monotone: 0.28·zero reads
`true` while 1.13·zero reads unread.

So a lens shallower than the zero, which the band says is no overlap,
reads as shared interior: `SameSideInterference` between two copies,
a false interference. It errs loud, never a false clear.

The likely path: `census/curved.rs` `interior_meeting` returns the first
meeting `curve_pair::meet` decided before it refused ("a decided one
standing whatever refusal followed it"). The stretch rule refuses two
crossings with nothing definitely apart between them, but the witness
search takes the first crossing alone and reads its cones at first
order (`cones_meet`), where they share a sector.

## What closing it takes

The witness search reads a crossing only from a meeting set the reader
did not refuse, or reads the lens at second order. Either way the row's
known `Ok(true)` below the zero turns to unread or `Ok(false)`, and the
row is turned with it.

## Evidence from `onecopy/transverse-range` (2026-11-09)

The orchestrator ruled that an extreme of a first harmonic in the zero band reads its touch at the extreme, by C5 (`curve_pair::pose`'s module docs, "A first harmonic's roots"). Since then, two coplanar circular arcs bounding a lens shallower than the zero meet in one touch, and both readings read them touching (`Ok(false)`) at 0.05ε, 0.3ε and 0.9ε on arcs of 20, 100 and 1000 widths. `census.rs` `arc_interiors::a_lens_across_the_band_is_never_read_apart` now pins that. The witness search still reads as shared interior whatever two crossings the meeting reader parts within the band. Lenses of non-circular arcs, which the coplanar subdivisions read, were not measured.

## Narrowed (PR 6638)

The known `Ok(true)` this row's `Owed` waited on is gone for circular arcs. A lens of two circular arcs shallower than the zero is one touch, and both readings read it touching at every radius and depth measured above. `arc_interiors::a_lens_across_the_band_is_never_read_apart` now pins that. `curve_pair::order_rows::two_circles_overlapping_by_a_lens_read_it_by_its_depth` pins the meeting reader's branches beneath it: one touch below the zero, an escalation in the gap, and two crossings past it. What remains is unmeasured: lenses of non-circular arcs, which the coplanar subdivisions read, and whether the witness search reads as shared interior any two crossings the meeting reader still parts within the band.
