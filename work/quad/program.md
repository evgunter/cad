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
paths: [crates/geom-brep/src/props/quad/*, crates/geom-brep/src/props/quad/trimmed.rs, crates/geom-brep/src/props/loop_area.rs, crates/geom-brep/src/props/rational_rim.rs]
keep_out: [CUT FROM PROPS 2026-09-20 with Ev's agreement in chat — PROPS keeps the closed-form flux arms in crates/geom-brep/src/props/curved.rs and props/mod.rs, this program takes props/quad/ and the quadrature half of the refusals those two files share; PropsError and its QuadratureBudget variant live in props/mod.rs which is PROPS' file, so a change to the enum is an announced seam either way and the budget item says so; SHARED GROUND IS EXPECTED here and is not a conflict (Ev, in chat, 2026-09-20) — what the two programs owe each other is awareness when a lane is live on the same file, which the per-branch territory check and the announced-seam convention carry, QUADTAIL merged in on 2026-11-20 (docs/doc-ledger/quadtail-and-spanfold-merge-into-quad.md) - it held no P0 or P1 row, from QUADTAIL: opened 2026-10-21 by QUAD's priority-seam cut (QUAD measured 72.5 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and QUAD kept its P0 and P1 rows. Shared ground is expected (Ev in chat 2026-09-20) - crates/geom-brep/src/props/quad/ is shared with QUAD and with SPANFOLD and SPIRICARM (FLUX's successors) - run scripts/work.py territory on your branch and announce the seam on the other program's log, SPANFOLD merged in on 2026-11-20 (docs/doc-ledger/quadtail-and-spanfold-merge-into-quad.md) - it held no P0 or P1 row, from SPANFOLD: opened 2026-10-27 when FLUX (51 budget points) and FLUXTAIL (44.5) were joined and re-cut by logical thread rather than by priority so that the successors can run in parallel (Ev in chat 2026-10-27) - the rows moved by git mv with ids and bodies unchanged and both directories left the tracker (docs/doc-ledger/flux-leaves-the-tracker.md), from SPANFOLD: crates/geom-brep/src/props/quad/* is QUAD's ground and shared with it - run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
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

## From QUADTAIL (merged 2026-11-20)

QUADTAIL — the quadrature lane's P2–P4 tail: enclosure looseness, missing volume rules and the tests over them

**The quadrature lane's tail.** QUAD keeps the engines, the budget's
shape, the dial and the rows where a measured value depends on where
the body sits. The rows here are what is left at P2 to P4: enclosures
looser than they need be (a 90° rational span, a micron wall's
map-residual pad, an unpadded junction gap), faces with no volume rule,
an area gauge that panics instead of refusing, and tests that assert
less than they could or hold a midpoint to a fixed tolerance.

`budget: 32`, because the cut's whole P2–P4 residue weighed 30.5
points, and a half-point overage does not justify a third track.

Slate, order and posture: `work/quad/plan.md`; narrative in
`docs/doc-ledger/quadtail-and-spanfold-merge-into-quad.md`.

## From SPANFOLD (merged 2026-11-20)

SPANFOLD — the quadrature's span folds and loop-area arms: refusals dropped through f64::min, chord lengths rooted to nearest, and pieces no arm takes

**Where a quadrature or loop-area fold loses a refusal or a sign.** The trim walk's span folds go through `f64::min` and drop a refused step or window; its chord lengths are rooted to nearest and read as a pad. A NURBS loop-area segment integrates an inverted window as zero, a spline carrier past the Gauss inventory has no composite arm, a rational rim with a near-zero weight is not live on equal pieces, and a cylinder split just off its rim vertex reads its sliver wide. The gating corpus reaches none of the collapsed-arm gates that would catch this class.

Slate, order and posture: `work/quad/plan.md`; narrative in
`docs/doc-ledger/quadtail-and-spanfold-merge-into-quad.md`.
