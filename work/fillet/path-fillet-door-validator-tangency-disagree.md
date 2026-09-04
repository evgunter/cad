---
id: path-fillet-door-validator-tangency-disagree
kind: issue
title: The PATHS .fillet(r) door builds tangent joints Profile::validate refuses as transversal, for turns from 1e-7 to 1e-4 rad
status: open
opened: 2026-09-04
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
