---
id: the-read-at-its-node-relabels-a-cancellation-above-it
kind: issue
title: The decision read answers at its node, ahead of a cancellation its parent would make: max(x + Z, 3) − max(x, 3) is sign_gated, a theorem with the read shut
status: open
opened: 2026-10-02
priority: P2
refs: [DECIDE-9, the-decision-read-answers-theorems-the-must-carry-stations-would-prove]
---


The decision read (`SymRules::decision_read`, in `combine`'s
`min`/`max` and `Select` arms in `crates/geom-core/src/sym.rs`) answers
a node at the node, behind every value-free fold there. It does not
wait for the node's parent. When two distinct nodes have equal early
forms, the read shut mints them as ONE atom and their difference
cancels: a theorem. With the read on, each node is read first, and the
same zero is reached through the arms, so it counts `sign_gated`. That
is the shape `docs/DECIDE-9-SPEC.md` suspected for the pad's and the
bracket's 48. Those 48 turned out to be a different shape (a product
with an ungated zero factor, fixed by DECIDE-9). This shape is real in
the tier all the same.

**The shapes.** These come from DECIDE-9's review probes
(`origin/decide/9-review` @ `d7fbffdd91`,
`sym_root_rows::decide9_review_read_ahead_of_a_cancellation_above`).
Each takes `x ∈ [1, 2]`, `y ∈ [3, 4]` and `Z = sqrt(x)² − x`, which is
zero under rule A in the early walk only.

| shape | read on | read shut |
| --- | --- | --- |
| `max(x + Z, 3) − max(x, 3)` | `sign_gated` | theorem |
| `min(x + Z, 3) − min(x, 3)` | `sign_gated` | theorem |
| `max(x + Z, y) − max(x, y)` | `sign_gated` | theorem |

The first shape is pinned at today's behaviour by
`sym_root_rows::the_read_relabels_a_cancellation_above_its_node_filed_defect`.
A fix flips its read-on label to `theorem`. Disabling the read at
`min`/`max` reds that row, which DECIDE-9 ran.

**On the measured documents.** No such decision showed up. DECIDE-9's
Phase 1 hook re-walked every gated early form read-free and every
gated-zero one at the top rung, on R2's bracket replay at
`certifies_at` and R2's pad leaf at `1e2·ε`. It found no theorem the
read costs, other than the 48 that DECIDE-9 fixed. So no pin moves
today. The class is open for the next document that spells one value
two ways under a `min`/`max`/`Select`.

**Candidate answers** (`docs/DECIDE-9-SPEC.md`, Phase 1 item 3; not
measured on this shape):
- **Settle read-free first.** Try the decision form read-free, and read
  only where that does not settle. This costs a read-free early walk
  over every decision whose early form is gated (DECIDE-9 measured it
  over gated zeros only: pad leaf `26.718 / 35.471 s` against
  `26.613 / 35.633 s`).
- **Read at the decision form, not at interior nodes.** This withdraws
  the read from every interior `Select` and `min`/`max`, which DECIDE-3
  ratified it to decide. Expect `numeric` to move.
- **Keep the node symbolic until its parent has folded.** Mint the atom
  and keep the arm beside it, then substitute only where the parent does
  not settle. This is a placement change in `form_in`/`combine` with a
  cost of its own.

Each must leave alone every decision the read answers where no form
settles, and its `numeric` count.
