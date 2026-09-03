---
id: TCOST-K1
kind: unit
title: the patch-flux lanes' exhausted-budget cost (kernel, A/B track)
status: spec
opened: 2026-09-03
---

Spec: `docs/TCOST-K1-SPEC.md`. Cut from TCOST-2's kernel finding:
`nurbs_patch_face` costs 22–33 s per call when it exhausts its round
budget against 3–5 s when it certifies or refuses early, and the
rational lane costs ~90× the integral lane on the same face. Runs on
the A/B track, not the test-only track.
