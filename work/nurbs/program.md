---
id: nurbs
kind: program
title: NURBS — the spline, net and fit doors: knot insertion and splits, the net-state reads, wide-parameter evaluation, and fit doors that refuse for reasons that are not geometric
status: ready
opened: 2026-09-20
group: kernel/geom/curves
prefix: nurbs/
tag: (NURBS orchestrator)
ab_band: 9300-9399
paths: [crates/geom-core/src/spline/*, crates/geom/src/curves/fit.rs, crates/geom/src/curves/nurbs.rs, crates/geom/src/surfaces/nurbs.rs]
keep_out: [opened by PROPS's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - PROPS measured 108.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are PROPS NURBS LINALG VERDICT and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, ENCL and STACK hold the certified-enclosure half, QUAD owns props/quad/, SSI and PCERT own the geom-brep certification lanes, PORT took the four refusal rows whose class is the crate boundary - since the 2026-10-26 tracker sweep it also holds KNOT's rows (KNOT was cut from FLUX on 2026-10-10 onto the same spline ground and merged in, ids kept), and crates/geom/src/curves/nurbs.rs and surfaces/nurbs.rs are shared with FLUX and its siblings]
---
The spline layer's own doors, split from the property arms that consume
them, and since 2026-10-26 the net and fit doors KNOT held beside them.
Wave 1 and wave 2 (2026-10-09 to 10-21) closed the hairline insertion,
the reversed-domain span meter, the per-split restrict compositions and
the loose-net pairing survivors (`log.md`).

What remains is three kinds of door that answer for a reason that is
not the geometry's:
- **Splits and elevation**: a split schedule that inserts a value twice
  on a narrow span, elevation past degree 10 with no bound, a weight
  power that underflows, a corner root with no reading.
- **The net's state and wide parameters**: `NetState` reading a net of
  infinities as described (and the curve with no net-state door at
  all), a curve's interval evaluation that loses the partition of
  unity, a surface derivative enclosure that grows with translation,
  an absolute projection epsilon that refuses a km-scale model.
- **The fit's doors**: a non-finite point passed through as NaN,
  unordered parameters refused as a count mismatch, a dense refit of a
  banded system.

Charter and order: `work/nurbs/plan.md`; narrative in `work/nurbs/log.md`.
