---
id: germ
kind: program
title: GERM — the cone and torus operand lanes, and the containment arms that stop short of them
status: ready
opened: 2026-09-20
group: kernel/boolean/curved
prefix: germ/
tag: (GERM orchestrator)
ab_band: 7000-7099
paths: [crates/topo/src/boolean/sectors.rs, crates/geom-brep/src/intersect.rs]
keep_out: [opened by REACH's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - REACH measured 151 budget points, about five sittings, and was cut into six tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on at once), the six are REACH GERM CONTACT ZIP BOXES PIN and they SHARE crates/topo/src/boolean/* by design - shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled rather than drawing a fence, TANG shares chord_join.rs and boolean/rest.rs for the declared-tangency and germ/pierce lanes, CHART owns the pcurve and SSI side, CURVED keeps offset_axial.rs]
---

The two operand kinds the boolean lane reaches last. Its P0 rows are
closed: the ray-torus root search's wrong answer at ε = 1e-12, the
cone and torus arms of `curved_face_containment`, and the declared
cover for a coincident torus pair. The circle × torus walk now reads
its Taylor terms at the conic point (PR 5394), and a near-square
plane × cylinder/cone section is the exact Ellipse (Ev, PR 5156).

What is left here is P1 and P2:

- `c5-plane-torus-cone-cylinder-arms` moved to SECT on 2026-10-25,
  beside the torus frames the section-first join reads it for.
- The P2 rows are Interval and band reach on the conic sections: an
  oblique rod clear of a cone refuses the extent scan at 1e-12; a conic
  section just past its band fails certification downstream; and a
  sliver crossing's long bisector ellipse is served past what its
  evaluation can represent.

The P3/P4 tail was cut out at the 2026-10-20 hand-back into GERMCERT
(rounding and certificate margins) and GERMTAIL (refusals and
structure), on the README's Track size rule.

Charter and order: `work/germ/plan.md`; narrative in `work/germ/log.md`.
