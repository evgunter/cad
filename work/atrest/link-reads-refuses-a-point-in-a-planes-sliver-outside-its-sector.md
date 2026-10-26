---
id: link-reads-refuses-a-point-in-a-planes-sliver-outside-its-sector
kind: issue
title: The census's parity reading refuses a point in a plane's sliver outside its sector
status: open
priority: P4
cost: E
opened: 2026-10-26
refs: [material-and-convexity-are-read-three-ways]
---

The census's parity reading (`Link::reads_over`, `crates/topo/src/link.rs`) takes each sector's side of `q` first, through `Link::side`, and returns `None` as soon as one reads in the band's sliver. It never asks whether `q` lies inside that sector (`Sector::place`), so a point in the sliver of a sector's plane refuses even where it lies decidedly outside the sector, far from the cone's boundary.

**What is left.** `Link::offset` (and with it `Gap`'s cell arms, at every scalar) reads such a plane as one `q` lies on outside its sector, with the arc's crossing stretch widened from the band's zero to `q`'s own distance from the plane (`Link::parity` with `wide`). That fix landed on `material-and-convexity-are-read-three-ways`'s fix pass, where reviewers measured the refusal sinking 831 of 1500 random edge-pair readings. The census's own reading still refuses, pinned by `link::tests::a_point_in_a_planes_sliver_outside_its_sector_reads_its_offset`: its call to `reads_over` passes `wide = false`.

**What a fix looks like.** Pass `wide` from the census too. It moves the census's verdicts on such points from a refusal to a reading, so it is checked against the census goldens at three ε and re-blessed with the moved rows named. Nothing found yet needs it.
