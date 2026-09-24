---
id: create-pane-hides-one-face-frame-fault-through-a-not-equal
kind: issue
title: pane/create.rs decides which FaceFrameFault the datum form draws with a != against one variant
status: open
opened: 2026-09-24
priority: P3
cost: E
---


## Finding

`crates/viewer/src/pane/create.rs`, the add-datum form's face-frame
seat (~`:514`):

```rust
if let Some(fault) = refused
    && *fault != FaceFrameFault::NoFace
{
    ui.weak(fault.to_string());
}
```

This is the `!matches!` policy shape spelled with `!=`: *which faults
the form draws under the seat*. `NoFace` is left out because the unmet
seat line above already says it. Every other `FaceFrameFault` draws,
including one added tomorrow that is ALSO already said somewhere else.
`crates/viewer/README.md`, *A policy over an enum names every variant*,
excludes this shape.

`chrome/subset-policy` (PR 3140) first classified it as identity, and
its review corrected that. It is fenced out of that lane because
`chrome/create-messages` works in the file. The fix is a `match` over
`FaceFrameFault` with `NoFace` as the silent arm and the rest named.
