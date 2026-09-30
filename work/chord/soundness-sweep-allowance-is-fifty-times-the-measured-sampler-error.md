---
id: soundness-sweep-allowance-is-fifty-times-the-measured-sampler-error
kind: issue
title: the rational soundness sweep's 64-ulp sampler allowance is ~50x the measured sampler error and hides the certificate's proven escape
status: open
opened: 2026-09-21
refs: [rational-cells-hull-the-f64-refined-net-so-the-described-patch-escapes, TESS-2]
---


Filed by the TESS orchestrator, 2026-09-21. CHORD's because the
falsifier is a guard over a certificate; TESS-2 depends on it.

## What

RING-2 (SCALAR, PR 3032) made `nurbs_cert::tests::Domination::
sampled_under_certified` widen the certified side by `SAMPLER_ULPS =
64` ulps, relative, and routed `r1_random_rational_soundness_sweep`,
`hessian_hull_dominates_sampled_second_partials` and every
`second_partials` row through it, on the premise that the reds those
rows showed under the new ring were the sampler's own rounding — "a
sampled `uu` sat under a certified one it exceeds in the reals by
nothing at all".

That premise is contradicted by a measurement already in the tree.
`work/tess/rational-cells-hull-the-f64-refined-net-so-the-described-
patch-escapes.md` (evidence branch `tess/nurbs-bound-diag`,
`db4cb45a8`, exact rational arithmetic): on the sweep's own failing
surface the DESCRIBED patch's true `‖S_uu‖` exceeds `muu` by 3.07e-16
relative (~1.4 ulps), and the sampler's own error is ≤ 6e-17 absolute
per channel on a value of 2.66 — about 0.1 ulp. The escape is the
certificate's (`patch_bound::rational_cells` hulls the f64-refined
net), not the sampler's.

So the allowance is ~50× the sampler error it is sized for and ~50×
the defect it now hides: the sweep is green on main and the
certificate is unsound. The PROPS row on the three sampler-allowance
spellings already notes the house 64 "would have tolerated 3.55e-14,
thirty-eight times the bound it guards" at the rehearsal site.

## What is owed

- TESS-2 fixes the certificate and MEASURES the sampler's error
  against the exact referee over a bilinear census (spec §The
  allowance), and puts the number here.
- With that number in hand, the allowance is re-sized from the
  measurement (the PROPS row's "where a measurement exists, use it"),
  or the sampler evaluates in the ring so the comparison is enclosure
  against enclosure and bare. Until then a 1e-15-relative escape in
  any rational certificate is invisible to hosted CI.

## The measurement TESS-2 owed, 2026-09-22

Measured with the exact-rational referee committed as
`crates/mesh/tests/nurbs_exact_referee.py`, driven by the apparatus
`nurbs_cert::tests::sampler_error_dump` +
`crates/mesh/tests/sampler_error.py`. One command re-takes the whole
table, which is the point of committing it:

```
cargo test --release -p mesh --lib sampler_error_dump -- --ignored --nocapture \
  | python3 crates/mesh/tests/sampler_error.py
```

400 random bilinear rational patches (degrees 1x1, knots `[0,0,1,1]²`,
log-uniform weights 1e-2..1e2 — `r1_random_rational_soundness_sweep`'s
own draw restricted to the tight stratum), and for each one
`sample_worst(&s, 60)`'s value against the exact truth AT THE SAMPLER'S
OWN ARGMAX, per component.

`|sampled − truth|`, in ulps of the certified figure, over **three
independent draws** (TESS-2 took two, the blinded review a third):

| component | median | p99 | max |
| --- | --- | --- | --- |
| `uu` | 0.27–0.30 | 1.21–1.43 | 1.39–1.97 |
| `uv` | 0.13 | 1.07–1.30 | 1.23–1.94 |
| `vv` | 0.27–0.28 | 1.39–1.47 | 1.63–2.75 |

TESS-2's figures on the frozen-then-fixed head (seed
`0xb705e20940cf4c84`) are the third row of each range: `uu` median
0.2956 / p99 1.4255 / max 1.6705; `uv` 0.1328 / 1.1410 / 1.4394; `vv`
0.2751 / 1.4731 / **2.7462**.

**The medians are a quarter of an ulp and the maxima are 1.4 to 2.8**, so
`SAMPLER_ULPS = 64` is 23x to 43x the sampler's measured error depending
on the draw — the same order as the factor the PROPS row measured at the
rehearsal site by a different route. Note that "under 2 ulps" does NOT
hold on every draw: `vv` reached 2.75 on TESS-2's third draw, so a
re-sizing wants the tail of several draws and not one run's maximum. The
absolute figures scale with the value (max 1.965e-9 on a certified
3.22e6), which is why the relative spelling is the right one here even
though the other two sites are absolute.

Two figures to size against rather than one: the sampler's error above,
and the escape the allowance must not hide. On TESS-2's merge base the
certificate's proven escape was 1.836e-15 relative (8.3 ulps) on fixture
A and 1.601e-15 (7.2 ulps) on fixture B — both inside 64 and outside 3.

The certificate half is closed: TESS-2's arms refine in the ring, so a
bare comparison of 30,000 bilinear trials shows 0 escapes where the same
draw on its merge base showed 2 in 6,000, and the general sweep 0 where
the merge base showed 1 in 1,500. The allowance is still 64 and still
hides anything under 1.4e-14 relative, so this row stays open on its own
terms.
