---
id: vacuous-reads-a-cell-at-a-corner-local-at-every-depth
kind: issue
title: injective's vacuous reads a cell that touches a face's corner from outside Local at every depth, because the touching line's edge is read without running on past the corner
status: open
opened: 2026-11-01
priority: P3
---


## Finding

POTDOC's review found this (PR #6172), and it predates that PR.

`crates/topo/src/injective.rs`, `vacuous`, reads a cell outside the face
as holding none of it only where a line the cell touches has an edge
covering the cell's whole side. It takes that edge's range from plain
`edge_ranges` and `covered`, with no run-on past a corner.

A cell beside a face's corner, outside the face, straddles the corner
along the touching line at every depth. For example, at the corner
`(2, 1)` of a rectangle `u ∈ [1, 2]`, `v ∈ [0, 1]`, take the cell
`u ∈ (2, 2 + h)`, `v ∈ (1 − h, 1 + h)`.

The `u = 2` edge stops at `v = 1` inside the cell's side, so the cell is
never covered. It is not read `Vacuous`, and `present_chart` returns
`Local` for it at every depth. A pair whose touching cells keep one such
cell splits to the depth limit and refuses, though the cell holds no
point of the face. That is an over-refusal.

`present_chart`'s clip sides now run on past a corner (`run_on`, read
by `sides_are_the_faces`). Whether `vacuous` wants the same reading or a
corner-aware one is this row's question: the cell there is past the
corner on both lines' non-face sides.
