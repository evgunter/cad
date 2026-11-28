---
id: main-red-the-tours-die-with-two-pips-run-together-refuses-its-fold-at-member-five
kind: issue
title: main red: editor-core the_tours_die_with_two_pips_run_together_builds::each_pip_moved_into_its_neighbour_builds_its_union_and_meets_a_fillets_door refuses the union's fold at member 5 (slow set)
status: open
opened: 2026-11-28
priority: P1
cost: M
---



Found by the BOXFINISH one-plane lane (`boxfinish/one-plane`, PR 7710's
fix pass, 2026-11-28) running the whole `editor-core` suite with its
slow set. No open row names the test.

**Measured** on main `2abe74440` (the lane's tree after merging main,
with main's `geom-brep/src/plane_eq.rs` and `carrier_eq.rs` restored),
default ε, the gate's profile:
`cargo nextest run -p editor-core --test all --ignore-default-filter the_tours_die_with_two_pips`
fails at `crates/editor-core/tests/the_tours_die_with_two_pips_run_together_builds.rs`
(its `the pips' union builds and names` expectation), at the parameter
value `0.5800000000000001`:

> the fold refused at the step that folds in member 5 (read
> 12cdcd2e6d62): the Boolean op refused: the operands' sections could
> not be joined: every vertex of a hole loop lies on the boundary of
> the piece being divided off, so which piece holds it cannot be
> decided. Recourse: move the geometry

The same message, to the character, with the lane's change applied, so
the one-plane reading does not reach it. Not bisected; the raising site
in the Boolean was not read.

**Done when** the row is green or pinned on its refusal with the row
that holds the cause.

Signed: (BOXFINISH one-plane lane)
