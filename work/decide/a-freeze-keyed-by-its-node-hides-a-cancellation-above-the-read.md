---
id: a-freeze-keyed-by-its-node-hides-a-cancellation-above-the-read
kind: issue
title: A freeze is keyed by its node, so a cancellation above the decision read that crosses a freeze is refused (S13)
status: open
opened: 2026-10-12
priority: P3
refs: [DECIDE-10, the-read-at-its-node-relabels-a-cancellation-above-it]
cost: M
---


**The shape (S13).** `x ∈ [1, 2]`, `Z = sqrt(x)² − x` (zero under
rule A in the early walk), `P = x + y + w`, `Q = u + v`, at a
four-term budget:

    max(x + Z, P)·Q − max(x, P)·Q

With the decision read on, each `max` is read at its node and takes
the arm `P`; `P · Q` is six terms, so both products FREEZE, each to
an indeterminate keyed by its own node, and the difference is not the
zero form. The decision is refused. With the read shut both `max`
nodes are one atom `M`, both products are `M · Q`, and they cancel.

Pinned at today's behaviour by
`crates/geom-core/tests/sym_root_rows.rs`'s
`a_cancellation_above_a_frozen_read_is_refused_with_the_read_on`
(read on refused, read shut proven). A fix flips the read-on half.

**Why it is not DECIDE-10's.** DECIDE-10 deleted the label kind and
the read's `gated` flag, which answered S8 (an atom keyed by its
argument's polynomial) and S12 (the door's gated refusal). S13 is not
a label: the freeze, not the read's gate, keeps the two products
apart (`Form::digest` keys an atom by its argument's form, but
`form_in` keys a frozen node by its node id).

**Candidates** (not measured):
- **Key a freeze by its content**: the operation over its kids'
  forms, so two nodes whose kids' forms are equal freeze to one
  indeterminate. It is sound by the argument the atom key already
  rests on (equal forms denote one real wherever clause 1 admits),
  and it touches every freeze in the tier, so `frozen` and the
  measured receipts move with it.
- **A measured read-free retry mask**: a `SymRetry` attempt with
  `decision_read` shut, asked only into the first attempt's silence.
  DECIDE-10's draft (PR 4095) measured a read-free walk asked on
  every decision-path decision at 25× on the tilted family's
  start-cap row; a retry asked only where the first attempt refused
  is a different population and has not been measured.

No measured document reaches the shape: DECIDE-10's draft found no
early-walk node frozen over a read on the bracket replay, the pad's
leaf or the plate (`work/decide/the-read-at-its-node-relabels-a-cancellation-above-it`).
