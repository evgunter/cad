---
id: shell-of-a-pole-touching-sphere-refuses-mapped-source
kind: issue
title: shell of a POLE-TOUCHING sphere refuses Certification(ResidualExceeded MappedSource) where the two-arc sphere shells
status: open
opened: 2026-09-09
---


Found while surveying demo coverage for a shell cell. SHELL-9's row
shells a sphere authored as **two cocircular arcs**; the pole-touching
one-arc ball — the shape `revolve_ball` mints and the shape a demo
would reach for — does not shell, and the refusal is not the one
SHELL-9 fixed.

## Measured

`Tol::witness()`, main at this branch's base.

| body | meridian | `topo::shell(&b, 0.1, tol)` |
|---|---|---|
| unit ball | ONE semicircular arc `(0,-1) → (1,0) → (0,1)`, pole-touching, no bore | `Face { face: FaceKey(1v1), error: Op { edge: Some(EdgeKey(2v1)), error: Certification { error: ResidualExceeded { check: MappedSource, sample: 1 } } } }` |
| bored sphere | two arcs on one circle plus an on-axis bore wall (annular, so every latitude rim is one closed edge) | **Ok**, volume `1.3808925058780275` |

Both are full revolves at the same ε; the difference is the meridian.

## Why it is a row and not a frontier note

SHELL-9 (#2223) landed the closing pcurve mint precisely so that the
sphere row flips to its closed form, and its item states the case it
measured: *"Measured on a sphere authored as two cocircular arcs …
`topo::mint_pcurves` on the assembled body makes it tier-3 valid at
`4/3·π(r−t)³`."* The pole-touching ball is the sibling shape — same
surface, one chart, two half-bands meeting at the seam meridian AND at
two poles — and it stops at a different door: a `MappedSource`
residual on the graft re-certification, not the `LoopDiscontinuity`
the closing mint retired.

So one of two things is true, and nothing in tree says which:

1. the pole-touching class is genuinely outside the offset lane's
   reach (a pole is a chart singularity, and the offset of a face
   whose boundary reaches it may have no `MappedSource` image), in
   which case the refusal wants to be typed as that rather than as a
   certification residual, and `shell`'s docs should name the class; or
2. it is the same repair SHELL-9 made, one graft further along.

## What the taker owes

One row either way: a fixture that shells the pole-touching ball and
either asserts the closed form `4/3·π(r³−(r−t)³)` or pins the typed
refusal with the class named in prose. Today the sphere reads as
"shells" from SHELL-9's row alone, and the first consumer to author a
plain ball finds otherwise.

Refs SHELL-9 (#2223).
