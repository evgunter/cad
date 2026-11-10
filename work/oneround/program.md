---
id: oneround
kind: program
title: ONEROUND — f64 rounding has one statement, a lever one source, the consumed extent one home
status: ready
opened: 2026-11-08
group: kernel/boolean/one-home
prefix: oneround/
tag: (ONEROUND orchestrator)
paths: [crates/geom-core/src/tolerance.rs, crates/geom-core/src/predicate.rs, crates/geom-core/src/running.rs, crates/geom-brep/src/extent.rs, crates/geom-brep/src/carrier_resolution.rs, crates/topo/src/session_box.rs, crates/topo/src/ring_path.rs]
keep_out: [opened 2026-11-08 by splitting ONEHOME per work/README.md Track size after PRs 6371 and 6380 landed - ONEHOME measured 78 budget points against 35 - every row moved by git mv with its id and body unchanged and a dated note on the row - the ratified contract is crates/geom-core/README.md Float rounding (Ev on PRs 6329 and 6380) - Ev on PR 6380 comment 6594257162 keep the README's lever sentence and fix its violations - BOXES cover-levers folds the per-kind ExtentBall constructors onto geom cover, so read its rows before a lever or extent unit - the bare-rounding and lever-sources gates ratchet these rows' sites, a unit lowers their pins - territory is shared with ONEHOME, BOXFINISH, ONECOPY, BOXES and the HONE family by design, run scripts/work.py territory on your branch and announce the seam in the PR]
---

**One statement of f64 rounding, one source for a lever, one home for
the consumed extent.** PR 6380 made the run own f64 rounding: the
admission rule on (ε, K), the session box at the finished-body gate,
and a bare-rounding gate. PR 6391 made `geom_brep::Reach` the one
consumed extent and `Lever<T>` the one lever. This track holds what
those units filed and could not finish:
- the far-centre conic;
- the lever's constructor doors and its pinned unsourced arms;
- the site charges still owed a re-posing;
- the session box's curved bulge;
- the extent reads still made outside the extent home.

Slate and posture: `work/oneround/plan.md`; narrative in
`work/oneround/log.md`.
