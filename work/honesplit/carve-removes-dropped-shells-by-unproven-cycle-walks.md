---
id: carve-removes-dropped-shells-by-unproven-cycle-walks
kind: issue
title: splitting::finish::carve removes the dropped shells' half-edges and loops by cycle walks it never proves claim their loops: a torn next leaves a dangling member or removes a kept shell's half-edge
status: open
opened: 2026-09-30
cost: M
priority: P3
---

## What

Found by the second pass of TOPO's walk-proofs unit (PR 3511), which
made every Euler plan that moves or removes half-edges by a loop walk
prove the walk claims its loop (`Body::require_run_of`,
`crates/topo/src/euler.rs`), and then checked every production
`loops.remove` and `vertices.remove` in `crates/topo/src`.

`splitting::finish::carve` (`crates/topo/src/splitting/finish.rs`,
~:845) collects a dropped shell's half-edges, start vertices and
edges from `body.loop_cycle(first)` of each loop (~:872), then removes
those loops, half-edges, edges and vertices (~:891). The walk steps
`next` and reads no `parent_loop`. A torn `next` closed past a member
leaves that half-edge in the carved body naming a removed loop, and
one diverted through a kept shell's loop removes that shell's
half-edges and the vertices they start at.

Not measured; `carve` runs on the split's own output, so how far a
torn input reaches it depends on what the split's earlier passes
already refuse.

## The shape to give

Prove each walked loop is exactly its walk (every member claims the
loop, and no half-edge outside the walk does), as
`Body::require_run_of` with `RunExtent::Whole` does, and refuse
`SplitFinishError::Corrupt` otherwise.

## Also: the dropped shells' other records (TOPO PR 3570)

The receipt of TOPO's kill-proofs unit (PR 3570) re-derived every
production `.remove(` on a topology arena in `crates/topo/src`; the
Euler kills now prove that no record they keep names one they remove
(`Body::require_vertex_unnamed`, `Body::require_edge_unnamed`,
`Body::require_loop_unlisted`, `Body::require_face_unnamed`,
`Body::require_shell_unnamed`, `Body::require_solid_unnamed`,
`crates/topo/src/euler.rs`). `carve` has the same shape past its
walks: it removes the dropped shells, their faces, edges and vertices,
and no kept record is proven not to name one — a kept face whose
`shell` names a dropped shell, a kept loop whose `face` names a
dropped face, a kept half-edge starting at a dropped vertex or naming
a dropped edge, or a kept solid other than `solid` listing a dropped
shell. The fix can call those helpers with the dropped sets as the
clearing, once per record kind rather than per record, since `carve`
removes many.

TOPO PR 3592 adds the half-edge side: `Body::require_killed_halves_unnamed`
proves that no kept half-edge's `next`/`prev`, loop's `first`,
vertex's `emanating` or edge's slot names a half-edge a kill removes.
`carve` removes the dropped shells' half-edges with the same four
namers unproven. The helper takes a kill's two halves; a caller that
removes many needs that parameter widened to a set.

## Also: the section walks that describe edges (TOPO PR 4016)

TOPO's row-walk unit (PR 4016) moved the pcurve rows walk and the
site-row walks onto `Body::loop_cycle_of` (`crates/topo/src/body.rs`),
which refuses a member that does not claim the loop. Its receipt
found two more writes `split` decides from `loop_cycle` walks with no
`parent_loop` proof and no entry gate (`classify::gate_operand`
checks surface and curve kinds only; the closing `validate_closed` per
side, `splitting/mod.rs`, is the only tier check):

- `splitting::finish::describe_section_boundary`
  (`crates/topo/src/splitting/finish.rs:685`, the walk at `:701`) hands
  each walked edge of a section face to `Body::set_edge_curve`.
- `splitting::finish::section_plane_restatements` (`finish.rs:614`,
  the walk at `:629`) restates the walked
  edges into `Body::set_face_surfaces_describing`.

A diverted walk re-describes another face's edge. The same call swap
covers all three (`loop_cycle_of(first, loop)`, refusing `Corrupt`
where it answers `None`), with `carve` also owing the whole proof
above. Not measured.
