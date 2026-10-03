---
id: sqrt-up-respelled-where-a-zero-sup-moves-bits
kind: issue
title: Three sites re-spell sqrt_up as sqrt().next_up(), which reads an exact zero sup as the smallest subnormal
status: open
opened: 2026-10-01
---


## The finding

`geom_core::interval::sqrt_up` is the kernel's one spelling of a
square root rounded up, beside `sqrt_down`, `norm_sq`, `norm_sup` and
`div_down` (the `ssi/chart-rate` lane collapsed the copies in
`offset_meters`, `ssi/enclose` and `patch_bound` into it). Three sites
still spell the rule inline as `x.sqrt().next_up()`:

- `crates/geom/src/curves/second_derivative.rs`,
  `nonrational_second_derivative_sup`:
  `let bound = sum_sq.hi().sqrt().next_up();` (PROPS)
- `crates/geom-brep/src/props/quad.rs`, `sqrt_enclosure`:
  `hi.next_up()` over `x.hi().max(0.0).sqrt()`, two lines apart, so a
  one-line grep for `sqrt().next_up()` misses it. The lower end is
  `sqrt_down` spelled out. (PROPS)
- `crates/mesh/src/chords.rs`, the rational second-derivative sup's
  closing `s.hi().sqrt().next_up()` (CHORD/TESS ground; named here so
  the class has one row).

Away from zero these are bit for bit `sqrt_up`. They differ at an
exact zero: `0.0.sqrt().next_up()` is `5e-324`, and `sqrt_up(0.0)` is
`0.0`, the exact sup of a `[0, 0]` enclosure. That is why the lane did
not route them. The zero is load-bearing downstream:
- `chords.rs` takes a `m_bound == 0.0` arm (one chord) that a
  subnormal never reaches;
- `step-export`'s `writer.rs` adds the second-derivative sup into a
  node-count bound;
- `mesh::nurbs_cert::cell_component` already documents the same
  zero as "load-bearing … must not leave here as subnormal dust".

## The fix shape

Route each site through `geom_core::interval::sqrt_up` (and the quad
lower end through `sqrt_down`). Then decide, per consumer, whether the
exact zero it starts seeing on a straight or degenerate carrier is
the right answer. It is the exact sup, so the expectation is yes. Any
tessellation or export count that moves is re-baselined with the
reason.

Found by the `ssi/chart-rate` lane's sweep for `sqrt().next_up()`.
That pattern cannot match a root and its step written apart, which is
how `quad.rs` was found only by a second pass over `fn *sqrt*`.

## Since `linalg/certification-gains-a-sqrt-door`

The door exists (`Certification::sqrt`, the backend's root), and
`sqrt_up`/`sqrt_down` are gone from `geom_core::interval`. Two of the
three sites retired into the door on that branch: `quad.rs`'s
`sqrt_enclosure` and the `chords.rs` closing root. The backend's root
of `[0, 0]` is now exactly `[0, 0]` (`interval_transcendentals`'
`sqrt_hi` used to pad it to the least subnormal, below the 2Prod
witness's floor), so a door root keeps the exact zero this row worried
about.

What is left is the first site:
`crates/geom/src/curves/second_derivative.rs`,
`nonrational_second_derivative_sup`, still
`sum_sq.hi().sqrt().next_up()`. The file is a listed door importer, so
the fix is `sum_sq.sqrt().hi()` behind the typed refusal it already
asks. The consumer judgement this row asks for still stands: at an
exact-zero sum it moves the bound from `5e-324` to `0.0`, and
`step-export`'s `writer.rs` reads it into a node count.
