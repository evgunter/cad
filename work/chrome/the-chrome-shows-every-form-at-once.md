---
id: the-chrome-shows-every-form-at-once
kind: issue
title: the panes show every form and control at once, each drawn its own way
status: closed
opened: 2026-10-31
priority: P1
cost: H
branch: chrome/forms-at-rest
closed: 2026-11-01
pr: 6223
---


## Finding

Ev (2026-10-31): "there's not really any consistency in how things are displayed. also way too much stuff is displayed all the time — much of it should probably only be shown after you've already selected what you want to do."

Seen driving the viewer: the Properties pane carries the Mate tool, Add feature, Combine bodies, Blend edges, Add part and the variables row at once, beside the selection's own properties; the variables row's lone disabled **Create** reads as a heading and says why it is disabled ("pick a dimension first") only on hover; New… adds a second **Create** to the toolbar while it is open; and Cancel drag / Cancel free-move stand on the toolbar at rest (`work/chrome/the-cancel-buttons-are-always-on-the-toolbar.md`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
tree
click "New variable…"
dumpall
```

The extrude form is drawn only while a profile is selected, by its door's own kind test (`extrude_ui`, `crates/viewer/src/pane/create.rs`); that is the shape the other forms here can take: drawn where their op would accept the selection.

## Closed (2026-11-01, PR 6223)

At rest the Properties panel shows only doors. The creation sections (Add feature, Combine bodies, Blend edges, Assemble parts) fold, and inside each, every tool and form sits behind one button that opens it. A form whose op takes the selection is drawn beside the selection, while its op's own admission test accepts it: the extrude form appears beside a selected profile. Opening and closing forms has one home in `widgets.rs` (`door`, `form_door`, `commit_row`, `CANCEL`). A form closes on Cancel, or once the document accepts its commit, so a refused commit keeps what was typed. **New…** draws one Create, and the variable form says why its commit waits, in the open, from the same gate the commit uses. Folding a section closes its form, so a held face is released and its trace stops. The drive replays in open rows follow the doors.
