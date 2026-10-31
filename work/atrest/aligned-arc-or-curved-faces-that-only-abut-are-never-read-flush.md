---
id: aligned-arc-or-curved-faces-that-only-abut-are-never-read-flush
kind: issue
title: Aligned faces with an arc or on a curved carrier that only abut are never read flush: the same-side reading finds meetings and certifies none
status: closed
opened: 2026-10-27
priority: P1
cost: M
refs: [curved-copies-filling-one-volume-read-as-undecidable]
closed: 2026-10-27
branch: atrest/aligned-arc-or-curved-faces-that-abut-are-flush
pr: 5884
---


Found by `curved-copies-filling-one-volume-read-as-undecidable` (refs).

## What

Two faces of two copies on one carrier, facing the same way, whose
boundaries meet while their interiors do not, are a flush pair: a row
of its own, no contact (`[assertions]`). The census reads that row only
for two planar faces bounded by lines. `census.rs`
`face_interiors_meet` certifies "the interiors do not meet" only through
`planar_interiors_meet`, whose argument (every meeting shows at a vertex
inside, a proper crossing or a shared cone) needs a polygon. A face with
an arc, or a face on a cylinder or a sphere, goes to
`shared_interior_witness`. That search can find that two interiors meet,
and never certifies that they do not. So:

- two half-discs side by side on one plane, sharing a diameter, are
  no flush row. `flush_at_rest` carries the region door's refusal of
  the arc (`CensusUnsupported`, `NonPlanarTrim`). An arc-bounded pair
  whose trims the region walk refuses at a shared boundary is
  `FLUSH_UNREAD` (`MayOverlap`);
- two half-cylinders of one carrier side by side, sharing only their
  seam rulings, are arm 1's `CurvedWithinReach` refusal
  (`sweep_cross_solid_backstop`), loud and undecidable. Pinned by
  `crates/sweep/tests/curved_copies_filling_one_volume.rs`
  `copies_that_abut_along_a_rim_or_a_seam_are_no_interference`.

## What closing it takes

A certificate of disjoint interiors for an arc-bounded or curved trim
pair on one carrier. One way is the polygon argument with arcs: every
edge pair's meetings read, each decided transverse (shared interior) or
tangent with its second order read. Another is the region door
answering `Empty` on the arc and curved trims.

## Closed

2026-10-27. The polygon argument with arcs, posed as pieces
(`census.rs` `arc_interiors_meet`). Each edge of either face is cut at
every meeting with the other face's boundary: each point meeting of an
edge pair (`curve_pair::meet`, complete or refusing) and each of the
other face's vertices on it. Each piece's midpoint is placed in the
other face. No piece inside and no shared run with both interiors on
one side certifies the interiors apart; the argument is the function's
rustdoc. `face_interiors_meet` runs it where the witness search finds
no meeting, so an aligned arc-bounded disc pair or a cylinder or sphere
pair on one carrier (Door 1 `Definite`) is read either way:

- `flush_at_rest` records the pair flush through one loop over planes,
  cylinders and spheres: the shared run is read by the one meeting
  reading for every edge kind (so a rod's end disc flush with a tube's
  end annulus is a row too), and every path it does not decide is a
  finding;
- the curved aligned arm hands the census's arm 1 the pairs certified
  apart, so the seam-sharing half-cylinders' walls are no longer
  `CurvedWithinReach`. Cylinder faces whose axial spans meet at most at
  a point are apart without the reading.

A lens shallower than the zero between two crossing arcs still reads
as shared interior through the witness search, which answers first:
`an-in-band-lens-reads-as-shared-interior`.

What stays unread is loud as before: Door 1 in band, other curved
kinds, a meeting reader that refuses, and the rim pairs (two carriers,
`curved-copies-filling-one-volume-read-as-undecidable`).
