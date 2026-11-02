---
id: the-incidence-read-first-freezes-more-sym-nodes-on-the-fillet-pads
kind: issue
title: PCERT E1c's incidence read first raises the plain walks' frozen nodes on the fillet pads three- to fourfold
status: open
opened: 2026-11-01
---


## Finding

Measured in E1c's M-pass triage (PR #5889, head `de74b14aa0`), at ε
1e-9, against main's merge base `c6874ee34e`. On the `Sym` receipts of
`crates/editor-core/tests/sym11_exact_channel_rows.rs:111`
(`[symbolic, numeric, frozen]`), the frozen column on the two fillet
documents grows three- to fourfold once every mint reads incidence first:

| document | main | E1c |
|---|---|---|
| `r2_rounded_pad` | `[1616, 1262, 2476]` | `[1716, 1262, 8024]` |
| `r2_filleted_bracket` | `[1540, 1171, 914]` | `[1630, 1171, 3071]` |

`sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` and
`m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials` move the same
way (the pad `(1592, 1250, 2465)` → `(1692, 1250, 8013)`). The symbolic
column rises with it, by 78–100, so the reads add symbolic decisions.
They also freeze about 2 000 to 5 500 more nodes in the plain walks. The numeric
column does not move.

Each of these rows is already red on main for its own reasons, so the
move does not appear as a new red. It costs node budget on the
documents that build fillets, though.

Owed: find which incidence read's forms freeze (the cylinder and torus
arms of `incidence_over_span` are the candidates: the fillets' rims and
rulings) and whether a form the read builds can be kept or shared, so
the plain walks need not freeze them.

## 2026-11-02: corrected, after the delta review's N2

The numbers above were measured against the old base `c6874ee34e`, and
the "rows already red on main" premise does not hold on current main.
The delta review found three of these rows green on main `19032ea883`
and red at E1c's head: the pad, `sym_9`'s drive and `boss_union`'s
parting.

Half of the cost was not the read itself:
- **Check 4's re-derivation** on the chart's orthonormalised twin read
  the carrier's incidence a second time, on a different chart, so the
  memo shared nothing.
- **The unmoved second read** at an enclosure scalar took
  min(moved, unmoved).

N2 removed both:
- certify and recertify carry the stored chart's read to check 4
  (`Placement::shown`);
- a line is anchored at its span's midpoint, which makes the one read
  as tight as the two.

What is left per edge: the stated or minted read, the pcurve
certification's read and the planar tier's read. These three are the
same read of the same carrier on the same chart, so the symbolic tier's
memo builds their forms once. They add decision calls (about 54 on the
plate), not forms.

| measure | main `19032ea883` | E1c head (N2) | ratio |
|---|---|---|---|
| plate `Plain/Decision` forms | 20618 | 60682 (141447 before N2) | 2.9× |
| `m10_9` pad, wall | 2.32–2.46 s | 3.30–3.77 s | 1.45× |
| `sym11`, wall | 4.79–5.18 s | 7.18–7.75 s | 1.50× |
| `sym_9` drive, wall | 74.6–75.7 s | 75.2–75.4 s | 1.00× |

Wall time is at CI's opt-level 1, `--test-threads 1`, two alternating
rounds; before N2 the delta review measured the `sym_9` drive at 3.2×.
The orchestrator accepted these at the 1.5× line.

What remains is the one read's own forms: about 40 000 on the plate,
65 sampled distances per carrier carried through the symbolic tier.
Owed, as before: whether the sampled distance can enter the symbolic
tier as fewer atoms.
