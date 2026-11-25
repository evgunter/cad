---
id: the-side-readers-ray-rides-the-body-it-reads-over-a-box
kind: unit
title: The side reader's ray rides the body it reads: over a box it is a family of rays, and every reading subtracts two stored copies of one moving quantity
status: dispatched
opened: 2026-11-24
priority: P1
cost: H
branch: boxfinish/fixed-ray
---


Weighed by two designers (2026-11-24; the record is on
`analysis/design-fork/the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence`,
the reports on `boxfinish/briefs` under
`probe/briefs/fork-side-reading/`), who agreed after one round. It
changes no ratified text, so it was ruled by the orchestrator and not
put to Ev. It carries three rows that stand before the switch's
deletion: `the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence`,
`a-walls-aim-hangs-from-its-moving-edge-so-its-ray-reads-a-box-a-tenth-as-wide`,
`a-moving-face-whose-normal-has-no-zero-component-reads-its-side-only-over-a-narrow-box`.

## The defect

`topo`'s side reader builds each aimed ray from the body's own stored
values at the lane scalar (`side_reader::loop_aim`: a point of the
face's first edge, stepped into the face). Over a parameter box the
ray is therefore a family of rays that rides with the body, and every
reading measures one stored value against another that carries the
same moving quantity: the aim against the face's plane, against
another edge's line, against a vertex. An enclosure does not record
that the two are one number, so a reading that is one number at every
member comes out as wide as the box and the ray is set aside. The
three rows are that one subtraction at three sites; two more rows are
the same defect
(`the-ray-walks-curved-and-fallback-readings-subtract-an-aim-from-a-carriers-own-origin`,
`an-aimed-rays-own-direction-is-read-on-a-selected-frame`).

Nothing consumes the ray: it is scratch, and any ray from outside
reads the same side. DESIGN `[scalars]` makes a continuous value that
depends on none of the document's variables a choice, not a claim, and
`geom-core`'s DL5.2 says a place the code chose to evaluate at is
never a `T`. The reader builds the ray as if it were a claim.

## The final state

1. **The ray is a place.** Its aim and direction are plain numbers,
   chosen as today and read off the middle of their enclosures, in a
   type that cannot hold a boxed value. Per scalar: at `f64` the ray
   it is today, bit for bit; at `Interval` and `Sym<Interval>` one
   line for every member, each reading enclosing every member's
   reading of that line; at a dual, the base value's line.
2. **Each reading takes one stored thing against the line.** A plane's
   advance reads the plane against the exact line; its width is then
   the members' real difference.
3. **Whether the ray crosses a planar face is the turn of the face's
   loops seen along the ray**, about the ray's own point: no crossing
   point is formed, so each vertex's enclosure enters once.
4. **A crossing definitely behind the first counted one stops
   nothing** ("definitely behind" is past the first count by more than
   the band, since crossings in band of each other are summed).
   Separable from 1 to 3; its soundness wants its own rows.
5. What sets a ray aside is otherwise unchanged.

Every caller of the reader gets the same reader and the same
guarantee, with no certificate needed: check 7 and check 10 in the
finish, the standalone tier-3 doors, the role reads during
construction (the pieces sort, the census's role gate, shell
classification), and point containment's side for a ray that crosses
nothing.

**Not built here:** reading the side at one member of the box and
carrying it by the embedding certificate. Both designers hold it as a
sound second layer for check 7 alone, wanted only if a body's
embedding certifies over a box that no fixed ray reads. M14 (the probe
lane) measures whether any measured body is of that kind; building it
adds a sentence to DESIGN `[topology.tiers]` and so would go to Ev.

## Done when

- At `Sym<Interval>` the door's unit slab reads its side by rays at
  `1 ± 0.45` and the tilted bricks
  (`side_reader_boxed_brick_rows::tilted_cap_rows`) at widths the
  riding ray does not reach, every read the members' side at members
  across the box. The widths are measured, not set here: M14's hand
  collapse (below) is the floor, and each stop above it is held to a
  real cause.
- Each ray reads up to the width at which a member's crossings truly
  change and no further (a wall's ray aimed 0.19 under a top edge that
  moves 0.25 does not read for every member, correctly), with a row
  per body holding the stop to that cause.
- No side moves at `f64`: aims, rays, advances and edge gaps bit for
  bit, the turn's decisions the same over the suites and the tour, and
  HONEJOIN's tie census unmoved.
- The three rows' closing tests are restated as the designers gave
  them: the advance row's retires (the own crossing is truly as wide
  as the box); the wall row's becomes "the body reads at ± 0.25"; the
  tilted row's stands with its turn read along the ray.
- A family that turns inside out within its box is refused, not read.
(BOXFINISH orchestrator)

## M14 (the probe lane, 2026-11-24)

`probe/M14-REPORT.md` on `boxfinish/probe-side-reading` at
`9de1e63216`. The ray's aim and direction collapsed by hand to the
middle of their enclosures and nothing else changed, at
`Sym<Interval>`, widths ± 0.05, 0.1, 0.25, 0.45.

- **Sound:** no boxed reading in any run is the wrong side, per aim or
  per reader (100 `f64` members checked).
- **It reads further:** the slab's top cap 0.25 to 0.45, its walls
  0.05 to 0.1; a tilted brick's own answer from 0.05 to 0.1 (to 0.25
  on the gentler tilt with the turn read along the ray). Per aim it
  trades the edge-gap stop for an advance or a winding stop one or two
  widths later.
- **Reading the turn along the ray is not uniformly a gain by
  itself:** the lined brick's bottom cap drops from 0.45 to 0.25, and
  one wall of the steeper tilt loses a width. The collapse leaves the
  advance and the other readings as they are, so this is a floor and a
  warning, not the design's reach.
- **Plain `Interval` builds none of these bodies** at any width (the
  extrude and the fixture's strut refuse at an edge's endpoint
  certification, enclosure `[0, 2w]`), so no side reading is reached
  there. The "Done when" above no longer asks for it.
- **Tier 3′'s embedding census by itself** passes the axis-aligned
  bodies at every width where edges carry stated lines, and is
  undecided on a tilted cap from ± 0.05
  (`the-embedding-census-is-undecided-on-a-tilted-moving-cap`). So a
  tilted body's finish does not pass over a box whatever the side
  reader does, and the one-member layer has nothing to offer there.
- **The second layer's condition** (embedding certifies where no fixed
  ray reads) is met by one fixture at one width under the hand
  collapse (the brick with no edge lines at ± 0.25, stopped on an
  advance of `[-15.8, 3.46]`), which is the reading the unit's point 2
  re-poses. It is re-asked on the built unit, not decided on the
  collapse.
(BOXFINISH orchestrator)
