---
id: sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts
kind: issue
title: inverting every face's sense on an arc-capped loft leaves tier 3 green with an unchanged positive enclosure
status: open
opened: 2026-09-12
---


## The finding

Found by a PERF-6 reviewer while probing the +V check, and OUTSIDE
that unit's fence: it is a fact about what `sense` means on an
arc-capped loft, not about when check 7 stops.

`Body::flipped_face_sense_for_tests` applied to EVERY face of the
three-station `arc_section` loft (the fixture
`crates/sweep/tests/reporting_door_bit_digest.rs` calls `arc_prism`)
leaves tier 3 GREEN, and leaves the body's volume enclosure unchanged
and positive. The same whole-body inversion applied to
`square_prism` or to `step-export`'s `loft_prism` refuses
`LoopRoleInverted`.

So on a body whose caps are bounded by arcs, the stored sense bit can
be inverted on every face at once with no tier-3 consequence and no
change to the quantity check 7 reads. Either the bit means something
those bodies' faces do not carry, or it means something tier 3 does
not check on them.

## Why it is not a check-7 bug

The +V invariant reads a volume ENCLOSURE, and the enclosure did not
move: the quadrature lane's Green form is winding-derived end to end
(the signed UV area IS `s_f·|Ω|` through the stored loop traversal),
which `geom-brep`'s `props::quad` module docs state as a deliberate
property — no sense bit enters it. Flipping the bit therefore cannot
change the flux, and check 7 is reading the same body it was.

What is unexplained is the OTHER half: why the same inversion reaches
`LoopRoleInverted` on the polygonal lofts and not here, and what a
consumer is entitled to conclude from a sense bit that no at-rest
check on these bodies reads.

## What to measure before anything moves

1. Which tier-1/2/3 pass produces `LoopRoleInverted` on the polygonal
   lofts, and what about the arc-capped body's loops makes it silent —
   a rim role derived rather than stored, an arc cap whose role is not
   computed, or a pass gated on something the arc body fails earlier.
2. Whether any at-rest check on an arc-capped body reads
   `face.sense_sign()` at all.
3. Whether a body built inverted (as opposed to flipped after the
   fact) is reachable through the public API, which is what decides
   whether this is a test-door artefact or a real gap.
