---
id: the-python-frame-constructors-feed-no-api-that-admits-a-frame
kind: issue
title: pncad-py's Frame constructors build a frame no Python API consumes, and their docs promise an edit door that admits it
status: open
opened: 2026-11-10
priority: P3
---


Filed by PR 6717 (ONEHOME rigid-motion, fork 258's review).

`pncad.Frame` has three static constructors (`crates/pncad-py/src/py/place.rs:198`, `:213`, and the `turns` constructor below them): `translation`, `rotate_then_translate` and the fraction-of-a-turn form. Each builds an `editor_core::Frame`, and no Python API takes one as an argument. The class is only ever returned, as a placement's pose read back (`crates/pncad-py/src/py/mate.rs:973`, `:1002`; `crates/pncad-py/src/py/value.rs:2414`).

The class's docs, both the Rust doc comment at `place.rs:180` and `crates/pncad-py/pncad.pyi:1971`, say that "a frame that is not a rigid motion is refused at the edit door". No edit door admits a frame: placements are mate bundles, and the only frame doors are the viewer's layout and gesture (`Frame::admission_fault`).

What is owed (pncad-py's lane): either
- remove the constructors and say the class is read-only; or
- name the API that consumes them, and correct the docs to the door that admits it.
