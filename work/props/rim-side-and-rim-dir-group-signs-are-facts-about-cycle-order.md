---
id: rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order
kind: issue
title: props_rim_side and props_rim_dir_group read whichever rim the loop walk from Cycle::first meets first, so their recorded signs are facts about cycle order rather than about the face
status: open
opened: 2026-09-14
priority: P0
cost: H
---

Filed by the TOPO revert-wrap fix pass (PR 2573, 2026-09-14), on this
slate because `crates/geom-brep/src/props/*` is PROPS's glob.

## Finding

Two of the closed-form props predicates are relative to whichever rim
the caller's loop walk hands over FIRST, so their recorded signs are
facts about cycle order rather than about the face:

- `props_rim_side` — `linear_rim_side`'s inner `side`
  (`crates/geom-brep/src/props/curved.rs`) reads
  `b.rims.first()` and classifies `lo + hi − 2·level` on THAT rim;
- `props_rim_dir_group` — `du_of_rims` seeds its groups from the first
  rim and classifies every later rim's `d_u − g.1` against it.

Both are compensated downstream (`Positive ⇒ rim.d_u_sign`,
`Negative ⇒ .flip()`), so the flux and the readings do not depend on
which rim comes first. The recorded verdicts do: measured on
`voided_rod` (`crates/sweep/tests/shell_census_is_thread_count_invariant.rs`),
both read `Negative ×2` when the reverted rod's wall loops are anchored
where the forward walk left them and `Positive ×2` once
`Body::revert` moves every loop's anchor to its source predecessor
(PR 2573) — same face, same rims, opposite recorded sign. The rim the
walk meets first is the anchor's, and the anchor is a topological
convention (`topo`'s `LoopBoundary::Cycle`), not a property of the
face.

Why it matters here: the k_stats verdict channel is this program's
(`crates/geom-core/src/k_stats.rs`), and k-lint's population and every
golden over recorded verdicts move under a pure re-anchoring of a loop
because of these two predicates alone — the other eleven the census
records on that body are per-rim, per-meridian or per-face facts and
count the same whichever rim comes first
(`voided_rods_verdicts_as_a_sorted_multiset`, the row PR 2573 added,
pins the counts).

Closing shape, undecided: pick the reference rim by a property of the
face rather than by cycle order (the rim at the LOWER level, say, so
`props_rim_side`'s margin is always `hi − lo`-signed and the recorded
sign is the face's), or record these two under a name that says they
are relative reads. The related open row on the same function is
`rim-stores-its-traversal-direction-twice` (`du_of_rims`' direction
compare through the funnel).

## Half retired, half measured (PROPS sphere-pole-side, 2026-09-15)

**`props_rim_dir_group` is gone.** `Rim` stored the traversal direction
twice and `du_of_rims` banded the difference of two values that are
`±1` by construction; the scalar copy is retired with
`rim-stores-its-traversal-direction-twice`, the direction is compared
as a `Sign`, and the predicate reaches no funnel site any more. One of
this row's two subjects therefore cannot move under a re-anchoring
because it records nothing at all. `voided_rod`'s multiset loses
`("props_rim_dir_group Positive", 2)` and the three census digests move
on that body's verdict channel alone.

**`props_rim_side` still moves, measured on one face.**
`crates/geom-brep/tests/props_sphere_pole_side.rs`'s
`the_interior_side_verdicts_are_a_face_fact_under_re_anchoring` presents
one spherical zone (rims at `−0.3` and `0.5`, opposite traversals) with
its edge list rotated, and records:

```
props_rim_side, anchored at the lower rim : ["Positive"]
props_rim_side, anchored at the upper rim : ["Negative"]
```

with the face's area and flux identical either way, and every other
recorded predicate of that face identical as a multiset. That is this
row's claim, executed on a two-rim face rather than on `voided_rod`.

**The closing shape did not fall out of the work, so it is left open.**
The unit's own new predicate, `props_rim_interior_side`, is decided for
EVERY rim on that rim's own offset from its face's extent, so its
recorded verdicts are a face fact and it does not join this row. Making
`props_rim_side` one would mean choosing its reference rim by a
property of the face (the rim at the lower level) or renaming it a
relative read — a change to the CYLINDER's and CONE's recorded
population as much as the sphere's, since `linear_rim_side` is shared,
and one that re-baselines k-lint and the census digests again for a
reason unrelated to either issue this unit closed. Nothing in the
sphere work forced the choice; it is still the call the row describes.

## Half closed by the sphere-pole-side unit (PR #2741, 2026-09-16)

`props_rim_dir_group` is **retired outright** — `Rim` no longer stores
the scalar direction it compared, so that half of this row is gone.

`props_rim_side` stays, and the unit measured it rather than closing
it: under a pure re-anchoring of a loop its recorded sign still flips
(`Positive` at some rotations, `Negative` at others) while every
reading, area and flux stays bit-identical. Its closing shape — pick
the reference rim by a property of the face rather than by cycle order
— did not fall out of this work and would re-baseline the cylinder's
and the cone's recorded populations for a reason unrelated to either
issue the unit closed, so it is left here with the measurement.

Two things this unit establishes that a taker should not re-derive.
The new `props_rim_interior_side` is per rim and reads no reference
rim, so its verdicts are a face fact under re-anchoring — including on
a REFUSING face, since every rim is decided before any refusal is
returned (an escalation still short-circuits, as everywhere else in
the module). And the material-sign gate now reads the sense-free
residue of the same question — every rim's encoded side agreeing —
which removed an anchor-relative definite ANSWER from that arm.
