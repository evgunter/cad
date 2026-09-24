---
id: validate-passes-a-body-with-a-zero-width-slit-face
kind: issue
title: topo::validate passes a split half whose face carries a zero-width slit and two coincident vertices
status: open
opened: 2026-09-24
priority: P2
cost: D
---


## What

Before `emit/split-spur`, `topo::split` of `brick(0..1.5, 0..1, 0..1)
∪ brick(1.2..1.3, −1..2, 0.5..3)` by the plane y + z = 2 returned two
halves. The plane touches the block only along the edge y = z = 1, and
the halves were degenerate in two ways:

- **A slit in the section face.** Each half's section face carried an
  out-and-back slit along y = z = 1, running from (1.2,1,1) to (0,1,1)
  and back, and from (1.3,1,1) to (1.5,1,1) and back.
- **Coincident vertices in the Below half.** The Below half held TWO
  vertices at each point along the slit.

`topo::validate` returned `Ok(())` for both halves: 1 solid each, 6
faces Above and 15 Below.

The join now refuses the spur
(`crates/topo/src/splitting/join.rs`, `refuse_section_spur`). That
stops the one producer that was measured, but it does not answer the
question this row asks: nothing at rest refuses a body with a
zero-width face excursion or with coincident distinct vertices. Every
whole-body measure the tree has is a net measure:

- `validate`'s enclosure-volume check (`crates/topo/src/validate.rs`,
  the `volume_hi`/`volume_lo` margins, ~3387);
- `solid_contain`'s V/A check.

None of them can see a slit.

## Why it matters

A later producer of the same residue would pass every tier and surface
only where a consumer trips over it. The measured case surfaced as
`NamingError::Duplicate` in the split's name emission, which reads as
a kernel bug and pointed at the wrong module.

Unmeasured: the cost of a coincident-vertex or zero-width-face check
at rest.

## Found by

The sweep for EMIT's `split-section-face-keeps-a-zero-area-spur-along-a-tangent-edge`.
