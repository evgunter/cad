---
id: pcert
kind: program
title: PCERT — pcurve certification and what validate_pcurves actually checks
status: open
opened: 2026-09-20
area: kernel
prefix: pcert/
tag: (PCERT orchestrator)
ab_band: 7600-7699
paths: [crates/geom-brep/src/pcurve_cache.rs]
keep_out: [opened by CHART's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CHART measured 105 budget points and was cut into four tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CHART SSI PCERT ISO and they share crates/geom-brep by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TRIM keeps nurbs_iso.rs and docs/PCURVE-P2-SPEC.md until its exit, SHELL owns editor-core/src/clearance.rs, TESS and CHORD own crates/mesh, QUAD owns props/quad.rs]
priority: P0
---
**A certifier that answers a clean bill on a body whose certification
failed.** `validate_pcurves` is the door the rest of the kernel reads
to know a body's pcurves are sound; three of this program's rows say it
cannot tell success from failure — it answers `Ok` after a mint that
failed, skips its re-certification and continuity passes on any face it
finds incomplete, and reads a face a door EMPTIED exactly as it reads
one never minted.

Beside them, `chart_stretch_sup` answers unit arms for a placeholder
chart while its sibling refuses — a bound that is a placeholder wearing
a bound's name — and `Pcurve::chart_box` is twice the true span of a
Harmonic image.

P0 because a certifier that cannot fail is worse than no certifier:
the kernel is built to trust it.

Charter and order: `work/pcert/plan.md`; narrative in `work/pcert/log.md`.
