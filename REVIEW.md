IN PROGRESS

# Review of PR #3962 (frozen head b60ea20c)

Notes so far:
- Claim 6 closed form checked by hand: L = 16 − 4 = 12; kite shoelace = 5/2;
  kite ∩ L = kite's y ≤ 2 half (5/4) less its x > 4 tip (1/12) = 7/6; 12 + 5/2 − 7/6 = 40/3. Holds.
- Sign of n̂₀·r̂₁ derived by hand from BoolSector's CCW start→end and `flankers`: fl[0] holds the
  common ray as `start`, fl[1] as `end`; wedge = CW sweep r̂₀→r̂₁ seen down the axis; n̂₀·r̂₁ = −sin α.
  Holds for planar flanks. Suspect: curved flank reps are chords, not tangents (probing).
- Probes: crates/sweep/tests/join_reflex_wedge_review_probes.rs (running).
- Own battery main(56345807) vs head: 22 194 lines, 0 BAD both; 2 508 ClassificationInvariant→SOUND,
  24 ClassificationInvariant→ResultInvalid(ScaffoldAtRest), 0 SOUND→refusal. θa=180 operand refuses
  CoplanarNeighbours (π unreachable). Curved flank: CurvedPierceUnsupported first, both trees.
- Mutants running.
