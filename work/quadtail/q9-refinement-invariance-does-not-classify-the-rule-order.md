---
id: q9-refinement-invariance-does-not-classify-the-rule-order
kind: issue
title: "Q9's fixture does not classify the OUTER rule's order at all: the outer-two-short mutant is exact on it, and refinement invariance separates neither"
status: open
opened: 2026-09-21
priority: P3
cost: D
---


## What

`props::quad::tests::q9_the_inner_rules_order_is_pinned_at_two_short_and_the_outer_at_four`
(`crates/geom-brep/src/props/quad.rs`) is the trimmed lane's only
exactness row for its two closed Newton–Cotes rules
(`trimmed_patch_face_rounds`, outer `3p_u + 3p_v − 1` in the chord,
inner `3p_v − 1` across the patch). Two things about it are weaker
than its name used to say, and one of them is a misattribution.

**1. The fixture does not see the OUTER order at two counts short.**
Measured at the rule site on `curved_chart(2, 2)` with the row's own
three-chord loop, round 0's midpoint against the `1e-12` window:

| rule cut | round-0 midpoint | from the shipped answer | row |
| --- | --- | --- | --- |
| none (shipped) | `3.2542626001979752e-1` | `2.3e-17` | passes |
| OUTER, 2 short | `3.2542626001979757e-1` | `0`; round 2 bit-identical | **passes** |
| INNER, 2 short | `3.2542430502718228e-1` | `1.95e-6` | reds |
| OUTER, 4 short | `3.2542626002142860e-1` | `1.63e-12` | reds |
| OUTER, 6 short | `3.2542623695793438e-1` | `2.31e-8` | reds |

The `1.95e-6` error that this row, its doc and the RING-2 PR body all
attributed to "the outer rule two counts short" is the **inner**
rule's. The outer rule is caught only from four counts short, and
there by `1.63e-12` against a `1e-12` window — a hair rather than a
classification. The cause is the fixture: the chord integrand's
`u`-degree is what the outer order has to cover, and this chart's
image is degree 1 (chosen so no lune pad rides along to blur the
comparison), so the degree-11 case the row's prose describes is never
reached. This was as true at the merge base as it is now; RING-2
re-derived the gate and found it.

**2. Refinement invariance classifies neither mutant on its own.** An
exact rule integrates each sub-chord exactly, so cutting the chord
into four times as many pieces cannot move the answer; a rule short of
the degree was supposed to converge instead. The inner-two-short
mutant is refinement-invariant too — wrong by `1.95e-6` at BOTH chord
counts — so refining does not move it. What the row's old quarter-ulp
gate on `|m₀ − m₂|` was measuring was the arithmetic's rounding: under
the retired C9 ring every operation padded one representable step
outward, symmetrically, so an enclosure's midpoint was exactly the
round-to-nearest value and the shipped order answered the two rounds
bit for bit (`0`) where the mutant answered them one ulp apart
(`5.55e-17`). A quarter-ulp gate separated `0` from one ulp of luck.
RING-2 made the ring a newtype over `interval-transcendentals`'
`DInterval`, which pads only where an operation is inexact, so a
bracket is no longer symmetric about its round-to-nearest value: the
shipped order now reads `1.11e-16` and the mutant `5.55e-17`, which
**ranks them backwards**. (The half is not powerless: the
outer-four-short mutant moves `1.63e-12` between the rounds against a
`9.96e-13` width, so it reds on the refinement claim as well as on the
answer.)

**3. The oracle is a golden.** `EXACT` is the shipped rule's own
output, pinned at a `1e-12` window. What keeps it from being purely
circular is that a different quadrature reproduces it — the
outer-two-short rule to `5.5e-17`, the outer-four-short rule exactly
at round 2 — but nothing in the row derives the integral
independently, and the row's doc says so.

## What RING-2 did, and what is left

RING-2 moved the classification onto the quantity that carries the
signal (the answer, at a `1e-12` window, four million times inside the
inner mutant's error and seven orders outside the arithmetic's own
width), kept the refinement claim as the consistency claim it can
support, and — in the fix pass, after both reviewers measured the
mutants independently — corrected the attribution and renamed the row
to say which order it pins at which cut.
`memories/output-stability-as-justification.md` is what required the
re-derivation rather than a widening.

What is left is the structural instrument, and it is QUAD's:

1. **A fixture whose chord integrand actually reaches `h`-degree 11**,
   so the outer order is exercised at all. It needs a chart whose
   image is not degree 1, which brings a lune pad into the comparison
   — a different fixture, not a wider one, which is why RING-2 did not
   take it.
2. **A rule-order knob**, so the row can assert order invariance
   directly: the shipped order and a HIGHER one answer the same
   integral, which is what exactness means. Needs a test-visible way
   to vary `(mu, mv)` at `trimmed_patch_face_rounds`, which is a
   production signature question.
3. **An independent value for the integral**, so the oracle stops
   being the shipped rule's own output.

## Disposition

QUAD's: `quad.rs` is this program's file. Filed by RING-2 (SCALAR),
which found it re-deriving the gate its own change moved, and which
did not widen the gate to pass.
