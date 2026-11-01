---
id: a-stale-rim-fault-says-a-body-as-node-body-n
kind: issue
title: RimNameFault::Stale says a drawn body as '<node> body N', where the viewer says 'body N of <node>'
status: open
opened: 2026-11-01
priority: P4
cost: E
---


## Finding

Found by the DOORS review of PR 6262 (`doors/face-words`). The viewer
now says a drawn body one way, `tree::DrawnBody`: the node alone for a
node's first body, `body N of <node>` past it. That spelling is used by
`PickedName`, the blend tool's target, and the pick index's refusals.

`editor_core::resolve::RimNameFault::Stale`
(`crates/editor-core/src/resolve/pick.rs`, `impl Say for RimNameFault`,
~:817) is kernel text that the viewer forwards whole, inside
`BlendEvent::RimUnanswered`. It says the same pair as
`a drawn edge of {node} body {body}`. So a blend refusal can name one
body two ways in one status line.

The viewer cannot route this sentence through `DrawnBody`, because the
kernel does not depend on the viewer. The fix is in the kernel's
wording: `a drawn edge of body {body} of {node}`. Or the kernel could
say the node alone for body 0, matching the viewer, if a shared spelling
there is wanted. `NodePickError::NoSuchBody` (same file, ~:650) says
`{node}'s value has no output body at index {body}`. That is a
different sentence about a body that does not exist, and it can stay.
