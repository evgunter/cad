---
id: path-fillet-door-validator-tangency-disagree
kind: unit
title: The PATHS .fillet(r) door builds tangent joints Profile::validate refuses as transversal, for turns from 1e-7 to 1e-4 rad
status: review
opened: 2026-09-04
branch: blend/10-fillet-stored-tangency
pr: 2497
---

## The witness

A line × line bend rounded by the path door:

    Open.at((0, 0)).angle(0).fillet(0.2).at(anchor).angle(theta)
        .line(1).line_to(Start)

with `anchor = (4 + 3 cos θ, 3 sin θ)`. Probe:
`crates/profile/tests/review_fillet_e2_probes.rs`.

| θ (rad) | path door | `Profile::validate` |
|---|---|---|
| 1e-9 | escalates (`path_corner_turn` in band) | — |
| 1e-7, 1e-6, 1e-5, 1e-4 | **builds** a 4-vertex loop | **refuses**: "joint 2 … is declared tangent, but the carriers definitely meet transversally — remove the declaration or make the tangency exact (the PATHS .fillet(r) door computes it)" |
| 1e-3 and up | builds | validates |

## What is wrong

The door and the validator disagree about the same joint across four
decades of turn angle, and the validator's own recourse names the door
that produced the joint as the way to make it exact. Whichever side is
right, the pair cannot both be: either the door mints an arc whose
stored representation (a fillet arc of length `0.2·θ ≤ 2e-5`) no
longer carries the tangent it computed, or the validator's tangency
test is too strict for a legitimately tiny arc. Not investigated here.

PR 1753 reported the 1e-6 instance as "may be a legitimate sliver
refusal". A single sliver would not span 1e-7 to 1e-4.

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/blend/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). The PATHS `.fillet(r)` door and `Profile::validate`'s tangency test are the profile fillet door, S-BOOL's glob by announced seam.

## Landed

**Two losses, not one.** Phase 1 measured the first and the dual review
found the second by execution.

1. **Flattening.** A profile holds an arc as a chord and a bulge, and
   `segment_straightness` classifies its sagitta `r(1 − cos(θ/2))`.
   Below ε the stored segment reads as a LINE and the carrier the door
   computed is not in the loop at all. Threshold `θ* = √(8ε/r)`.
   Refused as `PathError::FilletArcFlattenedInStorage`; levers: a larger
   turn, a larger radius while the scene still resolves one, or drop the
   fillet. At a tight ε and a tiny turn the radius lever's floor crosses
   the other loss's ceiling and NO radius works — the sentence says so
   rather than promising one.
2. **Scene resolution.** The stored form IS an arc and the classification
   is what fails: a carrier clearance is a difference of lengths at the
   scene's own magnitude, resolving only to about `scale · 2^-52`, so
   past the radius or the distance from the origin where that floor is
   coarser than ε the joint cannot be classified whatever the turn is.
   Refused as `PathError::FilletCarrierBelowSceneResolution`; levers run
   the OTHER way — a smaller radius, the geometry nearer the origin, or
   drop the fillet. Two variants and not one because the levers are
   opposite and `PathErrorKind` is the discriminant a caller branches on.

The in-band twin of either is relayed as `PathError::Escalated` naming
the same classification, with the stored-form site and a sentence that
names both checks and the lever that settles either.

**What the check does.** At the loop's close it re-reads every fillet arc
the chain emitted through `seg::build_seg` and `seg::joint_tangency` —
the verify layer's own classifications, so no new predicate name enters
the K stream — and refuses what that reading refuses. Its cost is 9 / 11
or 12 / 15 decisions per fillet (line × line / line × arc / arc × arc),
guarded by a row.

**What it does not cover**, each with its own reason recorded at the
skip: joints the door did not DECLARE (the exact outgoing fit, where the
direction leaving the fillet is free — there is no second carrier for the
joint to be tangent to, and `UndeclaredTangency` is a claim about the
declaration set rather than about the stored form); a neighbour the
segment pass itself refuses on its own terms; and the pair pass, which
can still refuse a loop with a small-turn fillet in it because the
fillet's own LEGS are close together — that is
`work/blend/arc-arc-shallow-corner-legs-escalate-arc-span.md`, a class
with three recorded instances across `arc_span` and `line_span`.
