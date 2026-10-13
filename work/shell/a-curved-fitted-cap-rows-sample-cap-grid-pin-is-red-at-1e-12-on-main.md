---
id: a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main
kind: issue
title: encl_curved_loft_shell's curved fitted cap row is red at ε 1e-12 on main: the fit stops at its pinned bound on (45, 48), not the pinned (45, 45)
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters]
---


The FLUX cylinder-band lane (PR 4839) saw this in the nightly ε set. It is
reproduced alone by name on clean main `e71d719f1`. The lane's diff does not
reach the offset fit.

`crates/sweep/tests/encl_curved_loft_shell.rs:1006`, in
`a_moved_curved_fitted_cap_builds_where_its_fit_certifies`, fails under
`CAD_TOLERANCE_EPS=1e-12`:

    eps 1e-12, d = 0.05: the cap's fit stopped at 4.3818950836273555e-11 on (45, 48)

The refusal is the one the row expects: `Fit { WallReached { SampleCap } }`,
with `achieved` inside 1e-14 of the pinned 4.382e-11. Only the grid moved,
from the pinned `(45, 45)` to `(45, 48)`. The default ε and 1e-6 are green.

The red is older than PR 4800 (`crate::descent`, merged at `428057bb7`). Its
first parent fails with the same message, so that PR is not the cause. The
pin and its comment (lines 991–995) were written by SHELL's PR 4672
(`5253d6dcb`, round budget retired). The commit that moved the grid is not
bisected.

Owed: decide whether (45, 48) is the fit's right stopping grid now. If it is,
re-baseline the pin and the comment, and say what moved it. If it is not, fix
the sample-cap walk. The per-PR gate runs this row only at the default ε;
the nightly runs it at 1e-12.

Reproduce with
`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep -E 'test(a_moved_curved_fitted_cap_builds_where_its_fit_certifies)'`.
