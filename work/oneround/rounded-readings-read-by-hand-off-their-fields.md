---
id: rounded-readings-read-by-hand-off-their-fields
kind: issue
title: Three sites read a Rounded's room or magnitude by hand off its fields
status: open
opened: 2026-11-19
priority: P1
cost: E
---


Found by the sweep of `oneround/rounded-room` (shape: a `.abs() - <error>`
or `.abs() + <error>` read off a value/error pair), which routed
`geom_brep::half_angle`'s Taylor terms through `Rounded::room`,
`magnitude` and `divisor`. Each site below holds a
`geom_core::running::Rounded` already and restates one of its doors on
its fields:

- `topo::boolean::conic_quadric` (`mod.rs`, the cone arm's `side`
  closure, row `CONE_DEFLATED_SIDE`): `Margin::of(res.value.abs() -
  res.error)` is `res.room()`, less its clamp at zero. It is decided at
  the caller's `band`, not at `Band::enclosure_sign()` as
  `Rounded::divisor` reads a room, and a second `decide` under the same
  row reads `res.value` for the sign: `Rounded::toward_zero` is the one
  reading that carries both.
- `geom_brep::props::curved` (the pole-plane read, row
  `chart_singularity_pole_plane`): the running `dot` is unpacked into
  `(height, rounding)` and read as `Margin::sign_of(height.abs() -
  rounding)` at `Band::enclosure_sign()`, with a second `decide` for the
  side. The same pair of reads as above.
- `geom_brep::implicit` (the sphere × circle frame defect, `middle`):
  `(a.value + b.value + a.value * b.value - c.value.powi(2)).abs() +
  a.error + … + two * c.value.abs() * c.error` is
  `(a + b + a * b - c.square()).magnitude()` expanded by hand, without
  each operation's own `u·|result|`.

Fix: read each through the door. None divides by the room, so none is
the unguarded-division defect. The first two change which band the room
is read at only if they move to `divisor`; `toward_zero` under the
site's own band keeps it.

P1 by `work/README.md` Priority (two implementations of one underlying
logic); small.
