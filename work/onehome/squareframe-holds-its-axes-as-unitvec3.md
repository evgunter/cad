---
id: squareframe-holds-its-axes-as-unitvec3
kind: issue
title: SquareFrame holds its axis and seam as bare Vec3, so its readers check them
status: open
opened: 2026-11-22
priority: P2
---


`geom::SquareFrame` (`crates/geom/src/convention.rs:60`) holds `axis`
and `u_ref` as bare `Vec3`, though `Frame::square` makes both by a
normalization (`crates/geom/src/frame.rs:267`). Every reader that hands
one to a line goes through `UnitVec3::checked` and states the enclosure
argument at the call:

- `crates/topo/src/boolean/section_cert.rs:1266`, the cone section's axis;
- `crates/geom-brep/src/intersect.rs`, `ApexFan::ruling` (`:2131`) and the
  four cone sections' axis lines (`:2546`, `:3218`, `:3370`, `:3670`).

Holding the two as `UnitVec3` (the axis squared as `square` squares it,
the seam the Gram–Schmidt residual squared) would type them at the one
mint and retire those six checks. It was left out of PR 7349 because
`convention.rs` is under edit in the check-1 frame-half lane
(`onehome/check1-frame-half`).
