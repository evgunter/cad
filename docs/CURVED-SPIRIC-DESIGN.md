# The spiric rim carrier and the pinch machinery — design conversation

**STATUS: RATIFIED (Ev, PR #1858, 2026-09-04)** — see §0a for the
rulings. (CURVED orchestrator; drafted 2026-09-04 against main
`d799235e2`, every code cite re-derived by symbol at that head; the
tracker item is `work/curved/spiric-carrier-ruling.md`.) Two questions the VERBS exit walk transferred
to design: `docs/VERBS-RIMCAP-SPEC.md` PR-2 (the torus half of the
partial-revolve rim — the spiric carrier) and
`work/curved/pinch-carrying-machinery-valence-4.md` (#1377, the
valence-4 pinch family). The RIMCAP spec guessed they are one
design; §6 measures that guess and refutes it. Nothing here
dispatches; §1 is what Ev rules on.

## 0. The orchestrator's reading (for Ev, in two paragraphs)

The RIMCAP spec deferred the torus rim as "a spiric quartic `Curve3`
cannot carry" and guessed it belongs with the pinch machinery. §3–§6
measure both: the rim is always the two-oval regime of the spiric
(the shell offset can never reach the double point), it has a closed
form in the torus's own minor angle with speed bounded below by `r`,
and the pinch family's curves are ellipses the kernel already carries
— its gap is vertex topology, not a carrier. So the honest shape is a
per-configuration exact kind, which is what D3's closed enum is for,
and the pinch stays its own conversation.

I recommend Q1 = (b), Q2 = (b1), Q3 = (i) now with (ii) opened as its
own `[ev]` conversation (its payoff is the whole "torus exact meters"
column, so it deserves to be argued on that ground), Q4 = (i), Q5 =
(ii) for the carrier unit with the props lane funded as a separate
numeric unit right behind it, and Q6 = ratify. Answer in this PR's
comments; I edit the doc in place and merge.

## 0a. Rulings (Ev, PR #1858 comments, 2026-09-04)

- **Q1 = (b), Q2 = (b1)**: an exact `Curve3::Spiric` variant, a
  per-configuration special case exactly as `Ellipse` is ("given that
  the kernel already represents Ellipse as a special case, probably do
  that").
- **(c2) is the general route, not a rejected one.** Ev: "for the
  cases where it's not something we already have a special case for
  we'd want to do something like that." §5(c2) is re-read
  accordingly: the rung-3 fitted carrier stays the ratified route for
  every pair without a closed form (the bulb's cylinder×torus and
  cone×torus rims); the spiric takes the closed form only because it
  exists.
- **Sequencing "(A) then (B)"**: ship the spiric variant first (PR-1
  as pre-logged), then open the C9 ring `sqrt` (Q3(ii)) as its own
  `[ev]` conversation — the change that unblocks the general rung for
  every torus operand.
- **Q3(i), Q4(i), Q5(ii), Q6**: taken as answered by Ev's 👍 on the
  orchestrator's summary comment (2026-09-04): a data-free exact
  `Pcurve` variant for the spiric's chart images; STEP export as an
  export-only approximating spline at ε; the carrier ships first and
  the elbow stops at the lune's `VolumeUncomputable` door with the
  props quadrature lane as the next numeric unit; the pinch is
  independent, RIMCAP's one-doc premise retired, the double-point
  spiric recorded as a #1377 member.

What follows from the rulings: `docs/DESIGN.md`'s D3 curve inventory
gains the spiric (the ratifying edit rides the unit's PR, where the
variant lands); `docs/VERBS-RIMCAP-SPEC.md` §PR-2 is superseded by this
document; PR-1's spec is cut from §8 by the orchestrator.

## 1. Decisions asked (each with the viable answers; §7 argues)

- **Q1 — fund the carrier, or fence.** (i) option (a), the fence made
  permanent: the klein elbow's `shell`/`shell_open` refuse forever at
  the door §2 names; (ii) option (b), an exact spiric kind
  (recommended, §7). NURBS-fitting is not a third answer — §5(c) says
  what it is and is not.
- **Q2 — the kind's shape.** (i) `Curve3::Spiric`, one variant for the
  plane×torus section, parameterized by the torus's own minor angle,
  evaluated in closed form (recommended); (ii) a general
  "exact implicit locus in a chart" kind evaluated by a certified
  1-D root-find per parameter — the shape that would also carry the
  bulb's cylinder×torus and cone×torus rims. §5(b) costs both.
- **Q3 — the pcurve, and the C9 ring.** The spiric's chart images on
  the plane cap and the torus wall are neither `Harmonic` nor any
  other exact `Pcurve` form, and the Fitted grade refuses on a torus
  chart (`enclose.rs` poisons the torus). (i) A data-free exact
  `Pcurve` variant — the chart inverse composed with the carrier —
  closed form on both charts; (ii) revise C9 to admit an
  outward-rounded `sqrt` in `RingInterval`, which retires the torus
  meters conversion and, with it, every "blocked on the torus's exact
  meters conversion" arm in the C5 table, not only this one.
  Recommended: (i) for the rim, (ii) as its own conversation.
- **Q4 — STEP export of a genus-1 carrier.** No exact form exists.
  (i) An approximating `B_SPLINE_CURVE` minted for export only, its
  tolerance stated in the file; (ii) typed `UnsupportedCurve` refusal.
- **Q5 — props.** The hollowed elbow's volume is an elliptic integral
  (§4.7); tier 3's +V needs it. (i) Fund the certified quadrature lane
  with the carrier; (ii) ship the carrier alone and let the elbow
  stop where the sphere lune stops today (`VolumeUncomputable`).
- **Q6 — the pinch.** Ratify §6: the two questions are independent;
  the one shared object (the double-point spiric, `d = R − r`) is a
  pinch-family member gated on #1372's declared-coincidence channel
  and has no consumer; RIMCAP's "one design doc" premise is retired.

## 2. Premise survey — what held and what was refuted

**Refuted: the klein elbow's door is no longer `TogetherAxialCorner`.**
RIMCAP §"What this unit is" and `docs/KERNEL-VERBS.md` row
*shell / hollow* both say the elbow refuses `TogetherAxialCorner
{ surfaces: 2, what: "one profile constraint…" }`. RIMCAP PR-1
merged (#1674) and its carried-datum arm now SOLVES that corner; the
measured door at this head is one deeper — the rim EDGE's carrier
mint: `ReplaceFaceError::TogetherAxialEdge { what: "a circular edge
between two charts whose centre is off the axis" }`, raised by the
`offset_axial_latitude` predicate in `topo/src/offset_axial.rs:
mint_carrier` (the latitude arm), pinned by
`sweep/tests/verbs_shell.rs:the_klein_wall_pair_waits_on_the_partial_revolve_rim`
(both `shell_open` and sealed `shell`, same face/edge/predicate) and
`sweep/tests/torax_axial.rs:torax_the_klein_elbow_rim_refuses_at_the_carrier_mint`.
The KERNEL-VERBS row is stale and should be corrected in the same
PR that ratifies this document.

**Held: the section is a genuine quartic.** The `torax_axial` row's
numbers (`R = 1.2`, moved `r = 0.225`, `d = 0.05`): half-width
`0.22520271607754455` against half-height `0.225`, re-derived here
from §3's closed form (Δ = 2.03e-4 m), whose residual against both
implicit forms is ≤ 3.5e-16 m at 10⁴ samples.

**Held: the C5 table routes the spiric to the general rung and the
general rung refuses it.** `geom-brep/src/intersect.rs:route`'s
`(Plane, Torus)` arm: "the axis-parallel offset plane's spiric
section … routes to the general rung … blocked on the torus's exact
meters conversion"; `plane_torus_section` answers
`SectionError::RoutesToGeneralRung` there (trilean 2's definite
`pt_axis_plane_gap`). The block is real and it is ONE function:
`geom-brep/src/ssi/enclose.rs:implicit_enclosure` (and its gradient
twin) return `RingInterval::poison()` for `Surface::Torus` because
the torus residual `((ρ − R)² + h² − r²)/2r` carries `ρ = |w|`, a
square root the C9 ring does not provide (`geom-core/src/
ring_interval.rs` module doc; README clause C9). Note for Q3(ii):
the same tree already carries an outward-rounded bracket square root
— `geom/src/curves/boxes.rs:Brk::sqrt_nonneg` — because `f64::sqrt`
is IEEE correctly rounded; the ring's omission is a ratified
minimalism, not an obstacle.

**Held: `Curve3` has no quartic kind** — `geom/src/curves.rs:Curve3`
is `Line | Circle | Ellipse | Nurbs`.

**Held, sharper: the pinch family needs no carrier.** Its section is
two exact `Ellipse`s (`intersect.rs:EqualCylinderSection::TwoEllipses`);
the refusal is a frame/topology statement,
`BooleanError::GermFrameCylinderPinch` (`topo/src/boolean/mod.rs`,
raised in `boolean/join.rs` from `FrameError::IntersectingCylinderAxes`).

**Refuted: RIMCAP's "same family as #1377".** §6, by §3.4's algebra.

SEAT-6 (PR #1593) is OPEN at `gh pr view` time; nothing here depends
on it.

## 3. The geometry, exactly

### 3.1 Frame and section

Torus `T(R, r)`, `R > r > 0` (the ring convention `surfaces.rs:
Surface::Torus`), centre `c`, unit axis `a`, chart
`S(u, v) = c + e(u)·ρ(v) + a·r sin v`, `ρ(v) = R + r cos v`, `e(u)` the
azimuth direction. Fix a unit `n ⊥ a` and `m = a × n`. The cap plane
`Π_d = { x : (x − c)·n = d }` is parallel to the axis at distance
`|d|`. In coordinates `(x, y, z) = ((·)·n, (·)·m, (·)·a)` the torus is
`(x² + y² + z² + R² − r²)² = 4R²(x² + y²)` and `Π_d ∩ T` is the plane
quartic

```text
(y² + z² + d² + R² − r²)² = 4R²(d² + y²)          (the spiric of Perseus)
```

a bicircular quartic (double points at the circular points at
infinity), hence of geometric genus **1** when smooth: a real
elliptic curve. It has NO rational parameterization, so no NURBS is
its locus and no `Pcurve::Harmonic` is its chart image (§4).
Genus drops to 0 exactly when a further real singular point appears
(§3.3).

### 3.2 The parameterization the kernel would store

On the plane `x = d`, `ρ² = x² + y²` gives `y = ±√(ρ(v)² − d²)`, so

```text
P±(v) = c + n·d ± m·√((R + r cos v)² − d²) + a·(r sin v),   v ∈ ℝ, 2π-periodic
```

— the torus's own minor angle `v` is the parameter, as the circle's
is its angle and the ellipse's its eccentric anomaly. Identities in
ℝ: `implicit_residual(torus, P±(v)) = 0` and `(P±(v) − c)·n − d = 0`
(both checked numerically, §2). Derivatives are closed form:
`dP/dv = ∓ m·(r ρ sin v)/√(ρ² − d²) + a·r cos v`, and

```text
|dP/dv|² = r² cos² v + r² ρ² sin² v /(ρ² − d²)  ∈  [ r²,  r²(R − r)²/((R − r)² − d²) ]
```

whenever `d < R − r` (the second bound at `ρ = R − r`, where
`ρ²/(ρ² − d²)` is largest). The speed is bounded away from zero, so
`v` is a regular parameter on each branch — the ellipse's situation
(`|dP/dθ| ∈ [minor, major]`), not the sphere pole's.

### 3.3 Regimes in `d` (real points, `d ≥ 0`)

- `d < R − r`: `ρ(v) > d` for every `v`; `P+` and `P−` are two
  disjoint smooth closed curves (two ovals), each a full `v`-period,
  each an embedded regular curve. Genus 1.
- `d = R − r`: the ovals meet at the single point `c + n·d`
  (`v = π`, `y = 0`), where `Π_d` is TANGENT to the inner equator —
  the section has a real node, a valence-4 point on the curve
  (four arcs, two crossing branches). Genus 0. This is the
  configuration the pinch family's vertex has (§6).
- `R − r < d < R + r`: one oval, the `P±` sheets glued at the two
  folds `v = ±v*`, `cos v* = (d − R)/r`, where `√(·) = 0`. The curve
  is smooth there (the fold is a chart artefact of the `v`-chart,
  like parameterizing a circle by `x`); only the `v`-parameterization
  is singular. A kernel edge in this regime would need the OTHER
  parameter (`u`, with `cos v = (d/cos u − R)/r`) or a split at the
  folds.
- `d = R + r`: one real point (tangent to the outer equator);
  `d > R + r`: empty.
- Classical cases: `d = r` gives the Cassini ovals with foci
  `(y, z) = (±R, 0)`, `a² = 2Rr` (identify `c² = d² + R² − r² = R²`);
  Bernoulli's lemniscate is the Cassini member `a = c`, i.e. `d = r`
  AND `R = 2r` — the double-point condition met inside the family.

Transversality (D2's precondition for `Intersection`): the torus
normal `e(u) cos v + a sin v` is parallel to `n` only at `sin v = 0`,
`e(u) = ±n`, i.e. only at `d ∈ {R − r, R + r}`. Away from those two
values the section is a transverse locus everywhere (measured on the
elbow: `min sin∠(normals) = 0.9987`).

### 3.4 The rim family lives in the first regime, always

Shell offsets the torus by struct-update (`geom-brep/src/offset.rs:
offset_surface`, `minor_radius ↦ minor_radius + d` with `d = −t` inward, `R` fixed) and moves the
meridian cap along its own normal by `t` (`offset_axial.rs` module
doc: "the moved meridian caps travel inward along their own normals,
so they stop containing the axis"). So the moved rim is the section
of `T(R, r − t)` by `Π_t`, and `t < R − (r − t) ⟺ 0 < R − r`, which
the ring convention guarantees. **The rim spiric is two disjoint
smooth ovals for every legal wall thickness**; the rim edge is one
oval, selected by the old rim's carried midpoint (the sign in `P±`
is a decided predicate on `m·(old_mid − c)`, metered at `ρ`). The
double-point regime, the folds and the tangency are all unreachable
from the shell door. The old rim is the profile circle, already
split into two edges at the profile's two vertices (`verbs_shell.rs:
circle_loop`), so each moved edge spans half a `v`-period and the
circle's at-most-one-period winding check (`certify.rs:
CertifyError::WindingExceeded`) transfers verbatim.

## 4. What the kernel asks of a carrier — measured from the code

Every `match` on `Curve3` names the `Nurbs` arm; the non-test sites
at this head, by function (72 arms naming `Ellipse` across 47 files;
the list below is the non-test set), and what each costs a spiric:

| need | site (`file:symbol`) | on a spiric |
|---|---|---|
| eval / deriv / deriv2 | `geom/src/curves.rs:Curve3::{eval,deriv,deriv2}` | **closed form** (§3.2): one `sin_cos`, one `sqrt`; generic over `Real`, so the `Interval` scalar encloses it with no lane fork |
| parameter of an on-carrier point | `curves.rs:Curve3::param_near` | **closed form**: in the meridian half-plane the point is `w = (ρ − R, h)` with `h = (p − c)·a`, `ρ = |p − c − a·h|`, and `v = near + atan2(w × w_near, w · w_near)` with `w_near = (r cos near, r sin near)` — the circle arm's anchored difference form verbatim; `Ellipse`/`Nurbs` answer `None`, the spiric need not |
| foot point / projection | `geom/src/curves/projection.rs:Projection3::project` | Newton from a seed — the same machine the ellipse and the spline use; a root-find, as it is for them |
| certified boxes | `geom/src/curves/boxes.rs:{circle,ellipse,nurbs}_arc_aabb`; consumer `topo/src/boolean/boxes.rs:EdgeBoxRule` | conservative closed form: per axis, `n·d ± m·[√((R−r)²−d²), √((R+r)²−d²)] + a·r·[−1, 1]` (sign per branch) through `Brk` (already has `sqrt_nonneg`); exact extremes need a 1-D root of a transcendental — not needed, C10 admits conservative supersets |
| attachment / tier-3 certification | `geom-brep/src/certify.rs:run_checks` (`Intersection`: `Surface{1,2}Residual`, `WitnessMidpoint`, `Transversality`, `ParamSpan`) | as for `Circle`/`Ellipse`: residual **zero by construction**; no C2 hull limb is owed (C2's three limbs are for FITTED carriers); `edge_extent`'s sag arm needs a `min speed·span` — `r·Δv` from §3.2 |
| exact meters composite (C9 ring) | `geom-brep/src/ssi/enclose.rs:implicit_enclosure` | **unavailable for a torus** (poison) — needed only by fitted certificates, not by an exact carrier |
| pcurves (per half-edge cache) | `geom-brep/src/pcurve_cache.rs:{Pcurve, chart_pcurve, run_fitted_checks}`; mint `topo/src/pcurves.rs:chart_mints` | **new form needed.** Plane cap: image `(±√(ρ²−d²), r sin v)` in the cap frame — one sqrt channel, not in `span{1, cos, sin, t}`. Torus wall: `(u(v), v)`, `u = atan2(±√(ρ²−d²), d)` — the meridional channel is the parameter itself, the azimuth channel transcendental. `chart_pcurve`'s torus arm refuses any non-`Circle` (`UnsupportedCarrier`); `run_fitted_checks` → `ssi::certify_rung3` → torus poison. Q3 |
| chord tessellation | `mesh/src/chords.rs:compute_chords` (per-kind step: `sagitta_step`, `ellipse_step`, `nurbs_chord_count`) | closed-form `sup|C″|` bound: `|C″| ≤ r + (rρ_max + r²)/f_min + r²ρ_max²/f_min³`, `f_min = √((R−r)²−d²)` — the ellipse's curvature-bound pattern; `trimmed.rs:trim_polygon` consumes the pcurve |
| STEP export | `step-export/src/writer.rs:edge_curve` (`EDGE_CURVE` with a bare curve; no `SURFACE_CURVE`/`PCURVE` written) | no ISO 10303-42 entity; genus 1 ⇒ no exact `B_SPLINE_CURVE`. Q4. Import (`step-import/src/adopt.rs`) would read a spline back; the adopted `Intersection{torus, plane}` with a `Nurbs` carrier passes `certify.rs`'s 9-sample gate but owes the C2 hull at rest, which the torus poison denies — a typed import refusal until Q3(ii) |
| rigid transform / scalar lift | `topo/src/transform.rs:map_carrier`; `geom/src/scalar_lift.rs:map_scalar` | frame maps, mechanical |
| props (+V, tier 3) | `geom-brep/src/props/curved.rs:torus_boundary` (circles only, else `NotIsoRectangle`/`Unimplemented`); `props/quad.rs` (cylinder-chart curved cuts only, C9 ring, harmonic pcurves) | **no lane; no elementary closed form** — §4.7 |
| kind tables, census, readback, queries | `topo/src/chart_iso.rs:classify_kind`, `query.rs`, `readback.rs:edge_pose`, `chord_join.rs:between_edge_in_plane`, `merge_faces.rs:loop_winding`, `boolean/{contain,join,reduce,sectors}.rs`, `splitting/*.rs`, `replace_face.rs:{transport_curve,plan_edge}`, `editor-core/src/eval/measure.rs:curve_reach`, `props.rs:face_flux` | each an explicit arm: a spiric is neither line nor conic, so most refuse typed exactly as `Nurbs` does today, and each refusal is a named frontier |
| the mint itself | `offset_axial.rs:mint_carrier` ("the carrier keeps its KIND and conventional frame") | **the law bends once**: the old rim is a `Circle` (meridian), the moved rim a spiric — the first kind-changing mint. It stays inside the door's inline-arithmetic fence (no section function called): a struct literal from the moved torus and the moved cap, then `param_on` and the midpoint meter verify it as for every other arm |

### 4.7 Props are elliptic integrals

The cap's oval area `∮ y dz = r ∫₀^{2π} √((R + r cos v)² − d²) cos v dv`
and the wall's flux (the `u`-integral over `[u₁(v), u₂(v)]` is
elementary; the outer `v`-integral carries `u(v) = arccos(d/ρ(v))`)
are integrals of algebraic functions on a genus-1 curve — elliptic,
not elementary. Measured on the elbow: oval area `0.1591851`,
against `π·r·half-width = 0.1591864` (an ellipse of the same
extents) — close, and not equal. So RIMCAP PR-1's acceptance pattern
("derive the cavity volume exactly") is unavailable, and `shell`'s
last act `validate_geometric` (`topo/src/shell.rs`) needs +V. Without
a quadrature lane the elbow would stop precisely where the sphere
lune stops today: `VolumeUncomputable { NotIsoRectangle }`
(`work/props/sphere-flux-arm-refuses-partial-bands.md`). A lane
would be certified quadrature in `v` with an enclosed remainder;
`props/quad.rs`'s rule ("no transcendental is ever evaluated on the
certified path" — the C9 ring is the substrate) does not admit the
integrand as written, so the lane is either a rationalization
(`w = tan(v/2)` turns everything algebraic — the sqrt of a quartic
in `w` remains, Q3(ii) again) or the `Interval` scalar as the
integrand substrate, which is a props-module policy question.

### 4.8 "Certified", per operation

`geom-core/src/real.rs`: `Bounds`/`Enclosure` say *what bracket a
value carries*; `CertifiedEnclosure` says *the computation was
defined on the whole box* (`None` otherwise); a `Dual` never
certifies (DUAL-DESIGN DL1). For an exact carrier every certified
statement is a residual evaluated at `T` against the run's band: the
`Interval` instantiation of §3.2 encloses the true point (`sqrt` and
trig are `Real` operations with correctly rounded enclosures), and
nothing is fitted. What is NOT certifiable today is whatever routes
through the ring — a fitted pcurve, a fitted carrier, props — all
because of the torus's `√`.

## 5. The options, costed

### (a) Permanent fence

Stays red: C5ARMS rows 3, 4, 8 (`docs/VERBS-C5ARMS-SPEC.md`
consumers table — the `verbs_shell` self-retiring row, the
`offd2_r1_probes` late-Err instance, the elbows' `R ± WALL/2` double
spelling), TORAX's klein-elbow half, and every partial revolve of a
circle profile about an off-centre axis. A user sees
`TogetherAxialEdge { "…whose centre is off the axis" }` — true, and
naming the wrong cause (the missing kind); a permanent fence owes a
door that names the spiric. Forecloses nothing in the boolean layer
(the operand gate never reads `route`); leaves the C5 `(Plane,
Torus)` arm half-implemented forever. Cost: one refusal rename, S.

### (b) An exact spiric kind

**(b1) `Curve3::Spiric`** — fields `{ center, axis, u_ref (= n), major_radius,
minor_radius, offset: d, side: ± }`, parameter `v`. Costs: the ~60
non-test dispatch arms of §4 (most one-line typed refusals or
delegations; the load-bearing ones are `eval`/`deriv`/`deriv2`/
`param_near`, `boxes`, `certify.rs:edge_extent`, `mesh/chords.rs`,
`transform`, `step-export`), one new exact `Pcurve` variant (Q3(i))
with its certification arm in `pcurve_cache.rs` and mint arm in
`topo/src/pcurves.rs`, the kind-changing arm in `mint_carrier` with
its side predicate, and — separately funded or not (Q5) — the props
lane. Ratified text touched: D3's curve inventory, C1's ladder (an
exact carrier for one general-rung pair), the C4/C5 pcurve lane
table, `mint_carrier`'s kind law, and the predicate-dimension audit
(new named predicates: the side selection, the reach guard
`(R − r)² − d² > 0` metered through `/2(R − r)`, the speed floor).

**(b2) a general implicit-locus-in-chart kind** — `{ s1, s2 }`
evaluated by a certified 1-D root-find in one chart parameter. It
would also carry the bulb's `cylinder × torus` / `cone × torus` rims
(KERNEL-VERBS row *shell / hollow*), space quartics that are not
plane sections. Cost beyond (b1): every `eval` is a Newton solve
with a per-parameter uniqueness argument (C2.3's tube shape, at every
evaluation instead of once at attachment), `param_near` and boxes
become solves, and the pcurve/props/ring questions return with no
closed form behind them. For plane×torus the root collapses to
§3.2's square root, so (b1) IS (b2)'s plane-section arm, on (b2)'s
parameter (the torus minor angle) — a later general kind subsumes it
without re-parameterization.

**One kind or two?** One kind serves neither the pinch (ellipses,
§6) nor the bulb (space quartics); the spiric is a kind for the
plane×torus pair alone — D3's closed enum adds a variant per
configuration, and that is the honest shape.

### (c) NURBS-fitting — three things that wear one name

- **(c1) a fitted curve as the description** — a `Nurbs` carrier with
  no intrinsic description, the "plausible body whose meters lie near
  the cap". Already forbidden: D2 has no `Explicit` variant and
  `certify.rs:run_checks` refuses a `Nurbs` carrier under any
  derived-image chart description. This is the fenced class RIMCAP
  names; the rejection stands.
- **(c2) the rung-3 fitted carrier** — `Curve3::Nurbs` under
  `Intersection { torus, cap, witness }` with the full C2 certificate.
  This is the RATIFIED general rung (C1), not the fenced class, and
  it is exactly what `route` says the spiric should take — and it
  STAYS the route for every pair without a closed form (§0a). Its cost
  is the torus meters conversion (Q3(ii)), the ℝ³ implicit-pair march
  (`ssi::cylinder_sphere_ssi`'s shape, exhaustiveness and all) run
  from inside a door whose law is "No marching, no SSI"
  (`offset_axial.rs` module doc), and an ε-residual where §3.2 has
  zero. **The crux answered:** a rim is one edge, one `CurveKey`,
  shared by both faces through the topology (`EdgeCurve` is the
  edge's, pcurves are per half-edge caches of it) — sharing is
  structural under EITHER carrier, so a certified envelope does not
  break the rim as topology. It is rejected here on cost and
  exactness, not on principle: the closed form exists, needs no
  ring work, and evaluates to rounding.
- **(c3) `Pcurve::General` at the Fitted grade** — a fitted chart
  image on an exact carrier. Same blocker as (c2)
  (`run_fitted_checks` routes a torus chart to `certify_rung3`, whose
  torus operand poisons), and the envelope statement on a periodic
  analytic chart is `OnLocusHull`, the carrier's incidence with the
  chart — which for an exact carrier says nothing a residual check
  does not already say.

## 6. The pinch half — independent, with the reason

**What #1377 asks for**: the boolean of two equal-radius cylinders
with intersecting axes — two bisector-plane `Ellipse`s crossing at
`p ± r·unit(â₁ × â₂)`, two valence-4 vertices — and the issue's
measured inventory is topology throughout: a four-fragment vertex
factory, chord pairing at a branch point, loop walking through it,
tier-3 valence, the second-order sector verdict at the tangency.
Interim state: `GermFrameCylinderPinch`.

**What SEAT's channel gives it**: recognition only.
`cylinder_cylinder_section` takes `RadiusEvidence::{Declared, None}`
and never reads radii; SEAT-6 (`work/seat/SEAT-6.md`, PR #1593)
mints the per-field `ParamSource` so a production caller can say
`Declared`. Named, the family still refuses at the pinch.

**Why the spiric carrier neither resolves nor shares it**:

1. The pinch curves are exact conics the kernel already carries; no
   carrier is missing. The gap is the vertex/loop vocabulary, which
   a `Curve3` variant cannot touch.
2. The rim spiric never has a node: §3.4, `t < R − (r − t)` for every
   legal `t`. Two smooth disjoint ovals, one of them the rim, both
   `Intersection`-transverse everywhere. No valence-4 vertex is
   minted, so none of #1377's five widenings is exercised.
3. The channel gives the spiric nothing: `d = t` is the caller's
   request magnitude (validated `f64` at the door), and `R`, `r − t`
   flow through `offset_surface`'s struct-update; no identity between
   two stored fields is ever needed.

**Where they touch**: the double-point spiric (`d = R − r`, §3.3) is
the plane×torus member of the pinch family — one node, four arcs,
walls tangent at the node. Reaching it needs a plane cut at a
DECLARED `R − r` (comparing a stored `d` against `R − r` is the
measurement-as-structure move the contract forbids), so it is gated
on #1372's channel exactly as the equal-radius family is, and it
inherits #1377's machinery unchanged if it ever has a consumer.
None does. Record it as a member in #1377's issue.

Two design documents, one cross-reference: RIMCAP's guess predates
the rim regime being computed.

## 7. Recommendation and argument

**Fund (b1) as a carrier unit with the exact pcurve variant (Q3(i)),
and decide Q5 explicitly rather than by omission.**

- Against (a): the fence's sentence names the wrong cause, and the
  fenced consumer is the register's designated shell demo (the Klein
  bottle's elbows, "paid once per wall"). The fix is one closed-form
  variant, not a mechanism.
- (b1) over (c2): exact where the fit is ε-off, no ring change, no
  marcher inside the offset door, no fit — strictly less machinery
  than the ratified route and strictly more exact. A variant per
  configuration is what D3's closed enum is for.
- (b1) over (b2): (b2) replaces the general rung, which nobody asked
  for; (b1) is its plane-section arm and does not foreclose it.
- Q3: the exact pcurve is the honest image of an exact carrier (a
  fitted image of a closed-form curve is (c3)). The ring `sqrt`
  (Q3(ii)) deserves its own conversation: its payoff is the whole
  "blocked on the … exact meters conversion" column of the C5
  table, its cost one correctly rounded operation plus the
  reciprocal-of-`A + 4Rρ` argument the `ssi.rs` module doc sketches.
- Q5: the carrier alone moves the elbow to the lune's door
  (`VolumeUncomputable`) — corner, carrier, pcurves, containment all
  passing, the red-the-day row shape RIMCAP PR-1 shipped. The props
  lane is the larger, numeric unit; funding it later costs nothing
  the carrier unit does not already pay.
- Q4: recommend (i): an export-only spline at the run's ε, certified
  against §3.2 on the CURVE side by the hull machinery the rung-3
  fit already uses (no surface operand enters, so the torus poison
  does not bite); conics already round-trip "only as an
  export/tessellation form" (C1). `StepExportError::UnsupportedCurve`
  is the (ii) vocabulary and exists.

## 8. PR shape and pre-log (if Q1 = (b))

- **PR-1 — the kind.** `Curve3::Spiric` + evaluators + `param_near`
  + boxes + `edge_extent` + transform/lift + chord step + STEP policy
  + every kind-table arm (typed refusals where no closed form is
  owed) + the exact `Pcurve` variant on plane and torus charts +
  `mint_carrier`'s kind-changing arm with its named predicates +
  the audit rows. Acceptance: the elbow's `shell_open`/`shell` move
  from `TogetherAxialEdge` to the lune's `VolumeUncomputable`
  door (payload quoted; the `verbs_shell` and `torax_axial` rows
  flip to that door WITH the old door recorded); a planted red keeps
  `offset_axial_latitude` reachable on a genuine latitude fixture;
  wedge, lune, barrel, teapot and every TORAX row bit-identical; a
  rigid-re-pose parity row; an `Interval`-lane row that certifies the
  same rim. Difficulty **M–H**, class **structural** (breadth across
  ~60 arms; the numerics are three closed forms with stated bounds).
- **PR-2 — props for spiric-bounded faces** (Q5): certified
  quadrature on the cap and the wall. Difficulty **H**, class
  **numeric**; its policy question (substrate) is stated in §4.7 and
  should be ruled before dispatch.
- **Not in either**: C9 sqrt (Q3(ii)), the general kind (b2), the
  pinch machinery, any boolean wall (the operand gate reads kinds;
  no `route` change is made).

## 9. What this lane could not verify

- §4.7's non-elementarity is by the genus argument plus a numerical
  inequality, not a symbolic proof.
- The dispatch-site inventory is `grep`-derived (non-test files
  naming `Curve3::Nurbs`); the compile break at dispatch is the
  exact set.
- STEP import's behaviour on a spline adopted under `Intersection
  {torus, plane}` is read from `certify.rs:run_checks` and C2's
  entry requirement, not executed.
