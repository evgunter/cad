---
id: the-shape-door-could-take-the-sense-free-rim-side-residue
kind: issue
title: props: require_iso_rectangle admits a face whose rims encode different material sides; the sense-free residue unanimous_rim_side already decides it in the gate arm
status: review
opened: 2026-09-16
priority: P0
cost: D
branch: props/curved-residues
pr: 2924
---


Filed by the PROPS sphere-pole-side fix pass (PR 2741), answering the
dual review's question "can the shape door take the second rectangle
predicate cheaply?" — measured: it can take a WEAKER form of it, and
that form is already written.

## The measurement

`require_iso_rectangle` asks ONE predicate (`props_rim_level`: every
rim sits at an extreme). The sphere's flux lane asks two — the second
is `props_rim_interior_side`, every rim's interior side points INTO the
folded extent. **The door cannot take the second**: σ is the rim's
traversal under the face's SENSE bit, and the door is handed a surface
and a loop with no face, deliberately, so that it answers a question
about the boundary alone.

What it CAN take is the sense-free residue: **every rim encodes the
same material side**. `unanimous_rim_side` (`props/curved.rs`) is that
predicate, written for `boundary_material_sign` in this same PR, and it
needs no bit. Executed there: a sphere face carrying a rim at `lo` and
a rim at `hi` traversed the SAME way — R2's staircase face — read
`Encoded(Positive)` with the lower rim first and `Encoded(Negative)`
with the upper rim first before the change, and is an `Err` after it.
The shape door still answers `Ok(())` for that face.

## Why it was not taken here

Scope and blast radius, not principle. The door is cited by `mesh`'s
walk and by `topo`, so adding a refusal to it changes which bodies
mesh, and this unit's fence is the sphere's flux lane. It is a
strictly-stronger premise on a door whose whole job is that premise, so
the change is a small diff and a large test surface — its own unit.

## What it would NOT close

The divergence the door's own docs now record: the **L-shaped
complement of a half-cap** has ONE rim, so there is nothing for a
unanimity rule to compare, and no sense-free door can tell it from the
half-cap. That residue is `props_rim_interior_side`'s alone and stays
with the flux lane.

## Taken — measured first (PR 2924, branch `props/curved-residues`)

The ruling was *measure the blast radius, take it only if the
measurement is clean, and STOP if anything legitimate stops meshing*.

**The change.** `linear_rims_at_extremes` — the door's predicate on
every linearly-leveled kind — now calls `unanimous_rim_side` instead
of `require_rims_at_extremes` alone. Unanimity subsumes the level
rule (it runs it first), so no decide is doubled.

**The charter is unchanged in the two places the door was explicit
about.** A rimless lune still passes: nothing to compare. A
ZERO-EXTENT face still passes: with `lo == hi` no rim sits at one
extreme rather than the other, so no rim encodes a side for another
to contradict — `rim_side` says exactly that with `DegenerateFace`,
and the door admits it, because extent is not a shape question. The
pinned row `a_zero_extent_cylinder_face_passes_the_shape_door_and_not_the_flux_lane`
is unmoved.

**The measurement.** `crates/mesh/tests/d9_mesh_goldens.rs`'s corpus,
40 body/δ pairs, at all six of CI's lane/eps points
(`{default, interval} × {default, 1e-6, 1e-12}`): exactly **one** body
moved — `apex_crossing_bowtie`, at both its δ — and it moved from one
REFUSAL to another (`d64f3d06937361f5` → `9cddb7a3543db550`, the same
digest at every point). **No body that meshes stopped meshing**, and
no digest of a body that meshes moved at all. The STOP clause is not
triggered, so the residue is taken.

**Two rows moved with it, both re-baselined here with their reason.**

* `mesh::all mesh11_arc_branch::the_apex_crossing_bowtie_refuses_at_the_door`
  — the bow tie's second face states two rims that encode different
  sides, so the shape door names `props_rim_side` before
  `require_one_chart_branch` is asked. Re-pointed and renamed
  `..._refuses_at_the_shape_door`. The cost is a witness, not a
  premise, and it is filed:
  `work/tess/apex-crossing-branch-premise-has-no-body-level-witness.md`.
* `topo::all mesh12_rim_row_reach::the_shape_door_admits_the_rim_only_cap_and_the_flux_lane_reads_the_gap`
  — MESH-12's hairline body states two rims `R·Δv = 1.5ε` apart, so
  "which extreme is this rim at" lands in the ambiguity band and the
  door ESCALATES: margin `-1.3163737899724026e-9` against
  `zero = 1e-9`, `escalate = 1e-8` (the axial-sine reading of
  `R·Δv = 1.5e-9`, shrunk by `cos v̄ = cos 0.5 = 0.8776` — audit note
  N8, already recorded, and the shrink does not move the verdict:
  `1.5e-9` is in the same band). That is the correct answer, not a
  defect: the row's own prose already says the in-band gap is *exactly
  what cannot be decided*, and the door now says it one predicate
  earlier than the flux lane's `props_rim_only_extent`. That body does
  not mesh today either way (the walk emits no triangles for a
  meridian-free loop — issue 1615), so nothing legitimate lost a mesh.
  The row is re-baselined to assert the escalation by name.

**The seam note** for the walk's owner is
`work/tess/shape-door-gained-the-rim-side-unanimity-premise.md`.

**What it does NOT close**, unchanged: the L-shaped complement of a
half-cap has ONE rim, so unanimity has nothing to compare and no
sense-free door can tell it from the half-cap. Pinned as
`iso_rectangle_door::the_one_rim_divergence_survives_the_residue`.
