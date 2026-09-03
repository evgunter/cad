---
id: m8-4-interior-column-row-vacuous
kind: issue
title: m8_4_intersection_iso's interior-column row asserts nothing at eps=1e-12 (and is the row P-2 must flip)
status: open
opened: 2026-08-29
github: 1167
refs: [498]
---

## From GitHub issue 1167

Opened 2026-08-29; 0 comments.

## What

`crates/sweep/tests/m8_4_intersection_iso.rs::an_interior_column_intersection_refuses_typed` (~line 405) **asserts nothing at ε = 1e-12**, and CI draws one ε per run.

Measured: at `CAD_TOLERANCE_EPS=1e-12` the fixture's seam does not attach at all —

```
Escalated{ PlaneNurbsCertificate, margin 6.217e-12, band.zero 1e-12, "ssi_hull_sup_chart" }
```

`seam_at_eps` turns that into `None`, and the test `return`s early having exercised nothing. It passes, silently, over a body that was never built.

## Why this one matters more than most

**It is the row a scheduled unit must flip.** PCURVE P-2 (#498's home) exists to make interior-column `Intersection` carriers mint instead of refuse; this row is *the only row in the tree* pinning that refusal. So the negative control for that unit is already vacuous at one of the three ε draws, before anyone touches it.

**It also has the weaker of the two vacuity shapes even when it does run.** `posture()` accepts `Refused` **or** `Escalated`; only `Certified` panics. So its teeth today are "does not mint" — which is satisfied by the fixture failing to build for an unrelated reason.

## This is the census-gap-2 shape, again

`docs/PCURVE-LOG.md` records the incident: a row whose subject was the ε band was merged on a CI draw that never exercised the band, and main went red at 1e-12 within the day. The rule stated then — **a stated coverage gap is a blocker when the untested axis is the row's own subject** — applies here with the axis inverted: this row does not go red at 1e-12, it goes *quiet*, which is worse because nothing tells you.

Related: `m5_pr9_sector2` (found during P-1b) was about to pass vacuously for a different reason — its walk selected on a retired variant, counting 0 rows against 2 asserted. Three instances now; the shape is a row that stops testing without failing.

## What would fix it

Either an operating point that is not ε-conditional, or an explicit three-cell ε table so the row states what it does at each draw. Whichever P-2 takes, its re-expression must assert a **definite** outcome — a `General` cache whose image is the interior column with a certificate envelope ≤ ε — rather than inheriting `posture()`'s Refused-or-Escalated tolerance.

Filed separately from P-2 because it is true on `main` today and independent of whether that unit is ever built.

## Provenance

Measured by the P-2 substrate survey, in a lane, on `main`. Not a code change — the substrate changed nothing.

## Home

A test-integrity finding on a PCURVE row; both PCURVE and S-QA are closed and may hold only closed items, and S-TCOST's charter is suite cost rather than vacuity, so it lands under `work/issues/`.
