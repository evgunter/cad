---
id: clearance-violated-verdict-names-its-pair-by-bare-keys
kind: issue
title: ClearanceReport's VIOLATED line and serialize's faces= print a cross-body witness pair as two bare FaceKeys
status: open
priority: P3
cost: E
opened: 2026-10-29
---


Found by CLEAR's CLEARTEXT unit (PR 5977), whose item 1 named each
face of a REFUSAL's pair by its side, and placed here by the CLEAR
orchestrator's adjudication of that PR's review: the same defect in a
verdict, which the unit's fence (refusals) left out.

`crates/editor-core/src/clearance.rs`, `ClearanceReport`:

- `render` writes `VIOLATED: {d} m between faces {a:?} and {b:?}`.
- `serialize` writes `verdict violated … faces={a:?}/{b:?}`.

`GeometryWitness::a` is a face of the first selection's body and `b`
of the second's. A `FaceKey` keys one body's arena, so a witness
between two bodies (two instances of one pattern, say) routinely
prints one key value twice for two different faces: the class
`clearance-refusal-names-one-face-twice-across-bodies` closed for the
refusals.

## Repair shape

The refusals' shape (PR 5977): name each face with its side.
`render` says the selections' bodies (the report holds no selection
today, so the door that builds it passes them, as
`ClearanceRefusal::Engine`'s `sides` does). `serialize` labels them
`first=`/`second=`, as `EngineRefusal::payload` does. That moves
every goldened violated report's `verdict` line, so it re-pins them.

