---
id: an-axis-pairs-bound-reads-the-chord-under-the-nearer-sign
kind: issue
title: The cylinder, cone and torus pair bounds read the axes' chord under the nearer sign, min(|a1 - a2|, |a1 + a2|), which is zero as an expression only where both axes carry one sign spelling
status: open
opened: 2026-11-24
priority: P2
cost: M
---



Found by the one-plane unit's sweep
(`the-one-plane-reading-multiplies-a-normal-by-a-decided-sign`,
`boxfinish/one-plane`, 2026-11-24). It is the comparison-free sibling
of the shape that unit removed from the plane reading: no sign is
decided, but the chord is taken under whichever sign brings the two
directions nearest, and neither `|a₁ − a₂|` nor `|a₁ + a₂|` is zero as
an expression where one axis is spelled with a sign factor the other
does not carry (a literal axis beside a chord over its own length).

**Sites** (each reads `min(|a − b|, |a + b|)` of two unit directions):
- `geom_brep::pair_section::line_tilt`, read by
  `geom_brep::carrier_eq::reading`'s cylinder arm (`carrier_cyl_reach`)
  and cone arm (`carrier_cone_reach`), by
  `pair_section::torus_coincidence`'s `TorusDisplacement::of` (the
  torus pair's bound, `line_tilt(..).max(sine)`), and by
  `pair_section/torus.rs`'s axis pose (`swing`, after
  `section_axes_tilt` reads Zero);
- `geom-brep/src/locus.rs`, `cylinders` (`turn`, the tangent locus's
  slack);
- `sweep/src/blend/reach.rs`, `tilt` (a sheet bound's slack), and in
  the same file the plane sheet's `(n − a·sg).norm()`, where `sg` is a
  sign read off an enclosure: there the sign is decided, and the chord
  is an enclosure's slack and not a zero claim.

**A neighbour with the sign fixed by the question** (read, not run):
`topo/src/shell.rs`'s moved-walls pairing reads two walls as opposed
through `(a.normal + b.normal).norm()` (`shell_walls_antiparallel`) and
`−a.normal·b.normal − 1` decided Zero
(`shell_walls_antiparallel_cosine`). No sign is decided and the Zero is
recorded nowhere (it selects which pairs are read), but it is zero as
an expression only where the two normals are spelled as exact
negations.

**Measured** (the unit's head, a scratch probe, not kept): two
cylinders on one axis with one radius, over a unit ball, in a deferring
`Sym<f64>` session, through `geom_brep::carrier_eq::pair_door_reading`.
Both axes the literal `(0, 0, 1)`: one carrier, the margin discharges.
One axis `(0, 0, v/√v²)`: one carrier, the margin does NOT discharge.
With the cylinder arm's `line_tilt(a₁, a₂)` replaced by
`‖a₁ × a₂‖` (a mutant, not kept) it discharges, so the chord is where
the identity is lost; the data rungs before it (`‖a₁ × a₂‖`, the
perpendicular offset, the radii) are sign-free already. The cone arm,
the torus bound, the locus and the blend sites are read, not run.

**What it costs today.** The Boolean's glue of such a pair records a
margin that is no theorem. The coincidence door still proves a
cylinder pair through its own reading (`coincide::replay::one_cylinder`,
whose tilt is `|da × db|`), so no door row was seen to move.

**A sign-free bound, not built.** The chord under the nearer sign is
`2·sin(θ/2)` for `θ ≤ π/2`, and `2·sin(θ/2) ≤ √2·sin θ` there
(`cos(θ/2) ≥ 1/√2`), so `√2·‖a₁ × a₂‖` bounds it with no sign and no
`min`, 41% looser in band. Whether the bound may loosen, or the swing
should be re-derived from the sine as the plane arm's was, is the
design question.

**Done when** the cylinder, cone and torus pair bounds are zero as
expressions for two axes that are one line as formulas whichever sign
each is spelled with, with a row like
`plane_eq::tests::one_plane_spelled_with_two_signs_is_one_plane_as_an_expression`
per kind, and the locus and blend sites are fixed or dispositioned.

Signed: (BOXFINISH one-plane lane)
