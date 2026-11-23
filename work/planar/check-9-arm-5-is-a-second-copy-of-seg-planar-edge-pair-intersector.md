---
id: check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector
kind: issue
title: tier 3 check 9's arm 5 is a second copy of profile::seg's planar edge-pair intersector, with at least four drifts, and the circle-pair margins have a third spelling in arc_fillet
status: parked
opened: 2026-09-25
priority: P1
cost: H
refs: [3217]
blocked_on: [check-9-reads-ring-against-outer-through-the-plane-instance, profile-validation-reads-its-pairs-through-the-plane-instance]
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

## Drift 6 (2026-10-21, PR 5552)

**Radial span certificates.** Where a carrier clearance reads in band,
seg now asks whether either segment certainly keeps off the other's
whole circle before it escalates:
- `segment_clear_of_circle`: the centre's foot past the line's span,
  and both ends outside the circle;
- `arc_clear_of_circle`: the arc's span past its point nearest the other
  centre with both ends outside, or past the farthest with both ends
  inside.

Each margin is charged, on `circle_foot_past_segment`,
`circle_extreme_past_arc` and `circle_end_clear`. Arm 5 has no such
reading: it carries the in-band existence reading forward and windows
the foot and chord ends against both trims (`meet_at`). Whether it
escalates where the spans are radially clear was not measured. A shared
home should take the certificates with their charges.

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
(`work/planar/plan.md`), so this row moved there and is parked on the
two units that delete the copies: check 9's arms 4 and 5 go in
`check-9-reads-ring-against-outer-through-the-plane-instance`, and
`seg`'s pair reader in
`profile-validation-reads-its-pairs-through-the-plane-instance`. It
closes when both land. The circle-pair margins' third spelling in
`arc_fillet` is out of the ruling (LB4 and its own rows).

## The ellipse's pair rows ride this build (2026-11-22)

The ovoid's cut (PR 7289, `work/ovoid/an-ovoid-cannot-be-authored.md`
"Cut") puts profile validation's line × ellipse, circle × ellipse and
ellipse × ellipse rows on this build's reader rather than on new `seg`
rows: the table already routes them (`curve_pair`'s `Arm::LineConic`,
`EllipseCircle`/`CircleEllipse`, `EllipseEllipse`), and
`curve_pair/pose.rs`'s `line_conic` and `coplanar_ellipses` were not
traced by this build's designers. OVOID's
`profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`
and SPLINEWALL's `a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it`
are parked on the unit that moves profile's pair pass onto the reader,
`profile-validation-reads-its-pairs-through-the-plane-instance`. The plane
instance's in-plane arms should cover the conic arms too, so the
designers' three measured defects on `meet` (an in-band carrier
clearance escalating whatever the spans, a tangent touch read as an
interior meeting, no kind on a meeting) are fixed for ellipses as for
circles.
