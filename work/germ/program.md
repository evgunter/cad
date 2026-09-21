---
id: germ
kind: program
title: GERM — the cone and torus operand lanes, and the containment arms that stop short of them
status: open
opened: 2026-09-20
area: kernel
prefix: germ/
tag: (GERM orchestrator)
ab_band: 7000-7099
paths: [crates/topo/src/boolean/sectors.rs, crates/geom-brep/src/intersect.rs]
keep_out: [opened by REACH's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - REACH measured 151 budget points, about five sittings, and was cut into six tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on at once), the six are REACH GERM CONTACT ZIP BOXES PIN and they SHARE crates/topo/src/boolean/* by design - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, TANG shares chord_join.rs and boolean/rest.rs for the declared-tangency and germ/pierce lanes, CHART owns the pcurve and SSI side, CURVED keeps offset_axial.rs]
priority: P0
---
The two operand kinds the boolean lane reaches last. `point_in_solid`
answers cone and torus; `curved_face_containment` has no arm for
either, so the two doors disagree about the same body. A coincident
torus pair cannot reach the declared-cover rung at all. And the
ray-torus root search **disagrees with its own geometric oracle** on a
rare pose at eps = 1e-12 — a live wrong answer with a recorded
counterexample, which is why this track is P0 rather than
verb-breadth P1.

`c5-gate-admits-every-pose-of-an-implemented-pair` is the structural
row under the others: the C5 gate reads `PairRoute::implemented` per
KIND pair, so a pose the arm would refuse passes the gate and fails
further in.

Charter and order: `work/germ/plan.md`; narrative in `work/germ/log.md`.
