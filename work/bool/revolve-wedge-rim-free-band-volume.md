---
id: revolve-wedge-rim-free-band-volume
kind: issue
title: Revolve wedge mass-props — a rim-free band with non-coplanar meridians is VolumeUncomputable (the natural ball's wedge)
status: open
opened: 2026-08-16
github: 542
refs: [530, BOOL-5]
---

## From GitHub issue 542

Opened 2026-08-16; 0 comments.

Surfaced by M9-D1 (#530) review NOTE-1, independently confirmed by
both reviewers. **Pre-existing gap, newly reachable — not a naming
defect and not caused by that PR.**

## The class

A revolve band face whose boundary has NO latitude rims and whose two
meridians are not coplanar makes tier-3 mass properties refuse:
`props_band_coplanar` fails and `sphere()`
(`crates/geom-brep/src/props/curved.rs`) answers
`VolumeUncomputable`. The refusal is typed and loud; nothing is
guessed.

The shape that hits it is the PARTIAL revolve of an all-on-axis
meridian — the natural ball's wedge. Its single band face runs
pole to pole, so there are no rims to feed `du_of_rims`, and its two
meridian arcs lie in different half-planes.

## Why it is newly reachable

Before #530 no all-on-axis loop could be named at all, so the only
authorable ball wedge was the two-quarter-arc one, whose equator rims
made the same volume computable. #530 retires that refusal, so the
natural wedge now builds, names and validates end to end — tiers 1-2
pass, tier 3 declines the volume. The pre-existing scope limit simply
became visible.

## Flip condition

The issue closes when a band face with no rims and non-coplanar
meridians reports a volume — i.e. when `sphere()`'s spherical-wedge
arm can integrate over the meridian pair directly instead of
requiring the rim parametrization (or when an equivalent surface
term is supplied for the rim-free case). At that point the two probe
sites that today assert tiers 1-2 only can assert tier 3, and the
comments pointing here come out.

## Where it is pinned today

`crates/sweep/tests/m9_d1_r2_probes.rs` —
`partial_wedge_pole_export_is_direction_safe_both_signs` and
`partial_with_hole_exports_outer_poles_and_no_hole_poles` both
validate tiers 1-2 only and say why, referencing this issue.

Scope note: naming acceptance is unaffected — every entity of the
wedge is named and `check_total` is satisfied. Only the mass-props
answer is missing.

(M9 orchestrator program)

## Home

S-BOOL: the fix is already scheduled there as the unit `BOOL-5` (the rim-free spherical-wedge props arm), which edits `props/curved.rs` by the recorded Track-R seam.
