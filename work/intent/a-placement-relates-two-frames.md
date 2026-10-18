---
id: a-placement-relates-two-frames
kind: issue
title: A placement relates two frames: mates fix what they fix, values set the rest (all six when unmated); a construction's coordinates are its space; an instance places another document's space
status: open
opened: 2026-10-18
priority: P0
cost: M
needs_ev: true
refs: [transform-retires-into-a-placement, intent-stage3-is-built]
---



Ev, 2026-10-18 (in chat, with a diagram of a DAG of frames): a placement relates the copy's frame to its target frame. Mates fix what they fix; values give the freedoms the mates leave, all six when the parts are not physically connected; rotations are in the target frame's axes. "Parts just don't have a location" means "you need to explicitly specify how they relate to each other". One document's world, or any other frame, can be embedded into another document as a non-world frame to make an assembly.

D10 `[placement]` said the opposite in four places ("relating two raw coordinate systems cannot be written", "a value names its mate", a lone point mate's rotation has no chart, a placement reads a list of shapes). The restatement is in the `[ev]` PR that opens this item. It is fork-log row 162, weighed by an Opus and a Fable designer.

When ratified, the work that follows (each a unit, sequenced after this):
- spec `docs/INTENT-STAGE3-SPEC.md` §1, §4 and §5 restated to match;
- stage 3 D pivots `stand` to an unmated placement with values, and `mate::anchor::Anchors`, the variable-free anchor search and the Axis-mate fallback retire;
- C's world pin (`WorldPin::Identity`, `PinWhereItStands`, `world_pin_stands_alone`, `WorldPinOnAMovedWorld`) retires into the identity placement;
- the unmated six-value chart is built;
- `Value.mate` becomes optional, and F's subgroup rule counts spins by axis;
- `[evgunter/cad#4748]` (instance outputs) closes as subsumed.
