---
id: num-formatter-prints-an-underflowed-component-as-zero
kind: issue
title: profile's num() renders 1e-180 as 0, so a ZeroDirection refusal tells the author their components are zero when they are not
status: open
opened: 2026-09-11
---


(FIX orchestrator) Found by the `direction-underflow-reports-zero-length`
lane (PR 2359) while executing `profile::unit_from_components` — by
rendering the refusal rather than reading the arm.

`unit_from_components(1e-180, 0.0, …)` refuses `ZeroDirection`, and
**that arm's sentence is right**: it already carries the true cause and
the right recourse, and says that only the ratio of the components is
read. The defect is one layer down. The payload renders through
`profile`'s `num()` formatter, which prints `1e-180` as **`0`**, so the
message reads `got (0, 0)`.

An author whose components are `(1e-180, 0)` is told they are `(0, 0)`.
They check, find a nonzero value, and have no way to reconcile the two
— and the one number in the message that could have told them what
happened is the one the formatter destroyed.

## Why it is its own row

This is not an arm choosing the wrong sentence, which is the class this
program has been closing all day; the arm chose the right one. It is a
**formatter** silently lying in a payload field, so no amount of work
on refusal vocabulary reaches it, and it is invisible to every sweep
that reads match arms or `Display` impls — the lane found it by
rendering a value, not by grepping.

That also means the blast radius is every message `num()` renders, not
this one arm. **Establish that first**: what is `num()`'s rounding
rule, how many call sites does it have, and is there a magnitude below
which it prints `0` for any nonzero input? A formatter that cannot
distinguish a small number from zero is a hazard wherever a refusal's
credibility rests on the number it quotes.

`crates/profile/*` is **BOOL's** territory glob.
