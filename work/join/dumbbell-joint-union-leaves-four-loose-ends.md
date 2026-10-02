---
id: dumbbell-joint-union-leaves-four-loose-ends
kind: issue
title: A dumbbell's two halves unioned across a declared joint disc refuse Join(UnpairedLooseEnds { count: 4 }), torus or cylinder handle alike
status: open
opened: 2026-09-25
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired]
priority: P0
cost: H
---

## What

Two halves of a dumbbell, each a full revolve about `y` (a bell of
radius 1.5 over `|y| ∈ [0.5, 1.5]` on a handle reaching the joint plane
`y = 0` at radius 0.3), pre-merged with `Body::merge_coplanar_faces`,
are unioned with the joint discs and the handle×handle pairs declared
`Rest`. It refuses `Join(UnpairedLooseEnds { count: 4 })`, and the
refusal is the same for both handles:

- a concave TORUS waist (`R = 0.8`, `r = 0.5`), whose two waists are
  one carrier meeting tangentially along the joint circle;
- the CONTROL, a straight cylinder of radius 0.3.

Pinned by `crates/sweep/tests/germ_torus_doors.rs`,
`the_torus_waisted_union_stops_at_the_join_like_the_cylinder_control`.

## Where it is raised (measured, instrumented)

`bool_connect` → `crates/topo/src/boolean/join.rs` (the leftovers
check at the end of the chord join) refuses `UnpairedLooseEnds`. The op
carries declarations and is a union, so `ops.rs` hands the reduction to
the declared-REST zip (`rest::try_rest_union`). For both handles the
zip declines at its segment enumeration (`enumerate_segments` answers
`None`: "matching did not complete — not this lane's frontier"), so the
join's refusal surfaces verbatim.

The joint is a pure REST contact (two coplanar opposite-oriented discs,
bounded by the joint circle) with the handle walls meeting along that
same circle. This is Ev's user-facing case (the dumbbell unioned in the
viewer, `work/germ/torus-operand-gate-admission.md`), and after the
GERM torus doors it is the next refusal whatever the handle.

## Home

ZIP (`join.rs`; `rest.rs` is shared with TANG).

## Measured (JOIN, 2026-10-02; probe branch `join/inface-probe`)

The two vertex-vertex contacts at `(±0.3, 0, 0)` carry crossed face
labels (cylinder/disc against disc/cylinder), so the join matches
nothing. The flank split is the same one as in
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`.
The REST zip declines first, for two reasons of its own:
`rest-zip-segments-read-a-straight-chord-facing-test-and-a-vertex-pair-identity`
on ZIP's slate. Loosening its facing test alone reaches
`RestZipUnsupported { ParallelSeamEdges }`.

## Measured (JOIN-1, 2026-10-02, branch `join/1-germ-locus`)

With germs naming their per-operand locus, each seam semicircle is
`OnEdge` on both halves at both `(±0.3, 0, 0)` sites, so the join now
matches the two segments. It stops further on, differently per handle:

- the TORUS handle at the match's section frame:
  `GermFrameUnsupported { a_kind: Torus, b_kind: Plane }` (no torus×plane
  frame arm; this refusal is not one the REST zip retries);
- the CYLINDER handle in its first chord: `Join(Euler(NotSameFace))`.
  These are edge-edge sites, and no germ record there folds the seam
  into the In run on either operand, so the record falls back to the
  flanking (orbit) order and the two ends put their null edges into
  different flanks. Ordering the flanks by membership instead (the
  flanker outside the other wedge first) makes the cylinder chord both
  segments and stop at role resolution (`JoinDesync`: "neither section
  loop's regions hold a decisive witness"), but moved twelve other rows,
  so JOIN-1 did not ship it.

Re-pinned by the same test.

## Builds (JOIN-1 fix pass, PR 3790)

Both handles build, sound at tiers 2, 3 and 3′ and the at-rest
certificate, at `vol(a) + vol(b)` (torus `14.697464134831966` against
`…963`, cylinder `14.41991027997715` against `…152`). At the seam's
edge-edge sites each half now folds the seam by its own membership,
and each half's chord along it is a copy of its own semicircle, so
neither the torus×plane section frame nor the germ's face pair is
read. Undeclared, both still refuse `CurvedPierceUnsupported` at the
circle rung. Re-pinned:
`crates/sweep/tests/germ_torus_doors.rs`
`the_torus_waisted_union_builds_like_the_cylinder_control`.

## Measured (TANG, 2026-10-02, branch `tang/abutting-rim`)

Both handles BUILD there, the joint discs declared `Rest` and the
handles continuations, through the declared-REST zip: tier 3 and 3′, at
exactly the two halves' volumes (torus 14.697464134831963, cylinder
14.41991027997715), `(12, 22, 14)` faces, edges, vertices, one shell.
The zip now matches germs along circle arcs both operands carry between
two sites before its straight-chord test, and a segment names its two
arcs, so the joint circle's two semicircles are two seams
(`boolean/arcs.rs`, `arcs_along`). JOIN-2's plan replaces
`enumerate_segments` with the join's segments; this is the interim, and
the rows in `germ_torus_doors.rs` are re-pinned to the bodies.
