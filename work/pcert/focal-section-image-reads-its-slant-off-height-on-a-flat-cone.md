---
id: focal-section-image-reads-its-slant-off-height-on-a-flat-cone
kind: issue
title: the cone's FocalSection image reads its slant channel off the ellipse's heights over cos α, so a section off the cone by δ images δ/cos α off it
status: open
priority: P3
cost: M
refs: [a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12, focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor]
opened: 2026-10-14
---

Found by the sibling sweep of `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`,
which moved the cone chart's two harmonic images (the ruling and the
rim, `crates/geom-brep/src/pcurve_cache.rs` `derive_harmonic`) from a
slant read as `h / cos α` to the projection on the generator
(`ruling_generator`, `rim_slant`): the cone's `v` is a length along the
generator, so a carrier `δ` off the cone along its normal, read by its
height alone, images `δ / cos α` off it (48× at α = 1.55), where the
projection images it `δ` off.

`derive_harmonic`'s `Curve3::Ellipse` arm still mints the
`Pcurve::FocalSection` slant the first way: `v0: h / c_ha`, `va:
seam·axis·semi_u / c_ha`, `vb: (e × seam)·axis·semi_v / c_ha`. Its
carrier stands off the cone by its own rounding, about `u·A` (`A` the
semi-major), so the image's miss is about `u·A / cos α`.
`flat_cone_tilted_ellipse.rs`
`a_flat_cones_section_images_and_certifies_at_its_own_resolution`
allows the envelope 64 `u` of the ellipse's size at α = 1.55, where
`1 / cos α` is 48: whether its measured envelopes carry that factor is
not measured.

## What closes it

Measure the envelope at α = 1.55 against the carrier's own distance
from the cone in double-double; if the factor is there, take the
section's slant by projection as the ruling and the rim do (the
`FocalSection` lemma restated to match), and tighten that row's
allowance to what it then reads.

