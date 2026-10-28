---
id: a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote
kind: issue
title: A full revolve's curved wall is built as two π-bands joined by two seam meridians the author did not write, against DESIGN's 'a wall keeps the cuts its author wrote'
status: open
opened: 2026-10-28
priority: P1
cost: M
design: true
---


Found by the SPLITDOOR cone-apex designer pair (2026-10-28), off their question; not measured
beyond what the probe shows.

## What

`docs/DESIGN.md` [topology]: "a wall keeps the cuts its author wrote: the kernel adds none, except
where a face or edge carrier is not C¹ by its knot structure". A `Full` revolve of a profile
segment builds its curved wall as two half-turn faces joined by two seam meridians (azimuths 0
and π): on the probe cone (`splitdoor/cone-apex-probe`, `zz_probe_cone_apex` with `ZZ_PROBE=1`)
the apex vertex has an orbit of two half-edges and two faces. Those meridians are cuts nobody wrote,
and the cone is C¹ across them. They are why a cone's apex has two half-turn sectors, and they
shape every split and Boolean that meets a revolved wall.

Related: `work/tcost/germ-cone-doc-claims-one-merged-face-where-two-half-bands-stay.md`.

## Open

Whether the two-band build is a sanctioned exception (closed-surface parametrisation needing a
seam) that the clause should state, or a cut the kernel should stop adding. Either way the clause
and the build disagree today.
