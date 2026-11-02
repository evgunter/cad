---
id: an-aligned-in-band-pair-with-a-curved-edge-is-never-read-thin
kind: issue
title: An aligned in-band face pair with a curved edge is never read thin: thin_overlap answers false for any face that is not a polygon
status: open
opened: 2026-10-28
priority: P3
cost: M
---

Found by the sweep for `!polygon` gives-ups on
`restread/pierce-span-curved-edge`
(`the-pierce-span-clearance-gives-up-on-a-face-with-a-curved-edge`).

## What

`census.rs` `thin_overlap` opens with
`if !(fa.polygon && fb.polygon) { return false; }`: two planar faces
standing off in band on near one plane, facing the same way, are read
`ThinInBand` (`aligned_unread`) only where both are bounded by lines.
A pair either of which has an arc reads `MayOverlap` however far apart
their projections lie. Both are `Undecidable`, so no body is admitted
or refused differently; the reason is, and an assertion that quiets a
thin pair cannot quiet this one.

Its argument (a direction square to a side along which the two faces'
vertex intervals overlap by no more than the band) rests on the
vertices bounding the face, which an arc's bulge breaks.

## Fix

Read an arc's extent along the direction rather than its vertices': a
circle's or an ellipse's extent along a direction in its plane is
closed-form, and a curved edge's span-bounded box
(`boolean::boxes::edge_box`) bounds it soundly from outside.

## Evidence

No measured body: no topo or sweep test is known to reach an aligned
in-band pair with an arc.
