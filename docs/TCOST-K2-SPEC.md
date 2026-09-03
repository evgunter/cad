# TCOST-K2 — `offset_fit::fit_offset`'s per-station seconds: the Bernstein product weight, hoisted (spec)

**Program:** S-TCOST (`work/tcost/plan.md`, kernel-logic track). **Track:** kernel
change — standard v6 unit (binding spec, drawn implementer arm, cross-model dual
review, union fix pass, record-at-merge). **Pre-draw fields, logged before the block
draw:** difficulty **M**, task-class **structural** (§7).

## 1. The finding, and the measured picture

**The claim** (TCOST-3, PR 1614, "For the A/B track"):
`geom_brep::offset_fit::fit_offset` is 99.9 % of
`offset_fit::a_patch_far_from_the_origin_certifies_as_well_as_one_at_it` at 3.5–3.7 s
per station against 0.004 s for its 437-point dense oracle, and is all 13.8 s of
`an_unreachable_tolerance_refuses_typed_at_the_budget`.

**Call chain** (under `crates/`): `geom-brep/src/offset_fit.rs:479` `fit_offset` →
round loop `:501` → `:502` `interpolate_offset_grid` (the A9.4 refit) and `:503`
`measure` (`:945`) → `:951` `Composite::build` (`:1352`), whose cost is the products
at `:1498` (`X = Ẽ·Ẽ − d²w̃²`), `:1499` (`Y = Ẽ × M̃`), `:1500` (`D = Ẽ·M̃`) and `M̃`
at `:1479`, each of which is `geom-core/src/spline/compose/patch.rs:302`
`PatchSpans::mul` → `:394` `mul_block` → `compose.rs:454` `bern_mul_row`; `measure`'s
per-cell loop (`:967`) reads `cell_bound` (`:1534`) and `on_locus_cell` (`:1171`). The
real program pays the same door: `topo/src/replace_face.rs:1195` →
`approx_offset_surface` (`:699`) → `fit_offset`, and tier 3 one `measure` per `Approx`
face via `topo/src/validate.rs:2706` → `certify_offset` (`:608`).

**Measured** (worktree at `origin/main` 0c9850e2c, `eprintln!` phase timers, reverted,
nothing committed; dev profile, `--test-threads=1`; a local iteration tool only —
`memories/perf-measurement-lane.md`). Recentring row, one station
(`quarter_cylinder(1,1)`, `d = 1e-6`, `tol = 1e-2`) — **4.95 s per station here**:

| round | `us×vs` | cells | `interpolate_offset_grid` | `measure` | `hull_sup` |
|---|---|---:|---:|---:|---|
| 0 | 4×4 | 1 | 0.0003 | 0.0134 | `inf` |
| 1 | 7×4 | 4 | 0.0004 | 0.0477 | `inf` |
| 2 | 7×7 | 16 | 0.0006 | 0.1876 | `inf` |
| 3 | 13×11 | 80 | 0.0017 | 0.9831 | `inf` |
| 4 | 25×17 | 308 | 0.0055 | 3.7228 | 3.2219e-4 (certifies) |

`measure` is **99.8 %** of the call; the fit itself (offset sampling plus the two
collocation solves) is 0.008 s, 0.17 %. Round 4's `measure` (3.72 s) is
`Composite::build` **3.68 s (98.9 %)** + the `cell_bound` loop 0.008 (0.2 %) + the
`on_locus_cell` loop 0.034 (0.9 %); inside the build, `dec` 0.039, `Ẽ` 0.162, `M̃`
0.511, `X` 0.839, `Y` 1.410, `D` 0.714 — **the Bernstein products are 99 % of the
build**. Cost is linear in cells at **11.7–13.2 ms/cell** and cells grow ~4× per
round, so the last round is 75 % of the call and the call is ≈1.33× its last round;
rounds 0–3 are 25 % and each strictly refines the schedule the answer is read at.
`--release` has the identical shape at **0.55 s per station**, 9.0× the dev profile:
the suite's seconds are the dev profile's, and the real program pays ~0.5 s per fitted
face. **Gate census** (which side condition sends a cell to `+∞`): rounds 0–3 are
`inf` because the sign witness `D` (`:1551`) is not sign-definite on EVERY cell (1/1,
4/4, 16/16, 80/80), never on `w`, `w̃` or the `‖E‖` floor — so on those 101 cell
measurements `X` and `Y` were built and never read.

Exhaustion row (`bumpy_patch`, `d = 0.05`, `tol = 1e-15`), 19.4 s: six rounds,
`measure` = 0.70 / 1.21 / 1.93 / 2.45 / 5.74 / 7.10 over 144 → 1444 cells; `hull_sup`
falls 4.04e-5 → 6.54e-7 **strictly every round**; the exit is `OFFSET_FIT_SAMPLE_CAP`
(41×40 at `:575`), not `OFFSET_FIT_BUDGET`. Every cell is sign-definite from round 0.

**The item's question, answered: K1's exhausted-budget shape is NOT at work here.** K1
could refuse before a schedule whose width it projects exactly; this loop has no such
projection (the fit is re-derived globally each round; its bound's decrease is
empirical, not a theorem) and no round of either row is spent after the answer is
knowable. The 3.5 s is the certified bound computed at the schedule the certificate is
issued on — necessary work. What is not necessary is a constant factor inside it,
which is this unit.

## 2. Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md`: measure-first is mandatory, and binds to the
picture above exactly as to a refusal's prose. Re-take on the merge base with a local
instrument (reverted; nothing committed prints): per round, the grid, cells,
`interpolate_offset_grid` and `measure` times, `measure` split into `Composite::build`
/ `cell_bound` / `on_locus_cell`, the build's per-product split, and the gate census —
on the recentring stations, the exhaustion row,
`cylinder_fit_matches_the_closed_form_both_signs`,
`refinement_follows_the_anisotropy_on_a_thin_patch` and the non-rational
`non_analytic_base_fits_and_the_bound_contains_the_sample`, dev AND `--release`. Also
measure **who else pays `bern_mul_row`**: it serves `compose.rs:527` `ch_mul`, hence
the implicit-composite/SSI certification path too — take the before/after over `-p
geom-brep` AND `-p geom-core`.

**Stop clause.** If the products are not ~99 % of `Composite::build`, the build not
~99 % of `measure`, the cost not linear in cells, or a round is found that neither
refines the schedule nor moves the bound — the lever rests on that picture, not on
this file: say so and stop at a report. **If L1 measures under 10 % on the crate suite
the unit is a report, not a change.**

## 3. The lever — the same values, computed with less machinery

The inner loop is `acc + a_i · b_j · w`, `w = point(C(a,i)·C(b,j)) / point(C(a+b,k))`
(`compose.rs:468`): a ring DIVISION plus two ring products per coefficient pair, where
`w` depends only on the two degrees — pure structure (D9) — and is recomputed for
every coefficient of every cell of every round. `binom_row` (`:414`) is already a memo
of that kind and states the argument this lever reuses: *"a memo, not a second
spelling: the rows are exactly the recurrence … bit for bit."*

- **L1 (primary): hoist the structural weight table out of the innermost product
  loop** — memoize the ring weight row per `(deg_a, deg_b)`, looked up once per
  `mul_block` (`patch.rs:394`), not once per coefficient. Same values, same
  association, same fold order ⇒ **bit-identical coefficients, hulls, brackets,
  certificates**. Measured in the worktree: the whole `offset_fit::` suite **89.2 s →
  65.9 s (−26 %)**, 15/15 green; the station 4.95 → 3.65 s. Subsumes the three `Vec`
  clones `binom_row` makes per row product (`:417`, ~2 % alone); pair it with reusing
  a scratch buffer for `bern_mul_row`'s output instead of allocating per `(i,k)` pair
  in `mul_block` (~470 allocations/cell, also bit-identical), sized in Phase 1 and
  dropped if it does not pay.
- **L3 (only if the gate census reproduces): form `X` and `Y` per cell, behind the
  sign witness.** `cell_bound` reads `w`, `w̃`, `D`, and only then `Ẽ`'s mignitudes,
  `X`, `Y`; a cell refusing at the sign gate never reads `X` or `Y` — measured as
  every cell of rounds 0–3, 61 % of their build, ~14 % of the call. Needs a per-cell
  product entry point on `PatchSpans` (one cell's coefficients, hulled by
  `cell_hull`'s own ascending fold) and `Composite` holding `Ẽ`/`M̃` rather than
  `X`/`Y`/`D`. Bit-identical where read; pays nothing where every cell is
  sign-definite.

**Rejected here, each its own unit if wanted:** the scaled-Bernstein product form
(≥2×, but it moves the ring rounding of every composite in the kernel, SSI's included
— blast radius the corpus, not this door); a cheaper enclosure of a product's hull
(loosens the certificate, barred by §4); reusing `M̃` across rounds by subdivision (14
%, not bit-identical); the degree-1 → degree-2 elevation at `:1364` (~30 % of the
product cost on ruled bases, but needs `derived_knots` to carry a degree-0
derivative).

## 4. Binding constraints

- **No fit result weakened.** Every certificate is issued with a bit-identical
  `hull_sup`, `on_locus_max`, `cells`, `rounds`; every refusal keeps its variant and
  payload (`BudgetExhausted`, `RefinementStalled`, `Limb`, the meter and patch-bound
  refusals); every oracle row still agrees (the closed-form and dense-containment
  rows); every pin holds inside its window. A pinned number that moves is not this
  lever and stops the unit; any re-baselined pin carries its reason IN the row
  (`docs/prompts/implementer-discipline.md` §3) — expected: none.
- **D9.** Schedule, seed rule, marking rule, stall guard, budgets and
  `OFFSET_CERT_SAMPLES` untouched; certification stays a function of (base, `d`,
  tolerance, band). No new data-dependent branch: L3 branches on the gate `cell_bound`
  already evaluates, in the order it evaluates it.
- **The digest instrument** (K1's, adapted): one roster line per certified
  `OffsetCertificate` (all six fields, keyed by call site) and per typed refusal
  (variant + payload) at every return of `fit_offset`, `certify_offset` and
  `recertify_approx`, over a full `--workspace` run at three ε rows on both lanes,
  from the merge base and from the head under the identical instrument.
  **md5-identical rosters, nothing surplus or missing** except what the unit's own
  rows add; anything else stops the unit.

## 5. Acceptance

- The Phase 1 tables (dev and release), the gate census and the `bern_mul_row` survey,
  in the PR body, from the merge base.
- **Hosted, both lanes** (`CI-Config: lane=default` on one head, `lane=interval` on
  another, at the ε the informative rows sit at), green; the digest identical across
  builds on every face and every payload.
- **Before/after in the hosted cost report:** the recentring row,
  `an_unreachable_tolerance_refuses_typed_at_the_budget`, the `offset_fit` file's
  total, and (the lever being in `geom-core`) both crates' shard totals.
- **The unit's own suite:** the memoized weight row equals the recurrence it replaces
  bitwise, over every degree pair the kernel can produce and past `BINOM_EXACT_MAX`
  where the row is all-poison; one product's per-cell coefficients are bitwise equal
  formed whole-patch or per cell (L3's precondition); with L3, a cell refusing at the
  sign witness still reports `+∞` and its round the same `hull_sup`. No timing is
  asserted (`plan.md` §Keep-outs).

## 6. Out of scope

The refinement schedule and its constants; the stall guard's predicate (its
reachability verdict at `offset_fit.rs:1032` is not reopened here); the composite's
mathematics — both limbs, the `τ²/‖E‖` term, the componentwise `‖E‖` lower bound the
docs name as the small-`|d|` limit, the meters; the scaled-Bernstein form and the
degree-1 elevation (§3); tier 3's per-face re-derivation
(`work/perf/tier3-approx-regrid-per-face-cost.md`); test content.

## 7. Pre-draw fields

- **Difficulty M** — one localized `geom-core` change plus, if the census holds, a
  mechanical per-cell restructure of a 160-line builder; no new theorem, no payload
  change, no soundness argument — the weight is in the evidence, not the code.
- **Task-class `structural`** — no predicate, bound or tolerance comparison is
  introduced or changed: every value is the same value, by the same operations in the
  same order, which is `docs/MODEL-AB-LOG.md`'s structural class; the numeric-shaped
  alternatives are out of scope (§3, §6) so it stays honest.
