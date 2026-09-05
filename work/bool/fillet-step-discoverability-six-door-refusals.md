---
id: fillet-step-discoverability-six-door-refusals
kind: issue
title: the Fillet chain step is hard to discover: six door refusals before a reviewer's rounded pad built
status: open
opened: 2026-09-05
refs: [M10-8]
---

**Found by M10-8's R2 review, by authoring.** To build a document of
their own for the review — a rounded-corner pad with four parametric
corner arcs and a central bore
(`crates/editor-core/tests/m10_8_r2_probes_interval.rs`, `pad`) — the
reviewer went through SIX door refusals before the chain built: the
`ProgramStep::Fillet(Expr)` step (`crates/editor-core/src/program.rs:115`)
is the authoring door for a tangent corner arc on a `LoopProgram::Chain`,
but nothing on the way to it says so, and the refusals along the way
(the shapes a `Fillet` needs to sit between — `LineTo` / `Toward` +
`FarEndTo` legs — and what it refuses) are learned one at a time by
being refused.

Filed on the PATHS lattice's slate (S-BOOL keeps `crates/profile` and
the chain vocabulary) as a discoverability finding, not a defect: every
refusal was typed and the pad built once the shape was right. What is
owed is whichever of these the lattice's owner judges right —

- the refusal text of each door on the way to a `Fillet` naming the
  step that would have been accepted (the refusal as the guide);
- or one worked chain in `docs/PATHS-DESIGN.md` with a `Fillet` between
  two legs, so the shape is read rather than discovered;
- or the `Fillet` step's own docs on `ProgramStep` stating the legs it
  needs on both sides.

The sequence of six is recorded in R2's review of PR #1828 (the
blinded review, not in this tree); the pad builder that finally built
is the reference shape.
