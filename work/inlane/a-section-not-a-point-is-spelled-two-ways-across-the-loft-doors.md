---
id: a-section-not-a-point-is-spelled-two-ways-across-the-loft-doors
kind: issue
title: loft_body refuses a section wider than a point as Skin(SectionProfile) from its f64 validation, loft_profiles as SectionNotAPoint: one fact, two spellings
status: closed
opened: 2026-10-11
priority: P1
closed: 2026-10-11
cost: E
---

## Finding

Raised in #4741's review (style lane; coded unilateral). Not taken there.

Both loft doors refuse a section whose values are wider than a point,
but they report it under different names:

- `loft_body` / `sweep_body` validate the sections' value channel at
  `f64` (`crates/sweep/src/skin.rs:1083`, `validate_loft`). A section
  with no single point shows up there as NaN coordinates, and is
  refused as `LoftError::Skin(SkinError::SectionProfile { section, .. })`
  carrying a `ProfileError` (`crates/sweep/src/skin.rs:970`, `:997`).
- `loft_profiles` checks the value channel first
  (`crates/sweep/src/loft.rs:1118`, `profile_is_finite` at `:1144`)
  and refuses `LoftError::SectionNotAPoint { section }`
  (`crates/sweep/src/loft.rs:192`).

So a caller that matches on the refusal has to know which door it went
through. Placements already have one home, `placements_are_points` in
`loft.rs`, which both doors call first.

## Repair shape

Give the section check the same single home: `build` runs the finite
check on its channel sections before `validate_loft`, so `loft_body`
also refuses `SectionNotAPoint`. That moves `loft_body`'s refusal at
`Interval` for a wide section, which is pinned at
`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs`
(`a_loft_over_a_section_wider_than_a_point_refuses_at_interval`).
Re-baseline that row, and every editor-core row that reads the
`Skin(SectionProfile)` arm for a not-a-point section.

## Closed

Closed with `the-skin-at-interval-encloses-the-nominal-skin`. A
section wider than a point no longer refuses at either door: at an
enclosure scalar it lofts its family. What remains of the refusal
has one home, `crates/sweep/src/loft.rs`'s `build`, which both doors
enter. Before the split it refuses a section or placement holding a
value that is not finite (poison or an infinity) as
`LoftError::SectionNotFinite` / `PlacementNotFinite`, renamed from
`SectionNotAPoint` / `PlacementNotAPoint`. So `loft_body` over a NaN
section now refuses `SectionNotFinite` where it refused
`Skin(SectionProfile)`. That is re-baselined at
`crates/sweep/tests/loft_profiles_door.rs`
(`a_value_that_is_not_finite_refuses_by_name_at_both_doors`).
No editor-core row read the `Skin(SectionProfile)` arm for a section
with no point.
