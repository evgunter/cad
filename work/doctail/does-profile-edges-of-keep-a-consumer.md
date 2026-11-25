---
id: does-profile-edges-of-keep-a-consumer
kind: issue
title: profile_edges_of (DM8) has no consumer left that needs it, now names spell step ids
status: open
opened: 2026-11-25
priority: P3
cost: E
---


`ProfileProgram::profile_edges_of` (`crates/editor-core/src/program.rs`,
DM8) answers a profile step's canonical segments, from the
`ProfileStructure` record the evaluation keeps on its `pub(crate)`
`ProfilePre`. It was built for the viewer's per-step focus and has only
test callers (`crates/editor-core/tests/edit_step_segments.rs`).

Since EMIT's #3223 a drawn wall's name spells each profile piece by the
minted id of the step that drew it (`ProfileEdgeRef::Piece { step, role }`).
`vgeom/focus-marking-is-per-node-not-per-segment` (PR 7766) lights a
step's walls by reading those ids off the names the pick index holds,
through `RoleSeg::pieces` (`crates/editor-core/src/names/role.rs`), and
needs no structure record and no segment index. So the consumer the door
was built for does not use it.

The one consumer left that names it is
`work/pydoor/python-has-no-step-to-profile-edge-door.md`. If what Python
wants is the walls a step swept, the same name-side route serves it and
the door has no consumer; if it wants canonical segment indices, it does.
The question for this slate: retire the door (and with it the reason for
`work/doctail/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`'s
carry), or keep it for that use.
