---
id: step-export-refusals-cannot-name-entities
kind: issue
title: step-export refusals cannot name the entity they refuse on — no Part 21 id exists before emission
status: open
opened: 2026-09-04
refs: [D343, 1854]
---


Filed from EXCH-E1's report (PR #1854), which measured the premise
while executing D343. The eight arena-key refusal spellings in
`step-export/src/lib.rs` were LEFT with argument: they fire BEFORE
the refused entity is emitted, so no Part 21 `#id` exists to name —
the real asymmetry with step-import, which names ids read from a
file. `topo::EntityId::Display` is the ratified spelling they inline.
The only genuine fix is a walk-position ordinal (or equivalent
caller-meaningful coordinate) threaded through
`advanced_face`/`closed_shell`/`manifold_solids`/`volume.rs` — a
design change to the emitter's error plumbing, not a Class-B spelling
fix. Related datum received by that lane (uv-j): step-import names a
key typed (`VertexWithoutPoint`) while step-export uses
`Corrupt { what }` with no key — two deliberate vocabularies whose
reconciliation is the same design conversation. Sibling in-crate
finding recorded in the PR: `carrier_kind` answers "nurbs curve" for
`Curve3::nurbs_placeholder()` (unlike `surface_kind`'s placeholder
distinction), so `CurvedShellClassification` would misname a
placeholder — `printable_carrier` spells the literal today.

Signed: (EXCH orchestrator)
