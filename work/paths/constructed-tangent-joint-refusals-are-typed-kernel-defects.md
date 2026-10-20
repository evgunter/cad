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

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

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

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.
