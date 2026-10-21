---
id: merge-faces-undescribed-ending-is-a-typed-kernel-defect
kind: issue
title: topo: merge_faces ends its undescribed edge and its unread pair reach in a typed KERNEL_DEFECT_ENDING
status: open
opened: 2026-10-20
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---

A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (as of main 5df6e6c854); why a typed `KERNEL_DEFECT_ENDING` is wrong in either D2 class is that row's "What" and "Shape of the work".

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

- **Class 2**: `unreachable!` or `debug_assert` per `[failure.taxonomy]` rows 4–5, answering row 0 first; the tests that pin the text move to `#[should_panic]`.

Re-check each `likely` or `unsure` argument before acting on it; one that fails moves the site to class 1.

## Second-pass sites

(Census second pass, as of main 69ab780194: `## Census, second pass` in the parent row.)

- `crates/topo/src/merge_faces.rs` `MergeDecision::ending`, `PairReach` (~1114), through `Unsized::Defect.recourse(_, Reading::Build)`. Door: `merge_coplanar_faces`. **Class 2, unsure**: the merge reads a caller's body under a build reading. An extent that does not read over stored geometry is the file's at rest, so the reading may be mislabelled rather than the state bug-only. Settle that before converting.
- `MergeDecision::ending`, `DeclaredPlanes(Parallel | Norm)` and `DeclaredOffset` (~1108). **Class 2, likely**. **HELD** (declared pairs, D10): it is listed so the census is whole, and it is not this row's to repair.
