---
id: both-planar-ray-walks-share-one-driver-beside-the-reader
kind: unit
title: both planar ray walks share one driver beside the curve-pair reader, and seg::ray_crossings is deleted
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [the-seg-differential-agrees-with-the-plane-instance]
---


PLANAR's seventh unit; runs beside unit 6 once the differential agrees.

## The ruled text

C5 (`crates/geom-brep/README.md`): the co-surface arm's readers include
*"the planar loop walks' rays against a line, conic or spiric arc"*.
PR 7290: *"both planar ray walks (`topo::ray_walk`'s driver moves
beside the reader)"*; `profile::seg::ray_crossings` is deleted only
after the differential agrees.

## What changes (named in the tree, 2026-11-22)

- **The driver moves.** `topo::ray_walk::walk` (`crates/topo/src/ray_walk.rs`),
  with `RayFault` and `NoRaySettled`, moves to `geom-brep` beside the
  curve-pair reader. It already climbs `geom_brep::ladder::climb`; what
  ties it to `topo` is `crate::validate::decide` and `crate::rows`. The
  rest of `ray_walk.rs` (the polygon walk's `on_segment`,
  `ray_crossings`, `ray_verdict`, `apart_as`) stays in `topo` unless the
  lane finds it reads only the driver. About 67 `ray_walk::` uses in
  `topo` re-point (the lane counts them in the PR).
- **Profile's walk reads it.** `crates/profile/src/validate.rs`
  `point_in_loop` casts its `N_RAYS` rays through the driver; each
  segment against the ray is read through the instance; a crossing
  counts, a touch or an end hit sets the ray aside. Running out is the
  driver's one exhaustion sentence (`NoRaySettled`, recourse
  `geom_core::NO_DECLARATION_RECOURSE`), so
  `ProfileError::RayCastingExhausted` renders it.
- **Deleted**: `seg::ray_crossings`, `seg::Graze`, and with unit 6
  `line_span`/`arc_span` if nothing else reads them.

## Done when

- Profile's containment rows (nested loops, a hole touching its outer at
  a vertex, a ray through a vertex) and `topo`'s containment suites
  (`splitting::containment` `point_in_carrier_loop`) are green, the
  verdicts unchanged or each move listed.
- `RayCastingExhausted` renders the driver's sentence and recourse
  (PROFILE's `profile-ray-casting-exhaustion-is-a-copy-of-the-ray-walk`,
  claimed, `parent:` this unit).
- No ray-against-segment reading remains in `profile` (grep).

## First check, and what to do if it fails

**Which reading a ray takes.** `topo`'s walk reads a ray against a conic
arc through the co-surface arm's crossing door
(`splitting::containment::conic_crossings`: the ray as
`SurfaceCurve::u_line` on a `Charted::held(face, …)` plane, read by
`OnSurface::across`), keyed by the face's `SurfaceKey`. A sketch plane
has no `SurfaceKey`, and the instance's in-plane arms cover lines and
conics but not the spiric arc C5's list names. Whether both walks read
the instance (a ray as a line span), or both read `across` (and a sketch
plane gains a key), or each its own, is not settled by the ruling; it is
a question in the cut's PR. Stop until it is answered.

## Rows

Closes `point-in-carrier-loop-is-a-second-copy-of-seg-ray-crossings`
(moved here, parked on this unit) and
`profile-ray-casting-exhaustion-is-a-copy-of-the-ray-walk` (claimed).

## Territory

`crates/topo/src/ray_walk.rs`: no program (PLANAR's paths name it).
`crates/topo/src/splitting/containment.rs`: ONEHOME and eleven others;
announce on ONEHOME's log. `crates/profile/src/validate.rs`: PROFILE.
`crates/geom-brep/*`: CPAIR, ONECOPY.

## Review

**Single FULL**, difficulty **H**: the move is mechanical, the two walks'
graze rules must agree, and the refusal text moves for a user-visible
error.
