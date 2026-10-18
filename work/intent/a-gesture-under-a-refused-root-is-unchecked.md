---
id: a-gesture-under-a-refused-root-is-unchecked
kind: issue
title: viewer: a free-move gesture on a copy whose space's root is refused is not asked about the root
status: open
opened: 2026-10-18
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
