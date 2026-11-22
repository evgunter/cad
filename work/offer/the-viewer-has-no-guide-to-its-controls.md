---
id: the-viewer-has-no-guide-to-its-controls
kind: issue
title: viewer: the viewer shows no guide to its mouse and keyboard controls, so pan is undiscoverable
status: closed
opened: 2026-11-22
priority: P1
cost: E
pr: 7336
branch: offer/the-viewer-has-no-guide-to-its-controls
closed: 2026-11-22
---

**Ev reported this** (in chat, 2026-11-22): "no user guide for the controls (and i don't know how to pan)".

The bindings are in `crates/viewer/src/input.rs`: the module doc at `:20-49` and the defaults at `:259-263`. Orbit is on middle-drag or alt+primary, pan on secondary-drag or shift+middle (`:334-339`), and the wheel zooms by 0.1 per notch (`:358`+). The only written guide is the "Mouse bindings" table in `crates/viewer/README.md:21-30`, with prose at `:39-43`. That is a developer page no user of the viewer sees. Nothing in the running viewer names a binding: no help pane, overlay, menu entry or hover hint mentions orbit, pan or zoom.

The fix is a controls guide reachable from inside the viewer (a Help menu entry or `?` overlay, say), generated from the same binding table `input.rs` reads, so it cannot drift from the real bindings. The README table should then point to it or be generated from the same source. A wider user guide (what each pane and tool is for) could grow from the same door, but this row is the controls.

## Closed (2026-11-22, PR #7336)

The toolbar's **Controls** button opens a window that lists every pointer gesture the viewport honours. The window reads the same `InputMap.bindings` table that `InputMap::map` and `pick` read. Rows that cannot drift pin it: every row is honoured, every honoured gesture is a row, the wording is tied to the row's fields, and the window holds exactly the table. The README's bindings table is checked against the source.

One part did not land: the wheel's modifiers are still decided in the adapter. That is `the-wheels-modifier-decisions-are-outside-the-binding-table` (P2).
