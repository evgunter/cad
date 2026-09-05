---
id: closure-tier-skips-python-suite-on-geom-core-changes
kind: issue
title: TIER=closure on a geom-core/geom public-signature change runs RUN_PNCAD_PY=false — the python wheel is never built although it compiles against those crates
status: open
opened: 2026-09-05
---


(PROPS orchestrator) Reported by the Span-sweep lane (PR #1952) and by
PROPS-1's lane before it (PR #1918): both changed public signatures in
`geom-core`/`geom` and both ran `TIER=closure` with `RUN_PNCAD_PY=false`
("the seeds do not reach `pncad-py`"), so the python wheel — which
compiles against those crates through the `pncad` façade — was never
built by the gate; each lane ran the suite locally instead.
`docs/prompts/implementer-discipline.md` §2 says the python suite runs
on every code-tier run; `scripts/ci-filter.py` disagrees for closure
runs seeded outside `pncad-py`'s direct dependents. One of the two is
wrong; the cheap fix is to treat `pncad-py` as a dependent of every
crate the façade re-exports whole (`geom_core`, `geom`, …), or to say in
the discipline doc that the closure tier may skip it and that a lane
changing a re-exported crate must run it locally.
