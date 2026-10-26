---
id: the-teapot-canal-volume-misses-its-frustum-at-1e-12
kind: issue
title: The teapot's canal volume misses its straightened frustum at eps 1e-12
status: closed
opened: 2026-10-23
closed: 2026-10-24
---


`demos/tour/tests/eps_regression.rs:82` (`tour_runs_green_at_eps_1e_12`)
is red on main at `52a71ab10c`: the tour panics at
`demos/tour/src/teapot.rs:2181`,

> the canal's V = 0.000055052648797962526 vs the straightened
> frustum's 0.000055052533158981754: they differ by
> 1.156389807722329e-10 m^3, OUTSIDE the certified enclosure's own
> half-width 8.094490616656182e-12

The same numbers come back on D′3's branch (PR 5636), where hosted CI
first showed it, and on a clean worktree of main built locally
(release, `--profile ci`). The default ε and 1e-6 rows are green. The
row's at-1e-12 enclosure is ~8e-12 m³ while the two volumes differ by
~1.2e-10 m³, so either the canal's enclosure is not the canal's volume
at this ε or the frustum comparison's expectation is not the canal's.
Not diagnosed further.

**The same red, filed twice.** PR 5635 (open) files it as
`work/iso/the-teapot-spouts-frustum-oracle-misses-its-measured-volume-at-1e-12.md`
and names PR 5629 (the per-row Cartesian lane in `sweep/src/skin.rs`)
as the likely cause, unverified. PR 5640 reads it the same way: the
canal used to refuse at 1e-12, it now decides, and the comparator is
the continuum tube, not the fitted body. Whichever lands second closes
its copy into the other.

Fixed on main by ISO's PR 5642 (merged at `5589ff6605`), which closed
its own twin of this row. D′3's branch has merged it.
