---
id: a-volume-reads-one-anchor-where-a-face-stands-off-its-carrier
kind: issue
title: A certified volume reads one anchor's value where a face's boundary stands off its carrier, and reports only rounding as its width
status: closed
opened: 2026-10-27
priority: P2
cost: M
refs: [a-block-resting-a-zero-band-gap-above-another-refuses-at-the-volume-backstop]
closed: 2026-10-27
---


Split out of `[ev]` PR 5780 (the Zero-band backstop fork) at Ev's
comment of 2026-10-27, which asked for the standoff enclosure to come
out of that PR: the P0 does not need it, and its reasoning as written
did not follow. Both designers raised it; it is filed here to be
brought back with a witness, not as a decided change.

## What

A valid body may hold a face whose boundary points stand off its
carrier by up to ε (`[tolerance.residuals]`). The face's share of the
volume, V = ⅓∮ x·n dA, then depends on where it is read, and differs
by up to that standoff times the face's area. The planar readers
read the face at one point of its loop and enclose only rounding:

- `crates/topo/src/props/quad_lane.rs:357` `planar_face_about` and
  `:421` `polygon_face_about` read the plane's offset at the loop's
  first point.
- `:381` `tilt_slack` encloses only the across-normal part of the
  vector area.
- `:450` `exact_polygon_volume` reports one anchor's polyhedron as an
  exact interval.

PR 3977 moved the anchor and flipped every pinned volume-backstop
refusal on the tilt fixture: 1.4e-9 m³ of anchor dependence on a
13.5 m³ body, against a 4.6e-12 m³ gap.

## Where it still matters after 5780

Once the glue re-derives a glued face's boundary on its kept carrier
(5780's implementing row), a glued planar result has no standoff
beyond rounding. Two cases remain:

- **A rim glued to two carriers that don't meet there.** For example,
  a tube's rim circle glued to a dome's rim 5e-10 off it, which one
  stored circle cannot lie on both.
- **Vertex rounding at computed intersections.** For example, a stored
  result 3e-19 m³ outside its operand at a 4e-18 m standoff.

## Owed

First, a witness where the anchor choice decides a backstop verdict
after 5780's implementation lands, so not a glued planar face. Only
then the question to Ev: should a certified volume enclose its body's
standoff, and what does a backstop "certain" violation mean against
it?

## Closed

Not wanted (Ev, PR 5780, 2026-10-27): "i am hesitant to have special
anything for comparing volume since it's not something we promise
anyway. maybe it is helpful for debugging but maybe we should just
drop it". A volume measurement gets no standoff enclosure.
