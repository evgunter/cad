---
id: aligned-arc-or-curved-faces-that-only-abut-are-never-read-flush
kind: issue
title: Aligned faces with an arc or on a curved carrier that only abut are never read flush: the same-side reading finds meetings and certifies none
status: open
opened: 2026-10-27
priority: P1
cost: M
refs: [curved-copies-filling-one-volume-read-as-undecidable]
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
  `flush_at_rest`'s `FLUSH_UNREAD` finding (`MayOverlap`), not a flush
  row;
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
