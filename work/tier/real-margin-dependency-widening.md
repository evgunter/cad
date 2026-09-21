---
id: real-margin-dependency-widening
kind: issue
title: the next ceiling: dependency widening in the numeric channel of margins that are NOT identities
status: open
opened: 2026-09-04
priority: P2
cost: D
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

## A third site, and the bulge family's literal arc

SYM-3's render of R2's D-tab with its bulge a LITERAL (`0.4`, and the
dyadic control `0.5`, `m10_10_evidence_interval`'s `r2_d_tab_literal`
rows): under the shipped set the document certifies 0.4308 (ε = 1e-6),
0.5611 (1e-9), 0.5611 (1e-12) of its real study, bounded at ceiling + δ
by `dihedral_wedge` `[9.99e-6, 2.49e-2]` at 1e-6 and
`arc_diameter_clearance` `[−5.16e-8, 5.63e-4]` at the finer rows — the
annulus's pattern, predicate for predicate. With the algebra off it is
`7.81e2·ε` on `carrier_matches_mapped_source`. The same document with
the bulge a PARAMETER stays ε-relative (`3.52e2·ε`, on and off alike),
so on this family the literal arc is already in this class and the
parameter arc is not yet.

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
- `crates/editor-core/tests/m10_10_r2_probes_interval.rs` — `d_tab`
  with a literal bulge, the third site; `m10_10_evidence_interval`'s
  `m10_10_ceilings_and_the_over_band_set` at `CAD_M10_10_DOCS=r2_d_tab_literal`
  reproduces the bracket.
- **A derived placement's re-derived Newell offsets — the fourth site,
  and the first where the widening stops being a bound and becomes a
  REFUSAL.** A boss on a `Datum::FaceFrame` over a body whose height is
  the widened parameter: at the boss's side plane the translate-to-origin
  offset `(p − centroid).z` encloses `[-0.19954, 0.44840]` for a true
  `−0.125`, the cross-sum's `N.y` then encloses `[-2.0507, 0.8977]` for
  a true `−0.5`, and `Vec3::normalize` divides by a length whose
  enclosure is `[0, 2.0924]` — so the margin is not merely wide, it is
  `MarginDiag::Invalid` and clause 1 refuses before the symbolic tier is
  asked. The tier PROVES that residual zero (its early form is the zero
  form), which makes this the sharpest statement of what this class
  costs. The row, with the eight-call probe and three candidate fixes,
  is `work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`;
  its acceptance test is
  `editor-core/tests/m10_derived_frame_interval`'s ported parity row.

## Re-homed at M10's exit sweep (2026-09-13)

Here because this is the numeric channel's half of E12's division of labour, and
it is what stands between the plate's whole-certifying ceiling (0.263 of its real
study) and the genuine flip at 0.625. SYM's ceiling lane.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.
