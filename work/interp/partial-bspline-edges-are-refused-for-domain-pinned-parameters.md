---
id: partial-bspline-edges-are-refused-for-domain-pinned-parameters
kind: issue
title: step-import: a B-spline edge's parameters are pinned to its knot domain's ends, so a legal edge with interior vertices is refused as a damaged endpoint
status: open
opened: 2026-10-20
priority: P3
cost: M
---


(Filed by the ENCL implementer of PR 5454, `encl/one-culprit-chooser`, from its review.)

## What

`geometry::endpoint_params` (`crates/step-import/src/geometry.rs`, ~142-155) gives every B-spline edge the whole knot domain, `(a, b)`. Its comment says a trimmed B-spline edge "would need parameter inversion".

STEP allows an EDGE_CURVE whose vertices are interior points of its B-spline. Such an edge is legal. Its endpoint residual (`CertCheck::EndpointStart`/`EndpointEnd`) measures each vertex against the curve's domain end rather than its own point on the curve. So the edge is refused as a miss, and past ε_in that miss reads as a defect.

## Not the file's to answer

The import door cannot honestly blame the file for this miss: the parameter it reads is pinned by the kernel, not stored by the file. At the import door `recourse::Provenance` reads every unsized check as `Derived`, so the refusal names the kernel or the file (PR 5454). A refusal that is honest about the capability gap would say the edge is not yet supported (`NOT_YET_ENDING`).

## Repair shape

Invert each vertex onto the B-spline (a certified foot-point) to get the edge's parameters. Until that exists, refuse a B-spline edge whose vertices are not its domain ends as not yet supported, before certification reads it as a miss.

Note (PR 5464, 2026-10-20): Ev ruled that the import door names the file for every defect ending a bad file reaches, stored or derived, and `recourse::Provenance` is gone. Past ε_in this refusal now ends in `DAMAGED_FILE_ENDING` ("the file is damaged"), so a sound file refused here is told it is damaged: the defect now also makes the message false, which raises its cost. The paragraph above describes the state before that PR. (ENCL implementer)
