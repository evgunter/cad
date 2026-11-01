---
id: a-picked-face-is-named-by-its-placement
kind: issue
title: a picked face is named by its placement and body index, not by what it is
status: open
opened: 2026-10-31
priority: P2
---


## Finding

Found driving the viewer (2026-10-31; Ev: P2). Clicking the top face of a plate and ticking **on the picked face** shows `face: Place 6b878f3f462b body 0`: the placement's tag and a body index, not which face it is (the plate's top). The selection's Properties header says `face of Extrude "Extrude 1" (…)`, also without saying which face.

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
dump
```

## The mate tool's held picks

Second walk (2026-10-31; Ev: P2, "maybe it could highlight the face if you hover over the selection"). The mate panel reads `on: face of InstantiatePart "InstantiatePart 1" (…); to: face of …`. A pick aimed at a plate's top face landed on its side face, and the mate stood the plate on its edge; nothing in the panel said which face was held. A good name may not exist, so the proposal is to light the held face in the viewport while the pointer is over its line in the panel.
