---
id: a-relation-the-snap-catalogue-does-not-make-exact-refuses
kind: issue
title: step-import: a relation the file band reads Zero and the run reads apart that the snap catalogue has no construction for refuses not-yet-supported
status: open
opened: 2026-11-19
priority: P3
---


Found by `importgate/snap`, which builds D7 step 3's snap catalogue
(`crates/step-import/src/snap.rs`).

## What

A relation the file's band reads `Zero` and the run's band reads apart
refuses `NotYetSupported` naming this row where the catalogue makes it
exact by no construction:

- **a relation outside the catalogue** (stated in `snap.rs`'s module
  docs and `SnapRelation`). The build's
  arms that find the file band and the run band reading another verdict
  (`build.rs`: `analytic_pair`, `served_pair`, `tangent`, `spline_pair`,
  each naming `crate::snap::UNCATALOGUED_LANE`) still meet the rest: a
  tangency of two cylinders, of a plane and a sphere, cone or torus, of
  two spheres, a coaxial tangency the run does not read coaxial, a
  spline wall's section.
- **a class the catalogue does not solve** (`snap.rs`, `solve` and
  `tangent`): a cylinder tangent to three or more planes; one tangent
  to two parallel planes its axis does not lie between; a cylinder
  whose tangency moves it while it is coaxial with another surface (a
  fillet ending at corner spheres, pinned by
  `a_relation_the_file_band_reads_zero_is_snapped.rs`'s
  `a_fillet_with_a_coaxial_partner_refuses_as_unsolved`); a move the
  file's band reads apart; and a class some relation of which its
  constructions leave not exact at the run's band. A surface the
  anchored solve would move twice is its own row,
  `a-snap-class-whose-solve-moves-a-surface-twice-refuses`.

No file of the corpus reaches either today (`importgate/snap`'s census,
2026-11-18, ε 1e-9, 1e-6 and 1e-12).

## What would close it

A construction for each relation and class shape a file needs, added to
the catalogue's one list (`snap.rs` module docs, `SnapRelation`).
