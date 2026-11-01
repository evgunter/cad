---
id: the-chrome-shows-every-form-at-once
kind: issue
title: the panes show every form and control at once, each drawn its own way
status: open
opened: 2026-10-31
priority: P1
---


## Finding

Ev (2026-10-31): "there's not really any consistency in how things are displayed. also way too much stuff is displayed all the time — much of it should probably only be shown after you've already selected what you want to do."

Seen driving the viewer: the Properties pane carries the Mate tool, Add feature, Combine bodies, Blend edges, Add part and the variables row at once, beside the selection's own properties; the variables row's lone disabled **Create** reads as a heading and says why it is disabled ("pick a dimension first") only on hover; New… adds a second **Create** to the toolbar while it is open; the extrude form offers `Extrude <selection>` for any selection, so with a face of an extrude selected it reads `Extrude Extrude "Extrude 1" (…)` and its click is refused (`extrude_ui`, `crates/viewer/src/pane/create.rs`); and Cancel drag / Cancel free-move stand on the toolbar at rest (`work/chrome/the-cancel-buttons-are-always-on-the-toolbar.md`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
tree
click "Create"
dumpall
```
