---
id: curve-pair-polish-at-interval-reads-the-iterates-width
kind: issue
title: The curve-pair polish at Interval meters its iterate's accumulated width, not its last step, so fine bands refuse Meter on clean crossings
status: open
priority: P2
cost: M
design: true
refs: [5227, curve-pair-polish-meters-refuse-shallow-meetings]
opened: 2026-10-20
---


Split from `curve-pair-polish-meters-refuse-shallow-meetings` (NURBS
build A, 2026-10-20). That item's fine-band fillet joints in `f64` were
the window's (fixed in its own PR). These are a different meter.

## Finding

At `Interval`, `Polish::closest`
(`crates/geom-brep/src/curve_pair/pose.rs`) refuses
`Unsettled::Meter` on its POLISH meter, not the basin, at crossings
the geometry decides cleanly. It is decided by `decide(row::POLISH,
Margin::of(moved), band)`, which needs `moved` within the zero band.

Probed with `eprintln` at the three meters (uncommitted, on main
`8cbd04be53`):

| row | ε | basin sine | turn | travel | `moved` |
|---|---|---|---|---|---|
| `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval` | 1e-12 | `[1 ± 3e-12]` | `8e-10` | `[0, 2–5e-12]` against a region of `4e-11` | `[0, 1.1e-12]` to `[0, 2.7e-12]` |
| `band_planar_cut_off_interval::a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar` (shear −0.6, convex, round) | 1e-10 | — | — | — | `[0, 2.6e-10]` |

The basin and travel meters pass on every pose. Only `moved`
straddles: its enclosure's upper end is one to three times the zero
band.

**Cause.** `polish::pair` (`crates/geom-brep/src/curve_pair/polish.rs`)
iterates the interval pair `(s, t)` itself for its four steps. Each
step's enclosure carries the last one's width forward (wrapping), so
the final step's `moved = |a′|·|ds| + |b′|·|dt|` bounds the iterate's
accumulated width, not the step. The module's own docs say a polished
parameter is "a place to read, never a decision".

**Consequences.**
- The three ε-1e-12 nightly reds bisected to PR 5227 include the cap
  ring, which is this meter.
- `band_planar_cut_off_interval`'s `tier_3_prime` admits a
  `NotCertifiable` census below `ε = 1e-9`. That admission is needed
  for this meter, at 1e-10 and at 1e-12: with the window fix in and
  the admission removed, the row still refuses on shear −0.6.

## Routes

- **Iterate on thin midpoints** and read only the last step at the
  enclosure: `s ← mid(s)`, then `moved` from one enclosed step at that
  point. A polished parameter is a place, so thinning loses nothing the
  meter certifies. It needs `Bounds::lo/hi` in `curve_pair`, a new
  `bounds_allowlist` entry (`geom_core::real::bounds_allowlist`),
  ratified per entry with a necessity argument (the weakest bound that
  works, the next tighter failing).
- **A `Decide`-only step form**: read the last step in a mean-value
  (centred) form so its enclosure does not carry the iterate's width.
  It needs no new seam, but it is heavier, and it still needs a point
  to centre on.

Either moves `Interval` bits on every polished pose. A design fork:
weighed by the designer pair before any `[ev]` PR.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. `band_ruled_cap_ring::a_bore_inside_the_d_rods_removed_sliver_goes_with_it_at_interval`
is still red at 1e-12 (tier 3′: `ContactLane(NotCertifiable { the meeting
reading of two curved edges could not settle a count })`). It is pinned
at 1e-12 on that text, citing this row; the pin goes red the day the row
passes.

Un-pinning owes a re-measure: each pin stops its row at main's first stop, so whatever runs after it in the row is unmeasured.
