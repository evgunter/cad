---
id: four-freecad-frames-drifted-between-the-renders-of-11-13-and-11-15
kind: issue
title: Four committed freecad-lane frames (klein, lily, twisted_tube, the montage) differ from a hosted render of 2026-11-15, with the runner image changed in the same window: look before a nightly re-baselines them
status: closed
opened: 2026-11-15
priority: P3
cost: E
closed: 2026-11-22
---


Filed by GREEN's `green/nightly-1115` lane (PR 6985).

The render lanes were red on the nightlies of 2026-11-14 and 11-15
(`ubuntu-latest` moved to the 26.04 image, whose `python3` is 3.14; the
pinned matplotlib recurses there; PR 6985 pins 3.12 in `render.yml`).
The first render to complete since 11-13 is run 43708126820, on
`green/nightly-1115` at `31639630c`. Its kernel, wild, uv and mc lanes
matched the committed cells. Its freecad lane did not:

`demos/renders-freecad drifted from the committed cells`: `klein.png`,
`lily.png`, `montage-freecad.png`, `twisted_tube.png`.

The cells were not committed (the branch moved during the render; that
is the PR's red `render refused (freecad)` check), so nobody has looked
at them.

**Two candidate causes, neither checked.** Kernel merges on main
between 11-13 and 11-15 changed what the tour exports for those three
scenes; or the 26.04 image draws the same STEP differently (its mesa,
Xvfb and fonts changed with it, and the lane's byte-identity claim is
about one image). The kernel lane matching for the same scenes points
at the second, weakly.

**Owed.** Before the first nightly after PR 6985 re-baselines the lane
on main, download run 43708126820's freecad artifact and compare the
four frames to the committed ones by eye; if the difference is the
image's, say so on
`work/ciw/tool-versions-outside-the-env-block-have-no-source-of-truth.md`
(`runs-on: ubuntu-latest` floats) and decide whether the workflows name
an image.

**Seen again, and re-baselined, on BOXFINISH's `boxfinish/sweep-frames`
(PR 6972), 2026-11-15.** `render (demos)` run 43739282769 on
`66d8de2a22` (main merged at `a998d249cc`, so with the 3.12 pin) drew
the same four freecad frames apart from main's committed cells and the
bot committed them onto that branch (`7a4e77f208`); the kernel, wild,
uv and mc lanes matched or moved only in the uv sheet's labels. Against
`a998d249cc`, pixel by pixel: `klein.png` 2,798 of its 1,080,000
pixels differ (6 by more than 0.1), `lily.png` 70 (max 0.012),
`twisted_tube.png` 1,670 (max 0.004). Side by side the klein frame is
the same picture: the differences are isolated pixels along silhouettes
and seam lines. The same four frames moving on two branches whose
kernels differ (GREEN's at `31639630c`, this one with the sweeps'
sketch frames) is more weight on the image's
rasteriser than on the geometry; it is still not a check of the image, and
GREEN's run's bytes were not compared with these.

## Closed

2026-11-22, CIW triage (stale). The row was a look-before-re-baseline, and the re-baseline has happened: commit 80ee27d843 (2026-11-16, "render(freecad): re-baseline committed cells") committed the frames on main. The open question it leaves (`runs-on: ubuntu-latest` floats) is already recorded on tool-versions-outside-the-env-block-have-no-source-of-truth.
