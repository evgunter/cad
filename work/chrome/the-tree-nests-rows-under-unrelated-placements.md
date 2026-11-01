---
id: the-tree-nests-rows-under-unrelated-placements
kind: issue
title: the feature tree's indentation does not follow the document's structure
status: review
opened: 2026-10-31
priority: P1
cost: M
branch: chrome/tree-structure
pr: 6218
---


## Finding

Found driving the viewer (2026-10-31; Ev: P1). After a plate, a face boss, a cylinder and a subtract, the `tree` reads:

```text
Profile "Profile 1" (bf935d4a4c62) Ok
Extrude "Extrude 1" (12b76f99b7dd) Ok
Profile 571b7fc9b723 Ok
Extrude 7d29928eef99 Ok
  Place d0da5efbc8a8 Ok
    Union "Profile 2" (d76139d3ff7f) Ok
Profile "Profile 3" (c05c7d9b118f) Ok
Extrude "Extrude 3" (b869bfd8c6d6) Ok
Place 087308912ecf Ok
    Subtract "Subtract 1" (a61a537ad329) Ok placed
    Place 6b878f3f462b Ok
```

`Subtract 1` and the bracket's world placement appear nested under the cylinder's `Place`, which they do not belong to; the indentation is `TreeRow::depth` ("how far the node sits below the document's sources", `crates/viewer/src/tree.rs`), a distance, so a row is drawn under whatever happens to precede it. The `Union` also carries the label typed for the boss's profile (`Profile 2`) while the profile has none. Ev plans a fancier tree view; this row is the defect it has to fix.

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
click "New…"
click "document name"
type "bracket"
click "Create" 0
click "Add feature"
click "rectangle"
click "0.0100" 0
type "0.06"
key Enter
click "0.0100" 0
type "0.04"
key Enter
click "Add profile"
click "Profile 1 Profile bf935d4a4c62"
click "0.0100" 0
type "0.005"
key Enter
click "Extrude Profile \"Profile 1\" (bf935d4a4c62)"
vclick 0.5 0.5
click "on the picked face"
click "rectangle"
click "0.0400" 0
type "0.005"
key Enter
click "0.0100" 0
type "0.03"
key Enter
click "Add profile"
click "on the picked face"
click "circle"
click "0.0000" 1
type "-0.012"
key Enter
click "0.0100" 0
type "0.003"
key Enter
click "Add profile"
scroll 1000 200 -600
click "Profile 3 Profile c05c7d9b118f"
click "0.0050" 0
type "0.02"
key Enter
click "Extrude Profile \"Profile 3\" (c05c7d9b118f)"
click "Add feature"
click "Combine bodies"
click "Boolean tool…"
click "subtract"
scroll 1000 200 -600
click "Profile 2 Union d76139d3ff7f »"
click "Extrude 3 Extrude b869bfd8c6d6 »"
click "Commit boolean"
tree
dump
click "View"
dump
```
