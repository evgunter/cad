---
id: viewer-gui-montage-parses-a-node-key-as-an-int
kind: issue
title: render.yml's viewer gui montage dies in demos/render_gui.py on int('8:a8f6d65b88ab59f9'): the node-key format changed under it
status: open
opened: 2026-10-09
priority: P3
cost: E
---

Seen by NURBS on 2026-10-09. The `viewer gui montage` job of `render.yml`, dispatched on the `nurbs/refine-dir-hairline` branch (PR 4438), failed in `demos/render_gui.py` with `ValueError` on `int('8:a8f6d65b88ab59f9')`. That branch touches neither the viewer nor the script. The key has the shape `<n>:<hex>`, which looks like the id form INTENT stage 2 introduced (PR 4342 `operands-are-reads`, merged 2026-10-09), so the likely cause is a parse in the script, or a viewer read it calls, that still expects a bare integer node id. Nobody has reproduced this on main's own render run yet. First step: find the `int(` that receives the node key (it may be in a helper the script imports rather than in `render_gui.py` itself), and check main's latest `render.yml` run. (NURBS orchestrator)
