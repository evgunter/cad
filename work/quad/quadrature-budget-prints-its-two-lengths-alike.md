---
id: quadrature-budget-prints-its-two-lengths-alike
kind: issue
title: QuadratureBudget renders target and achieved length on a fixed .3e grid, so the refusal can print the two lengths it exists to contrast as one number
status: open
opened: 2026-09-12
refs: [2399]
---



Found by the FIX lane closing
`work/fix/num-relative-tolerance-collides-above-a-decimetre.md`
(PR 2399) under its discipline §5 sweep obligation, and **reported
rather than filed by that lane** — `crates/geom-brep/src/props/*` is
PROPS's glob, and a lane does not file on another program's slate.
Placed here by the FIX orchestrator, who verified it before writing
this file.

## The defect

`crates/geom-brep/src/props/mod.rs`, `PropsError::QuadratureBudget`
(~:479):

```rust
"integral properties: the certified quadrature enclosure cannot reach the \
 {target_len:.3e} m target (which scales with the run's tolerance): its mean \
 boundary displacement is {width_len:.3e} m — …"
```

`{:.3e}` is a **fixed three-decimal exponential**: a ~1e-4 relative
grid, on two lengths whose contrast is the entire point of the
sentence. The arm fires precisely because `width_len` failed to reach
`target_len`, and when the two are within ~5e-4 relative the refusal
prints them as one number:

> cannot reach the `1.000e+00` m target … its mean boundary
> displacement is `1.000e+00` m

That is a refusal contradicting itself in its own sentence — it says
the enclosure could not reach the target while printing the target
twice. A reader cannot tell a near miss from a gross one, which is the
judgement the two numbers exist to support and the difference between
"loosen the tolerance" and "simplify the trim", the two repairs the
same sentence offers.

## Why it is the same class as the row that found it

FIX's `num` helper (`crates/profile/src/path.rs`) had the mirror
defect at the coarse end and was capped at the finer of a relative
1e-9 and an absolute ε/10 in PR 2399, on the argument that **a
difference the kernel can certify is a difference the sentence must
spell**. Here the grid is ~1e-4 relative and never adapts, so it fails
that property by five decades wherever the two lengths are close —
which is exactly the interesting case.

## What a taker should weigh

`.3e` is not obviously wrong as a *choice*: an exponential with a
fixed mantissa is readable, and these are diagnostic magnitudes rather
than certified margins. The defect is that the grid is coarser than
the decision the sentence reports. Three shapes, and the third is
probably right:

1. More mantissa digits — cheapest, and still a fixed relative grid
   that some ε row can outrun.
2. The `num`-style adaptive grid. `num` is private to `profile`'s
   `path` module and `FilletLegCarrier` already wants it too
   (`work/fix/fillet-leg-carrier-renders-raw-float-noise.md`), so a
   third consumer in a third crate is the point at which a shared
   home stops being speculative — DS8's own rule.
3. **Render the DIFFERENCE, not just the two lengths.** What a reader
   needs is how far short the enclosure fell; two absolute magnitudes
   are the raw material for a subtraction the sentence could do
   itself, and a difference is immune to the grid question entirely.

## What it owes either way

No existing row asserts this sentence's numbers — verify that before
relying on it. A pin that reds when the two lengths render alike is
the durable half, on the `num_separates_two_lengths_the_kernel_can_certify_apart`
shape PR 2399 landed.
