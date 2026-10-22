---
id: narrow-span-refusals-do-not-say-which-span
kind: issue
title: geom-brep: the narrow-span refusals name no direction or span, so the lever points at nothing the user can find
status: open
opened: 2026-10-20
parent: offset-fit-and-patch-bound-kernel-defect-endings-split-by-class
priority: P4
cost: M
---


(Filed by the ENCL implementer of PR 5476 at its review's request.)

## What

The narrow-span refusals (`geom_brep::recourse::NARROW_SPAN`):
`PatchBoundError::RefinementFailed`, `PatchBoundError::DerivedKnots`,
and `OffsetFitError::Fit` at `TooFewPoints`/`ParamCountMismatch`. They
end in `NARROW_SPAN_RECOURSE` ("widen the face's narrow knot spans, or
move its parameter range nearer zero"), but carry neither the direction
nor the span's parameter bounds, so the user has to find the span
themselves.

## Why it was not done in PR 5476

It is not a change local to those variants:

- `PatchBoundError` is fieldless, `Copy + Eq` and `strum::EnumIter`
  (topo's sample roster).
- Its `note()` returns `&'static str`, which mesh's
  `TessellateError::UnsupportedNurbsFace { note: &'static str }` stores.
  Carrying `(direction, lo, hi)` makes `note` a `String` (or `Cow`) and
  changes that public mesh variant. mesh only calls the unrefined
  `patch_cells`, which never raises these arms.
- `OffsetFitError::Fit(FitError)` carries geom's carrier whole, and the
  fit does not know which pass (u or v) refused unless
  `interpolate_offset_grid` wraps it.

## Repair shape

- Give the two patch-bound arms a payload, a direction and the span's
  `[lo, hi]`, found at the raise site from the source vector's spans.
  The span is the first one whose equal-split points repeat, which is
  what `RingRefinement::equal_split` refuses on and `Second::of` reads.
  Then render it in `Display`, and split `note()` into the static
  subject the mesh field takes and the located sentence.
- Wrap the fit's pass in `interpolate_offset_grid` so the seed arm names
  its direction.
- Then the lever can say "widen the knot span u ∈ [lo, hi]".
