---
id: a-non-isosceles-turn-overruns-past-the-mitre
kind: issue
title: blend: two requested edges at a non-isosceles trivalent corner refuse TURN_NOT_ISOSCELES, where Ev's ruling builds the mitre plus one short overrun curve
status: open
opened: 2026-10-07
priority: P2
cost: H
design: true
needs_ev: true
---


Split from `a-plane-plane-blend-cannot-end-at-an-unrequested-corner` at its
step 4 (PR 4209), as Ev's PR 4085 ruling directs ("step 5: the non-isosceles
overrun (a numeric probe before its spec) … split to their own rows when
step 4 lands"). The whole-face planar path that step 5 was also to delete
was already folded into the local carve at step 2 (that row's findings),
so nothing is left to delete.

## What refuses today

Two requested edges at a trivalent plane–plane vertex whose third edge L is
unrequested build the mitre only where `fillet3_turn_isosceles` reads the
trihedron isosceles (`crates/sweep/src/blend/battery.rs` `turn_at`). The
margin is the larger of the levered face-angle cosine difference and the
gap between the two bands' feet on L. A definite verdict refuses
`UnsupportedRunOut { TURN_NOT_ISOSCELES }`.

Users meet this at:
- the bracket's section-face rims (tour wall 3, `demos/tour/src/bracket.rs`):
  90° at the cap chord against 45° or 135° at the section edge;
- the sheared box's supplementary corner (φ, π−φ), where a chamfer's two
  feet coincide on L but the face angles differ (pinned in
  `band_planar_mitre.rs`);
- the leaning trapezoid prism (`common::operands::leaning_turn`).

## The ruling

"two [edges], the MITRE along the bands' intersection (line; planar
ellipse), with one more short curve where the trihedron is not isosceles."
The two bands' trimlines meet L at different points. The overrun is the
short curve that closes the gap between the feet, on one of L's faces.

## What the taker owes

1. **A numeric probe before any spec** (the ruling's order). On the three
   witnesses above, and on a sweep of face-angle pairs for both verbs and
   both convexities, measure:
   - where each band's trimline meets L;
   - which band overruns the other and onto which face;
   - the shape of the short curve (for the chamfer, a line on L's face?
     for the fillet, a section of which surface?);
   - whether the supplementary chamfer, whose feet coincide, needs any
     overrun at all.
2. Then a spec, through the designer protocol if the probe shows a fork:
   the overrun's carrier, its topology (a valence and a new role name),
   its clearance (which meter covers the overrun region), and the
   verdict's three-way split (isosceles mitre / overrun / in band).
3. The build, retiring the bracket's wall 3.

## Findings (step 5 probe)

The probe is `crates/sweep/tests/band_turn_overrun_probe.rs` (ignored;
run command in its module docs). It works from the cone alone, with no
battery code: the vertex's three edges, the inward normals of `S` (the
shared face, between `e₁` and `e₂`) and of `L`'s faces `F₁ ∋ e₁` and
`F₂ ∋ e₂`. It measures every case three ways: closed forms, the
trimlines and band surfaces intersected directly, and a grid over each
of `L`'s faces that counts what the union of the two band wedges takes
beyond that face's own band. All three agree on all 2250 sweep cases
(`φ₁, φ₂ ∈ 20°…160°` in 10° steps, dihedral `γ` at `L` ∈ {30°, 60°,
90°, 120°, 150°}, both verbs) and on the witnesses. The leaning prism
and the sheared box are read off their bodies
(`common::operands`); the bracket's corners come from its closed form.
Notation: `βᵢ` is the dihedral at `eᵢ` inside the cone, `φᵢ` the face
angle between `eᵢ` and `L`, `θ_S` the face angle between `e₁` and
`e₂`, and `h = sin φᵢ · sin βᵢ` (either `i`) the height over `S` of
unit length along `L`.

**Convexity does not change the geometry.** At a convex vertex the
cone is the material, and at a concave one (all three edges concave,
or it refuses `MixedConvexity`) it is the air. The kernel's setback
`r·√((1 − d)/(1 + d))` over the material's outward normals equals the
cone's `r·cot(β/2)` either way
(`the_setback_reads_the_cone_alone_at_either_convexity`). So every
row below holds for both convexities. The convex side cuts the
overrun region away; the concave side covers it with the far band's
fill.

**The feet** are distances along `L` from the vertex:
- chamfer: `tᵢ = d / sin φᵢ = d·sin βᵢ / h`;
- fillet: `tᵢ = r·(1 + cos βᵢ) / h`.

The gaps, with `Σ = β₁ + β₂` and `Δ = β₁ − β₂`:
- chamfer: `g = 2d·|cos(Σ/2)·sin(Δ/2)| / h`. The larger `sin β`
  reaches further, and the gap does not depend on `γ`.
- fillet: `g = 2r·|sin(Σ/2)·sin(Δ/2)| / h`. The smaller `β` reaches
  further.

**What the overrun is (both verbs).** Call `k` the band whose foot is
further and `j` the other. The mitre leaves `S` at the trimlines'
crossing `C` and always lands first on `Fⱼ`, at a point `q` on band
`j`'s trimline: 2030 of 2030 overrun rows. For a chamfer that is the
line `P₁ ∩ P₂`. For a fillet it is the ellipse in the bisector plane
(normal `e₁ − e₂`) through the axes' crossing, and it touches `Fⱼ`
there, since `Fⱼ` is tangent to cylinder `j`. From `q`, band `k` is
cut off by `Fⱼ` (band `j`'s far support) down to `L` at its own foot
`p_k`. The region this adds is `(R_k ∩ Fⱼ) \ R_j`, where `Rᵢ` is band
`i`'s wedge. It lies on `Fⱼ` alone and is bounded by three curves:
- `L` from `p_j` to `p_k`;
- band `j`'s trimline from `p_j` to `q`;
- the overrun curve from `q` to `p_k`.

`F_k` loses nothing beyond its own band (grid: zero on every row), and
`S` loses nothing beyond the two strips.
- **Chamfer**: the overrun curve is a line, `P_k ∩ Fⱼ`, the far
  band's plane cut by the near band's face of `L`. The region is a
  triangle similar to the cut-off sliver `(V, p_k, E)`, where
  `E = t_kS ∩ eⱼ`, with ratio `g / t_k`. Its area is
  `½·g²·sin φ₁·sin φ₂ / sin θ_S`, asserted against the shoelace on
  every row. Its angle at `p_k` (between `L` and the line) runs 9°–98°
  over the sweep. At `q` the band's boundary kinks 4°–103° from the
  mitre onto the line.
- **Fillet**: the overrun curve is an arc of `cyl_k ∩ Fⱼ`, an
  ellipse with minor `r` and major `r / |e_k · n̂ⱼ|`. It is a circle
  where `e_k ⊥ Fⱼ` (the bracket's `y = 0` corner), and its major runs
  up to `5.85 r` over the sweep. The arc is **tangent to `L` at
  `p_k`**, because `F_k` is tangent to `cyl_k` along `t_kF ∋ p_k`. So
  `Fⱼ`'s boundary is straight through `p_k` (interior angle π) and the
  region it loses is a cusp there. The arc is also **tangent to the
  mitre at `q`**: both lie in band `k`'s tangent plane and in `Fⱼ`.
  Band `k`'s boundary therefore runs G1 from `C` through `q` to `p_k`
  as two conic arcs. The finite-difference residue is ≤ 1° at both
  ends.
- **At the coincident feet** both ends of the mitre are the common
  foot on `L`, to 1e-9. There is no overrun, the topology is the
  isosceles one, and the foot has valence 4. Away from that, `L` ends
  at `p_k`, which has valence 3 (`L`, `t_kF`, the overrun). `q` also
  has valence 3 (the mitre, `t_jF`, the overrun). The isosceles
  valence-4 foot splits into two trivalent vertices, so blending `L`
  in a later call would not meet the `NEdgeVertex { valence: 4 }`
  refusal there.

**Regimes**, the same at every `γ`:
- the isosceles line `φ₁ = φ₂` (15 cases per `γ` per verb): no
  overrun.
- the chamfer's coincident-feet line `φ₁ + φ₂ = π` (`Σ = π`, the
  supplementary corner, 14 cases): no overrun for the chamfer, and the
  chamfer's overrun **swaps faces** across this line. The fillet
  overruns here like anywhere else.
- everywhere else, an overrun on the near band's face of `L`. **The
  verbs overrun opposite bands wherever `β₁ + β₂ < π`**: 98 of the 196
  rows per `γ` where both overrun, with zero exceptions to the rule.
  The bracket's `y = 0` corner is one: the chamfer overruns from the
  chord onto the side wall, the fillet from the section edge onto the
  cap.
- no other regime appears in the sweep. In every case the mitre lands
  on the near face, and the grid's count matches the three-curve
  region's area.
- finite-geometry regimes are a different matter, because the gap is
  not bounded by the band. Chamfer `g/d` reaches 1.92 at
  `φ = (20°, 90°)`. Fillet `g/r` reaches 5.5 at `γ = 90°` and 20.5 at
  `γ = 30°` (`φ = (20°, 160°)`), with region areas up to `32 r²`. The
  far foot can therefore pass `L`'s far end, or the region can reach
  `Fⱼ`'s other features, well before the band itself meets anything.

Signed `g/d` at `γ = 90°`. Positive means `e₁` reaches further and the
overrun is on `F₂`. The chamfer's table is the same at every `γ`; the
fillet's grows as `γ` shrinks (`γ = 30°` is about 3.7× the values
below):

| φ₁ \ φ₂ | 20 | 40 | 60 | 80 | 100 | 120 | 140 | 160 |
|---|---|---|---|---|---|---|---|---|
| chamfer 20 | 0 | 1.37 | 1.77 | 1.91 | 1.91 | 1.77 | 1.37 | 0 |
| chamfer 60 | −1.77 | −0.40 | 0 | 0.14 | 0.14 | 0 | −0.40 | −1.77 |
| chamfer 100 | −1.91 | −0.54 | −0.14 | 0 | 0 | −0.14 | −0.54 | −1.91 |
| fillet 20 | 0 | −1.56 | −2.17 | −2.57 | −2.92 | −3.32 | −3.94 | −5.50 |
| fillet 60 | 2.17 | 0.61 | 0 | −0.40 | −0.75 | −1.15 | −1.77 | −3.32 |
| fillet 100 | 2.92 | 1.37 | 0.75 | 0.35 | 0 | −0.40 | −1.02 | −2.57 |

**Witnesses** (`d = r = 0.1`). Edge 1 is the cone's first requested
edge as the probe orders it:

| corner | (φ₁, φ₂, γ) | (β₁, β₂) | chamfer: gap/d, onto, area/d² | fillet: gap/r, onto, area/r², overrun curve |
|---|---|---|---|---|
| bracket `y = 0` (chord, section edge) | 45, 90, 90 | 90, 45 | 0.414, side wall, 0.061 | 1.000, cap, 0.107, circle `r` |
| bracket `y = 1` | 135, 90, 90 | 90, 135 | 0.414, side wall, 0.061 | 1.000, side wall, 0.091, ellipse `(1, 1.414) r` |
| sheared box `s = 0.3`, supplementary | 106, 74, 85.3 | 73.3, 106.7 | 0 (mitre on `L`) | 0.624, `F₂`, 0.035, ellipse `(1, 1.044) r` |
| sheared box `s = 0.3`, isosceles | 106, 106, 94.7 | 106.7, 106.7 | 0 | 0 |
| leaning prism `s = 0.5` | 116.6, 90, 90 | 90, 116.6 | 0.118, leaning wall, 0.006 | 0.500, leaning wall, 0.017, ellipse `(1, 1.118) r` |
| leaning prism `s = −0.5` | 63.4, 90, 90 | 90, 63.4 | 0.118, leaning wall, 0.006 | 0.500, end face, 0.018, circle `r` |

At the bracket's `y = 1` corner, `L` runs 0.25 to the fillet's
tangent point, and both verbs' far foot (0.141) lands inside it.

**What the existing meters cover.**
- The **reach meter** covers the overrun's volume. Band `k`'s reach is
  closed at the vertex by every non-support face's plane there, so by
  `Fⱼ`'s ("at a mitre, the other band's support", `blend::reach`).
  That region is `R_k` itself, overrun included. It skips the faces at
  the vertex, however, so nothing on `Fⱼ` is its concern.
- **Arm (a)** does not cover it. Band `j`'s arm meters `Fⱼ`'s rings
  against `t_jF`, and passes a ring on the far side of that line,
  which is where the overrun lies. Band `k`'s arm meters only `S` and
  `F_k`. A ring on `Fⱼ` inside the triangle passes both and would be
  cut through.
- **Arm (d)** does not cover it. It meters `Fⱼ`'s outer edges against
  band `j`'s strip rectangle, and the overrun is outside that
  rectangle.
- **Arm (c)** would cover it, if run for band `k` with `Fⱼ` as its end
  face. Its sliver `R_k ∩ Fⱼ` is the overrun plus a piece of band `j`'s
  strip, which (a) and (d) already refuse anything inside. So it adds
  no refusal of its own beyond the overrun. The step-4 note that "the
  other band's cut by this face's plane … lies inside that strip"
  holds exactly when the feet coincide, and the overrun is precisely
  what is left outside the strip otherwise.
- `shared_rims_clear` meters the turn foot against other splits of
  `L`. That has to read `p_k`, and `p_j` is no longer a vertex.

**Forks for the spec** (not picked):

1. *The supplementary chamfer.* (a) Build it as the mitre: the probe
   puts both mitre ends on the common foot, with no area on either
   face, so the topology is the isosceles one. (b) Keep refusing it,
   or treat it as a zero-length overrun. For (a): the geometry is
   exactly a mitre. For (b): it is a second coincidence decided from
   values (`sin φ₁ = sin φ₂` with `φ₁ ≠ φ₂`), which no symmetry of the
   trihedron proves, so `IsoscelesTurn` and INTENT's structural proof
   do not cover it. The fillet at the same corner overruns, so the two
   verbs would build different topology there. Under (a) the verdict
   is four-way near the line: overrun onto `F₁`, mitre, overrun onto
   `F₂`, and the band between them.
2. *What the three-way verdict reads.* (a) The per-verb feet gap,
   which is what decides the built topology. Its zero set is the
   isosceles line, plus the supplementary line for the chamfer. Its
   sliver is the overrun's short edges (`|p_k − p_j|` on `L`,
   `|q − p_k|`), all proportional to `g`. (b) The trihedron's face
   angles, which are verb-independent and what `IsoscelesTurn`
   records, with the gap only picking the side. Under (b), a definite
   angle verdict near the chamfer's supplementary line still names an
   overrun shorter than the band. The current margin is the larger of
   the two readings.
3. *Where the overrun's footprint on `Fⱼ` is metered.* (a) Arm (c),
   with band `k` cut off at `Fⱼ` over its whole sliver. It is closed
   form, it reads the ellipse already, and it adds no refusal (see
   above), but it names the refusal as a cut-off's. (b) An exact
   region: the triangle, or `L` plus the trimline plus the elliptic
   arc, as a new arm or as arm (d) widened past band `j`'s strip. This
   is tighter only inside band `j`'s strip, where nothing survives
   anyway.
4. *How much of the cut-off the overrun is.* The probe shows the
   overrun is band `k`'s CUT-OFF at `Fⱼ` (same carrier, same
   `fillet3_cap_transverse` circle-or-ellipse choice, same foot on
   `L`), clipped at the mitre's end `q`. The sweep README names it
   that way already ("that band's `EndArc` and its end `FootVertex`").
   (a) Build it through the cut-off's path, `end_face.rs`'s `mef` from
   `q` to `p_k`. (b) Give it a role and carve of its own. Against (a):
   a cut-off's arc runs between feet on two rims, while this one
   starts at an interior point of `Fⱼ`, the mitre's end on a
   trimline. For the fillet, `q` is a G1 joint, so a margin on the
   mitre's kink there would read zero by construction.

Unsure: the bracket's corners are read from its closed form, not built
(the tour is outside `sweep`'s tests). Which requested edge counts as
"1" at a body's corner is the order of `topo::query::all_edges`; the
tables name the faces as well, so the rows read without it.
