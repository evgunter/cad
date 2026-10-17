---
id: an-edge-lying-in-a-face-at-a-pierce-refuses-ring-meets-ring
kind: issue
title: A prism whose side face holds another's edge at a pierce refuses ResultInvalid RingMeetsRing in eight member orders
status: closed
opened: 2026-10-08
closed: 2026-10-13
priority: P2
cost: M
---

## What

Found by PR 4346's sweep (`three-solids-touching-along-one-line-refuse-their-union`).

Row: `crates/topo/tests/three_solids_on_one_line.rs`,
`a_prism_whose_face_holds_the_line_builds_or_refuses_typed`. The fixture
is the plate `[0,3]×[0,2]×[0,1]` (member 0), upright prisms over the
0°–50° (z 0.5–2.0, member 1) and 120°–170° (z 0.47–1.7, member 2) sectors
of radius 0.4 about (1.5, 1), and a third prism (z 0.44–1.81, member 3)
over the triangle from 0.3 at 80° and 260° to 0.4 at 350°. The third's
side face holds the vertical line through (1.5, 1), in no other face's
plane.

Eight left folds of `topo::union` refuse
`ResultInvalid { errors: [RingMeetsRing { .. }] }`:

- [2, 1, 3, 0], [3, 1, 2, 0], [1, 3, 2, 0] and [1, 2, 3, 0] at step 3;
- [3, 2, 0, 1] and [2, 3, 0, 1] at step 2;
- [3, 2, 1, 0] and [2, 3, 1, 0] at step 3, since
  `a-touching-vertex-beside-a-partner-along-the-face-refuses` read the
  pair that refused `VertexReadTwice` at their step 2; that step now
  builds sound (closed-form volume, tier 3, material probes).

Every refusing step is the one that adds the plate to a body holding
the third prism and the 120° prism. The first six refuse identically
on main (8fd03fce). Twelve orders build sound.

Unmeasured: which ring meets which, and whether the result's ring
placement or the classification before it is wrong. The tier-3 check
refusing the result is the fail-loud backstop. The row asserts the
refusal by order, and fails when an order builds.

## Closed (2026-10-13, TANG)

The ring that meets a ring is on the plate's top. With the third prism
and the 120° prism one operand, the 120° prism's edge on the line
pierces the top at (1.5, 1, 1). The third's side face crosses the top
along a section through that point. The reduction read the pierce
against the top alone, so the join chorded the third's section straight
past the pierce's ring vertex. The top was left with two rings, one
standing on the other's edge (`VertexOnEdge`). The classification was
right. The first wrong decision was the missing site on that section:
the pierce point lies inside a face of the piercing operand's own that
does not bound the pierce.

`vtxfac::faces_holding_pierce` now mints a ring vertex in each such
plane face at the point. Its one Out run is the face's half above the
pierced face, and it joins the pierce's ring as a second pierce. The
join chords the section to that vertex, and the zips fuse it with the
pierce through the ring's vertex. The top's section is one ring through
the point twice, as the orders that fold the plate earlier build it.
All eight orders are resolved. Six now build, with the same body as
every other building order: [2, 1, 3, 0], [3, 1, 2, 0], [1, 3, 2, 0],
[1, 2, 3, 0], [3, 2, 1, 0] and [2, 3, 1, 0]. The other two, [3, 2, 0, 1]
and [2, 3, 0, 1], now build step 2 and refuse step 3 typed: the 0°
prism's contact line holds the third's subdivision bisector
(`recl-unbuilt-contact-line-refusals-are-classification-invariants`).

A point inside a curved face of the piercing operand, one whose trim
cannot be read there, or a plane face holding it over a curved pierced
face, refuses `BooleanError::HeldPierceUnbuilt` before anything is built
(`a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt`).
