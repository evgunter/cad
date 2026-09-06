---
id: tcost-area-pad-lever
kind: unit
title: The area pad: the next lever on what a refused patch face still pays
status: open
opened: 2026-09-03
---

TCOST-K1 (PR 1652) stopped the patch lanes from running a schedule
they can prove cannot certify: a refused face now pays round 0 (64
cells) and the last-round bound. What it still pays is everything
BEFORE the rounds — the area pass (`area_midpoint_taylor` at
`QUAD2_AREA_CELLS`), the hull blocks and the chord perimeter — which
is why an early refusal still reads 3–6 s per face on a loaded box
(K1's suite prints it) against ~0.5 s for the integral lane's exit.
The header of `rational_patch_face` names the next levers in order
(a higher-order rule; a tighter AREA pad — the symmetric Lipschitz pad
is what puts the extreme-weight rows out of reach, and the area pass
is now the largest fixed cost of a refusal; more hull blocks); the
area pad is the one this item owns. Spec first, measured (a per-phase
trace of one refused face: area pass, hulls, perimeter, round 0),
under the K1 constraints: D9, bit-identical certified brackets (the
two-build digest), refusal classes kept.
