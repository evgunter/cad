---
id: sweep-tests-hold-quadrature-midpoints-to-fixed-tolerances
kind: issue
title: Sweep tests hold certified-quadrature volume midpoints to fixed tolerances, ignoring volume_pad
status: open
opened: 2026-10-06
---


`topo::mass_properties` certifies a body with a curved-cut face only as
`volume ± volume_pad`: the quadrature refines until its enclosure's mean
boundary displacement is under `QUAD_TARGET_LEN_FACTOR·ε`
(`geom-brep/src/props/quad.rs`, `cylinder_cut_face_rounds`), so the
midpoint's distance from the truth is an ε-row quantity, certified to
no better than the pad. A test that holds the midpoint to a fixed
tolerance passes at a row only while the midpoint lands near the truth
there.

That shape went red once: `one_segment_loop::a_split_through_the_seam_builds_as_the_two_arc_form_does`
held the midpoint to `1e-9` relative. It was off by 7.344e-6 at ε = 1e-6, with
a pad of 1.32e-3. Branch `cleave/one-seg-seam-eps` fixes it: its `close`
asserts `|volume − closed form| ≤ volume_pad + 1e-9·max(|want|, 1)`.

The siblings below pass at every row today. CI run 37532070140's
1e-6 and 1e-12 rows ran `all()`, and this was its only red. Each one
compares a body that received a nonzero `volume_pad`, without reading
the pad. I found them by running sweep's `all` binary at default ε with
`mass_properties` temporarily logging every test that got a nonzero
pad, then reading each such module that never names `volume_pad`:

- `crates/sweep/tests/rehome_rings_lune.rs`,
  `an_oblique_cut_carries_a_lune_bore_with_its_half`: `< 1e-8` absolute,
  sized by its own comment ("lands ~1.5e-9 off").
- `crates/sweep/tests/common/mod.rs`, `TILTED_CUT_WALL_VOLUME = 1e-4`
  (used by `pis_arc_capped_poses.rs`): sized against a measured
  2.2e-6 at ε = 1e-6, not against the pad.
- `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs`,
  `a_rod_rim_on_the_dome_across_its_seam_meridian_builds_every_op_undeclared`:
  `reach = 1e-9.max(tol.eps())`, which scales with ε but does not read the pad.
- `crates/sweep/tests/split_cylindrical_feature_box.rs`, `halves`:
  the halves' midpoints sum to the whole's within `1e-9`. That holds
  while both halves stop at the same round; it is not certified.
- `crates/sweep/tests/review_cleave_wrongarc.rs`, the `> 1e-4` half-volume
  check near `total - want_below`.
- `crates/sweep/tests/reach_split_gate_window.rs`: slack `2e-3 * whole`,
  generous but fixed.

These are not this shape: `lib_u3_sections.rs` (a bit pin) and
`s393_start_frame_door.rs` (body against body). `contfp_reads_arcs_on_their_carriers`
gets pads but asserts no volume.

Blind spot: only sweep's `all` binary was run, and only at default ε.
281 test files across the tree call `mass_properties` without naming
`volume_pad`, and a static grep cannot tell which of their bodies carry
a pad. The same instrumented run over topo's, editor-core's and the
other crates' binaries is the remaining sweep.
