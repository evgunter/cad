---
id: lever-a-declared-pair-by-its-contact-patch-not-both-whole-faces
kind: issue
title: A declared pair is levered over both whole faces, so a small part on a large plate refuses a tilt its contact patch holds in band
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## What (design note)

The declared `Rest` door reads a pair as one displacement over its
consumed extent (`carrier_eq::declared_reading`), and the extent is the
ball enclosing BOTH WHOLE FACES (`rest::pair_extent`). The tilt term is
levered at that ball's reach. A small part resting on a large plate —
a 10 mm puck on a 2 m bed — therefore has its tilt levered at the
plate's ~1.4 m, not at the puck's 10 mm footprint where the two faces
actually meet. A tilt above `Kε / plate size` reads past the band:
the door refuses (unsettled, or contradicted where a plate vertex
stands off), even though every point of the actual contact patch is
in band and the contact is sound.

The verdict is consumed on the contact patch (the overlap region the
crossing layer and the REST zip consume), not on the parts of either
face the other never touches. Levering by both whole faces is sound
(it only refuses more), but over-refuses exactly the "small part on a
large base" assemblies the declared door exists for.

## The fix's shape

Lever by the consumed contact patch: a ball enclosing the overlap of
the two faces (the chart-region overlap `chart_region` already computes
for `Rest`, or a cheaper sound superset: the intersection of the two
faces' boxes, which encloses the overlap), and the witnesses restricted
to the boundary points inside it. This changes which faces' extent a
verdict is read over, so the declared rows (`rest::lever_rows`,
`contact9_side_codes`' wedge rows) need re-measuring with it. Filed
from the fix pass of PR 3795 (item 6b).
