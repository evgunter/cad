---
id: result-not-at-rest-renders-an-empty-findings-list
kind: issue
title: topo: write_result_not_at_rest renders an empty findings list as a typed kernel defect; the list is never empty
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/topo/src/validate.rs` `write_result_not_at_rest`, the `[]`
  arm (~5943). Doors: the Boolean's `ResultInvalid`, and the split's
  `FinishError::ResultNotAtRest` (`splitting/finish.rs` ~879).
  **Class 2, sure**: every caller passes the at-rest gate's findings,
  and that list is non-empty whenever the gate refuses. Row 0 applies:
  both variants can carry a non-empty findings type (a head and a
  tail), and then the arm does not exist.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
