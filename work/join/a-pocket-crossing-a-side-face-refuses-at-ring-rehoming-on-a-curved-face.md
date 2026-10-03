---
id: a-pocket-crossing-a-side-face-refuses-at-ring-rehoming-on-a-curved-face
kind: issue
title: A D pocket whose wall crosses the block's side face refuses SectionInvariant at ring re-homing on a curved face, an ordinary pocket in join-internal words
status: open
opened: 2026-10-03
priority: P1
cost: M
refs: [JOIN-3, blind-d-pocket-subtract-refuses-with-join-internal-words]
---


## Measured

JOIN-3's dual review (R2, `crates/sweep/tests/join3_r2_probes.rs`,
`j3r2_pocket_battery`, ignored) and its fix pass, release, main against
the fix-pass head. The block `[−1, 1]² × [0, 1]` minus a D prism (the
`rod_chord_at` D, turned and offset) whose wall crosses the block's side
face, e.g. D0.3 turned 4.0 rad, offset (0.8, 0.1), entering through the
top: 156 poses refuse

`Join(SectionInvariant { what: "ring re-homing reads the divided face's
plane; this face's carrier is not a plane (arm not wired)" })`

raised by `chord_join::face_plane_normal`, which `ChordJoiner::rehome_rings`
reads when a first `mef` divides a CURVED face that still owns
rings. On main the same poses refuse `JoinDesync "ring-run winding is
degenerate"` (132) or `SectionArcWindow { NoChartedRun }` (24): the
segment curve took them one door further, to a curved face's ring
re-homing, which has no arm.

## Why it matters

An ordinary blind pocket refuses in join-internal words: the carried
class of `blind-d-pocket-subtract-refuses-with-join-internal-words`.
The re-homing needs a chart-side test for a ring on a curved face (the
D's wall carries the pocket's rings once its section crosses the block's
side), or a typed frontier refusal naming the face.
