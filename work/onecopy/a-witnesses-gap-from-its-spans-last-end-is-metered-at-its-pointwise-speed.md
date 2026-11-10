---
id: a-witnesses-gap-from-its-spans-last-end-is-metered-at-its-pointwise-speed
kind: issue
title: pair_section's within_span meters a witness's gap from its span's last end at the carrier's speed at the witness, which bounds no arc over [t, hi]
status: closed
opened: 2026-11-10
priority: P3
cost: E
closed: 2026-11-10
---


Found by PR 6710's FULL review (MINOR-3), split out of
`a-parameter-gap-metered-by-hand-at-a-span-end-has-no-gate`, which
closed with this owed.

## What

`crates/geom-brep/src/pair_section.rs` `within_span` (~`:2764`) decides a
conic section component's witness `t` inside its span by
`Margin::metered(hi − t, InfSpeed::new(|C′(t)|))`
(`section_site_in_span`). Its Negative is then read past the span by
twice the zero (`section_site_in_span_past`). The meter is the carrier's
speed AT the witness, which is no bound over `[t, hi]`. On an ellipse the
speed between `t` and `hi` can be lower or higher than at `t`. So:
- a Positive "inside" can over-read the arc to `hi` where the speed falls;
- a Negative "past" can under-read the overshoot where it rises.

On a circle the speed is constant and the reading is exact. The error is
second order in `hi − t` (the speed's variation over the gap), so it
matters only near the band, at a thin ellipse's vertices.

## What closing it takes

Meter the gap by bounds over the gap itself. Inside wants the inf over
`[t, hi]` (`geom::carrier_speed::speed_range`'s floor is one, coarse;
`turn_defect_rate`'s local inf over the defect is tighter). Past wants
the sup. Or read the witness's point against the end by its chord, as
`curve_pair::Span::place_param` does, if the witness's twice-zero reach
can be stated on the chord.

`scripts/gates/span-end-gap-sites.tsv` holds the site as class `witness`,
and its header points here.

## Done

`within_span` reads its gap to the last end through `Span::end_gap`, metered over the gap:
- inside, at the inf (`GapMeter::Clear`), so a decided side is sound;
- the past-by-twice-the-zero check, at the sup (`GapMeter::Within`), so "nearer than that" is sound.

The pointwise speed is gone. `curve_pair::span_rows::a_gap_from_a_spans_end_is_metered_over_the_gap` pins the door on a thin ellipse against a quadrature of `|C′|`: the pointwise meter over-reads the arc on one side of a vertex and under-reads it on the other.

**Measured:** a probe decided main's margin beside the door's at every call, over the ci sets at both ε (1399 `within_span` readings and about 860 graze readings each). Not one decision moved.
