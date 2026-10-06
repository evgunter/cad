---
id: coincident-shell-has-no-fixture-for-its-unpaired-and-mixed-arms
kind: issue
title: CoincidentShell's Unpaired and Mixed arms have no fixture: only the not-covered-back arm is reached
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

`BooleanError::CoincidentShell` (`crates/topo/src/boolean/mod.rs`)
carries a `ShellOrientation` saying why the settled coincidence pairs
did not certify a shell `On` (`shell_witness::on_verdict`,
`shell_witness::check_mutual`). Three of its four arms are reached by
no fixture:

- `Same` is reached: `on_verdict::a_shell_its_partner_does_not_cover_back_refuses`
  (topo) and `on_verdict_rows::a_surface_covered_one_way_refuses_coincident_shell`
  (editor-core), X against X with a pocket.
- `Opposite` (the same shortfall, opposed pairs) is not.
- `Unpaired { face }` is not. Every witness of the shell must lie on
  the other boundary while one face has no settled pair, and the
  reduction refuses the planar shapes that do this first: an
  undeclared aligned twin face is `UndeclaredCoincidence` at the
  continuation scan (`reduce::refuse_undeclared_continuations`). The
  likely reach is a curved face, whose interior the ladder does not
  read (`work/cleave/the-uncut-shell-witness-reads-no-curved-face-interior.md`).
- `Mixed` is not: it wants one shell whose pairs to its partner are
  aligned on some faces and opposed on others.

Each arm only names the refusal's cause: all four refuse, so a
missing fixture cannot hide a wrong answer, only a wrong diagnosis.

## What would close it

A fixture per arm, or a measurement that the reduction refuses every
shape that would reach one (then the arm is unreachable and goes).
