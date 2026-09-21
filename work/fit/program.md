---
id: fit
kind: program
title: FIT — the camera, the display budget and the pick index: what the viewer decides to show and how much of it
status: open
opened: 2026-09-20
area: gui
prefix: fit/
tag: (FIT orchestrator)
ab_band: 8500-8599
paths: [crates/viewer/src/camera.rs, crates/viewer/src/pickindex.rs, crates/viewer/src/pickcache.rs]
keep_out: [opened by VGEOM's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - VGEOM measured 43 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are VGEOM FIT and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, CHROME and AUTHOR own the forms and panels, VSEAM owns what the viewer holds for the document, VDOC owns the prose]
priority: P1
---
What the viewer decides to SHOW, as opposed to the arithmetic it shows
it with. Two of its rows are things a person hits in the first minute:
**ctrl+wheel reaches no zoom at all** — the toolkit routes it into
`zoom_factor_delta` and the viewport reads only `smooth_scroll_delta` —
and **zoom-to-fit frames bodies only**, so a document holding only
profiles has nothing to frame.

Behind them, three rows say the budget and the index are not what they
claim: the display budget is not a cap, because the 1/delta law's
two-sided error puts the drawn picture OVER it; the display fit reads a
body flat across one rung step as flat at every finer delta; and
`pickindex` merges parts on a rounded `t` it never converts, slacked by
a tuned 1e-6. A failed id readback is reported as an empty cursor, so
the chrome blames the picture for a readback fault.

Charter and order: `work/fit/plan.md`; narrative in `work/fit/log.md`.
