# PR 4823 R2 review probes (frozen head 79953ad30c, base 34097ca6d6)

Probes, run on base and head and diffed:

- `crates/topo/tests/r2_touch_key_probes.rs` (debug,
  `cargo test -p topo --test all r2_touch_key_probes -- --nocapture`):
  a carried vertex-vertex record between distant corners; a chain of
  plain booleans that drops a corner kiss's record (outcome, keys, and
  tier 3' with each result's own records); `offset_planes_together`,
  `offset_charts_together`, `offset_surfaces_together`, `shell`,
  `shell_open` and `replace_face_offset` over a corner kiss.
- `crates/sweep/tests/join_pierce_runs_sweep.rs` (release, ignored):
  `r2_touch_dbl3_battery`, `r2_dbl3_face_battery` (written from the
  FLUSH row's description of PR 4207's review probe, not ported),
  `r2_four_pairs_t3p_kinds` (tier 3' error kinds of every four_pairs
  union that builds), `r2_blend_a_kiss`.
- `crates/editor-core/tests/intent_s2_c_world.rs`
  `r2_print_product_geometry` (release, ignored): every corpus
  product's counts and sorted vertex coordinates.

Measured: see the review report.
