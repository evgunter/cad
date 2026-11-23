# MIRROR and BLIND leave the tracker — 2026-09-28

MIRROR (the checkers and local scripts beside CI: the hosted/local parity
reader, the render and provenance lanes, the calibrators and their pins)
and BLIND (CI instruments that cannot see what they name) were opened
2026-09-20 by CIW's priority-seam cut. The CI-latency cut of 2026-09-28
(PRs #3340, #3369; `work/ciw/latency-cut.md`) deleted most of what they
were about: the parity checker and its siblings, `ci-local.sh`, the
opt-level calibrator, the lane axis and the test matrix, `check-run-jobs.py`
and per-PR rustdoc and renders. Neither plan set `## Exit criteria`, so
both close without a walk.

Deleted: 22 MIRROR rows and 5 BLIND rows whose subject is gone, both
directories' `program.md`, `plan.md` and `log.md`, and
`work/tcost/actions-cache-budget-under-a-hash-key` (its only dependent was
MIRROR's `cache-rendered-cells-on-input-hash`, whose premise — PR runs
render, main re-renders — went with per-PR renders). Two rows were fixed
in place rather than kept: `provenance-lane-dirs-is-not-the-lane-roster`
(`demos/check_render_provenance.py`'s comment) and
`k-probe-sweep-says-no-test-compares-probe-against-f64`
(`scripts/k_probe_sweep.sh`'s comment).

Re-homed with their ids: 11 rows to `work/ciw/` (4 from MIRROR, 7 from
BLIND), `a-citation-in-a-line-comment-is-not-checked` to `work/vdoc/`, and
`culling-is-load-bearing-with-no-pixel-test` to `work/chrome/`. Bands
9800–9899 and 9900–9999 stay allocated.

Sweep SHA `5369473ee5e229e8709886227cfd305082fa6fe8`.

    git show 5369473ee5e229e8709886227cfd305082fa6fe8:work/mirror/<FILE>
    git show 5369473ee5e229e8709886227cfd305082fa6fe8:work/blind/<FILE>
    git show 5369473ee5e229e8709886227cfd305082fa6fe8:work/tcost/actions-cache-budget-under-a-hash-key.md
