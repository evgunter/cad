---
id: read-body-index-and-emission-kernel-defect-endings-split-by-class
kind: issue
title: editor-core: read_body's NoSuchBody ends in KERNEL_DEFECT_ENDING though the caller's index reaches it; the emission category is bug-only
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: M
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

- `crates/editor-core/src/names/interrogate.rs` `InterrogateError`
  `Display`, `NoSuchBody` (~183). Doors: `pub fn
  Evaluation::read_body(var, index)` (`eval/mod.rs` ~255), and the
  interrogation doors. **Class 1, sure**: the index is the caller's,
  and any index past the node's outputs reaches it. The lever is to
  name a body the node outputs, giving the count it has.
  `tests/display_contract.rs` pins the ending, and moves with it.
- `crates/editor-core/src/names/emit.rs` `Category::ending`, `Emission`
  (~659). This is one site covering nine variants: `Duplicate`,
  `EntityNamed`, `KindMismatch`, `ShortTie`, `Unnamed`,
  `MissingUpstream`, `Emission`, `SplitLineage`, `Settle`. Door: every
  node's name emission. **Class 2, likely**: the type's own category 1
  is "an emission BUG". Per-variant reachability was not re-derived, so
  check each one before it panics.

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
