---
id: coplanar-sphere-arm-cannot-tell-opposite-from-coincident-meridians
kind: issue
title: sphere()'s coplanar two-band arm decides props_band_coplanar Zero for OPPOSITE half-planes (Δu = π) and for COINCIDENT ones (Δu → 0 or 2π) alike, and answers Δu = π for both
status: open
opened: 2026-09-16
refs: [BOOL-5, 2748]
---

Found by BOOL-5's measurement 2 (PR 2748) and filed by the S-BOOL
orchestrator on PROPS's slate (`crates/geom-brep/src/props/curved.rs`
is PROPS's path; BOOL-5 took the rimless branch under a recorded seam
and did not touch the coplanar arm). The rimless two-band arm decides
whether the meridian axes are parallel (`props_band_coplanar`, the
cross-product norm levered at R) and then sets Δu = π. Parallel axes
cover two cases the cross product cannot separate: the meridians in
OPPOSITE half-planes (the hemisphere, Δu = π — the intended case) and
the meridians in the SAME half-plane (a slit, Δu → 0, or its
complement, Δu → 2π); d_A·d_B ≈ −1 versus +1 tells them apart and
nothing reads it. BOOL-5's wedge arm is reached only when the coplanar
decide is definitely nonzero, so it does not see the case either. No
revolve reaches it (a slit narrower than zero/R would have to
construct); an Euler-door or hand-built loop can. The PR body carries
the construction and the reachable window. The fix is one more decide
on the sign of d_A·d_B in the coplanar branch (a structural fact of
the two carriers, not a value coincidence) with a typed refusal for the
slit. Difficulty S.
