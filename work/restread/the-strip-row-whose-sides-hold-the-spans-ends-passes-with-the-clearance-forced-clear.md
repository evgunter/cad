---
id: the-strip-row-whose-sides-hold-the-spans-ends-passes-with-the-clearance-forced-clear
kind: issue
title: The strip row whose sides hold the span's ends passes with span_clear forced clear: check what it pins
status: open
opened: 2026-11-02
priority: P4
cost: E
---


Found on review of PR 5900 (`restread/pierce-span-curved-edge`).

## What

`census/pierce_rows.rs`
`a_strip_whose_sides_hold_the_spans_ends_is_not_read_clear` still
passes when `census.rs` `span_clear` is forced to answer `Some(true)`.
Its two span ends read on two different boundary edges, so
`pierce_inside` takes the `(OnEdge, OnEdge)` arm. That arm reads the
half-span and never calls `span_clear`. So the row does not pin the
clearance, whatever its title suggests.

## Fix

Check what the row should pin. If it is the on-boundary arm's
half-span reading, with the two cells told apart, say so in its doc
and check that a mutant of that arm turns it red. If it pins nothing
that another row does not already pin, delete it.
