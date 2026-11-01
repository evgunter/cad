---
id: quad
kind: program
title: QUAD — the quadrature lane's engines, its budgets and its dials
status: ready
opened: 2026-09-20
group: kernel/props
prefix: quad/
tag: (QUAD orchestrator)
ab_band: 5700-5799
budget: 45
paths: [crates/geom-brep/src/props/quad/*]
keep_out: [CUT FROM PROPS 2026-09-20 with Ev's agreement in chat — PROPS keeps the closed-form flux arms in crates/geom-brep/src/props/curved.rs and props/mod.rs, this program takes props/quad/ and the quadrature half of the refusals those two files share; PropsError and its QuadratureBudget variant live in props/mod.rs which is PROPS' file, so a change to the enum is an announced seam either way and the budget item says so; SHARED GROUND IS EXPECTED here and is not a conflict (Ev, in chat, 2026-09-20) — what the two programs owe each other is awareness when a lane is live on the same file, which the per-branch territory check and the announced-seam convention carry]
---

The quadrature lane as its own program: `props/quad/`'s four
engines, the convergence block they triplicate, the budget that fires
from six sites in three lanes under three round budgets, and the
rounds dial that is a ruling for Ev. Cut from PROPS on 2026-09-20
because it is one file family with one design question inside it and
no dependence on the closed-form flux arms PROPS keeps.

The unit shapes are already measured: three of the eight rows carry
executed numbers and two of them name the flip conditions. The dial
(`quad2-rational-max-rounds-dial-decision`) is the program's one `[ev]`
question and should be asked early, because two shipped refusals and
one first-class classification move with it.

`budget: 45` (2026-10-21). The P0 and P1 slate, plus the frame-dependence
lane, weighs about 42 points. The lane is one P1 row Ev's criterion
raised (a frame dependence that actually blocked something, in chat
2026-10-21) and four P2/P3 rows that are the same defect found by probes.
It stays whole, because all of it, `C3`'s engine split and `D30` change
the same four engines, and INTENT stage 3 E's two-frame test reads all
of it. The rest of P2–P4 went to QUADTAIL on the priority seam.
