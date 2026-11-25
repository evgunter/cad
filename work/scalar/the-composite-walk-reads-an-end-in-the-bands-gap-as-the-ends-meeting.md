---
id: the-composite-walk-reads-an-end-in-the-bands-gap-as-the-ends-meeting
kind: issue
title: The composite walk drops a crossing where the span's end reads in the band's gap of the surface
status: open
opened: 2026-11-24
---


Found by the crossing-parameter unit, 2026-11-25. DEMONSTRATED by `geom-brep`'s `composite::crossing_parameter_rows::settle_answers_one_way_or_refuses_on_admitted_parameters_and_the_split_follows`, whose last assertion pins it.

`composite::crossings`' walk settles a monotone piece whose end at the span's own end reads `Reading::Near` as "that end's meeting" and reports nothing (`(Reading::Near(_), Reading::Past(_)) if piece.a == a => {}` and its mirror). `Near` holds a reading decided zero and one in the band's gap alike. On the row's carrier (a sine of `10⁻²`, the true crossing two escalation lengths inside the span, the end two zero bands off the plane) the answer is `Ok([])`: a crossing twenty zero bands inside the edge is left to an end vertex that does not read ON the plane. The caller reads the end itself and escalates there, so no body is built wrong by it that this unit found; the arm's own answer says "no interior crossing" where the band cannot say so.

It does not turn on which parameter was proposed, so it is outside that unit. What it asks: the gap reading at an end refuses (`CompositeRefusal::Escalated`) and only a zero reading is the end's.
