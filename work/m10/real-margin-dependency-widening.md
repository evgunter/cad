---
id: real-margin-dependency-widening
kind: issue
title: the next ceiling: dependency widening in the numeric channel of margins that are NOT identities
status: open
opened: 2026-09-04
---

**Found by M10-7's two reviews, by execution.** ERROR-DESIGN E12 rests
on a division of labour: the certification IDENTITIES widen with the box
and the symbolic tier discharges them, while "plain intervals suffice
for the real margins". M10-7 built the tier and the first half held. The
second half does not, and it is what bounds the slab today.

## What was measured (2026-09-04, M10-7's head, `slab(1.0, half)`)

The whole-certifying half-width, by bisection through the driver's own
doors, and what happens beyond it:

| ε | ceiling (certifies WHOLE) | tier OFF |
| --- | --- | --- |
| 1e-6 | 0.438988 | 1.2500e-7 (= ε/8) |
| 1e-9 | 0.488315 | 1.2476e-10 (= ε/8) |
| 1e-12 | 0.497892 | below the search floor |

Beyond the ceiling there are two more regimes before the flip:

| `half` | what happens |
| --- | --- |
| ceiling … ~0.5 | certifies AFTER bisection, nothing refused |
| ~0.5 … 1.0 | `newell_plane_residual` INVALID, and stays so |
| 1.0 | the extrusion flip (`FlipCrossing`) |

- What ends the first regime is **`dihedral_wedge`** landing in the
  band: at the ceiling its enclosure is `[9.99e-12, 7.52e10]` against a
  band of `(1e-12, 1e-11)`. One bisection separates it, so the box still
  certifies — it just stops certifying in one leaf.
- What ends the second is different in kind: the side wall's Newell
  normal has an enclosure that **reaches zero**, normalization drops the
  decoration to `Trv`, and `newell_plane_residual` reports an invalid
  margin. An invalid margin is not narrowed by bisection, so the driver
  spends its leaf budget and refuses `Budget`.

Neither is an identity, so neither is reachable by the symbolic tier;
both are the dependency problem in its ordinary form — an expression
mentioning a parameter several times, enclosed term by term.

**The ceiling is ε-dependent** (it RISES as ε tightens), which is the
signature of a band-straddle rather than of geometry: a narrower band is
escaped by a wider box.

## A second site, on curved geometry

R1's review measured `arc_diameter_clearance` on a two-hole bracket at a
±0.5 mm box: enclosure `[−3e-4, 1.7e-3]` on a margin whose true value is
0.73 mm. The interval spans zero on a clearance that is three decades
away from zero — the same widening, on a predicate nobody would call an
identity.

## Why it is filed rather than fixed

M10-7's scope is the identity tier and E3's lever. This is the mechanism
the tier HANDS THE CEILING TO, and naming it is what M10-7 owes; moving
it is a separate piece of work with its own design question, because the
remedies are not the tier's. The candidates, none of them chosen here:

- **Re-association at the site** — rewriting `newell` so the parameter
  appears once. Site-local, cheap where it applies, and E12 already
  rejected it as a general answer (sites see values; the dependence is
  lost upstream). Worth costing at these two sites specifically.
- **Centred forms / affine arithmetic in the numeric channel**, which is
  the standard remedy for first-order dependency and was rejected for
  the IDENTITY problem on a `√ε` ceiling argument — an argument that
  does not obviously carry to margins that are not identically zero.
- **A domain-honest normalization** for the Newell case in particular:
  the `Trv` drop is correct (the normal's enclosure contains zero, so
  the unit normal genuinely does not exist over the whole box) and the
  question is whether the wall's normal should be built that way at all.

## The next thing to measure

Which predicates in the driver population have enclosures whose width is
dominated by repeated occurrences of a parameter, ranked. M10-7's K
sweep already writes every driver margin with its band
(`target/k-fresh/driver/k-eps-*.csv`); the ranking is a query over that
file plus the expression each margin came from, and it would say whether
these two sites are a class or a pair.

## Sites

- `crates/sweep/src/extrude.rs` — the side-wall Newell residual
  (`newell_plane_residual`) and the join classifier (`dihedral_wedge`).
- `crates/editor-core/tests/m10_3_driver_interval.rs` —
  `the_certification_width_is_no_longer_bounded_by_epsilon` asserts the
  mechanism and the ceiling band, so a move here reds that row rather
  than passing silently.
- `crates/geom-brep/` — `arc_diameter_clearance`, R1's bracket site.
