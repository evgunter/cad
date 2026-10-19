---
id: free-move-admits-a-loose-space
kind: issue
title: viewer: free-move admits a loose space and keys its frame by the space's root (DI5), not an instance no mate names
status: closed
opened: 2026-10-17
priority: P2
cost: M
blocked_on: [a-mate-relates-two-poses]
pr: 5296
closed: 2026-10-18
---


DI5 (`crates/editor-core/IDENTITY.md`, approved on PR 5241) says a
free-move moves the picture of a loose space, its release writes nothing to
the document, and the frame is keyed by the space's root and dropped by
`DisplayState::prune`. The viewer (`crates/viewer/src/display.rs`) still
admits and keys by instance, for an instance no mate names. This item builds
DI5's admission. It waits on stage 3 C (`a-mate-relates-two-poses`), which
makes instances loose by default and restates the viewer's fixtures, so it
builds on C's instances rather than on today's.

## The work (from the INTENT lane's sizing, 2026-10-17)

It is not a re-key; admission needs new code. Spaces are readable at the
viewer: `Evaluation::space(node)` and `loose_roots()`, and `pickindex.rs`
already batches parts per space with loose spaces at the origin. So nothing
needs inventing. What it takes:

1. **Admission.** `free_move_check` takes the evaluation, resolves the
   instance's drawn copies, and requires each to be `Space::Loose { root }`.
   A world copy refuses with a new typed fault. `MateConstrained` goes: a
   copy mated onto another loose copy shares its space and moves with it,
   and one mated onto a world copy is a world copy.
2. **Keying.** The slot and the `moves` map are keyed by root. `view()`
   takes the evaluation and draws the frame over every placement in that
   space, and `prune` drops a root that stops naming a loose space.
3. **Callers.** The session's three free-move ops, the properties pane's
   check, `frame.rs` and `widgets.rs` follow.

**The material consequence.** An empty bundle makes a loose copy, and any
world offset or world pin makes a world copy. Today's free-moveable
instance, placed at a literal offset with no mate, is a world copy, so it
refuses under DI5. The viewer test fixtures (`common::asm::instance_at`) all
place at literal offsets. So nearly every free-move row has to be
re-fixtured onto loose copies: `gesture_table` has about 98 references, plus
`assembly_display`, `review_gui4_r1/r2`, `story_assembly`, `frame_policy`
and `assembly_walk`. For a user, that means parts placed at an offset stop
being free-moveable, and only unpinned copies probe.

The rows it owes: a loose space with two copies mated to each other
free-moves as one; a copy pinned to the world refuses, typed; prune drops
the frame when a placement pins the space.

## Ruled (INTENT orchestrator, 2026-10-17)

Spaces exist only after the solve (`spaces_of` takes `SolvedPoses`), so the
viewer reads them only off a landed `Evaluation`, never off the document
alone, and evaluations land asynchronously.

- **Admission** reads the session's landed evaluation. When there is none
  for the current document, it refuses with a new typed fault (DI3: an
  evaluation names its document) rather than waiting. It never reads an
  older document's spaces.
- **Prune** runs on a document change, as now, for nodes gone, and again
  when an evaluation lands. That second prune drops a root that stopped
  naming a loose space ("a placement pins the copy" takes effect when the
  solve that pins it lands).
- **Between an edit and its evaluation**, the frame keeps drawing over the
  root's last known copies: the display-only window.

One thing the build should re-check: the value-gesture overlap argument at
`SessionOp::permitted_during_value_gesture` rests on every display predicate
being a function of the node graph. A space is a function of the solve,
which decides pinning by structure (A11's subgroup algebra), not by a
slot's value. The argument should hold, but the build should state why.

## Built

`free_move_check` reads the space off an evaluation that answers the
document, `DocSession::space_evaluation`: the landed one when it answers the
committed document or the shown scratch. It refuses typed with `InTheWorld`,
`NotEvaluated` or `NoSpace`, and `MateConstrained` is gone. `space_evaluation`
accepts the scratch's landing too, because a value gesture leaves the node
graph alone and pinning is structural (the overlap argument at
`permitted_during_value_gesture` now says so). The one value-dependent answer
is a refused placement.

The probe slot and the `moves` map are keyed by root. `view` draws a frame over
every placement in its space, so a pattern's copies pinned to the world no
longer move with a probed instance, though a hide still reaches them.
`prune(doc, eval)` runs at the edit with no evaluation and again at the landing
with one. The landing's withdrawals wait in
`DocSession::take_landed_withdrawals` until the app shows them as notices.

The owed rows are `two_copies_mated_to_each_other_free_move_as_one`, which
also drops the frame when a world pin goes back on, and
`free_move_accepts_only_a_loose_space`, both in
`crates/viewer/tests/assembly_display.rs`. The fixtures probe after
`asm::unpin`, with `asm::where_b_stood` frames.

A frame whose space joins another loose space, because its root's copy was mated
onto another loose copy, goes at the landing with `MovedSpace`, which names
the space it now lives in. The probe still admits the copy to that space
(`a_frame_whose_space_joins_another_goes_saying_so`). An in-flight gesture is
re-asked by the copy it was opened on as well as by its root
(`gesture_check`): a delete kills it at the edit, and a split kills it at the
landing (`deleting_a_gestures_copy_kills_it_at_the_edit`,
`a_gesture_whose_copy_leaves_the_space_dies_at_the_landing`). The landing prune
runs only on a landing of the committed document, so a value gesture's preview
cannot discard committed display state. Review findings not
taken in this PR have items of their own:
`a-fused-instances-held-frame-has-no-reset`,
`sync-scene-landing-notices-have-no-row`,
`space-evaluation-compares-whole-docs-per-frame` and
`gallery-probe-may-probe-nothing-under-di5`.
