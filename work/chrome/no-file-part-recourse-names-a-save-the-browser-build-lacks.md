---
id: no-file-part-recourse-names-a-save-the-browser-build-lacks
kind: issue
title: viewer: on wasm32 a part in a document with no file states 'save it beside its parts', and the browser build has no save door
status: deferred
opened: 2026-09-29
priority: P3
cost: E
---


(EDIT, found by `edit/part-refusal-recourse`'s fix pass, PR 3492.)

## What

A viewer session with no backing file resolves its parts through
`crates/viewer/src/docio.rs` `NoFile`, whose refusal ends
"Recourse: save it beside its parts". On the native build that is the
way through: `SessionOp::Save` binds a `DirResolver` over the file's
directory (`crates/viewer/tests/instance_authoring.rs`,
`every_unresolved_part_badge_meets_the_refusal_standard`, follows it).

On wasm32 the file chooser is `Absent` (`frame::chooser_backend`, and
the `rfd` note in `crates/viewer/Cargo.toml`), and the browser has no
directory beside which to save. The recourse names an act the browser
build cannot take. Not verified: no wasm row renders the badge.

## What would close it

The browser build's own way through, stated where `NoFile` states the
native one (a `cfg(target_arch = "wasm32")` arm, or a resolver the
browser session carries instead), or the shared no-way-through ending
if the browser has none, with a row that renders the badge there.

## Deferred (Ev, in chat, 2026-10-01)

"idk if it makes sense to change the browser's error messages when
presumably it will have save added before it's released." The browser
build is expected to gain save before release, and the recourse will
then be true as written. Not-now; revisit if the web build ships
without save.
