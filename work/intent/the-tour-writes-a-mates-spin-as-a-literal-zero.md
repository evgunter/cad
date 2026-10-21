---
id: the-tour-writes-a-mates-spin-as-a-literal-zero
kind: issue
title: the tour writes each mate's spin as a literal 0 rad, which mints a variable where the design writes the constant 0
status: open
opened: 2026-10-21
priority: P2
cost: E
---


## What

`demos/tour/src/assembly.rs` `seated_at` (line 352) writes a plane
mate's spin as `Formula::literal(0.0, Dimension::Angle)`. Under
`[variables.literals]` a literal is a variable, so each of the bench's
three mates mints an angle variable that nobody wrote. The design's
constant zero is `Formula::zero`, which `Placed::identity` already
writes. DESIGN-FORK-LOG rows 159 and 161 found the same defect
elsewhere: literals written where the design says zeros.

Found by the seat designer pair (2026-10-21,
`a-mate-placed-seat-is-unproven-at-the-doors-budget`). At `Sym` the
literal binds as a symbol, so the bench's seats need `R(a, θ)·a = a`
through a symbolic θ. That is part of why they don't prove at the
door's budget today.

## Owed

`seated_at` writes `Formula::zero(Dimension::Angle)` for the spin. Check
`place_at`'s returned values, and every other demo or helper writing
`literal(0.0, …)` where the constant is meant, for the same defect.
Re-bless what moves, with the reason.
