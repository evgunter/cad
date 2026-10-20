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

(ENCL implementer, from the census in
`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`,
on main 5df6e6c854.)

`[failure]` (D2) splits every state in two: one input reaches, which
is a typed refusal whose ending must name the decision's lever, and one
only a kernel bug reaches, which MUST panic ("downgrading one … to a
typed error launders a bug into a supported outcome"). A typed refusal
ending in `geom_core::KERNEL_DEFECT_ENDING` is therefore wrong in either
class. The one exception is a kernel approximation limit, which takes
`[tolerance.recourse]`'s last resort (`KERNEL_LIMIT_RECOURSE` /
`KERNEL_LIMIT_LAST_RESORT`), not this ending.

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

- **Class 2** (only a kernel bug reaches it): `unreachable!` or
  `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0
  first (a type that cannot hold the state is preferred). The tests
  that pin the refusal's text move to `#[should_panic]`, or are deleted
  where they only built the error value by hand.
- **Class 1** (input reaches it): the decision's own lever, written in
  its own words (`[tolerance.recourse]` (i)), and a test that reaches it
  through the door.
- **Class 3** (an approximation limit): the last-resort recourse.

Re-check each `likely` or `unsure` argument before acting on it: one
that fails moves the site to class 1.
