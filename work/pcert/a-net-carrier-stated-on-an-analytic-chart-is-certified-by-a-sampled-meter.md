---
id: a-net-carrier-stated-on-an-analytic-chart-is-certified-by-a-sampled-meter
kind: issue
title: A spline net stated on an analytic chart is certified by a sampled meter, so a net off its plane, cylinder or cone between the samples gets an edge certificate
status: closed
opened: 2026-11-11
priority: P1
cost: M
pr: 6826
closed: 2026-11-11
---

Filed by `pcert/stated-image-bounded` (PR 6815, review fix LL1), from the
review's probes on `review/6815-full` at `1827b743`
(`crates/geom-brep/src/pcurve_cache/review_6815_probes.rs`,
`analytic_bump_between_the_samples_f64` / `_interval`, red by design).

## What

A degree-1 spline net along a ruling of a plane, a cylinder or a cone,
with one vertex at `t = 1/128` pushed 1 mm off the chart along its
normal (vertices at `0, 1/256, 1/128, 3/256, 1`), stated by its chart
image as a `Pcurve::IsoLine` or as a degree-1 `Pcurve::General`, gets a
kernel-issued edge certificate: `EdgeCurve::certify` answers `Ok` on all
three charts, through both images, at `f64` and `Interval`, at ε 1e-9 and
1e-12. That is 12 certificates per ε for a carrier 1 mm off its face.

## Trace

- `certify.rs` `run_checks`: a stated image on an ANALYTIC chart is read
  first (`StatedChart::Analytic` → `stated_analytic_read`) and then
  metered by the schedule's `pcurve_map_residual` at the nine
  `CERT_SAMPLES`. PR 6815 bounds a stated image over its span only on a
  spline chart (`StatedChart::Spline` → `stated_bound`).
- The read of a net on an analytic chart is `pcurve_cache.rs`
  `net_incidence` (through `read_shown`), at the `INCIDENCE_SAMPLES` + 1
  (65) even samples of the span. The bump lies between both sample sets. The spline
  chart's read samples a net's own breakpoints and Greville abscissae
  (`spline_incidence::own_fractions`); `net_incidence` does not.
- The pcurve doors refuse the same image (`PcurveCache::certify_stated_image`,
  the review's `analytic_bump_through_the_pcurve_door`), so the mint and
  tier 3 are closed. Only the edge-certificate door is open.
- `pncad` re-exports `geom_brep` whole (`pncad/src/lib.rs`), so any pncad
  caller obtains the certificate through
  `pncad::geom_brep::EdgeCurve::certify` (or `certify_stated`). That is the
  P1 criterion, as on the spline-chart row PR 6815 closed
  (`a-stated-chart-image-on-a-spline-face-is-certified-by-a-sampled-meter`).
- A sphere and a torus take the same path (read-first plus the nine-sample
  meter, `net_incidence` sampled), so they are argued members, not
  probed.

## Why it is not in PR 6815

Routing every stated image on an analytic chart through the pcurve doors
refused about seventy product rows (an iso line on a plane, a harmonic
image restated on a reverted or mirrored plane, restrictions placed in
another window): those images are not all ones a pcurve door certifies.

## Fix

Either make `net_incidence` (and the conic reads of
`a-closed-conics-and-a-spline-nets-incidence-on-an-analytic-chart-is-sampled`)
a supremum over the span, a bound per knot span subdivided where it does
not decide, as the spline chart's cell read is; or route a stated image on
an analytic chart through its pcurve door where that door certifies the
image class (a net carrier's `General`/`Fitted`/`IsoLine` images), keeping
the read-first path for the closed-form restatements the product stores.
Port the review's three charts × two images × two scalars as rows; a
carrier on its chart must still certify.

A candidate for the bound already exists: `pcurve_cache/projected.rs` has
a second, unrelated `net_incidence`, a hull-composite bound over the
projected image's parts (`ProjectedHull`), which is a sup where the
sampled read is not. Two same-named functions of opposite natures (one a
sampled lower bound, one a bound over the span) are worth a rename in the
fix, whichever it takes.

## Closed (PR 6826, 2026-11-11)

The fix took the first shape: the read of a net stated on an analytic
chart is a supremum over the span at the edge certificate. `certify.rs`
`stated_analytic_bound` reads a `Curve3::Nurbs` carrier through
`pcurve_cache::read_net_bound`. That read runs the sampled read first,
and a definite miss at a sample ends there. It then reads limb 2's bound
of the net's distance from the chart over `[t0, t1]`
(`limb2_offset_sup`: the chart's canonical composite along the net, a
bound per part, the parts halved where the bound does not clear). The
bound is decided by the edge certificate's own limb-2 decider
(`edge_nurbs::limb2`), so there is one home for "a net's distance from
an analytic chart over a span", and `projected.rs`'s hull-composite
`net_incidence` is its per-part metre conversion. The sampled read is
renamed `net_sampled_incidence`: it is a lower bound, kept for the mint's
and the lane choices' reads, whose images are then bounded by their
pcurve certificates.

Endings:
- a measured miss past the band ends `Incidence(Off)` (`CarrierOffChart`);
- a bound or floor in the band, an enclosure that neither clears nor
  misses, an upper bound past the band with no miss, or no certified
  bound at all (a logged straddle, `pcurve_net_bound_straddle`, in
  `net_bound_refusal`) ends `Incidence(Undecided)`, the lever;
- a net with no certified enclosing net (its data not finite) ends
  unreadable, as the spline chart's cell read does.

Rows (`pcurve_cache/analytic_net_rows.rs`):
- the witness, three charts × two images × two scalars;
- the bump narrowed toward the band;
- a net beside a cone's apex (the P2's row);
- the no-false-Off search against an exact supremum, on all five charts.

The mutant `net-bound-sampled` turns the witness and the narrowed bump red.

Not closed here, filed: the stated image's own fidelity to the carrier
on an analytic chart is still metered at the nine samples
(`a-stated-image-on-an-analytic-chart-is-metered-against-its-carrier-at-nine-samples`, P1).
