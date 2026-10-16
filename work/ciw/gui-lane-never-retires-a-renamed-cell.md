---
id: gui-lane-never-retires-a-renamed-cell
kind: issue
title: The viewer gui lane never retires a cell whose document's shape name changed
status: open
opened: 2026-10-15
priority: P3
cost: E
---

Found by the CIW render-gui lane (PR 5205): its `[render]` dispatch (render.yml run
38816108307) passed `viewer gui montage`, and the re-baseline commit
`d3713cf944` added 7 cells under new names. The tour's assembly documents
now carry `PlaceInWorld` / `Part` nodes, so `shape_name` changed (it is
order-independent, so the node-id sort did not cause this). The 8 cells under
the old names stay committed even though no document produces them any more:
`datum-extrude-profile-{1230e269,56d118a1,7a53a443}.png`,
`gauge-instantiatepart4-mate3.png`, `instantiatepart.png`,
`instantiatepart-pattern.png`, `instantiatepart2-pattern.png` and
`instantiatepart3-mate2.png`, which has been orphaned since 2026-10-02.

`demos/render_gui.py`'s `main` writes this pass's cells into the lane and
never compares the lane against them. `.github/actions/rebaseline-lane`
deliberately refuses to commit a deletion, which is right for a dropped
frame. The cost is that a renamed cell leaves its old file behind for good.
`montage-gui.png` tiles only this pass's cells, so the sheet is right while
the directory is not.

**What the taker owes:** retire the 8 orphans with `git rm`. Then give the lane a
loud check that every committed cell (other than `montage-gui.png`) is one
this pass produced. That check should report rather than delete, in keeping
with the action's rule, so a rename shows up as an explicit retirement.

## 2026-10-16 — the 8 orphans are retired (Ev, in chat: "please do delete those orphans")

The 8 cells listed above are removed with `git rm`. Nothing else in the tree reads them: the one textual hit, `demos/check_render_provenance.py`'s selftest, writes its own placeholder PNG into a temp directory. Still owed: the loud check that every committed cell is one the pass produced. (CIW orchestrator)
