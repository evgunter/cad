---
id: quadtail
kind: program
title: QUADTAIL — the quadrature lane's P2–P4 tail: enclosure looseness, missing volume rules and the tests over them
status: ready
opened: 2026-10-21
group: kernel/props
prefix: quadtail/
tag: (QUADTAIL orchestrator)
budget: 32
paths: [crates/geom-brep/src/props/quad/*]
keep_out: [opened 2026-10-21 by QUAD's priority-seam cut (QUAD measured 72.5 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and QUAD kept its P0 and P1 rows. Shared ground is expected (Ev in chat 2026-09-20) - crates/geom-brep/src/props/quad/ is shared with QUAD and with SPANFOLD and SPIRICARM (FLUX's successors) - run scripts/work.py territory on your branch and announce the seam on the other program's log]
---

**The quadrature lane's tail.** QUAD keeps the engines, the budget's
shape, the dial and the rows where a measured value depends on where
the body sits. The rows here are what is left at P2 to P4: enclosures
looser than they need be (a 90° rational span, a micron wall's
map-residual pad, an unpadded junction gap), faces with no volume rule,
an area gauge that panics instead of refusing, and tests that assert
less than they could or hold a midpoint to a fixed tolerance.

`budget: 32`, because the cut's whole P2–P4 residue weighed 30.5
points, and a half-point overage does not justify a third track.

Slate, order and posture: `work/quadtail/plan.md`; narrative in
`work/quadtail/log.md`.
