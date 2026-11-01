---
id: select-all-edges-picks-edges-a-fillet-refuses
kind: issue
title: select all edges picks the tangent edges between coplanar faces, which the fillet then refuses
status: open
opened: 2026-10-31
priority: P0
design: true
---


## Finding

Found driving the viewer (2026-10-31; Ev: P0). A bracket whose upright boss is exactly as wide as its base plate, so their side walls are coplanar: the union keeps those walls as separate faces (the at-rest check reports each pair as an `unproven-coincidence`), and the edges between them are tangent. The blend tool's **Select all edges** (`editor_core::all_edges`, `crates/editor-core/src/names/mod.rs`, through `BlendTool::load_all_edges`) loads those edges too, so a 1 mm fillet of all edges fails: "the fillet op refused: an edge's supports meet tangentially, so its dihedral has no definite wedge side (margin 0e0 m (fillet3_convexity_sign decided zero …))". An ordinary part cannot be filleted all round in one click.

The design question: should **all edges** leave out edges whose supports meet tangentially (they have no wedge to blend), or should the union merge coplanar faces so the edge does not exist? The second runs into the coincidence door, which leaves these coincidences unproven when the two widths are two typed numbers (`work/chrome/an-unproven-coincidence-is-hard-to-resolve-by-a-variable.md`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`:

```text
click "New…"
click "document name"
type "bracket"
click "Create document"
click "Add feature"
click "Add profile…"
click "rectangle"
click "0.0100" 0
type "0.06"
key Enter
click "0.0100" 0
type "0.04"
key Enter
click "Add profile"
click "Add profile…"
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
click "Add profile…"
click "on the picked face"
click "circle"
click "0.0000" 1
type "-0.012"
key Enter
click "0.0100" 0
type "0.003"
key Enter
click "Add profile"
click "Add profile…"
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
click "Combine bodies"
click "Blend edges"
vclick 0.5 0.5
click "Blend tool…"
click "Select all edges"
click "Commit blend"
tree
click "View"
dump
```
