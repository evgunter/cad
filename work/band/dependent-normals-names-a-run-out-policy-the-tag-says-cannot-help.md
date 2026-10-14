---
id: dependent-normals-names-a-run-out-policy-the-tag-says-cannot-help
kind: issue
title: blend: CornerConfig::DependentNormals names RunOutStopAtVertex, whose corner patch its own doc says the configuration does not determine
status: open
opened: 2026-10-01
---


(BAND implementer, from `band/recourse-tables-decide-per-tag`.)

## What

`CornerConfig::policy` (`crates/sweep/src/blend/mod.rs`) maps
`DependentNormals` to `RunOutPolicy::RunOutStopAtVertex`, and
`UnsupportedCorner`'s `Display` renders it as "a corner of the chain is a
trihedron with dependent support normals, which only a run-out policy
would handle (…stop at vertex…)". But `RunOutStopAtVertex` is "the blend
runs at full radius all the way to the vertex and a corner patch fills
the junction", and the tag's own doc says that at a dependent trihedron
"the corner's three distance conditions do not determine a centre, and
its three trimline crossings do not determine a patch". So the policy
the refusal names is one the configuration rules out.

Which policy (if any) a dependent trihedron names is corner-taxonomy
ground (OQ6), so it was not changed with the recourse; this unit made
the match exhaustive and left the arm where it was. `m5_pr12_refusals::
corner_tag_dependent_normals_refuses_definitely` pins the current
answer.
