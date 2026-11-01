---
id: a-clearance-refusal-says-body-0-of-a-node
kind: issue
title: A clearance refusal says 'body 0 of <node>', where the viewer says a node's first body as the node
status: open
opened: 2026-11-01
priority: P4
cost: E
---


## Finding

Found by the DOORS review of PR 6262 (`doors/face-words`), as a
companion to `work/doctail/a-stale-rim-fault-says-a-body-as-node-body-n.md`.
The viewer now says a drawn body one way, `tree::DrawnBody`: the node
alone for a node's first body, `body N of <node>` past it.

`editor_core::clearance`'s engine refusal
(`crates/editor-core/src/clearance.rs`, the `Self::Engine` arm of its
`Say`, ~:561) says each side as `body {i} of {node}`, writing `body 0`
too. So one body reads as `body 0 of Extrude …` in a clearance refusal
and as `Extrude …` in a pick, a blend refusal, or the pick index's
refusals.

The viewer cannot route kernel text through `DrawnBody`. The cheap fix
is in the kernel: say the node alone for body 0. That copies the
viewer's rule, so the fix that keeps one spelling is the owner's to
weigh, possibly together with the DOCTAIL row: a body-scope phrase in
`editor-core::spoken` that both the kernel and the viewer say through.
