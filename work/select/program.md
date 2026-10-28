---
id: select
kind: program
title: SELECT — the D2 selector: a two-surface edge picked by a structural selector or a witness, and the OnPair lanes it retires
status: ready
opened: 2026-10-17
group: kernel/topo/certify
prefix: select/
tag: (SELECT orchestrator)
ab_band: 13400-13499
paths: [crates/geom-brep/src/certify.rs, crates/geom-brep/src/dihedral.rs, crates/geom-brep/src/edge_nurbs.rs]
keep_out: [opened 2026-10-17 by TANG's priority-seam cut (Ev in chat 2026-10-17: split TANG into multiple programs) per work/README.md Track size - TANG measured 142.5 budget points against 30 - and the rows moved by git mv with ids and bodies unchanged, TANG is NOT closed and keeps the pinch machinery and the declared-tangency spine, the five tracks of the cut (TANG SELECT LEVER TWICE TANGTAIL) share crates/topo/src/boolean/* and chord_join.rs and geom-brep's certify and dihedral ground with each other and with GERM SECTOR PCERT ISO and CONTACTHOLD - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live - so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

**Build Ev's D2 decision of 2026-10-16 (PR 5216).** A two-surface edge
is one form, `{ pair, selector }`. A curve known in advance takes a
structural selector; one that must be derived numerically takes a
witness. The choice is asked of the geometry, so native and imported
bodies carry the same description. `OnPair` is removed. The build's
eight units are on `onpair-has-no-incidence-lane-on-a-spline-adjacent-pair`;
the other rows here are the `OnPair` lanes and chart-image certificates
the selector retires or must keep honest.

Slate, order and posture: `work/select/plan.md`; narrative in
`work/select/log.md`.
