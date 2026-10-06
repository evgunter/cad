---
id: tilted-cut-scene-can-engrave-after-the-cut
kind: issue
title: The tour's tilted cut can now engrave after the cut and onto the section faces; the scene still engraves the cylinder first
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found when `cleave/ray-walk` (the ray-walk driver unit) retired the
tour's two tilted-cut walls (`demos/tour/src/curvedcut.rs`, `walls`).
Both refused `Containment(VolumeUncertified)`: the point-in-solid door
refused the whole query when one schedule ray met nothing and the
props lane could not certify the half's volume, though other rays met
the boundary. That ray is now set aside.

- Wall 1, the C on the lower half's elliptical section face, builds:
  tier-3 valid, removing the glyph's area × depth inside the certified
  bracket, at all three ε rows.
- Wall 3, the C in the upper half's round cap after the cut (the
  natural order), builds and passes the same oracle.

Each is now a held check in `walls`, as the U on the upper half's
section face already was. The walls' own retire instructions ask for
more: move the lettering onto the section face (the oval nameplate),
and engrave the cap after the cut. Either changes what the stop builds
and renders (its `build`, the `stages` the pocket narration reads,
whose closed-form volume assertion holds only on the uncut cylinder),
so it needs a `[render]` pass and a look at the frames. That was left
out of the driver unit.
