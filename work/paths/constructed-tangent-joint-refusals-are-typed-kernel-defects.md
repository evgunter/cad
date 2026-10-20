---
id: constructed-tangent-joint-refusals-are-typed-kernel-defects
kind: issue
title: profile: TangentJointOutOfRange and TangentJointOnFullTurn end in KERNEL_DEFECT_ENDING; only a test-support fixture reaches them
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

- `crates/profile/src/validate.rs` `ProfileError` `Display`,
  `TangentJointOutOfRange` (~1394). Door: `Profile::validate` over a
  constructed loop. **Class 2, likely**: the lattice never mints such a
  joint. Only `ConstructedLoop::fixture` (`path/program.rs`, behind
  `cfg(any(test, feature = "test-support"))`, and absent from shipped
  builds) builds one.
- `validate.rs` `ProfileError` `Display`, `TangentJointOnFullTurn`
  (~1400). Same door. **Class 2, likely**, for the same reason.

The tests that pin them (`tests/tangent_joints.rs`
`an_out_of_range_constructed_joint_is_refused_typed`, and
`tests/one_segment_loop.rs`) move to `#[should_panic]`. So do the
Python tag rows in `pncad-py/src/tags.rs` (`tangent_joint_out_of_range`,
`tangent_joint_on_full_turn`) and their roster in `pncad-py/src/tests.rs`.

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
