---
id: constructed-tangent-joint-refusals-are-typed-kernel-defects
kind: issue
title: profile: four path refusals name the kernel alone; a test-support fixture or an elaborator backstop reaches three, and a fillet leg's definite arm may be input's
status: parked
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
blocked_on: [profile-refusals-say-what-they-mean]
rides_with: profile-refusals-say-what-they-mean
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

## Second-pass sites

(Census second pass, as of main 69ab780194: `## Census, second pass` in the parent row.)

- `crates/profile/src/path.rs` `PathError` `Display`, `UnderdeterminedLeg` (~2170) and `OverdeterminedJunction` (~2175), "(a kernel bug)". Door: `Profile` path elaboration. `OverdeterminedJunction` is **class 2, likely**: it is the elaborator's backstop, and the typed authoring surface establishes that a junction is bound once. `UnderdeterminedLeg` is **class 2, unsure**. It has 14 raisers, in `path/family.rs`, `path/onto.rs` (~161, ~210) and `path/arc_fillet.rs`. `arc_fillet.rs` uses it as a generic backstop for a dispatch that `path.rs` owns, which row 0 may answer by retyping. It also raises it in `map_refusal` (~459) on `LegDegenerate`, and the `fillet_corner_arm` gate's escalation of that arm has a user recourse (`FILLET_LEG_EXTENT_RECOURSE`). The definite arm should tell the same story, which would make it class 1. Probe it before any panic.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Absorbed (2026-10-26, tracker sweep)

`path-kernel-bug-refusals-end-in-a-tag` closed into this row: its
repair was the shape ENCL retired, and its subject is the second-pass
pair above. Today both arms end in "(a kernel bug)"
(`crates/profile/src/path.rs:2174`, `:2179`), with no recourse marker.
`path-pending-fillet-miss-renders-as-an-overdetermined-junction`
(this slate) is the wrong-cause half of the same two arms: if they
panic, its sentences go with them; if one is class 1, its cause
sentence is that row's.

## Rides with `profile-refusals-say-what-they-mean` (2026-10-26)

PROFILEPROSE folded into PROFILETAIL; its refusal-text rows travel as one unit. This row is that unit's spec for its part and closes with its own evidence when the unit lands.
