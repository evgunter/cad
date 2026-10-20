---
id: merge-faces-undescribed-ending-is-a-typed-kernel-defect
kind: issue
title: topo: merge_faces' undescribed_ending ends a bug-only describe failure in a typed KERNEL_DEFECT_ENDING
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

- `crates/topo/src/merge_faces.rs` `undescribed_ending` (~799, ~800:
  one site with two arms). Doors: `merge_coplanar_faces`, and the
  Boolean's `EdgeUndescribed`. **Class 2, likely**: per its doc, the
  kernel constructed an exact carrier, or its walk broke. The fitted arm
  takes the certification table's ending where the table gives one, and
  falls back to the defect ending where it does not. That fallback is a
  second class-2 state.

The same file also reaches the constant through
`Unsized::Defect.recourse(_, Reading::Build)` (~1108, ~1114). The parent
row's second pass lists those; they are not classified here.

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
