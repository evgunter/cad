---
id: the-sweep-doors-read-the-profiles-recorded-directions
kind: issue
title: The sweep doors read the profile's recorded directions where they derive one from a segment's ends, so the tour's stop and the cut plate certify
status: parked
opened: 2026-11-14
priority: P0
cost: M
blocked_on: [the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box, the-profile-drops-the-unit-directions-its-verbs-made]
---


Split from `the-profile-drops-the-unit-directions-its-verbs-made`
(2026-11-14), which keeps the `profile` half: a validated segment
records its unit directions. This row is the doors' half, on the tree
that has both that and `boxfinish/sweep-frames`
(`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`).

**The unit.** Each site in `crates/sweep` that derives a 2-D direction
from a segment's ends (an arc wall's and a rim circle's `u_ref`, a
line wall's normal, a line rim's director) reads the direction the
profile recorded.

**Known to stand behind the exact direction** (the profile lane's
measure-first, on the probe with `u_ref` substituted by hand): the
refusal moves to the rim carrier's certification,
`carrier_endpoint_start [0, 2.97e-3]`. The circle verb stores the
radius as |r| while its vertex is c + r·x, so `c + |r|·x − v` is
|r| − r, which is no polynomial; `rim.normalize()` got past it only
because ‖rim‖ = √(r²) folds to |r|. The profile half registers the
identity "centre + radius·radial = vertex" (`Arc2::register_radials`);
the door owes the same of its placed carrier's start.

**The tests that say it is closed.**
- On main, with the switch as it is: the tour's plate with its holes
  subtracted, over the tour's study box, no longer refuses on a
  `u_ref` (walls or rim circles) at its first `Subtract`. What it does
  next is measured and, if something else holds it, filed.
- With the switch's box arm off: the tour's certified stop certifies
  its share (`demo-tour certified`,
  `eps_regression::certified_cells_run_green_at_default_eps`).
- A width-boxed slab's moving wall stores an exact normal.

## Added from PR 6963's review (2026-11-14)

The profile half's review (approve-with-fixes, no MAJOR) leaves three
things to this unit, because each needs a door that reads a direction.

- **The directions move into `SegmentKind`'s arms.** PR 6963 holds
  `ValidatedSegment.directions` beside `kind` as a second enum, so a
  line with radials can be written and four sites handle that
  impossible pair. This unit rewrites every door's match anyway: each
  door binds the direction in the arm it reads (`Line { director }`,
  `Arc { arc, turn, start, end }`), and the parallel field goes.
- **The axis rule's two lanes, driven.** Where two different variables
  are equal at the nominal, the `f64` witness stores the axis and the
  box lane the quotient's enclosure (audit row F43). The review held
  that sound by inspection and did not build it. Build it through a
  door: the leaf the driver certifies is true of every member of the
  box.
- **The door owes the placed start's rigidity.** Reading the recorded
  radial, the placed carrier's start no longer contains the sketch
  node the profile's registration names;
  `swept::register_placed_carrier_end` builds its sketch end through
  `Arc2::carrier_end`, which divides by the rim, and should read the
  radial too.

**What this unit will not finish:** profiles whose directions are not
recorded (`fillet-side-and-centre-arcs-and-off-axis-hops-record-no-direction`).
Its closing tests say whether the tour's stop is among them.
