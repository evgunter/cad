---
id: the-selection-after-a-subtract-is-the-removed-body
kind: issue
title: after a subtract commits, the selection is the removed body rather than the result
status: open
opened: 2026-10-31
priority: P3
---


## Finding

Found driving the viewer (2026-10-31; Ev: P3, "easy to just click again"). Committing a subtract (keep the bracket, remove a cylinder) leaves the selection on the removed body, `Extrude "Extrude 3"`, which is no longer drawn, rather than on the new `Subtract`. A next action on "the selection" acts on a body that is gone; the blend tool's **Select all edges** is disabled for it.

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
