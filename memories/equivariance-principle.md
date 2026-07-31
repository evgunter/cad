---
name: equivariance-principle
description: Maintain isometry-equivariance (semantic, in ℝ) where it is free; the claim that the kernel currently has no designed asymmetry is unaudited
metadata:
  type: project
---

Evan (2026-07-30, during the S8 fillet-branch ruling): "everything
is equivariant right now, so maintain that if it's free (if that
is indeed true)."

Working principle: kernel constructions and selection rules should
commute with rigid motions AND reflections at the semantic level
(in ℝ) unless equivariance is provably impossible for the case or
costs something real. This is about DESIGNED rules (no left-hand
rules, no absolute-orientation tie-breaks) — not bitwise f64
equivariance, which D9's fixed evaluation orders already forgo.

**Why:** user geometry has no preferred handedness; a mirrored
design should behave as the mirror of the original.

**How to apply:** when specing a selection/tie-break/ordering rule,
prefer intrinsic quantities (arc lengths, distances, angles) over
enumeration/construction order; where a candidate-swapping symmetry
makes equivariance impossible, fall back deterministically and
DOCUMENT the residual (precedent: M5 S8's selection ladder, rung 3
— the first knowingly-designed residual). The "everything is
currently equivariant" premise is UNVERIFIED — an audit is banked,
not assumed; do not cite the kernel as equivariant in docs without
checking the claim at the site in question.
