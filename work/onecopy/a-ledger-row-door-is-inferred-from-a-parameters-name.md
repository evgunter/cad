---
id: a-ledger-row-door-is-inferred-from-a-parameters-name
kind: issue
title: The flagged census infers a ledger-row door from a parameter's name
status: open
opened: 2026-11-10
---


## The finding

The flagged census (`crates/geom-core/tests/flagged_census.rs`) decides
that a `k_stats` fn is a ledger-row door from how its parameters are
spelled:
- `row_like` (`flagged_census.rs:124-140`) reads a parameter as a
  ledger row when its name holds `row`, or when it is text under any
  name but `name`;
- `home_doors` (`flagged_census.rs:176-197`) takes every home fn with
  such a parameter as a door, and a forwarding wrapper is keyed by its
  bare name.

So the door is inferred, not declared. A rename can create a door, or
hide one, with no change to what the fn does. This is the fragility the
program keeps removing elsewhere.

## The evidence

PR 6769 (main red). `param_order(row, s, t)` and
`curve_pair::compare(row, …)` named their predicate-name parameter
`row`. The census read them as doors, so every `compare(` call became
a flagged site citing no ledger row. Nothing about the decisions had
changed. The fix renamed the parameter to `name`, which shows that the
census followed the spelling.

## What closes it

The census reads a door from a declaration rather than a parameter's
name. Any of these would do: an attribute on the fn, a registry the
census reads, or the `decide_flagged` signature itself (a door is a fn
that forwards to `decide_flagged`'s row argument). A rename then moves
no door.
