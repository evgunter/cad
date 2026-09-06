---
id: min-clearance-refusal-stringly-twin
kind: issue
title: MinClearanceRefusal ferries (class, String) instead of the ClearanceRefusal it mirrors
status: open
opened: 2026-09-03
---


M10-6 deviation D12 (PR #1685, R2's MINOR): `MinClearanceRefusal` at
`crates/editor-core/src/measure.rs:564` carries the engine's refusal as
`(class: &'static str, payload: String)` — a stringly twin of
`ClearanceRefusal` at `crates/editor-core/src/clearance.rs:526`. Since the
fix pass's MAJ-1 it is also the type an assertion arm dispatches on
(`decide_assertion` reads the class to name the endpoint it refused),
so the string is load-bearing.

Why it was filed, not fixed: `measure.rs` is ungated and `ClearanceRefusal`
is `interval`-gated, so carrying the type means either gating the measure
vocabulary itself or re-exporting the engine's refusal through an ungated
shim — both larger than the defect. The fix belongs with whichever of the
two the layering conversation picks (the same seam as the 1055 valve:
where a curved gate lives relative to `editor-core`).

Acceptance: the measure layer's refusal is the engine's own type (or a
typed projection of it), the class read by `decide_assertion` is an enum
match, and the `(class, String)` pair is gone.
