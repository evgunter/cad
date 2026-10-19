---
id: an-in-band-axis-parallelism-offers-a-proximity-lever
kind: issue
title: An in-band reading of a cylinder pair's axis parallelism offers the user a proximity lever
status: closed
opened: 2026-10-13
priority: P3
cost: E
refs: [crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync]
branch: cleave/join-face-surface-one-home
closed: 2026-10-14
---


Found in review of PR 4863 (`cleave/crossing-cylinders-join`).

## What

When `cc_axes_parallel` reads a cylinder pair's axes in the band
(`join::cylinder_pair_axes`, read by the frame dispatch and by
`segment_arm`), the refusal is `frame_refusal`'s
`Escalated { Coincidence(Section, Moot) }`. Its ending
(`refusal_routes`, `Coincide::Section => Ending::Lever(MEET)`) tells the
user to "move the parts so they clearly meet or clearly stand apart
there". That is a translation lever, but the reading is an angle: the
sine between the axes, levered at the walls' span. Walls that plainly
cross along two rulings get the advice to move apart, when the input
to change is the tilt between the axes, which should be made clearly
zero or clearly not.

A public door reaches it:
`parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_at_the_walls_span`.
Every op of the tipped pose ends there. The frame dispatch's own in-band
reading of the same row ends there too
(`frame_dispatch_tests::the_germ_frame_levers_a_cylinder_pair_at_its_walls_span`).

## Direction

Give the parallelism reading a decision of its own, with a lever that
names the axes' angle. One way is a `Coincide` variant, or a
`BooleanDecision` beside `WallRoots(AxisParallel)`, whose ending
already speaks of axes. The section predicates that are distances can
keep `MEET`.

## Built (branch cleave/join-face-surface-one-home)

- `BooleanDecision::CylinderAxes`, subject "whether the axes of a
  cylinder face of each solid are parallel", ending
  `Ending::Lever("turn one part so the two cylinders' axes are clearly
  parallel, or clearly not")`. `cylinder_pair_axes` raises
  `FrameError::AxesEscalated`, which `frame_refusal` refuses as it.
  No existing lever reads this tilt: the nearest are
  `WallRoots(AxisParallel)` ("turn the edge clearly away from the
  direction of the cylinder's axis", an edge against an axis) and the
  merge's "turn one of the two faces", so the verb is theirs.
- Measured on main at ε = 1e-9, 1e-6 and 1e-12: the seams pose tipped
  `1.5·zero/3.2` refused `Coincidence(Section, Moot)` with the proximity
  lever in every op. Following the new lever: toward parallel
  (`0.5·zero/3.7`, `0.9·zero/3.7`) builds SOUND at all three ε; away
  (`1.1·esc/3.7`, `3·esc/3.7`) ends at in-band distance readings
  (`VertexOnCurvedFace`, `Containment`) and further (1e-3, 0.1 rad) at
  `CurvedSectorSideUnsupported` / `GermFrameUnsupported` (or the
  `PierceCurvature` escalation at 1e-6), all typed, none a defect.
- Pinned at the door: `seams_lying_on_the_walls_read_the_axes_at_the_walls_span`
  (two tilt axes; the decision, the predicate and the ending's wording;
  the parallel side builds, 0.1 rad ends typed) and
  `one_face_walls_read_their_axes_as_an_angle`. The site census in
  `offer_rows` carries the new site; `every_decision` and so the word
  budget cover the new decision. Both tests are in the slow set.
- Sweep sibling filed: `the-germ-frames-in-band-angle-rungs-offer-a-proximity-lever`.

### Fix pass (review of PR 5061)

- The lever names the passing side alone: "turn one part so the two
  cylinders' axes are clearly parallel". A definite tilt never builds on
  these poses (the review's 240 runs; this branch's 0.1–0.5 rad at three
  ε), so offering "clearly not" offered a refusal.
- The door pin's turned arm asserts the exact frontier at 0.2 rad at every
  ε: about `x`, `GermFrameUnsupported`; about `y`, `CurvedPierceUnsupported`.
  (0.1 rad about `x` at ε = 1e-6 ends at the `PierceCurvature` escalation
  instead, so the pin tips further.)
- One spelling: `CylinderAxes` and the carrier ladder's
  `carrier_cyl_axis_parallel` both read "whether two cylinders' axes are
  parallel".
- `FrameError` carries one `Escalated { decision, diag }`; the section,
  axes and radius readings differ only in the decision they carry.
- The ε-sized margins at visible tilts: filed as
  `tipped-seams-pose-reads-eps-scaled-margins-at-visible-tilts`.
