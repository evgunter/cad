---
id: a-faces-loop-reads-each-joint-off-the-body
kind: unit
title: A planar face's loop reads each joint off the body, a corner or a tangent from the edge between its two walls, at every lane (FACELOOP U1)
status: open
opened: 2026-11-22
priority: P1
cost: M
parent: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
---


The first unit of the face-loop build (`work/faceloop/plan.md`). It is
small, it lands inert, and it settles the unknown both designers named
as the spec's first check: whether the body hands out, at every lane,
the fact a lowered joint has to be declared from.

## Why this comes first

A face's loop lowered to profile pieces meets the lattice at every
vertex. Where two consecutive loop edges meet tangentially, the
sixth-round ruling makes the joint a **declared** tangent joint
(`work/profile/program.md`, ratified ground): a lowering that wrote
`line_to` / `arc_to(Center)` across it would have the lattice decide a
turn of about zero, which refuses `JunctionTurnWithinTolerance`
(`crates/profile/src/path/program.rs`, the `arc_onto` and `line_to`
rows), and `work/profile/` is full of rows where that re-decision
escalates. So the lowering must read the joint off the body, never off
the two pieces' directions.

This is not only the filleted corner. Today `circle` lowers to two
arcs (`circle-lowers-to-one-segment` is parked), so the pipe's rim in
Ev's own elbow is two edges meeting tangentially at two vertices. The
elbow needs this read.

Smoothness is known per **edge**, not per vertex. Nothing maps "the
vertex where loop edges a and b meet" to "the edge between their two
walls" (checked: no such reader in `crates/topo/src`; the one valence
reader is `shell.rs`'s private `valence`, over
`Body::vertex_orbit_linked`, itself `pub(crate)`).

## Done when

A public read in `topo`, generic in the scalar (`T: Decide`, as
`topo::edge_wedge` is), answers for each loop of a face (the outer
loop, `Face::outer`, and each ring, `Face::rings`) the joint at each of
its vertices: the two loop edges that meet there and the third edge
between their two walls, classified as **corner** or **tangent**. A
test can assert each of these:

- **A box's top**: four corners, at `f64`, at `Interval` over a box of
  its variables, and in the `Sym` replay.
- **The pipe's end cap** (`circle` radii `R` and `r`, extruded): both
  loops' two joints read tangent, at all three lanes, with no margin
  read where the two walls lie on one surface record (the seam read by
  identity, `topo::edge_wedge`'s `one_surface_seam`).
- **A box whose four vertical edges are filleted, read on its top**:
  eight joints, each tangent, at all three lanes. The edge between a
  wall and a blend is minted by the blend; its tangency is read from
  what the edge stores (`geom_brep::EdgeDescriptionSpec::TangentIntersection`,
  `crates/geom-brep/src/description.rs`, which `topo::edge_wedge_description`
  and `geom_brep::MustCarryDescription` already speak of), not from a
  margined fold.
- **Refusals, each typed**: a vertex whose valence is not 3 (a slot
  ending on the face's rim, a cone apex); a third edge whose verdict is
  in band (`WedgeVerdict` neither a decided crease nor a seam); a loop
  that passes one edge twice (a slit). Each refusal names the vertex
  and the edges.

Nothing calls the read yet; U2 is its caller.

## The ruled text it serves

`docs/DESIGN.md` `[variables.kinds]` (PR 7284): the loop "is the
face's boundary as it is at each evaluation". The joint is part of
that boundary, so it is read, not decided.

## Left for later units

- The loop form, its lowering and its names:
  `a-profile-on-a-face-holds-that-faces-outline-or-a-hole` (U2).
- Any reading of edge carriers or vertex points into sketch
  coordinates: U2.

## First check, before building

**What each fixture's third edge stores, and whether it reads with no
margin at `Interval` and `Sym`.** Three reads are candidates, in this
order: one surface record on both sides (identity); a stored
description that states a tangency (`TangentIntersection`) or a crease
(`Intersection`); the whole-edge fold (`geom_brep::edge_wedge_over_as`),
which carries a margin. `topo::edge_wedge`'s own docs record that the
fold reads a surface against itself in band at `Sym` (ONEHOME fork
259). So:

1. On the pipe: are the two wall faces of a `circle`'s two arcs one
   surface record? If they are two records with a conventional
   description (`OnPair` or `Chart`), the only reading left is the fold,
   and it will escalate at `Interval` and `Sym`.
2. On a stadium extruded (a drawn tangent joint, `.tangent()` in the
   profile): does the sweep-minted lateral edge store anything that
   says tangent? It is the same question for every drawn-then-swept
   smooth joint.

If either has no margin-free fact at every lane, **stop and report**:
whether a sweep records a declared joint on the edge it mints is a
design question (it would be a change to what an edge stores), not
this unit's to improvise. Do not fall back to the fold with a widened
band.

Also check, and report either way: that the `Sym` replay
(`crates/editor-core/src/coincide/replay.rs`) builds these bodies
without crossing a `launders(…)` site of
`scripts/gates/from-f64-sites.tsv` on the way to the edges read. Two
such classes are open today (`an-edges-parameter-ends-enter-a-carrier-piece-as-constants`
in `geom-brep`'s NURBS and projected-pcurve paths, and the loft
enclosure arm in `profile/src/validate.rs`); neither is on a planar
cap's path by reading, which is unverified.

## Where, by name

- `crates/topo/src/edge_wedge.rs` (`edge_wedge`, `one_surface_seam`,
  `verdict_of`), a new sibling module beside it, and
  `crates/topo/src/lib.rs`'s re-exports. Loop and orbit walks:
  `Face::outer` / `Face::rings` (`crates/topo/src/entity.rs`),
  `topo::live::FaceView::loops`, `Body::vertex_orbit_linked`
  (`crates/topo/src/body.rs`, `pub(crate)`, which this read may use
  from inside the crate). Stored descriptions:
  `geom_brep::EdgeDescriptionSpec` (`crates/geom-brep/src/description.rs`).
- Tests where the fixtures can be built and the `Sym` replay run: an
  `editor-core` test file (the bodies are sweeps and fillets). Not
  found: an existing `topo` fixture for a filleted box at `Sym`.

## Ground crossed

`crates/topo/src/edge_wedge.rs` is no program's path (it is listed on
FACELOOP's). The verdict it composes is the one tier 3's check 4 reads,
so the read must reuse `verdict_of`, not copy it. Tests in
`crates/editor-core/tests/` cross nobody. Nothing to announce unless the
first check fails.

## Rows it closes or changes

None closes. It is the "first check for a spec" of
`a-faces-loop-or-closed-trace-cannot-become-a-profile-loop`.

## Review

**Single (FULL)**, difficulty **M**: a read at three lanes whose
correctness turns on what is margin-free, which a reviewer must check
against the tests' lanes, not take on reading.
