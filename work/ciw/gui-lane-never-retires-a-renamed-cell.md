---
id: gui-lane-never-retires-a-renamed-cell
kind: issue
title: A render lane whose publish step overlays the old cells never shows a renamed or retired cell as missing
status: closed
opened: 2026-10-15
closed: 2026-10-16
priority: P3
cost: E
branch: ciw/render-lanes-publish-wholesale
---

Found by the CIW render-gui lane (PR 5205): its `[render]` dispatch (render.yml run
38816108307) passed `viewer gui montage`, and the re-baseline commit
`d3713cf944` added 7 cells under new names. The tour's assembly documents
now carry `PlaceInWorld` / `Part` nodes, so `shape_name` changed. The 8
cells under the old names stayed committed although no document produces
them any more; PR 5219 retired them with `git rm` (Ev, in chat: "please
do delete those orphans").

The cause is not the gui lane's alone. `.github/actions/rebaseline-lane`
refuses to commit a deletion, so it catches a missing cell only when the
lane's publish step removes the old cells before it writes the new ones.
Three publish steps existed and only one did that:

- `demos/render_gui.py` `main` copied each new cell over the old files;
- `demos/render.sh` `publish()` removed only the frames and side files of
  the scenes it had just rendered, so a renamed or retired scene left its
  old cell in the kernel and freecad lanes alike;
- `demos/render-wild.sh` cleared `*.png` before moving the pass in.

`demos/render-mc.sh` copied its listed sheets, so a sheet dropped from
`SHEETS` would also have stayed committed, and `demos/render-uv.sh` wrote
its one sheet straight into the lane, so a renamed sheet would have too.

**What the taker owes:** one publish step for every lane that replaces the
lane's committed cells with the pass wholesale, so a cell the pass did not
draw shows up as a deletion and fails the re-baseline, and a person
retires it by deleting the old file in a commit.

## Closed

Ev's ruling (chat, 2026-10-16): "it seems simpler to just see what that
job produced and error if it's missing any file that was there before, so
that someone can clear it by manually deleting the old file if it was
actually supposed to go." So nothing auto-deletes a committed cell.

`demos/publish_lane.py` is the one publish step, called by every lane:
`render.sh` (both lanes), `render-wild.sh`, `render-uv.sh`,
`render_gui.py` and `render-mc.sh`. It refuses a stage that is not
exactly the cells the pass was asked for, by exact file name,
then removes every committed cell file in the lane and copies the stage
in (a `.fail.txt` displacing its frame, a `.note.txt` beside it), then
runs the provenance guard over the lane. The action's `lost cells` error
lists the missing files one per line and says what to do: delete a file
that was meant to go and push again with `[render]`, or fix the render
that dropped a scene.
