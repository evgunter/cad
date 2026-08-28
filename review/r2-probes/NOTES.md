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
