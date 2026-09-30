---
id: nurbs
kind: program
title: NURBS — the spline doors: knot insertion, span metring, the loose net and what restrict composes
status: ready
opened: 2026-09-20
area: kernel
prefix: nurbs/
tag: (NURBS orchestrator)
ab_band: 9300-9399
paths: [crates/geom-core/src/spline/*]
keep_out: [opened by PROPS's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - PROPS measured 108.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are PROPS NURBS LINALG VERDICT and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, ENCL and STACK hold the certified-enclosure half, QUAD owns props/quad.rs, SSI and PCERT own the geom-brep certification lanes, PORT took the four refusal rows whose class is the crate boundary]
priority: P1
---
The spline layer's own doors, split from the property arms that consume
them. Its two live rows are arithmetic: `refine_dir`'s exact-equality
insertion guard leaves knot pairs ONE ULP apart, so de Boor divides by
the hairline; and `nurbs_span_meter`'s `d1 - d0` goes negative on a
reversed domain, which is now indistinguishable from a collapsed span.

Behind them, `MappedCurve::restrict` composes the anchored rotation
into the stored placement on every split, re-applying `rotation_about`'s
diagonal enclosure each time where composing in the PARAMETER would
keep one placement — a widening that compounds per split. And the loose
`(knot vector, coefficient array)` shape survives outside the hull at
evaluators, tensor grids, composition and a public green-integral door,
which is the pairing invariant not being one.

Charter and order: `work/nurbs/plan.md`; narrative in `work/nurbs/log.md`.
