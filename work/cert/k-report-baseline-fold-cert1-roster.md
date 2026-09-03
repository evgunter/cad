---
id: k-report-baseline-fold-cert1-roster
kind: issue
title: k-report: fold the CERT-1 roster changes into the next baseline re-derivation (props_meridian_pole; sphere props_rim_level margins)
status: open
opened: 2026-08-29
github: 1251
refs: [1220]
---

## From GitHub issue 1251

Opened 2026-08-29; 0 comments.

**Scheduling register for PR 1220's K-telemetry consequences** (S-CERT CERT-1), so the roster change has a place that executes instead of an "if the census flags it" hedge.

What moved, for the next K-REPORT runbook pass:

- **New recorded name `props_meridian_pole`** (`props/curved.rs`, `sphere_meridian_span_levels`): two samples per sphere meridian arc; margin = signed chord from the pole's span-relative direction to the nearer span end, × R. Its indeterminate outcome **folds rather than refusing** (the decide still records; PR 1220's body carries the continuity argument), so its in-band population is expected and benign — the baseline should not read in-band samples on this name as a landing.
- **Sphere `props_rim_level` / `props_rim_level_group` margins re-shaped**: the axial `|Δ sin v|·R` became the direction chord `2·sin(Δv/2)·R` (larger wherever rims are distinct), and rims sitting at their own extreme now record a rounding-scale second-component residual instead of bitwise 0 — a new near-zero cluster in those populations.
- `rim_dim_scale_twins.rs`'s sphere twin now pins the chord and the two-population shape (nothing in the ambiguity band).

Per the K-REPORT runbook this is the re-derive-the-baseline case, not a geometry change; the sampled k-lint axis had not drawn a fresh row between PR 1220's merge base and its head, so the first draw lands whenever the schedule next picks it up.

Refs: PR 1220, `docs/K-REPORT.md`, `docs/predicate-dimension-audit.md` (the `props_meridian_pole` row and retired note N7).

## Home

`work/cert/` — the roster that moved is `props/curved.rs`'s, inside S-CERT's `crates/geom-brep/src/props/*` territory, and the change is CERT-1's own consequence.
