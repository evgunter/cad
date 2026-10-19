---
id: cone-point-reads-take-the-slant-as-height-over-cos-alpha
kind: issue
title: five crates read a point's cone slant as its height over cos α, which levers its normal offset by 1/cos α, where the harmonic images now project on the generator
status: open
priority: P4
cost: M
refs: [a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12, focal-section-image-reads-its-slant-off-height-on-a-flat-cone]
opened: 2026-10-14
---

Found by the sibling sweep of `a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12`.
That row moved the cone chart's harmonic images (ruling, rim) to a
slant taken by projection on the generator, `v = h·cos α + n·ρ·sin α`
for a point at height `h` and radius `ρ` on nappe `n`, so an image
misses its carrier by the carrier's distance from the cone. Reading the
slant as `h / cos α` instead levers a point's normal offset `δ` to `δ /
cos α` along the generator (48× at α = 1.55). On the cone the two
agree; off it by rounding they differ by about `δ·tan α`.

The point reads still spelled the second way, by name (lines as of
2026-10-19's main):

- `crates/topo/src/chart.rs` `Chart::v_of` (`:200`, `:267`) and the
  generic `v_of` (`:370`);
- `crates/topo/src/boolean/solid_contain.rs` the cone arm's slant window
  `cone_slant_window` (`:2166`) and the cone point test (`:2883`);
- `crates/topo/src/replace_face.rs` the cone `v` windows (`:1088`,
  `:2230`, `:2250`, `:2264`);
- `crates/geom-brep/src/props/curved.rs` the cone slant levels (`:1980`,
  `:2769`, `:2770`, `:2780`);
- `crates/step-import/src/chart.rs` the imported cone chart (`:156`).

The same shape's other hits, `/ cos_a` in
`crates/geom-brep/src/intersect.rs` (a rim radius `|t|·tan α`, a reach
bound and an azimuth lever), read no point's slant and are not this row.

Each reads a point that lies on the cone to its own rounding, and none
has a measured refusal; the risk is a comparison between such a read
and a harmonic image at a shared vertex on a flat cone at large extent,
where the two now differ by the read's own over-lever. No single program
owns all five (topo's chart, CLEAVE/HONE/INSIDE's containment,
SHELF/SHELL's replace-face, FLUX's props, EXCH's import), hence this
row's place here.

## What closes it

One shared slant read on the cone (the projection), used by every site
above, with a measurement on a flat cone at large extent showing which,
if any, was over the band.

