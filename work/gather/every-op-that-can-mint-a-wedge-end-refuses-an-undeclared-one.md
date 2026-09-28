---
id: every-op-that-can-mint-a-wedge-end-refuses-an-undeclared-one
kind: issue
title: with cusps derived at rest, every op that can mint a wedge-0/2π edge must trace it to a declared input or refuse typed at its own door
status: open
opened: 2026-09-28
priority: P3
cost: M
refs: [product-gate-refuses-a-declared-cusp-sweep-the-verb-now-declares]
---


Filed with Ev's ruling on PR 3317 (2026-09-28): a cusp is legal at rest
iff jet-determinate, so tier 3 no longer catches an op that mints a
knife edge nobody asked for. That refusal moves to the ops.

Known doors today (the designers' reading, not an audit): the profile
refuses an undeclared zero-turn joint (`JunctionCusp`), the curved
boolean refuses an undeclared tangent operand pair
(`CurvedBooleanUnsupported`; TANG's
`declared-cusps-second-order-wedge-arm` item 3 is the planned routing),
and STEP import refuses cusps. Fillet and offset mint no wedge ends.

Owed: sweep every body-producing op for a path to a definite wedge end,
list each with its door (or its refusal, added), and pin one row per
door. Future shell/draft/offset work inherits the obligation. Designer
D's optional companion: report `MaterialWedge::Cusp | Slit` through the
marks channel as a diagnostic (never a gate), and an importer option
that refuses on it.

## Findings from the unit that derived the arm (branch `gather/derive-cusp-legality`)

Not an audit — what the implementing lane met on the way:

- **STEP import no longer refuses a cusp.** `crates/step-import/src/lib.rs`
  holds no cusp or wedge logic of its own (`grep -i 'cusp\|wedge'` over
  `crates/step-import/src` is empty); its refusal was
  `topo::validate_geometric`'s `UndeclaredCusp`, reached through `gate`
  and `gate3`. With the arm derived, a file whose solid carries a
  jet-determinate wedge-0/2π edge passes both gates, and nothing else
  in the importer is known to refuse it (unprobed: no cusp fixture was
  imported). "STEP import
  refuses cusps" above is therefore stale; the importer needs its own
  door (Designer D's importer option) or a ruling that a file's cusp is
  the file's declared intent.
- **The curved boolean, one configuration probed:** a box minus a
  cylinder internally tangent to one side face (the crescent pair at
  the tangency, `declare: None`) refuses typed at the op —
  `CurvedPierceUnsupported` — before any wedge end is minted.
- **Downstream consumers now receive cusp bodies through `product`**
  (a `.cusp()` extrude, revolve, pattern, split half and a boolean clear
  of the strut all gather — `crates/editor-core/tests/m10_2_measure.rs`'s
  `a_cusp_*` rows). Whether each wedge-conditioned consumer (fillet,
  chamfer, shell/offset, mesh sizing, export) refuses a wedge-0/2π edge
  typed, per D1, was not checked here; before this change the product
  gate refused such a document, so no consumer met one through the
  editor.
