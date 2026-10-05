---
id: torn-hops-read-as-absent-in-the-split-the-chord-join-and-the-reach-rules
kind: issue
title: The split, the chord join and census's reach rules read a torn link as absent or as scaffolding
status: open
opened: 2026-10-05
priority: P3
cost: M
refs: [torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate, torn-hops-read-as-absent-across-the-boolean]
---

## What

The same sweep as `torn-hops-read-as-absent-across-the-boolean`, on
the split and the chord join, plus the reach rules both gates read:

- `splitting/join.rs` `fixed_partners` (a torn surface takes the conic
  arm), `certify_section_area` (a torn curve: `continue`),
  `refuse_section_spur` (`matches!(.., Some(Certified(Line)))` reads a
  torn curve as not a line);
- `splitting/finish.rs` `describe_section_boundary` and
  `splitting/section_loops.rs` `loop_edges` (a torn curve reads as no
  certified curve / `Undecided`);
- `splitting/classify.rs` `sphere_zone_reach`: `props::loop_edges(..).ok()`
  and `sphere_chart_trim(..).ok()??` swallow a torn loop's typed
  refusal into "no zone";
- `chord_join.rs` `chord_spec` (a torn wall surface takes the curved
  lane), `between_edge_is_section` (a torn curve answers `Ok(Some(true))`
  as null scaffolding), `run_azimuth_images` (`continue`), and
  `along_edge_spec`'s curve read (a torn curve refuses as "carries no
  certified curve");
- `census.rs` `face_reach_in`, `boundary_reach` and `edge_reach_in`:
  `?` on the face's surface, its loops, the walk, half-edges, vertices
  and points, so a torn boundary reads as "no claim". The split gate's
  `gate_face_reach` and the REST lane's `face_ball` resolve the face and
  its surface themselves now; the boundary walk inside is this row's.
  A null curve's `None` there is a legitimate "no box" and stays.

## Direction

Per site, as the closing PR of
`torn-records-read-as-absent-in-the-rest-lane-and-the-split-gate`: a
`CurveGeom::NullScaffold` arm keeps its own answer, and the record miss
goes through `edge_curve_linked` / `face_surface_linked` / `linked`,
with the at-rest or mid-operation premise stated at the site.


## 2026-10-05 — what the boolean folds a chord-join refusal into

`boolean/solid_contain.rs` reads `chord_join::face_azimuth_window` and
`face_azimuth_images` through `.ok()` into `CorruptFace`, and through
`Err(_) => unsupported()` (`wall_outline`) or `Err(_) => Ok(None)` into
the honest remainder. So a torn hop inside those walks reaches the
boolean today as a corrupt face or as "no class". Once this row's
chord-join hops panic, those arms fold only the geometric refusals.
Measured by the boolean record-hop unit: with `wall_outline`'s own
curve read restored to its old reading, its torn-curve witness still
panics, because `face_azimuth_images` reads the curve first.
