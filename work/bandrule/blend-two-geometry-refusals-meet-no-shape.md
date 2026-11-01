---
id: blend-two-geometry-refusals-meet-no-shape
kind: issue
title: blend: an undecided rim-trim separation and a full-period split window refuse UnsupportedGeometry with the shape recourse
status: open
opened: 2026-10-14
priority: P3
cost: M
---


## Finding

Found by the sweep of `blend-refusals-whose-sentence-does-not-fit-their-site`.
Two `unbuilt_geometry` sites render `FILLET3_GEOMETRY_RECOURSE` ("the blend
reads only planes … whose edges are lines and circles") for a fact that
recourse does not address:

- `surgery.rs`, the support-boundary meter of a rim (`rim_above`): "a rim
  and its trim are not separated on their support" is raised when the
  bracket read `rim_at - at_trim` straddles zero — an undecided read, which
  the comment above it says cannot happen wherever the trim exists. It is
  either an `Escalated` margin or, if the comment's claim holds, a
  `SurgeryInvariant`; it is not a shape.
- `surgery.rs` `split_param_in_span`: "a split edge's stored window is not
  under one period" is met by a circle or ellipse edge whose window spans a
  whole period. For a circle, a carrier kind the recourse names as read,
  following the sentence changes nothing. For an ellipse the recourse does
  partly apply, since it names only lines and circles, but it misses the
  fact the site met: the window, not the kind.

## What the taker owes

Move each to the variant that states what it met (or give
`UnsupportedGeometry` a typed kind with a recourse per kind), and pin the
new text with a row.
