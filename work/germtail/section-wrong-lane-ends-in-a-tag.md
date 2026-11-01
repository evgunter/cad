---
id: section-wrong-lane-ends-in-a-tag
kind: issue
title: geom-brep: SectionError::WrongLane calls itself a kernel bug, but any caller of a public section arm reaches it with the wrong pair
status: closed
opened: 2026-09-28
parent: a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids
priority: P4
cost: E
closed: 2026-10-26
---


(ENCL implementer, from the sweep of
`work/encl/kernel-defect-endings-and-repair-labels-have-no-shared-home.md`.)

`crates/geom-brep/src/intersect.rs` is GERM's, REACH's and TANG's by
territory.

## What

`geom_brep::intersect::SectionError::WrongLane`
(`crates/geom-brep/src/intersect.rs:708`) reads "the section was
dispatched to the arm for {expected}, not this pair's (a kernel bug)":
no `There is no way through` marker (zero to
`test_utils::refusal::recourse_markers`) and no report. The ending has
one home now, `geom_core::KERNEL_DEFECT_ENDING`.

## Repair shape

**Superseded in direction (ENCL, 2026-10-21): read the evidence below before acting on this section.**

Replace the tag with ". {KERNEL_DEFECT_ENDING}".

## Evidence: input reaches it, so the repair above is wrong (ENCL, 2026-10-21)

This arm is a site of the parent census's second pass (`work/encl/a-typed-kernel-defect-ending-may-launder-a-bug-d2-forbids.md`, `## Census, second pass`, main 69ab780194). **Class 1, sure**: `geom_brep::plane_sphere_section(plane, sphere, band)` (`intersect.rs` ~1188) is public, and handing it any non-plane first operand or non-sphere second one reaches `WrongLane` (~1199, ~1210). Its sibling arms behave the same (`sphere_sphere_section`, `cylinder_cylinder_section`, …). The tag and `KERNEL_DEFECT_ENDING` both make a false claim. The repair is the lever, as SSI's own `WrongLane` gives it (`ssi.rs` `WRONG_LANE_RECOURSE`): pass the pair the arm names, or route the pair through `geom_brep::route`.

## Closed (2026-10-26, easy-fixes lane)

`crates/geom-brep/src/intersect.rs` `SectionError`'s `Display` now renders `WrongLane` as "the section was dispatched to the arm for {expected}, not this pair's. {WRONG_LANE_RECOURSE}", the lever the evidence section names. `ssi.rs`'s `WRONG_LANE_RECOURSE` is now `pub(crate)`, so SSI's and the section's `WrongLane` share one ending. The chain test's two admissions for the recourse-less row, `Boolean/Join/Section` and `Split/Join/Section` in `refusal_concision_chains.rs` `FILED_NO_RECOURSE`, are removed, because its must-fire check flagged them as unneeded. `hone/reach-refusals-short-of-the-shape-guard` carries a note. Verified: `cargo nextest run -p geom-brep -E 'test(/sphere|intersect_table|compensated|wrong_lane|lane|enclose|nurbs/)'`, 404 passed. `cargo nextest run -p editor-core --test all -E 'test(/refusal_concision_chains|display_contract|recourse/)'`, 62 passed.
