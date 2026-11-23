---
id: a-non-isosceles-turn-overruns-past-the-mitre
kind: issue
title: blend: two requested edges at a non-isosceles trivalent corner refuse TURN_OVERRUN, where Ev's ruling builds the mitre plus one short overrun curve
status: closed
opened: 2026-10-07
priority: P2
cost: H
pr: 4962
closed: 2026-10-13
---


## Ruled (Ev, PR 4245, 2026-10-12)

"this design sounds good!" — after a conditional yes (2026-10-08) on the
limits near isosceles and at the T-joint, consistency with other CAD and
with the design principles, which the orchestrator checked numerically on
the PR (with the box-corner limit corrected after PR 4844's reviews: the
chamfer's chord is the gap times a corner-fixed ratio, both quadratic at a
box corner). The design is `crates/sweep/README.md`'s run-out paragraph:
the far band `k` is cut off past the mitre by the other band's face of L,
decided on the gap and the chord. Fork-log row 121. Build order: (1) the
verdict and its coincidence record (PR 4844); (2) the chamfer's overrun;
(3) the fillet's overrun.

Split from `a-plane-plane-blend-cannot-end-at-an-unrequested-corner` at its
step 4 (PR 4209), as Ev's PR 4085 ruling directs ("step 5: the non-isosceles
overrun (a numeric probe before its spec) … split to their own rows when
step 4 lands"). The whole-face planar path that step 5 was also to delete
was already folded into the local carve at step 2 (that row's findings),
so nothing is left to delete.

## What refuses today

Two requested edges at a trivalent plane–plane vertex whose third edge L is
unrequested build the mitre where the two bands' feet on L are one point
(`crates/sweep/src/blend/battery.rs` `turn_at`, the gap and the overrun
chord both Zero; step 1 below), and a chamfer builds its overrun where
they stand definitely apart (step 2). A fillet's feet definitely apart
refuse `UnsupportedRunOut { TURN_OVERRUN }` at the far band's edge.

Users meet the fillet's refusal at:
- the bracket's section-face rims (`demos/tour/src/bracket.rs`, whose
  chamfer now builds and renders): 90° at the cap chord against 45° or
  135° at the section edge;
- the sheared box's supplementary corner (φ, π−φ), where a chamfer's two
  feet coincide on L, so the chamfer mitres (step 1) and the fillet
  overruns (pinned in `band_planar_mitre.rs`);
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
  *Note (2026-10-07, PR 4254):* that skip now reads "the faces band
  `k`'s window is capped by" (`straight_reach` records them), and it
  is sound only while `Fⱼ` lies on that cap. If the overrun's build
  re-caps `R_k` anywhere but `Fⱼ`'s plane (at the mitre plane, say),
  or carries band `k`'s material past it, `Fⱼ` leaves the reach's
  boundary: meter `Fⱼ` past the cap, or keep the cap at `Fⱼ`.
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
  strip, which (a) and (d) already refuse anything inside. Its region
  is a bound, though, not the sliver itself: run, it refuses features
  past the overrun chord and band `j`'s trimline as well, and every
  feature it refuses in the overrun the reach refuses too
  (`a-turns-overrun-meters-the-whole-cut-off-sliver`). The step-4 note that "the
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
   form and it reads the ellipse already, but it names the refusal as a
   cut-off's, and its bound refuses features the overrun clears (see
   above). (b) An exact
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

## Step 1 — the verdict and its record (PR on `band/turn-overrun-verdict`)

`fillet3_turn_isosceles` (the levered face-angle cosines against the
feet's distance) is gone. `turn_at` reads two lengths from the links
alone (`turn_reading`), both signed positive where the first link's foot
on `L` is further:
- `fillet3_turn_feet_gap`: `(y₁ − y₂)·L̂`, each foot its trimline on its
  face of `L` met with `L`;
- `fillet3_turn_overrun_chord`: `|p_k − q_j|`, where `q_j` is band `j`'s
  trimline on `Fⱼ` met with the mitre's plane (band `k`'s plane for a
  chamfer; for a fillet the plane through the axes' crossing, normal
  `e₁ − e₂`). A Zero gap reads the larger of the two bands' chords.

`turn_verdict` classifies the gap, then the chord on the side the gap's
sign picks. Both Zero: the mitre, recorded as
`Relation::CommonFoot` (renamed from `EqualAngles`; the margin the gap's
`jointly` the chord's). Both definite: `UnsupportedRunOut { at: Edge(far),
TURN_OVERRUN }` (renamed from `TURN_NOT_ISOSCELES`), naming the far band;
the side is the other band's face of `L`. A reading in band escalates as
itself; two readings decided on different sides of the band escalate
through the funnel's straddle door as `fillet3_turn_feet`. All three share
`FILLET3_TURN_RECOURSE`, which offers the two ends that build today (the
feet meeting, or `L` requested too for the patch); its middle leg, feet
clearly apart, joins it when step 2 builds the overrun.

**What moved.** The sheared box's supplementary chamfer now builds (tier
3, naming total, ΔV the two chamfer half-spaces' to 1e-9, its
`CommonFoot` row recorded). Its fillet overruns and refuses. Nothing
that built before refuses: every mitre row in `band_planar_mitre` passes
unchanged. The concave supplementary corner (the sheared box cut up
into a block) reads the convex one's readings and builds its chamfer too.
At the door the supplementary row is unproven, as the box's
(`coincidence_door::a_supplementary_chamfer_turn_is_recorded_unproven`):
neither reading reduces at Sym.

**Measured** (`crates/sweep/tests/band_turn_overrun_verdict.rs`,
`d = r = 0.1`, band `(1e-9, 1e-8)`):
- The readings equal closed forms to `1e-10` on all 294
  corners of `φ₁, φ₂ ∈ 30°…150°` (20° steps) at `γ ∈ {60°, 90°, 120°}`,
  both verbs. Every corner lies clear of the band: the chamfer mitres on
  both lines (39), the fillet on the isosceles line (21), and every other
  corner overruns, naming the band whose foot is further. A chamfer's
  chord is held to the identity `chord = gap · sin φⱼ / sin θ` (the
  triangle on the near face), which shares no construction with the
  battery's.
- Escalate band (skew from the coincidence, sampled from `1e-6°` at
  six per decade): the fillet at the box corner and both verbs at
  `(60°, 60° + skew)` already escalate at `1e-6°` and overrun from
  `5e-6°…1e-5°`. At the box corner the chamfer's gap is `d·s²/2` in
  the lean and its chord `√2` times it, so it mitres to `≈0.0068°` (the
  chord's Zero edge), escalates, and overruns from `≈0.0256°` (the
  gap's definite edge); the row brackets both edges within one step of
  a 24-per-decade grid.
- The needle: `e₁ = x̂`, `e₂` at `180° − ε` in the shared face, `L`
  along `(0, 0.5, 1)`. The chamfer's gap closes as `ε²` and its chord as
  `ε`: overrun at `1°` and `0.1°`, the gap in band at `0.03°`, and at
  `0.01°` the gap Zero (3e-10) under a chord of 1.7e-6, which escalates
  as the straddle. The gap alone would build a mitre there. No ε builds
  for either verb. Below about `0.003°` the fixture's own Boolean
  escalates.
- `Interval` replays (the leaning box corner at four leans and the
  sheared box's supplementary corner, both verbs): every reading's
  enclosure holds the `f64` reading, under `1e-12` wide (measured
  under `1e-13`), and the verdict at `Interval` is the `f64` one.

**Unsure.** The straddle door's row (`fillet3_turn_feet`) is charged to
neither reading's predicate, so the k-lint sees it as its own name. The
bracket's corners are read from its closed form in `sweep`; the tour's
wall 3 now pins `TURN_OVERRUN`.

## Owed by step 2 (orchestrator, 2026-10-12)

`FILLET3_TURN_RECOURSE` dropped its "move them clearly apart" leg in step
1 because the overrun refuses today; the README's three-leg recourse is
the ratified end state. When the chamfer's overrun builds, restore the leg
for the chamfer (and its followability row), and make the README's
"both definite builds the overrun" true per verb as each verb lands.

## Step 2 — the chamfer's overrun (PR 4926)

Both convexities build where both readings are definite and the bands
are planes; a fillet's overrun still refuses `TURN_OVERRUN` (its text now
names the fillet).

- **The verdict's record.** `Turn` carries `run_out: TurnRunOut` —
  `Mitre(Coincidence)` where the feet coincide, `Overrun(Overrun { far,
  end })` where they stand apart, `end` the mitre's end `q` (band `far`'s
  plane met with the near band's trimline on its face of L, the point
  the chord reading already measured). `Turn::foot` is then the far
  band's foot `p_k`, and `Turn::foot_on` gives each face of L its
  station's foot (`q` on the near band's face). An overrun records no
  coincidence.
- **The carve** (`open/planar.rs`): L split at `p_k` as at any turn;
  `overrun_cut` `mev`s the far band's cut-off chord from `p_k` out to `q`
  on the near face (described `Chord`, named the far band's `EndArc`,
  `q` its `FootVertex` on that face), so the near band's trimline there
  runs from `q`; `turn_fusion` cuts the mitre across the NEAR band from
  the crossing to `q` and folds the quadrilateral it cuts off (the
  triangle and the overrun) into the far band. Every vertex is
  trivalent; the mitre case is unchanged.
- **Arm (c).** `turn_plan` plans the far band's cut-off at the near face
  through `cut_off_plan` (`EndSection::Chord`) and hands its sliver to the
  ring pass, with the far band's convexity. It answers first for a
  feature in the overrun, but the reach refuses every such feature too,
  and its bound refuses some features the overrun clears (filed:
  `a-turns-overrun-meters-the-whole-cut-off-sliver`).
- **PR 4254's note, resolved: the cap stays at `Fⱼ`.** The far band's
  material is cut by `Fⱼ`'s plane from `q` down to `p_k`, so the reach's
  cap at the vertex is still that plane, `Fⱼ` lies on it, and the skip
  holds; a feature in what the band takes of `Fⱼ` is the reach's to
  refuse, and arm (c)'s (the reach's module docs and arm (d)'s say so).
  No reach code changed.
- **Recourse.** `FILLET3_TURN_RECOURSE` has its middle leg back, for a
  chamfer ("move them clearly apart (a larger setback, or a more
  asymmetric corner)"); its followability row builds the leaning prism
  at `0.5`. The README's turn clause says the chamfer builds and the
  fillet refuses.

**Rows** (`crates/sweep/tests/band_turn_overrun_chamfer.rs`, `d = 0.1`).
The closed form is the half-space one: a chamfer of a convex polytope's
edge takes its side of the plane through the edge's two trimlines, so a
turn built as ruled leaves `B ∩ H₁ ∩ H₂` (concave: fills the air `A`
less `A ∩ H₁ ∩ H₂`), read by vertex enumeration over the input's planes.
- The witnesses — the bracket's two corners (the leaning prism at `∓1`,
  `(45°, 90°)` and `(135°, 90°)` at `γ = 90°`), the leaning prism at
  `±0.5`, the skewed box (`parallelepiped(0.3)`'s top and lateral edges
  at `(s, s, 1)`, `90°` against `≈106.7°`) — build tier 3 and 3′, naming
  total, `ΔV` to `1e-10`, the turn foot and the mitre's end trivalent at
  the probe's `p_k` and `q`, one edge between them, the far band's
  `EndArc`; at six rigid poses to `1e-9`; at `Interval` (leaning `±0.5`,
  the skewed box), the volume enclosure bracketing the closed form.
- Concave: the leaning prism as a pocket's ceiling corner and the skewed
  box as a void, `ΔV` the air's to `1e-10`.
- The sweep: every overrun corner of `φ₁, φ₂ ∈ 30°…150°` (20° steps) at
  `γ ∈ {60°, 90°, 120°}` builds convex (108 corners), and the 40°-step
  subset as voids (24), each at its `ΔV` with its cells where the probe
  puts them.
- The sliver meter: at the bracket's `y = 0` corner a boss standing in
  the overrun triangle refuses at a clearance meter (arm (c)'s
  `RingClearance` at the leaning wall first, the reach's `FaceClearance`
  without it); the same boss at `(2d, 0.1d)` builds at the boss-free
  `ΔV`.
- L chamfered later at the overrun's turn foot, at leans `±0.5` and
  `±1`, refuses `FaceClearanceUncertified` at the first setback and
  builds at a tenth of it, at its own half-space's `ΔV` over the first
  chamfer's body.
- `band_turn_overrun_oracles.rs` (the review's two oracles, sharing
  nothing with the vertex enumeration: a clipped cube over the
  construction's own half-spaces, and the operand less bisector slabs
  through the Boolean): the bracket's corner against a hand-integrated
  closed form; leaning prisms over 7 leans, 3 setbacks, 3 scales and
  both orders; 7 generic hexahedra and 2 as voids; whole rims (68
  builds; both rims of a `0.2`-tall prism refuse at a clearance meter);
  two turns on one L (23 of 27 build, the rest refuse typed); the
  bracket's corners at `Interval`; features in and around the overrun;
  a void over it (the reach's `WhollyInside`); and the two residues
  below, pinned.
- The near-T needle still escalates (`band_turn_overrun_verdict`); the
  verdict rows read the overrun's far band off the built record.

**What moved.** Every chamfer overrun that refused `TURN_OVERRUN` now
builds. Rows that used the leaning prism's chamfer as a shared refusal
(`blend6_verb_vocab`, `review_blend6_r2_probes`) now use its in-band
lean (`operands::leaning_turn_in_band`), which escalates for either
verb. The verdict's witness rows read the bracket's corners off the
leaning prism: `corner()`'s `(135°, 90°)` cone puts a valence-4 vertex
at `e₁`'s far end, which the battery now reaches. The tour's bracket
retires wall 3: both section faces' whole rims chamfer, held to a closed
form (`bracket::rim_taken`), and the scene renders that body.

**Unsure / residue.** The overrun's sliver is the far band's whole
cut-off sliver, so its foot on the near band's requested edge must lie
inside that edge (`FOOT_INSIDE_A_FACE` otherwise), which a short near
edge at an obtuse shared-face angle can violate where the overrun itself
fits (filed: `an-overruns-sliver-refuses-a-short-near-edge`, executed in
the fix pass). The same whole sliver's bound refuses a boss the overrun
clears, and adds no refusal the reach does not make (filed:
`a-turns-overrun-meters-the-whole-cut-off-sliver`). A just-definite
overrun builds a chord barely longer than the band's `escalate` width
(`1.46e-8` at a lean of `0.026°`), by design.

## Step 3 — the fillet's overrun (PR 4962)

Both verbs and both convexities now build where both readings are
definite. `TURN_OVERRUN` is gone.

- **The record.** `Overrun` carries `section`: the far band's section by
  the near face, `Chord` for a chamfer, and for a fillet what
  `cap_transverse` picks from `Fⱼ`'s plane and band `k`'s axis (the
  cut-off's own picker, margin and ellipse door).
- **The carrier.** `turn_plan` plans band `k`'s cut-off at `Fⱼ` in that
  section. `overrun_cut` describes the overrun edge with that cut-off's
  carrier (`Transverse`, a circle or an ellipse), which the description
  pass windows from `p_k` to `q` and certifies as any cut-off arc. `q`
  lies on it exactly (on `t_jF ⊂ Fⱼ` and on cylinder `k`). No new carrier
  kind and no new role. The design reports' final state ("k's cut-off at
  Fⱼ clipped at q", same carrier) and the probe's G1 finding left no
  choice open.
- **Recourse.** `FILLET3_TURN_RECOURSE`'s middle leg names both verbs,
  and says a round band's end ellipse may then escalate with its own
  recourse (the chamfer's text never says "fillet"). Its followability row runs the
  fillet's legs at a `60°` corner skewed into the turn's band, and at a
  box corner leaning its wall, where a larger setback or a slightly more
  asymmetric corner escalates `CapEllipse` and that recourse's legs
  build (tilted past the ellipse's edge, the overrun; squared, the
  mitre). `FILLET3_CAP_ELLIPSE_RECOURSE` says squaring at a turn may
  bring the feet together instead, and `FILLET3_CORNER_RECOURSE` names
  any two edges whose feet meet or stand clearly apart.

**Rows** (`crates/sweep/tests/band_turn_overrun_fillet.rs`, `r = 0.1`).
The oracle is `B ∩ (W₁ ∪ W₂)`, each `Wᵢ` the infinite prism over the
kite less the ball's sector. Each band is computed in closed form (area
and first moment); the overlap by globally adaptive Gauss–Kronrod over
band 1's section, the chord along band 1's edge read in closed form.
The oracle matches a hand form at the box corner
(`(1 − π/4)r²·3.5 − (5/3 − π/2)r³`) to `1e-12`, and matches the
kernel's box mitre.
- **Witnesses**: the bracket's corners, the leaning prism at `±0.5`,
  the skewed box, and the sheared box's supplementary corner.
  - Tiers 3 and 3′, naming total, trivalent, `ΔV`'s midpoint at the
    oracle within `midpoint_tol` (measured within `3.9e-9`) and its
    enclosure under its ceiling, each on its own.
  - `p_k` and `q` where the probe puts them; the arc the probe's circle
    or ellipse, tangent to the mitre at `q` and to L at `p_k`.
  - Also at six poses and at `Interval` (leaning `±0.5`, `±1`).
- **Concave**: the pocket ceiling and the void.
- **The sweep**: 126 convex corners and 36 concave ones.
- **Leaning prisms** over leans, radii, scales and orders, and generic
  hexahedra. A trimline running into a face before its end face refuses
  `FOOT_INSIDE_A_FACE`, read off the oracle's section.
- **Features** in and clear of the overrun, and about its arc.
- **A second model** (`band_turn_overrun_fillet_model.rs`, from a
  review's probe): spines solved from the planes, bands by Cartesian
  quadrature, the overlap along a generic direction. Every witness
  builds at it; the built body agrees with it point by point, convex and
  concave; the overrun arc runs the `f64` way at `Interval`.
- **A short near edge** refuses `FOOT_INSIDE_A_FACE` as the chamfer's
  does (the filed residue), and **the needle** builds at the oracle to
  `1°`, then refuses at the turn's far foot.
- **Escalations**: the in-band leans escalate (the leaning prism on
  `CapEllipse`, the `60°` corner on the turn readings). L filleted later
  refuses `END_FACE_CURVED`.

**What moved.** Every fillet overrun that refused builds. Near a box
corner, on the side where the far band is cut off at the leaning wall
(a positive lean), the fillet's overrun escalates `CapEllipse` past its
turn band, to `sec θ = 1 + escalate/r` (`≈0.0256°` at `r = d`), as a
plain cut-off at that tilt does. On the other side the far band is cut
off square, in a circle, and the overrun builds from just past the
turn's band (`≈1e-5°`). The window row brackets the first edge and pins
the second side.

**Filed**: `an-interval-fillet-cut-off-at-an-oblique-end-reads-its-arc-backward`.
At `Interval` the skewed box's top edge filleted alone refuses
`BodyNotIntact` where `f64` builds, on main too. The fillet turn there
refuses the same way, pinned. The two step-2 residues stay open, now
covering the fillet too: a short near edge refuses the fillet the same
way, and arm (c) adds no refusal the reach does not make for it either,
though its round sliver does not over-refuse at the bracket's corner.

