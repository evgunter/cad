---
id: site-rows-derive-through-chart-pcurve-bypassing-the-routed-arm
kind: issue
title: topo::pcurves::site_rows derives through chart_pcurve directly, so a site mint on a routed (fitted) face clears its row instead of taking the route
status: open
opened: 2026-10-07
---


Found by a spline-carrier designer (PR 4261, fork-log row 85).

`topo::pcurves::site_rows` derives a face's rows through `chart_pcurve` directly, not through `analytic_derive`. So on a face whose class the mint routes elsewhere (today the sphere's general circle, through the fitted lane), a site mint clears the row rather than deriving it. It then relies on the producer's closing mint to restore it.

That is consistent with "doors may drop rows" as long as every public Euler door that reaches a routed face has a closing mint. Check that it does. If one has no closing mint, `site_rows` should go through the routed arm.

Under PR 4261's projected-image route, every such class has one derivation, which removes the asymmetry.
