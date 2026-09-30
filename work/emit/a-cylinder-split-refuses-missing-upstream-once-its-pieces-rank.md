---
id: a-cylinder-split-refuses-missing-upstream-once-its-pieces-rank
kind: issue
title: A cylinder split whose same-side pieces are let through refuses MissingUpstream on the extrude
status: open
opened: 2026-09-30
priority: P0
cost: M
---


## What

A cylinder (`circle(0, 0, 0.5)` extruded 1.0) split by a plane that
crosses its wall twice (through `(0, 0.2, 0)`, normal `(0, 1, 0)`)
refuses today at the split ranker's `face_plane`
(`a-plane-split-of-a-curved-face-refuses-as-an-emission-bug`). A
designer lane on the curved-seam fork (fork-log row 22) let the
same-side pieces through as a tie in a scratch probe (not committed);
the split then refused `MissingUpstream { node: <the extrude> }`.

So a second, undiagnosed defect sits behind the first: whatever rule
the fork settles for the split's same-side pieces, this recipe will
still refuse. Root-cause it independently of the fork: reproduce by
tying the same-side group in the split fragment ranker, find which
lookup expects an upstream name the extrude never published.
