---
id: a-unions-stacked-coaxial-walls-finish-as-one-face-or-two-by-member-order
kind: issue
title: A union of two stacked coaxial cylinders and a slab finishes the lower wall as one face in three member orders and as two faces split at z = 0.5 in the other three
status: open
opened: 2026-11-02
---



## Finding

Measured by the dual review of PR 6387 (REFS, `reads/holdings`):
`topo::nary_boolean(NaryOp::Union, ..)` over three members:

- coaxial cylinders X (z 0 to 1) and Y (z 0.5 to 1.5), each a
  two-arc prism (`sweep::test_support::cylinder_of_arcs_at(2, ..)`, one
  radius, one centre);
- a slab spanning both, z 0.9 to 1.1.

The finished body differs by member order. In three orders the lower
wall is one face per half-cylinder. In `[2, 1, 0]`, `[1, 2, 0]` and
`[1, 0, 2]` each such face is two faces, split at z = 0.5, the height
where Y's bottom rim met X's wall. A boolean's output stage leaves
maximal faces (`docs/DESIGN.md`'s merge stage), so one of the two
spellings is a merge the fold missed: the co-cylindrical pieces on
either side of the dead rim are not glued (`crates/topo/src/merge_faces.rs`,
the fold steps in `crates/topo/src/boolean/nary.rs`).

PR 6387 changes no geometry. It reads holdings after the door has
built the body, from records the body already carries, so this is the
fold's own behaviour on `origin/main`. Its holdings are exact in every
order, and only the face count differs.
