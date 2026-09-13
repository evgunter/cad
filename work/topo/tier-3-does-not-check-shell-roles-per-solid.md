---
id: tier-3-does-not-check-shell-roles-per-solid
kind: issue
title: tier 3 accepts a solid whose shells classify to two Outer boundaries — shell-to-solid grouping is unchecked
status: open
opened: 2026-09-08
---


Measured by the SHELL-5 lane (PR #2159, 2026-09-08) and placed here by
the SHELL orchestrator: a body whose one solid holds an `Outer` shell,
a `Void` and a second `Outer` inside that void (the boolean's
hollow-operand subtraction, `work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`)
passes `validate_geometric`. Tier 2's per-shell Euler–Poincaré and
tier 3's per-face checks never ask which SOLID a shell belongs to, and
`classify_shells` is a props read no tier consumes, so a solid with
two outer boundaries — two material components filed under one
solid — validates. SHELL-5 pins its own grouping in its rows
(`roles_by_solid` in `crates/sweep/tests/verbs_shell.rs`) rather than
adding a check, because where the check belongs (a tier-3 row reading
each solid's shell roles: exactly one `Outer`, every other shell `Void`
and inside it) is TOPO's decision. Signed (SHELL orchestrator).
