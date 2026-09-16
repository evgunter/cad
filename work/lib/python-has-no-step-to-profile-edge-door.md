---
id: python-has-no-step-to-profile-edge-door
kind: issue
title: Python has no step-to-profile-edge door: canonical_segments_of and StepSegmentsError reached the facade unbound
status: open
opened: 2026-09-16
---

Filed by EDIT's unit
`authored-step-to-canonical-segment-map-has-no-home` (DM8), which built
the door. `crates/pncad-py/*` is LIB's ground, so the binding is LIB's
call and was not taken there.

## What landed

`ProfileProgram::canonical_segments_of(structure, naming, loop_, step)
-> Result<Vec<ProfileEdgeRef>, StepSegmentsError>`
(`crates/editor-core/src/program.rs`): which profile edges one authored
step of a loop program became, composed from the replay's per-step
segment span and the permutation canonicalization recorded, refusing
rather than guessing.

`StepSegmentsError` is carried through the façade
(`crates/pncad/src/document.rs`, beside `RecordedProgramError`), which
is what the document-layer completeness guard requires. The Python
census then reported it as a curated name with no Python spelling, so
it is dispositioned in `crates/pncad-py/tests/test_binding_census.py`
as `gap: B-STEP-SEGMENTS`, with that family's charter added to
`FAMILIES` — the same shape `B-EDGE-KIND` took when the edge
carrier-kind read arrived without its binding.

## What closing it needs

The charter in `FAMILIES` states it, and the first item is a Rust-side
dependency rather than a binding:

1. **The records are not reachable.** The door takes
   `profile::ProfileStructure` and `eval::ProfileNaming`. `ProfileValue`
   carries the naming; the structure lives only on `eval`'s
   `pub(crate)` `ProfilePre`. Until it is sited somewhere a consumer can
   read, no binding can call the door. That siting is EDIT's or WIRE's,
   not LIB's, and VIEW's
   `focus-marking-is-per-node-not-per-segment` names the same
   dependency from the viewer's side.
2. The method with its typed refusal, a `pncad.pyi` entry, and one
   Python row asking a reversed loop for a step's edges and naming the
   wall each addresses.

A Python caller can author a profile program today and cannot ask which
profile edges one of its steps became, which is the gap this row
records.
