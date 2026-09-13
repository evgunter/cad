# TRIM-2 — volume, area and tessellation of a face carrying a `General` pcurve

Item `work/trim/general-pcurve-face-props-and-tess-refuse.md` (#1179).
Branches `trim/2-trim-quad` (PR-1) and `trim/2-trim-tess` (PR-2).
Difficulty pre-logged **M** (PR-1 M, PR-2 S — argued under "PR shape");
task class **NUMERIC** (a new certified quadrature, ε-shaped acceptance).
Survey run 2026-09-13 against main `a7532e677`, every cite re-derived by
symbol on that head; the fixture measurement in §0 was RUN on it.

Jargon, once. A *chart* is a face's carrier surface `S : R = [u₀,u₁]×[v₀,v₁]
→ ℝ³`; a *pcurve* is an edge's image `P : [t₀,t₁] → R` in that chart,
stored with a certificate whose `envelope` bounds `sup_t |S(P(t)) − C(t)|`
in metres against the certified carrier `C` (`PcurveCertificate::envelope`,
`geom-brep/src/pcurve_cache.rs`). The *trim region* `Ω ⊂ R` is what the
face's loops bound in the chart. `Pcurve::General` is a 2-D NURBS image
certified at the Fitted grade (`PcurveCache::certify_general`); on this
head it is minted at exactly one site, `nurbs_iso_derive`'s `Intersection`
arm in `topo/src/pcurves.rs` (`derive_general_image`), for a spline
carrier on a spline chart whose image is not a boundary column. The
*flux* of a face is `∫∫_Ω ⟨S, S_u × S_v⟩ du dv` — its divergence-theorem
volume contribution; the *area* is `∫∫_Ω |S_u × S_v| du dv`.

## 0. The fixture (measured — the unit's frontier)

**No public `sweep` door builds a face carrying a `General` pcurve.**
`General` needs a spline chart, an `Intersection`-described edge with a
`Nurbs` carrier and an image off the chart's boundary columns
(`derive_general_image`'s `(Curve3::Nurbs, spline_chart)` precondition;
the `Intersection` arm's four-candidate schedule first). `loft_body`
describes every wall–wall seam `EdgeDescriptionSpec::iso` on the wall's
own `u ∈ {0,1}` column and never upgrades a rim to `Intersection`
(`sweep/src/loft.rs`, Phase 6); `revolve` upgrades rims to
`Intersection` (`revolve/upgrade.rs`) but mints no `Surface::Nurbs`.
The only at-rest producer today is the P-2 restatement route: a loft, one planar wall restated as its plane
(`Body::set_face_surface`), the seam re-described `Intersection`
(`set_edge_curve_nurbs_lane`), the bowed wall re-charted WIDER than the
face and the whole body re-minted (`mint_pcurves`). STEP import of a
trimmed NURBS is the other producer and has no fixture. **That is the
first finding, and the unit re-scopes to the fixture that can:** the
P-2 body of `m8_4_intersection_iso.rs::an_interior_column_intersection_mints_a_general_image`
with the widening re-cut to degree 2.

**The re-cut.** The loft wall's `u` net is degree 1 with two columns
(`widened_u_chart`'s assertion). The degree-2 widening keeps the
geometry bit-for-bit and removes the crease: knots `[0,0,0,1,2,3,3,3]`,
five columns at the Greville abscissae `ξ = (0, ½, 3/2, 5/2, 3)` placed
by linear precision, `c_i(v) = a(v) + (ξ_i − 1)(b(v) − a(v))`, so
`S(u,v) = a(v) + (u − 1)(b(v) − a(v))` exactly (Σ ξ_i N_i(u) = u), the
face on `u ∈ [1,2]` is the same surface, both seams are interior
columns, every interior knot is simple (multiplicity 1 ≤ p − 1) and
`patch_bound::check_direction` admits it. The `v` direction is the
loft's own degree-2 single span. Scale `1/1024` as the row has it (the
attachment's certified sup is a length and must sit inside ε at every
cell).

**Measured on that body (a scratch row beside
`an_interior_column_intersection_mints_a_general_image`, run at
`CAD_TOLERANCE_EPS ∈ {1e-6, default 1e-9, 1e-12}`, `f64` lane; the
`Intersection` seam here is the `x = +s` bowed wall's, the mirror of
the row's):**

| lane (`f64`) | ε = 1e-6 / 1e-9 / 1e-12 (identical at all three) |
| --- | --- |
| `mint_pcurves` / `validate_pcurves` | `Ok`, 0 findings. The `Intersection` seam mints `General` (degree 1, 33 control points, unit weights — `edge_nurbs::PXN_IMAGE_DEGREE = 1`, `PXN_FIT_SAMPLES = 33`), image on `u = 1`, control box `u ∈ [1 − 1.1e-16, 1]`, envelope `3.857e-14 m`, statement `MapResidualComposite`; the `Chart` seam `IsoLine` on `u = 2`; the cap rims `IsoLine` on `v ∈ {0, 1}` with `pl.x = 512` over `t ∈ [0, 1/512]`. The trim region is `[1, 2] × [0, 1]` of `[0, 3] × [0, 1]`. |
| `mass_properties` | `MassPropsError::Face { source: QuadratureUnsupported { what: "a NURBS-face half-edge carries a non-iso pcurve — …" } }` — `props.rs::quad_lane::nurbs_face`, the non-iso site. |
| `mesh::tessellate` (`chordal = 1e-5·scale`) | `TessellateError::UnsupportedCurve { edge, note: "NURBS-face half-edge carries a GENERAL curve-in-UV pcurve — no certified UV speed bound …" }` — `chords.rs::nurbs_tighten`'s `General` arm. The crease gate is gone. |
| `replace_face_offset(bowed, scale/16)` | 1e-6, 1e-9: `FittedBoundaryUnsupported { edge, what: "a chart image of a neighbour's chart" }`; 1e-12: `Fit { BudgetExhausted { achieved 2.86e-10, tolerance 1e-12 } }`. **The ORACLE prism's own bowed wall (no restatement, no `General`) answers the same variant at every cell** (`FittedBoundaryUnsupported` / `Fit { BudgetExhausted { achieved 6.69e-10 } }`). |
| oracle `mass_properties(prism)` | `volume ∈ [7.45058059692374e-9, 7.450580596923934e-9]` (closed form `8·(1/1024)³ = 7.450580596923828e-9`), `surface_area 2.4022e-5` — E1's oracle. |
| oracle `tessellate(prism)` | `Ok` (143 360 vertices, 6 patches) — E2's oracle. |
| degree-1 widening (control) | props: the same non-iso site; tessellation: `UnsupportedNurbsFace { "degree-1 NURBS direction with interior knots (a C⁰ crease) …" }` (TRIM-1's measurement reproduced); offset: `Fit { PatchBound(Degree1Crease) }`. |
| `chart_boundary` on the widened chart at `f64` | `Ok`: one loop of three `Segment`s and one `Envelope` with `slack 3.857e-14`, `image` and `hull` poison (NaN) — the point-scalar reading its doc promises; the description is an interval-lane object. |

Read against the item's six static sites: quadrature reaches the
`props.rs` **non-iso** site (one of the six); tessellation reaches the
`chords.rs` `General` arm (one of the six) — the chord pass runs before
any face lane (`tessellate.rs`, `compute_chords` precedes the face
loop), so `trimmed.rs`'s `General` arm (the sixth) is reached only once
`chords.rs` admits the class. The other three `props.rs` sites the item
lists are the rectangle certificate's own and are not on this trace;
the sites P-2's spec named in `chart_region.rs` and `replace_face.rs`
do not exist (the item already records that). **Offset is not this unit's frontier**:
`replace_face_offset` on a described NURBS face fits an `Approx` offset
and then refuses its boundary at `ReplaceFaceError::FittedBoundaryUnsupported`
(`replace_face.rs`, the `matches!(new_surface, Surface::Approx(_))`
arm) — the oracle prism's own bowed wall, with no `General` anywhere,
refuses identically (table). The item's offset claim is a premise the
survey refutes; the class is SHELL's
`work/shell/no-approx-faced-body-is-both-movable-and-valid.md` (an
`Approx` face's boundary has no route), and PR-1 corrects the item and
adds this measurement to that file rather than opening a second.

## What the survey refuted (binding premise corrections)

1. **`props/quad.rs` is not `topo`'s and `props.rs` has eight sites, not
   four.** The quadrature math is `geom_brep::props::quad` (PROPS's
   file, `work.py territory`); `topo/src/props.rs::quad_lane::nurbs_face`
   is the consumer and carries eight `QuadratureUnsupported` returns
   (placeholder, endpoint-not-exact, no-cache, non-iso, diagonal,
   carrier-inventory, vertex-inside, shoelace). The item's four are the
   middle of that list. Only the non-iso site is REACHED by a `General`
   edge; the other seven are the rectangle certificate's own and stay.
2. **The patch lane is an AREA rule on the chart rectangle, not a
   Green boundary integral.** `nurbs_patch_face_rounds` runs
   `patch_flux_exact` (tensor closed Newton–Cotes of order `3p` per
   knot span, exact for the bidegree-`(3p−1)` integrand
   `f = ⟨S, S_u × S_v⟩`) and `area_midpoint_taylor` (midpoint +
   Lipschitz pad on knot-aligned cells). `bspline_green_integral` is a
   1-D `∫ u v′ dt` for the CYLINDER chart's Green form and has no
   NURBS-patch consumer (its own doc). Nothing on the head integrates
   over a sub-region of the rectangle.
3. **`chart_bound.rs` cannot be the quadrature's region.**
   `MetredBound::certifies_outside` is one-sided by contract ("every
   rounding keeps the cell"), skips two-vertex loops, and decides per
   cell through the K funnel; a quadrature needs a certified INSIDE as
   well, and a cell decomposition against it is first order at the
   boundary (§1 (ii)), so a curved trim cannot reach the `1024·ε`
   target through it. It stays read-only.
4. **`chart_box` is whole-domain.** `Pcurve::chart_box(t0, t1)` on a
   `General` ignores `t0, t1` and returns the control net's box; a
   per-piece box needs the curve refined (`NurbsCurve2::refine_knots` /
   `split_at`, `geom/src/curves/nurbs.rs`) — a convexity fact after
   knot insertion, no evaluation.
5. **`General` images are non-rational.** `general_image_lane`
   interpolates foot samples with unit weights; the mesh speed sup and
   the piece boxes below rely on it, and a rational `General` refuses
   typed at both new arms (no producer, no fixture).
6. **The mesh certificate already carries the envelope.** `nurbs_cert`'s
   promise is `δ + ε` with a documented "≤ ε boundary-carrier residual"
   slack OUTSIDE the bound, and `certify_general` refuses an envelope
   above ε (hull sup limb). No mesh site reads `envelope` today (grep),
   and none needs to: the floor the brief names is the certificate's
   own ε. There are no `decide` rows in `mesh` (grep `decide("` — 0).
7. **Offset is not a `General` frontier** (§0): the refusal is the fitted
   offset's boundary rule and fires on every described NURBS face.
8. **The producer mints polylines.** `PXN_IMAGE_DEGREE = 1`: every
   `General` on this head is a degree-1 interpolant of 33 feet, so its
   chord polygon is exact and the lune step of §1 has no live input
   (it is pinned hand-built, and stays because #264 raises the degree).

## 1. The trimmed-region quadrature (PR-1)

**Statement.** Let the outer loop's traversal be `γ = (e₁, …, e_n)`,
interior-left (the S10 winding is the traversal's, as today). Each edge
contributes its stored image `P_e` over `[t₀,t₁]`, walked in loop
direction. Define the winding number `w_img : R → ℤ` of the closed chart
curve `Γ_img = ⋃ P_e`; for a face at rest `w_img ∈ {0, s}` with `s = ±1`
the traversal sense, and the flux is `Φ = ∫∫_R f · w_img`, area
`A = ∫∫_R g · |w_img|`, `f = ⟨S, S_u × S_v⟩`, `g = |S_u × S_v|`. Three
steps, each a certified enclosure:

**(i) Chords.** Subdivide each `General` image into `m` pieces at its
own knots and then uniformly (the `refine_knots` net; `m` is the
round's lever). Piece `k` has chord `c_k = [P(s_{k−1}), P(s_k)]` and box
`B_k` = the refined net's span box (convex hull property). An
`IsoLine`/`IsoArc` image is one exact chord (`p0 → p0 + pd`, endpoints
by structure; no box). `Γ_chord` is the closed polygon of all chords.
**On this head every `General` image is a degree-1 spline**
(`PXN_IMAGE_DEGREE = 1`, §0), so its knot-span chords ARE the image,
every box is its chord and every lune is empty: the fixture exercises
(i) and (iii) exactly and the round lever is inert on it. Step (ii) is
the mechanism's general form, pinned by the hand-built rows (Q2, Q5)
so that the day `PXN_IMAGE_DEGREE` rises (#264's banked edge work) the
lane is already sound.

**(ii) The lunes.** `w_img − w_chord = Σ_k w_k`, `w_k` the winding number
of the closed curve `(arc_k − chord_k)`, supported in `B_k` (a homotopy
inside the box moves no winding number outside it). `|w_k| ≤ 1` when the
arc is a graph over its chord: `⟨P′(t), c_k⟩ > 0` on the piece, a hull
fact on the refined derivative net (`SplineCoeffs::derivative_coeffs`),
decided under one row `props_trim_piece_monotone` and, on `Zero`/in-band,
refined by bisection to a fixed depth, then refused typed
(`QuadratureUnsupported`, "a trim piece is not monotone over its chord —
a cusp or a fold"). Then

```text
|∫∫ f·w_k| ≤ A(B_k) · sup_{B_k} |f|,   |∫∫ g·|w_k|| ≤ A(B_k) · sup_{B_k} g
```

with the sups from the patch grids' hulls over the box (`PatchGrid`,
`Collapse::Over`). The pad is symmetric on the flux and one-sided on the
area (`g ≥ 0`). Order: `A(B_k) = O(h²)` for a smooth arc (sagitta), `h`
the piece length, so the total lune pad is `O(L³κ / m²)` — second order
in the round's lever, which is what keeps this lane inside the existing
round window (`QUAD2_MAX_ROUNDS`). The alternative — a knot-and-box
aligned lattice with cells `certifies_outside` drops and boundary cells
padded by `A_cell·sup|f|` — pads `O(h)` per boundary cell along a
diagonal or curved chord, `≈ 10⁶` cells per metre of boundary at
`ε = 1e-9` against the `1024·ε` target; it reaches the fixture (whose
boundary strip is `1e-16` wide) and nothing past it.

**(iii) The chord polygon, exactly.** With `G_f(u,v) = ∫_{v₀}^{v} f(u,s) ds`
(Lipschitz in `u`, `C¹` in `v`, `∂_v G_f = f`), Green gives
`∫∫ f·w_chord = −∮_{Γ_chord} G_f du`. A chord with `u_a = u_b` contributes
nothing; every other chord is `v = ℓ(u)` affine, split at the chart's
`u`-knots (exact) and at its `v`-knots (`u* = ℓ⁻¹(v_knot)`, a ring
bracket; the definite split is the bracket's midpoint and the mismatch
sliver `{u ∈ bracket, v between v_knot and ℓ(u)}` is padded by
`½·w²·|ℓ′|·2·sup|f|`, `w` the bracket width — an O(ulp²) term). On a
sub-chord inside one knot cell,

```text
h(u) := G_f(u, ℓ(u)) = (ℓ(u) − v₀) ∫₀¹ f(u, v₀ + σ(ℓ(u) − v₀)) dσ
```

is a polynomial of degree `≤ 3p_u + 3p_v − 1` in `u` (`ℓ` affine, `f`
bidegree `(3p_u − 1, 3p_v − 1)`), and the inner integral at each outer
node is a polynomial of degree `3p_v − 1` in `σ` per `v`-span. So the
nested closed Newton–Cotes rule — outer order `3p_u + 3p_v − 1` in `u`,
inner order `3p_v − 1` in `σ` with the inner interval split at `v`-knots
per node — integrates `h` EXACTLY, and the enclosure width is the
nodes' and weights' ring rounding (`newton_cotes_weights`, exact
`i128` fractions, `m ≤ 12`). Nodes evaluate through
`Collapse::AtSpan { mid, t }` with the sub-chord's midpoint pinning the
cell, exactly as `patch_flux_exact` does. **The window `m ≤ 12` admits
`p_u + p_v ≤ 4`** — the loft walls (`(1, 2)`, `(2, 2)`) and this
fixture; a chart outside it refuses typed at a NEW named site
("trimmed exact lane's Newton–Cotes window") and the implementer files
`work/trim/trimmed-quadrature-composite-rounds.md` for the composite
fallback (`h·f(m) + F₂·h³/24` on trapezoids, `F₂` by the chain rule
through the derivative grids). Not built here: no fixture reaches it.

**The area** takes the same decomposition with `G_g(u,v) = ∫_{v₀}^{v} g`
— `g` is not polynomial, so the inner integral is `area_midpoint_taylor`'s
own rule (midpoint + Lipschitz pad, the lane's `|∂_d |c|| ≤ |∂_d c|`
constants) on `QUAD2_AREA_PIECES` sub-intervals of `[v₀, ℓ(u_j)]` at
each of `QUAD2_AREA_PIECES` outer midpoints, with the outer pad from the
`u`-Lipschitz constant of `G_g ∘ ℓ` (`sup|∂_u g|·(ℓ − v₀) + |ℓ′|·sup g`).
First order and fixed-resolution, as the rectangle's area is today; the
area is a denominator (`mean_boundary_displacement` divides by its
midpoint) and a gauge (`props_quad_face_extent` on `area.lo()`), never
the convergence meter.

**The envelope.** Exactly today's honesty pad, unchanged in kind:
`boundary_defect = Σ_e L_e · envelope_e` (metric edge length × metres
through the map) widens the area and widens the flux by
`boundary_defect · p_bound` — the true edge lies within `envelope_e`
metres of `S ∘ P_e`, so the true region and `Ω_img` differ by a metric
area `≤ L_e · envelope_e` on which `|⟨S, n⟩| ≤ sup|S|`. The envelope
never widens a chart box: it is a statement in metres about `S ∘ P`,
and `P` itself is an exact chart curve. For a `General` edge `L_e` is the
carrier's control-polygon length (the existing `Curve3::Nurbs` arm).

**Direction of every rounding.** Chord endpoints are `P_e(t)` brackets
(at the interval scalar, fat; the rule's nodes and `ℓ` inherit them and
the exactness argument holds per realization); boxes are outer
enclosures (larger pad); sups are hulls (larger pad); a monotonicity
verdict short of definite refines, then refuses — never pads; the
Newton–Cotes weights are outward-bracketed fractions; the crossing
sliver is padded. No path from an imprecise input to a narrower
enclosure. Rounds: the lane enters at `window.first`, doubling `m`
from `QUAD2_INIT_PIECES` per round, and reports `Converged`/`Open`
through the existing `classify_len` rows `props_quad_converged`,
`props_quad_face_extent`, `props_quad_last_round` — no new
convergence row; a budget exhaustion is `PropsError::QuadratureBudget`
under the `rounds` convention `work/props/quadrature-budget-conflates-its-lanes-and-budgets.md`
records (a seventh site; the PR body names it there, the item's fix
shape applies to it when PROPS takes the item); ONE new K row, `props_trim_piece_monotone` (roster
entry in `docs/K-REPORT.md`; it is a sign of a chart-unit dot product
levered by the chord length, so its margin is metres like the others).

**Engine door (PROPS's file, announced).** `geom_brep::props::quad`:

```rust
pub struct TrimChord { pub a: (RingInterval, RingInterval), pub b: (…),
    pub piece: Option<TrimPiece> /* refined image net, for the box and the monotone row */,
    pub len: f64 /* metric carrier length */, pub env: RingInterval }
pub fn trimmed_patch_face_rounds<T: Decide>(kv_u, kv_v, control, weights,
    chords: &[TrimChord], eps, band, window) -> Result<RoundOutcome, PropsError>
```

weights ≠ 1 refuse typed (the rational chart carries `IsoArc` rims only
on this head and the quotient integrand is not polynomial); the
rectangle arm `nurbs_patch_face_rounds` is untouched. **Consumer seam
(Track M's fence, announced to PROPS): ONE function,
`topo/src/props.rs::quad_lane::nurbs_face`, and its rows.** Dispatch is
by structure (C5): a loop whose every cache is `IsoLine | IsoArc` keeps
the rectangle certificate verbatim (its seven remaining sites, its
bit-identical results); a loop with any `Pcurve::General` assembles
`TrimChord`s from the caches and calls the new door. `Fitted` keeps its
refusal (no shipped producer). Rings on a curved face still refuse at
`face_flux`'s `RingOnCurvedFace` — untouched.

## 2. Tessellation (PR-2, S-MESH's ground, announced)

Two arms flip, both on the `General` variant; the `Fitted` arms and
every analytic-chart refusal stay typed.

**(a) `chords.rs::nurbs_tighten`.** The `General` arm returns the
per-axis UV speed sups `(s_u, s_v) = (max_i |u′_i|, max_i |v′_i|)` from
the image's derivative coefficient nets
(`SplineCoeffs::derivative_coeffs` per channel, unit weights by
refutation 5) — a convexity fact, the same shape as the `IsoLine` arm's
exact `|pl|` and the harmonic arm's amplitude sum; the count
`n ≥ ⌈s_u·Δt/h_u⌉, ⌈s_v·Δt/h_v⌉` follows unchanged.

**(b) `trimmed.rs`' trim walk.** `Pcurve::General(_) if nurbs_chart => {}`
beside `IsoLine`/`IsoArc`: the polygon vertex is `cache.pcurve().eval(ts[idx])`
at the shared chord parameters, which `Pcurve::eval` already does for
the variant. Watertightness is by id and is untouched: the 3-D positions
are the carrier's chord points, shared with the neighbour; only THIS
face's UV shape changes. The certificate: the triangle's corner sits at
`C(t_i)`, within `envelope ≤ ε` of `S(P(t_i))` — the class the crate's
`δ + ε` promise already names (refutation 6). No new gate, no K row.

## 3. Rows — each red-first, with the mutant it kills

`geom-brep` unit rows (PR-1, hand-built charts and images, key-free):

| row | fixture | asserts | kills |
| --- | --- | --- | --- |
| Q1 flat patch, diagonal trim | `z = c` bilinear patch, loop `(0,0)→(1,0)→(1,1)→(0,0)` with the diagonal a `General` (linear NURBS) | flux ∋ `c·½`, area ∋ `½`; widths `< 1024ε·…` | rectangle lane reads chord endpoints only |
| Q2 flat patch, arc trim | quarter-disc: the arc a degree-2 `General` (three-point interpolant, not exact) | flux/area enclose the true polygon-plus-lune value; lune pad `> 0` and shrinks ×4 per round | lune pad omitted / first order |
| Q3 rectangle loop through the new door | the fixture's chart, all-iso loop | enclosure overlaps `nurbs_patch_face_rounds`'s | trapezoid sum inconsistent with the rectangle rule |
| Q4 planted envelope | Q2's image shifted by `Δ` in `u`, `env = Δ·arm` | encloses the true value; with `env = Δ·arm/2` it does NOT | envelope pad dropped or halved |
| Q5 fold | a piece whose derivative reverses along its chord | `QuadratureUnsupported` naming the monotone row | `w_k` bound assumed |
| Q6 window | a degree-`(3,2)` chart with a `General` edge | typed refusal naming the Newton–Cotes window | silent composite / wrong order |
| Q7 knot crossings | Q1 with interior knots at `u = ½`, `v = ½` | Q1's values | unsplit sub-chord (NC on a piecewise polynomial) |

e2e rows (PR-1 flips in place; PR-2 adds), `sweep/tests/m8_4_intersection_iso.rs`:

| row | asserts | kills |
| --- | --- | --- |
| E1 (flip the row's tail) degree-2 body: `mass_properties` `Ok`, enclosure overlaps the ORACLE prism's (same solid, original chart; measured in §0) at every ε cell; the degree-1 body still refuses at the crease gate for tessellation and now MEASURES for props | non-iso site dispatch; sign/winding inverted |
| E2 (PR-2) `mesh::tessellate` `Ok` on the degree-2 body; vertex count equals the oracle prism's within the chord schedule's own ± | `chords.rs`/`trimmed.rs` arms |
| E3 (PR-1) `replace_face_offset` on the bowed face answers the oracle prism's variant at every cell — recorded, not flipped | a spec that claims offset |

E1 runs at the three ε cells the matrix draws (the scale-`1/1024`
attachment is the row's own guarantee). The mass-properties oracle is
the head's own answer on the unwidened chart — the same solid — not a
Pappus form; the lune-pad rows (Q2) are where a closed form pins the
mechanism.

## 4. Fences

- **PROPS's `geom-brep/src/props/quad.rs`** (the new door, Q1–Q7) and
  **Track M's `topo/src/props.rs`** (`nurbs_face` only): announced to
  PROPS on the away channel before PR-1 opens; PR-1 does not merge
  without PROPS's acknowledgement.
- **S-MESH's `mesh/src/chords.rs`, `mesh/src/trimmed.rs`**: the two
  arms in §2, announced before PR-2 opens.
- **Read-only:** `chart_bound.rs`, `pcurves.rs`, `pcurve_cache.rs`
  (Track Q; nothing here mints or certifies), `patch_bound.rs` (the
  crease gate is not a door this unit opens), `replace_face.rs`.
- **Out:** rational charts with `General` edges; `p_u + p_v > 4`
  (residue file); rings on curved faces; `Fitted` images; offset (the
  SHELL item above, given §0's measurement by PR-1); STEP-imported
  trimmed NURBS (no fixture — named in the PR body, not claimed).

## 5. STOP conditions (pre-registered)

1. The opening measurement on the implementer's head differs from §0's
   table (a site moved): re-survey before coding.
2. The degree-2 widening does not mint `General` on the `Intersection`
   seam or `IsoLine` on the `Chart` seam at some ε cell — the fixture is
   wrong, not the lane; stop and re-cut with the orchestrator.
3. Q2's lune pad does not shrink by `≥ 3×` per round — the piece box
   is not O(h²) (a refinement that is not knot insertion): stop.
4. E1's enclosure and the oracle's do not overlap at some ε with Q1–Q7
   green — a sign or winding defect in the consumer: stop, quote both
   enclosures, do not widen.
5. A k-lint fires on `props_trim_piece_monotone` or a `props_quad_*`
   row: distribution evidence, K-REPORT runbook, never geometry.

## 6. PR shape, difficulty, task class, lane obligations

**PR-1 (M / NUMERIC): the quadrature** — the engine door, Q1–Q7, the
`nurbs_face` dispatch, E1/E3, the K roster row, the residue file
(`trimmed-quadrature-composite-rounds`), the SHELL item's measurement,
the item's site list and offset claim corrected. **PR-2 (S / NUMERIC): tessellation** — §2's
two arms and E2; opens after PR-1 merges (E2 measures the same body).
Two PRs because the seams have two owners and PR-2 has no dependency
on PR-1's code, only on its fixture. M because the nested rule's
exactness argument has three places a natural spelling gets wrong
(the v-knot split, the sub-chord's cell pin, the monotone premise) and
each is caught only by a row written for it (Q7, Q7, Q5).

**Opening measurement (PR-1, before code):** run §0's probe on the
implementer's head and quote the table in the PR. **Lane obligations:**
`docs/prompts/implementer-discipline.md` binds; own `CARGO_TARGET_DIR`
outside the worktree; hosted CI un-narrowed is the verification of
record; no `-A` adds; announce each seam before its PR opens and merge
only with the owner's acknowledgement; `python3 scripts/work.py lint`
green with the filed residues.

## 7. Open questions for a ruling

- **Q1** Keep the rectangle certificate as the all-iso fast path
  (recommended: exact, cheaper, bit-identical for every loft/sweep
  wall; Q3 pins agreement) or route every described NURBS face
  through the trapezoid lane (one lane, moved goldens).
- **Q2** The Newton–Cotes window fence (`p_u + p_v ≤ 4`, residue file)
  versus building the composite fallback in PR-1 (recommended: fence —
  no fixture reaches it and the composite doubles the engine).
- **Q3** Whether PR-2 also flips `chords.rs`/`trimmed.rs`'s `Fitted`
  arms by the same mechanism (recommended: no — no producer, and a
  flipped arm with no row is a claim).
