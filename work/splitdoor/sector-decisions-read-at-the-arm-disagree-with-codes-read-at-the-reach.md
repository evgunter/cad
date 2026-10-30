---
id: sector-decisions-read-at-the-arm-disagree-with-codes-read-at-the-reach
kind: issue
title: Sector decisions read at the shorter arm can disagree with side codes read at each bound's reach
status: dispatched
priority: P2
cost: M
opened: 2026-10-29
---

Found by the review of PR 6005 (branch `splitdoor/boss-in-hole-germ`).

The boolean's sector passes take some decisions at a sector's **arm**,
the shorter bounding chord: `bool_faces_parallel` (the coplanar test in
`sectors::pair_search`), `within` / `within_signs` (`sectors.rs:1221`, `sectors.rs:1238`),
`SECTOR_REFLEX` (`sector_shape.rs:386`) and the germ line in
`insert.rs` `germ_dir` (`insert.rs:2422`). The side codes (`side_code`,
`plane_side_code`, `pair_codes`) read each bound at **its own reach**: a
line edge's far vertex, a curved edge's extent. Where a bound is much
longer than the arm, a direction can sit in band at the arm and
definitely off at the reach, and the two readings then disagree about
one ray.

PR 6005 settled one combination at one site: in `pair_search`, a ray
`within` holds at the arm while both of a sector's codes read one
definite side at the reach. The pair is now no record
(`sectors::one_side`). The boss inside a hole cut into short arcs
either side of the touch reached it and refused
`ClassificationInvariant`.

The other sites, unmeasured:

- `insert.rs:2398` and `insert.rs:2450`: the declared-Tangent germ
  direction and `germ_dir` sign a surviving record's line by `within` at
  the arm. Both rays can read within when a sector spans within about
  ε/arm of a half-turn, which refuses "germ direction not uniquely
  within its sector pair".
- `sectors.rs:1910` `runs_into`.
- `sectors.rs:2006` `sector_overlap`: the coplanar overlap, by `within`
  at the arm.
- `sectors.rs:1264` `half_planes`: `plane_side_code` at the reach
  chooses the faces, and `within_signs` at the arm places the ray.
- The `recl_sectors` propagation (`recl.rs:393-436`, `recl.rs:497-518`):
  codes written into a neighbour that `pair_search` admitted at the arm.
- The reverse case: a bound reads `On` at its reach while `within` reads
  the line definitely outside the sector at the arm, so an `On` mention
  lands on a pair that was never recorded.

To do: for each site, decide which reading wins and why. The coplanar
test sides with the arm deliberately (`sectors` module doc); the
crossing record sides with the reach since PR 6005. Build a witness
before changing any site.

