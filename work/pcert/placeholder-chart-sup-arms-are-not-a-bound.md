---
id: placeholder-chart-sup-arms-are-not-a-bound
kind: issue
title: chart_stretch_sup answers unit arms for a placeholder chart while chart_stretch_inf answers certifies-nothing
status: closed
opened: 2026-09-15
priority: P0
cost: H
branch: pcert/placeholder-chart-sup
pr: 3614
closed: 2026-10-01
---



## Finding

`crates/geom-brep/src/pcurve_cache.rs`, `chart_stretch_sup`: a NURBS
payload that `is_placeholder()` answers `(SupSpeed(1), SupSpeed(1))`.
A placeholder's control net is all-poison by construction
(`NurbsSurface::placeholder`), so every evaluation of it is poison and
there is no locus for an arm to be an arm of — `1` is not an upper
bound on anything, in the same sense the cone's `1` was not before
RATE-PAIR's fix pass made that door refuse.

The asymmetry is with its own sibling: `chart_stretch_inf` answers
all-zero for a placeholder and its comment says why — "the placeholder
certifies nothing". The sup side says the opposite shape of thing with
a number that looks like a claim.

Why it was not converted with the cone: refusing the placeholder turns
a today-inert number into a new refusal that `chart_arms_at`,
`trim_containment` and `v_meter` would all have to carry, and whether
a placeholder chart can reach those doors with a FINITE chart span
(the surface is poison, but a pcurve's chart coordinates are not) was
not established in that pass. That question is this row's work: either
a placeholder cannot reach a metred verdict, in which case say so and
cite the gate, or it can, in which case the door must refuse it.

## Home

Filed by SCALAR's RATE-PAIR lane in its fix pass, as the weaker half of
the reviewer finding that produced the cone refusal. TRIM owns
`crates/geom-brep/src/pcurve_cache.rs`.

## Closed (PR 3614, 2026-10-01)

A placeholder chart could reach a metred verdict: the public
`topo::chart_boundary` took its chart from the caller and walked a
placeholder's unit arms to an `Ok(ChartBound)` carrying NaN images.
`chart_stretch_sup` now refuses it (`NoChartSup::Placeholder`);
`topo`'s meters take a `DescribedChart` that cannot hold one, and
`chart_boundary` refuses it typed (`PcurveMintError::PlaceholderChart`).
"Is this the placeholder chart" has one home,
`geom::Surface::is_placeholder_chart`, which also catches an `Approx`
whose fit is the placeholder; the refusal text has one home,
`geom::PLACEHOLDER_SURFACE`. `weight_ratio_factor` answers poison for
weights knot insertion can write and `validate_counts` would refuse
(filed upstream on PROPS:
`surface-knot-insertion-writes-weights-validate-counts-refuses`).
Merged with one red inherited from main and annotated on the PR:
`sweep`'s `reach_volume_backstop` off the default ε (REACH's, caused by
PR 3611).
