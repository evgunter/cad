---
id: render-gui-sorts-ordinal-digest-node-ids-as-ints
kind: issue
title: render_gui.py sorts snapshot node ids with key=int, but PR 4244's ids are ordinal:digest, so the viewer GUI montage lane crashes
status: open
opened: 2026-10-08
priority: P1
cost: E
---


Found by BAND's rocker lane (PR 4287): its `[render]` dispatch (render.yml
run 37703953592) failed the `viewer gui montage` lane before drawing a
cell:

    File "demos/render_gui.py", line 109, in node_kinds
        for key in sorted(nodes, key=int):
    ValueError: invalid literal for int() with base 10: '8:a8f6d65b88ab59f9'

PR 4244 (`emit/ordinal-ids`, merged 2026-10-07 14:46 UTC) mints node ids
that display as `ordinal:digest` (`crates/editor-core/src/mint.rs`, the
id's `Display`: `"{}:{:016x}"`), and a saved `.pncad` snapshot keys its
`nodes` map by that text. `demos/render_gui.py`'s `node_kinds` still
sorts the keys with `int`. The last nightly (11:49 UTC) predates the
merge, and this was the first `[render]` dispatch after it, so from now
on every dispatch, and the next nightly's render pass, fail this lane. The kernel and FreeCAD
lanes are unaffected.

**What the taker owes:** sort by the id's ordinal, i.e. the integer before
the `:` (the order `node_kinds` relies on is creation order, which the
ordinal carries), or read whatever order the snapshot itself declares.
Fail loud on a key with neither form, as the function's docstring
already requires. Then re-render the lane and check that its cell names
are unchanged.
