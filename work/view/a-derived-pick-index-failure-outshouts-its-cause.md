---
id: a-derived-pick-index-failure-outshouts-its-cause
kind: issue
title: viewer: a pick-index failure caused by a failed node is the loud banner, and the node's own refusal is the quiet line
status: open
opened: 2026-09-17
---

**Ev reported this** (in chat, 2026-09-17), from unioning the two
halves of a local dumbbell document (a git-ignored
`demos/tour/gallery/dumbbell.pncad`). The top of the window showed:

> pick index: root 11's bodies could not be tessellated or indexed:
> hit test: node 11 failed, so it has no name table to invert — fix
> the node's own failure before picking against it

and "much quieter", lower down, the actual cause: node 13's Boolean
refusal (a torus×plane operand pair, which is CURVED's
`c5-plane-torus-cone-cylinder-arms` / `torus-operand-gate-admission`).

The loud message is a CONSEQUENCE of the quiet one: the pick index
cannot invert the name table of a node that failed to evaluate
(`viewer`'s `pickindex.rs`, the "could not be tessellated or indexed"
arm; `editor-core`'s `resolve/hit.rs`, "no name table to invert"). A
downstream effect of a failure the user already has in front of them
should not outrank that failure. Either drop it, when the failure it
depends on is already shown, or place it under that failure. It also
names a different node (root 11) from the one that failed (13), so a
reader cannot tell the two messages are about the same event.

Not investigated beyond locating the two message sites.
