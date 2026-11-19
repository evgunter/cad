---
id: a-relation-the-snap-catalogue-does-not-make-exact-refuses
kind: issue
title: step-import: a relation the file band reads Zero and the run reads apart that the snap catalogue has no construction for refuses not-yet-supported
status: open
opened: 2026-11-19
---


Found by `importgate/snap`, which builds D7 step 3's snap catalogue
(`crates/step-import/src/snap.rs`).

## What

A relation the file's band reads `Zero` and the run's band reads apart
refuses `NotYetSupported` naming this row where the catalogue makes it
exact by no construction:

- **a relation outside the catalogue.** The catalogue reads coaxial,
  tangent (plane × cylinder only) and axis-normal between the two faces
  of an edge, and one record of a surface (`one_record.rs`). The build's
  arms that find the file band and the run band reading another verdict
  (`build.rs`: `analytic_pair`, `served_pair`, `tangent`, `spline_pair`,
  each naming `crate::snap::UNCATALOGUED_LANE`) still meet the rest: a
  tangency of two cylinders, of a plane and a sphere, cone or torus, of
  two spheres, a coaxial tangency the run does not read coaxial, a
  spline wall's section, and two records of one surface no
  `one_record` reading joins (a spline or spindle record).
- **a class the catalogue does not solve** (`snap.rs`, `solve` and
  `tangent`): a cylinder tangent to three or more planes; one tangent
  to two parallel planes its axis does not lie between; a cylinder
  whose tangency moves it while it is coaxial with another surface (a
  fillet ending at corner spheres, pinned by
  `a_relation_the_file_band_reads_zero_is_snapped.rs`'s
  `a_fillet_with_a_coaxial_partner_refuses_as_unsolved`); a surface two
  constructions would move, or one whose construction reads a surface
  another moves; and a class some relation of which its constructions
  leave not exact at the run's band.

No file of the corpus reaches either today (`importgate/snap`'s census,
2026-11-18, ε 1e-9, 1e-6 and 1e-12).

## What would close it

A construction for each relation and class shape a file needs, added to
the catalogue's one list (`snap.rs` module docs, `SnapRelation`).
