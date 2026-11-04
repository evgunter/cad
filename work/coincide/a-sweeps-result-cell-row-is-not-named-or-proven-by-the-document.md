---
id: a-sweeps-result-cell-row-is-not-named-or-proven-by-the-document
kind: issue
title: A partial revolve's coplanar-caps row names result cells, which the document layer neither names nor proves
status: open
opened: 2026-11-03
---


A half revolve's two caps are one plane, decided from the turn's value:
`sweep::revolve` now leaves them on one surface record and returns the
decision's row in `RevolvedKind::Partial::coplanar_caps`
(`crates/sweep/src/revolve/partial.rs` `finish_partial`, through
`topo::Body::unify_across`). The row's cells are `RowCell::Result` faces:
a sweep has no input body to key them in. `editor-core` reads no such
row: `emit_sweep.rs` ignores the field, and `coincide::name_rows` holds a
result cell `unreachable!` ("the at-rest census is the only producer of a
result cell"), so the `unproven-coincidence` lint never sees a revolve
whose caps hold only at θ = π. The document layer wants a name for a
sweep's output cell in a row and a prover rung for it (the caps are
`plane(frame, θ)` and `plane(frame, 0)`, one polynomially at no θ but π).
