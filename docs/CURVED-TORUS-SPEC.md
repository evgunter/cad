# CURVED-TORUS — the torus lane's completion (two PRs: the operand box, the circle-residual arm)

Items `work/curved/torus-operand-boxes-span-whole-ring.md` (#1488, PR-1)
and `work/curved/circle-residual-harmonics-needs-torus-arm.md` (#1489,
PR-2) — adopted by CURVED at its opening (2026-09-04, PR #1835);
CURVED's plan names the lane "Torus lane completion". Branches `curved/torus-box`, `curved/torus-arm`.
Difficulty pre-logged: PR-1 **M / NUMERIC**, PR-2 **M / NUMERIC**
(§PR shape). Survey run 2026-09-04 against main `d799235e2`, every
anchor re-read by symbol on that head. **Three of the brief's premises
measured false and the corrections bind — read §Refuted first.**

## What the survey refuted (binding)

**(R1) The kernel's torus implicit is not the quartic; the residual of
a circle against it is not a trigonometric polynomial.**
`geom_brep::implicit_residual` (`crates/geom-brep/src/implicit.rs:88`,
module table) is the LINEARIZED form `((ρ − R)² + h² − r²)/(2r)` with
`ρ = |w|`, `w = q − axis·(q·axis)`, `q = p − c`. Along a circle
`C(t)`, `ρ(t) = √(Q(t))` with `Q` a degree-2 trig polynomial, so the
residual is `(P₂(t) − 2R√Q₂(t) + R² − r²)/(2r)` — a square root of a
trig polynomial, not a degree-4 trig polynomial. The quartic
`(|q|² + R² − r²)² − 4R²ρ²` appears in the tree only as the ray×torus
root lane (`solid_contain.rs:2778` `line_torus_roots`, BOOL-3). The
consumer (`reduce.rs:1195-1215`) reads `implicit_residual` at the
endpoints and needs the SAME function's sign, so PR-2 must enclose
the linearized residual, and the brief's "degree 4 → degree 8 in
tan(t/2)" framing does not apply. What does apply is worse: the exact
extremes of the distance from a circle to a circle (the spine) are a
degree-8 problem (Neff 1990), and the tree has no isolation lane for
it — GERMARMS' fact 3 re-verified on this head: the only root lanes
are specific quadratics (`line_wall_roots`) and the BOOL-3 quartic.

**(R2) The consumer never needs roots.** `circle_residual_extremes`
returns `Option<(lo, hi)>` — a RANGE enclosure over the whole carrier —
and `circle_residual_curvature_bound` returns `Option<f2>`, a bound on
`|d²F/dθ²|`; `reduce.rs:1200-1215` folds the carrier margin
`max(lo, −hi)` with the arc's two-endpoint chord-dip `f2·Δθ²/8` and
`decide`s the larger. Both come from `circle_residual_harmonics`'
`(c₀, A₁, A₂)` triple (`implicit.rs:397`): extremes `c₀ ± (A₁ + A₂)`
(the ℓ1 bound — exact for plane/sphere, conservative for the
cylinder), `f2 = A₁ + 4A₂`. The torus residual has no harmonic
triple, so the torus arm cannot live inside `circle_residual_harmonics`;
it lives in the two readers, by subdivision (§PR-2).

**(R3) A boundary-tight torus box does NOT retire lily wall 1, with or
without declarations.** MATE-7a's R2 inference ("min true separation
0.008 m ⇒ a tight box retires wall 1", PR #1477 deviation 1, issue
1488) conflates locus separation with box separation. The weld is a
disc of radius 0.052 (the arch's start cap) concentric and coplanar
with the stem tube's end circle of radius 0.060; every AABB containing
that circle contains the disc, so the stem's torus wall box overlaps
the arch's start cap under ANY sound box, and the same holds for the
arch's wall box against the stem's end cap and for the two walls
against each other (they touch at the weld plane). `Torus` is not on
`boolean_arm_exists` (`reduce.rs:172-180`), so `first_unsupported_pair`
(`reduce.rs:280`) refuses on the first overlapping pair in arena
order; today that is the far cap (2.08 m away — a real box artifact,
`lily.rs:3931-4018`), and after PR-1 it is the next overlapping arch
face in arena order. **PR-1's honest acceptance is a RE-AIM of wall 1
from the far cap to a weld pair, not a retirement.** Retiring the gate
refusal needs `Torus` on the roster, which is a gate-policy change
and is only honest once the crossing layer has torus arms (PR-2 and
§Beyond). The pre-registered measurement (§Lily) confirms or refutes
this at the payload; the geometry above admits no other outcome.

Premises that held: `circle_residual_harmonics` has Plane/Sphere/
Cylinder arms only and `Cone | Torus | Nurbs | Approx => None`
(`implicit.rs:456`); the circle rung takes the frontier on `None`
BEFORE the covered rung (`reduce.rs:1200-1203`); every torus-body edge
is a circle; `boxes.rs:883-895` boxes a torus face as the whole ring
through `torus_extent` (`boxes.rs:540`), reading nothing from the
boundary; the cylinder arm clips its slab to the boundary hull
(`clip_to_boundary`, `boxes.rs:990`); the cone arm is the frustum
over the boundary's axial window (`cone_frustum_extent`, `:502`) with
its clip fenced out by jurisdiction (`:678-687`); `torus_chart_windows`
(`solid_contain.rs:1214`, BOOL-3) already derives a torus face's two
chart windows on the `Decide` lane from `chart_pcurve`'s closed forms.

## PR-1 — the boundary-tight torus operand box

### The geometry (for Ev)

Torus `(c, n, R, r)`, `R > r > 0` (tier-3 `DegenerateTorus` otherwise),
chart `S(u, v) = c + (R + r cos v)·ê(u) + r sin v·n`,
`ê(u) = cos u·u_ref + sin u·(n × u_ref)` (`geom/src/surfaces.rs:215-247`,
`:490-506`: `S_u = (R + r cos v)·t̂(u)`, `S_v = r(−sin v·ê + cos v·n)`,
`S_uu = −(R + r cos v)·ê`, `S_vv = −r(cos v·ê + sin v·n)`,
`S_uv = −r sin v·t̂`). For a unit direction `e` write `e_n = e·n`,
`e_⊥ = e − e_n n`, `u* = atan2(e·(n×u_ref), e·u_ref)`, so
`a(u) := e·ê(u) = |e_⊥| cos(u − u*)` and

    f(u, v) := e·S(u, v) = e·c + (R + r cos v)·a(u) + r sin v·e_n.

Interior critical points: `e·S_u = −(R + r cos v)|e_⊥| sin(u − u*) = 0`
forces `u ∈ {u*, u* + π}` (the factor is positive on a ring torus);
`e·S_v = r(−sin v·a(u) + cos v·e_n) = 0` forces `tan v = e_n / a(u)`.
Four points with values `e·c ± R|e_⊥| ± r`: a Morse max, a Morse min
and two saddles (χ = 1 − 2 + 1). When `e ∥ n` the critical set is the
two circles `v = ±π/2`. So the WHOLE torus reaches exactly
`e·c ± (R|e_⊥| + r)` along `e`; `torus_extent`'s per-coordinate
`(R + r)√(1 − nᵢ²) + r|nᵢ|` is the triangle-inequality relaxation of
that, exact for an axis-aligned torus.

**Over a chart rectangle** `W = [u₀,u₁] × [v₀,v₁]` the extremum is
closed-form. At fixed `v`, `f` is affine in `a(u)` with positive slope,
so `max_u f` sits at `a_max = |e_⊥| cos δ_u`, `δ_u = dist_τ(u*, [u₀,u₁])`
(the periodic distance from `u*` to the window, 0 when inside); then
`g(v) = R a_max + r(a_max cos v + e_n sin v) = R a_max + r A cos(v − φ)`,
`A = √(a_max² + e_n²)`, `φ = atan2(e_n, a_max)`, and
`max_W f = e·c + R a_max + r A cos(dist_τ(φ, [v₀,v₁]))`. The minimum is
the same with `a_min = −|e_⊥| cos(dist_τ(u* + π, [u₀,u₁]))` and the
phase's antipode. Note `φ ≠ atan2(e_n, |e_⊥|)` once `δ_u > 0`: the
clamped whole-torus pole is NOT the rectangle's extremum, which is why
the box cannot be built by "hull the boundary and add the pole if it
is inside" — that construction needs a point-in-region test the box
module may not run. The closed form above is the ORACLE the rows pin
against; it is not what the kernel computes.

### Why the cylinder's argument does not port, and what does

`clip_to_boundary` is sound because every world coordinate has NO
interior critical point on a cylinder chart (`∂x/∂h = axisᵢ` is
constant; if it is zero the coordinate depends on azimuth alone, which
the boundary covers). A torus coordinate has four, so the face box is
the boundary hull ONLY if none of them lies in the face — a chart
statement. The sphere arm never asks (`WholeBall`). So the torus arm
needs the face's chart window, and the box module's contract fixes
where it may come from: no `decide`, no `Band`, no escalation
(`boxes.rs` module docs, "No Q1 predicate runs on a box"). Two
sources were considered:

- **Rejected: the `Decide`-lane walk** (`torus_chart_windows`, or
  `chart_pcurve` per edge). It escalates and refuses typed; a box
  that escalates is a new door shape and a contract change.
- **Rejected: the inverse chart of each boundary carrier, sampled with
  a Lipschitz charge** (`atan2` on bracketed points, branch
  unwrapping by `periodic_branch`). Sound and band-free, but it
  re-derives on the box lane what the pcurve mint already certified,
  and needs a Span `atan2` the file does not have.
- **Chosen: the stored certified pcurves.** Every revolve/tube/boolean
  result mints them (`revolve/tube.rs:510`, `revolve/mod.rs:739`,
  `boolean/ops.rs:585`, `splitting/mod.rs:650`); a torus face's
  boundary images are `Pcurve::Harmonic{p0, pa, pb, pl}` with the
  branch already pinned per loop by the one-branch walk
  (`pcurves.rs` §3), and each cache carries `params()` and a
  `certificate().envelope` — the certified sup of `|S(P(t)) − C(t)|`
  in metres (`pcurve_cache.rs:1007-1017`). Reading them is a read of
  certified data, exactly as `EdgeCurve::params` is for `arc_extent`.

### The mechanism

1. **`FaceBoxRule::WholeTorus` becomes `TorusWindow`** with the same
   payload plus `u_ref` (the chart's `u = 0`). `face_box_rule` is the
   one kind→rule site; the rename is compile-guided at both lanes and
   at `every_surface_kind_has_a_sound_box_and_none_claims_the_world`.
2. **The window, per lane's own arena walk** (as `axial_window` /
   `boundary_axial` are today): over every half-edge of every loop
   (outer and rings), `body.pcurve(he)`. `None` on any half-edge, or
   any non-`Harmonic` variant ⇒ no window. For a `Harmonic` the
   channel extent over `(t₀, t₁)` is
   `hull(P(t₀), P(t₁)) ± hypot(paᵢ, pbᵢ)·(t₁ − t₀)²/8` — the chord-dip
   of the trigonometric part (zero, by arithmetic, for the torus
   lane's purely linear images; no zero-test on `T`). **Not
   `Pcurve::chart_box`**, which is `p0 ± |pl|·max(|t₀|,|t₁|)` — twice
   the true span for an edge starting at `t₀ = 0` and possibly on the
   wrong side of `p0`; a row pins that the 22° window IS 22°. Widen
   the hull by the certified slack `2·envelope/(R − r)` in `u` and
   `2·envelope/r` in `v` (the chart inverse's Lipschitz constants on
   the torus, from `ρ ≥ R − r` and the tube's radius), taking the
   largest `envelope` over the walk. The window is then a superset of
   the face's chart region: the region is the planar region the
   pinned loop images bound, in the lift the mint certified closed
   (`bool_torus_chart_closure`'s argument at `solid_contain.rs:1150+`
   applies verbatim), and a region lies in its boundary's bounding
   rectangle. Two loops on different branches give a wide window —
   loose, sound.
3. **The extent, once, against `Span`**:
   `torus_window_extent(center, axis, u_ref, R, r, (u₀,u₁), (v₀,v₁))`
   samples `S(u_k, v_l)` on the `(N+1)²` grid, `N = ARC_SAMPLES`, in
   Span arithmetic (trig only at exact sample parameters, the
   description's brackets in the products — `arc_extent`'s shape), hulls
   the samples, and widens coordinate `i` by
   `(h_u²·M_uu,i + h_v²·M_vv,i)/8` with `h_u = (u₁−u₀)/N`,
   `h_v = (v₁−v₀)/N`, `M_uu,i = (R + r)·perp_room(axisᵢ)` (from `S_uu`,
   axis component bounded below as `slab_extent` does) and
   `M_vv,i = r` (from `S_vv`: `|cos v·êᵢ + sin v·nᵢ| ≤ √(êᵢ² + nᵢ²) ≤ 1`).
   **The charge is a proof**: on a cell, bilinear interpolation of a
   C² function errs by at most `(h_u²‖f_uu‖ + h_v²‖f_vv‖)/8` — interpolate
   in `u` along the two `v`-edges (1-D chord bound), then in `v` between
   the two interpolants, which are convex combinations and commute with
   `∂_v`; the interpolant lies in the hull of the four corners. The
   mixed derivative never enters. Then intersect coordinate-wise with
   `torus_extent` (both are supersets; this makes "never looser than
   today" structural and absorbs a non-finite window as the cylinder
   clip absorbs a poison hull), and `Aabb::padded(pad)` outward on the
   boolean lane.
4. **Rounding**, stated per step, in the file's own policy (undirected
   arithmetic; the pad dominates ulps — `boxes.rs:140-146`): window ends
   enter as the brackets' OUTER ends (`lo()`/`hi()`); sample parameters
   are exact `f64`; radii enter at `.hi()`; `perp_room` bounds the axis
   component BELOW so the charge is bounded above; the whole-torus
   intersection is `min`/`max` of two supersets; the pcurve residual
   enters through `envelope`, a certified sup, not the sampled
   `max_residual`. The only discrete step is "window or no window", and
   its cost is not discrete: a missing window widens to the whole
   torus, never narrows.
5. **The census lane mirrors the arm** in `face_reach`
   (`census.rs:1763-1770`), reading the same window at its own scalar
   and calling the same extent — `the_two_box_lanes_agree_face_for_face`
   forces this, so **`census.rs`'s torus arm is inside PR-1's fence**
   (the brief's "boxes.rs only" is refuted by that row).

### Fences (PR-1)

`boxes.rs` (the torus arm, the window read, the shared extent, its
rows), `census.rs::face_reach`'s torus arm only, and the lily row's
re-pin in `demos/tour/src/lily.rs`. No gate policy: `boolean_arm_exists`
untouched. No `reduce.rs`. No pcurve changes (`chart_box` stays as it
is; the box computes its own extent from the public fields). No new
door and no predicate: `every_door_that_reads_a_box_is_inventoried`
and the S234 gap are untouched.

### STOP conditions (PR-1)

- The lily's stem or arch torus wall carries a half-edge with no stored
  pcurve (the window is `None` and the box is unchanged for the demand
  signal): report; the inverse-chart alternative above is the fallback
  design and needs a ruling before it is built.
- A door's pin flips from a refusal to a GRANT that exposes a downstream
  defect (a separation certificate now issued over a genuine contact;
  a census instance now cleared wrongly). The box is tighter but still
  a superset, so a wrong grant is a defect in the door, not the box:
  file it, do not fix it here.
- The window from stored pcurves and the `Decide`-lane
  `torus_chart_windows` disagree by more than the certified slack on
  any fixture: one of the two is wrong; STOP and report both numbers.

### Acceptance rows (PR-1) — each red-first, with the mutant it kills

A test helper `torus_wall(R, r, axis, u₀, u₁, v₀, v₁)` builds a real
torus patch (two parallels, two meridians, `mint_pcurves`) beside
`revolved_wall`; full-tube and full-ring variants close by a seam.

1. **Locus** `a_torus_windows_locus_is_inside_its_box`: windows swept
   over `u` spans {22°, 100°, 200°, 350°, full} × `v` spans {60°, 200°,
   full} at three `(R, r)` and two axes; samples at cell MIDPOINTS as
   well as corners. Kills: the boundary-hull-only box (a window holding
   the `u = 90°` pole loses the bulge) and the uncharged construction.
2. **Ceiling** `the_torus_arms_box_is_exactly_the_construction_its_rule_states`
   restated: the box equals the sampled-rectangle construction in the
   fixture's parameters (both directions, six faces), and on the 22°
   full-tube fixture at the lily's numbers (`R = 5, r = 0.06`) every
   side is within `charge + pad` of the §Geometry oracle, with
   `charge ≤ 3.7e-4 + 1.2e-3 m`; the whole-ring box is ≥ 3 m off on two
   sides. Kills: the loose arm (not a regression — the pin is on
   tightness) and any charge dropped from either channel.
3. **Fallback ceiling**: a face with no stored pcurves (today's relabelled
   fixtures) gets exactly `torus_extent` — the current row, re-titled.
4. **Window fidelity**: the 22° edge's window is `[u₀, u₁]` to within the
   slack, not `chart_box`'s `p0 ± 22°`. Kills: reading `chart_box`.
5. **Lanes agree**: `the_two_box_lanes_agree_face_for_face` gains a
   windowed torus case.
6. **Lily wall 1 re-aims** (`lily.rs:3931-4018`): the refusal no longer
   names the far cap; the pair it names is measured and pinned by
   payload (§Lily), and its true separation is stated in the row.
7. **The MATE-7a socket row** (`a_partly_covered_torus_pair_still_gates_on_the_uncovered_one`,
   `mate7a_torus_rest.rs:352`): re-measure whether the socket's outer
   wall still overlaps; the row's comment says this is "issue 1488's
   question" — answer it in the row.
No three-outcome ε rows: the box runs no predicate (state this in the
PR rather than leave a reader looking for them).

## PR-2 — the circle residual's torus arm

### The math (for Ev), corrected per R1

Circle `C(t) = p + ρ_c(û cos t + v̂ sin t)`, torus `(c, n, R, r)`,
`q(t) = C(t) − c`, `h(t) = q·n = h₀ + a_h cos(t − φ_h)` (exact first
harmonic, `a_h = ρ_c·hypot(û·n, v̂·n)`), `w(t) = q − h n`,
`ρ(t) = |w(t)|`, `Q = ρ² = c₀' + A₁'cos(t − φ₁) + A₂'cos(2t − φ₂)` (the
cylinder arm's algebra, `implicit.rs:427-449`, with the torus axis as
the cylinder's). The residual is `g(t) = (d(t)² − r²)/(2r)` with
`d² = (ρ − R)² + h²` the squared distance to the spine circle, so `g` is
monotone in `d` and its sign is `d − r`'s. There is no harmonic form;
what exists is a **certified second derivative**: with `|w'|, |w''| ≤ ρ_c`
(perpendicular projection is a contraction), `ρ' = w·w'/ρ`,
`ρ'' = (|w'|² + w·w'')/ρ − (w·w')²/ρ³`, hence
`|ρ'| ≤ ρ_c`, `|ρ''| ≤ ρ_c + 2ρ_c²/ρ_min`, and

    |(d²)''| ≤ 2ρ_c² + 2·D_max·(ρ_c + 2ρ_c²/ρ_min) + 2a_h² + 2·H_max·a_h,
    f2 := |(d²)''|_max / (2r),

with `ρ_min, ρ_max` from `√` of the ℓ1 range of `Q` (clamped at 0),
`D_max = max(|ρ_min − R|, |ρ_max − R|)`, `H_max = |h₀| + a_h`. All
enclosures are the existing harmonic ones; nothing new is solved.
`ρ_min = 0` (a circle that may meet the axis) makes `f2` infinite, and
an infinite enclosure is the honest one: `g` has a kink there. No
branch — the arithmetic carries it (`f64`: `−∞/+∞` margin, `Negative`,
frontier; interval: poison, `Escalated`). A row pins both lanes.

**The enclosure is by subdivision with the chord-dip charge**, the
`arc_extent` doctrine (`boxes.rs:1236-1275`) and the very argument the
consumer already runs at `N = 1`: on `K` sub-arcs of width `h`,
`range(g) ⊆ [min_k g(t_k) − f2 h²/8, max_k g(t_k) + f2 h²/8]`.

**Why an arc-scoped enclosure is REQUIRED, measured on the lily.** The
stem's outer-equator seam (a 22° arc of a 5.06 m circle) against the
arch's torus carrier (`R = 1.1, r = 0.052`, rings coplanar, axes 3.90 m
apart): `ρ ∈ [1.16, 8.96]`, `f2 ≤ 7930 m/rad²` by the bound above
(measured `|g''|` along the arc 296 — the bound is loose by the
full-carrier `D_max`), true residual `0.0086 m` at the weld end rising
to `8.7 m`. The full-carrier range straddles by metres (the carrier
passes through the arch), the two-endpoint chord-dip charges
`7930·0.384²/8 ≈ 146 m`, and only an arc-scoped subdivision resolves
the 0.0086: the charge `f2·(Δθ/K)²/8` is 0.036 (K = 64), 0.0089 (128),
0.0022 (256), 0.0006 (512) m. **`K = 256`** as the named constant
`ARC_RESIDUAL_SAMPLES`, with the resolution law stated at the constant:
an arc of span `Δθ` clears iff its true clearance exceeds
`f2·(Δθ/K)²/8 + band`. The stem's end circle against the arch (a
meridian-like circle: `f2 = 0.285`, constant residual `+0.0086`) and
the arch's start circle against the stem (`−0.0075`, one-sided inside)
resolve at any `K`. Cost: `K + 1` residual evaluations per examined
circle×curved-face pair.

### The arm and the consumer

- **New public door** `circle_arc_residual_range(surface, center, axis,
  radius, u_ref, t₀, t₁) -> Option<(T, T)>` in `implicit.rs`: the
  subdivision above, kind-generic (`implicit_residual` at the samples,
  `f2` from `circle_residual_curvature_bound`), arc-scoped `f2` for the
  torus (the `Q`/`h` ranges are taken over the arc's sampled `w`, `h`
  with the Lipschitz charge `ρ_c·h/2`, which is what brings the lily's
  7930 down toward the measured 296 — state the number the
  implementation achieves). `None` where `f2` is `None`.
- `circle_residual_curvature_bound` gains the torus arm (full-turn
  `f2`); `circle_residual_extremes` gains it as the arc door over
  `[0, τ]`. `circle_residual_harmonics` is UNCHANGED and keeps
  `Torus => None` — it answers a question the torus has no answer to.
- **`reduce.rs`'s arc margin** (`:1204-1215`) becomes the `K`-sample
  door for every kind; the two-endpoint chord-dip is deleted as its
  `K = 1` instance (the comment at `:1206-1213` is then true of the
  samples rather than the endpoints). This is the arity widening the
  brief permits: the call gains `(t₀, t₁)`; the fold and the `decide`
  are verbatim. For plane/sphere/cylinder the new arc margin is ≥ the
  old one (the same `f2`, a finer `h`), so no existing clear can turn
  into a frontier — a row pins the monotonicity on random arcs.

### Fences (PR-2)

`implicit.rs` (the two readers' torus arms, the new door, the sample
constant, their rows) and the one `reduce.rs` fold. No `contain.rs`
(#1484 is S-BOOL's), no `boolean_arm_exists`, no join/zip, no
`tangent_locus`, no pierce lane for circles.

### STOP conditions (PR-2)

- With `K = 256` the lily's seam×arch pair still reads `Zero`
  (the arc-scoped `f2` did not close the gap): report the measured
  `f2` and clearance; do not raise `K` past 1024 without a ruling.
- The MATE-7a frontier row's new landing is a validated BODY rather
  than a typed refusal (a coincident torus pair zipped by accident).
- Any existing plane/sphere/cylinder clearance pin moves from
  `Positive` to `Zero`/`Negative`.

### Acceptance rows (PR-2) — red-first, at all three bands

1. **Coaxial circle at `(ρ_c, h)`**: constant residual
   `((ρ_c − R)² + h² − r²)/(2r)` inside `(lo, hi)`, `hi − lo ≤ 2·charge`.
   Kills the wrong sign on `h²` (the `n`-projection term).
2. **Coplanar ring-plane circle offset from the axis**: closed-form
   extremes at `ρ = |w₀| ± ρ_c`; the enclosure contains both and is
   within `charge` of each. Kills a dropped `ρ''` term (loose by more
   than `charge`) and a dropped `D_max`.
3. **A meridian of the torus itself** (`lo ≤ 0 ≤ hi`) and a concentric
   circle at `r' ≠ r` (constant, one-sided).
4. **Grazing between samples**: a circle tangent to the tube at a
   parameter placed mid-cell; the uncharged sampled hull says clear, the
   arm's enclosure contains 0. Kills the dropped charge.
5. `the_curvature_bound_encloses_the_sampled_second_derivative`
   (`implicit.rs:874`) gains tilted, offset and near-axis torus cases;
   the through-axis circle pins the infinite enclosure on the `f64`
   lane and the escalation on the interval lane.
6. **Lily pairs as fixtures** (numbers above): seam×arch clears at
   `K = 256`, end-circle×arch clears, start-circle×stem clears
   inside; each at three bands.
7. **The boundary row** `the_admitted_torus_lane_stops_at_the_curved_pierce_frontier`
   (`mate7a_torus_rest.rs:382`, the only row by grep on the refusal —
   MATE-7a's "two rows" includes `a_fully_covered_torus_pair_reaches_past_the_operand_gate`,
   which stays true) asserts a refusal that is no longer the circle
   rung's `None`; what it becomes is MEASURED and pinned by payload.
   Pre-registered candidates: `CurvedPierceUnsupported` again, from
   the covered endpoint posture's `Undecided` (`vertex_on_curved_face`
   → `curved_face_containment` answers `None` for a torus interior,
   `contain.rs:598`, #1484), or a downstream join refusal.
8. **Monotonicity**: random arcs against plane/sphere/cylinder — the
   `K`-sample margin ≥ the two-endpoint margin.

## The lily: what it does NEXT (pre-registered, one measurement)

Scratch tightening applied for measurement only (uncommitted:
`boxes.rs`'s torus arm clipped to the boundary hull — exact for the
stem's 22° wall, whose window holds no pole; a scratch print of every
face of both operands in `lily.rs`). The run queued three times on the
express slot under load 23–26 and never obtained it (`exit 75`); the
log is `~/.local/share/cad-work/curved-torus-spec-lily-run.log`.
**Prediction from the geometry of R3, to be confirmed by the
implementer's opening measurement before any code**: wall 1 stays
`CurvedPairUnsupported { operand: A, kind: Torus, .. }` naming the
first arch face in arena order among {start cap (Plane), torus wall};
true separations 0.008 m. If the measurement lands anywhere else, R3
is wrong and PR-1's acceptance row 6 is re-cut from the payload.

Beyond PR-2, the stem glue's door sequence, derived (each a separate
unit, none promised here): (a) `Torus` onto `boolean_arm_exists` — the
gate never boxes a supported kind, so the box's tightness then serves
the sweep tree, `separation`, `ops` and the census, not the gate; (b)
the arch's outer-equator seam against the STEM's carrier crosses it at
29° along the arch (`−0.0075 → −0.030 → +`), outside the stem face's
window — a circle×torus pierce/trim question with no root lane; the
honest negative certificate maps the arc into the stem's chart; (c)
`curved_face_containment`'s torus interior (#1484); (d) the weld's
coplanar concentric caps (the F7 repair half B). MATE-7a's "one function
away" was measured on the coincident full-torus pair, not on the lily.

## PR shape and difficulty

Two PRs, sequenced: PR-1 changes what the gate NAMES, PR-2 what the
sweep DECIDES; landing them together would hide a re-aim behind a
clearance. Both **NUMERIC** (certified enclosures, no topology).

- **PR-1 = M.** The closed form is a page; the work is the window read
  from certified pcurves with its slack argued, the rectangle charge
  proof at both lanes, a real torus fixture helper, and the lily re-pin
  from measurement. L would be honest only if the pcurve read were
  already shared; it is not.
- **PR-2 = M.** The `f2` derivation must be sound at every term
  (the interval lane will punish a loose one with escalations), the
  arc-scoped ranges need their own Lipschitz charge, the consumer fold
  changes for all kinds, and the boundary row's landing is unknown
  until measured. Not H: no new predicate, no topology, no join.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds; the run gates the full
matrix (this clause used to forbid a `CI-Config` trailer; that spelling
was deleted on 2026-09-04 and nothing in CI reads one); measure-first
checkpoint before code
(`memories/refusal-text-is-not-cause.md` — R3 is exactly that shape,
and the lily row is the instrument); lane-private `CARGO_TARGET_DIR`
outside the worktree; merge origin/main before opening; confirm the
twelve test jobs; watch to completion in the foreground. Do not merge.

---

## Rulings at ratification (CURVED orchestrator, 2026-09-04)

Ratified as written, with these answers to the lane's questions:

1. **A RE-AIM is the honest close of #1488, and the item is re-scoped
   to say so.** Its motivation (MATE-7a R2's "a tight box retires wall
   1") is refuted by R3's geometry; what the box buys is the sweep
   tree, `separation`, `ops` and the census, and the lily row's
   re-pin from the far cap to a weld pair is the acceptance. The
   implementer records the refuted motivation in the item's Closed
   section and cites R3.
2. **`Torus` on `boolean_arm_exists` is CURVED's ground** (the operand
   gate is the curved-operand-reach lane) and is a THIRD unit after
   PR-2: the door sequence §Lily (a)–(d) gets its own item,
   `work/curved/torus-operand-gate-admission.md`, filed by the
   orchestrator at ratification with (c) noted as S-BOOL's containment
   door (`curved-face-containment-lacks-cone-torus`, handover pending)
   and (b) as the circle×torus pierce/trim question. MATE-7a's "one
   function away" is corrected there by citation.
3. **`K = 256` is accepted as the named constant with its resolution
   law**; an adaptive scheme is not asked for until a measured consumer
   needs a finer arc than the law admits — that would be its own row.
4. **Length stands.** The refutation section is the part a reader most
   needs.

**Opening measurement (PR-1, before any code):** the lily wall-1 row
under the scratch tightening, per §Lily — quote the payload and the
first overlapping arch face in arena order in the PR body; if it lands
anywhere else, R3 is wrong and row 6 is re-cut from the payload.

**Branches** `curved/torus-box` (PR-1), `curved/torus-arm` (PR-2).
**Pre-log stands: PR-1 M / NUMERIC, PR-2 M / NUMERIC.**
