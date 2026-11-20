---
id: a-stored-solved-parameter-is-an-enclosure-of-each-members-root
kind: issue
title: At an enclosure scalar a stored spline crossing is one checked number for the family, so a crossing that moves across the box refuses
status: open
opened: 2026-11-20
priority: P3
cost: H
refs: [from-computed-claims-a-zero-tangent-at-a-dual, a-solved-value-the-body-stores-is-admitted-by-its-check]
design: true
---

Filed by the SCALAR orchestrator, 2026-11-20, on Ev's ruling on PR 7176.

## What

A parameter a boolean splits a spline edge at is an `f64` candidate admitted by a check over the whole box (`crates/geom-core/README.md`, DL5.2). At `Interval` it is one number for the family. Each member's own crossing lies elsewhere by up to the band over the sine of the crossing angle; where it moves farther than that across the box, no one number passes and the site refuses (`spline_face::settle` returns `Unsettled::Meter`).

Ev, PR 7176: "if there's a boolean of two bodies with intervals of size > eps, surely in general they rarely meet at one place? i am glad that the current state A is already valid, though, and just restrictive. leaving it at A is definitely fine for now although B might be needed to support booleans later if my argument here is correct".

B is: at an enclosure scalar the lane builds the value, an enclosure of each member's root, as DESIGN `[scalars]` says of every other continuous value. Every `T` in an `Interval` body would then mean one thing; today a stored parameter is an enclosure on an analytic carrier (closed form) and a checked point on a spline.

## What is known

- Only `spline_face` holds a proven enclosure of the root today (`Isolated.roots`, each meeting's narrowed Krawczyk box, which `settle` drops). `composite::crossings`' `bisect_row` returns at the first midpoint that reads on, with its bracket still wide; the feet (`replace_face::carrier_foot`) have no bracket.
- Inferred, not run: at plain `Interval` a box-valued `t` makes `C(t) - p` in `certify_edge_spec` as wide as speed times the box, so a wide family would likely refuse at certification instead of at the door. At `Sym<Interval>` that difference is identically zero, so the gain may be real there only.
- Not measured: whether any document or corpus row drives a spline crossing over a box wide enough to refuse. The `Interval` rows in `spline_face/rows.rs` and `composite/rows.rs` lift `f64` points only.
- `editor-core::drive` subdivides a box on a refusal, so today's cost is leaves, down to boxes about the tolerance wide.

## The question

Measure first: a boolean over a toleranced spline body, box widths typical of a stack-up (far wider than the tolerance) — does it refuse, and how many leaves does `drive` spend? Then: what a stored parameter is at an enclosure scalar, and whether edge certification can hold a box-valued bound.
