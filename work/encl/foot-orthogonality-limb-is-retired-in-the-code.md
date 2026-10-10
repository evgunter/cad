---
id: foot-orthogonality-limb-is-retired-in-the-code
kind: issue
title: geom-brep: retire the C2 foot-point orthogonality limb in code (Ev, PR 4498)
status: review
branch: encl/retire-foot-orthogonality
pr: 4517
opened: 2026-10-10
priority: P3
cost: M
---



(Ruled by Ev on 2026-10-10 in PR 4498, after design-fork row 104.)

## What

C2 limb 1 no longer bands the foot's orthogonality residual; the README text landed in PR 4498. The code still has the limb:
- `ssi_foot_orthogonality` in `crates/geom-brep/src/ssi/certify.rs` `nurbs_limbs`, and `SsiLimb::FootOrthogonality`;
- its check row;
- the `projection.rs` module doc's "band them together" three-residual story, including the false "clamped domain-edge foot: small distance, large orthogonality";
- `SsiLimb::OnLocus`'s doc ("including the foot-point orthogonality check");
- `PlaneNurbsRefusal::Limb`'s Display;
- `ssi/refine.rs` `RoundMargin::Over`'s reading of it.

Its margin `|S_u·r|/|S_u| ≤ |r|` (same `r`, same `(u,v)` as the distance limb) cannot refuse anything limb 1 passed. It can only refuse on interval slop.

## Repair shape

- Delete the limb and its decision.
- Reword the docs to the README's limb 1.
- Check whether `SurfaceProjection`'s `orthogonality_*` fields still have a production reader, and delete them if not.
- Any test that plants a corruption tripping this limb alone tests an unreachable refusal and goes with it.
- The k-stream loses only `ssi_foot_orthogonality` rows; nothing else moves.

Coordinate with `a-certified-bound-refusal-reads-as-a-stored-contradiction`, which touches the same file: land after it, or merge it in first.
