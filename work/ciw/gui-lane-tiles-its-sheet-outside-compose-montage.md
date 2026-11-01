---
id: gui-lane-tiles-its-sheet-outside-compose-montage
kind: issue
title: The viewer gui lane tiles its sheet with ImageMagick montage rather than compose_montage.py
status: open
opened: 2026-10-16
priority: P3
cost: M
---


Every PNG lane but one composes its sheet with `demos/compose_montage.py`.
`demos/render_gui.py` `main` instead shells out to ImageMagick
(`montage ... -tile 2x -geometry +8+8 -background #1b1b1b`) and then
stamps the sheet with its own `stamp_author`. Found while
`demos/publish_lane.py` unified the lanes' publish step (PR for
`gui-lane-never-retires-a-renamed-cell`), and not folded there, because
the two differ in what they are, not only in spelling:

- **What lays the grid out.** `compose_montage.py` reads
  `<outdir>/scenes.json` through `manifest.read_scenes` for order,
  captions and the montage flag. The gui lane has no `scenes.json`: its
  cells are named by `cell_names` from the assembly store's documents, so
  the composer needs a manifest-free input before it can tile them.
- **Captions and title.** `compose_montage.py` draws a matplotlib title,
  per-cell captions and an optional banner. The gui sheet has none; the
  window's title bar inside each shot is its only label.
- **Provenance.** A `compose_montage.py` sheet is matplotlib-authored and
  exempt by name in `check_render_provenance.py` (`SHEETS`, read back from
  each script's `--montage=`). The gui sheet is stamped with the lane's
  `Author` and checked like a cell (`GUI_LANE`), so moving it to the
  composer changes the guard's rule for that directory, not just the
  composer.
- **Dependencies.** The gui lane runs on the system `python3` with no venv;
  `compose_montage.py` needs the pinned numpy + matplotlib venv.

What the taker owes: decide whether the gui sheet should be a
`compose_montage.py` sheet (manifest-free mode, a caption per cell, the
gui sheet joining `SHEETS` or keeping its stamp) and, if so, drop the
`montage` call and the ImageMagick package's montage role
(`demos/render-gui.sh` header).

`render_gui.py`'s `stamp_author` and `render.py --author` are not a pair
to fold: `render.py` asks matplotlib's `savefig(metadata=)` to write the
`Author` chunk, and `stamp_author` inserts it into a PNG that ImageMagick's
`import` wrote. The byte-level chunk encoder they could have shared was the
real copy, and `render_gui.py` and `check_render_provenance.py` now import
`strip_png_stamps.encode_chunk`.
