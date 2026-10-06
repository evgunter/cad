---
id: ssi-loose-side-hull-reports-a-region-on-an-empty-locus
kind: issue
title: a side whose Bernstein hull straddles zero though phi along it does not reads in band, not clear: the search reports a Side region on an empty locus
status: open
opened: 2026-10-04
priority: P2
cost: M
refs: [limb3-at-rest-proves-the-graph-not-the-arc]
---


## Found (PR 4012's delta review, probe `d_loose_phantom` on `analysis/limb3-rest-review/delta`; filed 2026-10-04)

A wall `z = k·x + h(y)` over `[0, 1]²`, `h` quadratic over `m` C0
spans with Bernstein coefficients `(P, −N, P)` on each, `P > N > 0`.
Along the side `x = 0`, `φ = h ≥ (P − N)/2 > 0`, and the wall rises
inward, so the exact locus over the wall is empty: #3862's rule reads
this side clear. But every piece's hull holds `−N`, so no piece of the
side is certified one-signed:

- `boundary_section` (`section.rs`) takes `side_of_plane` from
  `Pieces::distance`, the hull of the whole side's Bernstein ratios,
  which straddles zero; `Pass::side_region` (`boundary.rs`) then never
  reads `clears` true, and the side's cover is within ε, so the search
  reports a `Side` region (reach ≈ 1.6ε) on an empty locus.
- At rest, `read_stretch` cuts the stretch into 64 pieces, each still
  a hull over whole Bernstein spans when `m ≥ 64`, so no piece clears
  either, and limb 3's side arm would accept the side. Only limb 2
  (`HullSup`) refuses a carrier along it today.

Measured at the head of PR 4012, ε = 1e-9, `P = 0.6ε, N = 0.2ε` (and
`0.9ε, 0.5ε`), `k ∈ {10, 100}`, `m ∈ {64, 256, 1024}`: the search
answers 0 branches and one `Side { fixed: U, end: Low }` region at
every row; the declared carrier `(0,0,0) → (0,1,0)` refuses
`Limb { HullSup }`.

## Whether it matters

Unsure. Under D4 the plane lies within ε of that side, so a `Side`
region there is in band; what is lost is #3862's exact empty answer,
for an input whose own net hides it. The doors still agree on
certifying nothing.

## Repair shape

Decide a side's sign on a refined hull: subdivide a piece whose hull
straddles zero (or degree-elevate) before reading it unsigned, in the
one reader both doors share (`section.rs`'s `Pieces`), so the clear
test and the side arm move together.
