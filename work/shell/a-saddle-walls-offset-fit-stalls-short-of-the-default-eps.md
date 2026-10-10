---
id: a-saddle-walls-offset-fit-stalls-short-of-the-default-eps
kind: issue
title: the twisted loft's saddle wall offset fit refuses BudgetExhausted at the default eps (4.12e-9 achieved against 1e-9), so no lofted body shells at the default tolerance even once its seams and corners are built
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [a-wall-seam-between-two-fits-has-no-section]
---


Filed from the wall-seam designer pair (PR 4515). Both designers named this as a separate gate that no item covers.

At the default ε (1e-9), and at 1e-12, `shell` on the twisted loft refuses at its first wall's offset fit, before any edge is planned: `ReplaceFaceError::Fit { BudgetExhausted }`, best bound 4.12e-9 m on a (27, 17) grid, at `d = 0.05`.
- This is pinned in `crates/sweep/tests/encl_curved_loft_shell.rs::shelling_the_curved_loft_refuses_at_a_walls_fit`. The `eps < 1e-8` arm there reads "the fit reaches about 4.1e-9 m in its round budget".
- The round budget and its stall guard are in `crates/geom-brep/src/offset_fit.rs`, at the budget constant's doc above `BudgetExhausted`.

So even with the ruled seam arm, the general simultaneous door and the narrowed iso-row arm in place, the loft shells only at ε of about 1e-8 or more.

Owed: measure first. Does the bound still fall when the budget is raised, and which `LastRound` the fit ends on? Then either:
- reach the default ε on the saddle wall (a refinement that keeps paying, or a budget lever that is honest); or
- state a measured reason a saddle offset cannot reach it at this degree, and pin that reason.
