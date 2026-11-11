---
id: a-stated-spline-image-is-certified-twice-by-the-edge-certificate-and-the-mint
kind: issue
title: A stated chart image on a spline chart is certified twice per build, by its edge certificate and again by the mint
status: open
opened: 2026-11-11
priority: P3
cost: E
---

Filed by `pcert/stated-image-bounded` (the row
`a-stated-chart-image-on-a-spline-face-is-certified-by-a-sampled-meter`).

## What

Since that PR the edge certificate bounds a stated chart image on a
spline chart through its pcurve certificate (`certify.rs` `stated_bound`
→ `PcurveCache::certify_stated`) and keeps only the bound
(`Certificate::max_residual`). Every product path then mints the same
image's row (`topo::mint_pcurves`, `pcurves.rs` through the same door),
so the same certificate is derived twice per stated edge.

- Per call, `EdgeCurve::certify` of a stated spline image went from about
  3–8 µs to 15–640 µs at `f64` (an iso line on the ring and parabola
  walls ~15–30 µs; a curve-in-UV general image ~380–640 µs) and from
  ~35–45 µs to ~320–680 µs at `Interval` (release build, the
  `stated_image_rows` fixtures).
- End to end the loft and spline rows moved about 1.03× (ten rows, each
  run twice one at a time), so the mint dominates; the arc loft's
  symbolic rim reads count one more (`a_loft_computes_its_walls_in_t`).

## Fix

Hand the certified `PcurveCache` the edge certificate derived to the mint
(an `EdgeCurve` carrying the stated image's certificate, or a mint that
accepts one for the same carrier and span, `covers`-style), so a stated
image is certified once per build. Not a soundness item.
