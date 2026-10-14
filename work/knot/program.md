---
id: knot
kind: program
title: KNOT — the spline, net and fit doors: refinement inside an enclosure, the net-state reads, and the NURBS and fit doors that refuse for reasons that are not geometric
status: ready
opened: 2026-10-10
area: kernel
prefix: knot/
tag: (KNOT orchestrator)
ab_band: 12300-12399
paths: [crates/geom-core/src/spline/*, crates/geom/src/curves/fit.rs, crates/geom/src/curves/nurbs.rs, crates/geom/src/surfaces/nurbs.rs]
keep_out: [opened 2026-10-10 by FLUX's priority-seam cut (FLUX measured 125.5 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and FLUX kept the curved closed-form arms. Shared ground is expected (Ev in chat 2026-09-20) - crates/geom-brep/src/props/* is shared with FLUX FLUXTAIL and QUAD, crates/geom-core/src/* with FLUX's siblings KNOT SCALAR and with FRAME NURBS SYM ENCL - run scripts/work.py territory on your branch and announce the seam in the PR]
priority: P2
---

**The doors a spline passes through on its way into a flux arm or an
enclosure.** An f64 knot refinement inside an enclosure (five sites of
TESS-2's class), knot insertion writing weights `validate_counts`
refuses, refinement copying the differencing skeleton, `NetState`
reading a net of infinities as described (and the curve side with no
net-state door at all), a NURBS derivative enclosure that grows with
translation, an absolute projection epsilon that refuses a km-scale
model, and the fit's refusals and dense solves.

Slate, order and posture: `work/knot/plan.md`; narrative in
`work/knot/log.md`.
