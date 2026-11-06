---
id: the-census-has-no-fault-arm-and-sorts-a-kernel-fault-as-unsupported
kind: issue
title: ValidationError has no kernel-fault arm: the census sorts BoundaryRefusal::Fault and FaceUnread::Uncertified as CensusUnsupported, where the op reads ClassificationInvariant
status: closed
opened: 2026-11-06
priority: P2
cost: M
refs: [the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes]
branch: onecopy/engine-leaves-pins
closed: 2026-11-06
---


From the census-sort fork (designer B).

## What

- **The census misfiles faults.** `census.rs` `line_unread` sorts `BoundaryRefusal::Fault { what }` and `FaceUnread::Uncertified` as `CensusUnsupported(NotCertifiable)`, a pair the census does not support, because `ValidationError` has no kernel-fault arm.
- **The op reads the same arms differently.** It sorts them as `ClassificationInvariant`, a kernel fault, in `locus_through_both_faces`.
- **`read_containment`, in the same file, panics on `StaleFace`.**

So one thing has two readings. Give the census a fault vocabulary, or decide that a census fault panics per D2 row 4. Then sort both arms through it. This lands best with the reshaped `FaceUnread` (the census-sort row's move), but it can go first.

## Closed (2026-11-06, onecopy/engine-leaves-pins)

`[failure.taxonomy]` row 4 decides it: a census fault panics, and `ValidationError` gets no fault arm. The census reads a body tiers 1–3-local passed. Tier 2 refuses a null edge at rest, so `FaceUnread::Uncertified` cannot reach it. `BoundaryRefusal::Fault` is "a kernel fault, never a pose" by its own type. One door, `census.rs` `census_fault` (with `meet_fault` for a `MeetFault`), takes each arm:
- `line_unread`'s `BoundaryRefusal::Fault`;
- the census's other meeting-fault folds (`edge_crossings`, `curved.rs`, `InteriorsUnread::of`; splitdoor's two rows).

`FaceUnread::Uncertified` panics with tier 2's reason. `read_containment`'s `StaleFace` already panicked (`driver_face_stale`). `boundary_crossings`' own stale face, which was typed `CensusUnsupported`, now panics too. The tests `a_boundary_fault_on_a_census_pair_panics`, `an_uncertified_boundary_on_a_census_pair_panics`, `a_stale_face_at_a_census_containment_read_panics` and `a_meetings_{desync,contradiction}_in_the_census_panics` pin them. The op's typed `ClassificationInvariant` for the same states is evidence on `work/cleavetail/classification-invariant-family-types-bug-only-states-against-d9-row-4.md`.
