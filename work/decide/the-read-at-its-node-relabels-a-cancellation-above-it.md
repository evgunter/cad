---
id: the-read-at-its-node-relabels-a-cancellation-above-it
kind: issue
title: The decision read answers at its node, ahead of a cancellation its parent would make: max(x + Z, 3) − max(x, 3) is sign_gated, a theorem with the read shut
status: dispatched
opened: 2026-10-02
priority: P2
refs: [DECIDE-9, the-decision-read-answers-theorems-the-must-carry-stations-would-prove]
cost: M
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

## What Phase 1 found (DECIDE-10)

**The instrument.** A probe patch on `decide/10-phase-1-probe`
(`481c39bf82`, off this branch's base `3a1b96b49`; not for merge). It
puts every candidate behind `CAD_DECIDE10` in `rungs` and `combine`,
counts the read-free walks asked and the labels they moved, and counts
early-walk freezes over a gated kid. The runs:
- **the shapes**: `geom-core`'s `decide10_probe::decide10_shapes`
  (dev, `sym-profile-testing`), each shape decided with the read on
  and with it shut (`SymRules::without_the_reads`);
- **the bracket**: `m10_9_pins_interval::decide10_bracket_replay`,
  `replay_counts` at `certifies_at = 3.870e2·ε`, ε = 1e-9, dev;
- **the pad**: the release leaf instrument,
  `m10_10_leaf_cost_with_and_without_the_algebra` with
  `CAD_M10_10_DOCS=r2_rounded_pad`, `CAD_M10_10_COLUMNS="the ladder"`,
  `CAD_M10_10_TAKES=3`: one whole-box leaf at `1e2·ε`, best of 3;
- **`m10_10_pins`**: the module in dev, plus a probe printing the
  plate's whole split at the nominal (`decide10_plate_split`).

**The candidates as built.** Candidate 1 in six placements, because
where the read-free walk is asked is what it costs:

| | where the walk with the read shut is asked (its own memo) |
| --- | --- |
| 1a | where the early or door form is a GATED ZERO (DECIDE-9's trial (b)) |
| 1c | where the early or door form is gated, zero or not |
| 1b | first, on every decision that reaches the early rung |
| 1d | as 1c, on the decision path only (`discharge_retried`) |
| 1e | as 1b, on the decision path only |
| 1h | as 1d, and on every decision-path decision once a node of the leaf froze over a gated kid |

Candidate 2 walks read-free and reads only the decision's own root
node, where it is a `min`/`max`/`Select`. Candidate 3 walks read-free
with each read's arm recorded beside its atom, and substitutes the arms
into the decision's form, to a fixpoint, where the read-free form is not
zero.

### The shapes

`x ∈ [1, 2]`, `y ∈ [3, 4]`, `Z = sqrt(x)² − x` (zero under rule A, in
the early walk only).

| shape | read on (base) | read shut | 1a | 1c, 1b, 1d, 1e | 1h | 2 | 3 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| S1 `max(x+Z, 3) − max(x, 3)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S2 `min(x+Z, 3) − min(x, 3)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S3 `max(x+Z, y) − max(x, y)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S4 `select(x−3+Z, y, y+1) − select(x−3, y, y+1)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S5 `(max(x+Z, 3) + 1) − (max(x, 3) + 1)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S6 `atan(max(x+Z, 3)) − atan(max(x, 3))` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S7 `max(max(x+Z, 3), y) − max(max(x, 3), y)` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S8 `atan(max(x+Z, 3) − max(x, 3) + x) − atan(x)` | **refused** | theorem | refused | theorem | theorem | theorem | theorem |
| S10 `atan(select(x−3+Z, …)) − atan(select(x−3, …))` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S11 `(max(x+Z, 3) − max(x, 3)) · y` | sign_gated | theorem | theorem | theorem | theorem | theorem | theorem |
| S12 door: `(max(x·x, 3) − max(x, 3)) + (x·x − x)`, `x·x` registered `= x`, `x ∈ [0.9, 1.1]` | **refused** | registered | registered | registered | registered | registered | registered |
| S13 `max(x+Z, P)·Q − max(x, P)·Q` at a 4-term budget, `P = x+y+w`, `Q = u+v` | **refused** | theorem | — | 1d refused, 1e theorem | theorem | — | — |
| C1 `max(x, 3) − 3` | sign_gated | numeric | sign_gated | sign_gated | sign_gated | **numeric** | sign_gated |
| C4 `min(x, y) − x` | sign_gated | refused | sign_gated | sign_gated | sign_gated | **refused** | sign_gated |
| C5 `max(select(x−3, y, y+1), 5) − 5` | sign_gated | numeric | sign_gated | sign_gated | sign_gated | **numeric** | **numeric** |
| C7 `max(x, 3)·y − 3y` | sign_gated | refused | sign_gated | sign_gated | sign_gated | **refused** | sign_gated |
| S9 door: `max(x·x, 3) − max(x, 3)`, `x·x = x` | sign_gated | registered | sign_gated | sign_gated | sign_gated | registered | sign_gated |
| C6 `atan(max(x, 3)) − atan(3)` | numeric | numeric | numeric | numeric | numeric | numeric | numeric |

(— is not run: S13 was built after those runs.) C2 and C3 (`max(x+Z, 3)
− 3`, and S1 plus `max(x, 3) − 3`) read as C1 under every candidate.
What the rows show:
- **The class is wider than the three shapes.** S4–S11 are the same
  mechanism through a `Select`, one level up, under an atom, nested,
  and through a product. S8 and S13 are worse than a relabel: with the
  read on the decision is REFUSED, a theorem with it shut.
  - **S8**: the read-on form of `max(x+Z, 3) − max(x, 3) + x` is `x`
    GATED, and an atom's key carries its argument's gate
    (`Form::digest`), so `atan` of it is a different indeterminate from
    `atan(x)`. The read-on form is gated and NON-zero, so 1a, which
    re-walks gated zeros only, misses it. (`sqrt`, S8s, does not show
    it: rule G keys a root by its argument's value class.)
  - **S13**: the read's arm `P` makes `P·Q` overflow the budget, so
    both products FREEZE, each keyed by its own node, and a frozen form
    carries no gate. The read-on form is ungated and non-zero, so 1d,
    which asks only behind a gated form, misses it; 1e and 1h do not.
  - **S12** is the door's instance. The read-on door form is a gated
    zero, and a gated door form does not discharge (`rungs`), so the
    decision falls to the numeric channel; the read-free door form is
    an ungated zero.
- **S9 is not in the class.** With the read on, the EARLY rung settles
  it (`3 − 3` over the box), ahead of the door, which is SYM-9's ladder
  order. Every shippable candidate leaves it `sign_gated`.
- **C1–C7** are comparisons no form settles. Every shippable candidate
  keeps them `sign_gated`. Candidate 2 sends C1, C4, C5 and C7 to the
  numeric channel. Candidate 3 sends C5 there: its read sees read-free
  kids, and the deep enclosure does not enter a `Select` atom. It also
  sends `sym_root_rows::a_zero_factor_times_a_read_factor`'s
  `copysign(min(x, 3) − x, y)` from `sign_gated` to refused, because a
  substitution at the root does not reach inside the `copysign` atom.
- **`sym_root_rows`** under 1a, 1b, 1c, 1d and 1e: only
  `the_read_relabels_a_cancellation_above_its_node_filed_defect` reds,
  the pin of this defect.

### The documents

| candidate | bracket replay, dev | pad leaf, release: receipt | pad leaf, best of 3 | bracket replay time, dev, one take | read-free walks: bracket / pad per take |
| --- | --- | --- | --- | --- | --- |
| base | 1282 / 37 / 154 / 1092 | 1137 / 2 / 156 / 1373 | 43.155 s; 44.030 s re-taken last | 10.9–11.1 s | 0 / 0 |
| 1a | same | same | 43.305 s | 11.0 s | 37 / 2 |
| 1c | same | same | 44.469 s | 18.2–18.9 s | 803 / 510 |
| 1b | same | same | **85.614 s** | 21.2 s | 3208 / 5147 |
| 1d | same | same | 42.864 s | 10.9–11.7 s | 37 / 2 |
| 1e | same | same | **82.805 s** (one take) | 15.6 s | 1306 / 3103 |
| 1h | same | same | 43.917 s | 11.1 s | 37 / 2 |
| 2 | 1282 / **5** / 154 / **1124** | same | 52.193 s | 14.8 s | — |
| 3 | 1282 / **25** / 154 / **1104** | same | 53.496 s | 15.0 s | — |

Receipts are `symbolic_zero / sign_gated / registered / numeric`;
`frozen` is 1893 on the bracket and 2907 on the pad under every
candidate. The bracket with the read shut reads
`1282 / 0 / 154 / 1129` under every candidate.
- **No shippable candidate moves a decision's value or a count on the
  documents.** Every read-free walk the probe asked moved no label,
  and no early-walk node froze over a gated kid on either document.
- **`m10_10_pins`** is green under every candidate. The plate's whole
  split at the nominal reads `955 / 0 / 148 / 704` under every one,
  predicate by predicate.
- **Candidates 2 and 3 move `numeric`**: 32 and 12 of the bracket's
  `sign_gated` go to it.
- 1e's three-take run printed no line. The table gives its one-take
  re-run.

### What each costs, and the recommendation

- **1a** costs nothing measurable, and leaves S8 refused.
- **1c and 1b** ask the walk on the CONTRADICTION path too
  (`discharge`, run on every definite margin). There a gated zero
  already counts, and a read-free zero under a definite sign would be
  the tier's own soundness bug, so the walk can change no answer
  there. It costs the bracket 70–95 % in dev, and 1b doubles the pad.
- **1e** asks first on every decision-path decision: 1306 walks on the
  bracket, and the pad leaf 43 → 83 s.
- **1d** is 1a's cost and closes S8, but not S13.
- **1h** is 1d's cost on both documents, which froze nothing over a
  gated kid. It labels every shape as 1e does, and as candidate 1's
  literal reading ("read-free first") does.

  The argument is short. An early or door form with no gate, built in
  a leaf where no node froze over a gated kid, is the read-free form
  bit for bit. The walk drops a gate in three places only, and none of
  them can make the two walks differ:
  - a product or `copysign` whose zero factors are ungated: those
    factors are the same in both walks;
  - the arm A0's `Select` does not take: the decision is constant and
    ungated, so the same in both walks;
  - a freeze: the leaf has made none over a gated kid.
- **2** is not shippable (a ratified decision) and moves `numeric`.
- **3**, as a substitution at the decision, moves `numeric`.
  - It loses every per-node fold the read's arm feeds: the arms reach
    the decision's polynomial, not an atom's argument or a `Select`
    the enclosure cannot enter.
  - Made faithful, it re-walks the node's subtree with the arms
    substituted. That is the read-on walk, asked where the read-free
    one does not settle, which is 1b.

**Recommendation: candidate 1 as 1h** (Phase 2). Its costs:
- two more memos per attempt, the early and door walks with the read
  shut, filled only for the decisions that ask them;
- pad leaf within noise of base (42.9–43.9 s against 43.2–44.0 s);
- the contradiction check unchanged.

S8, S12 and S13 move from refused to the answer the read-shut tier
gives (a theorem, `registered`, a theorem). That is the class's own
defect, not a value the read decides: on each, a read-free form
settles.
