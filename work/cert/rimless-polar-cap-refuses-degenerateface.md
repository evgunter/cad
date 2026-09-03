---
id: rimless-polar-cap-refuses-degenerateface
kind: issue
title: props: a rimless-boundary polar cap — one circular edge, no meridian — refuses DegenerateFace; its extent has no arc to derive from
status: open
opened: 2026-08-29
github: 1250
refs: [723, 1220]
---

## From GitHub issue 1250

opened 2026-08-29, 0 comments.

**Filed from CERT-1's fix pass (S-CERT), found independently by both blinded reviewers.** Deliberately not fixed in PR 1220, whose ground is the meridian-arc extent (issue 723) and the rim lever (issue 893); this is the sphere arm the span-derived fold cannot reach because there is no span to read.

## The input

A spherical cap face bounded by **one rim circle and nothing else** — `[0, 2π] × [v₀, π/2]`, the pole interior to the face, no meridian edge. Entirely ordinary STEP (a ball cut by one plane produces exactly this face) and an ordinary native construction.

## The disposition, executed (identical before and after PR 1220)

```
full cap: one rim circle, no meridian    REFUSE DegenerateFace
half cap: rim + pole-crossing arc        ACCEPT rel=-1.66e-16
```

(`r2_probe_sphere_polar.rs::probe_polar_cap_no_meridian`, committed on the PR 1220 branch.) The face's levels list holds only the rim's own latitude sine, so `min_max` gives `lo == hi` and `require_extent` refuses `DegenerateFace`. The half of the same cap bounded by a pole-crossing arc certifies exactly — so the whole is refused while its half is served, the same one-edge-flips-the-answer alarm shape issue 723 recorded for the split vertex.

## Why PR 1220's fold cannot see it

The span-derived extent (`sphere_meridian_pole` fold) reads pole latitudes out of **meridian arc spans**. A rim-only boundary has no meridian: the fact "the pole is interior to the face" is encoded in the loop's winding around the chart pole, not in any edge's parameter span. Serving it needs a different derivation — e.g. the rim's `d_u` traversal direction plus which pole the face's chart side contains (the material-side machinery already distinguishes this), setting the missing extreme to ±1.

## Classification

D2 addendum **row 2**: reachable by input, valid, lane unbuilt — the refusal is typed (`DegenerateFace`, arguably the wrong name for a face that is not degenerate; renaming or retyping belongs to whoever builds the lane). The honest serving alternative today is the certified-quadrature lane at the cost of a `pad > 0` enclosure.

## Sibling to check

The **cone apex cap** — a cone face bounded by one rim with the apex interior — has the same shape: no generator edge, extent from `min_max` over one level, `lo == hi`. Whether it reaches the same refusal (or is caught by `props_cone_nappe` first) was not measured here; whoever takes this issue should check it alongside.

Refs: issue 723 (the meridian-arc half of the extent premise), PR 1220 (CERT-1), the two reviewer probe branches `cert/1r1-probes` (`probe_full_polar_cap_disposition`) and `cert/1r2-probes`.

## Home

`work/cert/` — `crates/geom-brep/src/props/*` is S-CERT territory and the charter names the sphere polar acceptance defects; filed from CERT-1's fix pass.
