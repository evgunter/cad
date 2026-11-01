---
id: the-read-at-its-node-relabels-a-cancellation-above-it
kind: issue
title: The decision read answers at its node, ahead of a cancellation its parent would make: max(x + Z, 3) − max(x, 3) is sign_gated, a theorem with the read shut
status: closed
opened: 2026-10-02
priority: P2
refs: [DECIDE-9, the-decision-read-answers-theorems-the-must-carry-stations-would-prove]
cost: M
closed: 2026-10-13
---


The decision read (`SymRules::decision_read`, in `combine`'s
`min`/`max` and `Select` arms in `crates/geom-core/src/sym.rs`) answers
a node at the node, behind every value-free fold there. It does not
wait for the node's parent. When two distinct nodes have equal early
forms, the read shut mints them as ONE atom and their difference
cancels: a theorem. With the read on, each node is read first, and the
same zero is reached through the arms, so it counts `sign_gated`. That
is the shape `docs/DECIDE-9-SPEC.md` (deleted; `docs/doc-ledger/decide-9-spec.md`) suspected for the pad's and the
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

**Candidate answers** (`docs/DECIDE-9-SPEC.md` (deleted; `docs/doc-ledger/decide-9-spec.md`), Phase 1 item 3; not
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

## Question, and Ev's answer

Should a label say what the tier's own proof relied on, or the least
that any proof would need?

Some zeros can be proved two ways. Take `max(x + Z, 3) − max(x, 3)`:
- The tier reads `x + Z ≤ 3` and `x ≤ 3` on the box, replaces both
  maxes by `3`, and gets `3 − 3 = 0`. That proof relied on a sign
  read, so it is `sign_gated`.
- Simplifying `x + Z` to `x` makes the two maxes one expression. That
  proof reads no sign, so it is `symbolic_zero`.

(a) **The proof found.** The label is `sign_gated`, and it costs
nothing.

(b) **The least needed.** The label is `symbolic_zero`. To know that,
the tier has to try the proof again with sign reads switched off.
DECIDE-10's draft (branch `decide/10-read-behind-the-parent`) does
this. On the tilted derived-frame family, the only measured documents
where such zeros occur, it costs 25× and 21×:
- the start-cap/FlipZ row goes from 0.525 s to 13.35 s;
- the boss row goes from 0.130 s to 2.74 s.

The cost is the second proof's own size: 368 distinct roots and
read-free frames up to 1,391 terms. No memo removes it.

Either way every decision gets the same answer. Only the label, which
feeds reports, differs.

Two designers weighed it independently and converged on (a) after one
round:
- What a document needs the read for is the differential against
  `SymRules::without_the_reads`.
- The ladder keeps asking its rungs in order of strength, which keeps
  the draft's cheap top-rung check.
- The three answers the read loses are each fixed at their own cause:
  - **S8:** an atom is keyed by its argument's polynomial, with the
    gate carried on the form.
  - **S13:** a freeze is keyed by its operation over its kids' forms,
    not by its node.
  - **S12:** a label is a set of premises (`{}`, `{read}`,
    `{registration}`, `{read, registration}`), and
    `MarginDiag::jointly` is a union. On main, `jointly` reports
    `sign_gated ⊔ registered` as `registered`, which drops the read.
- The spec's invariant asked for (b), so it is retired.

**Ev's answer (2026-10-12): the kind is deleted.** Ev first accepted
(a), with a condition: "only if it can be made certain that the
decision type cannot determine anything important". Ev said keeping
it as debugging information or deleting it were both acceptable. Both
designers were asked how to make that certain, and Ev chose full
deletion ("full deletion sounds good!"):
- **No decision carries how a zero was proved.** `Theorem`'s kinds
  go; `Zero::Proven` carries nothing. `MarginDiag`'s theorem is a
  "proven" bit. `jointly` is proven if both are. `deferred::discharged`
  answers proven or not. `topo::Discharge` is `{Numeric, Proven}`, and
  the coincidence records' `Rung::MarginIdentity` carries nothing.
  This is certain by type: no reader can branch on a kind no type
  holds.
- **The read's `gated` flag on forms goes, with its bit in
  `Form::digest`**, since it existed only to compute the kind:
  - S8 is fixed by that: an atom is keyed by its argument's
    polynomial.
  - `zero_factors_gate` retires. It is DECIDE-9's zero-product rule,
    which only ever chose a label, and it retires by this ruling.
  - The door's refusal of a gated zero goes (S12 is answered).
  - `door_zero` sees the read's zeros.
- **The receipt and the K vocabulary lose the split.**
  - `SymCounts` keeps `symbolic`, `numeric`, `frozen`, `retried`, and
    the two registry soundness columns.
  - The K tokens `sign_gated` and `registered` retire.
  - Per-kind pins re-baseline to totals.
- **What the read, or the registry, buys a document is the
  differential** against the tier with that dial off
  (`SymRules::without_the_reads`, `registered: false`), asserted in
  decisions and verdicts.
- **Rule C (`signed_root`)** rests on a sign of the box, not on a
  branch the construction took, so a zero it reaches is not E12's
  claim. It stays off in every shipped rule set and in the coincidence
  replay. Retiring it is DECIDE's to schedule.
- **What stays:**
  - S13, a freeze keyed by its node, stays a filed reach item. It is
    fixed by keying a freeze by its content, or by a measured retry
    mask.
  - DECIDE-10's shut walk never ships.


## What DECIDE-10 shipped

The label kind is deleted (branch `decide/10-delete-the-kind`):
- `MarginDiag::proven` is one bit; `Theorem`, `Discharge`'s kinds,
  the forms' `gated` flag, `zero_factors_gate` and rule C
  (`signed_root`, with rule G's certified side-condition source) are
  gone. `topo::Discharge` is `{Numeric, Proven}`, and
  `Rung::MarginIdentity` and `Rung::CarrierVerdict` carry nothing.
- `SymCounts` has one discharge column, `symbolic`; the K tokens
  `sign_gated` and `registered` retire, and `k-lint` refuses them.
- The contradiction check asks the tier's own algebra with the door
  shut, so a registered zero under a definite sign stays the door's
  event (`registrations_contradicted`), as it was.

**The shapes**, as rows (`geom-core/tests/sym_root_rows.rs`, section
5), proven with the read on / with it shut:
- S1–S8, S10, S11: proven / proven. S8 is answered: an atom is keyed
  by its argument's polynomial.
- C1–C7: proven / not. C6 moved from numeric to proven by design.
- S9, S12 (the door's): proven / proven. S12 is answered.
- S13: refused / proven, filed as
  `a-freeze-keyed-by-its-node-hides-a-cancellation-above-the-read`.

**What a dial buys** is a differential row: the read on the tilted
family (`m10_what_the_read_buys_on_the_tilted_caps`: the end cap and
`FlipZ` refuse at `newell_plane_residual` with the read shut, the
start cap reads the same either way), and the registry on the link,
the bracket and the pad (`m10_9_what_the_registry_buys`).

Closed by DECIDE-10 (PR #4929), 2026-10-13.
