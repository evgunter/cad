---
id: trim-piece-monotone-span-fold-drops-a-refused-window
kind: issue
title: props_trim_piece_monotone folds its span with f64::min, so a refused chord window drops out of the margin instead of escalating it
status: open
opened: 2026-10-01
---


Found by the SSI lever-arm lane's shape sweep (a `min`/`max` fold whose
result feeds a guard with a NaN or indeterminate arm), PR "SSI: the
lever-arm fold propagates poison, from one home". On `f64` the inherent
`f64::min` shadows `geom_core::Real::min`, and only the inherent one
drops a NaN operand.

## The site

`crates/geom-brep/src/props/quad.rs`, `piece_monotone`:

```rust
let mut span = f64::INFINITY;
for w in block.windows(2) {
    span = span.min(lo_or_refuse(
        (w[1].0 - w[0].0) * du + (w[1].1 - w[0].1) * dv,
    ));
}
```

`lo_or_refuse` answers NaN for a refused projection by design, so that the
refusal reaches the guard; the inherent `min` then drops it. `span` feeds
`classify_len("props_trim_piece_monotone", Margin::metered(span, ..))`,
whose indeterminate arm is the one a refused window should land on.
Instead the margin is the minimum over the windows that answered — the
refused window's advance along the chord, which could be negative, is not
in it. If every window refuses, `span` stays `+∞`, and only a refused
`rate` (from the block box) keeps that from reading as definitely
monotone.

## Why it is latent today

The window projections are sums and products of certified brackets on
finite inputs; a refusal needs a non-finite control point, which the
props doors refuse earlier. Not reached by any fixture the lane knows of.

## Repair

`span = Real::min(span, lo_or_refuse(..))` (the trait method, which
propagates NaN). The file's own `sqrt_enclosure` and the `.max(0.0)`
clamps elsewhere in it are already guarded and are not this shape.
