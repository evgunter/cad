---
id: the-ten-readings-that-are-not-levers-are-decided-by-what-they-are
kind: unit
title: the ten readings that are not levers are decided by what they are
status: closed
opened: 2026-11-22
priority: P1
cost: H
refs: [7410, 7268]
branch: oneround/readings-not-levers
pr: 7442
closed: 2026-11-23
---


The unit that converts the ten `Lever::unsourced` references the
stand-ins unit (PR 7268) left pinned. Its question was a design fork
(`docs/DESIGN-FORK-LOG.md` row 303); Ev answered on PR 7410
(2026-11-22): "this is definitely a correct code change". The
question's row, filed by the stand-ins lane on its branch, is
`unsourced-levers-with-no-door-that-names-them`; this unit closes it.

What a lever is (`crates/geom-core/README.md`, "Float rounding", as PR
7410 left it): the length at which a tilt displaces the geometry it
governs, and a reading is levered only where that displacement is what
is held against the tolerance. None of the ten is that.

## Per site, the final state both designers signed

- `geom_brep::dihedral::wedge_reads_zero_at_the_arm`:
  `Margin::levered(sin θ, arm)` at the wedge's real, sourced arm (both
  callers hold it), decided in the band at the tolerance the arm
  refusal offers. That band comes from one new door in geom-core,
  `Indeterminate::offered_band() -> Option<Band>`, built from the same
  spelled offer the refusal's sentence prints; `None` means no offer.
- `geom_brep::dihedral::wedge_reads_zero_at_the_offer`: the same wedge
  in the pairing refusal's own `offered_band()`. The `÷|cos θ|` goes.
- `topo::boolean::fate::tip_wraps`: the existing `Period::decide` with
  the half-turn as the period, on `whole − turned`, metered at the
  least of the corners' reaches read from below; the integer it returns
  is compared with ± the corner count as integers. `Undecided` maps to
  the corner escalation the site already raises, `OnMark` to
  `TipUnread`.
- `topo::boolean::join::off_conic_slack`: the swing of the existing
  `k_stats::decide_across(names, datum, swing, band)` inside `travel`.
  `travel` today decides the reading and the slack one at a time for a
  tie, which `decide_across`'s own doc calls unsound (two terms each
  just inside the band sum to nearly twice it): the conversion fixes
  that. `bool_join_arc_slack` leaves the K-name roster.
- `geom_brep::intersect::pn_apex_tangent_reach` and
  `pn_apex_point_reach`: `Margin::of` of one named function for the
  apex start, `(|δ| + rounding)/sine`, a plain division on the arm
  where the divisor is decided definite.
- `geom_brep::intersect::pair_bound`'s far, near and offset: plain
  arithmetic inside the bound, far and near calling that same function;
  the bound still ends in its existing `Margin::of`.
- `topo::validate::resolved_as`: `Margin::of(ρ)` in the band one K-step
  finer, `(ε/K, ε)`, through a named constructor in geom-core, until
  `site-rounding-charges-owed-a-re-posing` removes it.

Both derived bands get named constructors in geom-core; no site builds
either with `Band::new`
(`a-band-built-by-hand-from-thresholds-is-pinned-by-no-gate`).

## Not decided by the designers; the unit finds out and reports

- Which fold of the corners' `LeverBounds` is "the least reach from
  below" for the count's meter.
- How `offered_band` reads on the `Interval` lane, and whether it sits
  at the told offer (`Band::offer_told`, private today) or at the raw
  `|m|/K`: it should match the number the refusal's sentence prints.
  `MarginDiag` is unreadable outside geom-core, so the door lives in
  `predicate.rs`.
- That the pairing refusal's margin is `|cos θ|·arm`.
- `vertex_section` calls `resolved_as` before it decides, so it has no
  refusal in hand: it needs the `(ε/K, ε)` form.

## Out of scope

- Reading the join's sites on the conic itself, so the swing goes: it
  changes what `Turn` holds (Ev's records condition, PR 7141) and is
  its own row, to be filed by this unit.
- The `gap_err` rounding term inside the apex start: it goes with the
  re-posing row.
- Pinning `Band::new`
  (`a-band-built-by-hand-from-thresholds-is-pinned-by-no-gate`), beyond
  the two named constructors.

## Expected to move, to be measured and reported

Last bits at the two `dihedral` sites (and the offer's rounding to the
told grid); more tips decided at `tip_wraps` (a corner admitted at a
1 mm reach can leave the turning sum 3e-7 half-turns off an integer,
which the unit arm refuses today: argued by a designer, not run); none
at `intersect` if the product is kept, a last bit if spelled as a
division; a stricter tie at the join (reading plus swing within ε
together).

Letter H, tier single FULL: ten sites in five files and a new door in
geom-core, verdicts move at two of the families, and a wrong meter or
band reads plausibly with no row going red.

## Closed (2026-11-23, PR 7442)

The ten references are each decided by what they are, as Ev's answer on PR 7410 set out: the wedge read at the offer in the refusal's own offered band; a tip's wrap counted as an integer of half turns, metered at the half-turn corners' least lever; the join's travel decided across its stand-off through `decide_across`; the apex reaches as a quotient formed on the sine's definite arm; `resolved_as` in a band one step finer. `lever-sources.counts` falls by ten. A single FULL review with no MAJOR, a fix pass, a delta read that approves with fixes, a last pass read by the orchestrator. Inside the fix pass the shared door `decide_across` was found to over-offer for every caller with a nonzero swing: its far row's refusal now offers what the bracket's floor supports, or nothing. What the unit left, each in its own file:
- `travels-refusal-withholds-a-tolerance-that-exists-when-its-swing-scales-with-the-band` (P3).
- `the-joins-sites-are-read-on-the-conic-so-the-travel-carries-no-swing`.
- `work/instr/k-names-that-record-another-quantity-or-band-since-the-readings-unit.md`; `work/pcert/a-pinned-fuzz-witness-reads-the-environments-seed.md`.
- `site-rounding-charges-owed-a-re-posing` carries the apex's rounding term, now pinned, and the sweep's hit list.

The row is `docs/model-compare-log/rows/PR-7442.md`.
