---
id: a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main
kind: issue
title: Two curved cap rows are red at ε 1e-12 on main since PR 4551 — encl_curved_loft_shell's fitted cap stops on (45, 48) not the pinned (45, 45), and shell_general_door's spline cap stops at 3.24e-12 on (48, 48)
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [a-rigid-map-can-still-refuse-a-sound-approx-face-at-its-edges-or-meters, a-curved-spline-cap-row-is-red-at-1e-12-on-main, saddle-wall-certifies-at-1e-12-where-its-row-pins-the-cap]
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

## 2026-10-14 — still red at 1e-12 on main (NURBS, fresh target)

Re-run alone with a fresh `CARGO_TARGET_DIR` per commit: red at `a71403e3` and still red on main at `307ed404`. Not moved by NURBS PR 4984 (red on both sides of it). (NURBS build A)

## 2026-10-19 — bisected to PR 4551 (NURBS)

CONTACTHOLD's nightly-red lane bisected it over first-parent merges from `61ee8bf5a4` (green, which includes PR 4672's pin) to `360cf55eb2` (the 2026-10-12 nightly, red), at ε 1e-12. It passes at `071a630b58` and fails at **`aa1f124298`**, the merge of PR 4551 (`nurbs/monotone-step-and-blossom`), with the same `(45, 48)` stop. The pin was right when PR 4672 wrote it, and PR 4551 moved the grid. `shell_general_door`'s curved spline cap row moved at the same merge. Still red in the 2026-10-19 nightly (run 39205007024).

## 2026-10-26 — one row for both reds (sweep follow-up)

`a-curved-spline-cap-row-is-red-at-1e-12-on-main` is closed into this
row: its red was bisected to the same merge (`aa1f124298`, PR 4551,
`Certification::between` and `CoeffWindow::blossom`), on the same
offset fit. What it carried:

- `crates/sweep/tests/shell_general_door.rs`'s
  `a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold`
  (`:125`) fails at `CAD_TOLERANCE_EPS=1e-12`, green at 1e-6 and the
  default. Its text accepts either limb-2 refusal at ε < 1e-7, so at
  1e-12 it should read a typed refusal rather than fail: either the
  1e-12 outcome is a refusal the row does not list, or the fit's stop
  reaches no refusal at all. Which one is not yet read.
- Reproduce with
  `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test shell_general_door a_curved_spline_cap_moves_with_its_sides`.

Re-run both today at 1e-12 on main `d621f67e`, still red:

- `shell_general_door.rs:153`: "eps 1e-12: the cap's fit stopped at
  3.239736902471488e-12 on (48, 48)";
- `encl_curved_loft_shell.rs:1144` (the pin is at `:1145`): "eps 1e-12,
  d = 0.05: the cap's fit stopped at 4.38189780844083e-11 on (45, 48)".

Owed, for both: decide whether PR 4551's stop is the right one. If it
is, re-baseline each pin and its comment and say what moved it. If it is
not, fix the fit. The per-PR gate runs these rows only at the default ε.

**The saddle row.** `saddle-wall-certifies-at-1e-12-where-its-row-pins-the-cap`
looks like the same class: the offset fit's stop at 1e-12 was moved by a
merge of 2026-10-12. Its suspect is PR 4800 (`f026b3643`, "one stop rule
(crate::descent)", merged `428057bb7`), not PR 4551, and it was never
bisected. Its test,
`saddle_wall_offset_fit::a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap`,
**passes** at 1e-12 on `d621f67e`. It now fits at a fixed 1e-9 and pins
`(49, 26)`, round 9 and 3.8503e-10
(`crates/sweep/tests/saddle_wall_offset_fit.rs:23-41`), so the red that
row describes no longer reproduces. It is left open for its owner to
close with the commit that re-wrote the row.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Both still red at 1e-12 with the messages above. Each is pinned at
1e-12 on its stop (`d = 0.05: the cap's fit stopped at` and `eps 1e-12:
the cap's fit stopped at`); each pin goes red the day its row passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
