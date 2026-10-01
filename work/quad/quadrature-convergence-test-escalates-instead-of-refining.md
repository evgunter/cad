---
id: quadrature-convergence-test-escalates-instead-of-refining
kind: issue
title: the quadrature lanes escalate an in-band convergence test instead of refining another round, so a face measures in one boolean operand order and not the other
status: open
opened: 2026-10-01
---


## What

Each quadrature lane's round loop (`crates/geom-brep/src/props/quad.rs`,
`cylinder_cut_face_rounds` and its three copies, `props_quad_converged`)
decides whether the round's enclosure met the reporting target with
`classify_len(..)?`. When the convergence margin `target_len −
width_len` lands inside the band, the `?` escalates the whole face
(`PropsError::Escalated`) instead of treating "not definitely
converged" as "run another round". A width a hair from the target is
not a question about the model, and the next round, whose enclosure is
about 8× narrower, would decide it.

Measured (REACH, branch `reach/volume-backstop`, 2026-10-01):
- `union(base, boss)`, where base is `brick((-1, 1), (-1, 1), (0, 1))`
  and boss is a radius-0.25, height-1 three-arc cylinder on the sketch
  plane through `(0, 0, 0.5)` turned 0.5 rad about `x`, refuses
  `VolumeUnmeasured { operand: None, source: Face { source: Escalated
  { margin 3.13e-9, band (1e-9, 1e-8), predicate "props_quad_converged" } } }`.
- `union(boss, base)` builds and certifies at the analytic volume. The
  result is the same solid in a different arena order, so one of its
  wall faces lands its convergence margin in band in one order and not
  the other.
- At 0.45 and 0.55 rad both orders build.
- The GERM half-donut row
  (`work/germ/union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`)
  refuses through the same arm on its subtraction (margin −1.85e-9).

Both orders are pinned by `crates/sweep/tests/reach_volume_backstop.rs`
`a_boss_at_half_a_radian_measures_in_one_operand_order`. The refusing
order goes red when this lands, and should then build at
`penetrating_union(0.5)`.

## The shape of a fix

In-band at the convergence test means not converged: take the next
round, and refuse `QuadratureBudget` only at the last round, as a
definitely unconverged face already does. The four copies of the block
are the C3 finding (`work/quad/C3.md`), so the fix lands once if the
block is factored first.

The text the escalation renders is PROPS's
`props-escalation-renders-the-coincidence-menu-unlabelled`. The
backstop's own refusal now names the quadrature's convergence and
offers no coincidence (`topo::validate::classify_mass_props`).
