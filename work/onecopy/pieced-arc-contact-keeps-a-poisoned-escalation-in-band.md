---
id: pieced-arc-contact-keeps-a-poisoned-escalation-in-band
kind: issue
title: pieced_arc::contact keeps a poisoned escalation in band, where ladder::Kept::of makes it a limit
status: closed
opened: 2026-11-06
priority: P3
cost: E
closed: 2026-11-06
pr: 6553
---


Found by PR 6539's review (`onecopy/refusal-standing`).

## What

`splitting::pieced_arc::PiecedArc::contact`'s piece walk keeps each
leaf's unsettled reading through `Evidence::in_band`, whatever its
margin (the `kept.in_band(ArcRefusal::Escalated(diag))` arm). The
ladder's rule (`ladder::Kept::of`, which `Rung::kept` and
`Refusal::Margin` read) makes a poisoned margin a limit, ranked ahead
of an in-band one. So where a poisoned leaf follows an in-band leaf, the
walk refuses with the in-band one's diagnostic, and the ladder would
refuse with the poisoned one's.

Nothing argues for the walk's order. PR 6539 kept it only so that its
port onto `Evidence` moved no outcome.

## What closing it takes

Keep each leaf with `kept.keep(Refusal::Margin(diag), ArcRefusal::Escalated(diag))`.
Pin it with a walk that keeps an in-band leaf, then a poisoned one, and
check that it refuses with the poisoned one. The outcome's variant
(`ArcRefusal::Escalated`) does not move; only the diagnostic it carries
does.
