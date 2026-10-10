---
id: intent
kind: program
title: INTENT — build DESIGN.md D10, one way to say dependency, placement and intent
status: active
opened: 2026-10-03
area: api
prefix: intent/
tag: (INTENT orchestrator)
ab_band: 10800-10899
paths: [crates/editor-core/src/expr.rs, crates/editor-core/src/doc.rs, crates/editor-core/src/parse.rs, crates/editor-core/src/eval/slots.rs, crates/editor-core/src/param_source.rs, crates/editor-core/src/persist/*, crates/pncad/src/document.rs, crates/viewer/src/props.rs, crates/viewer/src/drafts.rs]
keep_out: [D10 is ratified (Ev, PR 3990); this program builds it and does not re-open it — a question D10 does not settle goes to designers and, if it is a fork Ev cares about, to an [ev] PR, never decided by a lane. The ground is shared with RECIPE (doc.rs, edit.rs, node.rs, persist), MSOLVE (mate.rs, mate/*), WIRE (eval/mod.rs, eval/wire.rs), CONTACT, SECT and FLUSH (topo's boolean declarations and census), PATHS (profile tangent joints) and the viewer programs; each crossing is announced on that program's log. The refactor hold every active program carries waits on d10-one-way-to-say-intent-is-unbuilt, which this program owns and closes last.]
priority: P0
budget: 45
---

The program that builds D10 (`docs/DESIGN.md`), ratified by Ev on PR
3990 from the ruling `work/recipe/one-way-to-say-dependency-and-intent`
(closed; Ev's words verbatim at the commits it cites). Every row is P0
(Ev: the problem worsens as more is built haphazardly, and the work
blocks nearly everything else).

`budget: 45` because the first stage is one indivisible representation
change (a variable table, the literal arm, persistence, the façade and
the GUI move together or the tree does not build); later stages split
into their own programs as `plan.md` says.
