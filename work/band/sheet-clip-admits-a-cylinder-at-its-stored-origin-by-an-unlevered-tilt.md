---
id: sheet-clip-admits-a-cylinder-at-its-stored-origin-by-an-unlevered-tilt
kind: issue
title: sheet_clip admits a cylinder by its stored origin's offset and an unlevered tilt gate
status: closed
opened: 2026-10-07
priority: P2
cost: M
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
closed: 2026-10-14
pr: 5052
branch: band/sheet-clip-levered-cylinder
---


## What

`sheet_clip` (`crates/sweep/src/blend/reach.rs`) classes a surface as
`Kind::Cylinder` when `near(off(*origin)) && near(tilt(*axis))`: the
cylinder's STORED origin (any point of its axis) measured off the band
torus's canonical axis `(o, a)`, and a tilt gate `tilt.hi() <=
band.zero()` with no lever. A tilt in the zero band moves the origin's
offset by the tilt times how far along the axis the origin is stored,
and nothing levers the tilt at the extent the clip is consumed over.
Whether a wrong `Kind::Cylinder` can make the sheet enclosure unsound
(rather than only returning `None`) decides the priority; read, not
probed.

The fix's shape is `cylinder-offsets-read-at-a-stored-origin-off-the-reach`'s
(PR 4118 and its sibling): read the offset at the foot of the consumed
region on the axis, and decide the tilt through a named margin levered
at that region's extent from the foot.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep.

## Closed

Probed: a wrong `Kind::Cylinder` cannot make the clip unsound. Its
`Clip::Z` is the boundary's span, and along a cylinder's ruling `z` is
affine, so that span holds the face at every pose of the axis, ⊥
included. The stored-origin gate's only harm was a lost clip: a coaxial
wall stored 3 m along at half the zero band's lean returned `None`.
`sheet_clip` now clips a cylinder or a plane at any pose and reads no
origin. `sheet_fn` reads a plane's and a cylinder's slack at the foot of
the reach box's centre, and `tilt`'s chord no longer cancels to zero in
f64. Rows: `crates/sweep/src/blend/reach.rs`'s
`the_clip_reads_one_span_at_every_stored_origin`,
`a_clip_holds_its_face_at_every_pose`,
`a_sheet_slack_reads_one_value_at_every_stored_origin`,
`a_sheet_slack_encloses_the_distance`, `tilt_reads_the_chord`. The
sweep's hit list is in PR 5052; nothing filed.
