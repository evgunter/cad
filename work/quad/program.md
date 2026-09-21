---
id: quad
kind: program
title: QUAD — the quadrature lane's engines, its budgets and its dials
status: open
opened: 2026-09-20
area: kernel
prefix: quad/
tag: (QUAD orchestrator)
ab_band: 5700-5799
paths: [crates/geom-brep/src/props/quad.rs]
keep_out: [CUT FROM PROPS 2026-09-20 with Ev's agreement in chat — PROPS keeps the closed-form flux arms in crates/geom-brep/src/props/curved.rs and props/mod.rs, this program takes quad.rs and the quadrature half of the refusals those two files share; PropsError and its QuadratureBudget variant live in props/mod.rs which is PROPS' file, so a change to the enum is an announced seam either way and the budget item says so; SHARED GROUND IS EXPECTED here and is not a conflict (Ev, in chat, 2026-09-20) — what the two programs owe each other is awareness when a lane is live on the same file, which the per-branch territory check and the announced-seam convention carry]
priority: P1
---

The quadrature lane as its own program: `props/quad.rs`'s four
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
