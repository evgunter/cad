---
id: a-refusals-edge-locator-pushes-it-past-the-word-budget-unmeasured
kind: issue
title: A blend refusal's edge locator pushes it past the 75-word budget, which the concision pin does not measure
status: open
opened: 2026-11-17
priority: P3
cost: E
---

Found in PR 7050's review (filed by the band-stripfru lane's fix pass).

`refusal_concision_chains::every_blend_detail_renders_within_the_budget` renders each blend detail through the feature tree's chain at `NodeErrorKind::Blend { edge: None }`. A refusal the viewer shows for a real request carries the edge locator. The reviewer measured the straight-link-too-short refusal (`junction::JUNCTION_FEET_CROSS`, an `UnsupportedRunOut`) at 87 words with it, over the 75-word budget, while the pin reads it within budget. PR 7050 shortens that detail; it reads 63 words without the locator.

The pin cannot see what the locator adds. Either the budget is meant for the text with its locator, and the pin should render with one, or the locator is outside the budget, and the budget's doc should say so.
