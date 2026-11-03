---
id: topo-stands-and-shell-moved-walls-re-spell-a-conics-frame-by-kind
kind: issue
title: stands::line_meetings and shell::moved_walls re-spell a conic's frame per kind instead of reading Curve3::conic
status: open
opened: 2026-11-03
---

Found by the PCERT sweep for the `harmonic3-circle-and-ellipse-arms-re-spell-conic-of`
row (conic frames re-spelled per kind instead of read through
`Curve3::conic()` and `geom::Conic::semi_axis_vectors`). PR 6349 moved
`pcurve_cache`'s `carrier_harmonic` and the sites its own row listed
onto the trait; these two were not on that list.

- `crates/topo/src/stands.rs`, `line_meetings`: a `Curve3::Circle` arm
  and a `Curve3::Ellipse` arm destructure `geom::Frame { center, axis,
  u_ref }` plus `radius` / `semi_u, semi_v`, and hand them to a local
  `conic` closure that rebuilds `v = axis.cross(u)`. One arm reading
  `carrier.conic()` (`frame().center`, `semi_axis_vectors()`) covers
  both and drops the reads of `Curve3::Ellipse`'s field names.
- `crates/topo/src/shell.rs`, `moved_walls`: the same two arms feed a
  local `conic` closure building `EdgeArc::Conic { u, v: axis.cross(u),
  ru, rv, .. }`. The same one-arm reading covers it; `EdgeArc::Conic`
  could hold the two semi-axis vectors rather than `u, v, ru, rv`.

`stands.rs` has no owning program in `work.py territory`; it is filed
here beside `shell.rs` because both are the same rewrite.
