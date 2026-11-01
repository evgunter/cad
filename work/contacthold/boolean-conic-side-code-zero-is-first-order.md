---
id: boolean-conic-side-code-zero-is-first-order
kind: issue
title: A curved edge's side code at a boolean vertex reads its departure to first order, so an arc tangent to the other face at the vertex reads On while it curves off
status: closed
opened: 2026-09-28
priority: P3
cost: H
branch: contacthold/side-code-order2
pr: 4575
closed: 2026-10-11
---


Filed by CONTACT-9. Reached; the findings below are the measurement.

`boolean::sectors::side_code` (`crates/topo/src/boolean/sectors.rs`,
`Reach::Extent`) reads a conic or fitted edge's side of a face plane
at a boolean vertex as its departure direction levered at the edge's
own extent. A definite sign is exact. A Zero is a first-order tangency:
the departure lies within the band of the plane over the extent, to
first order. An arc that departs tangentially and then curves off by
more than the band reads On, and `vtxfac`'s on-edge resolution (or
`recl_edges`) hands it its neighbours' side.

The splitting twin descends one order at the same Zero
(`splitting/neighborhood.rs`, `split_conic_departure`, then
`geom_brep::enters_material_order2`). The boolean has that descent only
for declared `Tangent` pairs (`tangent_lump`).

Reached, the wrong reading refused in `sectors::along`, not at the
join (findings below).
The fix's shape is the splitting lane's: descend to second order at
the Zero, and keep On only for a second-order tie.

**Reached, in its declared form (TANG m9-3 residues, 2026-10-01).**
`crates/sweep/tests/m9_3_zip.rs`'s
`a_tangent_curved_sector_on_a_face_lumps_whole` puts a quarter round's
arc, tangent at its end vertex, on a face containing that vertex: the
arc bound reads On at first order exactly as described. Declared
`Tangent`, the wall sector descends through `tangent_lump`; undeclared,
the curved on-carrier sector refuses C8 (`CurvedBooleanUnsupported`)
first, so the undeclared wrong-germ path is still not reached. The same
fixture shows the arc split at the band's edge
(`work/honeroot/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`).

## Findings (measured 2026-10-10)

Fixture: `crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs` (a
radius-1 quarter round on a slab, its cap arcs tangent to the slab's top
at the rim vertices; postures `wide`, `flush`, `corner`, a `dip` whose
arc bends down into the slab, and a `tilted dip` turned about the rim's
tangent line). A probe logging every `Tangent` reading of a curved
bound in `side_code` showed the whole quarter arc (`len = √2`, not a
band-edge sliver) read `On` against the slab's top in every posture: the
side reading runs before any split.

- **The tilted dip reaches the wrong germ this row predicted.** Turned
  by `α = 0.3` or `0.6`, the round's wall crosses the slab's top, so
  neither face the arc bounds touches the top at the rim vertex, and the
  arc's own reading decides its side. Read `On`, every op refused
  `Join(UnpairedLooseEnds { count: 2 })`. Read by its bend (`In`), every
  op answers, and the intersection is the closed form
  `(5/3 − π/2)/(2·tan α)` (0.1549614 at 0.3).
- **Before INTENT's E (main 493bf8b)** the undeclared postures refused
  `UndeclaredCoincidence` (`bool_plane_offset`) before a sector was
  read. Declared, `wide` with `Rest` refused ∖ ∖ ∩
  `CurvedBooleanUnsupported` on the wall, and answered right when the
  arc was read by its far vertex; `flush` with `Rest` refused its union
  `Escalated bool_wedge_reflex` at `-6.1e-17`, the arc's tangent.
- **After E (main 7e607dded9)** undeclared tangent pairs verify and reach
  the sectors. `wide` answers at every level. The untilted `dip` refuses
  ∖ ∖ ∩ `ClassificationInvariant "a germ sector with both edge bounds on
  the partner face"` (`sectors::along`). That is not the arc's reading:
  its wall touches the slab's top along the rim and lumps whole as a
  tangent sector, and the run boundary lands on the coplanar bottom
  sector beside it, both of whose edges lie on the top
  (`a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector`;
  not the HONE row, whose raising site and sector differ). Forcing every
  reading of the arc to its far vertex moved it only by unlumping the
  wall, onto the HONE row's `vtxfac::pierce_germ_dir`. `flush`/`corner` refuse ∖ ∖ ∩ on an odd
  crossing count that the arc's reading does not move
  (`a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex`).

## The fix

A conic bound is `Reach::Conic { extent, bend }`; `orbit_corners` mints
its curvature vector and its carrier's curvature bounds (`Bend::of`). The
vertex-on-face pass reads a bound through `sectors::pierce_side`, which
at a conic's first-order tie with a PLANE face reads one order down
(`sectors::conic_side`): `In`/`Out` where a certified lower bound on the
arc's lift off the plane clears the band plus the first-order residue;
`On` where the whole edge lies within the band of the plane; a typed
refusal (`BooleanDecision::ConicBend`) otherwise, spiric sections
included. The pass's whole-sector lump keys on first-order ties
(`PierceSide::tied`), so a sector tangent to the face still lumps whole.
`side_code` and `Reach::departure` stay first order and share `rise`.
What still reads first order, and why, is
`conic-ties-outside-the-vertex-on-face-pass-still-read-first-order`.

## Closed 2026-10-11 (PR 4575)

The vertex-on-face pass reads a conic tied with a plane one order down
(`sectors::pierce_side` / `conic_side`, `Reach::Conic { extent, bend }`,
refusing `BooleanDecision::ConicBend` where the bend certifies nothing).
Conic ties elsewhere (the sector passes, recl, curved faces) still read
first order. They are filed as
`conic-ties-outside-the-vertex-on-face-pass-still-read-first-order`.
