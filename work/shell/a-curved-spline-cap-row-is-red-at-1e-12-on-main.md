---
id: a-curved-spline-cap-row-is-red-at-1e-12-on-main
kind: issue
title: shell_general_door's curved spline cap row is red at ε 1e-12 on main (fit stops at 3.2e-12)
status: open
opened: 2026-10-12
---


`crates/sweep/tests/shell_general_door.rs`'s
`a_curved_spline_cap_moves_with_its_sides_where_its_certificates_hold`
fails under `CAD_TOLERANCE_EPS=1e-12` on main. The TANG must-carry-chain lane
(PR 4859) measured it identical on its base `f9f617fe2` and on its head,
so it is not that PR's. The fit stops at about 3.2e-12, just past the
1e-12 band. The per-PR gate does not run this row at 1e-12; the nightly
does.

The row's own text (after ENCL's PR 4538 seam note in `work/shell/log.md`)
says it accepts either limb-2 refusal at ε < 1e-7, so at 1e-12 it should
read a typed refusal rather than fail. Either the 1e-12 outcome is a refusal
the row does not list, or the fit's stop is not reaching a refusal at all.
Reproduce with
`CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test shell_general_door a_curved_spline_cap_moves_with_its_sides`
and read which.

## 2026-10-13 — still red on main

The FLUX cylinder-band lane (PR 4839) re-ran the row alone by name on
clean main `e71d719f1`. It is red at `CAD_TOLERANCE_EPS=1e-12` and green at
1e-6.
