---
id: teapot-scene-through-node-shell
kind: issue
title: Convert the tour's teapot vessel from the plane scan to Node::Shell (a render-lane change with tess-budget rows)
status: open
opened: 2026-09-06
---

`demos/tour/src/teapot.rs` builds the vessel kernel-direct: a
`revolve` of `vessel_meridian`, then `shell_open` with the mouth's two
faces found by a numeric plane scan (`plane_chart_at`), because no
document could name them. `Node::Shell` now can: the document spelling
is `crates/editor-core/tests/corpus/vessel.rs` — the same meridian,
station for station, then `Node::shell(pot, len(WALL),
[band(pot, SEG_MOUTH), band_pi(pot, SEG_MOUTH)])`, the mouth named by
ROLE and carried through a rebuild.

Converting the scene is a render-lane change with tess-budget rows
(`docs/TESS-BUDGET.md`; the tour's frames are committed evidence and a
changed frame is re-baselined with its reason, never restored), so it
is its own unit rather than a rider on the door. The lid and the spout
were always sayable (`docs/guide/north-star-audit.md` row 27); the
handle is `Node.tube`. After the conversion, rows 27, 44 and 45 of the
audit flip when a Python row executes the scene.
