---
id: via-and-center-arc-closers-declared-arrival
kind: issue
title: "paths: the declared seam arrival for the Via and Center arc closers (arc_to with a via point or centre targeting Start.arrives_tangent())"
status: open
opened: 2026-09-02
github: 1579
refs: [1573, 433, BOOL-12]
---

## From GitHub issue 1579

Opened 2026-09-02; 0 comments.

**Filed from BOOL-12 (PR [#1573](https://github.com/evgunter/cad/pull/1573)) as the schedule for a disclosed deviation.**

BOOL-12 gave the arrival declaration (`Start.arrives_straight()` / `Start.arrives_tangent()`) to `line_to`, `continue_to`, `tangent_arc_to` and, in its fix pass, to `arc_to(Bulge { p: Start.arrives_tangent(), .. })` (the arc's end tangent is fixed by its bulge, so the CHECK form applies unchanged). The `Via` and `Center` arc data still refuse the declared target as a replay `Transition` violation — scheduled here rather than left as prose.

**Owed:** the same declare-and-check arrival on the `Via` and `Center` forms (their end tangents are equally fixed by the authored data), one row per form both directions, and the target census's admissible-pair table updated so the census, not a comment, says which pairs are spellable.

Refs issue 433, BOOL-12 (`docs/BOOL-12-SPEC.md`), the target census in `crates/editor-core/tests/switch_program_vocabulary.rs`.

## Home

`work/bool/` — BOOL-12's own disclosed deviation on the PATHS lattice vocabulary, S-BOOL's charter ground.
