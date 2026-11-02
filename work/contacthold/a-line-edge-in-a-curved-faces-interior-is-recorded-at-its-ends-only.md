---
id: a-line-edge-in-a-curved-faces-interior-is-recorded-at-its-ends-only
kind: issue
title: A line edge lying in a curved face's interior is recorded by its ends' vertex-on-face rests alone; D1's edge-face record is absent
status: parked
opened: 2026-10-11
priority: P2
cost: M
blocked_on: [an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane]
refs: [4650]
---


Measured on PR 4650 (FLUSH, `flush/corner-ruling-tier3`). Raised by its
review r2.

## What

`docs/DESIGN.md` D1: "Every pair whose interiors meet is recorded, an
edge lying in a face or along an edge included". In wall `∖` box of
`sweep/tests/a_ruling_through_a_box_corner_edge.rs` the box's corner
edge, a line, lies in the interior of the near wall piece (a cylinder
face): the result's two wedges of wall material touch along it. The
result records the edge's two ends as vertex-on-face rests on that
piece and nothing for the edge itself. `a_ruling_lying_on_a_wall.rs`'s
`t ∖ b` (a prism edge resting on a tube's wall) records the same way.
The census does not miss it, because same-solid curved pairs are the
constructor's obligation (`census.rs` module docs).

The test pins the record count as it stands, with this row named as
the gap.

## Why it waits

An edge-face record off a line and a plane has no census lane to
confirm it: `census::confirm_edge_face` refuses it typed
`CensusUnsupported(ContactLane(NotCertifiable))` away from a
conventional vertex (RESTREAD's
`an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane`).
Minting the record now turns a correct body into a refusal at the
result gate.

## Owed

When that lane lands: the carry records the edge-face pair (the edge's
two end rests imply it: one group's cells, an edge and the face its ends
rest in), and the two tests' counts move by one, named in their PR.

## 2026-10-11 — the planar case is minted (FLUSH)

The reduction sweep's `boolean::reduce::settle_in_plane` records each
piece of a line edge lying in a plane face that the face holds (an end
recorded inside it, else its midpoint inside). It skips a curved face
by design: a line between two points of a curved face need not lie in
it. When the curve-touch lane lands, this row's step is that
function's face test widened to a curved face whose carrier holds the
line (the sweep's `LyingIn::Curved`), not a new producer.

## 2026-10-19 — two tubes tangent along a seam (CONTACTHOLD)

`sweep/tests/two_tubes_in_the_zero_band.rs`: a tube's seam ruling lies
in the other tube's wall along their tangent line. ∪ records the two
rim circles' touches on the seam as vertex–edge records and nothing for
the seam's span in the wall, alike in both operand orders; the rows pin
the count at two.
