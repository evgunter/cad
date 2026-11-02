---
id: the-mate-editor-asks-before-moving-a-copy-into-the-targets-space
kind: issue
title: A mate can move its placement into another space (the 'move the copy into the target's space' checkbox); a mate belongs to an existing placement and never changes its target
status: open
opened: 2026-11-01
---


**Ev's direction (2026-11-01, in a session):** a placement consists of
several constraints (mates, slides, rotations and so on), and a mate
can only be created as part of a specific placement that already
exists. So the placement's target space is the placement's own
property, and a mate never changes it.

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

- **The checkbox goes**, with `into_target_space` on the draft and on
  `SessionOp::AddMate`. No mate moves its placement, so there is
  nothing to opt into.
- **A `to` pick read in another space than the placement's target is
  refused**, said in the editor ("not in this placement's target
  space", naming both), and the draft is left as it was. The
  document-level refusal (`EditError::SpaceMismatch`) is unchanged.
- **Changing a placement's target space is its own edit** of the
  placement, made deliberately (today `SetParam` of its `target`;
  whether the editor offers it as a field is the design work below).
- **The README's mate-tool section** is rewritten to this rule: a mate
  is a constraint of one placement and reads its `to` in that
  placement's target. Drop "the gesture never moves a copy as a side
  effect" and the "move the copy into the target's space" sentences.

## Design work before building

The consequences the checkbox's move had now belong to the
target-space edit:
- **Mates read in the old space.** Today's re-target stands the copy
  by the identity, dropping every mate. Under the edit, each mate whose
  `to` reads the old space is shown struck out, with the reason, before
  Apply, rather than reset after it.
- **What the move strands.** Re-targeting a placement leaves each read
  of its copy in the old space (a `⊔` operand, another placement's
  mate pose) unresolved and typed (INTENT stage-3 spec §3, "An edit
  that re-targets a placement ... is admitted"). The verdict
  (`DocSession::bundle_verdict`) should say which reads it strands
  before Apply. It is not a refusal.
- **Where the edit lives.** It could be a target field on the Edit
  door, or the existing `Add to the product` / `IntoOwnSpace` doors
  (which already re-target placements) and a general one beside them.

Rides after #6300 merges. It changes text in the viewer README's
mate-tool section, which is a design page, so the PR implementing it
needs Ev's sign-off before it merges. The Create door's
"placement and its first mates in one Apply" is filed separately, on
`creating-a-thing-and-editing-it-are-two-implementations`.
