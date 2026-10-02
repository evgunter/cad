---
id: lofts-correspondence-twist
kind: unit
title: the lofts cell gains a third body: the same sections with a shifted start vertex, so the loft twists by correspondence alone
status: open
opened: 2026-10-02
priority: P3
cost: E
---

## What

EMIT PR #3147 (ratified on [ev] PR 3102): a loft takes each section's
start vertex as authored, so shifting one section's start vertex twists
the skin — `crates/editor-core/tests/edit_step_segments.rs`,
`a_twisted_loft_takes_the_twist_the_author_wrote` (+30° against −60°).

**Why `lofts` and not `twisted_duct`/`twisted_tube`**: `twisted_tube`
already holds the twisted-spine cell, and its roll is a PLACEMENT
roll — each section rotated rigidly about the spine. A correspondence
twist is a different fact: the sections and their placements are
identical to the straight loft and only WHICH vertices are joined
changes, so the walls become ruled twists between them. `lofts` is
already the cell about what a loft reads from its author (placement in
`loft_prism`, parameterization in `nonuniform_loft`), and this is the
third thing it reads. Add the body to the same cell, one more rigid
offset along +x, so all three share one camera and one scale (the
cell's own reason for being one cell).

## Oracle

The edit_step_segments row's volume relation, and the twisted body's
volume derived in closed form for the section and shift the scene
chooses (read how the +30° row gets its angle before choosing: a
square shifted one vertex is a quarter turn), with the derivation in
the narration as the cell gives for its other two.
