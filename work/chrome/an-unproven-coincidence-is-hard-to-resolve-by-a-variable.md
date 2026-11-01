---
id: an-unproven-coincidence-is-hard-to-resolve-by-a-variable
kind: issue
title: an unproven-coincidence finding cannot be resolved in a couple of clicks by making the two sizes one variable
status: open
opened: 2026-10-31
priority: P2
cost: H
---


## Finding

Found driving the viewer (2026-10-31). A bracket's upright made exactly as wide as its plate raises six `unproven-coincidence` findings (one per coplanar side-wall pair). The door treats each typed number as its own symbol, so two widths typed equal are equal only by value, never proven (`two_variables_typed_equal_are_one_constant_to_the_analysis_lane_and_two_symbols_to_the_door`, `crates/editor-core/tests/coincidence_door.rs`); two widths read from one variable should be identically equal and proven. Not yet confirmed through the GUI.

Ev's direction (2026-10-31, P2): it should be easy to resolve an unproven-coincidence finding by making a variable, in a couple of clicks. Today the two widths are point coordinates in two separate profiles (the boss's in the picked face's coordinates), so making them one variable means rewriting both profiles' points as expressions by hand. The finding names both cells, so it is the natural place to offer "make these one variable".

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
click "checks: 8 finding(s)"
dump
```
