---
id: cap-plane-refusals-end-as-typed-kernel-defects
kind: issue
title: sweep: CapPlaneError's Orientation and EdgeOn end in KERNEL_DEFECT_ENDING for states a validated profile never reaches
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

## Sites

- `crates/sweep/src/swept.rs` `CapPlaneError` `Display`, `Orientation`
  (~795). Doors: the extrude, revolve, sweep and loft verbs' `CapPlane`
  arms. **Class 2, likely**: the type's own doc says it is
  "unreachable from a validated profile; surfaced rather than trusted".
  Validation certifies the region's winding on the same plane the cap's
  Newell normal is read against.
- `swept.rs` `CapPlaneError` `Display`, `EdgeOn` (~801). Same doors.
  **Class 2, likely**, for the same reason.

`swept.rs`'s `a_caps_undecided_orientation_ends_as_a_defect` writes the
ending out as a literal, so a sweep for the constant does not find it.

## Repair shape

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
