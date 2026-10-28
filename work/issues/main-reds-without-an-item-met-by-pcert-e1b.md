---
id: main-reds-without-an-item-met-by-pcert-e1b
kind: issue
title: main reds with no item, met by PCERT E1b's round: the tier-1 census names validate_pseudomanifold, emit_union_borders' corner boss, and three sweep rows at 1e-6 (one builds a wrong volume)
status: open
opened: 2026-10-27
priority: P1
cost: M
---


## Finding

PCERT E1b's local suite round (PR #5797) met these reds. Each is red
on main `2f9e6f9e39` itself, and none has an open item that covers it.
Filed together, to be split by whoever picks them up.

- **topo, at the default ε and 1e-6:**
  `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
  (`crates/topo/src/review_m1_pr5_internal.rs:563`): "public mutation
  path(s) that neither declare the tier-1 debug postcondition nor open
  a surgery scope nor appear on this test's allowlist:
  [`…/crates/topo/src/props.rs::validate_pseudomanifold`]". The
  2026-10-24 entry in
  `work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`
  was a different door (`join_edges`).
- **editor-core, at the default ε and 1e-6:**
  `emit_union_borders::the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean`
  panics with "no entry found for key" (`emit_union_borders.rs:680`).
  It is also red on main `be2b7a139c`.
- **sweep, at 1e-6** (each red on main `2f9e6f9e39` at 1e-6):
  - `spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong`
    (`spline_spiric_operands.rs:293`): "lofted cylinder / far / O ∪ B:
    builds V = 7.283217721708709, the closed form gives
    7.283185307179586". **A wrong build, not a refusal**, so P1. The open
    `work/spliceseam/the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12.md`
    is the 1e-12 quadrature refusal, a different failure.
  - `operand_gate_pose::an_approx_cap_clear_of_a_bar_reads_alike_at_every_pose`
    (`operand_gate_pose.rs:197`): "ε 1e-6, identity, ∪: the cap clears
    the bar, so the op reaches the classification's Approx door:
    Ok(true)".
  - `m5_s11_concave_sense::a_lofted_operand_refuses_the_union_check_typed`
    (`m5_s11_concave_sense.rs:853`): the refusal now names the spline
    face pairing ("the Boolean cannot yet work out where one solid's
    spline (NURBS) face meets the face of the other solid…") where the
    row expects the operand named.

Not bisected.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. `spline_spiric_operands::no_spline_or_spiric_operand_builds_wrong`
still builds the wrong volume at 1e-6 (`lofted cylinder / far / O ∪ B:
builds V = 7.283217721708709`, closed form `7.283185307179586`). It is
split out to its owner as
`work/spliceseam/main-red-the-lofted-cylinders-far-union-builds-the-wrong-volume-at-1e-6`,
and the 1e-6 pin cites that row, not this file. The 1e-12 pin cites
`work/spliceseam/the-lofted-cylinder-operand-runs-out-of-quadrature-budget-at-1e-12`.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
