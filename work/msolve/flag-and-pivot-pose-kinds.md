---
id: flag-and-pivot-pose-kinds
kind: unit
title: the Flag and Pivot pose kinds: reads, charts, folds, persistence, picks, and the rung-1 composition through each part's construction
status: open
opened: 2026-10-22
priority: P1
cost: H
---


## What

`[variables.kinds]` and `[variables.poses]`, as ratified on PR 5555,
add two pose kinds.

- **`Flag`:** a plane, a line in it, and the side of the line its face
  lies on.
  - Symmetry: `Prismatic`.
  - Read off a face and one of its bounding edges, or built as a plane
    cut by a plane.
  - Its `Flip` keeps its side: the half-turn about the in-plane line
    perpendicular to the flag's line.
- **`Pivot`:** an axis with a point on it.
  - Symmetry: `Revolute`.
  - Read off a circular edge, a cone, a torus, or a face with a vertex.

They follow the principle stated alongside them: there is a kind for
each subgroup that is the symmetry of something a cell determines,
alone or with one incident cell.

Today, "seated and flush" and a bolt head seated on a hole (an axis
mate plus a plane mate) both refuse as overconstrained. A cone, a torus
or a circular edge is read only as an `Axis` or as a `Point`, never as
both together.

## Owed

- **Reads and kinds.**
  - `PoseDef` reads: face+edge, plane∩plane, and the pivot reads.
  - `VarKind` gains both kinds. `Prismatic` and `Revolute` are already
    in `crates/editor-core/src/mate/coset.rs`.
- **Mates and values.**
  - Each mate leaves one freedom: a flag leaves a slide along its line,
    a pivot a spin about its axis.
  - Their charts fit D′'s `[placement.values]`.
- **The rung-1 composition.**
  - A wall each part builds through a flag mate's edge is one
    construction read over both parts. That makes the flush pair
    structural.
  - A pivot mate makes the shank's coaxiality and the head's seat
    structural.
- **Persistence and the GUI:** wire tags, persistence, the content
  key, and the GUI's pick (a face, then an edge or a vertex).
- **Rows:**
  - a flush seat via a flag mate gives 0 findings;
  - a bolt pivot-mated to a hole gives 0 findings;
  - axis plus plane on the same bolt refuses `Overconstrained`;
  - a drafted wall gives no flush pair.
- **Sequencing:** after D′2
  (`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`),
  whose value chart the two kinds' mates fit; it can run beside D′3
  (`docs/INTENT-STAGE3-SPEC.md` §12, "Sequencing", step 3).

## Released (INTENT, 2026-10-27)

The value chart (PR 5742) merged, with ruling (b) built: the spin is about the target's own direction, with any `Flip` undone. The pivot and flag kinds' mates fit that chart now. Open for MSOLVE.
