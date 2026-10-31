---
id: space-evaluation-compares-whole-docs-per-frame
kind: issue
title: viewer: space_evaluation compares whole documents, and the properties pane asks it every frame
status: open
opened: 2026-10-18
priority: P4
cost: E
---

Found by the sequential review of PR 5296 (style Q7, unsure).

`DocSession::space_evaluation` (`crates/viewer/src/session.rs`) decides whether
the landed run answers the committed or shown document by comparing whole
`Doc`s with `==`, and the properties pane calls it every frame for the
selected instance. Most of the time the first comparison hits the same
`Arc`-backed value, but nothing measures the cost on a large document. Measure
it, and if it shows up, compare a generation or the history position instead
(the landed run already carries its generation).

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to VGEOM's charter (viewer geometry and display). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
