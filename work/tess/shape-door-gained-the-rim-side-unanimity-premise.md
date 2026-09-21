---
id: shape-door-gained-the-rim-side-unanimity-premise
kind: issue
title: tess: require_iso_rectangle now refuses a face whose rims encode different material sides — the seam note for mesh's walk
status: open
opened: 2026-09-20
---



Filed by the PROPS curved-residues unit (PR 2924), which took the
residue named in
`work/props/the-shape-door-could-take-the-sense-free-rim-side-residue.md`.
`crates/mesh/src/curved.rs` is tess's glob, and the door gates the
walk, so this is the seam note the ruling asked for. **It is a
disclosure, not a request** — nothing here is unfinished.

## What changed at the door

`geom_brep::props::require_iso_rectangle` used to ask ONE predicate on
a linearly-leveled face: every rim sits at one of the face's two
extreme levels (`props_rim_level`). It now also asks the sense-free
residue of the flux lane's interior-side premise — **every rim encodes
the SAME material side** (`unanimous_rim_side`, decided as
`props_rim_side`) — which needs no `Face::sense` and so is available to
a door that is handed a surface and a loop with no face.

The door's charter is unchanged in the two places it was explicit
about: a rimless lune still passes (nothing to compare), and a
ZERO-EXTENT face still passes (no extreme for a rim to sit at, so no
rim encodes a side for another to contradict —
`linear_rims_at_extremes` admits `rim_side`'s `DegenerateFace`).

## The measurement, before it was taken

The ruling was *take it only if no body that legitimately meshes today
stops meshing*. Run on `crates/mesh/tests/d9_mesh_goldens.rs`'s corpus,
**40 body/δ pairs at every one of CI's six lane/eps points**
(`{default, interval} × {default, 1e-6, 1e-12}`): exactly **one** body
moved, `apex_crossing_bowtie`, at both its δ (0.05 and 0.15), and it
moved from one REFUSAL to another — digest `d64f3d06937361f5` →
`9cddb7a3543db550`, identical at every point. No body that meshes
stopped meshing, and no digest of a body that meshes moved at all.

## What the walk inherits

`curved::require_iso_rectangle_face` cites the door and maps its error
to `UnsupportedCurvedShape`, so a face whose rims contradict each other
now refuses there rather than reaching the walk. That is the premise
`linear_rim_side`'s own docs say the pairing exists to establish: on
such a face the derived side is a property of where the owning body's
loop flattening started, not of the face.

The one divergence the door still has is unchanged and is NOT closed by
this: the L-shaped complement of a half-cap has ONE rim, so there is
nothing for unanimity to compare. That residue is
`props_rim_interior_side`'s and stays with the flux lane.

## The consequence worth knowing

The bow tie's refusal NAME changed, which cost the branch premise its
body-level witness —
`apex-crossing-branch-premise-has-no-body-level-witness`, filed beside
this.

## A second row moved, outside tess's territory

`crates/topo/tests/mesh12_rim_row_reach.rs` (tcost/tint) states a
sphere rim row as two arcs `R·Δv = 1.5ε` apart. "Which extreme is this
rim at" is undecidable at that gap, so the door now ESCALATES
`props_rim_side` (margin `-1.3163737899724026e-9`, `zero = 1e-9`,
`escalate = 1e-8`) where it used to answer `Ok(())`. That row is
re-baselined in PR 2924 with its reason — the row's own prose already
called the in-band gap *exactly what cannot be decided*, and the door
now says so one predicate earlier than the flux lane's
`props_rim_only_extent`. Recorded here because it is the same seam;
nothing is left for an owner to do.

## The corpus measurement has a blind spot, and here it is

**"A zero-extent face still passes" is true at EXACTLY zero.** The
door's charter says extent is not a shape question, and at `lo == hi`
that still holds: no rim sits at one extreme rather than the other,
`rim_side` answers `DegenerateFace` and `linear_rims_at_extremes`
admits it. Move the extent into the AMBIGUITY BAND and the same
question stops being vacuous and starts being undecidable, so the door
escalates. Executed on a cylinder wall, every offset from the run's own
`Band` (`zero = ε`, `escalate = 10ε`):

| wall extent | `require_iso_rectangle` |
| --- | --- |
| `0` | `Ok(())` |
| `1.5 × zero` | `Escalated { props_rim_side }` |
| `5 × zero` | `Escalated { props_rim_side }` |
| `100 × zero` | `Ok(())` |

So **every linearly-levelled face whose extent lands in the band now
refuses at the door that gates the walk** — a class, where the
measurement above found one sphere body, because no corpus body has a
band-scale extent and a 40-pair golden run cannot see one. The posture
is the ratified one (escalate, never guess) and PROPS is not arguing
with it; this is the disclosure that goes with it. Pinned as
`geom-brep::all iso_rectangle_door::the_doors_zero_extent_charter_is_exactly_at_zero`,
which holds at each ε on the matrix because its offsets are the band's.
