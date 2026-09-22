---
id: the-render-grids-cap-is-a-length-and-angles-go-through-it
kind: issue
title: the render grid's absolute cap is derived from epsilon, which is a length, and the chrome renders angles and scalars through the same door
status: open
opened: 2026-09-22
priority: P3
cost: D
---


(VGEOM) Disclosed by `vgeom/render-grid`, the unit that put the cap
there. Filed rather than left in that PR's body, because a residue in
a PR body is gone once the program closes.

## The finding

`crate::readout`'s grid is `EPS_CAP.min(value.abs() * REL_TOLERANCE)`,
and `EPS_CAP` is `DEFAULT_EPS * 0.1` — one decade below ε. **ε is a
LENGTH** (`docs/DESIGN.md` D4 ¶1). The cap's whole argument is that a
difference the kernel can DECIDE must be a difference the render
SPELLS, and that argument is about lengths.

`crate::readout::number` is handed a value in whatever notation the
caller chose, and `crate::props::written_text` reaches it from every
row of `quantity::UNITS` — `deg`, `rad`, `pi rad` and the
dimensionless row included. For those four the cap is not derived from
anything:

- **It is never too coarse**, which is why it is not a defect: 10⁻¹⁰
  of a radian, of a degree or of a bare scalar is far below any
  distinction the kernel makes, so nothing is lost.
- **It is not argued either.** `the_grid_is_never_coarser_than_epsilon_in_any_written_unit`
  (in `readout`'s own rows) reads only the LENGTH rows of the table,
  because those are the rows the claim is about. The angle and scalar
  rows ride on a length constant, and the only reason that is safe is
  that it happens to be very small.

## What it costs today

Characters, and only characters. An angle or a scalar off the cap's
own decimal grid is spelled to ten decimal places or falls to the
scientific arm, where four figures would have named it to anything an
angular predicate could tell apart.

## What a fix has to answer

What an ANGLE render owes, and what a dimensionless one does — the
same question `a-fields-text-commits-within-the-renders-own-tolerance`
answered for a length, asked of the other three quantities the closed
unit table carries. `crates/geom-core`'s tolerance vocabulary is where
the answer would come from if there is one, and whether there IS an
angular ε the chrome may cite is the first half of the question.

`the-camera-hud-spells-its-angles-at-a-fixed-tenth-of-a-degree` is the
one production render waiting on it.
