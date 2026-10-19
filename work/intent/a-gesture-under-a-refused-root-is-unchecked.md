---
id: a-gesture-under-a-refused-root-is-unchecked
kind: issue
title: viewer: a free-move gesture on a copy whose space's root is refused is not asked about the root
status: closed
opened: 2026-10-18
closed: 2026-10-18
priority: P3
cost: E
---


Split out of `free-move-landing-prune-gaps-have-no-row`. The review of the PR
that closed that item pointed out that the question was answered by reading
only, never run.

`gesture_check` (`crates/viewer/src/display.rs`) asks a landing whether the
gesture's own copy still lives in the root's space (`in_space`), and asks
whether the root still exists. It never asks whether the root's own placement
evaluated. `spaces_of` (`crates/editor-core/src/eval/mod.rs`) gives a space to
a failed node too. So if a root refuses while a copy mated onto it does not,
the gesture survives a landing that would drop a frame keyed by that root
(`space_check` answers `NoSpace`).

By reading, a copy whose mate reads a refused root's pose refuses as well, and
the copy check then answers `NoSpace` first. To close this: build a shape whose
root refuses (for example, a root instance whose part stops resolving) under a
gesture on a mated copy, and check what the landing does. If the copy survives,
ask `in_space(.., root, root, ..)` too.

## Closed: the premise holds, by execution (2026-10-18)

`a_gesture_under_a_root_that_comes_to_refuse_dies_at_the_landing`
(`crates/viewer/tests/instance_authoring.rs`) builds the shape. post_b is
seated onto the loose shelf, which roots post_b's space. The shelf's file then
moves on, so the saved reference refuses as a pin mismatch, and accepting the
new version resolves it. A gesture opens on post_b, and undoing the accept puts
the refusal back under it. At the landing the shelf has no value, and neither
does post_b, because its mate reads the root's pose. The gesture's own-copy
check therefore kills it with `NoSpace` naming post_b. A refused root needs no
question of its own.
