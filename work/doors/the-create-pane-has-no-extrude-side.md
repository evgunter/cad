---
id: the-create-pane-has-no-extrude-side
kind: issue
title: The extrude form has no side control, so a ruled depth-and-side extrude cannot be authored from the GUI
status: closed
opened: 2026-10-01
priority: P1
cost: M
branch: doors/extrude-side
closed: 2026-11-01
---


The viewer half of Ev's ruling on #3551 (2026-10-01): an extrude's distance is a positive depth with a structural `side`. It is blocked on EDIT's `extrude-distance-is-a-depth-and-a-side`, which builds the node and the kernel door.

**What to build here:**
- a `side` control in the extrude form (`pane/create.rs`, `extrude_ui`);
- `SessionOp::AddExtrude` carrying it;
- the property panel's structural-parameter row for an existing extrude's `side`.

The negative-depth refusal's recourse names `side`, and the form should make that recourse one click: the form's own control, not a typed sentence the author must act on.

The range probe needs no change: the floor becomes a half-line every seed brackets.

## Closed (2026-11-01, superseded by D10)

The lane dispatched on this row (PR 6260) found nothing to build. `Node::Extrude` carries no `side` on main (`crates/editor-core/src/node.rs`: only `profile` and `distance`), and `SetExtrudeSide` is retired. FORK-S3P (`docs/DESIGN.md` `[placement]`, commit `9d33cbd02c`, ratified by Ev on PR 4324) moved "which side of the face its material lies on" to the placement's mates. INTENT D (PR 5506) removed the field. Ev's #3551 ruling predates FORK-S3P, and this row's spec described the pre-D10 node.

The residue is the negative-depth recourse. A document extrude with a negative depth still says "set the side to against the sketch normal", which no document door can do. PR 6260 filed it on LATHE as `work/lathe/a-documents-negative-depth-recourse-names-a-side-it-cannot-set`. #3551's "offer the other side in one click" belongs to that row's recourse: once the kernel states a recourse a document can take, the viewer offers it through its existing offer home. (DOORS orchestrator)
