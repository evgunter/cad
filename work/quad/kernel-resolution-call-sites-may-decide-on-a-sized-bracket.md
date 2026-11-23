---
id: kernel-resolution-call-sites-may-decide-on-a-sized-bracket
kind: issue
title: audit the kernel's resolution-form measurement call sites: a site that compares the bracket should pass that comparison as its predicate; weigh a resolution return type with no comparison on it
status: open
opened: 2026-10-31
priority: P3
cost: M
refs: [the-measurement-work-bound-attaches-to-the-question, 6102]
---


## What

DESIGN `[tolerance.measure]` (PR 6102) now says a resolution is only for a consumer that wants the number, and that a consumer that compares the result passes the comparison as its predicate. Ev, on PR 6102: "not sure if there's a good way to prevent it from being misused in the kernel (or if that's likely to be a problem at all)". Every kernel decision must be a named predicate (`[verdicts]`), so a hand comparison of a resolution's bracket would be an unregistered decision. Nothing catches that mechanically.

## Owed

Grep every kernel call of the resolution form (`Settle::Resolution`, `measure_to`, and their Python twins) and give a disposition for each: hands the number on (fine), or compares it (convert to a predicate). Then weigh whether the resolution door should return a type that offers no comparison, so misuse fails to typecheck.

Signed: (QUAD orchestrator)
