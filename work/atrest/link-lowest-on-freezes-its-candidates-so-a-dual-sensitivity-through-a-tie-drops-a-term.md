---
id: link-lowest-on-freezes-its-candidates-so-a-dual-sensitivity-through-a-tie-drops-a-term
kind: issue
title: Link::lowest_on re-enters its candidate parameters with tangent zero, so a stackup sensitivity through a tie candidate is missing a term
status: open
opened: 2026-11-19
priority: P1
cost: M
---


Filed by the SCALAR orchestrator, 2026-11-20, from a read-only survey
of every `Real::from_computed` call (the DL5 fork, PR 7155). **Argued
from the code; no row was run. Execute it before fixing it.**

## What

`topo::Link::lowest_on` (`crates/topo/src/link.rs`) finds its
candidate parameters in `f64` and lifts each with
`t.point_value().map_or(t, T::from_computed)`. At `Dual<f64>`
`point_value` is always `Some`, so every candidate comes back with
tangent zero. The comment beside it justifies that by the envelope
theorem: at a smooth stationary candidate the offset's derivative in
`t` vanishes, so freezing `t` costs the total derivative nothing.

The same loop also pushes the "two wedges equally far" tie candidates.
A tie is a kink of the lower envelope, not a stationary point of
either branch, so `∂f/∂t ≠ 0` there and the frozen-`t` partial is not
the total derivative. Inside material that is where the lowest offset
typically sits.

## Why it matters

The value is consumed as a measurement
(`crates/editor-core/src/eval/measure.rs`, `link_reads`), and
`editor-core`'s stackup evaluates the document at `Dual<f64>` and
reads `measured.deriv` as the sensitivity. So a stackup contribution
through a link whose lowest point is a tie would be reported with a
term missing, and nothing marks it: a re-entered value's zero tangent
looks like any other number.

`topo::Round::lowest_on` was read as sound (an interior minimum of a
smooth convex function, otherwise the ends).

## What closing it takes

A row that differentiates a link measurement whose minimum is a tie,
against a finite difference. If it is wrong: carry the candidate's
tangent (the tie's `t` moves with the inputs along the tie locus), or
forfeit the tangent in the value (ERROR-DESIGN E9), never a comment.
What `from_computed` owes at a dual in general is the DL5 fork's
question (PR 7155); this row is the one instance on the path
production runs.
