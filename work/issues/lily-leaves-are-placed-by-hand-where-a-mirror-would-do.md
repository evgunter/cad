---
id: lily-leaves-are-placed-by-hand-where-a-mirror-would-do
kind: issue
title: The lily's three leaves are placed by hand; with a mirror door the scene can author one and mirror it
status: open
opened: 2026-10-22
priority: P4
cost: M
refs: [5607]
---


Found by `a-mirror-is-a-construction` (2026-10-22), which retired the lily's wall 5 (`demos/tour/src/lily.rs`, `wall_probes`): a leaf mirrored across the plant's plane now builds through `topo::mirror_body`, and the probe asserts that it does. The scene still places its three leaves by hand. Its retire note said "author leaves once and mirror them". Doing so is a scene change that moves the lily's frames (`[render]`). No program owns `demos/tour/src/lily.rs`.
