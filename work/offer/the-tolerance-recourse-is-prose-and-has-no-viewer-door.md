---
id: the-tolerance-recourse-is-prose-and-has-no-viewer-door
kind: issue
title: the tolerance recourse is prose in fourteen kernel files, and the viewer has no door to the tolerance edit it names
status: open
opened: 2026-11-22
priority: P2
cost: M
design: true
---

Owed by the decided offer design (Ev, #7341). Under the signed `[one-way.places]` sentence, a failure carries as data what its recourse determines. The tolerance recourses ("lower" or "loosen the tolerance", in fourteen files across `editor-core`, `geom-brep`, `profile`, `sweep` and `topo`; `grep -rliE "(lower|loosen)(ing)? the tolerance" crates/*/src`) are prose. [tolerance.recourse] says such a recourse is phrased "with the value the margin gives". So the geometry layer should carry that value, and the document layer names the `DocEdit::SetTolerance` it determines. No `SessionOp` emits that edit, and a session's ε is fixed when the session is built.

Two parts:
- **Kernel:** type the value.
- **Viewer:** a tolerance door, which may need the session to rebuild at a new ε.

Until both land, the act has no control and no plain-words statement, because it is not typed. (OFFER orchestrator)
