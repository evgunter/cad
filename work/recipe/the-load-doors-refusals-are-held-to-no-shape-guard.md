---
id: the-load-doors-refusals-are-held-to-no-shape-guard
kind: issue
title: persist: the load door's PersistError refusals are held to no shape guard, and Unreadable renders two stage prefixes and an unlabelled recourse
status: open
opened: 2026-10-02
priority: P3
cost: M
---

(RECIPE, found by `a-metadata-value-nested-deep-enough-kills-the-process`'s pin, PR 3909.)

## What

No suite holds `PersistError`'s `Display` (`crates/editor-core/src/persist/mod.rs`) to `test_utils::refusal::problems`: the editor-core and viewer `refusal_concision*` rosters render `EditError`, the feature tree, the refactor doors and the at-rest checks, and none renders a load refusal. Measured on a file holding a metadata value one past its bound (`crates/editor-core/tests/meta_nesting_bound.rs`, `a_file_nested_past_the_bound_refuses_on_the_smallest_stack`), `PersistError::Unreadable` renders:

> persist: this build cannot read the document (body line 760 column 13: the metadata value nests deeper than 128 levels of lists and maps at line 760 column 13) — regenerate the file from its source recipe with a current build (every saved document replays from source; this kernel is unreleased and writes no old-format files)

and `problems` reports three faults: the stage prefix `persist:`, the stage prefix `body line 760 column 13:`, and no recourse (the regenerate advice is unlabelled and is not one of `BARE_RECOURSES`). The place is said twice, too.

## What would close it

A load-door roster over every `PersistError` arm (and `SnapshotError` through `PersistError::Snapshot`), rendered as the edit roster renders `EditError`, with the arms rewritten to the standard or admitted by exact id under a filed row. `a-bad-label-in-a-file-refuses-with-the-regenerate-recourse` (EMIT) is one arm whose recourse is wrong rather than missing; read it first.
