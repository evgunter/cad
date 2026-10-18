---
id: free-move-landing-prune-gaps-have-no-row
kind: issue
title: viewer: the scratch-landing prune gate and two gesture_check arms have no row
status: open
opened: 2026-10-18
priority: P3
cost: E
---


Found by the verifier of PR 5296's second fix pass. Each gap was shown by a
mutant that survives the viewer suite:

- **The scratch gate.** `DocSession::land` prunes only when `requested_doc ==
  history.doc()` (`crates/viewer/src/session.rs`, "The second prune"), so a
  value gesture's preview never discards committed display state. Replace the
  gate with `if true` and no row goes red. The row it wants: a value gesture
  whose previewed value makes a loose space's placement refuse, pumped
  mid-drag, then cancelled, with the committed frame still held. The bench has
  no value-gesture target, so it needs a fixture of its own, such as the
  pattern in `gesture_table`'s `a_value_gesture_and_a_free_move_probe_do_not_disturb_each_other`.
- **The root half of `gesture_check`** (`crates/viewer/src/display.rs`). Remove
  `space_check(doc, None, root)?` and no row goes red. That case is a gesture
  whose root is deleted while its own copy remains, which a cascade delete
  probably makes unreachable. Either find a shape that reaches it, or argue it
  away in the function's doc.
- **A refused root (unsure).** If the root's own placement refuses while the
  gesture's copy still sits in a space keyed by that root, `gesture_check`
  keeps the gesture, where `space_check` would drop a frame with `NoSpace`.
  Spaces are given to failed nodes too (`spaces_of`). Check whether a copy
  mated to a refused root refuses as well. If it can survive, ask the root
  too.

**Update (the PR that closed `an-overtaken-pin-landing-is-never-pruned`).** The
gate is now `Answers::Structural` at a scratch landing, and
`a_refused_placement_waits_for_the_committed_landing` pins it: the scratch
landing keeps a refused frame, and the committed landing drops it. Still open:
the root half of `gesture_check`, and a gesture whose root is refused while
its copy is not. A frame's deleted root is pinned by
`deleting_a_frames_root_drops_it_at_the_edit`.
