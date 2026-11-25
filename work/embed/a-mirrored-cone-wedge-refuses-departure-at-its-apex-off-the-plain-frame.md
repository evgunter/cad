---
id: a-mirrored-cone-wedge-refuses-departure-at-its-apex-off-the-plain-frame
kind: issue
title: A cone wedge revolved by a negative angle past a half turn refuses Departure at its wedge plane and cone off the plain frame, at one scalar or both, where the same wedge by the positive angle is embedded in every pose
status: open
opened: 2026-11-22
priority: P1
cost: M
---


## Finding

Filed by lane `embed/apex-separator` (2026-11-22, the PR 7315 fix pass),
on its head after merging main `7669fcfdd` (PRs 7120 and 7321).

The triangle with a leg on the axis, revolved about `y` through the
verdict door (`topo::clearance::embedded`, the default budget). By `+θ`
every pose below is `Embedded` at both scalars, and
`self_intersection_proves_injectivity::a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
pins it. By `−θ` only the plain frame is:

| `−θ`, pose | f64 | `Interval` |
|---|---|---|
| −4.7224, plain | Embedded | Embedded |
| −4.7224, turned (axis (1,2,3), 0.7) | Embedded | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` |
| −4.7224, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −4.7224, moved (640, −570, 520) | `Uncertified { 1v1, 4v1, Edge(6v1), Departure }` (381 discharged) | `Budget(Pairs)` |
| −4.7224, a hundred times the size | `Uncertified`, the same pair | `Budget(Pairs)` |
| −4.7224, a needle (r 0.05) | `Uncertified`, the same pair | Embedded |
| −4.7224, flat (h 0.05) | `Uncertified`, the same pair | Embedded |
| −5.5, plain | Embedded | Embedded |
| −5.5, turned | `Budget(Depth)` | `Budget(Depth)` |
| −5.5, moved (64, −57, 52) | Embedded | `Budget(Pairs)` |
| −5.5, moved (640, −570, 520) | Embedded | `Budget(Pairs)` |
| −5.5, a hundred times | `Uncertified`, the same pair | `Budget(Pairs)` |
| −5.5, a needle | `Uncertified`, the same pair | Embedded |
| −5.5, flat | Embedded | Embedded |

The pair is the wedge's plane `1v1` (the start plane, normal ≈ x) and
the cone `4v1`, across the ruling `6v1`, near the apex vertex `3v1`.
Several poses read one class at f64 and another at `Interval`; that is
part of the finding, not tuned away.

**Traced** (−4.7224 moved (640, −570, 520), f64, on PR 7315's head with
prints, 2026-11-22; nothing committed). It is the depth budget, not a
reading of the plane through the apex: all 8 refused splits are
`CellBudget::Depth` at depth 40, plane `1v1` (and `2v1`) against cone
`4v1`, the cone's cells `u ∈ (4.7001, 4.7124)`,
`v ∈ (−1.38e-3, 1.6e-13)`, reaching the apex vertex `3v1`. There the
cone is not presented at all: `present_chart` returns `Local` at
`sides_are_the_faces` (the cover), because the shared ruling `6v1`'s
image on `XLo` (`u = 4.70239`) covers `v ∈ [−1.414213562373102,
−7.105427357601002e-15]` and stops `7.1e-15` short of the apex, the
rounding of its stored end 640 units out, so it does not cover the
cell's side `(−1.38e-3, 0)`. No separator is tried on those cells, the
plane through the apex included.

**It is PR 7115's case**: that PR runs an edge whose image ends an ulp
short of a collapse on to the collapsed side (`ends_on_collapse`).
Measured on PR 7115's head `e88481264`: with PR 7315's `injective.rs`
applied the pose is `Embedded` (173 discharged); with PR 7115's head
alone it refuses `Departure` on `1v1`/`4v1` at `6v1` (260 discharged).
So the mirrored poses need both: PR 7115 to present the cone at the
apex, then PR 7315's plane through the apex to sign it. The review of
PR 7315 found no false certificate across 960 mirrored and re-charted
variants of the hand-built wedge
(`injective::presentation_rows::mirrored_and_recharted_wedges_never_certify_a_meeting`).

**Unchecked:** only this pose is traced; the table's other mirrored
poses are taken to be the same shape, not measured. Re-measure them and
pin the holding ones once PR 7115 lands.

## Re-measured with PR 7115 and PR 7315 together (EMBED, lane pole, 2026-11-23)

On PR 7115's branch merged with the landing train
(`origin/embed/wave3-land` at `f06547ccf`: PR 7315's plane through the
apex and PR 7115's `ends_on_collapse` in one tree), through the verdict
door, the default budget. Every `−θ` pose of the table, at both
scalars, discharged / refused / splits:

| `−θ`, pose | f64 | `Interval` |
|---|---|---|
| −4.7224, plain | Embedded, 205 / 0 / 195 | Embedded, 207 / 0 / 197 |
| −4.7224, turned | Embedded, 894 / 0 / 884 | Embedded, 894 / 0 / 884 |
| −4.7224, moved (64, −57, 52) | Embedded, 175 / 0 / 165 | Embedded, 175 / 0 / 165 |
| −4.7224, moved (640, −570, 520) | Embedded, 173 / 0 / 163 | Embedded, 173 / 0 / 163 |
| −4.7224, a hundred times the size | Embedded, 181 / 0 / 171 | Embedded, 181 / 0 / 171 |
| −4.7224, a needle | Embedded, 1,472 / 0 / 1,462 | Embedded, 1,472 / 0 / 1,462 |
| −4.7224, flat | Embedded, 269 / 0 / 259 | Embedded, 273 / 0 / 263 |
| −5.5, plain | Embedded, 211 / 0 / 201 | Embedded, 191 / 0 / 181 |
| −5.5, turned | `Budget { Depth { 40 }, 1v1, 4v1 }`, 1,533 / 1 / 1,612 | the same, 1,533 / 1 / 1,612 |
| −5.5, moved (64, −57, 52) | Embedded, 182 / 0 / 172 | Embedded, 182 / 0 / 172 |
| −5.5, moved (640, −570, 520) | Embedded, 180 / 0 / 170 | Embedded, 180 / 0 / 170 |
| −5.5, a hundred times | Embedded, 188 / 0 / 178 | Embedded, 210 / 0 / 200 |
| −5.5, a needle | Embedded, 1,030 / 0 / 1,020 | Embedded, 1,030 / 0 / 1,020 |
| −5.5, flat | Embedded, 335 / 0 / 325 | Embedded, 341 / 0 / 331 |

The verdicts are the same at ε 1e-6 (the turned −4.7224 pose at
897 / 0 / 887, the turned −5.5 pose at 1,545 / 1 / 1,626). At ε 1e-12
the poses that build read the same; the two at coordinates of a hundred
and more do not build there, as by `+θ`. One of those build refusals is
a kind the `+θ` poses did not meet: the `−4.7224` and `−5.5` wedges a
hundred times the size, at `Interval`, refuse the cone's rung 3 with
limb 2 undecided (`AnalyticRung3Refusal::Stalled`), which
`at_the_build_limit` now names beside the kernel limit.

So thirteen of the fourteen mirrored poses are embedded at both
scalars, and the scalars agree on every pose.
`a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex` now asks every
pose by `−θ` as by `+θ`.

**What is left on this row: `−5.5` in the turned frame**, which runs
out of depth between the wedge's start plane `1v1` and the cone `4v1`,
one cell refused, at both scalars and all three ε. The same row pins it
as that refusal, red when it holds. Its exit is not traced: whether it
is the cover test at the apex again, at a ruling this PR's run-on does
not reach, or another reading, is not measured.

## Evidence from EMBEDWALK: the turned `−5.5` wedge holds once windows are read off supports

Lane `embedwalk/unit-cone` (PR 7779, its head after merging main at
`e329f79ce`), with `injective.rs` unchanged. `window_of` there reads a
face's affine chart coordinates (a plane's `u` and `v`, a cone's slant)
off its boundary pieces' supports along the coordinate's direction
instead of the boundary's world box, which in a turned frame reached
past the boundary (on a cone, past the apex into the other nappe;
`work/embedwalk/the-unit-cone-on-a-turned-plane-held-until-the-windows-came-from-covers-and-refuses-departure-since.md`).
At f64 and `Interval`, the default ε, release build:

| `−θ`, pose | f64 | `Interval` |
|---|---|---|
| −5.5, turned | Embedded, 195 / 0 / 185 | Embedded, 221 / 0 / 211 |
| −4.7224, turned | Embedded, 205 / 0 / 195 | Embedded, 186 / 0 / 176 |

Every other pose of `a_cone_wedge_past_a_half_turn_is_embedded_at_its_apex`
is embedded too, and the test now asks `Embedded` of every pose. Its
exit was not traced cell by cell: what is demonstrated is that the
window reading alone moves it. The ε 1e-6 and 1e-12 rows are CI's.
