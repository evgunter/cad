---
id: a-far-meeting-point-fails-membership-by-its-own-rounding
kind: issue
title: A near-parallel pair's far meeting point fails its own membership by the rounding of its coordinates — a Contradictory or an escalation for a pair that meets
status: parked
opened: 2026-10-01
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---



## What

Found by MSOLVE-12's census (`crates/editor-core/tests/msolve12_honest_translation.rs`,
`c1_every_separated_pair_measures_what_it_refuses`). Since MSOLVE-12
the translation stage (`crates/editor-core/src/mate/coset.rs`,
`candidate_translation`) solves two planes the table separates at the
edge of its band to their true meeting line: a finite pose, the
offset over the sine away. At the edge that sine is `K·ε / arm`, so
an offset `h` puts the pose about `h · arm / (K·ε)` out: at the
witness ε = 1e-9 (K·ε = 1e-8), `h = 1 m` and `arm = 3.7 m` meet some
1e7–1e8 m away. The membership check (`member_of`) then decides
`mate_member_translation_in_plane` on the candidate's distance from the
added plane, and at that magnitude the coordinates' own rounding,
about `2.2e-16 · |t|`, exceeds the zero band. Measured, with
`n₁ = (0.3, 0.2, 1)/‖·‖`, tilt shape `(0.6, 0.8)`, offsets along
`(0.01, 0.02, 0.03)/‖·‖`:

| arm | offset | outcome |
| --- | --- | --- |
| 3.7 m | 1 m | `Clash { Length 1.49e-8 m }` |
| 123.456 m | 0.03 m | `Clash { Length 2.98e-8 m }` |
| 123.456 m | 1 m | `Clash { Length 4.77e-7 m }` |
| 1e6 m | 1 m | `Clash { Length -3.9e-3 m }` |
| 1 m | 1 m (`n₁ = (1,1,1)/√3`) | `Indeterminate` under `mate_member_translation_in_plane`, a valid in-band margin |

With `n₁ = ẑ` the same pairs solve: the rounding falls along a
direction the dot product with `n₂` scales by the sine. The refusal is
`MateFault::Contradictory` (or an escalation with
`COINCIDENCE_RECOURSE`) for two planes that do meet, so the cause the
user reads is false; the true one is that the meeting point lies past
the range in which the session's ε is resolvable (DESIGN D4: ε ≈ 1e-9 m
covers micron to kilometre with ~4 orders of headroom).

## Why it is not MSOLVE-12's

The predicate measured a real residual of a real candidate; nothing is
non-finite and no margin is `Invalid`, which is what MSOLVE-12's A2
and C1 bound. Refusing it under the range's cause needs a statement of
the session's range — a bound on a coordinate's magnitude at which ε is
still resolvable — and no door in `geom-core` states one today
(`RANGE_RECOURSE` is spoken only where a quantity overflows or
underflows the format). `MateFault::PoseOutOfRange` is the arm that
would carry it.

The rotation stage has the same blind spot at larger levers: two
rotation axes the table separates by a sine near the format's own
resolution (`arm ≳ K·ε / 2.2e-16`, about 4.5e7 m at the witness) have
their `atan2` angle (`candidate_rotation`, the two-axis arm) set by the
rounding of the composed rotation, and membership then decides on that
angle.

## What it wants

A decided statement of the session's range that the translation stage
asks of its candidate, refusing `PoseOutOfRange` past it; or membership
margins carried as enclosures so a residual inside its own rounding
escalates as the enclosure it is.
