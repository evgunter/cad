---
id: the-incidence-read-first-freezes-more-sym-nodes-on-the-fillet-pads
kind: issue
title: PCERT E1c's incidence read first raises the plain walks' frozen nodes on the fillet pads about threefold
status: open
opened: 2026-11-01
---


## Finding

Measured in E1c's M-pass triage (PR #5889, head `03d1cad55d`), at ε
1e-9, against main's merge base `c6874ee34e`. On the `Sym` receipts of
`crates/editor-core/tests/sym11_exact_channel_rows.rs:111`
(`[symbolic, numeric, frozen]`), the frozen column on the two fillet
documents roughly triples once every mint reads incidence first:

| document | main | E1c |
|---|---|---|
| `r2_rounded_pad` | `[1616, 1262, 2476]` | `[1716, 1262, 7170]` |
| `r2_filleted_bracket` | `[1540, 1171, 914]` | `[1630, 1171, 4099]` |

`sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` and
`m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials` move the same
way (the pad `(1592, 1250, 2465)` → `(1692, 1250, 7159)`). The symbolic
column rises with it, by 78–100, so the reads add symbolic decisions.
They also freeze about 4 700 more nodes in the plain walks. The numeric
column does not move.

Each of these rows is already red on main for its own reasons, so the
move does not appear as a new red. It costs node budget on the
documents that build fillets, though.

Owed: find which incidence read's forms freeze (the cylinder and torus
arms of `incidence_over_span` are the candidates: the fillets' rims and
rulings) and whether a form the read builds can be kept or shared, so
the plain walks need not freeze them.
