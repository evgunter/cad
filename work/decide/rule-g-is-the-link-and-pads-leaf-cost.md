---
id: rule-g-is-the-link-and-pads-leaf-cost
kind: issue
title: rule G is the link's and the pad's leaf cost: shutting it takes the pad's release leaf 73.8s to 17.2s and the link's 8.5s to 3.8s, where shutting the decision read moves neither
status: dispatched
priority: P2
cost: D
opened: 2026-09-25
refs: [decision-read-triples-the-plate-pin-suites-wall-time, DECIDE-6]
---


## What was measured (DECIDE-6, 2026-09-25)

DECIDE-3 added rule G and the decision read in one unit, and
`decision-read-triples-the-plate-pin-suites-wall-time` put the plate
pin suite's 143 s → 535 s (dev) on the read. DECIDE-6 measured the
read and found it under half a percent of every replay (that row's
"Where the read's cost is (DECIDE-6)"). The same replays with rule G
shut (`SymRules::without_canonical_root`) are where the time moves.

On the release leaf instrument
(`m10_10_evidence_interval::m10_10_leaf_cost_with_and_without_the_algebra`,
one attempt per rung, `CAD_M10_10_COLUMNS=rules`):

| document | shipped (fastest of 3) | read shut (fastest of 3) | rule G shut (1 take) |
| --- | --- | --- | --- |
| R2 rounded pad (`1e2·ε`) | 73.83 s `[890, 6, 150, 907]` | 74.53 s `[894, 0, 150, 909]` | 17.15 s `[850, 6, 128, 969]` |
| R2 link (`1e1·ε`) | 8.52 s `[545, 0, 108, 449]` | 8.36 s `[545, 0, 108, 449]` | 3.83 s `[505, 0, 110, 487]` |
| R2 filleted bracket (`1e1·ε`) | 1.98 s `[1104, 7, 144, 766]` | 2.01 s `[1106, 0, 144, 771]` | not taken |

In dev (`decide_6_read_cost_interval::decide_6_where_the_reads_cost_is`,
ε = 1e-9, one take each; the pad's row is the DECIDE-6 review's probe,
`decide_6_review_probe_interval` on `decide/6-review` @ `094ba2251`):

| leaf | shipped | read shut | rule G shut | both shut |
| --- | --- | --- | --- | --- |
| pad at `2.4990e3·ε` | 280.6 s | not taken | 122.4 s | not taken |
| link at `4.930e2·ε` | 34.72 s | 35.01 s | 19.06 s | 18.76 s |
| bracket at `3.870e2·ε` | 12.34 s | 12.59 s | 12.93 s | 12.90 s |
| plate at 0.2630 | 2.34 s | 2.30 s | 2.53 s | 2.58 s |
| annulus at 0.8416 | 2.45 s | 2.63 s | 2.48 s | 2.37 s |

**The separation from DECIDE-3's other additions**, measured by the
same review in dev:
- rule G's exact quotient shut (`SymRules::without_root_quotient`):
  pad 281.8 s, link 35.2 s — flat against shipped, so the quotient is
  not the cost;
- A0's `min`/`max` folds shut (an uncommitted patch): flat, with
  every receipt identical — not the cost either;
- rule G shut: the pad 280.6 → 122.4 s and the link 34.72 → 19.06 s
  above.

`m10_10_pins_interval` itself is 337.83 s in dev at DECIDE-6's head
(`--test-threads 2`). Its ceiling row replays the link and the pad at
both ends of their brackets (two each), and its bound-at-ceiling-plus-δ
row the link once more. The pad's two leaves are about 80 % of the
suite. **An ESTIMATE, not a run**: with rule G shut the suite would be
about 155 s, the review's figure from the per-leaf differentials above;
no suite was run with rule G shut, and its pins would not hold there.

## What is not known

Where inside rule G the time goes: the mint site's quotient split and
side condition, the magnitude door, or the larger forms the canonical
atoms leave downstream (the exact quotient is ruled out above). The
profile (`geom_core::sym::profile`) already clocks each walk; a
per-rule clock at rule G's mint site is the next measurement.

## Home

`crates/geom-core/src/sym/root.rs` (rule G's mint site),
`crates/geom-core/src/sym.rs`'s `combine`.

## Where rule G's time goes (DECIDE-7)

**The instrument.** `geom_core::sym::profile` under
`sym-profile-testing`, beside `ReadProfile`:
- `RootProfile`: `root::canonical`'s calls by branch, the parts inside
  it (`div_exact`, the content split, `poly_sqrt`, the side condition,
  the magnitude door), the `abs` node's door, and `own`, the outermost
  entries into rule G summed once.
- `NodeCost` per `(walk, node id)` with `SymProfile::node_delta`, the
  join of two profiles on the same DAG nodes.
- The per-node reduction (`algebra::reduce_steps` at `EARLY_STEPS`) by
  outcome, by the atom kinds the form carried to an even power, by a
  refusal's cause and site, and by whether its input form had already
  been reduced this session (`reduce_by`, `reduce_refusals`,
  `reduce_repeats`).

The dev row is `decide_7_rule_g_cost_interval::decide_7_where_rule_gs_time_goes`
(ε = 1e-9, no ladder, one take each, `6d990999b`). The release row is
`m10_10_leaf_cost_with_and_without_the_algebra` with
`CAD_M10_10_PROFILE`, `CAD_M10_10_COLUMNS=rules`, and
`CAD_M10_10_RULES` unset or `no_g` (`6d990999b`). The profiled walls
carry the instrument's clocks.

| | pad, release (`1e2·ε`) | pad, dev (`2.4990e3·ε`) | link, dev (`4.930e2·ε`) |
| --- | --- | --- | --- |
| wall, shipped / rule G shut (profiled) | 75.65 / 18.47 s | 301.10 / 136.72 s | 39.48 / 22.71 s |
| difference | 57.18 s | 164.38 s | 16.78 s |
| rule G's own time (`own`) | 0.20 s | 2.14 s | 0.46 s |
| of it: `abs` node door / `canonical` | 0.13 / 0.07 s | 1.48 / 0.66 s | 0.001 / 0.46 s |
| the per-node reduction, shipped / shut | 66.92 / 10.37 s | 224.58 / 65.48 s | 24.91 / 10.72 s |
| of it: inputs already reduced this session | 45.25 / 6.01 s | 152.90 / 40.09 s | 13.49 / 5.90 s |
| the walks' other work (the difference less the two above) | 0.43 s | 3.15 s | 2.13 s |

Receipts under the profile: pad `[890, 6, 150, 907]` shipped and
`[850, 6, 128, 969]` shut, link `[545, 0, 108, 449]` and
`[505, 0, 110, 487]`, the unprofiled ones.

**Inside the mint site** (pad, dev; the release row has the same
counts):
- `canonical`: 832 calls. Declined for no sign of `D`: 235 (0.40 s).
  `den = 1`: 403 (0.02 s). Exact quotient: 106 (0.07 s). Split with `D`
  manifestly `≥ 0`: 87 (0.16 s). Poisoned: 1.
- Parts: `div_exact` 831 calls, 106 exact, 0.12 s; the content split
  659, 0.11 s; `poly_sqrt` 659 searches, 204 roots, 0.04 s; the side
  condition 322, 87 proved, 0.02 s; the magnitude door 204, 24 folded
  to `R`, 0.003 s.
- 522 of the 832 calls repeat an argument this session already asked
  (0.43 s).
- The answers average 2.07 terms against the atom's one; 87 carry a
  non-constant denominator.

On the link the split with `D ≥ 0` is 83 calls in 0.23 s, the no-sign
declines 48 in 0.10 s, and the content split 0.17 s of the 0.46 s.

**Downstream.** The same DAG nodes, joined across the two replays
(`node_delta`):
- Plain: identical, as it must be (rule G is early-walk only).
- Early (pad, dev): forms average 48.1 terms against 39.9, and take
  139.7 s against 67.5 s.
- Door (pad, dev): 52.6 against 42.7 terms, 145.9 s against 55.9 s.
- Link, early: 20.8 against 20.6 terms, 27.7 s against 16.8 s. The
  link's forms barely grow, but they carry more even-power atoms.

The time is in the per-node reduction (rules A/B and rule G's companion
`|X|² = X²`):
- Pad, dev: forms carrying `sqrt`, `abs` and `sin` to an even power
  REFUSED 918 times in 133.18 s shipped, against 325 in 20.47 s shut.
  Every refusal in both replays is the ring or a product refusing a
  step (`apply` answered `None`), and the walk then keeps the unreduced
  form. The refusals' sites, shipped:
  - the quotient's product on the term pre-bound: 276 in 97.15 s;
  - a term times its factor on the coefficient bound: 677 in 25.65 s;
  - the running sum on the coefficient bound: 270 in 19.32 s.
- Link, dev: the time is in reductions that succeed. Forms with all
  three kinds reduced 46 times in 12.05 s, against 102 in 2.40 s shut.

**Attribution** (pad, release, 57.18 s):
- the mint site's own time is 0.20 s (0.35 %);
- the per-node reduction is +56.55 s, of which +39.24 s re-reduces an
  input form the same session already reduced;
- the walks' other work is the remaining 0.43 s.

Dev, pad: 2.14 s own, +159.09 s reduction (+112.81 s of it repeats),
3.15 s other. Dev, link: 0.46 s own, +14.19 s reduction (+7.59 s
repeats), 2.13 s other. Within the mint site the largest branch is the
no-sign decline on the pad (0.40 s dev) and the split on the link
(0.23 s dev).

**Rule G's conjunct dials, unprofiled, dev**
(`decide_7_where_rule_gs_time_goes`, `983a9a3b7`):

| leaf | shipped | rule G shut | `abs_square` shut | `root_magnitude` shut | `root_quotient` shut |
| --- | --- | --- | --- | --- | --- |
| link | 36.10 s `[545, 0, 108, 449]` | 19.07 s `[505, 0, 110, 487]` | 14.72 s `[445, 0, 112, 545]` | 39.21 s `[545, 0, 108, 449]` | 35.32 s `[545, 0, 108, 449]` |
| pad | 280.6 s (DECIDE-6) | 122.4 s (DECIDE-6) | 161.76 s `[773, 5, 142, 1033]` | 449.48 s `[890, 6, 150, 907]` | 281.8 s (DECIDE-6's review) |

Shutting the companion `|X|² = X²` alone recovers the link's time and
most of the pad's, and moves both receipts. Shutting the magnitude
door keeps the pad's receipt and costs 169 s more than DECIDE-6's
shipped figure. The quotient is
flat.

**The candidate answers, with what each would save** (release pad
first, then dev pad and dev link):
- **Memoise `canonical` by argument**: at most the repeats' time, 0.04
  / 0.43 / 0.33 s.
- **A cheaper decline before `poly_sqrt`**: the whole search is 0.008 /
  0.04 / 0.03 s.
- **A cheaper side condition**: the whole condition is 0.004 / 0.02 /
  0.12 s.
- **The `abs` node door**: 0.13 / 1.48 / 0.001 s whole.
- **A different spelling of the same atoms**: none of rule G's dials
  is one. The companion's is cheaper and moves receipts; the magnitude
  door's keeps them and is slower. None was built beyond the dials.
- **Memoise the per-node reduction on its input form, per session**:
  at most the repeats' time, 45.25 / 152.90 / 13.49 s shipped. The
  reduction is a function of the form, the rules, the ring bound and
  the session's fixed budget and step cap (and of the atoms its form
  names, which never change once minted), so a memo keyed on those returns the form the reduction would have built,
  and no form, digest or decision moves.
- **Refuse the quotient's product before the numerator is built**: the
  term pre-bound it refuses on is known from the denominator side
  before the substituted numerator exists. At most 30.82 / 97.15 s
  (pad) is spent reaching that refusal, repeats included; it lives in
  `algebra.rs`, outside this unit's files.

The mint-site answers the spec names are each under 1 % of the
difference. The time is in the per-node reduction of the forms rule G
leaves, and two thirds of the pad's release difference is that
reduction asked again of a form it already answered. Phase 2 takes
the reduction memo.
