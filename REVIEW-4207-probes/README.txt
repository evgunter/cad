PR 4207 review probes (not compiled here).
- r1_4139_probes_r4207.rs: PR 4139 r1's example (branch join/pinch-one-vertex-per-cone-review-r1)
  plus sets dbl3, two, touch, r2p (exact-counter replay of r2's pinched operand) and the
  R4_KEYS column; drop into crates/sweep/examples/r1_4139_probes.rs.
- zip-mutants.patch: env-gated mutants (R4_SHARE=skip|largest, R4_CLASS=first|multi) and R4_DIST.
- run.sh / mut.sh: the battery and mutant drivers.
