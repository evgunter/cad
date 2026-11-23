---
id: an-edge-between-two-records-of-one-surface-one-record-does-not-join-refuses
kind: issue
title: step-import: an edge between two records of one surface that one_record does not join (a spline or spindle record, two records facing opposite ways) refuses at the build
status: open
opened: 2026-11-19
priority: P3
---


Found by the dual review of `importgate/snap` (PR #7135).

## What

`crates/step-import/src/build.rs`'s `served_pair` refuses an edge whose
two surfaces the pair section finds coincident
(`geom_brep::SectionError::CoincidentSurfaces`) not-yet-supported,
naming this row (`ONE_LOCUS_LANE`). `one_record.rs` makes two records of
one plane, cylinder, cone, sphere or torus one record where their faces
face one way, so the arm is reached only by two records it does not
join: a spline or a spindle record stated twice, or two records of one
round surface whose faces face opposite ways.

## What would close it

One record for a spline or spindle surface stated twice, and a reading
of two records of one surface facing opposite ways along an edge (which
bounds no valid solid there and should refuse as the file's).

## Evidence: which arm the two-record cases reach (EXPORT, `export/import-records`, 2026-11-22)

Since records follow the file (`[import]`, "Records follow the file"),
two entities printed alike are two records unless `one_record` joins
them; nothing joins them silently any more. Two of the cases this row
names do not reach `ONE_LOCUS_LANE`'s arm:

- **A spline stated twice** with an edge between its two faces (an
  interior ruling of `loft_prism`'s twisted wall) refuses as a tangent
  section, `SECTION_LANE` ("a section construction for this pair"): the
  pair section reads two records of one spline as a tangency, not as
  `CoincidentSurfaces`. Pinned by
  `crates/step-import/tests/records_follow_the_file.rs`,
  `one_spline_stated_twice_with_an_edge_between_refuses_typed`.
- **Two records of one sphere facing opposite ways** along an edge (the
  ball's bands, one reversed, on two printed-alike entities) refuse as a
  sphere × sphere pair whose section the kernel does not build, also
  `SECTION_LANE`. Pinned by the same file's
  `a_pinch_is_refused_at_tier_3_on_one_record_or_two`.

So the row's closing reading has to meet these pairs before the
section lane does, or the lane they name is the one that would recover
them.
