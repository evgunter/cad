---
id: python-has-no-step-to-profile-edge-door
kind: issue
title: Python has no step-to-profile-edge door: canonical_segments_of and StepSegmentsError reached the facade unbound
status: open
opened: 2026-09-16
priority: P3
cost: D
---

Filed by EDIT's unit
`authored-step-to-canonical-segment-map-has-no-home` (DM8), which built
the door. `crates/pncad-py/*` is LIB's ground, so the binding is LIB's
call and was not taken there.

## What landed

`ProfileProgram::profile_edges_of(structure, naming, loop_, step)
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
   read, no binding can call the door — the door has no caller outside
   its own tests today, and this is why. That siting is **already a
   filed row**, WIRE's:
   `work/wire/section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made.md`,
   whose subject is precisely that `NodeValue` does not carry
   `ProfilePre`. VIEW's `focus-marking-is-per-node-not-per-segment`
   names the same dependency from the viewer's side. Nothing new was
   filed for it here: a second row would be a duplicate of WIRE's.
2. The method with its typed refusal, a `pncad.pyi` entry, and one
   Python row asking a reversed loop for a step's edges and naming the
   wall each addresses.

A Python caller can author a profile program today and cannot ask which
profile edges one of its steps became, which is the gap this row
records.
