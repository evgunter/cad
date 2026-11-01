---
id: prose-census-finds-sentence-impls-by-their-trait-names
kind: issue
title: prose_census finds the impls it reads by the literal trait names Display and Say
status: open
opened: 2026-10-02
priority: P4
cost: E
---

Found by PR 3782's review (`memoized-refusals-speak-inner-nodes-through-the-frame`).

`census` in `crates/pncad-py/src/prose_census.rs` picks the bodies it reads by matching the literal text `"Display for "` and `"Say for "` within 200 characters after an `impl`. When PRs 3760 and 3782 moved sentences from `impl Display` into `impl crate::spoken::Say`, the census stopped reading them. Nothing failed: `NamingError`'s positional `{:?}` site simply dropped off the `UNDECIDED` list. PR 3782 added `"Say for "` to the match. The next sentence trait, or a renderer that writes prose outside either trait (a `fn render`, a `struct …(…)` with its own `Display` named by alias), will drop out the same way, and silently.

A repair would find renderers by what they do rather than by the trait's name. One option is every `impl` whose body holds a `write!`/`format_args!` into a `Formatter`. Another is a census assertion that every type implementing `editor_core::spoken::Say` is also read. Either way, add a planted case so a dropped impl fails.
