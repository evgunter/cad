---
id: point-in-solid-mints-outside-its-period-and-nappe-rows
kind: unit
title: point_in_solid's wall junction, wall trim, latitude and root-count arms mint INVALID by hand (mints C13-C18)
status: open
opened: 2026-10-10
priority: P1
cost: M
parent: topo-mints-indeterminates-outside-the-funnel
refs: [classification-invariant-family-types-bug-only-states-against-d9-row-4, period-headroom-margin-has-no-shared-home, cone-nappe-is-decided-in-five-places]
---

Unit 3 of `topo-mints-indeterminates-outside-the-funnel`'s re-scope
(2026-10-10, main `98a3817a1d`).

## Sites

All are in `boolean/solid_contain.rs` unless named otherwise.

- **C13, `:2760` (`bool_wall_junction`, the pieces' sides disagree).**
  C14, `:2766` (both ends of a piece are active).
  - Two decided verdicts that cannot both hold.
  - Each is either an invariant (a panic, D9 row 4) or a typed finding
    carrying both readings. Read each and decide (design item 7).
- **C15, `:2774` (`bool_wall_trim` decided Zero).** Its sibling Zero
  arms answer `Ok(None)`, a graze.
  - Decide whether this one is a graze too.
  - If it is not, route it through `decide_nonzero`.
- **C16, `:3534` (`latitude_extremes`, no levels).** A structural empty,
  with no margin read. It should be a typed `PointInSolidError` arm, not
  an `Indeterminate`.
- **C17, `:5405` (`bool_ray_torus_count`).** C18,
  `boolean/sphere_region.rs:384` (`bool_sphere_region_roots_count`).
  - `CountDisagrees` is a broken invariant.
  - Whether it panics or stays typed is
    `classification-invariant-family-types-bug-only-states-against-d9-row-4`'s
    call. Take that row's answer, or settle it here and say so there.

## Not this unit

- PRED's four sites stay on PRED's rows: `:1289` and `:2987`
  (`bool_wall_trim_period`) on `period-headroom-margin-has-no-shared-home`,
  and `:1938`/`:1943` (`cone_nappe`) on `cone-nappe-is-decided-in-five-places`.
- PR 4497 accepted `PointInSolidError::Escalated`'s ending (the
  placement lever plus the note). Moving an arm off `INVALID` changes
  which note it takes, so pin it.
