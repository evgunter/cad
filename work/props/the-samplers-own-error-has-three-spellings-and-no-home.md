---
id: the-samplers-own-error-has-three-spellings-and-no-home
kind: issue
title: "The sampler's own error has three spellings and no shared home: the crate all three reach is geom-core"
status: open
opened: 2026-09-21
priority: P3
cost: D
---

## What

Three places in the tree compare a dense `f64` SAMPLE against a
certified bound, and each has to add the sampler's own rounding back
because the certificate never covered it. RING-2 (PR #3032) minted all
three in one change, and they are three spellings of one rule:

| site | spelling | shape |
| --- | --- | --- |
| `crates/mesh/src/nurbs_cert.rs` | `SAMPLER_ULPS = 64.0`, named, production | RELATIVE: 64 ulps of the certified figure |
| `crates/geom/tests/curves/hull_circle_rehearsal.rs` | `SAMPLER_PLANE_SLACK_ULPS = 2.0`, named | ABSOLUTE: ulps of `r`, derived from the one measured escape; the sphere limb takes none |
| `crates/geom/tests/curves/review_m5_pr2_e2e.rs` | `64.0 * f64::EPSILON * 4.0`, a literal with a prose "≈ 4" scale | ABSOLUTE: ulps of the residual's operand scale |

They are not the same number and should not be — one is relative and
two are absolute at different scales — but they are the same
OBLIGATION, and nothing in the tree says so in one place: what the
sampler's error is, why it is outside the certificate, and how a lane
picks a bound for it. The rehearsal's version is now derived from a
measurement and the other two are house scales, which is the
difference a shared home would have made visible.

## Why RING-2 did not fold them

`crates/mesh/src/nurbs_cert.rs` is production code, so `test-utils` —
the obvious home for a testing constant — cannot be it: `test-utils`
is a dev-dependency and `mesh/src` cannot name it. The lowest crate
all three reach is **`geom-core`**, and the constant would have to
live at its root rather than in `ring_interval.rs` or `spline/*`,
which are the only `geom-core` paths in RING-2's fence
(`docs/RING-2-SPEC.md` §5). So the fold is out of fence by exactly one
file, and the three sites carry a cross-reference to this row instead.

## What a fix looks like

One documented home in `geom-core` — not one constant, since the
shapes differ, but one doc and one named unit each:

* the rule: a certified bound encloses the REAL quantity, so a
  comparison against an `f64` sample of it widens the SAMPLE's side
  and never the bound's;
* how to size it: count the sampler's own operations at the
  residual's own scale, and where a measurement exists, use it (the
  rehearsal's escape is `3.91e-16` against a `9.42e-16` bound; the
  house 64 would have tolerated `3.55e-14`, thirty-eight times the
  bound it guards);
* the three call sites re-pointed at it.

## Disposition

PROPS': `crates/geom-core/src/` is this program's ground, and the home
has to be there for `mesh/src` to reach it. The consumers are TESS
(`nurbs_cert.rs`) and TCOST/TINT (the two test files); they are named
here rather than filed separately because one home is one change.
Filed by RING-2 (SCALAR), which minted all three spellings.

## A measurement for the `nurbs_cert.rs` site (TESS orchestrator, 2026-09-21)

The sampler's error at that site HAS been measured, in exact rational
arithmetic (`work/tess/rational-cells-hull-the-f64-refined-net-so-the-
described-patch-escapes.md`): ≤ 6e-17 absolute per channel on a
second-partial norm of 2.66 — about 0.1 ulp relative — on the sweep's
own failing surface. The 64-ulp house figure is ~50× that, and what it
absorbs there is a proven certificate escape (+3.07e-16 relative), not
sampler rounding. TESS-2 (`docs/TESS-2-SPEC.md`) fixes the certificate
and measures the sampler error over a bilinear census so this row can
size the allowance from data; the falsifier-side consequence is
`work/chord/soundness-sweep-allowance-is-fifty-times-the-measured-
sampler-error.md`.

## A measured number for the rule, from TESS-2 (2026-09-22)

TESS-2 refereed `crates/mesh/src/nurbs_cert.rs`'s sampler exactly, so
the `SAMPLER_ULPS = 64` row now has a measurement beside the
rehearsal's. Over 400 random bilinear rational patches, `sample_worst`
against the exact rational truth at the sampler's own argmax,
`|sampled − truth|` in ulps of the certified figure: median 0.13–0.30,
p99 1.07–1.47, max 1.39–2.75 across three independent draws (the full
table and the draws are on
`work/chord/soundness-sweep-allowance-is-fifty-times-the-measured-sampler-error`).
The house 64 is 23x to 43x that — the same order as the factor the
rehearsal site measured by a different route, which is the sharper form
of this row's point: two independent measurements of the same obligation
both land near a quarter of an ulp at the median and both sit far under a
house figure of 64.

Two things the measurement adds to the "how to size it" bullet. The
median is not the number to size from — the MAXIMUM over draws is, and it
moves between draws (1.39 to 2.75), so a re-sizing wants a tail over
several runs rather than one run's worst. And the apparatus is committed,
so this is a re-runnable method and not only a number:

```
cargo test --release -p mesh --lib sampler_error_dump -- --ignored --nocapture \
  | python3 crates/mesh/tests/sampler_error.py
```

with `crates/mesh/tests/nurbs_exact_referee.py` as the exact engine
(rational arithmetic, no `Decimal` in the rounding, shares no code with
the kernel).
