---
id: q9-refinement-invariance-does-not-classify-the-rule-order
kind: issue
title: Q9's refinement-invariance oracle does not classify the outer rule's order: the two-counts-short mutant is refinement-invariant too
status: open
opened: 2026-09-21
priority: P3
cost: D
---


## What

`props::quad::tests::q9_the_outer_rules_order_is_pinned_by_refinement`
(`crates/geom-brep/src/props/quad.rs`) claims to pin the trimmed
lane's outer Newton–Cotes order
(`trimmed_patch_face_rounds`, `(3p_u + 3p_v − 1, 3p_v − 1)`) by
**refinement invariance**: an exact rule integrates each sub-chord
exactly, so cutting the chord into four times as many pieces cannot
move the sum, while a rule short of the degree converges instead and
its round-0 and round-2 answers "differ by orders more than their own
widths".

**The mutant the row names is refinement-invariant too**, so the
oracle does not separate them. Measured on this fixture
(`curved_chart(2, 2)`, the three-chord loop the row builds), with the
rule order cut by two at the site above:

| rule | round 0 | round 2 | `|m₀ − m₂|` |
| --- | --- | --- | --- |
| shipped `3p_u + 3p_v − 1` | 3.254262600197975e-1 | 3.2542626001979763e-1 | 1.11e-16 |
| two counts short | 3.254243050271822e-1 | 3.2542430502718217e-1 | 5.55e-17 |

The mutant's answer is wrong by `1.96e-6` — a relative `6e-6`, seven
orders above the enclosure's own `2.5e-13` width — and it is wrong by
the same amount at BOTH chord counts. Refining does not move it.

**What the row's quarter-ulp gate was actually measuring** was the
arithmetic's rounding: under the retired C9 ring every operation was
padded one representable step outward, symmetrically, so an
enclosure's midpoint was exactly the round-to-nearest value and the
shipped order answered the two rounds bit for bit (`0`) where the
mutant answered them one ulp apart (`5.55e-17`). A quarter-ulp gate
separated `0` from one ulp of luck. RING-2 made the ring a newtype
over `interval-transcendentals`' `DInterval`, which pads only where an
operation is inexact, so a bracket is no longer symmetric about its
round-to-nearest value: the shipped order now reads `1.11e-16` and the
mutant `5.55e-17`, which **ranks them backwards**.

## What RING-2 did, and what is left

RING-2 re-derived the gate onto the quantity that does carry the
signal — the answer itself, pinned at a `1e-12` window, four million
times inside the mutant's error and seven orders outside the
arithmetic's width — and kept the refinement claim as the consistency
claim it can support (the two midpoints inside the wider enclosure's
own width, plus the existing overlap and the existing "the width is
the nodes' and weights' rounding" assertions). That keeps the
classification correct, which is what
`memories/output-stability-as-justification.md` requires of a
structural exactness a gate rests on.

What is left is the structural instrument the row's NAME claims. Two
candidates, neither RING-2's to take:

1. **A rule-order knob**, so the row can assert order invariance
   directly — the shipped order and a HIGHER one answer the same
   integral, which is what exactness means and which the mutant fails
   by `1.96e-6`. Needs a test-visible way to vary `(mu, mv)` at
   `trimmed_patch_face_rounds`, which is a production signature
   question.
2. **A fixture whose mutant genuinely converges**, so refinement
   invariance regains its power. Why the present one does not — the
   mutant is biased rather than converging — is not explained by
   anything in the file and is the first thing to find out.

## Disposition

QUAD's: `quad.rs` is this program's file. Filed by RING-2 (SCALAR),
which found it re-deriving the gate its own change moved, and which
did not widen the gate to pass.
