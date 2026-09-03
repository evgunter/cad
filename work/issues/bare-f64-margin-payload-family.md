---
id: bare-f64-margin-payload-family
kind: issue
title: error payloads - the bare-f64 margin family, NaN-as-hole and unnamed margins outside pcurve_cache
status: open
opened: 2026-08-23
github: 934
refs: [931, 925]
---

## From GitHub issue 934

opened 2026-08-23, 0 comments.

**The class #931 fixed in one file, and where else it lives.**

#931 (VERBS-SSIFLAT) established an invariant at one seam: *an error payload must carry the number it measured in a shape that says what the number IS, and must never manufacture a sentinel that collides with a real reading.* The concrete defect there was `ssi_refusal` projecting every classified margin onto one `f64` and reporting `NaN` for anything that was not a bare value — so at `T = Interval`, where a margin is always an enclosure, every honest escalation was indistinguishable from genuine poison.

That fix protects `pcurve_cache.rs`. Both reviewers of #931 enumerated the same shape elsewhere, and the sweep obligation is recorded here rather than left in a merged PR body. **This is a disclosed blind spot with a schedule, not a promise to fix now**: each row goes to whichever unit next opens its file.

## The family

| site | shape | why it is the same class |
|---|---|---|
| `crates/sweep/src/fillet/battery.rs:1057` | `let margin = match (n_a, n_b) { (Some(a), Some(b)) => a.cross(b).norm().lo(), _ => f64::NAN };` then `FilletError::ConvexitySignFlip { margin, .. }` | **NaN-as-hole.** "I could not read the outward normals" and "the cross product measured NaN" arrive as the same payload. Exactly #925's shape: a structural absence wearing a measurement's costume. |
| `crates/sweep/src/fillet/mod.rs` — **seven** `margin: f64` payload fields (`:326, :350, :368, :374, :384, :395, :555`) | bare `f64` named only `margin` | Anonymous: nothing at the type level says whether it is a classified margin, an enclosure endpoint already projected, or a certified clearance that is zero-when-unknown. A reader cannot tell a `0` that means "measured zero" from one that means "nothing certified". |
| `crates/geom-brep/src/edge_nurbs.rs:129-134` `PlaneNurbsRefusal::Limb { limb, value }` | `/// The measured bound, in meters.` | The **twin translation** of the same `SsiError` set that `pcurve_cache::ssi_refusal` performs. #931 gave its own translation named shapes (`FittedMagnitude::{LimbResidual, CertifiedClearance, LastFootDistance}`); this one still flattens. Note `edge_nurbs` is also where the good precedent lives — its `TubeStraddles` arm already renames the number `certified_clearance` and says why — so the file is half-converted, not wrong. |

## The invariant to apply

1. A payload that measured nothing carries `None` (or its own variant), never a sentinel value.
2. A number keeps a name that says what it is: a classified margin, a projected enclosure endpoint, and a certified clearance are three different things and only the first may be rendered as "the margin".
3. A classified margin travels with the band it was judged against — `geom_core::Indeterminate` carried whole, rendered through `IndeterminatePayload`. #931 is that renderer's first consumer; a second consumer costs nothing.

## What the #931 sweep could not match, stated

The grep was over `MarginDiag` mentions under `crates/*/src/`, which finds only sites that name the type. It found one. It does **not** find a site that flattens an `Indeterminate` without mentioning `MarginDiag`, nor one that stores a margin as a bare `f64` field before any diagnostic is built — which is precisely how every row in the table above hides. Those were found by two reviewers reading code, not by the pattern. **A future sweep of this class must grep for `margin: f64` payload FIELDS and for `f64::NAN` in non-test `src/`, not for the diagnostic type.**

## Home

The rows straddle `crates/sweep/src/fillet/*` (S-BLEND's, closed) and `crates/geom-brep/src/edge_nurbs.rs` (PCURVE's, closed), and no open program's territory covers either, so it lands under `work/issues/`.
