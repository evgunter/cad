---
id: bandmeter
kind: program
title: BANDMETER — the blend surgery's ring, cap and strip meters reading wider than they need
status: ready
opened: 2026-10-26
group: kernel/sweep/blend
prefix: bandmeter/
tag: (BANDMETER orchestrator)
ab_band: 18000-18099
paths: [crates/sweep/src/blend/surgery.rs, crates/sweep/src/blend/open/*]
keep_out: [opened 2026-10-26 by BAND's priority-seam split (work/README.md Track size) when BAND measured about 100 points against 30 - BAND keeps the P2 slate and the P4 residue, BANDRULE the shapes the band-feature rule would build, BANDREACH predicate 2's reach meter and support screen, all four share crates/sweep/src/blend by design, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**The surgery's meters, P3.** Before it carves, the blend surgery
meters what the band leaves in place against what it removes:
`ring_clearance_pass`'s arms (a) to (d) in `surgery.rs`, the rim's
`support_boundary_clearance`, and the cut-off's cap sliver
(`CapSliver`, `blend/open/end_face.rs`). Every row here is one of
them reading wider than the geometry:

- **The ring and strip meters.** An ellipse ring edge, a requested
  ruled ring edge, a ring wholly inside the strip read as crossing,
  the planar strip's bounding rectangle, and other carriers bounded by
  the whole carrier.
- **The cap sliver.** A curved edge read term by term, an oblique
  cap's inside reading charged the major semi-axis, a chord end's
  triangle with no outside verdict, the rim arcs' whole-circle arms
  with no cap row, and a co-requested open link read at its stored
  place.
- **The turn's overrun.** It is metered as the far band's whole
  cut-off sliver, and it refuses a short near edge.

Charter and order: `work/bandmeter/plan.md`; narrative in
`work/bandmeter/log.md`.
