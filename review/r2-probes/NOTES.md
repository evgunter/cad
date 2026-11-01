# R2 probe lane (VERBS-LILYWELD PR-1, frozen head a1aa5289)

Mutation patches, each applied to the frozen head for one unique-signal
run and reverted; results in the R2 report.

- probeA-archr.patch — ARCH_R 0.052 -> 0.050: is the weld-circle
  exactness derivation-robust, and do the derived quantities (globe
  centre, sepals, closed-form volume) move together?
- probeB-tangent-success.patch — the tangent-cone probe fed the
  AUTHORED 70 deg: the leg authors fine, so expect_err must panic
  (the banked-refusal pin reds on success).
- probeC-neck-foldin.patch — wall-7 measurement with the neck folded
  back in: the re-cut's claim is that this would measure a different
  thing.
- probeD-wall2-wrongpin.patch — wall 2's test pin flipped to
  Sphere x Torus: the pin must red on a payload that is not the
  measured one.

## Results (all on a1aa5289, release, lane-own target)

- Main suite: 12 passed / 0 failed; weld residuals (0.0, 0.0, 1.94e-16),
  runner-up 3.5042473984792544e-2; census (1,10,18,10); volume errors
  (1.2504e-2, 5.2822e-3) inside the pinned bands; wall payloads verbatim.
- Fresh sweep: 1330 rows / 1367954 triangles; tess-lint 0 findings; key
  set and first-five columns identical to the committed baseline.
- Probe A (ARCH_R 0.050): weld pin PASSES with residuals still
  (0.0, 0.0, 1.94e-16) — the exactness is derivation-robust, not a
  property of 0.052; sepal tangency PASSES (globe centre derived);
  SPHERE1_C pin REDS (center.x) — the literal pin catches the move.
- Probe B (tangent probe fed 70 deg): authors fine -> expect_err
  panics -> RED. The pin fires on success as claimed.
- Probe C (neck folded into wall-7 gap loop + AABB): still green, but
  min gap 0.4131 -> 0.1406 and AABB tightest -0.2453 -> -0.0107 — a
  different measurement, margins collapsed; the carrier split is load-
  bearing. NOTE: wall 7's comment (lily.rs:2102) still cites 0.29 for
  a gap the re-authored scene measures at 0.4131 — stale.
- Probe D (wall-2 test pin flipped to Sphere): RED (panic at 3525).
- eps stand-ins: census_g2_carrier band-edge RED at 1e-12 (the #1102
  red verbatim; #1108's fix is NOT an ancestor of a1aa5289), green at
  1e-6 (CI's drawn point).
