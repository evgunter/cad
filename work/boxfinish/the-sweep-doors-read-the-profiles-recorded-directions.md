---
id: the-sweep-doors-read-the-profiles-recorded-directions
kind: issue
title: The sweep doors read the profile's recorded directions where they derive one from a segment's ends, so the tour's stop and the cut plate certify
status: parked
opened: 2026-11-14
priority: P0
cost: M
blocked_on: [the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box]
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

- **A table's director is unit only at its chord's lever.** The profile
  accepts a written direction `d` where ‖o + ℓ·d − p‖ is within the
  band, which bounds its turn and `(|d| − 1)·ℓ`: check 1's measure for
  a `u_ref` at radius ℓ. Check 1 length-reads no plane, so a door that
  stores a table's director as a wall's normal stores one that may be
  off unit by band/ℓ (measured in the review: 3% on a 30ε side).
  Only fixtures and `into_loop` write such a table today. The door
  either takes its normal from the frame turning the director and says
  what reads its length, or this is RESTDATUM's
  `unlevered-frame-conventions-are-uncertified-at-rest`; say which.

- **A minted direction lies along an axis only where its zero is
  proven** (`boxfinish/profile-directions-revalue`, after the nightly's
  five `revalue_corpus` reds): validation mints the axis for a segment
  that recorded no direction only where a component is an identity of
  the parameters, never where it reads zero by coincidence. What each
  scalar stores:
  - `f64`: the quotient, which is the axis but for the sign of a zero:
    where one end of a side, or a radial's vertex against its centre,
    holds `−0.0` and the other `+0.0`, the zero component is the
    difference's own signed zero where PR 6963's axis stored `+0`
    (`at_f64_a_minted_direction_is_the_quotient_with_its_zeros_sign`).
  - plain `Interval`: nothing is proven either, so an axis-aligned
    literal rectangle's hop directors, exact `(0, ±1)` from PR 6963 to
    that fix, are ulp-wide again (`[0, 0]` over the length is exact;
    `y/√(y²)` carries rounding).
  - `Sym<Interval>`: a hop between two points that share a variable
    (the slab's `(W, 0) → (W, 1)`) keeps its exact axis in an eager
    session, over a box or a point; in a deferring session only at a
    point box (over a box it takes the quotient, with `W − W`'s width).
    One between two different variables that agree at the nominal takes
    the quotient with its width.
  - `Sym<f64>`: an identity mints its axis only where its value reads
    exactly zero; one whose value rounds takes the quotient
    (`work/boxfinish/the-axis-mint-asks-for-proof-only-where-the-value-reads-zero.md`).
  Verb-recorded directions are untouched. Nothing reads a direction
  yet; a door that reads a hop's director at plain `Interval` meets this
  rounding width.

**What this unit will not finish:** profiles whose directions are not
recorded (`fillet-side-and-centre-arcs-and-off-axis-hops-record-no-direction`).
Its closing tests say whether the tour's stop is among them.

**Carried from PR 6979's review (2026-11-15): a row that goes red on a
wrong side through the curved edge arm.** That PR sets a curved edge's
straddle aside over a box, and no test in it has power over the class:
with the rule's gates opened by mutation nothing reads a wrong side,
because every curved body buildable over a box today closes each cap on
a whole-turn circle that the loop walker refuses first
(`work/inside/a-whole-turn-circle-cap-refuses-corrupt-loop-over-a-box-where-f64-answers.md`).
This unit is the first to finish a curved body over a box through a
sweep door, with two-arc caps. Its closing tests include one that reads
a wrong side or a wrong containment answer under the old rule
(`Ok(Err(_)) => Passes::Met(..)` in `curved_edges::passes`) and under
`reaches_root_in_band` opened, on such a body. The searches PR 6979
adopts from its review are the tripwires to run first.


## Re-stated at dispatch (2026-11-15), on the tree with PRs 6972 and 6990

- **The sites are two functions.** PR 6972 funnelled every direction
  the doors derive from a segment's ends into `swept::sketch_direction`
  (a line's director; an arc's start radial, `(a − centre)` over its
  own length) and `swept::sketch_chord` (a line's director and length),
  read by four arms: `extrude::side_surface`'s line and arc arms and
  `swept::placed_segment_spec_about`'s line and arc arms, the second
  being every door's rim. `swept::SweptSeg` copies a validated
  segment's ends and kind and drops its directions, so `crates/sweep`
  reads a recorded direction nowhere. The older spellings this file and
  the extrude-mint row cite (`rim.normalize()`,
  `(q_to − q_from).normalize()`) are gone.
- **"The axis rule's two lanes, driven" describes PR 6963's rule.**
  After PR 6990 two different variables equal at the nominal take the
  quotient in every lane (the bullet after it, and audit row F43). What
  the door is driven for instead: a leaf the driver certifies holds
  every member of its box, on a document whose hop director is the
  quotient with a width (two variables equal at the nominal) and on one
  whose hop is an identity (the slab's `W − W`), in the lanes a drive
  runs in.
- **The mint and the verbs are not this unit's.** Which directions are
  minted where, and how the mint asks whether a component is zero, is
  `the-axis-mint-asks-for-proof-only-where-the-value-reads-zero`, with
  Ev on `[ev]` PR 6993. This unit reads what a validated segment
  holds and changes where it is held (`SegmentKind`'s arms), not how it
  is made.
- **The tour's blank is a polygon, its holes circles.** The plate's
  hole radials are recorded by the circle verb; its blank's directors
  are minted hop by hop. So whether the tour's certified stop and the
  cut plate certify after this unit also turns on what the mint stores
  in the session a drive runs in. The closing tests say which
  directions held them, by payload.
- **A wall that goes red by design.** `tolerance::cut_wall`
  (`demos/tour`) asserts the cut plate certifies no leaf over the
  study, citing
  `work/tally/a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound.md`.
  If the doors make it certify, the wall is re-stated with what was
  measured and that row gets the evidence.
- **The slab's row exists and is loose.**
  `a_sweep_keeps_the_frame_it_is_handed::a_slab_over_a_width_box_stores_its_fixed_faces_exact_and_holds_the_moving_walls`
  asserts only a rounding width on the three moving walls; this unit
  tightens it to exact where the profile's direction is exact.
