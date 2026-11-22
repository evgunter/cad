---
id: point-in-carrier-loop-is-a-second-copy-of-seg-ray-crossings
kind: issue
title: point_in_carrier_loop's ray x arc crossing is a second copy of profile::seg's ray_crossings (planar ray parity with arcs, two spellings)
status: parked
opened: 2026-09-25
priority: P2
cost: H
refs: [3204]
blocked_on: [both-planar-ray-walks-share-one-driver-beside-the-reader]
---

Found by the ATREST-9 dual review (PR #3204). Filed on PATHS because it
owns `crates/profile/src/seg.rs`, the first copy.

**Two implementations of one walk.** `profile::seg::ray_crossings`
(`seg.rs`, near the `ray_side`/`ray_advance` rows) counts an in-plane
ray's crossings of a profile loop's segments, arcs included: a line by
the endpoints' straddle, an arc by `carrier_line_circle` plus
`arc_span`, every tangency or endpoint hit a graze. ATREST-9 added a
second one on the B-rep side,
`topo::splitting::containment::point_in_carrier_loop` (with
`conic_crossings` and `ConicArc::in_window`): the straight edges through
`ray_walk::ray_crossings` (`ray_parity` before CLEAVE renamed it), each circle or ELLIPSE arc as a root of
`|P + D·s|² = 1` in the arc's unit coordinates inside a cosine window.
Same question, two spellings, different row names
(`ray_side`/`ray_advance` against `point_in_arc_loop_*`), different
discriminant forms, and the B-rep copy covers ellipses where the
profile one does not.

**Sibling filing.** ATREST-11's
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`
(on its branch at filing time) records the same shape for the
edge-pair intersector; the two rows are one consolidation question —
which crate homes the planar arc-aware primitives, and whether
`profile` (2-D, `Seg`) and `topo` (3-D, loop edges) can share them
through `ray_walk`'s `RaySpace` abstraction the way the polygon walk
already does.

**Not measured:** whether the two copies disagree anywhere. Both are
exact constructions with graze-on-ambiguity; a drift would show as a
different graze set, not a different answer.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Weighed (2026-11-21)

Three designers, two rounds, agreed (fork log row 299, the three-designer
table). This row's framing is corrected:

- **The home is already ruled.** `[one-way.readings]` puts carrier-pair
  readings in `geom-brep`, and C5 (`crates/geom-brep/README.md`) is the
  rule for how two spans meet. `topo` reads it elsewhere (the census,
  the boolean). So the copies are not "profile against topo, pick a
  home": `seg`'s reader and check 9's are private readings beside a
  ratified one. `profile` is not below `topo`; neither depends on the
  other.
- **Why the copies exist.** PR 6567 rules that two curves on one surface
  record never go to `meet` and meet where one crosses the other's
  plane. On a plane record two arcs both lie in the plane, so nothing
  ratified reads them, and each planar consumer wrote its own.
- **Stale in the body above.** Drift 4: check 9's arc trim is now
  `curve_pair::Span::place`. The circle-pair margins all read
  `geom_core::Circles`; what is spelled several times is the
  classifier. On the B-rep side the ray against a conic arc already
  reads the co-surface arm (`containment::conic_crossings`), so only
  `profile::seg::ray_crossings` is outside it. Profiles have full
  turns (`ArcSpan::FullTurn`).
- **Measured on main (699a1d2a8b)**, `meet` on planar pairs lifted to
  z = 0: a carrier clearance inside the band's gap escalates on
  `curve_pair_pose_extreme` whatever the spans, on five span-clear
  pairs profile validation accepts today; a tangent touch returns as a
  plain interior meeting; `curve_pair::Meeting` carries no kind.

The agreed final state: one planar meeting reader in `geom-brep`, a
plane instance of C5's co-surface arm entered through the table's
in-plane arms with coplanarity given by a type; a span keeps its ends
as given points; a span-clearance stage in the pose (a method on the
span's cover) proves "apart" before an in-band carrier reading
escalates; a meeting carries its kind. Profile's pair pass (with a new
`geom-brep` dependency), check 9's contact reading, the census's
plane-record pairs and both ray walks read it. `seg`'s pair and ray
readers and check 9's arms 4 and 5 are deleted once a differential of
`seg`, kept as the test oracle, agrees over profile's rejection rows,
pair-contact rows and reach fuzz. Not part of it: profile's joint pass;
the two fillet constructions' placement (LB4 and their own rows).

## Cut (2026-11-22)

Ev approved the plane clause (PR 7290). The build is PLANAR's
(`work/planar/plan.md`); this row moved there and is parked on
`both-planar-ray-walks-share-one-driver-beside-the-reader`, which
deletes `seg::ray_crossings` after the differential agrees and moves the
driver beside the reader.
