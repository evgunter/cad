---
id: the-census-has-no-fault-arm-and-sorts-a-kernel-fault-as-unsupported
kind: issue
title: ValidationError has no kernel-fault arm: the census sorts BoundaryRefusal::Fault and FaceUnread::Uncertified as CensusUnsupported, where the op reads ClassificationInvariant
status: open
opened: 2026-11-06
priority: P2
cost: M
refs: [the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes]
---


From the census-sort fork (designer B).

## What

- **The census misfiles faults.** `census.rs` `line_unread` sorts `BoundaryRefusal::Fault { what }` and `FaceUnread::Uncertified` as `CensusUnsupported(NotCertifiable)`, a pair the census does not support, because `ValidationError` has no kernel-fault arm.
- **The op reads the same arms differently.** It sorts them as `ClassificationInvariant`, a kernel fault, in `locus_through_both_faces`.
- **`read_containment`, in the same file, panics on `StaleFace`.**

So one thing has two readings. Give the census a fault vocabulary, or decide that a census fault panics per D2 row 4. Then sort both arms through it. This lands best with the reshaped `FaceUnread` (the census-sort row's move), but it can go first.
