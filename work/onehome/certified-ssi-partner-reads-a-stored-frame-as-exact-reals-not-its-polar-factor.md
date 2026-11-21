---
id: certified-ssi-partner-reads-a-stored-frame-as-exact-reals-not-its-polar-factor
kind: issue
title: Three certified SSI readers read a stored f64 frame's entries as exact reals, where the Interval lift reads its polar factor: one kernel, two truths of one carrier
status: closed
opened: 2026-11-21
priority: P1
cost: M
closed: 2026-11-21
pr: 7131
branch: onehome/carrier-frames
---


## What

Under decision 1 of PR 7131 (Ev, 2026-11-20) a frame stored at `f64` a
rounding off rigid is read at `Interval` as the enclosure of its polar
factor (`RigidMotion::map_scalar` → `Real::enclose_rotation`), so the
carrier a certification reads is the carrier on that rotation. Three
production readers instead lift the stored entries as points and
certify on them, so they read a different shape:

- `crates/geom-brep/src/ssi/residual.rs:174-180`, `ImplicitForm::of`:
  the stored origin, axis and `u_ref` as `Interval::point`, then
  Gram–Schmidt at `Interval` (`e₂ = axis/‖axis‖`). It is the partner
  form of every spline × {plane, cylinder, sphere, torus} section:
  `ImplicitSection::of` / `within` (`ssi/implicit_chart.rs:246`), the
  plane trace (`ssi.rs:3760`) and the curved pair arm
  (`pair_section/spline_implicit/curved.rs:89`), whose hull `Apart`
  (line 91) returns an empty section nothing re-certifies. The limb-3
  certification of the same partner reads the polar factor
  (`ssi/certify.rs` `chart_partner_form`), so one section mixes the two.
- `crates/geom-brep/src/pair_section/spline_implicit.rs:435-446`,
  `SplineImplicitPose::pivot`: the certified `lift` over the stored `o`
  and `m` as points.
- `crates/geom-brep/src/ssi/section.rs:171-173`, `Pieces::of` (through
  `boundary_section`): the stored origin and normal as `Interval::point`,
  fed by `crates/topo/src/offset_derive.rs:1448` (`plane_spline_root`)
  and `:1342` (`level_row`); certified root brackets and On/Graze on the
  stored-entry plane.

The two truths differ by a turn of a few u (the polar `w` against
`w/‖w‖`: about ½(G_uw·u + G_vw·v)), about 1e-12 m at 10 km, which a
near-graze root amplifies: ONEROUND measured the two curves' roots
3.7e-10 m apart at a 1 km radius and 3.6e-9 m at 10 km
(`work/onehome/log.md`, 2026-11-21 07:30Z and 08:02Z). On main every
reader lifts entry-wise, so main reads one truth; the split is PR 7131's.

The test oracles judge the `Interval` doors against the stored-entries
truth too: `conic_oracle/exact.rs` `truth` (double-double on the stored
entries, used by the in-band, shallow, ellipse and ellipse-torus
sweeps) and the `carrier_resolution` tests. The gap is negligible at
their unit scale.

## Fix

Read each partner's frame through `Frame::map_scalar(Interval::from_f64)`
and take its columns (`ImplicitForm::with_frame` already takes a frame
enclosure); `boundary_section` takes the `Frame` and lifts it the same
way. Pinned widths and refusals on the SSI rows move by about an ulp, so
each moved pin is re-measured. The oracles take their axes from the polar
factor (its enclosure's midpoints, or the polar correction carried in
double-double).

Also audit ONEROUND's chord readings when they land:
`oneround/anchored-conic` `conic.rs:389-399` enters the frame's centre,
`u_ref` and `n × u` exactly.

## Fixed by #7131

The three readers take one spelling, `geom-brep` `ssi/residual.rs`
`certified_frame`: the stored frame's origin and the columns of its polar
enclosure (`Frame::map_scalar` to `Interval`). `ImplicitForm::of` reads
it in place of the point lift and Gram–Schmidt; `boundary_section` takes
the plane's `Frame` (its two `topo` `offset_derive.rs` callers pass it);
the pivot's lift is `spline_implicit.rs` `stored_height`. Each site has a
row against a double-double polar truth (`residual::polar_truth`): 4,096
tilted frames out to 10 km, a near-graze point 10 to 20 km out along each
seam. `ssi::residual::a_partner_form_reads_a_stored_frame_on_its_polar_factor`,
`ssi::section::the_boundary_section_reads_its_plane_on_the_polar_factor`
and `pair_section::spline_implicit::a_pivots_lift_reads_its_stored_plane_on_the_polar_factor`
go red under the stored entries lifted as points and squared at the
scalar (the old lifts), and under them read raw.

The test oracles (`conic_oracle/exact.rs` `truth`, `carrier_resolution`)
still read the stored entries; at their unit scale the gap is under their
tolerances, and none of their rows moved.

**ONEROUND's chord reader needs the same check when it lands:**
`oneround/anchored-conic` `conic.rs:389-399` enters the frame's centre,
`u_ref` and `n × u` exactly, the stored entries as reals.

