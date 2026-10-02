---
id: the-decision-read-answers-theorems-the-must-carry-stations-would-prove
kind: issue
title: The decision read answers 32 of the pad's and 16 of the bracket's decisions that are theorems with it shut; 28 of the pad's are the must-carry stations
status: dispatched
opened: 2026-10-01
priority: P2
cost: M
refs: [rule-g-trades-sixteen-of-the-links-carrier-on-surface-2, 2468]
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

With the read shut, every theorem is still a theorem: the merged pad's
925 is the pre-merge 894 plus `main`'s 28 must-carry station theorems
(`geom_brep::must_carry_over_edge`'s `classify_dihedral` gate, seven
stations on each of the pad's sixteen rule-reached edges) and its three
fillet run-out theorems (`path_run_out_carrier`). With the read on, 32
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
theorem the read costs outright. The runs:
- **the bracket**: `m10_9_pins_interval`'s replay (`replay_counts` at
  `certifies_at = 3.870e2·ε`, ε = 1e-9, dev), shipped
  `1105 / 21 / 144 / 783`, read shut `1121 / 0 / 144 / 788`. Its
  nominal split, `m10_10_splits_at_the_nominal_under_a_rule_set` with
  `CAD_M10_10_DOCS=r2_filleted_bracket` and `CAD_M10_10_RULES` set to
  `shipped` and then `no_reads`.
- **the pad**: the release leaf instrument,
  `m10_10_leaf_cost_with_and_without_the_algebra` with
  `CAD_M10_10_DOCS=r2_rounded_pad`, `CAD_M10_10_COLUMNS=rules`. That is
  one whole-box leaf at `1e2·ε`. It reads `893 / 34 / 150 / 1002`, the
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
`ON + the ladder` columns. Each candidate is a trial patch on this base.

| candidate | pad receipt | bracket replay | pad leaf, rules / ladder |
| --- | --- | --- | --- |
| none (the base) | 893 / 34 / 150 / 1002 | 1105 / 21 / 144 / 783 | 25.565 / 34.421 s |
| (a) a product with a zero factor carries that factor's gate alone | **925 / 2 / 150 / 1002** | **1121 / 5 / 144 / 783** | 25.988 / 35.993 s |
| (b) an early gated zero is re-walked with the read shut, and is a theorem if that walk proves it | 925 / 2 / 150 / 1002 | (not run) | 26.272 / 35.470 s |

- **(a) restores all 48.** It moves no `numeric`, `registered` or
  value: the form is the same zero polynomial and only its flag
  changes. Its cost is one boolean per zero product. The three columns
  differ by about 1.5 s on the ladder column, which is noise between
  single best-of-3 sets, not the flag. Phase 2 re-measures before and
  after back to back.
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

**The choice is (a)** (Phase 2). A zero factor annihilates the product
wherever the other factor has a value, and the poison check
(`tainted`) already guarantees it has one. So the product's claim rests
on its zero factors, and an ungated one makes the product a theorem.
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
