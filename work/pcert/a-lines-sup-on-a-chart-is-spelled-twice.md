---
id: a-lines-sup-on-a-chart-is-spelled-twice
kind: issue
title: "A line's distance supremum on a cylinder or cone is spelled twice: the incidence read's critical points and the projected lane's form bounds"
status: open
opened: 2026-11-04
priority: P3
cost: C
---



Filed by PR 6527's fix pass (review DD5).

## What

Two readings answer "how far is this line from the chart over its
span":
- `pcurve_cache::line_incidence::critical_fractions` names the
  parameters where the exact distance peaks, and the incidence read
  evaluates them (a lower bound that is the supremum);
- `pcurve_cache::projected::line_off_cylinder_sup` and
  `projected::line_off_cone_form` bound the chart's implicit form along
  the segment from above (`segment_form_sup`), in form units converted
  by `Meters`. The projected lane's certificate reads them.

They are not one door today. The projected forms are upper bounds on a
squared form, read where the image certificate needs a ceiling, and the
incidence read is a placed evaluation of the exact distance, where
the Off decision needs a floor. Folding one into the other would
change one of the two lanes' arithmetic.

## Fix

Decide whether the projected lane's ceiling can be the incidence
read's supremum plus its placement charge (one spelling), or record in
both docs why the two stay apart.
