---
id: onecopy
kind: program
title: ONECOPY — one home for a carrier's geometric readings, and for topo's face walks, refusal mappings and ladder searches
status: ready
opened: 2026-11-03
group: kernel/boolean/one-home
prefix: onecopy/
tag: (ONECOPY orchestrator)
paths: [crates/geom/src/curves/*, crates/geom/src/surfaces/*, crates/geom-brep/src/ladder.rs, crates/geom-brep/src/curve_pair/*, crates/topo/src/boolean/*, crates/topo/src/splitting/*]
keep_out: [opened 2026-11-03 by cutting ONEHOME per work/README.md Track size after its orchestrator ran out of usage - ONEHOME measured 70 budget points against 35 and every row it had not started moved here by git mv with its id and body unchanged and a dated note on the row - ONEHOME keeps what is in flight or staged (PR 6371 the finished body, PR 6380 the run owns f64 rounding with the lever and session-box rows filed against it, and the topo star stages 2 and 3 with the pierce and planner rows they fold) - so a row here that turns out to need one of those units moves back rather than racing it - BOXES cover-levers folds the per-kind ExtentBall constructors onto geom cover, and extent and lever rows stayed on ONEHOME for that reason - this track shares crates/topo/src/boolean/* and crates/topo/src/splitting/* with ONEHOME CLEAVE and the HONE family by design, shared ground is legitimate per the README's 2026-09-20 rule and what is owed is awareness while a lane is live, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled - several rows reach outside the paths (sweep mesh import), name the owner in the PR]
---

**One reading, one home: what ONEHOME found and had not started.** Two
halves, each a story of its own:

- **A carrier's geometric readings live in `geom`.** A point's foot on a
  line, circle or plane, its radial offset from an axis, a carrier's
  speed, a local line type, a circle's fields, an analytic surface's
  frame, the interval-safe angle about a conic: each spelled again
  outside the home ONEHOME's `geom-readings` and `conic-frame` units
  made (`geom::Line`, `geom::Circle`, `geom::Frame`, the `Conic` trait).
- **Topo's walks, refusal mappings and searches read their one home.**
  A face's loops and its surface, the `CarrierRefusal` mapping, the
  curve-pair meeting question, the identity ladder's `climb`: the
  follow-ups the reviews of PRs 6327, 6333 and 6362 filed, and the
  sweeps' leftover inline sites.

The P1 rows are the class `work/README.md` Priority bands P1 ("two
implementations of one underlying logic"). All of them were filed after
INTENT's 2026-10-26 design-signal sort, so none carries a sort and none
carries `design: true`; read each row's fix and judge for yourself
before dispatching it, and take one that turns out to be a fork
to designers first (`memories/orchestration-model.md`).

Slate and posture: `work/onecopy/plan.md`; narrative in
`work/onecopy/log.md`.
