---
id: the-decision-read-answers-theorems-the-must-carry-stations-would-prove
kind: issue
title: The decision read answers 32 of the pad's and 16 of the bracket's decisions that are theorems with it shut
status: closed
opened: 2026-10-01
priority: P2
cost: M
refs: [rule-g-trades-sixteen-of-the-links-carrier-on-surface-2, 2468]
closed: 2026-10-02
---

## What was measured (LINALG's merge of `main` into `props/sign-hull`, 2026-10-01)

`m10_9_pins_interval`'s replay (`replay_counts`, one whole-box replay at
`Sym<Interval>` over each document's `certifies_at · ε` box, ε = 1e-9,
release), receipts as `symbolic_zero / sign_gated / registered / numeric`:

| document | tree | shipped | `without_the_reads` |
| --- | --- | --- | --- |
| R2's rounded pad | `props/sign-hull` before the merge (`3dc3becc0`) | 890 / 6 / 150 / 907 | 894 / 0 / 150 / 909 |
| R2's rounded pad | merged with `main` | 893 / 34 / 150 / 1002 | 925 / 0 / 150 / 1004 |
| R2's filleted bracket | `props/sign-hull` before the merge | 1104 / 7 / 144 / 766 | 1106 / 0 / 144 / 771 |
| R2's filleted bracket | merged with `main` | 1105 / 21 / 144 / 783 | 1121 / 0 / 144 / 788 |

With the read shut, every theorem is still a theorem. By arithmetic
alone, the merged pad's 925 is the pre-merge 894 plus 31. That would
fit `main`'s 28 must-carry station theorems
(`geom_brep::must_carry_over_edge`'s `classify_dihedral` gate, seven
stations on each of the pad's sixteen rule-reached edges) plus its three
fillet run-out theorems (`path_run_out_carrier`). No split has shown
it; DECIDE-9's split found 32 `dihedral_wedge`, below. With the read on, 32
of the pad's and 16 of the bracket's decisions that reduce to the zero
form are answered by the read first, and count `sign_gated`. The
decision stays discharged; the claim on it is the read's bracket, not a
theorem — the weakening `m10_9_no_registrant_lies_on_any_measured_document`
names.

`sym.rs`'s rule table says the read fires on "the frame's conditioning
comparisons, which no form settles: `sign_gated` where it fires and
never `symbolic_zero`", and the module header orders it BEHIND every
value-free fold for exactly this reason. These rows are forms that DO
settle, so either the ordering does not hold at the site these
decisions reach (a `min`/`max` or `Select` read at its own node before
the residual above it reduces), or the statement is narrower than it
reads. The pre-merge tree already shows the class at 4 and 2; `main`'s
station gate multiplied the pad's by eight.

## Not established

Which predicates the 32 and 16 are. The per-predicate split
(`m10_10_splits_at_the_nominal_under_a_rule_set`, `CAD_M10_10_DOCS=r2_rounded_pad`)
was killed at 388 s on a 15 GB box before printing — the shape report's
rendering of the pad's blocked residuals. That the pad's 28 are the
`dihedral_wedge` stations is read off the arithmetic (925 − 894 = 31 =
28 + 3), not off a split.

## Home

DECIDE: the read's ordering is `geom_core::sym::signed`'s and the
session's. The pins re-baselined to the shipped numbers with this cause
stated (`m10_9_pins_interval`, `sym_9_retry_interval`).

## What Phase 1 found (DECIDE-9)

**The instrument.** A throwaway patch, not committed, hooked at
`rungs`' early rung in `geom_core::sym`. For every first-attempt early
form that is a GATED zero it did three things:
- walked the same decision again under the same rules with
  `decision_read` shut, in a memo of its own;
- asked the top rung over the plain residual;
- printed the predicate (`k_stats::current_predicate`), the
  `Select`/`min`/`max` nodes whose read-on early form is gated while no
  kid's is (the node the read answered at), and the decision root's
  kids rendered both ways.

A second mode also walked read-free at every gated NON-zero early form,
and printed any decision that walk proves. Such a decision would be a
theorem the read costs outright.

**The tree.** Every number below is from `8b640cded`: this branch with
`origin/main` merged in. `props/sign-hull` had been merged into `main`
as `dc39bce95`. Every run was first taken on the branch's original base
`494d477ef` and then re-taken on the merged tree. The attribution, the
shape and the candidates' restorations read the same on both trees.
What differs is `main`'s own `registered`/`numeric` move, which `main`
re-baselined in the pins' notes (bracket `144 / 783 → 146 / 781`, pad
`150 / 1002 → 148 / 1004`), and the timings, given per tree below. The
runs:
- **the bracket**: `m10_9_pins_interval`'s replay (`replay_counts` at
  `certifies_at = 3.870e2·ε`, ε = 1e-9, dev), shipped
  `1105 / 21 / 146 / 781`, read shut `1121 / 0 / 146 / 786`. Its
  nominal split, `m10_10_splits_at_the_nominal_under_a_rule_set` with
  `CAD_M10_10_DOCS=r2_filleted_bracket` and `CAD_M10_10_RULES` set to
  `shipped` and then `no_reads`.
- **the pad**: the release leaf instrument,
  `m10_10_leaf_cost_with_and_without_the_algebra` with
  `CAD_M10_10_DOCS=r2_rounded_pad`, `CAD_M10_10_COLUMNS=rules`. That is
  one whole-box leaf at `1e2·ε`. It reads `893 / 34 / 148 / 1004`, the
  same receipt the pin reads at `2.083e3·ε`.

### Which predicates, which nodes

| document | predicate | shipped | read shut | the hook's reading |
| --- | --- | --- | --- | --- |
| bracket, nominal split | `dihedral_wedge` | 0 / 16 / 0 / 216 | 16 / 0 / 0 / 216 | — |
| bracket, nominal split | `line_span` | 6 / 5 / 0 / 1 | 6 / 0 / 0 / 6 | — |
| bracket, replay at `certifies_at` | `dihedral_wedge` | 16 `sign_gated` | — | all 16 read-free THEOREM; top rung no |
| bracket, replay at `certifies_at` | `line_span` | 5 `sign_gated` | — | read-free non-zero; top rung no |
| pad, leaf at `1e2·ε` | `dihedral_wedge` | 32 `sign_gated` | — | all 32 read-free THEOREM; top rung no |
| pad, leaf at `1e2·ε` | `line_span` | 2 `sign_gated` | — | read-free non-zero; top rung no |

Every other predicate on the bracket's split reads the same at both
dials. The second mode printed nothing on either document: no gated
non-zero early form is a theorem with the read shut.

**All 48 are one shape, at one node.**
- The decision root is a `Mul`.
- Its first factor's early form is the zero polynomial, with the read
  on and with it shut.
- Its second factor is `min(min(9007199254740991·2^971, ½·|…|), c)`:
  the lever arm.
- The read answered at the INNER `min`, the one node per decision whose
  read-on form is gated while no kid's is. Its read-on early form is
  the arm `½·|…|`; read-free it is the atom `min(f64::MAX, ½·|…|)`.
- The decision's early form is the zero form both ways: gated with the
  read on, ungated with it shut.

**The pad's 32 are all `dihedral_wedge`, not 28.** The predicate cannot
tell `must_carry_over_edge`'s `classify_dihedral` stations from the
other `dihedral_wedge` callers. The pre-merge tree was not re-run here.
So "28 of the 32 are the stations'" is still the arithmetic's claim
(925 − 894), not this split's. The 2 `line_span` on the pad and the 5 on
the bracket are the read doing its job: no form settles them, and with
the read shut they are numeric.

### The mechanism, replaced

The suspected shape was a node-level read ahead of a cancellation above
it, `max(A, B) − max(A, B)`. None of the 48 has that shape.

Each one is `dihedral_wedge`'s margin, `Margin::levered(sin_theta, arm)`
in `geom_brep::dihedral::wedge_decided`, which is `sin θ · arm`.
- The point lies on a tangent join, so `sin θ`'s early form is the zero
  polynomial: a theorem by itself.
- The arm is `folded_lever_arm`, `min(min(arm₁, arm₂), extent)`, and a
  plane's curvature arm is `f64::MAX`. The read settles the inner `min`
  at its own node, behind every fold there, as its contract says.
- The product is then built by `combine`'s early zero arm, `0 · x = 0`.
  That arm ORs both factors' `gated` flags. The zero rests on `sin θ`
  alone, but the flag says it rests on the read.

With the read shut, the same product is `0 · min-atom`: an ungated zero
and a theorem at the early rung. So the read is ordered correctly at its
node. What overclaims is the gate's propagation: "sticky through every
combinator" carries a factor's gate through a product whose zero does
not depend on that factor's value.

The minimal row is `sym_root_rows::a_zero_factor_times_a_read_factor`:
`Z · min(x, 3)` with `Z = sqrt(x)² − x` (zero under rule A in the early
walk only) and `x ∈ [1, 2]`. It reads `sign_gated` shipped and
`theorem` with the read shut. The same row pins the class's other
instance in the same `combine` arm, `copysign(Z, min(x, 3) − 3/2)`,
which reads the same way. `copysign` carries `Z`'s magnitude, so its
zero does not depend on the sign argument either. Its contrast rows,
`(min(x, 3) − x) · y` and `min(x, 3) − x`, are zeros the read itself
reached. They read `sign_gated` shipped and refuse with the read shut.

### The answers, and what each costs

Pad leaf, release, `CAD_M10_10_TAKES=3`, best of 3, the `rules` and
`ON + the ladder` columns. Each candidate is a trial patch on the tree
named.

| candidate | pad receipt | bracket replay | pad leaf, rules / ladder, `8b640cded` | the same, `494d477ef` |
| --- | --- | --- | --- | --- |
| none (the base) | 893 / 34 / 148 / 1004 | 1105 / 21 / 146 / 781 | 26.613 / 35.633 s | 25.565 / 34.421 s |
| (a) a product with a zero factor carries that factor's gate alone | **925 / 2 / 148 / 1004** | **1121 / 5 / 146 / 781** | 27.178 / 35.970 s | 25.988 / 35.993 s |
| (b) an early gated zero is re-walked with the read shut, and is a theorem if that walk proves it | 925 / 2 / 148 / 1004 | (not run) | 26.718 / 35.471 s | 26.272 / 35.470 s |

- **(a) restores all 48.** It moves no `numeric`, `registered` or
  value: the form is the same zero polynomial and only its flag
  changes. Its cost is one boolean per zero product. The columns
  differ by at most 0.6 s on the merged tree, and by 1.6 s on the
  ladder column on the original base. That is noise between single
  best-of-3 sets, not the flag. Phase 2 re-measures before and after.
- **(b) restores the same 32 on the pad.** It costs a second early walk
  over every gated-zero decision's DAG, in a memo of its own, to
  recover a label the flag lost. The propagation that lost it stays in
  place for every other reader of `gated`: the door's `!d.gated` check,
  and atom keys, because `Form::digest` carries the flag.
- **(c) reading at the decision form rather than at interior nodes**
  restores nothing here. Each of the 48 roots is a `Mul`, which the read
  never answers. It would also withdraw the read from every interior
  `Select` and `min`/`max`: what DECIDE-3 ratified it to decide where no
  form settles. Not run.
- **(d) keeping the node symbolic until its parent has had its folds**
  reaches the 48 the way (b) does, by building the read-free form of
  the subtree above the read. Not run: (a) is its effect at the one
  fold that matters, without the second form.

**The choice is (a)** (Phase 2). The poison check (`tainted`)
guarantees only that no factor is poison: another factor may still have
a pole in the box. Soundness rests on two things:
- the argument the early zero arm already makes for `0/d`: a point
  where a factor has no value is one clause 1 has already refused;
- agreement with the read-shut label, where the other factor is an
  ungated atom and the same zero is a theorem.

So the product's claim rests on its zero factors, and an ungated one
makes the product a theorem. This holds in the early walk, where the
zero arm is on (`early_ab || trig_of_atan`). Elsewhere `Form::mul` ORs
the gates: the conservative direction, filed.
`copysign`'s zero fold gets the same rule. The stop rule does not
apply.

**The class sweep.** The pattern was every `gated` assignment in
`crates/geom-core/src/sym.rs` and `sym/*.rs`, each read for a result
that does not depend on an operand whose gate it carries.
- `combine`'s early zero arm, at `Mul`: the 48. Phase 2 fixes it.
- `combine`'s `copysign` zero fold, and rule F's two `copysign` arms
  over a zero magnitude: Phase 2 fixes them together, by asking the
  zero first.
- `min`/`max` both-zero, the `atan2` fold, the A0 `min`/`max` of
  constants, the A0 `Select`, every atom, the hull: each result depends
  on every operand whose gate it carries. Not this class.
- `Form::mul` (`sym/form.rs`): a product with a zero factor ORs the
  gates too. It is outside this unit's files. A trial counter in
  `Form::mul` and in `combine`'s `copysign` arm printed nothing on the
  bracket replay or the pad leaf. Filed as
  `work/decide/form-mul-carries-the-gate-of-a-factor-a-zero-annihilates`.

## What Phase 2 changed (DECIDE-9)

The tree is `8b640cded` plus this unit.

- **`geom_core::sym`'s `combine`, the early zero arm.** A product is
  zero whatever its other factor is worth, so it carries the AND of
  its zero factors' gates. One ungated zero factor makes the product a
  theorem. A sum still carries both gates.
- **`combine`'s `copysign`.** A zero first argument folds to zero,
  carrying that argument's gate alone. It is asked before rule F's two
  arms, which reach the same zero carrying the sign argument's gate
  too.
- **The docs.** The module header's "The decision read",
  `SymRules::decision_read` and its rules-table row now state the
  ordering the code keeps: behind every value-free fold AT ITS NODE.
  Above the node, a value built from the arm carries the arm's gate,
  and a zero that does not depend on the arm does not. The table row
  no longer says `sign_gated` "where it fires": the read fires inside
  each of the 48, and they are theorems.

**The invariant, shown.** All on the merged tree. Shipped first, read
shut second, as theorem / gated / registered / numeric:

| run | before | after | read shut |
| --- | --- | --- | --- |
| pad, `replay_counts` at `2.083e3·ε` (release, `m10_9_the_pad_at_both_rule_f_dials`; the gating row reads the same `symbolic_zero`) | 893 / 34 / 148 / 1004, frozen 2510 | **925 / 2 / 148 / 1004**, frozen 2510 | — |
| pad, release leaf instrument at `1e2·ε` | 893 / 34 / 148 / 1004 | 925 / 2 / 148 / 1004 | 925 / 0 / 148 / 1006 |
| bracket, replay at `3.870e2·ε` (dev) | 1105 / 21 / 146 / 781 | **1121 / 5 / 146 / 781** | 1121 / 0 / 146 / 786 |

- On the pad and the bracket, every decision the read-free walk proves
  is a theorem with the read on: 925 and 1121. That is not the tier's
  invariant. A read at a `min`/`max` node can still answer ahead of a
  cancellation its parent would make (`max(x + Z, 3) − max(x, 3)` is
  `sign_gated`, a theorem with the read shut). No measured document
  reaches that shape. It is filed as
  `work/decide/the-read-at-its-node-relabels-a-cancellation-above-it`,
  pinned at today's behaviour, and not fixed here.
- `sign_gated` keeps 2 on the pad and 5 on the bracket. Those are the
  `line_span` reads, which the hook found non-zero read-free and which
  the top rung did not settle. With the read shut they are `numeric`
  (pad 1004 + 2 = 1006, bracket 781 + 5 = 786).
- `registered`, `numeric` and `frozen` do not move on either document.
- The plate, the annulus and the link keep their `measured_studies`
  pins (`m10_9_no_registrant_lies_on_any_measured_document` passes
  unchanged for them). The segment boss and the link keep
  `sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`'s rows.
  `m10_10_pins_interval` passes too, but its rows are the plate's alone
  and carry no `dihedral_wedge`, so it is no evidence here.

**Pins re-baselined.**
- `m10_9_pins_interval`: the pad's `symbolic_zero` 893 → 925 and the
  bracket's 1105 → 1121, with their notes.
- `m10_9_the_pad_at_both_rule_f_dials` (ignored):
  `(893, 34, 150, 1002, 2577)` → `(925, 2, 148, 1004, 2510)`. Of that
  move, `registered`, `numeric` and `frozen` are `main`'s: this row on
  the merged tree before this unit reads `(893, 34, 148, 1004, 2510)`.
- `sym_9_retry_interval`: the bracket `[1105, 21, 146, 781]` →
  `[1121, 5, 146, 781]` without the ladder, and `[1105, 21, 152, 775]`
  → `[1121, 5, 152, 775]` with it. The drive's receipt
  `[1121, 5, 152, 775, 6]` and `[1121, 5, 146, 781, 0]`. `retried` 6
  either way.
- `sym_root_rows::a_zero_factor_times_a_read_factor`: the two shapes,
  `sign_gated` → `theorem`.

**Leaf cost.** Pad, release, best of 3, merged tree, two interleaved
sets:

| column | before, set 1 / set 2 | after, set 1 / set 2 |
| --- | --- | --- |
| `algebra ON (rules)` | 26.613 / 26.141 s | 27.362 / 26.846 s |
| `ON + the ladder` | 35.633 / 35.765 s | 36.005 / 36.022 s |

The rules column reads +0.70 to +0.75 s after (2.7 %) in both sets. The
same column differs by 0.47 s between the two "before" sets. The change
adds one boolean per zero product and one zero test per `copysign`, and
nothing here attributes the 0.7 s to it.

**Not this unit's.** `sym11_the_exact_channel_never_contradicts_past_the_ceiling`
(ignored, re-taken by hand at each SYM unit's close) pins the pad's
`symbolic_zero` past the ceiling at 885 and `registered` at 128. That
was stale before this unit (`registered` has read 148 at the pin's own
scale since `main`'s copied arc carriers), and it was not re-taken here.

## The review's fixes (DECIDE-9)

FULL review: APPROVE-WITH-FIXES, no MAJOR.
- **One home for the rule.** `zero_factors_gate` in `sym.rs` is gated
  exactly when every zero factor is gated. The early zero arm's `Mul`
  and `copysign`'s zero fold both call it.
- **A gated zero under `copysign` stays gated.** The row
  `copysign(min(x, 3) − x, y)` reads `sign_gated` with the read on and
  refused with it shut. Planting `z.gated = false` in the fold reds it.
- **The read ahead of a cancellation above its node is real in the
  tier.** It is filed, P2, as
  `work/decide/the-read-at-its-node-relabels-a-cancellation-above-it`,
  and pinned at today's behaviour by
  `sym_root_rows::the_read_relabels_a_cancellation_above_its_node_filed_defect`.
  Shutting the read at `min`/`max` reds that row. The docs now say the
  read is behind every value-free fold AT ITS NODE, and that above its
  node it can re-label a cancellation: the header, `SymRules::decision_read`
  and its rules-table row.
- **The door's answer moved.** `(x·x − x)·min(x, 3)` over `[0.9, 1.1]`,
  with `x·x` registered equal to `x`, reads `registered` with the read on
  and with it shut. Before, the read-on door form was gated and did not
  discharge, so it fell to the numeric channel. This is sound, and no
  measured document moves. It is pinned by
  `sym_root_rows::a_registered_zero_times_a_read_factor_is_registered`.
  Restoring the OR at the `Mul` arm reds it.
- **`Form::gated`'s doc** (`sym/form.rs`) no longer says "sticky through
  every combinator". It says where the walk drops the flag.
- **The `Form::mul` item** now names the whole class: `Form::mul`,
  `powi_form` and `algebra::apply`.

## Closed (DECIDE-9, PR #3807, 2026-10-02)

Answered on the measured documents: the pad and the bracket read their
read-shut theorem counts with the read on. The tier-wide form of the
contract is not restored: `the-read-at-its-node-relabels-a-cancellation-above-it`
carries it.

## 2026-10-02 — three more instances, from PATHS 5b (#3774)

With the constructions storing the carriers they build, the decision read
answers decisions that are theorems with G and the read shut
(`decide_3_split_rows_interval`, at the nominal, every ε):
- `r2_link` `dihedral_wedge`: `[32, 0, 0, 96] -> [0, 32, 0, 96]`;
- `r2_link` `path_seam_arrival_turn`: `[1, 0, 0, 0] -> [0, 1, 0, 0]`;
- `r2_filleted_bracket` `dihedral_wedge`: `[8, 0, 8, 216] -> [0, 8, 0, 224]`.
  Here the 8 decisions the fillet's incoming-tangency registration
  discharges with the read shut stay numeric with it on. Main's shipped
  row was `[0, 16, 0, 216]`, so the bracket's 8 are a loss against main.

`decide_3` re-baselines all three, citing this item.
