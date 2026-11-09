---
id: a-torus-refusals-ending-is-dispatched-by-hand-at-four-doors
kind: issue
title: topo: a TorusRefusal's verdict is dispatched to its ending by hand at four doors
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by PR 6634's review (SEAL, `carrier-readings-keep-the-margins-they-decided-on`).

## What

Four doors turn a `geom_brep::TorusRefusal`'s `TorusVerdict` into an ending, each by hand:

- **`boolean::off_ring_torus`** (`crates/topo/src/boolean/mod.rs`). Undecided ends as `BooleanError::Escalated { Torus(half) }`, Refused as `BooleanError::DegenerateTorus { operand, face }`.
- **`join::section_refusal`** (`crates/topo/src/boolean/join.rs`). The same two arms, into `FrameError::{Escalated, DegenerateTorus}`. `frame_refusal` later picks the operand by which surface is the torus.
- **`rest_pair_verdict`'s `OffRing` arm** (`crates/topo/src/boolean/contact_verify.rs`). Undecided ends as `ContactRefusal::Escalated`, Refused as `NotCertifiable`.
- **The merge's `MergeCoplanarError::DegenerateTorus`** (`crates/topo/src/merge_faces.rs`). It carries the refusal whole, and its `Display` reads `TorusRefusal`'s own ending.

The four target error types differ, so no one helper serves them as they stand.

## Shape

One reading of a refusal, `(convention, Result<Refused, Indeterminate>)`, that each door ends in its own type. Alternatively, the Boolean's two doors share `off_ring_torus` once `FrameError` carries the face. Decide which before folding.
