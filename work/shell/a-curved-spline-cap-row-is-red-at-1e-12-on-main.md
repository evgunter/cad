---
id: a-curved-spline-cap-row-is-red-at-1e-12-on-main
kind: issue
title: shell_general_door's curved spline cap row is red at ε 1e-12 on main (fit stops at 3.2e-12)
status: closed
opened: 2026-10-12
priority: P3
cost: E
closed: 2026-10-26
refs: [a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main]
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

## 2026-10-14 — still red at 1e-12 on main (NURBS, fresh target)

Re-run alone with a fresh `CARGO_TARGET_DIR` per commit: red at `a71403e3` and still red on main at `307ed404`. Not moved by NURBS PR 4984 (red on both sides of it). (NURBS build A)

## 2026-10-19 — bisected to PR 4551 (NURBS)

CONTACTHOLD's nightly-red lane bisected it over first-parent merges from `61ee8bf5a4` (green) to `360cf55eb2` (the 2026-10-12 nightly, red), at ε 1e-12. It passes at `071a630b58` and fails at **`aa1f124298`**, the merge of PR 4551 (`nurbs/monotone-step-and-blossom`: `Certification::between` and `CoeffWindow::blossom`), stopping at 3.2397e-12 on (48, 48). `encl_curved_loft_shell`'s curved fitted cap row (`a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main`) moved at the same merge. Still red in the 2026-10-19 nightly (run 39205007024).

## Closed (2026-10-26, sweep follow-up)

A duplicate of `a-curved-fitted-cap-rows-sample-cap-grid-pin-is-red-at-1e-12-on-main`.
Both reds are on the offset fit and both were bisected to PR 4551's merge
(`aa1f124298`), so they are one re-baseline-or-fix decision. That row
survives (it carried a band and the pin's history), and this row's test,
its reproduction and its open question (an unlisted refusal, or no
refusal) are carried into it. Still red at 1e-12 on main `d621f67e`
(`shell_general_door.rs:153`, stop 3.2397e-12 on (48, 48)).
