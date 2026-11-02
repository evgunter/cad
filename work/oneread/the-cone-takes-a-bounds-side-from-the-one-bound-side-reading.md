---
id: the-cone-takes-a-bounds-side-from-the-one-bound-side-reading
kind: unit
title: bound_side reads both orders on the caller's Reading (band + tie slack), and the Cone admits a given side for any sector unless its own metric decided against it
status: dispatched
opened: 2026-11-02
priority: P1
cost: M
branch: oneread/cone-given-side
parent: the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading
---


Items 2 and 3 of `the-line-cone-reads-a-flanks-side-beside-the-one-bound-side-reading`'s `## Decided`, which is the spec. Also closes the main defect both designers found: today `Cone::reads_given` takes the held plane's side unconditionally and `PierceDatum::side` reads it on the plain band, so at recl's edge-edge site a held-face side decided within the tie of zero bypasses the cone's slack guard (`Cone::reads_given`'s `given` arm; `resolve_edge_edge`'s `given` construction). Tier: dual (the one reader every consumer reads).
