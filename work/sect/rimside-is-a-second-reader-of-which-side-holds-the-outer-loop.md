---
id: rimside-is-a-second-reader-of-which-side-holds-the-outer-loop
kind: issue
title: RimSide answers which side of a one-site loop holds the face's outer loop beside path_island_winding, which answers it for ring islands on every kind; the sphere arm reads the conic's plane, valid only for plane sections
status: open
opened: 2026-11-20
priority: P1
cost: M
refs: [a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm, a-seamless-band-mints-and-passes-tier-3-though-loopwraps-says-no-constructor-makes-one]
---


Filed by SECT's orchestrator, 2026-11-20, from two Opus designers'
weighing of the closed-edge row (two rounds; they converged). Banded P1:
two implementations of one logic.

## What

"Which side of a closed loop lying in a face holds the face's outer
loop" is answered twice in `topo`:

- for a one-site section loop (`boolean/join.rs` `closes_at_its_site`,
  `chord_join` `join_lone_ring`, `place_strut`) by `chord_join::RimSide`,
  a per-kind table: the plane by loop winding, the sphere by the conic's
  plane (cap by plane), and nothing on a cylinder, cone or torus;
- for a ring-lane island by `chord_join::path_island_winding`, a
  chart-free path parity (a path on the surface from a point of the outer
  loop to the closing chord, crossings counted, arrival side read against
  `N × t`), which reads plane, cylinder, sphere, cone, torus and spline
  faces. Its contract is `holds_outer`'s.

The kind table stands for homotopy class: every closed conic on a plane or
sphere bounds a disc in the surface (null), every closed conic on a
cylinder, cone or torus winds round it (essential). An at-rest face is one
disc of its chart, so an essential loop meets a seam (a `Wrap` site, or a
crossing the crossing layer splits) and never reaches a pierce cell.

The sphere arm's premise (the loop is a plane section) is narrower than
the sphere: a wall whose transverse section with a ball is not planar
(e.g. a tilted tube crossing its own one seam inside a ball face) would
reach (Wrap, Pierce(Sphere)) and refuse `SingleSiteSectionLoop` (designer's
prediction, not measured).

## The shape to give (designers agreed; orchestrator's call where they
differed)

- `RimSide` retires: a pierce or null strut is sided by the island
  reader (plane by its planar winding inside that reader, as the ring
  lane already does). `closes_at_its_site`'s `rim(..).is_some()` /
  `== Some(RimSide::Plane)` tests become "the reader reads this face";
  `place_strut`'s plane-only guard goes; the wrap-against-pierce rows
  widen to every face the reader reads (pin the sphere case above).
- Adapter: `IslandReading::closed_by` needs a constructor for a loop
  that is one whole closed edge (empty run; arrival point on the edge
  away from its vertex). A face with no boundary reference point refuses
  typed.
- An essential loop reaching a pierce cell is a `JoinDesync`-class
  invariant failure naming that it winds (the section value's
  `SectionComponent.winds` carries the fact), not `SingleSiteSectionLoop`.
  It holds once tier 3 refuses a winding loop at rest
  (`a-seamless-band-mints-…`, PCERT); until then a seamless band would
  fail loud there, which is acceptable. (One designer preferred only a
  documented premise; the orchestrator chose fail-loud.)

## Done when

One reader answers the question for both lanes, `RimSide` is gone, the
sphere non-plane-section pierce is pinned building, and the essential
pierce is an invariant with a row that reaches it (or a stated reason
none can).
