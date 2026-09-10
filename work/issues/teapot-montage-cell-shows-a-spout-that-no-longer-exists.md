---
id: teapot-montage-cell-shows-a-spout-that-no-longer-exists
kind: issue
title: the teapot montage cell renders a spout the code no longer builds
status: open
opened: 2026-09-10
---



`demos/renders/teapot.png`, `demos/renders-freecad/teapot.png` and both
montage sheets draw the teapot's spout as a straight cone frustum
tilted into place. The scene has not built that shape since #2286: the
spout is a `Node::Loft` through seven octagonal annular sections on a
circular arc's tangent frames — it bends, tapers and thins, and it has
18 faces where the frustum had 4.

So the committed pixels for a MONTAGE cell disagree with the code they
claim to picture, and the montage sheet is the curated contact sheet a
reader looks at first.

**Why it is not fixed in the PR that caused it.** Renders are produced
by `.github/workflows/render.yml`, the sanctioned renderer, and
committed by it directly (`render(kernel): re-baseline committed cells
[skip ci]`). The workflow is `workflow_dispatch` / `workflow_call`
only, and the agent that landed #2286 got `403 Resource not accessible
by integration` on the dispatch endpoint — so it could not take the
pass it needed. `demos/hosted-render-guard.sh` refuses a local pass by
design, and correctly: an off-hand local render puts one box's GL stack
into the repo's tracked pixels.

**What closes it.** A dispatch of `render.yml` against `main` with
`lanes: all`, which re-baselines the kernel and freecad cells for
`teapot` and re-composes both sheets. Nothing in the tree needs
editing.

**Two things worth reading while it is open, because neither is
obvious.**

1. **The drift is not silent, but it is not red either.** #2286's CI
   carried `render drift (kernel)` and `render drift (freecad)` checks
   and both came back **neutral**, which is what they do — they report
   rather than gate, so a scene can change shape and merge with its
   cell stale and nothing fails. That is a deliberate design (the
   renderer is not on the CI path), and it means the only thing
   standing between a shape change and a wrong montage cell is someone
   remembering to dispatch. This issue is that reminder for one scene;
   whether the general case wants more than a neutral check is a
   separate question and not this issue's to settle.
2. **Merging is what made it visible, not what made it wrong.** The
   cell was already stale on the branch — the canal landed in #2286
   itself — so there was no window in which a render could have been
   taken against a merged canal without merging first. The ordering is
   forced: merge, then dispatch.
