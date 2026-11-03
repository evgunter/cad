---
id: an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand
kind: issue
title: An angle read clear of its cut, anchored at a seed, is spelled five ways, and Conic::angle_of is written by hand at about seven sites outside geom
status: open
opened: 2026-11-03
priority: P1
cost: M
design: true
refs: [6449]
---


Found by the FULL review of PR 6449 (`onecopy/foot-and-angle`, style S5 and NOTE-5), at head `5751445aa5`.

## The anchored family

"The angle of an in-plane point, read clear of `atan2`'s cut by anchoring at a seed" is spelled in five places:
- `geom::curves::on_carrier::angle_near` (the caller's branch, half a turn on where the point is behind);
- `geom_brep::curve_pair::pose`'s `separation`;
- `geom_brep::curve_pair::span::Span::branch`, with its own antipode fallback;
- `geom::on_span_branch`;
- `sweep::blend::surgery`'s `past`.

PR 6449 settled `Azimuth` (a windowed read, which keeps `Sym` folding and documents why it does not anchor) against `angle_near`, and kept both. The anchored readers among themselves are the open question. Is there one anchored reading in `geom` that all five call? If so, does it belong on `Azimuth` as an exit, against that type's documented reason, or beside it?

## `Conic::angle_of` by hand

The hand-written `w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref))` reads of a point on a conic appear in production at:
- `topo/src/boolean/rim_wedge.rs` `locus_param` (waits on PR 6443 and stage 3 of the placement decision);
- `topo/src/ring_path.rs` (two; `CircleArc` is being repointed by `the-frame-carries-every-placement-reading`, so take what remains after it lands);
- `sweep/src/blend/surgery.rs`;
- `topo/src/boolean/circle_torus.rs`;
- `mesh/src/walk.rs` (two).

The sweep that found them is `atan2(` alongside `u_ref|v_ref|cross(`, outside geom. It cannot see an angle spelled through local names.

## Related class

A branch or formula tie-break decided at a caller's band, rather than at `Band::least()`, records an escalation the answer does not rest on. PR 6449's fix pass moves the pose's phase read back to `Band::least()` and looks at `angle_near`'s half-turn flip. An anchored home built here should decide its tie-break at `Band::least()`.
