---
id: shell-classification-volume-refusals-end-as-a-kernel-defect
kind: issue
title: topo: classify_shells ends a zero, straddling or undecided shell volume as a kernel defect, over a caller's body
status: open
opened: 2026-10-21
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P3
cost: E
---


A child of the census in `work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md` (`## Census, second pass`, as of main 69ab780194). Why a typed kernel-only ending is wrong in either D2 class is in that row's "What".

## Sites

- `crates/topo/src/props.rs` `ShellClassifyError::recourse`, `Escalated | ZeroVolume | Straddles` → `Unsized::Defect` (~3718). `Display` renders it at a build through `ending()` (~3727). Doors: the public `classify_shells`, `classify_shells_structural` and `classify_shells_of` over a caller's body, plus the shell op, the piece sort and `stands`. **Class 1, likely**: those doors take any tier-1 body. That includes a flat shell built through the Euler doors, or a sliver void a subtract leaves, whose certified volume enclosure holds zero. The arm's doc calls this "the kernel's, which no tolerance reads otherwise". But a shell of near-zero volume is geometry the user made, and the arm's "(D10)" premise (`props.rs` ~3678, "read off its certified enclosure alone (D10)") is stale.

## Repair shape

- **Class 1**: the shell-volume decision's lever (move the geometry so the shell bounds a clear volume), read at rest where the door reads a caller's body.
- **Class 2**: keep a kernel-postcondition reading only where the caller is an op that certified the volume itself; there it panics per `[failure]`.
