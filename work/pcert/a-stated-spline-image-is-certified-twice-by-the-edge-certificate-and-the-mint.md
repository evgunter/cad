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
→ `PcurveCache::certify_stated_image`) and keeps only the bound
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

- Knot-rich carriers (review fix LL4): a degree-3 net with 11 interior
  knots on the ring wall, stated as a curve in UV, per
  `EdgeCurve::certify`, base `ab4fc6ca` against the PR's head:

  | carrier | build | base f64 | head f64 | base Interval | head Interval |
  |---|---|---|---|---|---|
  | on the wall | release | 0.07 ms | 1.42 ms | 0.41 ms | 1.77 ms |
  | in band (½ band) | release | 0.42 ms | 0.55 ms | 0.99 ms | 1.26 ms |
  | 0.999 × escalate | release | 0.40 ms | 0.53 ms | 0.97 ms | 1.19 ms |
  | on the wall | debug | 0.43 ms | 9.42 ms | 2.12 ms | 11.28 ms |
  | in band (½ band) | debug | 3.25 ms | 4.29 ms | 6.29 ms | 7.89 ms |
  | 0.999 × escalate | debug | 3.07 ms | 3.96 ms | 6.39 ms | 8.03 ms |

  The read's own samples (`own_fractions`) cost the refusing carriers
  about 1.3×; the on-wall carrier's cost is this row's double
  certification.
- The review's fuzz carriers (its `no_false_off` family: degree 1–3, up to
  11 interior knots, offsets `0, ½·zero, zero, ½ band, 0.999·escalate`),
  24 draws on a pinned seed, debug build, per `EdgeCurve::certify`:
  base 32.1 s (`f64`) and 138.3 s (`Interval`) in all, head 38.5 s and
  154.5 s (1.20× and 1.12×). The slowest four on each side are all at
  `d = zero` exactly: base 3.5–4.3 s (`f64`) and 17.3–17.8 s
  (`Interval`) per call, head 4.6–5.6 s and 19.4–21.9 s. So the review's
  seconds per call are not this PR's: base is as slow, and the cost is
  a carrier at the band's zero edge,
  `a-carrier-at-the-bands-zero-edge-of-a-spline-wall-costs-seconds-per-certify` (P3).

## Fix

Hand the certified `PcurveCache` the edge certificate derived to the mint
(an `EdgeCurve` carrying the stated image's certificate, or a mint that
accepts one for the same carrier and span, `covers`-style), so a stated
image is certified once per build. Not a soundness item.
