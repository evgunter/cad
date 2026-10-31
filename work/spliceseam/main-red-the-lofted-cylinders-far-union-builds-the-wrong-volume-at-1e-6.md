---
id: main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6
kind: issue
title: Main red: spline_spiric_operands' lofted cylinder far union builds a volume 3.2e-5 off its closed form at 1e-6
status: open
opened: 2026-10-28
priority: P1
cost: M
---


Split out of `work/issues/main-reds-without-an-item-met-by-pcert-e1b`
by GREEN (`green/full-suite-pins`, 2026-10-28), which pinned the row.
SPLICESEAM holds the lofted-cylinder operand's other red
(`the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12`).

## What

`sweep` `spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong`
(`crates/sweep/tests/spline_spiric_operands.rs`, the volume check in its
row) at `CAD_TOLERANCE_EPS=1e-6`:

    lofted cylinder / far / O ∪ B: builds V = 7.283217721708709, the closed form gives 7.283185307179586

A built body, not a refusal: the union's volume is 3.24e-5 above the
closed form, at a band of 1e-6. Green at the default ε; at 1e-12 the
row stops earlier, on the operand's measurement (SPLICESEAM's 1e-12 row).

Red on main `2f9e6f9e39` (PCERT E1b, 2026-10-27), in nightlies 73
(`fb476030a9`) and 74 (`a1cfd1cfdb`), and on main `91c83c38`
(2026-10-28). Not bisected: the row came in with PR 5704 (`173271c`),
and whether it was ever green at 1e-6 is unmeasured.

## Pinned

The row is pinned at 1e-6 on `lofted cylinder / far / O ∪ B: builds V =`
(`test_utils::pinned_red::row`), so main reads green while this stays
open. The pin goes red the day the row passes.

## What closing it takes

Find which face of the far union carries the excess volume, and whether
the build is wrong or the volume reading is. A wrong build is fixed in
the kernel; then the pin comes out.

Reproduce: `CAD_TOLERANCE_EPS=1e-6 cargo nextest run -p sweep --profile ci
--ignore-default-filter -E 'test(=spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong)'`
With the pin in place it passes while the row is red; the panic it
catches is printed in the test's output.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured. Known behind it: the far union's other operations after `O ∪ B`.
