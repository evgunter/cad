---
id: an-undecided-span-reading-at-rest-on-the-unminted-side-ends-as-a-defect
kind: issue
title: An undecided span reading at rest on the side no construction mints ends as a kernel defect or a damaged file
status: open
opened: 2026-11-11
priority: P2
cost: E
---


A message-ending follow-up from ONEHOME fork 259 (PR 6820), kept out of
that PR by the orchestrator's ruling.

## What

Fork 144 (`crates/topo/src/validate.rs:7370`) and fork 259 rule that an
undecided reading is reported as undecided, never as a definite breach
and never in the "kernel defect … report it" ending. PR 6820 makes the
unsized decisions do so past a build (`Unsized::recourse`,
`crates/geom-brep/src/recourse.rs:299`).

A sized decision does not take that path. `CertCheck::recourse` hands a
sized check to `SizedDecision::recourse`
(`crates/geom-brep/src/certify.rs:1809`,
`crates/geom-brep/src/recourse.rs:665`), and on the side no
construction mints, a span reading at rest still ends in "There is no
way through: this is a kernel defect or a damaged file; report it",
even when the reading is undecided. The "unminted" rows of
`each_decision_family_ends_in_its_routed_sentence`
(`crates/geom-brep/src/certify.rs:8779`) pin that: `ParamSpan` at
`MarginDiag::value(-5e-9)`, in band and undecided, ends at rest in
`KERNEL_OR_FILE_DEFECT_ENDING`.

## Done when

An undecided sized reading at rest, on any side, ends as undecided: over
a box in `BOX_SCALAR_RECOURSE`, at a point in the decision's lever or the
last resort. The unminted rows are re-pinned to say so, and a definite
reading on the unminted side keeps the defect ending.
