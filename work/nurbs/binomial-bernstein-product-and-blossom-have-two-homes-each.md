---
id: binomial-bernstein-product-and-blossom-have-two-homes-each
kind: issue
title: Binomials, the Bernstein product and the span blossom each have two homes in geom_core::spline
status: closed
opened: 2026-10-13
closed: 2026-10-20
priority: P3
cost: M
refs: [4954]
---

## What

PR 4954 (`nurbs/quotient-bound`) gave the quotient bound its own
binomials, Bernstein product and blossom. Each sits beside an older
home in `crates/geom-core/src/spline`.

**Pair 1: binomials and the Bernstein product.**
- `pieces::binomial` is exact in integers to `n = 56`.
  `derivative`'s private `product` and `Grid::raised_to` form the
  product weights `C(m,i)·C(n,j)/C(m+n,k)` at any `Real`.
- `compose.rs` has `binom_row`/`binom_table`, capped at
  `BINOM_EXACT_MAX = 54` because its multiplicative recurrence rounds
  at 55. Its `BernWeights` Bernstein product runs at the certification
  scalar.
- A third copy, `binom` (multiplicative `f64`), sits in
  `crates/geom/tests/curves/piece_coverage_meter.rs`.

**Pair 2: the span blossom and restriction.**
- `pieces::blossom` / `span_row` / `restrict` are Real-generic de Boor
  steps with exact dyadic piece weights.
- `blossom.rs` has `CoeffWindow::blossom` and `SpanRow::restrict`.
  Every step goes through `Certification::between`, under the
  argument contract in that module's docs. (`ssi/enclose.rs`'s
  `bezier_on` cuts read it until `nurbs/cut-becomes-restriction`
  retired them.)

## Why it is not in PR 4954

Neither fold is mechanical. `compose`'s weights are pinned bit for bit
by the rehearsal's ratified bit-identity rows, and its docs reject ring
quotients for exactly that reason. `blossom.rs`'s steps are
certification arithmetic with an argument contract that the
Real-generic one does not carry.

## Shape

One home per primitive.

- **Binomials.** Integer-formed, which also lifts `compose`'s cap
  from 54 to 56.
- **Bernstein product.** Either one Real-generic product that
  `compose` instantiates at its certification scalar, or a stated
  reason the two must differ.
- **Blossom.** `CoeffWindow::blossom` as the Interval instance of
  `pieces::blossom`, if `between`'s clamp can ride as a scalar hook.

Each fold has to show its pinned bits unmoved, or re-baseline them
with a cause.

## Evidence: halving at ½ has a second home (2026-10-14)

PR 5081's review (style, Q1) found a second home for halving a
Bernstein row at ½, next to the pairs above:
- `derivative`'s `Grid::halved`: generic `T`, unscaled de Casteljau
  sums and one exact `2⁻ʳ` scale per output.
- `blossom.rs`'s `restrict_bernstein` / `restrict_segment`:
  certification scalar only, general `[a, b]`. `ssi/section.rs:337`
  and `props/quad.rs:4376` call them at `(0, ½)` / `(½, 1)`.

The step-3 cut (`nurbs/cut-becomes-restriction`) adds a third
restriction, `blossom.rs`'s `split_restrict`: two de Casteljau splits,
`O(p²)` against `restrict_segment`'s `O(p³)` pyramid. The fold should
take all three: one restriction door with a halving fast path.

## Progress

- **Binomials: folded** (PR 5414, with the cap's one home and the last
  `f64` folds in PR 5433). `pieces::binomial` is the one home, and
  `pieces::BINOMIAL_EXACT_MAX` the one cap.
- **Bernstein product: two homes, by precision** (PR 5439). Each
  association is the tighter one for its own callers.
  `compose::bern_mul_row` folds a memoized outward quotient weight per
  term; `derivative::product` keeps exact integer numerators and
  divides after each coefficient's sum, at any `Real`. Both enclose
  the exact rational product
  (`derivative::tests::both_bernstein_products_enclose_the_exact_rational_product`).
  The measurement (2026-10-20, main `b74e5b8c`) took three candidates:
  A, main; B, compose on exact numerators with one division per
  coefficient; C, `product` on per-term outward quotients.
  - **Decisions.** geom-brep and sweep slow sets (4682 rows) at the
    default ε and 1e-12: neither B nor C changes an outcome beyond
    pinned values. B moves the offset-fit weights-2 sup golden 166 ulp
    tighter and the `m5_pr7_ssi` arithmetic-floor pin (minimum margin
    8.97e-15 to 1.04e-14, six rounds to four, the typed `HullSupChart`
    refusal unchanged); C moves the offset-fit golden 20 ulp wider. The
    offset-fit scale invariance holds under both. The `limb_three`
    census on shelling, r1, `m5_pr7_ssi::` and `nurbs_nurbs_section::`
    (735 rung lines) is identical in rungs and one-arc outcomes under
    both; C differs only in margin bits (110 lines), B in one rung's
    box count (4945 to 5729, same outcome). Not seen: decisions outside
    those four rows and the slow sets' assertions.
  - **Widths**, the other association against each home's live one,
    over geom-core, geom, mesh, geom-brep and sweep:

    | other association at | coefficients | wider | > 4 ulp | > 2× and > 4 ulp | worst |
    |---|---|---|---|---|---|
    | compose `tensor.rs:383` (powers) | 3.39 M | 32.4% | 4.50% | 95 530 | exact points widened |
    | compose `tensor.rs:384` | 3.39 M | 30.4% | 2.61% | 35 309 | 6× |
    | compose `tensor.rs:386` | 6.45 M | 47.4% | 5.41% | 14 606 | exact points widened |
    | compose `tensor.rs:453` | 17.1 M | 36.6% | 2.73% | 376 | 6× |
    | compose `ch_mul` | 2.84 M | 30.2% | 2.62% | 203 | 4× |
    | compose `patch::mul_block` | 165 M | 27.1% | 4.86% | 31 570 | 4× |
    | `product` in `area_raised` | 53.4 M | 20.4% | 4.96% | 92 996 | 1470× |
    | `product` in `doubled_cross` | 0.92 M | 59% | 25% | 10 298 | 208× |
    | `product` in `cell_pieces` | 12.6 M | 63.1% | 10.1% | 554 | 10.5× |

    B is narrower than A on 7–40% of compose's coefficients, and its
    widest excess is about 1e-15 to 1e-11 of the coefficient's
    magnitude. The ruling (2026-10-20): no fold direction may widen a
    real caller past about 2×, so both homes stay.
- **Restriction: three homes, one door by width, and one fold** (#5461).
  Measured (2026-10-20) at every real call during the geom-brep
  slow set and sweep's shelling, r1 and shell rows, each other home on
  the same inputs against the pyramid (`restrict_segment`):

  | caller | other home | coefficients | wider | > 4 ulp | worst |
  |---|---|---|---|---|---|
  | `ssi/section.rs:246` general cuts | `split_restrict` | 2.59 M | 18.2% | 15.2% | 98× |
  | `ssi/section.rs:246` exact-half cuts | `Grid::halved`'s arithmetic | 7.8 k | 1.8% | 0.3% | 2× |
  | `ssi/section.rs:261` | `split_restrict` | 1.1 k | 78.8% | 53.0% | 14× |
  | `ssi/section.rs:337/338` halves | split, halving | 1.2 M | 0.01% | 0 | 1.6× (≤ 3 ulp) |
  | `props/quad.rs` halves | split, halving | 48 | 0 | 0 | bit-equal |
  | `derivative::cut_pieces` | `split_restrict` | 204 M | 42.6% | 32.6% | 417× |

  The pyramid is never meaningfully wider on any caller. So it is the
  door for a row read as it comes; `split_restrict` (`O(p²)` against
  `O(p³)`) is the cut lane's form, whose reader meets every cut box with
  the whole cell's; and `Grid::halved` stays `derivative`'s generic
  halving. Each home's docs state this. The one fold: the trim
  bisection's halves move to `restrict_segment` on `[0, 1]`, bit for bit
  (`props::quad::tests::the_bisection_halves_are_the_retired_doors_to_the_bit`),
  and `restrict_bernstein` retires. PR 5424's two `split_restrict`
  carry-overs ride the same PR.

## Closed

Each primitive is resolved: the binomials folded to `pieces::binomial`
(PRs 5414, 5433); the Bernstein product keeps two homes by precision
(PR 5439); restriction keeps three by width, with
`restrict_bernstein` folded onto `restrict_segment` (#5461). The
measurements above are the evidence.
