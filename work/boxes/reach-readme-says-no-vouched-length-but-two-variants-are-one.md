---
id: reach-readme-says-no-vouched-length-but-two-variants-are-one
kind: issue
title: geom-core README says a Reach is never built from a length a caller vouches for, but Reach::Measured and Reach::Face are
status: open
opened: 2026-11-14
priority: P2
cost: M
---

`crates/geom-core/README.md`, "Float rounding", the consumed-extent bullet, says `geom_brep::Reach` is "built only from the geometry consumed … never from a length a caller vouches for". But two variants in `crates/geom-brep/src/extent.rs` are built exactly from such lengths:
- `Reach::Measured` holds a caller's measured length, built through `of_frame`;
- `Reach::Face` holds a caller's `at` and `across`, built through `of_face`. From these `turn_lever` and `hinge_lever` are derived.

Either the text or the variants is wrong. If the variants are wrong, the destination is the one the `foot_on` fork (`a-reachs-pivot-is-chosen-per-kind`, `[ev]` PR 6968) takes for edges: the reach holds its region as a cover. A face reach would need its own derivation of the hinge and turn levers.

Both designers on that fork raised this independently, as off-question.
