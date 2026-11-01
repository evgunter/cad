---
id: tilted-cut-scene-can-engrave-after-the-cut
kind: issue
title: The tour's tilted cut can now engrave after the cut and onto the section faces; the scene still engraves the cylinder first
status: open
opened: 2026-10-06
priority: P4
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

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P4: a demo tour's staging, not kernel behaviour: code improvement that is not architectural, P4 by `work/README.md` Priority.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
