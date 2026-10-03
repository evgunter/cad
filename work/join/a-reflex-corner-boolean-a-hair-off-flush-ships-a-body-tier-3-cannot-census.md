---
id: a-reflex-corner-boolean-a-hair-off-flush-ships-a-body-tier-3-cannot-census
kind: issue
title: Sixteen reflex-corner booleans turned 0.003 degrees off flush ship a body with the closed-form volume that tier 3-prime's census cannot decide (CensusEscalated, point_in_loop_side in band)
status: open
opened: 2026-10-03
priority: P0
cost: H
refs: [reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap]
---


(Found by the review of PR 3900 (`join/reflex-corner-review`, NOTE-2);
measured and filed by the reflex-corner lane. Release build, on main
plus PR 3900's fix pass. The same lines appear with the PR's strut
order reverted, so this predates it.)

## What

`crates/sweep/tests/join_rc_probes.rs` `rc_wide_battery` turns each
reflex-probe profile about `a`'s 315° corner. At 0.003°, sixteen ops
build a body that has these properties:

- passes tier 2 (`validate_closed`);
- passes the at-rest certificate (`validate_geometric_certificate`);
- unites with a far brick, so it is a legal operand;
- matches the closed-form volume within 1e-7.

But tier 3′ (`validate_pseudomanifold` with the result's contacts)
fails. A boolean must not hand back a body its own tier 3′ cannot
pass.

Each line is one pose's tier-3′ error, from
`RC_CASE="<profile> <rot> <sx> <sy> <op>"` and
`cargo test -p sweep --test all rc_detail -- --ignored --nocapture`.
All are `Err([CensusEscalated { cause: Indeterminate { … } }])` at
band `{ zero: 1e-9, escalate: 1e-8 }`:

| pose | predicate | margin |
|---|---|---|
| `dRight 0.003 0 -0.1 S_ab` | `point_in_loop_side` | 2.7414132074054285e-9 |
| `dRight 0.003 0 -0.25 S_ab` | `point_in_loop_side` | 2.74141320663499e-9 |
| `dRight 0.003 0 -0.3 S_ab` | `point_in_loop_side` | 2.7414132062323954e-9 |
| `dRight 0.003 0 -0.5 S_ab` | `point_in_loop_side` | 2.7414132038867535e-9 |
| `dRight 0.003 0 -0.75 S_ab` | `point_in_loop_side` | 2.7414131992994063e-9 |
| `sqQ1 0.003 -0.25 -0.75 U` | `point_in_loop_arm` | 5.651568173133848e-9 |
| `sqQ1 0.003 0 -0.1 U` | `point_in_loop_side` | −1.3707783796945525e-9 |
| `sqQ1 0.003 0 -0.25 U` | `point_in_loop_side` | −1.3707783796944795e-9 |
| `sqQ1 0.003 0 -0.3 U` | `point_in_loop_side` | −1.3707783796945705e-9 |
| `sqQ1 0.003 0 -0.5 U` | `point_in_loop_side` | −1.3707783796942072e-9 |
| `sqQ1 0.003 0 -0.75 U` | `point_in_loop_side` | −1.3707783796934805e-9 |
| `sqQ1 0.003 0 0.1 U` | `point_in_loop_side` | −1.3707783796945525e-9 |
| `sqQ1 0.003 0 0.25 U` | `point_in_loop_side` | −1.3707783796944795e-9 |
| `sqQ1 0.003 0 0.3 U` | `point_in_loop_side` | −1.3707783796945705e-9 |
| `sqQ1 0.003 0 0.5 U` | `point_in_loop_side` | −1.3707783796942072e-9 |
| `sqQ1 0.003 0 0.75 U` | `point_in_loop_side` | −1.3707783796934805e-9 |

Both profiles put a `b` edge along one of `a`'s corner edges when
unturned: sqQ1's along the 0° edge, dRight's along the 45° one. A turn
of 0.003° takes that edge 5.2e-5 off the corner edge at unit reach.
That is far outside the band, yet the census reads a point
1.4e-9 to 5.7e-9 from a loop: some vertex or edge of the result sits
within the band of a loop it is not on.

## Not measured

- Which result entity the census reads in band, and whether the
  boolean placed it there. It could be a seam vertex snapped onto an
  edge it does not lie on, or a minted chord that should have reused
  the existing edge.
- Whether the same ops refuse or answer at 0.001° and 0.01°.

## The bar

`rc_wide_battery` reports no build that fails tier 3′. The boolean
either builds a body tier 3′ passes, or refuses typed.
