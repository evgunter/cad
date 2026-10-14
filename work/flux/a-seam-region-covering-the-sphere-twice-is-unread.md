---
id: a-seam-region-covering-the-sphere-twice-is-unread
kind: issue
title: A seam region whose faces cover the sphere twice has no boundary, reads no side, and passes tier 3 with twice the volume
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [a-pair-of-seam-siblings-flipped-together-is-silent, a-seam-region-whose-boundary-carries-both-poles-is-unread]
---

## Measured

Found by the second review of PR 4746 (probe
`review_4746_2_an_exterior_meridian_doubles_the_ball_silently`, branch
`review/4746-2-016LSzZh`; the review reports the same on main). It is
pinned in `crates/sweep/tests/a_seam_region_covering_the_sphere_twice.rs`.

A z-poled unit ball is two lunes. Split one with `mef` along the meridian
leaving the north pole toward `+y`, which lies OUTSIDE that lune. The
result has three sphere faces, each with both poles on its loop, and
they cover the sphere twice. Their seam region has no boundary, so check
6 reads no side for it (`sphere_region_material_sign`: no boundary →
`Unencoded`). Tier 3 passes, and `mass_properties` measures `8π/3`, twice
the ball.

No sense bit is flipped here. The faces overlap, and nothing that reads
the sphere's faces sees it.

## What does not cover it

- Checks 7 and 10 read a shell's signed volume, which is positive here.
- The both-poles row is about a region with a boundary carrying both
  poles. This region has no boundary.
- Tier 3's global self-intersection is the clearance engine's
  (`editor-core`), which tier 3 does not run.

## A reading that may close it

Members that tile one sphere once have Gauss–Bonnet areas summing to
`4πR²`. Twice over, they sum to `8πR²`. For one-loop members, each
member's area under its stored side is well defined. A region with no
boundary whose member areas do not sum to `4πR²` (`props_sphere_face_area`
classified against it) is a cover more than once, and could be refused by
name.

