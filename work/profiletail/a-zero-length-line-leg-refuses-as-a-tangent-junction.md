---
id: a-zero-length-line-leg-refuses-as-a-tangent-junction
kind: issue
title: A LineTo onto the tip itself is classified against a stand-in +x ray, so it refuses as a tangent junction (or passes) instead of naming the empty leg
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found by the chord-director fix
(`the-declared-tangency-bracket-refuses-its-closing-junction-at-interval`),
which kept this behaviour rather than change it.

A line leg's chord of no length names no ray. `path::chord_director`
(`crates/profile/src/path.rs`) stands in `+x` for it, and the departure
junction is then classified against that stand-in before anything asks
about the leg. The old `atan2(0, 0)` spelling did the same for a chord
of `(+0, ±0)`, and read `−x` for `(−0, ±0)` (`atan2(±0, −0) = ±π`); the
chord door reads `+x` for both, so after an incoming along −x the old
spelling refused `JunctionTangent` where the door now refuses
`JunctionCusp`, and the reverse after one along +x. Neither class names
the empty leg.

Measured at `f64`: `Open.at((0, 0)).angle(0).line(1).line_to((1, 0))`
refuses `JunctionTangent { margin: 0, arm: 1 }` — "a tangent departure",
for a leg that does not exist; after an incoming along −x it refuses
`JunctionCusp`; with any other incoming direction the junction passes and the empty segment is refused later, by shape
validation, under its own name.

The leg's length is decided by the lever gates and the segment shape
validation, so the chord door deliberately asks no band question of it
(a second decision on one length, and one that would pre-empt the
`SeamArrivalLeverTooShort` arm `bool12r2_probes::r2_a_degenerate_closing_leg_is_refused_for_want_of_a_lever`
pins). What is owed is a refusal at the door that names the empty leg
when the chord is the zero vector — the door already asks that, without
a band (`geom_core::is_exact_zero` of its largest component) — rather than a junction class read off a
stand-in.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILETAIL (`work/profiletail/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
