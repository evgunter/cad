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

## Narrowed (2026-11-09, `onecopy/prose-census-red`)

The census now also reads every inherent method taking `self` and a `Formatter`, whatever its name (`write_renderer_bodies`). That covers the `fn write` that `geom_brep::ended_by_write!` wraps, `write_payload` delegations, and renderers with no `Display` (`AsShelled::write`). The planted row `a_sentence_written_in_an_inherent_formatter_method_is_read` reds if one is dropped. PR 6430 had taken the census from over 200 renderings to 134 this way (`work/onecopy/main-red-prose-census-reads-no-write-renderer.md`).

**What remains of this row:**
- a sentence trait under another name, still found by its head;
- a free function writing into a `Formatter`. None renders a `Debug` placeholder today; the check is stated in the module doc.
