---
id: the-conic-root-lanes-chord-rungs-quote-a-length-where-two-offer-cases-declare-a-negative-margin
kind: issue
title: The conic root lane reads an end and a root pair by a chord, so two offer cases declared negative quote a positive margin
status: open
opened: 2026-11-14
priority: P3
cost: E
---


Filed by GREEN's default-eps lane (`green/default-eps`, 2026-11-14) from
nightly 43371638371 (main `e36675c1`), where the test is red at every ε
row. It replaces GREEN's two rows on the same test
(`work/green/main-red-two-offered-tolerances-fail-just-below-their-margin.md`,
`work/green/main-red-two-offer-rows-quote-a-positive-margin-against-a-negative-case.md`).

## What

`topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it`
(slow set, `crates/topo/src/boolean/offer_rows.rs`) reported two false
offers on main `b363f5fa`:

    arc_root_just_outside_its_span: quotes margin Some(5.500000048223606e-9), computed -5.5e-9
    roots_a_hair_apart: quotes margin Some(5.499881129594528e-9), computed -5.5e-9

Both cases are declared `-D` in the `cases!` table and raise at
`ROOT_SITE` (`conic_roots`, straight into
`geom_brep::curve_face::plane_crossing::plane_crossing_lane`).

## The merge

PR 6687 (`onecopy/needle-basin`, merge `f875961fe`, 2026-11-09). Read
from its diff of `plane_crossing.rs`, not bisected by running:

- `span_place` read `Margin::metered(t - t0, meter)` and
  `Margin::metered(t1 - t, meter)` under `plane_crossing_root_in_span`,
  signed. It now reads `Span::place_param`, which asks an end's raw
  chord first (`curve_pair_span_end_chord`, `Margin::norm3`), a length.
- `span_roots` read `Margin::metered(roots[1] - roots[0], meter)` under
  `plane_crossing_root_order`, signed. It now asks
  `plane_crossing_roots_apart` first, `Margin::norm3` of the chord
  between the two roots' points, a length, and reads the order only
  once the chord is definite.

So a root `D` past its end quotes `+D` as one `D` short of it does, and
two roots `D` apart quote `+D` about either tip of the ellipse. PR 6687
did not touch `offer_rows.rs`, so the slow test did not run on it; the
row was first reported red at `53ed936f` (PR 6705's base, the same
evening).

## The pin

`offer_rows.rs`'s `QUOTED_AS_A_LENGTH` names the two cases, and
`check_value` holds their quote to `|margin|`. The declared `-D` stays,
because `every_arm_that_offers_a_value_has_an_executed_case` counts a
case by its key and its declared side: re-declared `+D`, the offering
arms `("Crossing(OnEdge)", negative)` and `("Crossing(Order)", negative)`
would have no executed case. The pin turns red if either quote goes back
to a signed margin.

## Wanted

One of, decided by reading whether any site can still raise a negative
margin on `Crossing(OnEdge)` or `Crossing(Order)` (unread here: past the
chord rungs the sides are metered at the span's sup speed,
`Margin::metered_sup`, which looks like it leaves no in-band negative,
but that was not run):

- no site can: the two cases are re-declared `+D` (or dropped as copies
  of `arc_root_just_inside_its_span` and
  `roots_a_hair_apart_across_the_window`), the negative arms leave the
  offering census, and `QUOTED_AS_A_LENGTH` goes;
- a site can: a case that raises it replaces each pinned one.

Reproduce (red before the pin, at any ε row):

    cargo nextest run -p topo --lib -E 'test(=boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it)'

`plane_crossing.rs` is SPLICE's by `scripts/work.py territory`, and
`curve_pair/span.rs` CPAIR's and ONECOPY's; the row is on ONECOPY because
the change was its PR and the case table is its ground too.
