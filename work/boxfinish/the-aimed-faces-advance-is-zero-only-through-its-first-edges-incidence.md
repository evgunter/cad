---
id: the-aimed-faces-advance-is-zero-only-through-its-first-edges-incidence
kind: issue
title: The aimed face's advance is zero only through its first edge's incidence, which the walk does not hold
status: open
opened: 2026-11-22
priority: P1
cost: M
design: true
---


Split from the ray-readings unit
(`an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`,
its item 3, branch `boxfinish/ray-readings`) by ruling: the unit ships
the two readings that are arithmetic identities (the loops' turn, the
aim's step) and leaves this one as built.

**The reading.** `solid_contain::ray_crossings`' plane arm reads a
ray's advance to a plane as `t = (origin − q)·n̂ / (d·n̂)`. For the face
an aimed ray is aimed at, `q` is the aim and `origin` the face's stored
plane origin. On a cap that rides a boxed height `h` both carry `h`, so
`t` is `(h − h)/(d·n̂)`: on the door's slab at depth `1 ± 0.25`,
`[−0.508, 0.508]`, where every member's is `0` (read at `f64`, depths
0.75, 1, 1.25).

**Why arithmetic cannot cancel it.** The aim is `m + into·step`, `m` a
point of the face's first edge read off that edge's stored line. The
plane's origin and that line's origin are two stored points, and
`x − x` is not zero at `Interval`. What makes the advance zero is
incidence: the first edge lies on its face's plane. Then
`(m − q)·n̂ = −step·(into·n̂)`, and `into = n̂ × τ` is square to `n̂`.
The same holds for the face across the first edge, whose plane also
holds `m`.

**Why it was not built.** The walk does not hold that premise. PR 6958
declined to pin the advance for the same reason ("the aim lies on the
stored plane only as far as the face's boundary does"), and check 7
runs where the stored certificates are taken as stated, before they
are replayed.

**What it would add, measured** (a local build with the advance to the
two planes on the first edge read from `m`, the turn and the step
re-posed as the unit ships them; door slab, `Sym<Interval>`,
eps 1e-9, on the unit's pre-merge head):

| | turn + step (the unit) | + this reading |
|---|---|---|
| top cap's ray reads to | below ± 0.3 (stops on `side_at_infinity_winding`) | past ± 0.6 |
| bottom cap's ray | ± 0.46 | ± 0.46 (unchanged) |
| the four walls' rays | ± 0.05 | ± 0.09 |
| fixture brick, all six | ± 0.025 | ± 0.04 |

At `f64` it moves the aimed face's advance, and the advance to the
face across the first edge, by that edge's miss of the plane at `m`:
rounding on a door-built body, up to the body's tolerance otherwise.
The `f64` differential was not taken with it in.

**The fork.** Either the walk may read a face's first edge as lying on
the face's plane (then say where that is held and by what, and read
the advance from the edge), or it may not (then a ray aimed at a face
that moves along its own normal reads its own crossing as wide as the
box, and the reader leans on rays aimed at faces that do not move).

**The test that says it is closed.** The fork is ruled; if the first
arm, the top cap's ray of the door's slab reads at `± 0.45` with a row
holding its own crossing's span against the members' `0`.
It also turns one row red by design:
`sweep`'s `an_aimed_ray_over_boxed_door_bodies::a_ray_whose_own_crossing_is_as_wide_as_the_box_reads_no_pair`
pins the top cap's ray as set aside at `± 0.45` and the reader as
unread at `± 0.5`, both of which hold only while the advance is as wide
as the box. With the advance read from the edge the top cap's ray
reads at both widths; that row is then re-stated on a box where a
crossing's span still reaches its neighbour's, or retired with the
reason.
