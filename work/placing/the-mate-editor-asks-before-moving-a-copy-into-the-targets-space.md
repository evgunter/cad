---
id: the-mate-editor-asks-before-moving-a-copy-into-the-targets-space
kind: issue
title: The placement editor asks by checkbox before moving a copy into the target's space; the draft's target should follow its to picks
status: open
opened: 2026-11-01
---


**Ev's direction (2026-11-01, in a session):** a mate exists only as
part of a placement, so a checkbox whose only job is to make the mate
refuse when unticked is not needed.

## Today

On `main`, the mate tool's panel has a "move the copy into the
target's space" checkbox (`crates/viewer/src/pane/create.rs:692`,
`drafts.mate_into_target_space`), which feeds `SessionOp::AddMate`'s
`into_target_space` (`crates/viewer/src/session/op.rs:516`). Unticked,
a `to` read in another space than the one the copy is placed into
refuses `EditError::SpaceMismatch`, and nothing changes. Ticked, the
same action stands the copy by the identity bundle (if its bundle holds
a mate) and re-targets the placement into `to`'s space before adding
the mate (`crates/viewer/src/session.rs:1901`). The rule came in with
`0af5028ae2` (D′1 viewer: "the mate tool moves a copy into another
space only by name").

The placement editor (PR #6300, `placing/bundle-editor`) keeps the
choice as a flag on the draft (`PlaceDraft::into_target_space`,
`MateTool::set_into_target_space`, the checkbox in `pane/create.rs`),
and its README section keeps "the gesture never moves a copy as a side
effect" (`crates/viewer/README.md`, in "The mate tool commits into a
placement's bundle" as #6300 rewrites it).

## Wanted

The checkbox goes, and so do `into_target_space` on the draft and on
`SessionOp::AddMate`. The draft's target space is part of the draft:
the editor writes a placement's whole bundle in one Apply, and the
space a placement's mates read is part of that placement.

## Design work before building

- **Which space a draft targets when its `to` picks disagree.** Two
  mate rows whose `to` sides are read in two different spaces are a
  draft that no target satisfies. The verdict should name it, and
  Apply should stay withheld.
- **Mates already in the bundle.** Today's re-target stands the copy by
  the identity, which drops every mate. Under the editor, a loaded mate
  whose `to` is in the old space is a row the draft can show struck,
  with the reason, before Apply, rather than a reset after it.
- **What the move strands.** Re-targeting a placement leaves each read
  of its copy in the old space (a `⊔` operand, another placement's
  mate pose) unresolved and typed (INTENT stage-3 spec §3, "An edit
  that re-targets a placement ... is admitted"). The verdict
  (`DocSession::bundle_verdict`) should say which reads it strands,
  so the person sees the cost before Apply. It is not a refusal.
- **The README sentence** "the gesture never moves a copy as a side
  effect" is rewritten. The move is the draft's, shown before Apply,
  not a side effect.

Rides after #6300 merges. It changes text in the viewer README's
mate-tool section, which is a design page, so the PR implementing it
needs Ev's sign-off before it merges.
