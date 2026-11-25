---
id: a-carriers-value-and-gradient-have-a-second-home-in-injective
kind: issue
title: The analytic carriers' values and gradients are spelled twice, generically in geom_brep::implicit and over boxes in topo::injective's Psi, and the two read the cylinder's and cone's gradient differently
status: open
opened: 2026-11-22
priority: P3
cost: M
---


## Finding

Filed by EMBED's lane `embed/iso-end` (PR 7327, 2026-11-22), at review.

`geom_brep::implicit::{implicit_residual, implicit_gradient}`
(`crates/geom-brep/src/implicit.rs`) hold the plane's, sphere's,
cylinder's and cone's values and gradients, generic in `T`.
`topo::injective`'s `Psi` (`value`, `rate`, `gradient_norm`) holds them
again, over interval boxes, for the injectivity certificate's carrier
separator.

The two spellings are not the same quantity in every arm:

- `implicit_residual` is the squared form for some carriers;
  `Psi::value` reads a distance less a radius.
- `implicit_gradient`'s cylinder reads `w / radius`, with `w` the radial
  part from `axial_radial`. Its cone reads
  `w.normalize()·cos − axis·sin.copysign(h)`, with the axis taken as unit.
- `Psi::rate` reads a direction's rate, not the vector. Its cylinder and
  cone dot the radial unit with the direction less its part along the
  axis. Its cone's axial term is over `|axis|`, because a stored axis is
  unit only to its convention: an axis 1 + 5e-11 long put the true rate
  3e-11 outside the old enclosure.

**Checked:** the four arms of each, by reading. Not checked: every
caller of `implicit_gradient`, and whether any of them reads it over a
box.

**What it needs:** one home for an analytic carrier's value and its rate
along a direction, read by both. Or a statement at each of why two are
owed.
