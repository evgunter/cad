---
id: ruled-cap-meter-reads-a-co-requested-open-link-at-its-stored-place
kind: issue
title: blend: the ruled cut-off's cap meter reads an open link requested in the same call at its stored place, not its trimline
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## Finding (by reading; not measured)

`ring_clearance_pass`'s arm (c) (`crates/sweep/src/blend/surgery.rs`)
meters every edge a convex ruled cut-off leaves on its transverse cap
against the region that encloses the sliver it removes. It reads each
edge at its STORED place (`stored_piece`). That edge can be an open
link requested in the same call. Its carve moves that piece of the
cap's boundary to its trimline, closer to the sliver, and the meter
reads the gap before the move.

Rings are not the gap: the predicate-3 argument in
`ruled-cap-meter-reads-a-co-requested-ring-at-its-stored-circle`
closes them. A trimline is a segment, though, and a segment inside the
cap can still cross the sliver. The candidate is a convex RULED link in
a cap cycle: a half-cylinder groove sunk into the D-rod's end cap,
parallel to the flat, its end walls being its own transverse caps, and
its near ruling requested with the creases.
- The cap edges at the D's crease vertex are unrequested, so none of
  them can be the co-requested edge. A plane–plane link in a cap ring
  ends at a mixed corner, which refuses.
- Predicate 2's screen keeps the groove's trimline off the flat and
  the wall only up to its sample stations.
- Near the corner, a trimline about `0.01·r` off the flat whose end
  lies about `0.8·r` from the wall lies beyond the cut-off arc (inside
  the sliver), while its stored ruling, one groove setback `< r`
  farther off the flat, clears the enclosure.

Found by PR 3822's review, sweeping the support-boundary meter's class.

## What the taker owes

Build the groove-in-the-cap-corner case and see whether it carves
wrong. If it does, read a co-requested edge in arm (c) through
`co_requested_trim`, which already answers where an edge bounds a face
after its own carve, and pin it with the row that reds without that.

## The planar cut-off shares the arm (2026-10-06)

The plane–plane band's cut-off (`blend/open/end_face.rs`, the
`band/plane-plane-band-cuts-off-at-its-end-face` branch) meters its
end face's other edges through the same arm (c), now fed by both
bands' `CapSliver`s, so the same stored-place read applies to a
planar link requested in the call whose support is that end face (a
box's top-front edge cut off at the left face while the left face's
back edge is also requested). Predicate 2's screen meters that pair
only when the two features do not touch.
