---
id: a-union-across-a-tangent-crescent-refuses-unbuilt
kind: issue
title: A union of two plane faces overlapping in a crescent tangent at a vertex refuses TangentCrescentUnsupported: no germ runs along a region of zero angle
status: parked
opened: 2026-10-25
priority: P1
cost: H
blocked_on: [the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways]
---


Filed from
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`,
whose wrong body now refuses typed. Pinned by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`.

## The refusal

A boss `r = 0.3` over a hole or pocket `r = 0.1` about `(0.2, 0)` has
its floor and the plate's top face lying on each other with opposed
normals over a crescent that narrows to the tangent point `(0.3, 0, 0)`.
Where either rim has a vertex there (the other rim is split there), or
neither has and the sweep splits both at their touch
(`boolean/reduce.rs` `interior`), the two vertices are a vertex pair,
and at that pair the floor's sector and the top face's sector share the rims' common tangent and lie on opposite
sides of it: at first order they only touch, and `sectors.rs`
`pair_search` lists the pair as touching (`sector_overlap` reads no
overlap). One order down they overlap, `(κ_b − κ_a)·û > 0`
(`sectors.rs` `crescent`). Under union, `sectors.rs` `crescent_refusal`
(called from `boolean/mod.rs` after `pair_search`) reads that on two
opposed plane faces and refuses `TangentCrescentUnsupported`, in
one-segment and two-arc forms, through and pocket, both operand orders,
at the default ε, 1e-6 and 1e-12. ∩ and both differences keep the
operands and are `SOUND`. The refusal also fires on one pose `main`
built: `a-crescent-refusal-fires-where-short-arcs-carry-the-section`.

## What building it takes

The union glues the two faces over the crescent: the plate's top keeps
the boss's rim as its ring, and the boss's floor shrinks to the part
over the hole, its boundary the hole's rim. The two rims then meet at
one pinch vertex (one shell, through; a pocket's cavity becomes a void
shell touching the outer shell at that point). Nothing upstream of the
join produces the section's germs along a region of zero angle:

- the sector passes (`recl.rs` `recl_sectors`, Program 15.10) read the
  pair as coplanar `On` at most, and reclassify it to no surviving
  record (measured: with the crescent read as an overlap, every record
  of the pair ends `intersect: false` and the union is unchanged);
- the section is two closed loops through one site, each a rim lying in
  the partner's face, and `join.rs` `one_site_segments` (≈935) takes
  one loop per site.

## Around it

- Faces aligned rather than opposed (a holed plate standing on `z = 0`
  beside the boss, its hole tangent inside the boss's rim) put the same
  crescent between the floors; the operands then overlap, and every op
  refuses at the join (`SingleSiteSectionLoop { count: 1 }` one
  segment, `UnpairedLooseEnds { count: 4 }` two arcs). The crescent
  reading is not run there.
- The reading takes a bound's bend from a conic or a line edge. A
  fitted (spline) edge and a bisector read none, so a crescent between
  a spline outline and a circle is not read, and neither is one between
  two conics whose curvatures tie at the vertex (an ellipse's vertex
  osculating a circle). Both are unmeasured.
- Two faces on one curved carrier could overlap in a crescent too; the
  reading runs on plane faces only, and no such pose has been built.

## Plan (measured on PR 5852's tree, 2026-10-27)

Weighed by the lane that took this row with
`a-flush-closed-rim-tangent-at-its-vertex-reaches-the-fate-cut-limit`
and judged larger than `H` for one lane: it is a second-order reading
in the vertex classifier, which every tangent vertex pair runs through,
before it is a join or a result question. Measured on the boss over a
hole tangent inside its rim, two arcs, vertices at the touch, `A ∪ B`
(the boss `A`), with `crescent_refusal` bypassed:

- **What the classifier reads at the touch.** `A`'s sectors there are
  the floor (two, split at `−x`, normal `−z`) and the wall (two, split
  at `+z`, normal `+x`); `B`'s the top (two, split at `+x`, normal `+z`)
  and the hole's wall (two, normal `−x`). Four records, wall × top and
  floor × hole wall on each side of the shared tangent `±y`, every one
  `intersect: false`: each pair touches along `±y`, one bound `On` the
  other's plane. The join then refuses `UnpairedLooseEnds { count: 4 }`
  (the four arcs' ends at the touch with no germ there); the one-segment
  forms refuse the same with count 2, or ship two shells with both faces
  whole where both vertices are at the touch.
- **Reading the crescent as an overlap** (floor × top, both halves, as
  `On`/`On` records) moves `recl_sectors` to read them `(In, In)` and
  `recl_edges` to reclassify the walls' `On` bounds, but the two crossing
  records the section needs stay `intersect: false`: wall × top reads
  `sa = (Out, In)`, `sb = (On, Out)`, and floor × hole wall
  `sa = (On, Out)`, `sb = (Out, In)`. The `On` left in each is a bound
  tangent to a CURVED partner face: the hole's rim against the boss
  wall's tangent plane, the boss's rim against the hole wall's.
  `sectors::pair_codes` reads a bound against the face's tangent plane
  only, and `pierce_side` says so of a tie with a curved face ("which
  way it leaves the face is the two carriers' relative curvature").
  Read one order down (the bound's curvature against the face's normal
  curvature along it), both are `In`: the hole's rim bends inside the
  boss's cylinder, the boss's rim stays in the plate's material outside
  the hole's. With them `In` both records cross, and their germs are the
  two the union needs: the boss's rim (`A` on its edge, `B` in its top)
  and the hole's rim (`A` in its floor, `B` on its edge). Their loci
  are the second-order reading `sectors::runs_into` already makes.

## What building it takes, in order

1. **The crescent as an overlap**, under union of two opposed plane
   faces (`sectors::crescent`, which `crescent_refusal` reads today): a
   coplanar record per crescent half, so `recl_sectors` reclassifies it.
2. **A bound tangent to a curved partner face read one order down** in
   the pair codes: the bound's curvature vector against the face's
   normal curvature in the bound's direction, as a decided margin in
   metres (`(κ_b·n̂ − II(d̂, d̂))·l²/2` at the bound's reach), registered
   like `bool_sector_runs_into_bend`. This is the step with reach: every
   bound tangent to a curved face at a vertex pair reads it, so its
   outcome diff runs over the whole boolean suite.
3. **Two one-site loops through one site** at the join: each rim lying
   in the partner's face, tangent to that face's boundary at the touch
   (`join.rs` `closes_at_its_site`'s `Closed` against `Other`, which a
   boss standing over a hole tangent outside already takes once). Today
   `one_site_segments` takes one record per site.
4. **The pinch**, two vertices at the touch with a vertex–vertex contact
   record (`[topology]`: touching through distinct entities): one shell
   touching itself through a hole, and the pocket's cavity a void shell
   touching the outer shell.
5. `crescent_refusal` and `TangentCrescentUnsupported` go, and
   `a_boss_over_a_hole_by_pose`'s crescent cells read `Sound(1)` through
   and two shells pocketed.

Step 2 is a design question with more than one answer (a curvature
reading in `pair_codes`, or the declared-tangent machinery of
`sectors::tangent_relative_side` extended to undeclared ties), so a
designer pass first.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)

## Step 2 decided (2026-11-01, TANGRIM)

The designers' weighing of step 2 is decided: one bound-side reading for every pass, and one parting primitive, spelled out on the carrying unit `the-split-walk-and-the-boolean-sector-pass-read-a-bounds-side-two-ways` (`## Design`). This row is parked on it. Two consequences for the steps above:

- Step 1 is not a union-of-opposed-planes special case: the crescent is `sector_overlap` read one order down by the same primitive, in every op; `crescent` stops being a refusal detector.
- Steps 3–4 (two one-site loops at the join, the pinch) were measured with the ties read `On` and `crescent_refusal` bypassed; re-measure them once the reading lands, before designing them. Unmeasured too: whether ∩ and both differences stay `SOUND` once the crescent reads as an overlap in every op.
