---
id: committing-a-mate-floods-the-banner
kind: issue
title: committing a mate floods the banner: a refused label and raw-id dropped-value lines
status: open
opened: 2026-10-31
priority: P1
---

## Finding

Found driving the viewer (2026-10-31; Ev: P1). Committing a plane mate between two instances of one part puts five notices on the banner at once:

- `label: this form creates no node to label`. The mate form offers a label field (`Mate 1`), and `ViewerBehavior::push_labelled` (`crates/viewer/src/pane/create.rs`) refuses it because a mate edits a placement and mints no node. The form should not offer a label it cannot use, or the label should land somewhere.
- Four lines of `InstantiatePart "InstantiatePart 1" (…)'s slide along z read #3:9c848cad38ef0d85; the bundle written whole does not leave that freedom, so this edit dropped the value` (`EditFault::ValueDropped`'s `Display`, `crates/editor-core/src/edit.rs`), one per freedom the mate replaced. The variable is spelled as a raw id, and the four say one thing: the mate replaced the copy's free slides and spins.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
save <dir>/plate.pncad
click "New…"
click "document name"
type "assembly"
click "Create" 0
save <dir>/assembly.pncad
click "Add part…"
click "add" 2
click "Add part…"
click "add" 2
click "InstantiatePart 2 *"
click "0.0000" 2
type "0.03"
key Enter
click "Mate tool…"
vclick 0.18 0.575
vclick 0.25 0.4
click "Commit mate"
settle
picture shot.png
```

`<dir>` must be an empty directory.
