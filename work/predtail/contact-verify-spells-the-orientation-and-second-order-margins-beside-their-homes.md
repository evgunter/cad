---
id: contact-verify-spells-the-orientation-and-second-order-margins-beside-their-homes
kind: issue
title: contact_verify spells the orientation lever, the second-order margin and the parallelism lever choice beside certify's and dihedral's homes
status: open
opened: 2026-10-25
priority: P1
cost: M
---

Cut from `lever-arm-fold-six-hand-rolled-siblings` by the PRED
re-census (2026-10-25, main `2d5677e1`). These are the parts of that
row that sit in `topo::boolean::contact_verify`'s
`tangent_locus_relation`. `contact_verify` is the declared-contact
verifier, and the census reads it at rest. D10 stage 4 retires declared
pairs and `ContactClass` on mates (INTENT, CONTACTHOLD), and
`tangent_table` still takes `declared: bool`, so this ground is under
the refactor hold and may be rebuilt or deleted. A unit taken now would
likely be undone.

What is spelled twice:
- **`contact_tangent_opposed`** levers `n1·n2` by `extent`, as certify's
  `tangent_gap_orientation` (`gap_orientation`) does. Neither reads
  `classify_material_pairing`, which levers by the folded arm, so the
  original "undisclosed twin" premise no longer holds: the twin is
  certify's orientation decide. Whether an orientation decide levered by
  extent, against one levered by the folded arm, is a deliberate split
  is the open question.
- **`contact_tangent_second_order`** and `contact_tangent_second_order_at`
  meter `Margin::sagitta(|κ_rel| − drift, arm)`. That is a drift-widened
  hand-rolled sibling of `geom_brep::tangent_second_order`.
- **The parallelism lever choice** is "`levered_inv(sin θ, |κ_rel|)`
  when the second order is definitely positive, else the folded value".
  Certify (`named_by_parallelism`) and `contact_tangent_parallel` each
  spell it. The margin itself already has one home,
  `certify::folded_parallelism`.

When the hold fires, re-read `contact_verify` against the tree first:
whether these sites survive stage 4 is the first thing to settle.

## Moved (2026-10-26)

PRED folded into PREDTAIL (Ev, in chat, 2026-10-26): PRED was handed back
`blocked` with this row and one other parked, and PREDTAIL now holds
PRED's parked rows beside its tail. Id, body and blocker unchanged.

**Re-read 2026-10-29 (INTENT, D10 hold):** re-parked on `a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band`. The sites survived stage 4: `contact_tangent_opposed`, `contact_tangent_second_order` and `contact_tangent_parallel` are at `crates/topo/src/boolean/contact_verify.rs:553`, `:634` and `:651`. `tangent_locus_relation` still takes `declared: bool` (`:291`), and that posture bridges in-band second-order residue. Whether the posture survives is that row's question, and it decides the second-order arm this row would fold.
