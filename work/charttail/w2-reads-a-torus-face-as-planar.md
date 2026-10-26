---
id: w2-reads-a-torus-face-as-planar
kind: issue
title: W2 on a torus face assumes the face is planar: a seamless torus face with a handle and null loops would lift wrongly
status: open
opened: 2026-10-24
priority: P3
cost: M
---


## What

`topo::boolean::ops::ChartCache::describes` reads W2's premise off the loop
walk (`pcurves::loop_windings`). The premise is that every closed curve in
`int F` winds nothing. The reading is: every loop off the apex winds the
identity, at most one loop meets it, and a cone face lies on one nappe. That rests on `F` being planar, so that every boundary circle
but one generates its first homology. On the cylinder and the cone, the
other kinds W2 is asked of, this is automatic, because the face lies on a
planar carrier.

A torus is not planar. A torus face could be the torus less a disc: one
null loop and no seam edges. Its loop walk winds nothing, yet it holds a
meridian and a longitude in its interior. W2 would clear an essential
section in it, which is a wrong boolean.

`chart_boundary(..).is_ok()`, the reading this replaced, has the same gap,
so nothing regressed. The faces the kernel builds on a torus are cut open
by their `u` and `v` seams into discs with holes. No constructor is known
to leave a torus face with a handle, and the one known to make a seamless
periodic face is the cylinder band, whose two loops each wind. "Known" is
not "cannot".

## Owed

Decide whether a torus face with genus is reachable at rest. If it is,
W2 needs a planarity reading of its own. For example, one loop's lift
bounds the face's side, which the face's sense can decide. If it is not,
make it unrepresentable or refuse it at rest. Found by CHART's
`w2-reads-a-one-sheet-predicate-off-the-walk` lane while proving that
reading. Not reproduced. (CHART implementer)

## Evidence (PR 5657's FULL review, 2026-10-24)

- A genus-1 torus face arriving by STEP import is refused at the import
  door by `CurvedSenseInverted` (`topo::validate`, the curved-sense check),
  in both of its loop encodings.
- The booleans keep "`int F` minus its edges is planar": no boolean leaves
  a torus face with a handle.
- The open route is a torus face bounded by non-iso edges. Validation's
  check 7 likely refuses it; this has not been run. A probe that builds
  one and reads its at-rest verdict closes or confirms the row.

Priority stays P2.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P3: no known constructor leaves a torus face with a handle: latent unsoundness, unreached by any known input, P3 by `work/README.md` Priority.
- Moved to CHARTTAIL (`work/charttail/`) by `git mv`, id and body unchanged: which holds CHART's rows since CHART folded in today: chart readings and the region doors. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
