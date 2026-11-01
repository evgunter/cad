---
id: carrier-cross-in-span-meters-by-the-local-speed-against-the-rate-pair-rule
kind: issue
title: bool_carrier_cross_in_span meters a root's gap by the carrier's local speed through Margin::of, where the rate-pair rule says Margin::metered with an InfSpeed
status: open
opened: 2026-10-12
priority: P3
cost: E
---



Found by the review of PR 4782 (`cleave/one-carrier-reads-ellipses`).
The conflict belongs with the rule, so it is filed here.

## What

`topo::boolean::carrier_cross::strictly_inside` decides whether a
candidate lies strictly inside the swept span. It decides each gap as
`Margin::of(gap * speed)`, where `speed` is the carrier's speed at the
candidate (`crate::carrier_speed::speed_at`). `reduce::wall_crossing`'s
`bool_wall_root_in_span` reads the same way, at the span's end.

`geom_core/src/predicate.rs` ("The rate pair, beside the doors") says
a "definitely apart" claim, naming "a root clear of its endpoints",
takes `Margin::metered(span, InfSpeed)`, which is a certified lower
bound on the rate. For an ellipse that bound is the minor semi-axis.
DR-111 (PR 4372's dual review) measured that metre losing a crossing
≤ ε·a/b from an arc end on an a/b = 20 ellipse: it reads the crossing
as the end. The local speed is the honest metre near Zero: the chord
is the gap times that speed to second order.

## The question

Should the rule admit a local speed for a gap decided near the band,
where the inf is the wrong side of soundness for a Zero, or should
these sites move to the typed door? PR 4782 pins the local metre
(`carrier_cross::crossing_rows::a_swept_span_is_metred_by_its_speed_at_the_meeting`
goes red under the minor-axis or the major-axis metre), so a change
at the rule's home has to re-derive that pin.

## Where it stands

`carrier_cross`'s site is gone: `carrier_cross::boundary_crossing` reads
each candidate against the swept span through
`geom_brep::curve_pair::Span::contact` (stage (c) of
`the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). That reading decides the chord
to each end first, in metres, and meters only the side of an end the
chord already put past the band, by the greatest speed. The pin was
renamed `a_swept_spans_end_is_read_by_its_chord_to_the_meeting` and
holds unchanged. The question stands for `bool_wall_root_in_span`
alone.
