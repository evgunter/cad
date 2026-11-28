---
id: a-strut-rings-side-is-read-from-its-own-germ
kind: issue
title: A strut ring whose pierce point the face door reads on the new face's boundary or in band refuses, where its own germ would say which side it is on
status: open
opened: 2026-11-28
priority: P3
cost: M
refs: [a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate]
---


Filed by the LATHE strut-ring lane (`lathe/strut-ring-homing`,
2026-11-28). No row is known to reach it.

## What

`chord_join::ChordJoiner::rehome_rings` reads a pierce ring on a curved
face that no path places (`path_ring_side` → `Undecided`) by its pierce
point in the new face, through the point-in-face door
(`chord_join::strut_in_run`, `contain::placement_on_carrier`). That
door decides the ball-donut spline placement's struts (6v1 and 7v1,
whose points graze the torus at its extreme parallels `y = ±0.2`,
where no path's parity settles).

It refuses `RingHomingAmbiguous` where the door reads the point on the
new face's edge or at a vertex, in band, or not at all, and on every
spline face (its in-trim foot speaks for one sheet:
`work/splice/the-spline-placement-read-certifies-no-second-in-trim-foot.md`).
A strut at a pinch that the run passes through on a curved face would
be one such case.

## What it would need: the germ reading

Read a strut ring's side at the division that meets it, from its own
germ:

- The ring sits at a pierce point `p`, on the run or grazing it. Its
  null halves carry the germs along which its section polygon leaves
  `p`.
- Read the germ against `newf`'s inward direction at `p` (the run
  edge's tangent there, crossed with the face's outward normal), as
  `boolean::join::germ_side` reads a chord's region.
- Positive is In and negative is Out. In band or zero refuses, as now.
- The premise: the ring does not cross the run (`paths_rung`'s), so its
  polygon lies on its germ's side of the run near `p`. A path that
  reaches `rehome_rings` without that premise must refuse the germ
  reading.

**The plumbing.** `rehome_rings` belongs to `chord_join::ChordJoiner`,
which the boolean and the split lane share, and the joiner carries no
germs. Each lane would hand it a half → germ lookup:

- the boolean, from its reduction's null-edge records
  (`boolean::BoolNullEdgeRecord::germs`, `HalfGerm::dir`,
  `crates/topo/src/boolean/mod.rs:2616`);
- the split, from its apex germs (`splitting::insert`'s `apex_germs`,
  `crates/topo/src/splitting/insert.rs:116`). The split's own
  `NullEdgeRecord` carries no germs.

## Not at `place_pending`

Reading the side at `place_pending`'s `l1 == l2` case, where the germ
is in scope, was weighed and ruled out: it needs the run the ring was
deferred against, which `pending` does not record, and that face may
have been divided again in between.
