---
id: check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector
kind: issue
title: tier 3 check 9's arm 5 is a second copy of profile::seg's planar edge-pair intersector, with at least four drifts, and the circle-pair margins have a third spelling in arc_fillet
status: open
opened: 2026-09-25
priority: P1
cost: H
refs: [ATREST-11]
---


Found by the ATREST-11 review (PR #3217). Filed on PATHS because it
owns `crates/profile/src/seg.rs`, the first copy.

**Two implementations of one logic.** `profile::seg` has a planar
edge-pair intersector over its own `Seg`/`ArcGeom` types: `line_line`,
`line_arc`, `arc_arc` and `push_arc_arc_contact`, with trims tested by
`line_span` and `arc_span`. Tier 3's check 9 now carries a second one,
arm 5 of the ring-vs-outer contact half: `validate::segments_meet`,
`lines_meet`, `meet_at` and `window` over `validate::MeetSegment`. The
circle-pair margins are spelled three times:
- `validate::circle_pair` (`ring_outer_circles_apart` / `_nested`);
- seg's `carrier_circles_external` / `_internal`;
- `profile/src/path/arc_fillet.rs`'s `circle_circle`.

**Drifts between the two intersectors, as of ATREST-11:**
1. **Cocircular carriers.** seg decides `carrier_circles_identity` first
   and resolves span overlap on the shared carrier (`arc_apex_identity`).
   Arm 5 returns apart on concentric equal circles
   (`ring_outer_meet_centres`) and relies on check 9's vertex arms
   (arm 2 in both directions, and arm 1) to have reported any overlap.
2. **Tangency within the band.** seg's `line_arc` `Zero` branch tests the
   foot only. Arm 5 also tests both half-chord ends, which can be
   `√(2rε)` apart.
3. **Existence escalation.** seg propagates an in-band existence margin
   (`?`) whatever the trims say. Arm 5 rules a candidate out when it is
   definitely past either trim, and escalates only otherwise.
4. **Arc trim.** seg's `arc_span` is a one-apex chordal defect. Its
   conditioning falls as `cos(θ/4)` towards a full circle, bounded by
   the `arc_diameter_clearance` gate. Arm 5's `window` decides the
   direct distance to each end first, then a two-apex sum
   (`cos(w/4) + sin(w/4) ≥ 1`) that is valid up to and including a
   whole circle, which topo loops have and profiles do not. `window`'s
   two-term margin is the better conditioned of the two: the ATREST-11
   review proved by hand that its derivative per unit arc length is
   `−(cos(|φ|/2) + sin(|φ|/2))`, magnitude at least 1 everywhere, so it
   never compresses below the arc length it measures. seg's single
   term compresses as `cos(θ/4)` and leans on its construction gate to
   bound the compression. A shared home should take the two-term form.
5. **Classification.** seg separates `Touch`, `Tangency` and `Crossing`.
   Arm 5 only answers meet / apart / unsure.

**Why this is not fixed where it was found.** `profile` sits below `topo`,
and seg's intersector is crate-private over profile's own 2-D types. One
home means moving a shared planar edge-pair intersector down a layer
(into `geom-core` or `geom-brep`, over a carrier-plus-trim type both
crates can build), and that crosses programs. The reason for doing it:
the two copies have already drifted in five places, and each drift is a
place where a profile the sketch door accepts could be refused at rest,
or the other way round.
