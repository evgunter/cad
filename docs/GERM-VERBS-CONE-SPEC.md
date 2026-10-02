# GERM VERBS-CONE — the cone as a boolean operand (unit spec)

This spec admits `Surface::Cone` as a boolean operand
(`work/germ/VERBS-CONE.md`). It lists every kind dispatch the admission
reaches (§1), derives each arm the cone needs (§2), and cuts the units in
dependency order (§3). The roster flip is the last unit. Until it lands,
the cone keeps refusing typed at the operand gate. The spec is deleted
when its last unit merges.

It builds on PR 3372 (the section certificate, merged) and PR 3375 (the
circle × torus root lane, in review). §3 names the units that need 3375's
`wall_crossing` restructure.

Read on `e6f3eaaf9`. Line numbers ride beside names and may rot.

## 0. Survey: what was measured, and what moved

**The preview is stale. Since PR 3372, every preview fixture refuses
typed; none answers wrong.** The item's preview was measured on
`378f66744`, before the section certificate merged. I re-ran it on
`e6f3eaaf9` with `Cone` added to both `boolean_arm_exists` and
`revert_arm_exists` in a scratch tree (a throwaway probe, reverted). The
cone is the preview's: the triangle `(0,0) (1,0) (0,1)` revolved fully
about `y`, then `merge_coplanar_faces`. All four ops (∪, ∩, A∖B, B∖A)
gave the same class on every fixture:

| # | B operand | what came back (all four ops) |
|---|---|---|
| P1 | the preview's 300° bite (arc about `(y,z) = (0.45, 0.8)`, `r = 0.3`, `x ∈ [−2, 2]`), no crossings | `FallbackExtentUnsupported`, R-reach (cone × oblique cylinder) |
| P2 | the bite plus the preview's pin through the base disc | `CurvedPairUnsupported { site: InteriorLoopGuard, kind: Cone, other_kind: Cylinder }` |
| P2a | the pin alone | `InteriorLoopGuard`, cone × plane: R-reach. The preview answered this one right; it now refuses. |
| P3 | apex pin `x ∈ [−.02, .02]`, `y ∈ [.5, 1.5]`, `z ∈ [.02, .06]` | `FallbackExtentUnsupported`, R-reach. **The op reached the no-crossings path: see below.** |
| P4 | brick `y ∈ [.2, .3]`, `z ∈ [.9, 1.2]`, clear of the cone, boxes overlapping | `CurvedPierceUnsupported` (the `f2` door) |
| P5 | axis-normal slab `y ∈ [.3, .6]` | `CurvedBooleanUnsupported { kind: Cone }` (the join) |
| P6 | a small box strictly inside the cone | `FallbackExtentUnsupported`, R-reach |
| P7 | a 3π/2 partial cone against the P5 slab | `CurvedBooleanUnsupported { kind: Cone }` (the join, before any containment) |
| P8 | a quarter cone against a brick clear of it inside its box | `CurvedPierceUnsupported` |
| P9 | the cone nested in a big box | correct: ∪ `OperandB`, ∩ `OperandA` (`π/3`), A∖B `Empty`, B∖A `Voided` (`216 − π/3`) |

So the admission is **safe today and useless**: R-reach, the crossing
layer's doors and the join refuse every cone pair whose boxes overlap.
The preview's two wrong answers were the no-crossings path with no cone
gate, and the crossings path with no interior-loop guard. Both were
closed by PR 3372's per-pair pass, which scopes cone faces on both paths
(`ops.rs` `SectionPath::scope`, :765) and answers R-reach for every cone
pair (`section_cert::classify`'s `_ => Intractable`, :461).

**A measured premise-S break, masked by R-reach.** P3's four long edges
cross the cone's lateral face near the apex, at `y = 1 − ρ ∈ [0.937,
0.972]`, inside the face. Yet the op reached `FallbackExtentUnsupported`,
which runs only when the reduction recorded **no** crossing. Each of those
edges has both ends inside the double cone, one in each nappe.
`curved_face_arm`'s `(Negative, Negative) => Ok(CurvedEvent::None)`
(`reduce.rs` :1523) cleared them on convexity, which the double cone does
not have (§2.3). Today R-reach refuses the pair. Premise S (sweep
completeness) is what makes the certificate's W1 and W2 clearances sound,
so **the day a cone × plane row lands without §2.1, P3 becomes a valid,
wrong body.** That fixes the order: the root lanes (U1, U2) land before
the certificate rows (U4). The flip comes after both.

**Also moved:**

- **The extent gates.** `cylinder_extent_gate` and `torus_extent_gate`
  no longer exist. The no-crossings path now runs `sphere_extent_scan`,
  then `section_extent_pass`, which is the certificate on
  `SectionPath::Fallback` (`ops.rs` :979). The "cone arm in the extent
  gates" is therefore the certificate's cone rows, plus one change to the
  sphere scan (§2.6).
- **"A partial revolve refuses every op at `PartialConeFace`"** holds
  only where containment is reached first. P7 met the join first. The
  refusal is real: `cone_trimmed_window` (`solid_contain.rs` :1498)
  refuses any sector wider than π (§2.4).

**No live defect on main.** The operand gate (`first_unsupported_pair`,
`reduce.rs` :284) refuses every cone face whose box meets the partner,
before any of the sites below. It is pinned by `verbs_germarms.rs`
(:256), `verbs_gate_r1_probes.rs` (:386) and `review_m3_pr4.rs` (:590).
A declaration cannot cover a cone pair, because cones are outside the
C8 inventory (`validate_declarations`, `mod.rs` :2596).

## 1. The inventory: every kind dispatch the admission reaches

**How it was swept.** I grepped `crates/topo/src/boolean/`,
`face_normal.rs`, `sector_face.rs`, `chord_join.rs`, `pcurves.rs` and
`geom-brep/src/implicit.rs` for `Surface::Cone`, `SurfaceKind::`,
`S::`/`Sf::` kind patterns and `Surface::Cylinder` (a kind match that
names the cylinder but not the cone is a candidate `_ =>` arm). Every hit
in production code was read in context. Hits in `#[cfg(test)]` modules
were dropped (`boxes.rs` :2015 on, `contact_verify.rs` :467 on,
`join.rs` :2040 on, `mod.rs` :2633 on, `rim_wedge.rs` :551,
`sectors.rs` :816).

**Blind spot.** A dispatch that reads a surface through a helper, and not
through a pattern, is invisible to that grep: `face_plane` returning
`None`, `face_carrier` returning `None`, a chart function returning
`Err`. So there was a second pass over the callers of the helpers that
return `None` or `Err` by kind: `face_plane`, `rest::face_carrier`,
`face_outward_normal_at`, `sector_face`, `circle_residual_extremes`,
`chart_boundary` and `cone_trimmed_window`. The probe (§0) is the third
pass: every fixture went through the whole pipeline, and each refusal
names its raising site.

**Legend.** **H**: handles the cone. **R**: refuses the cone typed.
**S**: would mishandle the cone silently once the roster admits it.

### 1.1 Gates and rosters

| site | status | what it does with a cone |
|---|---|---|
| `reduce.rs` `boolean_arm_exists` (:175), read by `first_unsupported_pair` (:284) | R | `CurvedPairUnsupported { site: OperandGate }`: the gate the flip opens (U7) |
| `reduce.rs` `revert_arm_exists` (:196), read at the ∖/∩ front door (`ops.rs` :451) | R | `site: RevertRoster` (Q2) |
| `reduce.rs` `gate_operand_edges` (:423) | H | edge-carrier gate: lines, circles and ellipses pass, whatever their face's kind |
| `mod.rs` `validate_declarations` `inventory_face` (:2596) | R | `InvalidDeclaration`: C8, cone is not declarable. Stays. |

### 1.2 The sweep and the crossing layer (`reduce.rs`)

| site | status | what it does with a cone |
|---|---|---|
| `sweep_direction` plane lane (:683–950): a cone operand's rims and generators against a plane face | H | `conic_plane_crossing_roots` reads the edge's carrier, not its face's kind |
| `curved_face_arm` with a cone operand's edge against a cylinder, sphere or torus face | H | dispatches on the partner's kind; a cone body's edges are lines and circles |
| `curved_face_arm` NURBS/Approx guard (:1161–1172) | H | the cone is listed and passes to the arms below |
| circle rung: `circle_clearance` against a cone face (:1237) | R | `circle_residual_extremes` has no cone form (`implicit.rs` :653), so it answers `None`, and `.ok_or_else(frontier)?` raises `CurvedPierceUnsupported`. Every circle edge whose box meets a cone face refuses, coaxial rims included (U2). |
| carrier neither line nor circle (:1332) | R | `CurvedPierceUnsupported`, kind-generic |
| `(Zero, ·)` and straddle arms: `wall_crossing` roots (:1753), cone `_ => Unsettled` (:1825) | R | the door (U1, U2) |
| same-side torus root arm (:1510–1519) | R | guarded on `Torus`; a cone falls to the two arms below |
| **`(Negative, Negative) => Ok(CurvedEvent::None)` (:1523)** | **S** | **convexity of the residual along a line: false on the double cone (§2.3). Measured: P3's four crossings are cleared silently. R-reach masks it today.** |
| `(Positive, Positive)` dip bound, `f2` match `_ => return Err(frontier())` (:1579) | R | `CurvedPierceUnsupported` (P4, P8). The cone's `f″` is unbounded near the axis, so the dip bound does not port (§2.3). |
| `vertex_on_curved_face` (:1994) → `curved_face_containment` | H | cone arm, `cone_face_containment` (`contain.rs` :916); a partial sector gives `Trim(None)`, the door (§2.4) |

### 1.3 Containment

| site | status | what it does with a cone |
|---|---|---|
| `contain.rs` `curved_face_placement` cone arm (:628) | H | double-cone carrier first, then the face's own trim. The mirror nappe is `Trim(Out)`, never `OffCarrier`. |
| `solid_contain.rs` `face_geo` (:778), `cast_ray` cone quadratic (:4323) | H | the ray × cone quadratic, nappe and slant window |
| `solid_contain.rs` `cone_trimmed_window` (:1498) | R | `PartialConeFace` for an apex-closed sector wider than π: the walk crosses the apex by nearest branch and reports a full period (§2.4). Through `cone_face_containment` it is `Trim(None)`, which is also a door. |
| `solid_contain.rs` `face_plane` (:528–553) | R | `KindUnsupported`, plane-only by contract; never asked of a cone by the lanes above |

### 1.4 The section certificate, both paths

| site | status | what it does with a cone |
|---|---|---|
| `ops.rs` `SectionPath::scope` (:765) | H | cone faces are in scope on both paths |
| `section_cert.rs` `classify` `_ => Intractable` (:461) | R | R-reach. Crossings path: `InteriorLoopGuard` (P2, P2a). No-crossings path: `FallbackExtentUnsupported` (P1, P3, P6). |
| `section_cert.rs` `torus_pair` partner `_ => Intractable` (:545) | R | torus × cone: R-reach, coaxial included (§2.5.4) |
| `ops.rs` `ChartCache::describes` → `pcurves::chart_boundary` | R (conservative) | an apex-closed cone face refuses `SingularChartJoint`, so W2 is never available on the preview cone's face (§2.4) |
| `ops.rs` `place_witness` (:854) | H | through `curved_face_placement` |
| `ops.rs` `sphere_extent_scan` cone arm (:2274) | R | `CurvedBooleanUnsupported { kind: Cone }` whenever a closed sphere group's ball box meets a cone face. `SectionPath::Fallback` excludes every sphere pair, so the certificate never sees sphere × cone on that path (§2.6). |

### 1.5 Pierce, sectors, join

| site | status | what it does with a cone |
|---|---|---|
| `vtxfac.rs` pierce normal: `face_outward_normal_at` → `Ok(None)` (`face_normal.rs` :217), refused at `vtxfac.rs` :136 | R | `CurvedBooleanUnsupported`, the pierced face a cone |
| `sector_face.rs` `sector_face` (:209) | R | `SectorFaceError::Unsupported` |
| `vtxfac.rs` :225 and :244, `recl.rs` `carrier_of` (:51), both through `rest::face_carrier` (:557) → `None` | R | `CurvedBooleanUnsupported` |
| `sectors.rs` `tangent_lump` (:437), through `geom_brep::tangent_locus` → `Unsupported` | R | `CurvedBooleanUnsupported` |
| `join.rs` germ-pair dispatch `(a_s, b_s) =>` (:456) | R | `CurvedBooleanUnsupported { kind: Cone }` (P5, P7). Every pair with a cone face, the plane × cone pair included. |
| `join.rs` `pair_section_frame` `_ => NoArm` (:943) | R | `GermFrameUnsupported` |
| `chord_join.rs` `bool_planar_chord_spec` wall `_ =>` (:1447) | R | `SectionInvariant`; unreachable behind the join |

**Summary.** 9 sites handle the cone. 21 refuse it typed. **One is
silent: the `(Negative, Negative)` convexity arm.** A second shape shows
up only in the admission's own order: every certificate row that clears on
W1 or W2 rests on premise S, which that arm breaks. The preview's silent
wrong answers (no cone extent gate; the crossings path unguarded) were
closed by PR 3372.

What admission leaves refused **by design**: every pair that reaches the
join. A cone face with an event on it has no join arm (plane × cone
included), and neither the pierce normal nor the sector normal has a cone
arm. Admission in this spec is the torus's shape (PR 3265): it answers
where the cone faces' pairs are certified with no event on them, and it
refuses typed at the join otherwise. The join arms are §3's U8 and Q1.

## 2. The math, per arm

**Notation.** The cone is `(A, â, α)`, with `s = sin α`, `c = cos α` and
`τ = tan α`. For a point `p`, `q = p − A`, `h = q·â` and
`ρ = |q − hâ|`. The carrier is the DOUBLE cone: `S(u, v)` puts `v > 0` on
the nappe that opens along `â`, so `sign h = sign v` is the nappe. Two
forms of the carrier are used:

- the elevation `f = ρc − |h|s` (`geom_brep::cone_elevation` with no
  nappe, the one home of the residual);
- the quadric `Q = ρ²c² − h²s² = |q|²c² − h²`.

`Q = f·(ρc + |h|s)`, and the second factor is `≥ 0`, zero only at the
apex. So **`sign Q = sign f` everywhere except the apex**, and `Q` is a
polynomial along any line or circle, where `f` is not.

### 2.1 Line × cone: the quadratic

The line is `o + t·d`, with `|d| = 1`. Put `w = o − A`, `d_a = d·â` and
`w_a = w·â`. Then

```
Q(t) = a·t² + 2b·t + k₀,   a = c² − d_a²,   b = (w·d)·c² − w_a·d_a,   k₀ = |w|²·c² − w_a²
```

The ray lane already solves this quadratic (`cast_ray`'s cone arm,
`solid_contain.rs` :4335, with all three coefficients negated). There
are three cases, by `a`:

- **`a > 0`**: the line lies outside the aperture. It has 0 or 2 roots,
  and **both are on one nappe**. Along the line `{Q < 0}` is one
  interval. The interior of the double cone has two convex components,
  which meet only at the apex, where `Q = 0`. So the interval lies in one
  of them.
- **`a < 0`**: the line lies inside the aperture. It **always** has two
  roots, one on each nappe, because `Q → −∞` at both ends of the line.
  The one exception is a line through the apex.
- **`a = 0`**: the line is parallel to a generator. `Q` is linear, so
  there is one root or none, and `Q ≡ 0` exactly when the line IS a
  generator.

The discriminant is `Δ = b² − a·k₀`.

- Positive: two roots `(−b ± √Δ)/a`.
- Zero: a **tangency**. That is either a graze along a generator or a
  line through the apex. Near the apex `Q = a·(t − t*)²`, so every line
  through the apex has a double root there. The apex has no tangent plane
  and no crossing order this lane reads, so it keeps the door.
- Negative: a miss, possible only when `a > 0`.

**The unit.** Factor the ray lane's quadratic into
`solid_contain::line_cone_roots(o, d, apex, axis, half_angle, lever,
band) → ConeRoots { GeneratorParallel, Tangent, Miss, Two([t; 2]) }`.
This is the `line_wall_roots` precedent, and it keeps the ray lane's
predicate names `bool_ray_cone_lead` and `bool_ray_cone_disc` with their
metering (`Margin::levered(a, lever)`, `Margin::over_lever(Δ, lever)`).
The lever is the face's slant extent `max(|v_lo|, |v_hi|)`, as in the
ray lane.

`wall_crossing`'s line lane gets a cone arm: `Two` is a certified set,
`Tangent` and `GeneratorParallel` are `Unsettled`, and `Miss` is `Miss`.
The trim places each root. A root on the mirror nappe is `Trim(Out)` by
the slant window's signed bounds (`cone_face_containment` docs), so it
reads as crossed elsewhere. Every root of the quadratic is examined.

**The `(Zero, Zero)` invariant.** The arm's docs say "the lines that lie
on a wall are its rulings, which answer `Constant`". A line on a cone is
a generator, and a generator is `GeneratorParallel`, so `Unsettled`: the
door. The sentence gains the cone clause. It never reads `NoInterior`, so
the chord argument is unchanged.

### 2.2 Circle × cone: a quartic in the half-angle, plus two closed forms

The circle is `C(θ) = C₀ + r·e(θ)`, with `e(θ) = û cos θ + v̂ sin θ`. Put
`δ = C₀ − A`. Then

```
|q|² = |δ|² + r² + 2r·δ·e(θ)         a first harmonic
h(θ) = δ·â + r·(û·â cos θ + v̂·â sin θ)   a first harmonic
Q(θ) = c²|q|² − h²                    a trigonometric polynomial of degree 2
```

So with `φ = θ − θ_a` and `t = tan(φ/2)`, `Q·(1 + t²)²` is a **quartic in
`t`**. That matches Bézout's `2 × 2 = 4`; unlike the torus, no degree is
lost at the circular points.

It uses PR 3375's machinery (`circle_torus.rs`) verbatim:

- the anchor `θ_a` is the arc's midpoint;
- the pole `θ_a + π` must be definitely off the cone. The leading
  coefficient is `Q` there, and `sign Q = sign f` off the apex, so it is
  decided on `cone_elevation` in metres (`bool_circle_cone_pole`);
- a pole on the cone retries at two other anchors;
- the root variable is the length `2r·t`;
- a `QuarticRows` constant holds `bool_circle_cone_*`;
- the lever is the face's slant extent.

**Special poses, decided first and geometrically, in metres:**

- **Coaxial** (the circle's axis `∥ â`, its centre on the axis): `Q` is
  constant. The answer is `CircleConeRoots::Coaxial { elevation }`, and
  the circle rung decides the elevation itself.
  - Definite: the circle never meets the carrier, `CurvedEvent::None`.
  - `Zero`: the circle is a parallel ON the cone, so the door.

  This case matters. A pin coaxial with a cone has rims of exactly this
  pose, and today they refuse at `circle_clearance` (§1.2).
- **Parallel axes** (the circle's plane `⊥ â`, its centre off the axis,
  so `h ≡ h₀`): the plane `h = h₀` meets the double cone in exactly one
  parallel, of radius `ρ₀ = |h₀|·τ`. The crossings are circle ×
  circle in that plane. With `e = |δ_⊥|`, decide
  `bool_circle_cone_reach` on `r + ρ₀ − e` and `bool_circle_cone_nest`
  on `e − |r − ρ₀|`.
  - Both Positive: two roots,
    `θ = θ_δ ± arccos((ρ₀² − e² − r²)/(2re))`.
  - Either Zero: tangent, `Uncertain`.
  - Either Negative: `Miss`.

  `h₀` in the band of 0 (the plane through the apex) is `Uncertain`.
  Here `Q` is a first harmonic, and the quartic would carry the exact
  complex pair `t = ±i`, which is the instrument trap PR 3375 measured on
  the lily. So the closed form is used instead.
- **A circle ON the cone** is only ever a parallel. The planes that cut
  a right circular cone in a circle are the axis-normal ones, so this is
  the coaxial case.
- **A circle through the apex**: `Q` has a double root there, so the
  ladder answers `Uncertain` and the door holds.

**The circle rung.** For a cone face, `circle_clearance` answers `None`
(there is no enclosure form). Today that raises the frontier. After this
unit it routes to the roots instead:

- coaxial is decided as above;
- every other pose falls into the endpoint-sign arms, as PR 3375 routes
  a torus.

Those arms never read convexity for a cone (§2.3), and the
declared-cover arms are closed to circles by 3375's `on_line`.

### 2.3 The convexity arms: why a cone takes the roots for every sign pattern

Along a line, `ρ(t)` is convex and `|h(t)|` is convex. So
`f = ρc − |h|s` is **convex on each side of the apex plane `h = 0`, and
concave across it**. It has a kink there, where `f = ρc ≥ 0`.

- **`(Negative, Negative)`, ends on one nappe:** the interior of a nappe
  is a convex cone, so the segment stays inside. The arm's answer is
  right.
- **`(Negative, Negative)`, ends on opposite nappes:** the segment
  crosses `h = 0`, where `f ≥ 0`, so it crosses the surface at least
  once on each side, or passes through the apex. **The arm's
  `Ok(None)` is wrong.** P3 measures it.
- **`(Positive, Positive)`:** on each side, `f″ = c·ρ″`, and
  `ρ″ = (|d_⊥|²ρ² − (q_⊥·d_⊥)²)/ρ³` is unbounded as the line nears the
  axis. There is no constant `f″` for the dip bound, and the kink breaks
  the chord argument across `h = 0`.

**The replacement:** extend the torus guard at `reduce.rs` :1510 from
`Torus` to `Torus | Cone`. `Q` is exactly quadratic along a line (and a
quartic in `t` along a circle, §2.2), and `sign Q = sign f`, so the
certified roots are exact for every sign pattern:

- `Pierce` is a pierce;
- `NoInterior`, `Elsewhere` and `Miss` are no event;
- `Constant` and `Unsettled` keep the door.

After this, the `(Negative, Negative)` arm and the `f2` match never see
a cone. `f2`'s `_ =>` stays as the documented door. Both arms' comments
name the cylinder and the sphere as the kinds that carry the convexity
they rely on.

**An edge ending AT the apex** (a pin's corner on the tip, two cones
apex to apex) decides `Zero` at the discriminant. It keeps the door,
typed.

### 2.4 The apex closure: containment on partial cones, and W2 on apex-closed faces

**The mechanism.** `face_azimuth_window` (`chord_join.rs` :1392) walks
the outer cycle and pins each edge's azimuth branch by nearest-branch
continuity. At the apex every azimuth maps to one point, so the walk
takes the branch nearest the one it arrived on.

Take an apex-closed sector `[θ₀, θ₀ + W]`. Its cycle is the rim
`θ₀ → θ₀ + W`, then a generator in to the apex at `θ₀ + W`, then a
generator out at `θ₀`. Among the branches `θ₀ + 2πk`, the one nearest
`θ₀ + W` is:

- `θ₀` when `W < π`;
- `θ₀ + 2π` when `W > π`.

So any sector wider than π reads as a full period, and
`bool_cone_trim_period` refuses `PartialConeFace`. The quarter cone
(`W = π/2`) answers; the 3π/2 revolve refuses. At `W = π` the choice is
a tie, but the full revolve's two half-bands are the wrapped class and
never reach it.

**The closure rule.** An apex visit is not a continuity question.

- Lift the non-apex edges by nearest-branch continuity, as today.
- At the apex vertex, set the jump `J` so that the lifted loop closes.

The face's chart region lies in the half-strip `v ∈ (0, V]`, `u ∈ ℝ`,
lifted. Its closure meets the line `v = 0`, which is the apex blown up,
in the segment between the incoming and outgoing azimuths. The lifted
boundary is a closed curve there, so its net `Δu` is zero:
**`J = −Σ Δθ(non-apex edges)`, exactly**.

- The window is the hull of the lifted images, and its width is
  `W = |Σ Δθ|`. For a revolve that is the rim arcs' span, since
  generators contribute nothing.
- `bool_cone_trim_period` then puts `W < 2π` in the trimmed class.
- `W = 2π` is a face that covers every azimuth of its slant window: the
  full-revolve single face with its seam traversed twice, which
  `cone_face_trim` already calls `alone`.

**Its preconditions:**

- exactly one apex visit on the outer cycle;
- no inner loop, since a ringed face stays `Trim(None)` or
  `PartialConeFace`;
- the non-apex walk is today's.

A face that visits the apex twice (a bow-tie) keeps refusing.

**Its two consumers:**

- **Containment.** `cone_trimmed_window` serves both the solid door and
  the face door. It retires `PartialConeFace` for single-apex sectors of
  any width.
- **W2 on apex-closed faces.** `ChartCache::describes` (`ops.rs`) is
  today `chart_boundary(..).is_ok()`, which refuses `SingularChartJoint`
  at the apex. For a cone face it becomes: `chart_boundary` answers `Ok`,
  OR the closure lift closes (one apex visit, no ring).

  This is sound. `int F` excludes the apex, which is on `∂F`. `int F`
  maps homeomorphically onto the interior of the lifted region, which is
  a bounded region of one sheet of the punctured nappe's universal
  cover. So every closed curve in `int F` lifts to a closed curve with
  zero winding, and no essential component lies in `int F`. That is
  exactly W2's premise.

  Without this, no apex-closed face ever clears by W2 (the preview
  cone's merged face included), and §2.5's two-component essential rows
  refuse R-undec whenever a pair has an event.

**Should `chart_boundary` itself learn the closure?** It is the pcurve
minter's door too, and it refuses `SingularChartJoint` for its own
reasons. That is Q4. This spec keeps the rule local to the cone trim and
to `ChartCache`.

### 2.5 The section certificate's cone rows

**Conventions.** The cone is `F`; `swapped()` handles it as `G`.
Components are listed on the DOUBLE cone.

- A component on the other nappe from the face has a witness the trim
  places `Trim(Out)`, so W3 clears it. `classify` stays pure on surfaces
  and never needs the face's nappe.
- Every witness is a closed-form point of the carrier section.
- `single` counts components on the double cone.
- The angular margins take the certificate's pivot and lever
  (`axis_pose`, `section_cert.rs` :569). The aperture margin takes its
  own named lever (the certificate spec's Q9).

#### 2.5.1 Cone × plane

The plane is `(p₀, n̂)`. Let `m_A = (A − p₀)·n̂` in metres, and let the
aperture margin be `μ = |n̂·â| − s`, levered.

| pose | section | parts |
|---|---|---|
| `m_A` definite, `μ` Positive | an ellipse, one closed curve on one nappe | one part, `essential_f`, `single: true`, witness the vertex below |
| `m_A` definite, `μ` Negative | a hyperbola, one branch per nappe | two parts, `unbounded` |
| `m_A` definite, `μ` Zero | a parabola, or a near one: 1 or 2 components, each unbounded or essential | one part, `essential_f`, no witness, `single: false`. W2 clears it when `F` describes; otherwise R-undec. |
| `m_A` Zero, `s‖â×n̂‖ − c|â·n̂|` Negative | the apex alone | `none()`: the apex is never in `int F`. It is on `∂F` for an apex-closed face, and every other describable cone face stays off it, because `cone_nappe` escalates a window that straddles the apex. |
| `m_A` Zero, that margin Positive or Zero | two lines, or one double line, through the apex | one part, `unbounded` |
| `m_A` undecided | — | `Tangent("section_cone_plane_apex")` |

**Why the ellipse is essential.** When `μ > 0`, the plane's direction
set misses the cone's asymptotic directions. So the section on each
nappe is bounded, and a bounded section is met once by each generator of
its nappe. It is a graph over the azimuth, and it winds once about the
axis.

**The witness** is a vertex of the major axis. In the meridian plane
through `A` spanned by `â` and `m̂ = unit(n̂ − (n̂·â)â)` (use the cone's
`u_ref` when `n̂ ∥ â`), the generator LINE `g = c·â + s·m̂` meets the
plane at `A + λg`, with `λ = ((p₀ − A)·n̂)/(g·n̂)`. When `μ > 0`,
`g·n̂ = sin(α + β)` with `cos β = |n̂·â|`, and it is nonzero. That point
lies on the ellipse, on whichever nappe the ellipse is.

**The item's claim, "the ellipse is always essential on a face with a
seam": confirmed, with one qualification.** It is essential on every
cone face.

- It clears by W2 on a face that describes: a frustum band with its
  seam, and after §2.4 an apex-closed face.
- It clears by W4 when evented, or by its witness, on a face that does
  not: a seamless band, or an apex-closed face before §2.4.

It can never be an interior loop. It meets every generator of its nappe,
and so every generator edge's line, which puts it on `∂F` or outside the
slant window at that azimuth.

#### 2.5.2 Cone × sphere

The sphere is `(c_s, ρ_s)`. Let `δ = A − c_s`, `k = |δ|² − ρ_s²`,
`β₀ = c·(â·δ)` and `β₁ = s·|δ_⊥|`, and let `θ₀` be `δ_⊥`'s azimuth. The
generator line at azimuth `θ` is `A + t·w(θ)`, with
`w = c·â + s·r̂(θ)`. Positive `t` is the `v > 0` nappe; negative `t` is
the mirror nappe, at azimuth `θ + π`. It meets the sphere where
`t² + 2b(θ)t + k = 0`, with `b(θ) = β₀ + β₁cos(θ − θ₀)`. Every margin is
in metres.

- **`k` Zero** (`section_cone_sphere_apex`): the apex is on the sphere,
  so R-tan.
- **`k < 0`**: every generator line has `t₋ < 0 < t₊`. That gives one
  component per nappe, a graph over `θ`, essential on the cone. Two
  parts, `essential_f`, with witnesses at `θ₀` for `t₊` and `t₋`, so W3
  clears the mirror one when `F` does not describe.
- **`k > 0`**, with `κ = √k`: the two roots share a sign, `sign(−b)`.
  So nappe `+` needs `b < −κ` and nappe `−` needs `b > κ`. Over
  `b ∈ [β₀ − β₁, β₀ + β₁]`:

| nappe | whole circle: 2 essential components | an arc: 1 null loop (the hazard) | empty |
|---|---|---|---|
| `+` | `β₀ + β₁ < −κ` | `β₀ − β₁ < −κ < β₀ + β₁` | `−κ < β₀ − β₁` |
| `−` | `β₀ − β₁ > κ` | `β₀ − β₁ < κ < β₀ + β₁` | `β₀ + β₁ < κ` |

  Each bound is a margin, and a Zero bound is R-tan: a generator tangent
  to the sphere at an arc's end.
  - The null loop's witness on nappe `+` is at `θ₀ + π`, with
    `b = β₀ − β₁` and `t = −b + √(b² − k)`.
  - On nappe `−` it is at `θ₀`, with `b = β₀ + β₁` and
    `t = −b − √(b² − k) < 0`.
  - `single` is "the total count is 1".
  - With `β₁ = 0` (the centre on the axis) there is no arc case, and
    `θ₀` falls back to `u_ref`.

  Both nappes can carry components at once: a large ball beside the
  apex.

#### 2.5.3 Cone × coaxial partners

"Coaxial" is `axis_pose`'s reading: the tilt levered and the offset read
at the pivot.

- **Cylinder `r_c`:** the meridians meet at `h = ±r_c/τ`, which gives
  two parallels, always. They are essential on both, so
  `essential_pair(true, true)`.
- **Cone `(A₂, α₂)`, with `d = (A₂ − A)·â`:** the meridians
  `ρ = |h|τ₁` and `ρ = |h − d|τ₂` meet at `h = d·τ₂/(τ₁ + τ₂)` (always
  between the apexes), and at `h = d·τ₂/(τ₂ − τ₁)` when `τ₁ ≠ τ₂`.
  - `d` Zero: a common apex, where the cones touch only at the apex or
    coincide. `Tangent`.
  - Otherwise `essential_pair(true, true)` with `single: false`. The
    count, 1 or 2, is never needed: every part is essential, so no margin
    on `τ₁ − τ₂` is needed either.
- **Torus `(C, R, r)`, with `z = (C − A)·â`:** nappe `±` meets the tube
  iff `r − |R·c ∓ z·s| > 0`. That is the distance from the tube's centre
  to the nappe's generator line in the meridian half-plane, and `ρ > 0`
  on the whole tube since `R > r`.
  - Any Positive: `essential_pair(true, true)`.
  - Both Negative: `none()`.
  - Zero: R-tan.

  This lands in `torus_pair`'s coaxial arm, beside the cylinder's.
  §2.2's table in the certificate spec claimed it.
- **A parallel, non-coaxial torus:** R-reach.

#### 2.5.4 Cone × parallel-axis cylinder

The cylinder is at offset `e` with radius `r_c` (`Pose::Parallel`). Its
ruling at azimuth `θ` is at distance `ρ₀(θ) ∈ [|e − r_c|, e + r_c]` from
the cone's axis, and meets each nappe once, at `h = ±ρ₀/τ`.

- Decide `section_cone_cylinder_apex` on `e − r_c`. Zero means a ruling
  runs through the apex: R-tan.
- Otherwise there are **exactly two components, one per nappe**. Each is
  a graph over the cylinder's azimuth, so it is essential on the
  cylinder always. It is essential on the cone iff `r_c > e`, that is,
  iff the cylinder encloses the cone's axis; the same margin's sign
  decides.
- The witnesses sit on the ruling nearest the axis:
  `ρ₀ = |e − r_c|`, `h = ±ρ₀/τ`.

#### 2.5.5 Cone × parallel-axis cone (U5)

The offset is `e > 0`, the axial offset `d`. At height `h` the two
parallels are circles in the plane `⊥ â`: centres `0` and `E` with
`|E| = e`, radii `r₁ = |h|τ₁` and `r₂ = |h − d|τ₂`. They meet iff
`|r₁ − r₂| ≤ e ≤ r₁ + r₂`.

On each of the three pieces cut by `h = 0` and `h = d`, both radii are
affine in `h`. So each boundary (`e = r₁ + r₂`, `e = r₁ − r₂`,
`e = r₂ − r₁`) has at most one root per piece, in closed form. The
meeting set is a finite union of maximal intervals.

- **A bounded interval is one closed component.** The two symmetric
  intersection points join at its ends, where the circles are tangent.
- **An unbounded interval is two unbounded components.** It needs
  `τ₁ = τ₂`.
- **The class.** At an end where `e = r₁ + r₂`, the tangency point lies
  between the axes: angle 0 about each. Where `e = r₂ − r₁` it is at
  angle π about axis 1 and 0 about axis 2, and symmetrically where
  `e = r₁ − r₂`. **A component is essential on cone `i` iff its two ends
  sit at different angles about axis `i`.**
- **The witness:** one intersection point at the interval's midpoint
  height.
- **Degenerate poses:**
  - an interval reaching `h = 0` needs `e = r₂(0)`, that is, apex 1 on
    cone 2. Its margin Zero is R-tan, and the same holds at `h = d`;
  - two interval ends that coincide (the surfaces tangent) are R-tan;
  - `τ₁ − τ₂` in the band is R-tan, since the bounded/unbounded split is
    undecided.

Until this row lands, the pair is R-reach.

#### 2.5.6 What stays R-reach

These pairs stay R-reach:

- cone × oblique cylinder (the preview's bite, P1 and P2);
- cone × tilted cone;
- cone × non-coaxial torus;
- a cone against a NURBS or `Approx` face.

The ruling reduction makes the first two tractable (the certificate
spec's §2.7 and Q5, filed as
`cone-pairs-in-general-pose-have-no-section-arm`). It is not a unit
here.

#### 2.5.7 The degenerate poses, collected

| pose | row | answer |
|---|---|---|
| apex on the partner | plane: `m_A` Zero; sphere: `k` Zero; parallel cylinder: `e = r_c`; parallel cone: an interval end at an apex level; coaxial cone: `d` Zero | plane: the lines or the point, as in the table (never an interior loop). Every other row: R-tan. |
| axis through the partner | plane containing the axis: through the apex, so two lines, unbounded; sphere centred on the axis: `β₁ = 0`, parallels; cylinder: coaxial | the rows above |
| a tangent generator | plane: `m_A` Zero, the double line, unbounded; sphere: an arc-bound margin Zero | W1, or R-tan |
| a tangency elsewhere | any margin Zero | R-tan |

### 2.6 The no-crossings arm

`SectionPath::Fallback` already scopes cone pairs, so §2.5's rows serve
the no-crossings path unchanged. There `evented = false`, W4 never fires,
and a lone component decides by its witness: `In` both is R-loop, `Out`
is W3.

**One change: sphere × cone moves from the scan to the pass.**

- `SectionPath::scope` (`ops.rs` :765) excludes a sphere pair on the
  Fallback path only when the partner is not a cone.
- `sphere_extent_scan`'s cone arm (`ops.rs` :2274) `continue`s past a
  cone face.

This is sound. The scan's contract is "certified disjoint, an escape
(re-cut), or refused". A cone face is never an escape plane (the re-cut
rotates about plane normals), so the pair's only question is
disjointness. On the no-crossings path, "every component cleared" IS
disjointness. With no event, L1 leaves `γ ∩ F ∩ G ∈ {∅, γ ⊂ int F ∩
int G}`, and each of W1, W2 and W3 excludes the second.

The torus has the same shape, and it is Q5.

### 2.7 The cone box (optional, U0)

`ConeSlab` (`boxes.rs` :1515) boxes the full ring at every height of the
face's axial window. `clip_to_boundary`'s argument holds on a cone
verbatim: azimuth is a chart coordinate with no interior extremum, and
the apex's footprint is an axis point that the boundary contains. The
rule's own docs (`boxes.rs` :1215–1228) defer it to "the cone lane".

A tighter box sends fewer pairs to the certificate: P8's quarter cone is
boxed today as a whole frustum. The change moves box-derived baselines;
re-baseline, and say what moved.

## 3. Units, in dependency order

**How the units stay safe.** Every unit before U7 leaves
`boolean_arm_exists` untouched, so the cone keeps refusing typed at the
operand gate, and the three pins stay green through them all
(`verbs_germarms.rs` :256, `verbs_gate_r1_probes.rs` :386,
`review_m3_pr4.rs` :590). No op reaches a cone face before U7, so the
rows before it are verdict-level: in-crate tests call `sweep_direction`,
`wall_crossing`, `classify` and `certify` directly, below the gate. From
U7 on, every body a row returns is checked two ways: a closed-form
volume (`mass_properties`), and `point_in_solid` at named points.

| unit | scope | depends on | cost | review |
|---|---|---|---|---|
| U0 `cone-box-clip` (optional) | `ConeSlab` through `clip_to_boundary` (§2.7) | — | E | orchestrator read |
| U1 `line-cone-roots` | `line_cone_roots`, factored from the ray lane; `wall_crossing`'s line arm for a cone; the same-side guard becomes `Torus \| Cone` (§2.1, §2.3) | PR 3375 merged | M | single |
| U2 `circle-cone-roots` | `circle_cone.rs`: the quartic, the coaxial and parallel-axes closed forms; the circle rung falls through to the roots for a cone (§2.2) | PR 3375, U1 | M–H | dual |
| U3 `cone-apex-closure` | the closure rule in `cone_trimmed_window`; the cone clause of `ChartCache::describes` (§2.4) | — | M | single |
| U4 `section-cert-cone-rows` | `classify`: cone × {plane, sphere, coaxial cylinder, coaxial cone, parallel cylinder}, plus torus × coaxial cone (§2.5.1–2.5.4) | U1, U2 (premise S), U3 (W2 on apex-closed faces) | H | dual |
| U5 `parallel-axis-cone-pair` (optional before U7) | §2.5.5 | U4 | M | dual |
| U6 `sphere-cone-no-crossings` | scope and scan change (§2.6) | U4 | E | single |
| **U7 `cone-roster-flip`** | `Cone` onto `boolean_arm_exists` (and `revert_arm_exists`, Q2); re-pin the three gate rows; op-level rows | U1–U4, U6 | M | dual |
| U8 `plane-cone-axis-normal-join` (after U7; Q1) | the (Plane, Cone) join arm for `AxisNormalCircle`, with the cone's pierce normal and `sector_face` arm | U7 | H | dual |

**U1–U4 must precede any clearing row.** A W1 or W2 clearance on a cone
pair is sound only under premise S, and the `(Negative, Negative)` arm
breaks premise S on the double cone (§0, P3). The flip comes last, so
nothing between them is reachable.

### U0 — cone-box-clip

- **Row, red first:** a quarter cone's box excludes the opposite
  quadrant's point `(−0.5, 0.2, 0.5)`. The mutant omits the clip, and the
  box contains the point.
- **Baselines:** box-derived counts move. Re-baseline them, and say what
  moved.

### U1 — line-cone-roots

**The ray lane stays bit-identical.** Every `bool2_cone_doors.rs` row
passes unchanged. The rows go through `sweep_direction` against the
preview cone:

1. **P3, the apex pin:** four pierces on the lateral face, at
   `y = 1 − ρ` (`ρ ∈ {0.028, 0.063}`, each twice). **Red against the
   mutant "guard not extended"**, which records none. That is the
   silence §0 measured.
2. **A same-nappe `(Negative, Negative)` segment near the axis:** no
   event (`NoInterior` or `Miss`).
3. **A belly chord** (`y = 0.5`, `z = 0.2`, `x ∈ [−2, 2]`): two pierces.
   Red on main: `CurvedPierceUnsupported` (the `f2` door).
4. **P4's edges:** no event. Red on main: the `f2` door.
5. **An edge through the apex:** `CurvedPierceUnsupported`, never no
   event. The mutant reads `Tangent` as `Miss`.
6. **An edge parallel to a generator that crosses the face:** the door
   (Q3).
7. **A counterexample search** (shape 1; seed logged, env override,
   `CAD_FUZZ_EFFORT`): random segments against the cone, where the
   certified interior-root count equals a dense sign-change count of `Q`,
   away from the band.

### U2 — circle-cone-roots

1. **Coaxial rims:** inside the cone, no event; outside, no event. A
   parallel lying ON the cone is the door. Red on main:
   `CurvedPierceUnsupported` at `circle_clearance` for every coaxial
   rim.
2. **A parallel-axis rim crossing the lateral face:** two pierces at the
   closed-form `θ`, within the band. Red on main.
3. **Tilted rims:** 2 and 4 certified roots, against dense sampling (a
   shape-1 search).
4. **A rim through the apex, and a tangent rim:** the door.
5. **A parallel-axis rim a few mm clear of the parallel at `ε = 1e-6`:**
   it answers. This is the lily trap, and the closed form avoids it.
6. **Mutants:**
   - the pole check dropped (an anchor at a root loses it);
   - the coaxial `Zero` read as clear.

### U3 — cone-apex-closure

1. **`point_in_solid` on the 3π/2 partial cone:** interior, exterior and
   boundary points, and a point in the gap's quadrant is `Out`. Red on
   main: `PartialConeFace`.
2. **`curved_face_containment` on that face:** `In` over `[0, 3π/2)` and
   `Out` in the gap. Red on main: `None`.
3. **Widths `π/2` (unchanged), `3π/2`, and the full single face
   (`alone`).**
4. **`describes`:** the preview cone's merged face describes (red on
   main: `SingularChartJoint`), and a two-apex bow-tie still refuses.
5. **The mutant:** nearest-branch continuity restored at the apex.

### U4 — section-cert-cone-rows

1. **The numeric cross-check**, the certificate spec's method (a
   shape-1 search, seed logged). Per row, random poses biased toward each
   margin's zero. Trace the section on a `(u, v)` grid, count components,
   and class each by its seam-crossing parity. The rows must match
   `classify` with **0 mismatches** in count and class.
2. **Every witness is on both carriers within the band.**
3. **The item's ellipse fixture:** W2 on a frustum band with its seam,
   and on the apex-closed face (through U3's `describes`). W4 when the
   face does not describe (a seamless band) and the pair has an event.
4. **Sphere × cone:** a small ball that meets the lateral face in one
   null loop, its own seam turned away and no event. The answer is
   R-loop. **Red against the mutant "arc case read as whole circle"**,
   which clears the loop by W2. That is the row that guards the hazard.
5. **Coaxial cylinder, coaxial cone and coaxial torus:** W2 on seamed
   faces, `essential_pair` counts.
6. **Parallel cylinder:** `e = r_c` gives R-tan; `r_c > e` gives
   essential on both; `r_c < e` gives essential on the cylinder only.
7. **Swapped roles:** each row with the cone as `G`.

### U6 — sphere-cone-no-crossings

- **A ball strictly inside the cone:** the section pass clears it by W0.
- **A ball meeting the lateral face in a null loop, with no crossing:**
  R-loop, `FallbackExtentUnsupported`.
- **Red against the mutant "scan `continue`s without the scope
  change"**: then no one examines the pair, and U7's op row returns a
  wrong body.

### U7 — cone-roster-flip (last)

Re-pin the three gate rows to the door each now reaches. The op-level
rows, all four ops each, use the preview cone, `V = π/3`:

| fixture | ∪ | ∩ | A∖B | B∖A |
|---|---|---|---|---|
| P9 nested in the 6³ box (stays green) | `216` | `π/3` | `Empty` | `216 − π/3` |
| P6 box `0.6² × 0.2` inside | `π/3` | `0.072` | `π/3 − 0.072` | `Empty` |
| P4 brick clear, boxes overlapping | Assembly `π/3 + 0.12` | `Empty` | `π/3` | `0.12` |
| P2a pin through the base disc | `π/3 + 0.012` | `0.004` | `π/3 − 0.004` | `0.012` |
| P10 coaxial pin `r = 0.1`, `y ∈ [−0.5, 0.5]` | `π/3 + 0.005π` | `0.005π` | `π/3 − 0.005π` | `0.005π` |
| 3π/2 cone against a brick clear of it in its box | Assembly `π/4 + v_B` | `Empty` | `π/4` | `v_B` |
| P3 apex pin | typed refusal (pierce normal or join), never a body | same | same | same |
| P1 bite, P2 bite + pin | R-reach, typed, naming the cone face | same | same | same |
| P5 slab, P7 | `CurvedBooleanUnsupported { kind: Cone }` at the join | same | same | same |

- **`point_in_solid`** at `(−0.6, 0.05, 0)` (P2a: `In` both) and
  `(0, 0.25, 0)` (P10: `In` both), and one point per region.
- **The admission's red-first row is P3, run against the mutant "U1's
  guard reverted".** It returns a valid, wrong body, because §2.5.1's
  hyperbola and circle rows clear the pairs. The row asserts that no
  body is returned.
- **`docs/DESIGN.md` :410** ("cone and torus operands refuse") is
  describing text, and it has been stale for the torus since PR 3265.
  `git log -S` finds only the editing pass `99cc678bf`. It is re-worded
  in this PR, as a describing clause, not a design change.

### U8 — plane-cone-axis-normal-join (after the flip)

- **P5:** slab ∩ cone is the frustum
  `π·0.3/3·(0.49 + 0.28 + 0.16) = 0.093π`.
- The unit needs the cone's pierce normal (`face_normal.rs` :217), a
  `sector_face` arm, `bool_planar_chord_spec`'s cone wall, and the
  closure-pinned window (U3).
- The tilted cut is Q1.

## 4. Open questions (⚑ = design fork)

- **Q1 ⚑ The join.** Does U8 (the axis-normal plane × cone join) belong
  to this item, or to its own? The tilted ellipse cut is the larger
  question, and Ev ruled on it on 2026-10-01: the exact tilted ellipse
  is permitted ("exact ellipses are certainly allowed there"). The
  ellipse is in the inventory; the hyperbola and parabola are out by
  decision. `crates/geom-brep/README.md` C1 and C5 route the tilted
  plane×cone section to the exact `Ellipse` (rung 2), and a parabolic
  or hyperbolic section refuses typed, naming its conic.
- **Q2 The revert roster.** Should `Cone` go onto `revert_arm_exists` in
  U7, or wait behind `torus-onto-the-subtract-and-intersect-roster`? The
  probe put the cone on both rosters, and ∖ and ∩ reached the same doors
  as ∪ on every fixture. Recommendation: flip both, with every op-level
  row run under all four ops. That is the orchestrator's call; it is not
  a design fork.
- **Q3 Generator-parallel lines:** keep the door (recommended; the ray
  lane grazes there too), or add the linear root `t = −k₀/2b`. Land the
  root only if a fixture needs it.
- **Q4 Where the apex closure lives:** locally in the cone trim and
  `ChartCache` (recommended), or in `pcurves::chart_boundary`, whose
  `SingularChartJoint` also serves the pcurve minter. Moving it changes
  the minter's contract, so it would be its own unit.
- **Q5 Sphere × torus on the no-crossings path** refuses at the scan
  (`ops.rs` :2274, the same arm as the cone), although `torus_sphere`
  exists (`section_cert.rs` :650). U6's move applies to it verbatim. It
  is a conservative refusal, not a wrong answer.
- **Q6 U5 (parallel-axis cone × cone):** before the flip, or after it as
  a follow-up? Until then the pair is R-reach. It is a refusal, so after
  is safe.
- **Q7 The item's "what a cone admission must carry"** is stale. The
  preview's wrong answers were closed by PR 3372, and the extent gates it
  names no longer exist. The item should cite §0 and the silent arm.
- **Q8 The levers** for the new margins: the aperture margin `μ`, the
  circle lane, and the coaxial and parallel readings. The implementer
  picks each and states it in the predicate's doc; they are not design
  forks.
