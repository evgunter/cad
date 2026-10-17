---
id: a-crown-touching-the-plane-along-one-edge-panics-at-the-section-spur-check
kind: issue
title: A crown whose ring touches the split plane at one corner panics at the join's section-spur check
status: closed
opened: 2026-10-10
priority: P1
cost: M
branch: cleave/crown-spur-panic
closed: 2026-10-11
pr: 4622
---

Found by the re-review fuzz of PR 4565, on main before it and on its
head alike.

## What

`topo::split` panics in the join's section-spur check
(`splitting/join.rs`, the `split_section_spur` reading's `Sign::Zero`
arm: "section polygon … runs out along a touching edge and back"). It
does so on exact, valid input: a `meeting::posed_crown` whose ring has
one corner exactly at `MEET`'s height and the rest above it, split by
the plane through `MEET`, at every pose and both normals (80 cells in
the fuzz). That is a D9 panic on valid input.

Witness C2: `posed_crown` with bottom (0, 0, −1) and ring
(0.4760, 1.3181, 0.3518) (-0.7080, 0.8592, 0.1597)
(-1.1890, 0.5653, 0.1028) (-1.1262, -0.2072, 0) (0.4166, -1.4394, 0.2963),
the plane through `MEET` with normal +z, at rest.

The plane touches the crown's top fan along the one edge from `MEET` to
the corner at height 0, and the section polygon runs out along it and
back, which the `unreachable!` argues no contact can mint. Not traced
further.

## Neighbour

`work/emit/split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge.md`
(closed, PR 3133): a plane tangent along an edge and crossing elsewhere
kept the edge as a zero-area spur. That fix's guard is the arm this
witness reaches.

## Owed

A row over C2 at every pose and both normals, at all three ε, that
builds sound or refuses typed. Then the arm's argument re-made, or the
arm made a typed refusal if the input class is honestly unbuildable.

## Built (branch cleave/crown-spur-panic)

Measured on `origin/main` at 6a304186a8 (PR 4565 merged): C2 panics at
the `split_section_spur` `unreachable!` at all 5 poses × both normals,
at ε = default, 1e-6 and 1e-12.

Cause: the arm's argument was false. Rule (b) reads the valley from
`MEET` to the corner in the plane reflex and sends it Below. At the
corner the runs above lie apart (two null edges); at `MEET` they join
round it (one null edge). So the polygon runs out along the valley and
back, and both loops carry the spur. Below's loop has the valley edge
itself, both halves in a row (a strut); Above's has two distinct
chords from two copies of the corner to the one copy of `MEET` (a
slit). The slit is right: Above's piece touches itself along the
valley, through distinct entities. The strut is not: with the panic
bypassed, every cell refused `ResultInvalid` with
`ScaffoldingStrutVertex` at `MEET`'s Below copy.

- `Sweep::strip_spurs` (the join) replaces `assert_no_spur`. It reads
  both loops' straight tips (`join::spur_tips`), requires as many struts
  as slits (an unpaired one panics, as the old invariant did), and
  kills every strut (`join::strut_turn`, curved included) with its tip.
  Below's section face is then the plain polygon, Below keeps no vertex
  at `MEET`, and the finish and `plane_section` read one loop.
- `plane_section` takes each slit out of its outline
  (`section::drop_slits`): the slit lies inside the region. A slit is
  read straight or curved: two conic arcs that agree at the ends and at
  a quarter, a half and three quarters of the way along are one.
- `meeting::corners_disjoint` reads a slit's tip as a full turn where
  the face lies round it, and refuses a spur out of the face
  (`corner_sweep`, with a unit row for both).

Pins, in `split_through_an_apex`:
- `a_crown_with_one_corner_in_the_plane_splits_sound`: C2 plus four
  ring families (3 to 6 corners; the seed, a middle and the last corner
  in the plane), the corner at 0, ±0.6 ε and ±0.9 ε, 5 poses × both
  normals. Both sides' volumes against `crown_above`, tier 3, corners
  disjoint, one vertex at `MEET` above and none below, and
  `plane_section`'s one region against the bottom fan's section: its
  area, and an outline of as many distinct corners as the ring.
- `a_trench_whose_keel_lies_in_the_plane_splits_sound`: a box less a
  trench (`topo::subtract`) whose keel runs from `MEET` out through the
  box's side, at every pose: the same assertions against the closed
  form.

- `sweep` `a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`:
  a 270° annulus revolve less a 150° V revolve turned −20° or ±160°,
  its keel an arc in the plane `y = 1`, the tip at 0, ±0.6 ε and
  ±0.9 ε, both normals: the halves' volumes sum to the body's, tier 3,
  and `plane_section` reads one region, the sector, with 5 distinct
  corners.

All pass at the three ε.

## Closed (PR 4622, 2026-10-11)

Rule (b)'s reflex verdict leaves a strut on Below and a slit on Above.
The join now kills every strut with `kev` before the section is
recorded, and `plane_section` drops slits, straight or conic, from its
outline, so the split and the query read one loop. An unpaired spur is
D9 `unreachable!`. Pinned across crowns at 0, ±0.6ε and ±0.9ε, a
boolean-built trench, and a curved keel, with outline corner counts.
