---
id: a-traced-faces-carriage-reads-its-reach-about-the-space-origin-not-the-frames
kind: issue
title: A traced face's carriage reads its reach about the space origin, not the frame's
status: closed
opened: 2026-11-22
priority: P1
cost: M
closed: 2026-11-22
---

Found by the FULL review of PR 7151 (MAJOR 2): `eval/wire/trace.rs` `drawn` carried a face chart over the traced faces' covers, read about the space origin. A placement reads its copy's reach about the copy's own origin, plus the slides and the datum.

## Closed

2026-11-22, in PR 7151, by Ev's ruling on [ev] #7353 (`docs/DESIGN.md` `[placement]`; the geom-core README, "Float rounding"). A traced face is read, not carried. The traced carriage and its reach are gone. The trace reads the faces through the turn selected before the loops resolve: `FaceChart::select`, its pole verdict alone over the session box's reach about the frame's origin, through `arm_of`, under `face_chart_selection_pole`. The profile's band is then decided over its loops.

Pinned by `face_chart_and_signed_depth::a_traced_profile_is_decided_as_its_placed_twin_is`, whose fixtures are the review's (block at x = 400 with the frame on it, and the frame 400 off its block), plus a small disc tracing a wide face. Each traced profile agrees with its placed twin.

