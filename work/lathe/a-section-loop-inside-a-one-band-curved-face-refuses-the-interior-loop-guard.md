---
id: a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard
kind: issue
title: A section loop wholly inside a one-band curved face and the partner face refuses at the interior-loop guard, where the π meridian used to cross it
status: closed
opened: 2026-11-22
priority: P1
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
cost: H
closed: 2026-11-26
pr: 7440
---

Found by the LATHE one-band lane (2026-11-22, PR 7414), measured on its
branch `lathe/full-revolve-one-band`.

## What

A full revolve's sphere is now one face whose only cut is its seam, the
`u = 0` meridian. A plane whose section circle misses that meridian (a
slab plane on the side of the ball away from the seam) leaves a closed
section loop inside the sphere face and inside the partner's plane face,
crossing no edge of either. The crossings path's interior-loop guard
(`crates/topo/src/boolean/ops.rs` `interior_loop_verdict`) cannot certify
the pair free of such a loop, so the boolean refuses
`CurvedPairUnsupported { op: Some(Intersect), site: InteriorLoopGuard, kind: Sphere, other_kind: Plane }`.
With two π-bands the angle-π meridian crossed every such circle that
missed the angle-0 one, unless its plane was parallel to the meridian
plane, so the class existed before but was much narrower.

Failing on the branch:
`seam_siblings_flipped_together::a_holed_region_reads_its_side_by_its_area`
(a ball ∩ slab `x ∈ (−0.5, 0.5)`: the circle `x = −0.5` misses the seam).
Other rows whose fixtures used the π meridian may meet it once their
two-band assertions are re-baselined; the re-baseline in PR 7414 lists
any it finds here.

## Ground

`crates/topo/src/boolean/ops.rs`, `crates/topo/src/boolean/mod.rs`
(`PairRefusalSite::InteriorLoopGuard`), and the section certificate it
asks; topo's boolean territory (as the single-site row). Claimed by LATHE.

## Open

Whether the guard's interior loop is built (inserted as a ring on both
faces, the shape `topo::insert_void` and the no-crossings path already
read) or the join cuts the face at a conventional meridian first. The
latter would put back a cut nobody wrote, against `[topology]`'s "the
kernel adds none".
