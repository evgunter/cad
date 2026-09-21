---
id: import-normalizes-the-rim-only-cap
kind: issue
title: STEP import re-mints a rim-only sphere cap into the seamed form, as a reported normalization
status: open
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census]
priority: P3
cost: D
---


Filed by the TESS orchestrator from Ev's ruling on `[ev]` PR 2850
(2026-09-18, in chat): **a chart singularity inside a face is a vertex
of it** — a loop of rims only, with the sphere pole (or cone apex) in
the face's interior, is not a face of this kernel; import re-mints it
and validity refuses it. Deliberately not in DESIGN.md (Ev: provisional,
"too much weight" there); this unit states it in `NormalizationKind`'s
docs, present tense.

A STEP sphere cap stated with one latitude circle and no meridian — as
two half arcs or as ONE closed circle edge — imports today as `Solid`,
unnormalized, and is the face the ruling says is not one. Import owes a
fifth `NormalizationKind` beside `EdgeFreeSphere` and
`DegenerateApexCone`: re-mint as two half-caps on meridians through a
pole vertex (the form revolve and the boolean plane cut mint), reported.
Which pole is the rim's traversal against the face's sense — PR 2741's
`rim_interior_side` is that decision, written once. The cone apex cap
with one rim is the same statement on a cone.

Fixtures: `crates/step-import/tests/fixtures/tess-cap-diag/`
(`rimonly1.step`, `rimonly2.step`, `gen_cap.py`) on branch
`tess/rim-only-cap-diag` at `83833e586`; TESS-1 lands them on main with
`mesh`'s typed refusal, which is what such a body meets until this
lands. Order: this before TOPO's validity rule
(`work/topo/validity-refuses-an-interior-chart-singularity.md`), or
import starts refusing files it accepts today.

Signed: (TESS orchestrator)
