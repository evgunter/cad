# Phase 2: what moved when every conic point was summed from its zero point

Local runs on `oneround/anchored-conic` (stacked on `onehome/carrier-frames` at
3d1ccd39d), `--profile ci`. Evidence for the question the unit's PR puts to Ev.
The unified-chain code is the branch `oneround/anchored-conic-one-chain`
(commit 801623f7b).

- `moved-run1-…txt`: 12 of 6,969 (geom, geom-brep, topo) at commit 9a2cd8368:
  the unified chain alone, in its first form (`m = 2·sin(θ/2)`, `−m·sh`,
  `m·ch`), azimuth read from the zero point on every conic.
- `moved-run2-…txt`: 29 of 10,866 (adds sweep, verbs, mesh, step-export,
  step-import) with the cone mint, a cylinder mint at "the vertex nearer the
  reach", and `RESOLUTION_ROUNDINGS` 40 also in.
- `payloads-run3.txt`: those 29 re-run after the offsets became
  `(−2·sin²(θ/2), sin θ)` and a centre-born conic's azimuth went back to its
  centre: 26 still move.
- `payloads-run4.txt`: after the cylinder mint came out: 22 still move
  (commit 801623f7b, the one-chain branch).
- `payloads-run5.txt`: with the chain following the born form: 2 move, both
  from the cone mint.
