---
id: step-import-returns-a-checked-solid-as-a-bare-body
kind: issue
title: StepImport::Solid returns a Body that already passed the at-rest gate, so exporting an imported solid validates it a second time
status: open
opened: 2026-10-25
priority: P3
cost: M
---

Filed by PRED's S18 fix pass (PR 5684, 2026-10-25), which made
`step_export::step_string` / `write_step` take `&topo::AtRestBody<f64>`.
`step_import`'s `StepImport::Solid { body }` hands back a `Body<f64>`
that has already passed `AtRestBody::validate` inside the importer. A
caller that round-trips import → export must therefore validate it
again, and the type forgets a check the importer made. If the
importer's gate is the at-rest gate, returning the `AtRestBody` itself
would carry that fact. Check first whether the importer's gate is
exactly the at-rest gate (tier 3) or a different one. The return type
is EXCH's to change.
