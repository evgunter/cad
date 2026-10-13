---
id: a-sphere-cut-in-cap-fails-its-names-missing-upstream
kind: issue
title: A document's lens ∩ tilted brick (a sphere cut-in cap) fails EMIT with MissingUpstream naming the lens
status: closed
closed: 2026-10-13
opened: 2026-10-07
priority: P2
cost: M
---


## The finding

A document of two revolved balls (the unit ball, and radius 0.8 at
height 1.4, each a full revolve of a semicircle about y), their
`Intersect` (the lens), and a 0.4 × 0.3 × 0.4 block rigidly turned so
its near face lies 0.985 from the origin toward latitude 68°, azimuth
50°, then `Intersect(lens, brick)`: the lens names (15 rows), and the
cap node fails `Naming(MissingUpstream { node: <the lens> })`, raised
at `names::defer::upstream_name`. Measured on `fuse/curved-join` with
the curved join switched off as well as on, so it does not come from
the join. The same pair builds at the kernel (`sweep`'s `snowman`
rows), so the gap is the emitter's: an entity of the lens the cap's
emission asks a name for is not one the lens's table holds.

## Why it matters

This is the one construction known to leave a conventional vertex (the
cap circle's, after its chart-meridian cut is glued away), so the
naming half of `work/fuse/curved-joinable-vertices-are-left-unjoined`
has no document that reaches it until this names.

## Closed (branch `emit/naming-refusals`, 2026-10-13)

Priority P2: a sound body refused as an emission bug, and the fuse
row's conventional-vertex vocabulary had no document until this one
named.

**Measured.** `crates/editor-core/tests/emit_sphere_cut_in_cap.rs`
builds the document: two full revolves of a semicircle, their
`Intersect`, and the brick on a frame turned `y` onto latitude 68°,
azimuth 50°. The refusal is raised in `emit_topo::name_boolean`'s
`operand_face_name`. A result face descends to operand-A face `6v5`,
and the lens table names only `1v1`, `3v3`, `4v3` and `5v1`. The
fragment rows chase `7v1 → 2v7 → 6v5` and stop there.

**Root cause** (topo). The brick's plane meets the lens's unit-sphere
face in a circle no edge reaches. `sphere_extent_scan` asks for a
cut-in, and `boolean::ops::apply_cut_ins` divides the face along its
chart's meridian with a `mef` before the op re-enters
`boolean_op_recut` on the cut operands. Nothing recorded that division
or the cut edge. So the re-entered op's naming record started at the
cut operand, and `6v5` (the cut's new piece) had no row back to the
lens face. The same gap holds for the carry
(`work/orbit/a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key`).

**Fix.**
- `apply_cut_ins` returns each operand's `(new face, divided face)`
  rows and its meridian edges. The re-cut caller puts the rows ahead
  of the re-entered op's `face_fragments_a`/`_b`, in the same clone
  keys. The edges go in a new `BooleanNaming::chart_cuts`.
- A cut piece is a fragment of the face it divides (N2, as a chord's
  piece is). The cut is no edge of the operand, so `emit_topo`'s
  `senses_at` does not read a chart cut as a line of a vertex.
  Without that, the circle's vertex asked the lens for the cut's name
  (`Emission("a classified operand edge is not named")`).

**Now.** The lens and the brick name. The cap, in either slot, and the
brick minus the lens refuse only
`ConventionalVertex`. That is the circle's one vertex, after the output
stage glued the cut away, and its edge-derived name is
`work/fuse/a-conventional-vertex-mints-no-edge-derived-name`'s to
build. The union and the lens minus the brick keep the cut as two edges
between two pieces of one face, and refuse at the vertex where the
circle crosses it. Filed as
`work/fuse/a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick`.
All six are pinned in
`a_sphere_cut_in_names_the_lens_entities_it_divides`.

Not covered: a cut-in end inside a boundary arc splits that arc
(`split_edge`), and the piece's key is read in the cut operand's keys
only. No document here reaches such a piece at a classified vertex, so
it is unmeasured.

