---
id: two-rims-read-coplanar-refuse-unsupported-or-untyped
kind: issue
title: Two ellipse rims read coplanar refuse at frontiers — CurvedPierceUnsupported when definitely apart, FallbackExtentUnsupported when one curve within the band
status: closed
opened: 2026-10-14
priority: P1
cost: M
closed: 2026-10-30
pr: 5993
---

Found by the body pin added on `cleave/carrier-cross-one-curve`
(`crates/sweep/tests/a_ruling_lying_on_a_wall.rs`
`two_rims_the_band_reads_coplanar_are_one_curve_only_within_it`),
measured at ε = 1e-9.

Two radius-0.01 rods on one axis, cut below planes through the origin
of slopes 100 and 100(1 + k·ε). The rims cross at `(0, ±0.01, 0)`;
their far ends lie k·ε apart, and the planes' tilt carries 0.01·k·ε, so
the transverse row reads the planes parallel at every k below 100.

1. **k = 30 and 80: a definite reading refused as unsupported.**
   `carrier_cross.rs` `one_curve` reads Positive (the rims are two
   curves), `meetings` answers `None` (no coplanar conic pair is held),
   and all six ops refuse `CurvedPierceUnsupported { face: FaceKey(4v1),
   edge: EdgeKey(11v1) }`. The rims definitely cross, so under the
   governing rule this is a missing capability (the coplanar
   ellipse × ellipse meeting in `meetings`), not a band reading.
2. **k = 0.1, 0.5 and 0.9: one curve, then refused as unsupported.**
   `one_curve` reads Zero: within the band the rims are one curve,
   which is a definite reading, not an in-band one. All six ops then
   refuse `FallbackExtentUnsupported` ("tangent or near-tangent
   carriers and no crossing layer saw an event"). A refusal after a
   definite reading is the kernel's missing capability, a frontier
   (PR 4335's rule), never the user's lever: the fix is to build the
   coincident-cut case, not to escalate it.

The pin asserts both refusals exactly; the fix updates it.
3. **k = 80, the three ops with the first rod first: an escalation on
   `conic_bend`.** `t ∪ b`, `t ∖ b` and `t ∩ b` refuse
   `Escalated { decision: Coincidence(SectorSide, Moot) }` on
   `conic_bend` (`geom_brep::conic_bend::conic_bend_side`), at a margin
   of about `1e-19`: the bend does not clear the residue. It showed once
   `carrier_cross` became a caller of `geom_brep::curve_pair::meet`
   (`the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227), stage (c)). Before that, a
   boundary vertex on the far side of the rod was projected onto a seam
   ruling and split it at a point inside the run the ruling shares with
   the face's seam edge, and the op reached the `CurvedPierceUnsupported`
   frontier first. Whether the sector reading belongs in band at a rim
   80·ε from its partner is not yet measured.

## Slate triage (2026-10-26, CLEAVE)

- Banded P1: a definite reading followed by a missing lane (coplanar ellipse pair, coincident cut): verb breadth beyond the everyday shapes (a split or boolean that refuses, or ships a body its own gate fails, on a legal input), P1 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to SPLITDOOR (`work/splitdoor/`) by `git mv`, id and body unchanged: the P1 track opened for split and boolean shapes that refuse, or ship a side their own gate fails, where a uniform reading would build: the split door's tier-3′ bar, tangent and pinch poses, and the curved-carrier lanes that read lines and circles only. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Measured on `splitdoor/two-rims-coplanar` (2026-10-28)

At ε = 1e-6, 1e-9 and 1e-12, over k ∈ {0, 0.1, 0.5, 0.9, 2, 5, 30, 80}, each with its payload and raising site.
- **k < 1, the coincident cut (built).** At 1e-9 and 1e-12 every op refused `FallbackExtentUnsupported { face: FaceKey(4v1) }`. The site was `ops::section_extent_pass`, on the pair (A's wall, B's wall): the cylinder pair section `section_cylinder_pair_coincident` (R-tan), the pair the reduction settled one carrier. Nothing about ellipses is involved. A rod, a cut rod and a full torus against an identical copy refused the same way (k = 0), while a brick built. The fix defers such a pair to the fallback's `On` verdict, and every op builds: `∪` and `∩` the cut rod at `3π·0.01²` within its pad, tier 3 and 3′, and `∖` nothing. At 1e-6 the seam ruling's pierce refuses first, on the basin meter, k = 0 included.
- **k = 2, 5.** Every op refuses `Escalated { Crossing(OnEdge), predicate: "curve_carrier_gap" }`, margin ≈ k·ε, at every ε: in band, as pinned.
- **k = 30, 80.** The site is `reduce.rs`'s curved sweep arm. Its frontier closure is reached through `interior`, where `carrier_cross::boundary_crossing` answers `Unread` on `MeetFault::Unsettled(SignInBand)` from `curve_pair::pose::coplanar_ellipses`.
  - **The touch.** The rims share their minor axis and differ in their major semi-axis by about kε. In the common plane the level is `F ≈ −2δ·cos²θ`: a touch at the minor-axis tips, within the escalation of each other over `|cos θ| < 10/k`. The subdivision's first cut finds no definite sign there. This is the cpair unit's open touch (`work/cpair/two-coplanar-ellipses-have-no-meeting-method.md`, evidence added).
  - **The conflict.** The glue door decides every face pair a continuation, the two cut planes included: `6v3`↔`6v3` is decided Zero at a tilt of 0.01·kε. So the band reads the planes one carrier and the walls one carrier, while it reads the rims that those two carriers cut two curves. The plane meets the wall at a grazing angle of about 0.01, which carries the planes' 0.01·kε into a kε gap between their rims.
  - **How it is read.** Decided below: the glue is right, and the rims' meeting is in band.
  - **Past k = 80.** At k = 200 and 500 the ops escalate in band (`bool_vertex_face_side`, `curve_pair_pose_swing`). At k = 2000 and 10⁴ they refuse `CurvedPierceUnsupported` on `Unsettled::Meter`, the transverse arm's basin meter (`work/cpair/curve-pair-polish-meters-refuse-shallow-meetings.md`).
- **k = 80, first rod first.** The `conic_bend` margin is 9.87e-20 at 1e-9 and 9.87e-23 at 1e-12, which scales as ε·(the rims' 1e-11-rad crossing angle). At 1e-6 those ops reach the meter refusal first. The sector question is asked of the same two in-band rims, so it is part of the decision below and not fixed on its own.

## Decided (2026-10-29, Ev on PR 5973)

Both designers agreed, and Ev ruled (PR 5973, no DESIGN change):
- **The glue is right.** The merge keeps each edge's own curve and only re-describes it, so nothing moves past ε.
- **k < 1: one curve.** The rims are one curve within the band, and the ops build the coincident cut (PR 5969).
- **1 < k ≲ 1000: an in-band meeting.** The rims' 3-D gap is under ε over most of the rim and reaches kε only near the grazing tips, so the meeting is in band. Every op owes `Escalated` on where the rims part, not `CurvedPierceUnsupported`. Today the coplanar ellipse reading answers `Unsettled(SignInBand)` and the crossing layer keeps its frontier door. The fix is `work/splitdoor/roots-unsettled-folds-band-precision-and-fault-into-one-frontier.md`: `roots::Unsettled` flattens band, precision and fault into one frontier.
- **This item closes** when that row lands and this item's pin asserts `Escalated` at 1 < k ≲ 1000. That covers the k = 2, 5, 30 and 80 rows and the `conic_bend` escalation at 80.

## Closed (2026-10-30, PR 5993)

Ev's decision for 1 < k ≲ 1000 is now built at every pose but one. Two rims cut from two glued planes, meeting in band, refuse a located `CurvedPierceEscalated` whose decision is `CurveMeeting`, with the meeting's lever:
- k = 2 and 5: on `curve_carrier_gap`;
- k = 30: on `curve_pair_coplanar_sub_side`;
- k = 200: on `curve_pair_pose_swing`.

The exception is k = 80 with the second rod first. There the width arm's clear reads a **decided** Zero, so it keeps main's frontier (`CurvedPierceUnsupported`). That is the open question of `a-decided-zero-tangency-tells-the-frontier-story-where-recourse-iv-would-escalate`, and this pose is recorded there as its witness. k < 1 builds (PR 5969); 1e-6 refuses on the meter.
