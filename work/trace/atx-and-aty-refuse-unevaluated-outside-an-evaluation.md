---
id: atx-and-aty-refuse-unevaluated-outside-an-evaluation
kind: issue
title: trace::resolve_chain refuses AtX/AtY Unevaluated outside an evaluation, though they read no face, so the edit-door check skips them
status: open
opened: 2026-11-02
priority: P3
cost: E
---

Found by a designer weighing `work/selfref/a-profile-cannot-refer-to-its-own-earlier-parts`
(2026-11-02); filed by the VSEAM orchestrator.

`trace::resolve_chain` (`crates/editor-core/src/trace.rs`) refuses
`AtX`/`AtY` with `TraceStepFault::Unevaluated` whenever it runs outside
an evaluation. They read no face, but `Traces` bundles the tolerance
and the records with the faces. As a result the VQ9 edit-door check and
`drawn_pieces` skip programs holding them, so "checks bind twice" is
false for those programs. Untested claim from a designer's reading.
Verify before fixing.

## Note (2026-11-18, DOCTAIL)

The consequence named above is gone: after #7075 neither the edit door
nor the strand report resolves a program (they ask
`ProfileProgram::lattice` and `drawable_pieces`, values erased). The
cause stands for the callers that still resolve outside an evaluation,
`ProfileProgram::pieces` among them.
