---
id: topo-reads-angles-off-the-geometry-without-a-stated-reference
kind: issue
title: topo reads angles off the geometry with bare atan2 and compares them through reduce_periodic / periodic_branch; C4 says an angle becomes one only through a stated reference — audit each site onto Azimuth::near or a span-anchored read
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Filed by unit B1 of the `[ev]` PR 5142 answer (branch
`pcert/azimuth-placement`), which moved geom-brep's chart-image
derivation onto `geom_brep::Azimuth` (the principal read, placed in
the window through `geom_brep::Period::decide`'s marks) and left
topo's own reads for this audit, as the plan with the orchestrator
agreed.

## What

C4: "An angle read off the geometry becomes one only through a stated
reference." The derivation now does that, but topo still reads angles
with a bare `atan2` (or `stable_azimuth`) and then places or compares
them by hand. Censused at B1's step 0 (line numbers at
`pcert/whole-periods-cap`, `fbd2b6de5e`):

Placed or stored (a):
- `topo/src/chart.rs` `Chart::azimuth` / `u_of` (`:156`, `:193`–`:203`,
  `:353`), the torus `v_of` (`:204`, `:374`) and `rim_v` (`:268`–`:269`);
  read by `chart_iso.rs:65` (`TravKind::Meridian { u_raw }`),
  `coherence.rs:611` and `shell.rs:3601`/`:3638`;
- `topo/src/shell.rs:3606` `chart_read`'s unwrapped steps;
- `topo/src/chart_region.rs:1532` the frame rotation `δ` between two
  cylinder charts;
- `topo/src/chord_join.rs:1196` `SectionConic::param`, and ring
  re-homing (`:5348`), which since PCERT B1 reads through
  `geom_brep::Azimuth::principal` (topo's `stable_azimuth` is gone) and
  still lifts the read by hand with `periodic_branch`;
- `topo/src/boolean/rim_wedge.rs:531` `locus_param`;
- `topo/src/offset_axial.rs:2443` `param_on`, `:2687` `reauthor`,
  `:2857` `azimuth_turn`.

Compared against a span or window (b):
- `topo/src/ring_path.rs:464`, `:1191`, `:1351`, `:1364`, `:1568`;
- `topo/src/splitting/classify.rs:626`;
- `topo/src/boolean/circle_roots.rs:305`;
- `topo/src/boolean/rim_wedge.rs:874`;
- ~~`topo/src/boolean/sphere_region.rs:389`~~ fixed by RESTCURVE's
  pole-door PR (#6252): the door reads roots as directions, and a root
  against its arc at the nearer end, relative to that end's parameter;
- `topo/src/boolean/boxes.rs:1841`;
- `topo/src/boolean/section_cert.rs:1677`;
- `topo/src/shell.rs:2532`, `:4590`.

`geom::Curve3::param_near` (`geom/src/curves.rs:1630`, `:1645`,
`:1658`) is already an anchored read; its callers state their anchor.

## The question

Which of these reads a period-ambiguous angle that a reader then tells
apart by copy (each such site goes onto `Azimuth::near` with a stated
anchor, or through the marks), and which read a quantity that is never
ambiguous (a span in `(0, π]`, a short-way turn) and need nothing.
