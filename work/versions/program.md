---
id: versions
kind: program
title: VERSIONS — what the document store keeps, so a pin can always be honoured and an undo never destroys
status: ready
opened: 2026-11-23
group: api/assembly
prefix: versions/
tag: (VERSIONS orchestrator)
paths: [crates/pncad/src/workspace.rs, crates/viewer/src/history.rs]
keep_out: [opened 2026-11-23 by the VSEAM orchestrator on Ev's ruling on PR 7462 - the document store (pncad::workspace, LIB's ground before LIB closed, PYDOOR's and BIND's now for its Python side) is shared ground and a row here that edits it announces the seam, the kernel memo's reuse rule is editor-core's (crates/editor-core/src/eval/parts.rs and part.rs) and the session's read of the store is VSEAM's (crates/viewer/src/session.rs, evalseam.rs, parts.rs) - so the X row's build crosses three owners and is cut into units on them before dispatch, IDENTITY DI2 and ASSEMBLY AQ1 and GQ3 and GUI-6 are ratified or banked design text and any change to them is an ev PR]
---

**What the document store keeps.** A folder today holds one file per
document id, so re-saving a part destroys the version every older
assembly pins, and the session's undo is a tree that ends at a reopen.
Ev wants version control that allows **arbitrary non-destructive undo**,
and a store that can always serve what a pin names. Neither is planned
out yet (Ev, PR 7462, 2026-11-23).

The track holds one row, an extra-hard P0: work out the design, then
build it. The VSEAM designers' converged recommendation on
`document-seam-no-in-session-change-detection` (fork-log row 301, PR
7462) is its starting evidence, not its answer.

Plan: `work/versions/plan.md`. Log: `work/versions/log.md`. Lanes are
held to `docs/prompts/implementer-discipline.md`,
`docs/prompts/reviewer-style-lane.md` and `docs/prompts/designer.md`,
handed over by path.
