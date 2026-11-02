---
id: a-seamless-band-mints-and-passes-tier-3-though-loopwraps-says-no-constructor-makes-one
kind: issue
title: A seamless band mints and passes tier 3 through public Euler doors, though LoopWraps says no constructor produces one and ring_path leans on the mint refusing it
status: open
opened: 2026-10-12
---



## What

`crates/topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs`
(FLUX, `flux/cylinder-band-quad`) builds a seamless pipe through public
Euler doors alone (`mvfs`, `set_face_surface`, `mef`, `mev_line`,
`kemr`, `set_face_surfaces_describing`, `set_edge_curve`): one
cylinder face whose two loops each wind the cylinder once. Measured:
`topo::mint_pcurves` succeeds, `validate_closed` and
`validate_geometric` are `Ok(())`, and `mass_properties` measures it.

Two pieces of text say this body cannot exist:

- `topo::pcurves::PcurveMintError::LoopWraps`' Display
  (`crates/topo/src/pcurves.rs`, the `Self::LoopWraps` arm): "no
  constructor is known to produce this, so report the body that
  reached it". `chart_boundary` returns it on a seamless band
  (`boolean::section_cert_rows`'
  `w2_refuses_a_seamless_band_and_clears_a_banded_wall` pins it), and
  the public doors above produce one. (`validate.rs`' reading of the
  same variant already says "cannot yet map", the honest one.)
- `topo::ring_path`'s module docs argue the parity of a path is
  path-independent because "tier 3's pcurve mint refuses a face whose
  outer loop spans a full period or whose loop wraps
  (`PcurveMintError::{OuterSpansPeriod, LoopWraps}`), so a face that
  reaches round the axis is cut by its seam" — on a cone, and on a
  torus for a loop whose walk closes only by a period. On a cylinder
  the mint accepts the band (measured above). Unmeasured: whether a
  seamless cone frustum or torus band built the same way is also
  accepted. If it is, the premise the parity argument rests on is
  false for a body the public doors make.

## The shape of a fix

Measure the cone and torus cases first. Then either the mint refuses
a wrapped loop where `ring_path` needs it to (and the Display stays,
re-worded to name the doors that reach it), or `ring_path` decides
its premise rather than citing the mint.

## Re-read (2026-10-26, tracker sweep, main `bb25fa4f`)

The first bullet's text is gone: `PcurveMintError` has no `LoopWraps`
variant now (no `LoopWraps` anywhere in `crates/`). The second bullet
stands in a changed form: `topo::ring_path`'s module docs
(`crates/topo/src/ring_path.rs:41-55`) no longer cite the mint's
refusal. They now say "the at-rest pass does not yet refuse a loop that
winds", cite this row, and still rest the path-independence of a
crossing's parity on "no loop of the face winds". So the owed work is
the second half of "The shape of a fix": either a winding loop refuses
where `ring_path` needs it to, or `ring_path` decides its premise.

## Evidence (2026-10-31, SECT `sect/rehome-by-path`, PR 6047)

A cylinder face re-homes its rings by the parity of a path across the
run (`chord_join::path_ring_side`). That reads membership only where
the run bounds a disc of the face's chart: on a seamless band whose
run winds round the axis, which side is the new face's would be read
off the run's orientation, which the parity never sees
(`crate::ring_path`'s module docs). A constructor that makes such a
band needs that reading, or a typed refusal there, first.

