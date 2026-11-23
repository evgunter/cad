---
id: the-store-keeps-every-version-and-history-undoes-without-destroying
kind: unit
title: Work out what the document store keeps, so a pin is always honoured and any undo is non-destructive, and then build it
status: open
opened: 2026-11-23
priority: P0
cost: X
points: 30
design: true
refs: [7462]
---

Ruled into existence by Ev on PR 7462 (2026-11-23): "maybe open a new program for it where it just has an extra hard P0 like 'figure out how to do this and then do it'". Ev on what the version control was meant to be: "i don't think it was entirely planned out, just that it should allow arbitrary non-destructive undo". Filed by the VSEAM orchestrator.

## The problem

- **The store forgets what pins name.** `pncad::Workspace` holds one file per document id, so re-saving a part destroys the version every older assembly pins (`ContentPin`, the SHA-256 of the canonical bytes, `[assemblies.pins]`). A reopen then fails `PinMismatch`, and the break cascades through nesting. ASSEMBLY AQ1 has this open: "how a one-version-per-id store serves mixed-pin state".
- **The open session cannot see the store change.** This is `document-seam-no-in-session-change-detection`, parked on this row: the memo keeps serving the old version, green, while a reopen would refuse it. The **Accept updated version** offer can only be read off a broken instance.
- **Undo ends at a reopen.** `viewer::history` keeps a tree-shaped undo that nothing persists. GUI-6 (`crates/viewer/GUI-DESIGN.md`, "Banked post-v1"; `docs/LONGTERM-IDEAS.md`, "Undo as a history tree") banks a separable history store and a branch picker. GQ3 says a document's file never depends on that store.

Ev wants arbitrary non-destructive undo. These three are one question about what the store keeps, and the design should treat them together.

## Evidence to start from

Two designers converged on one recommendation for the store-change row (fork-log row 301; PR 7462's body has both reports verbatim). It was made before version control was in view, so take it as input:
- a folder keeps the bytes of every pin written into it, as `(id, pin)` snapshots, until an explicit prune whose roots are what the heads pin;
- every save keeps its document's pins in the target folder;
- the kernel memo reuses a carried part only while the store it is given still holds that pin and every pin beneath it (`PartResolver::holds`, a lookup in a read value);
- the session reads the store once, as a value, with a transitive per-reference standing that the Accept offer reads, and a **Re-read parts** door;
- build order: the store, then the kernel check, then the session.

The VSEAM orchestrator's answer to Ev on PR 7462 noted that those pinned snapshots are content-addressed objects. Under a version control system that keeps every saved state, retention follows automatically, and the only rule it adds is that a pinned version is a root for the prune. So the kept area should be that system's object store, not a second mechanism.

## Open, for the design

- **Scope:** per folder (commits spanning documents) or per document (GUI-6 as written)?
- **What "arbitrary non-destructive undo" spans:** sessions, documents, a fork or a split?
- **Git:** does git track the store's hidden area?
- **The wasm lane:** how does it hold a store with no filesystem?

Each of these is a design question. A change to IDENTITY DI2, ASSEMBLY AQ1, GQ3 or GUI-6 is an `[ev]` PR.

## Price

X, 30 points by hand. The row covers a design fork with three ratified texts in play, a store-format change in `pncad::workspace`, a kernel reuse rule in editor-core, the session's read in the viewer and a history store: about four hard units' work.
