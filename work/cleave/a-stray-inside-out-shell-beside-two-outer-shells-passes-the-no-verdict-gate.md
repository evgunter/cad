---
id: a-stray-inside-out-shell-beside-two-outer-shells-passes-the-no-verdict-gate
kind: issue
title: At a dual, an inside-out shell in a solid that holds two decided Outer shells passes the no-verdict operand gate: check 10's winding read is skipped there, and the piece-count finding is not read
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

Disclosed by PR 4084's fix pass, and not measured. The no-verdict operand gate
(`AtRestBody::gate_unverdicted`, through `validate::wound_negative`) refuses only the
`ShellWinding` findings of tier 3's check 10 (`shell_winding_errors`). It reads them behind a clean
check 7, as tier 3 runs them. Check 10 reads winding only where a solid holds at most one decided
`Outer` shell. The gate deliberately does not read `SolidOuterShells`: that finding is a piece count,
and the split's `Pieces` witnesses at a dual depend on it passing. So a solid holding two decided
outer shells plus a stray inside-out one passes the gate at a dual, and is left to the result sort
(`pieces::pieces_of`).

At f64 the at-rest gate refuses that body through `SolidOuterShells` before any winding question
arises, so only the dual scalars are open.

## Owed

Build the witness (two disjoint outward bricks plus a clockwise wedge, merged into one solid, at
Dual64, through `split` and `union`) and measure what each door returns. Then decide whether the
winding read belongs in the gate whatever the outer-shell count, which would mean reading winding
independently of check 10's guard.
