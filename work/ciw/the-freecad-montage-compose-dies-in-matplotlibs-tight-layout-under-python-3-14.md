---
id: the-freecad-montage-compose-dies-in-matplotlibs-tight-layout-under-python-3-14
kind: issue
title: The render's freecad lane dies composing its montage: matplotlib's tight_layout raises under the runner's Python 3.14, so no [render] publishes the kernel or freecad cells
status: open
opened: 2026-11-15
priority: P2
cost: E
---

Found by BOXFINISH's `boxfinish/sweep-frames` (PR 6972), whose fix pass
asks for a render on its final head.

`render (demos)` runs 43658737118, 43732262752 (both on that branch) and
43708126820 (GREEN's `green/nightly-1115`) fail in the job "freecad
montages (kernel + freecad)", step "render both lanes
(demos/render.sh)". After every cell is drawn and published
(`publish_lane: 53 cell(s) -> renders`, provenance checked),
`demos/compose_montage.py:133` (`fig.tight_layout(pad=0.4, rect=...)`)
raises `RecursionError: maximum recursion depth exceeded`: matplotlib's
tight layout updates an axis's ticks, `MarkerStyle._set_marker`
deep-copies a marker's `__dict__`, and `Path.__deepcopy__`'s
`copy.deepcopy(super(), memo)` recurses under the runner's Python 3.14
(`/usr/lib/python3.14/copy.py`). The venv pins `matplotlib==3.10.3`
(`demos/render.sh:109-112`), whose `Path.__deepcopy__` predates 3.14's
`copy` changes. So no `[render]` re-baselines the kernel or freecad
lanes; the uv, gui and wild lanes are unaffected.

What closes it: the render venv on a Python and matplotlib pair that
composes (a matplotlib release that supports 3.14, or the venv on a
pinned older interpreter), and a `[render]` that publishes both lanes.

