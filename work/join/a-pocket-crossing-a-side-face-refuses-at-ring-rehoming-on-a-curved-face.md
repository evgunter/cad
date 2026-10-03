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

## Built

Ring re-homing was not the cause. TANG's PR 3851 wired
`chart_ring_side`, the chart arm this row asked for, before the fix
started: on main the 222 poses that refused `SectionInvariant` at
`face_plane_normal` at the filing commit (the 156 this row measured plus
their `through` siblings) reach the chart arm and refuse
`RingHomingAmbiguous` (198) or build sound (24). The ring sat
`OnBoundary` because the chord that walled the run off was minted to the
wrong partner: `boolean::join`'s `find_match` ranked partner sites by
CHORD LENGTH, which on a conic germ line grows to the half-turn and
shrinks again, so the site a major arc away read nearer than the next
site along the section and the chord crossed it. Nearness is the germ's
own half-turn first (`germ_arm`), then the chord within it (`nearer`).

A second cause behind it: the planar side selects its chord arc against
the partner WALL face's azimuth window, read from the face the germ was
recorded against at insertion, which an earlier segment's `mef` may have
divided — the window then covers a part of the wall the arc does not lie
in and neither candidate is contained. `wall_region` reads the region
that owns the segment's halves at call time.

All 222 poses build sound at tier 2, tier 3′, the certificate and
`assert_legal_operand`, at their closed-form volume
(`crates/sweep/tests/pocket_wall_crossing_a_side_face.rs`, the disc
against the block's square clipped by the D's flat).
