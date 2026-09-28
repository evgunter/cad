# GERM section certificate — the interior-loop class, certified per pair (spec)

**Program:** GERM (`work/germ/`). **Items:**
`torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere`,
option (b), and
`cylinder-wall-pair-meeting-in-an-interior-loop-while-crossings-exist-elsewhere`
(P0; `origin/germ/orchestrator`). **Supersedes, when built:**

- every kind half of `ops::interior_loop_verdict`: the torus and sphere
  halves of PR 3336, and the cylinder half the concurrent stopgap lane is
  adding;
- on the no-crossings path, `ops::torus_extent_gate` and
  `ops::cylinder_extent_gate`, and it supplies the cone arm that
  roster lacks.

**Ratified text touched:** none. Nothing here edits `docs/DESIGN.md` or
a crate README design page; §8 names the questions that would.

**Pre-logged difficulty: H** for the whole scope (§6). **M** for the
core slice, which is four pieces:

- torus × {plane, sphere};
- cylinder × {cylinder, plane};
- the sphere half;
- the per-pair rewrite, on both paths.

**Task class: SOUNDNESS.** The numerics are closed forms with square
roots and no solve (§2). The risk is a witness that proves less than it
claims, so every certificate below has its argument and a mutant row
that goes red when the argument is broken (§7).

---

## 0. Survey: what was measured, and what moved

- **The stopgap, read on main `378f66744`.** `interior_loop_verdict`
  refuses a torus face on reach: any undeclared overlapping pair, unless
  `carriers_apart`. It never consults events. It refuses a sphere face
  per pair. Such a pair is cleared by an event when the partner is a
  plane, a sphere or a cylinder. With no event, it is cleared only by
  `carriers_apart` or, for a plane partner, by `circle_misses_a_face`.
  Both refuse as `CurvedPairUnsupported { op: Some(op) }`. Both are
  decided on the reduction and raised where a body would be returned.
- **Premise S holds at the sweep.** Every box-overlapping edge × face
  pair is either a certified miss, a recorded contact, or a typed refusal
  (`reduce.rs` `curved_face_arm`: "Never a silent fallback"). The
  circle × torus pair has no root lane, and it refuses
  `CurvedPierceUnsupported`
  (`circle-crosses-a-torus-face-with-no-root-lane`). It never passes
  silently. §1's lemma rests on this and on nothing else about events.
- **Measured (throwaway probe, reverted): `topo::chart_boundary`
  describes every torus and cylinder face the revolve and extrude doors
  mint.** That covers the full donut's two faces, the half donut's two
  faces, the full-revolve cylinder walls of `annulus-full` and
  `spool-full`, and the spool's full torus band. For the full faces the
  hull is `u ∈ [0, 2π]`, one sheet. It refuses a sphere wall whose loop
  meets a pole (`SingularChartJoint`, `ball-full`, `ball+90`). A cone
  face through its apex refuses the same way (the refusal names "sphere
  pole or cone apex"). §3's W2 uses exactly this.
- **Measured (throwaway numeric probe, reverted; Python on a 720² `(u,v)`
  grid, marching-squares components, seam-crossing parity for the
  homology class).** §2.1's closed-form classification of plane × torus
  and sphere × torus was checked against the traced level sets.
  - 435 random poses, biased towards the axial and near-axial
    directions: 0 mismatches in component count or class. Every class
    occurred: none, one null oval, two `(1,0)`, two `(0,1)`.
  - §2.3's parallel-axis cylinder × torus: 145 poses, 0 mismatches.
  - The geometric margins of §2.1 agree in sign with the algebraic
    discriminants on 200 000 random poses.
- **Measured by the GERM measurement lane (2026-09-28, on `378f66744`),
  and recorded in the item files on `origin/germ/orchestrator`:**
  - **The cylinder is the class, P0.** A wall meets a PARTIAL arc-prism
    wall in one saddle loop that touches no edge, and a pin crosses A's
    cap elsewhere. Every op returns a valid wrong `Seamed` body:
    ∩ = `0.008` against the true `0.0900944`. The same happens tilted
    `0.15` rad. The control without the pin refuses at
    `cylinder_extent_gate`.
  - **The cone, previewed with a scratch roster admission:**
    - cone × a partial cylinder face with NO crossings is wrong under
      every op (∪ `Assembly`, ∩ `Empty`, each difference its first
      operand), because the fallback has no cone extent gate;
    - with a pin added, every op is wrong and `Seamed`.
  - **The backstop's tilted rod** (oblique cylinder × torus, in the half
    donut) is this class. The body built before the backstop drops the
    lens, so the guard is the real barrier.
  - **Sphere × cylinder with two components:** every fixture refuses
    earlier, at `CurvedPierceUnsupported`. The sphere clause has no
    op-level evidence either way, so §7's rows for it are verdict-level.
- **Moved: "a plane can cut a cone in an interior ellipse" is false on
  every cone face the tree can build** (§2.5). An ellipse section
  encircles the axis, so it meets every generator. That includes the
  face's seam generator, or its sector edges. So it is never interior to
  the cone face. It can be interior to the plane face, and that is not
  the class. The fixture stays, as a row that must ANSWER (§7).
- **Moved: "one plane cuts a tube in two loops" is not an interior-loop
  hazard on a face that describes.** §2.1 proves that a two-component
  plane or sphere section of a ring torus is two ESSENTIAL curves. They
  are both `(1,0)` or both `(0,1)`, and neither fits in the interior of a
  face that lifts to one chart sheet. The lone hazard for these partners
  is the single null-homotopic oval: the scrape. The stopgap's "per-pair
  is unsound" is true of a rule that clears on events. It is not true of
  §3's rule, which uses events only where the section has one component.

## 1. The evidence lemma: what an event proves, and what it does not

Let `F` be a face of operand A and `G` a face of operand B. The pair is
undeclared and its certified boxes overlap. Let `Σ = carrier(F) ∩
carrier(G)`, and let `γ` be a connected component of `Σ`. `F` and `G`
are compact and closed. `int X` is the face minus its boundary. The
boundary is every edge and vertex of every loop, seam edges included.

**Lemma L1.** Assume premise S. If `γ ∩ F ∩ G ≠ ∅` and `γ ⊄ int F ∩ int
G`, then the reduction recorded an event of the pair `(F, G)` at a point
of `γ`, or it refused.

*Proof.* There are two cases.

- If `γ ⊂ F ∩ G`, some `x ∈ γ` lies on `∂F` or on `∂G`.
- Otherwise `γ ∩ F ∩ G` is a non-empty proper closed subset of the
  connected `γ`. So it has a relative boundary point `x`: `x` is in
  `F ∩ G`, and it is a limit of points of `γ` outside `F ∩ G`. So `x`
  is on `∂F` or on `∂G`.

Either way, `x` lies on an edge `e` of one face, inside the other face.
Take `e ⊂ ∂F` with `x ∈ G`, the other case being symmetric. Both boxes
contain `x`, so the sweep examines `e × G`, and by S it records the
contact at `x` or refuses. Seam edges are edges, so a seam crossing is
recorded as well. ∎

The recorded contact names a vertex that bounds `F` and lies on `G`, or
the mirror. `event_pairs` pairs exactly those
(`ops.rs` `event_pairs`, `faces_by_vertex`).

**Corollary.** The components that no event can evidence are exactly
those with `γ ⊂ int F ∩ int G`. **The certificate's whole job is to
prove, for every component `γ` of `Σ`, either that `γ ∩ F ∩ G = ∅` or
that `γ ⊄ int F ∩ int G`.** Tracing an evidenced component correctly is
the crossing layer's contract, not this certificate's (premise P-trace,
§8 Q7).

**Why a pair's events do not clear it (the corner-bar lesson).** An
event is a point of SOME component. With `Σ` split into `γ₁ ∪ γ₂`, an
event on `γ₁` says nothing of `γ₂`, and `γ₂ ⊂ int F ∩ int G` stays
unevidenced. The rule below never reads events per pair except in two
places:

- **W4:** `Σ` has exactly one component on `F`'s nappe, so every event
  lies on it.
- **The no-event decision:** with no event on the pair,
  `γ ∩ F ∩ G` has no relative boundary point, so it is empty or all of
  `γ`. One point then decides.

A pair with events on some components and none on others is handled per
component. The evented ones clear by W4, when the count is 1, or by
W1/W2/W3. Each unevented one needs its own W0–W3 witness, or the pair
refuses. §2 shows that in the tractable set every multi-component
section is essential on a carrier, so W2 carries it. The shape the
lesson warns about, two null components on one face pair, occurs only
in pairs this spec refuses anyway. Two coplanar-axis tori whose tubes
graze at two places are an example (§2.4).

## 2. Component structure per kind pair, in closed form

**Notation.** The torus is `(C, a, R, r)`, and it is a ring torus,
`R > r > 0`. The kernel's convention is `geom::ring_torus` and
`require_ring_torus`. Genus 1 and the Morse count below need it.

Every surface of revolution is written in its meridian half-plane
`(ρ, z)`. `u` is the azimuth and `v` the tube angle, so the torus point is
`((R + r cos v) cos u, (R + r cos v) sin u, r sin v)`.

A closed curve on the torus has a class `(p, q)`: `p` windings about the
axis, and `q` about the tube. **Null** means `(0, 0)`. **Essential** means
non-null in the carrier minus its chart singularities: the poles of a
sphere, or the apex of a cone. On a cylinder, cone or sphere, essential
means the curve encircles the axis.

### 2.1 Torus × plane and torus × sphere: one classification

Take the meridian plane `Π` that contains the axis and either the plane's
normal `n` or the sphere's centre `c`. The configuration is symmetric
under reflection in `Π`, and every tangency between the carriers lies in
`Π`.

- For the plane, the torus's height `n·P` is critical only where the
  torus normal is parallel to `n`.
- For the sphere, `|P − c|²` is critical only where the normal line
  passes through `c`.

A torus normal lies in its own meridian plane, so both kinds of critical
point sit at `u' ∈ {0, π}`. Here `u'` is the azimuth measured from `Π`.

**Reduction.** Substitute the torus chart, with `s = |n⊥|` and
`n_a = n·a` for the plane. Both equations become

`cos u' = g(v) = (K + A cos v + B sin v) / (s·(R + r cos v))`

- **plane:** `K = h`, `A = 0`, `B = −n_a r`, with `h = n·(p₀ − C)`, the
  plane's signed offset from the torus centre;
- **sphere:** divide through by 2, with `s = |c⊥|`, `c_a = c·a` (`c`
  relative to `C`), `ρ_s` the sphere radius, and
  `K = (R² + r² + |c|² − ρ_s²)/2`, `A = Rr`, `B = −c_a r`.

The denominator is positive for a ring torus, so `g` is continuous on
the `v`-circle. `S = {v : |g(v)| ≤ 1}` is where the section lives. Over
each arc of `S` the section is the two branches `u' = ±acos g(v)`. The
equation `g = σ`, with `σ = ±1`, is linear in `(cos v, sin v)`, so it has
0 or 2 simple roots, or a double root at a tangency. The ways the arcs
can close up are therefore few:

| roots of `g = +1`, `g = −1` | `S` | components | class |
|---|---|---|---|
| 2, 2 | two arcs, each running `+1 → −1` | 2 | both `(1,0)` |
| 2, 0 or 0, 2 | one arc, `+1 → +1` (or `−1 → −1`) | 1 | null: **the scrape oval** |
| 0, 0 | the whole circle, or empty | 2 or 0 | both `(0,1)`, or none |

A two-arc `S` with ends `(+,+)` and `(−,−)` would need a third root of
one equation, so it does not occur. Neither does a lone `(1,0)` arc. So
**Σ has at most 2 components; at most one is null; and a two-component
Σ is essential.** This agrees with the Morse count. For `n` not
parallel to `a`, the height on a ring torus has 4 critical points: a
maximum, two saddles and a minimum. A level set therefore goes
1 → 2 → 1 components across the saddles, and a two-circle level set is
the boundary of an annulus with an essential core.

**The margins.** They are geometric, in metres, and have no division.
Let `d_σ` be the distance from the partner to the tube circle `T_σ`. That
circle lies in `Π` at the core point `u' = 0` for `σ = +` and `u' = π`
for `σ = −`.

- **plane:** `g = σ` has roots iff the plane cuts `T_σ`.
  - `m_σ = r − |h − σ s R|`
  - "whole": `m_w = s R − r − |h|`
- **sphere:** let `d_σ = hypot(R − σ s, c_a)`, the distance in `Π` from
  the sphere centre to `T_σ`'s centre. The sphere cuts `T_σ` iff the two
  circles of `Π` cross:
  - `m_σ = min(d_σ + r − ρ_s, ρ_s − |d_σ − r|)`
  - "whole": `m_w = min(ρ_s − d₊ − r, d₋ − r − ρ_s)`

Decide `m₊` and `m₋` on the band. Any `Zero` or undecided margin is the
tangency class, and it refuses (§4). Then:

- **both positive:** two `(1,0)`;
- **exactly one positive:** one null oval, on the `u' = 0` side when
  `m₊ > 0` and on the `u' = π` side otherwise;
- **neither:** two `(0,1)` or none. The distinction is not load-bearing
  when `F` describes, since W0 and W2 both clear. `m_w` decides it only
  when `F` does not describe.

**The witness point of the scrape oval** is where the oval crosses `Π`.
There the oval meets `T_σ`, and it does so in closed form with one
square root:

- for the plane, it is a line against a circle in `Π`;
- for the sphere, it is a circle against a circle in `Π`.

The two crossing points are the ends of the arc at `u' = 0` (or `π`).
Either one serves.

The in-plane direction `n⊥ / s` is ill-conditioned as `s → 0`. A scrape
oval there lives only in a band of width `2sR` about `|h| = r`, and on
the Interval lane the enclosure carries the conditioning. A point whose
containment verdict the band cannot decide is not a witness (§3).

`s = 0` exactly (a normal along the axis, or a centre on the axis) needs
no special case: `m₊ = m₋`, so the section is two parallels or nothing.

### 2.2 Torus × {cylinder, cone, torus, sphere}, coaxial

Both surfaces revolve about one axis, so `Σ` is the union of the
parallels through the points where their meridians meet in the
`(ρ, z)` half-plane:

| partner | meridian | points met | components |
|---|---|---|---|
| cylinder | a line | ≤ 2 | ≤ 2 |
| cone | a ray per nappe | ≤ 2 per nappe | ≤ 4 on the double cone, ≤ 2 per nappe |
| torus, or a sphere centred on the axis | a circle | ≤ 2 | ≤ 2 |

Each component is a parallel: `(1,0)` on the torus, and it encircles
the partner's axis. All are essential, and the meeting points are circle
against line, or circle against circle, in closed form. A tangency is a
double point, so it has a zero margin and refuses. **Coaxial** is itself
decided on the band: `|a₁ × a₂|` and the offset between the axes are
`Zero`. An undecided margin is not coaxial, and the pair falls to §2.4.

### 2.3 Torus × cylinder, axis parallel to the torus's

Let the cylinder have radius `ρ_c` and axis offset `e ≥ 0` from the
torus axis. Parametrise it by `θ`, with `ρ(θ)² = e² + ρ_c² + 2eρ_c cos θ`.
`ρ` is even in `θ` and monotone on `[0, π]`. Over the `θ`-set where
`R − r ≤ ρ(θ) ≤ R + r`, `Σ` is `z = ±√(r² − (ρ(θ) − R)²)`. The two
branches join where `ρ = R ± r`.

Let `ρ_min = |e − ρ_c|` and `ρ_max = e + ρ_c`.

| where `[ρ_min, ρ_max]` falls | components | class on the torus | class on the cylinder |
|---|---|---|---|
| inside `(R − r, R + r)` | 2 (`z > 0`, `z < 0`) | `(1,0)` if `ρ_c > e`, else null | encircling (essential) |
| across exactly one bound | 1 | null | null: **the hazard** |
| across both bounds | 2 | `(0,1)` | null |
| outside | 0 | — | — |

Every margin is a difference of `ρ_min` or `ρ_max` against `R ± r`. The
witness of the single null loop is `θ = 0` or `θ = π`, whichever lies in
the band, at `z = ±√(…)`.

### 2.4 Torus pairs with no tractable closed form

Three pairs have no closed-form classification here:

- torus × cylinder with an oblique axis;
- torus × cone, not coaxial;
- torus × torus, not coaxial.

`Σ` is a degree-8 curve in these cases. Each cylinder ruling meets the
torus in a quartic (`line_torus_roots`). The component count changes at
the double roots of that quartic in `θ`, a resultant of high degree, and
no bound or class argument is claimed.

Two null components on one pair do occur. Two tori with parallel axes,
whose tubes graze at two places, are an example. So the refusal must not
consult events: **it refuses on reach, per pair, unless
`carriers_apart`** (§4).

One sub-case is tractable but deferred (§8 Q8): the cylinder axis
perpendicular to the torus axis, as in a radial hole through the tube.
There `sin(u − φ)` solves a quadratic in `v`, and the arc ends are roots
of a degree-8 polynomial.

### 2.5 Cone × plane

On the double cone, the section is one conic, and each nappe carries at
most one component of it:

| conic | what lies on a nappe | class |
|---|---|---|
| ellipse | the whole curve, on one nappe | encircles the axis: essential |
| parabola | the whole curve, on one nappe | unbounded |
| hyperbola | one branch per nappe | unbounded |

A plane through the apex gives two rays, one ray, or the apex alone on
each nappe. A plane tangent along a generator gives a double line. Both
are unbounded or empty. So **every component on `F`'s nappe is
essential or unbounded, in every pose, tangent ones included.**

When `F` describes (§3 W2), the pair clears with no margin, no witness,
and no decision about which conic it is. W1 needs no chart. When `F`
does not describe (a pointed cone: the apex is on `∂F`), the count on
the nappe is at most 1, so W4 or the one-point decision applies.

- The witness is the conic's point in the plane of `a` and `n`.
- If the ellipse-or-parabola margin `|n·a| − sin α` (which needs a lever
  to put it in metres, §8 Q9) is undecided, only W4 or a W3 point clears
  the pair. A point `In` both, with no event, is then R-undec rather than
  R-loop, because an unbounded component in both faces would contradict
  L1.

### 2.6 Cone × sphere

Put the apex at `A`, and let `δ = A − c` and `k = |δ|² − ρ_s²`. The
generator at azimuth `θ` is `A + t·w(θ)`, with `t > 0` on the `F`
nappe. It meets the sphere where `t² + 2b(θ)t + k = 0`, with
`b(θ) = w(θ)·δ = β₀ + β₁ cos(θ − θ₀)`, a sinusoid.

| case | components on each nappe | class |
|---|---|---|
| `k < 0` (apex inside the ball) | 1 (the roots have opposite signs) | encircling |
| `k > 0`, `{b ≤ −√k}` is an arc | 1 | null: **the hazard** |
| `k > 0`, `{b ≤ −√k}` is the whole circle | 2 | both encircling |
| `k > 0`, `{b ≤ −√k}` is empty | 0 | — |
| `k = 0` (apex on the sphere) | — | degenerate: refuse |

The margins are `k`, `|β₀| − |β₁| ∓ √k` and the like. The witness of
the null loop is at `θ = θ₀ + π`, where `b` is least, and it takes one
square root. A sphere centred on the axis (`β₁ = 0`) gives circles.

### 2.7 Cone × cylinder, cone × cone

- **Coaxial:** parallels, as in §2.2.
- **Cylinder axis parallel to the cone axis, apex off the cylinder:**
  each ruling meets each nappe exactly once, so each nappe carries one
  component. It encircles the cylinder, which makes it essential.
- **General pose:** the ruling reduction. Each cylinder ruling (a fixed
  direction) meets the cone in a quadratic whose leading coefficient is
  constant. Its discriminant `D(θ)` is a trigonometric polynomial of
  degree 2, with ≤ 4 roots by Ferrari. Every arc of `{D ≥ 0}` is one
  component that does not encircle the cylinder. A whole circle gives
  two encircling components.

  Cone × cone reduces the same way through the rulings of one cone,
  with a non-constant leading coefficient. That brings in asymptotic
  unbounded branches and splits at the nappe sign.

  Both pairs are **tractable, deferred** (§8 Q5), and they refuse on
  reach per pair (R-reach) until built. The measurement lane's cone
  preview fixture (a cone about `y` against a partial cylinder about `x`)
  is this pose. It refuses, as it must.

  A nonsingular intersection of two quadrics is an elliptic quartic with
  at most two real projective components. That is the standard QSIC
  classification, not re-derived here, and nothing in this spec leans on
  it.

### 2.9 Cylinder × cylinder

Let `m̂` be the unit vector along `d₁ × d₂`, and let
`δ₀ = (o₂ − o₁)·m̂`, the signed common-perpendicular distance between
the axes.

A ruling of cylinder 2 at azimuth `θ` is a line with direction `d₂`
through `o₂ + r₂·e(θ)`. `m̂ ⊥ d₂`, so `m̂` lies in cylinder 2's
cross-section plane, and `e(θ)·m̂ = cos(θ − φ)`. The ruling's distance to
axis 1 is therefore exactly `|δ₀ + r₂ cos(θ − φ)|`. The ruling meets
cylinder 1 twice, once (tangent), or never, as that distance is below,
at, or above `r₁`.

Over each arc of `{θ : |δ₀ + r₂ cos(θ − φ)| ≤ r₁}` the two roots join at
the arc ends, so each arc is one component. The whole circle gives two
components, `t > 0` and `t < 0`. With `r_min` and `r_max` the smaller and
larger radius:

| pose | components | class |
|---|---|---|
| `|δ₀| + r_min < r_max` | 2 | each encircles the THIN cylinder's axis: essential there |
| `|r₁ − r₂| < |δ₀| < r₁ + r₂` | 1 | null on both walls: **the saddle loop, the P0 hazard** |
| `|δ₀| > r₁ + r₂` | 0 | — |
| an equality | pinched | tangency: R-tan |

This is pose-generic: the angle between the axes never enters.

- **Parallel axes** (`|d₁ × d₂|` decided `Zero`): common rulings, which
  are lines and so unbounded (W1), or nothing.
- **Undecided `|d₁ × d₂|`:** R-tan, because a near-parallel pair can
  carry a long null saddle loop.
- **The margins:** `r₁ + r₂ − |δ₀|`, `|δ₀| − |r₁ − r₂|`,
  `r_max − r_min − |δ₀|`.
- **The saddle loop's witness:** the arc's middle ruling, at
  `cos(θ − φ) = ±1`, whichever lies in the set, with
  `t = (−b ± √D)/a`. That is one square root, and `a = |d₁ × d₂|²` is
  decided away from zero.

This is the item's own argument ("one loop that encircles neither axis,
or two loops that each encircle the thinner wall's axis"), derived here
rather than measured.

On the P0 fixture, `r₁ = 1`, `r₂ = 0.5` and `|δ₀| = 1.3` fall in the
middle row: one null loop. It has no event on the pair. Its witness
lies on the arc's inner ruling, `y = 0.8, z = 0`, where that ruling
meets A at `(±0.6, 0.8, 0)`, and it is strictly inside both faces. So
the pair is **R-loop**. The same holds tilted `0.15` rad, since the
table never reads the angle.

### 2.10 Cylinder × plane

Every component of a plane section of a cylinder is essential or
unbounded:

- an ellipse encircles the axis;
- a plane parallel to the axis gives two lines, a tangent double line, or
  nothing.

So the pair clears by W1 or W2 in every pose, tangent ones included, and
it needs no margin. This is the argument `cylinder_extent_gate`'s plane
exemption already makes. W2 is where it checks the "wall carries a
meridian edge" premise per face, instead of assuming it.

Cylinder × {sphere, torus, cone} are §2.8, §2.2–§2.4 and §2.7.

### 2.8 Sphere × {plane, sphere, cylinder}: checking the stopgap's claim

- **plane, sphere:** one circle. ✓
- **cylinder:** let the axis be at distance `e` from `c`, with the
  cylinder parametrised by `θ`, and `q(θ)² = e² + ρ_c² + 2eρ_c cos θ`.
  Then `Σ` is `t = ±√(ρ_s² − q(θ)²)` over `{q ≤ ρ_s}`. That set is one
  arc symmetric about `θ = π`, which gives **one** component, encircling
  neither the axis nor the poles in general. Or it is the whole circle,
  which gives **two** components, each encircling the cylinder (`t > 0`
  and `t < 0`). ✓
- At `q_max = ρ_s` exactly (Viviani) the two components pinch into a
  figure-eight. That is still one connected curve crossing the seam, so
  the claim holds there too.

The stopgap's argument is sound, given its unstated premise that every
cylinder wall carries a meridian edge (`cylinder_extent_gate`'s docs
state the same premise). §3 replaces that premise with a per-face
check.

## 3. The certificate

The certificate runs on each undeclared pair `(F, G)` whose boxes
overlap, where either face is a torus, a sphere, a cylinder or a cone
(the cone once admitted; §8 Q3). A plane × plane pair is not this class,
since two planes meet in a line. Per pair:

1. **W0, apart.** Either `carriers_apart` holds, or the §2
   classification returns no component. → clear.
2. **Classify.** Take the §2 arm for the kind pair and the pose. If
   there is no arm, or the pose is intractable, refuse on reach
   (**R-reach**). If one of the arm's margins is `Zero` or undecided,
   refuse as a tangency (**R-tan**). The plane arms of §2.5 and §2.10
   have no margins on this path. Otherwise the result is a list of components,
   each carrying:
   - `unbounded: bool`;
   - `essential_on: {F-carrier?, G-carrier?}`;
   - the nappe it lies on, for a cone;
   - one closed-form witness point `p_γ`.
3. For each component `γ` on `F`'s nappe (and `G`'s), clear it if any
   one of these holds:
   - **W1, unbounded:** `γ` is unbounded. Faces are compact, so
     `γ ⊄ F`.
   - **W2, essential:** `γ` is essential on the carrier of `X ∈ {F, G}`,
     and `chart_boundary(X)` returns `Ok`. An `Ok` means `X`'s loops lift
     into one sheet of the chart's cover: no `LoopWraps`, no
     `OuterSpansPeriod`, no `SingularChartJoint`. So every closed curve in
     `int X` lifts to a closed curve and has zero winding in every
     periodic channel, and `γ ⊄ int X`. Compute this once per face and
     cache it.
   - **W3, point Out:** `p_γ` is certified `Out` of `F`, or `Out` of `G`.
     Use `curved_face_containment` for the curved face and `contfp` for
     the plane. Then `γ ⊄ F ∩ G`.
   - **W4, lone component with an event:** `Σ` has exactly ONE component
     on `F`'s nappe, and `event_pairs ∋ (F, G)`. The event's vertex lies
     on `carrier(F) ∩ G` or `F ∩ carrier(G)`, and within the band it lies
     on `Σ`, hence on `γ`. It also lies on `∂F` or `∂G`, so
     `γ ⊄ int F ∩ int G`. Tangency is irrelevant here: the vertex is a
     boundary point, however it was met.
4. **The no-event decision.** If no witness cleared `γ` and
   `event_pairs ∌ (F, G)`, then `γ ∩ F ∩ G` is `∅` or `γ` (§1).
   - `p_γ` certified strictly inside both faces means `γ ⊂ int F ∩ int G`:
     a **definite interior loop**. Refuse **R-loop**.
   - Anything else (on a boundary, or no verdict) refuses **R-undec**.
5. **Otherwise** (the pair has an event, `Σ` has ≥ 2 components, and
   `γ` has no W1–W3), refuse **R-undec**. By §2 this does not occur in
   the tractable set on faces that describe. It is the guard for the
   day an arm is added whose sections break that property.

**One witness point per component is complete** in the tractable set:

- count 1 with an event is cleared by W4;
- count 1 without an event is decided by one point;
- count 2 is essential, so W2 carries it.

More sample points would add answers only where W2 is unavailable (a
pole or apex joint). Adding them is optional and changes no soundness
argument.

**Premises the certificate rests on,** each named at its site:

- **S:** sweep completeness (§0).
- **Ring torus.**
- **Band-closeness of event vertices to `Σ`.** This is the posture
  `circle_misses_a_face` already takes.
- **Witness points are enclosures on the Interval lane,** and within
  rounding of `Σ` on `f64`, with containment decided on the band.
- **Lone-vertex loops.** A vertex of `F` that lies on `G` with no edge
  (`LoopBoundary::Empty`) must be seen by the reduction, or refused.
  W2 and W3 never clear a component that touches `∂F` only at such a
  vertex. W4 could, so the implementer confirms that the reduction
  either records such a vertex or refuses, and adds a row.

## 4. What it returns, and the refusals

| outcome | meaning | payload today | proposal (§8 Q1) |
|---|---|---|---|
| clear | every component of every pair is W0–W4 | — | — |
| R-loop | a component certified in `int F ∩ int G` with no event on the pair | `CurvedPairUnsupported { op: Some(op) }` | a distinct variant naming both faces and the witness point: the one refusal that is a definite fact, and the one a cut-in (Q2) would answer |
| R-tan | a classification margin `Zero` or undecided | same | same variant as R-undec, with the predicate named |
| R-reach | intractable pair or pose (§2.4, §2.7 deferred, NURBS) with overlapping boxes, not apart | same | unchanged |
| R-undec | step 4's non-decision, or step 5 | same | as R-tan |

`event_pairs` stays as it is. W4 and step 4 are its only readers.

## 5. Placement

- **The crossings path:** exactly where `interior_loop_verdict` runs
  now. It is decided on the unmutated reduction, before
  `enter_join_surgery`, and raised only where a body would be returned
  (the gate site and the REST door), so every earlier refusal stands
  verbatim.
- **Per pair, not per op.** The torus half's per-op reach is replaced:
  - §2.1 through §2.3 answer per pair;
  - R-reach is itself per pair, and it never consults events, so a
    per-pair refusal on reach is as sound as a per-op one.
- **The concurrent cylinder stopgap** (the P0 item's lane) is replaced
  the same way. Whatever form its cylinder half takes, §2.9, §2.10, §2.8,
  §2.3 and §2.7 subsume it.
  - If it refuses on reach, those refusals become classified verdicts.
  - If it clears evented wall pairs by the item's "one loop, or two
    loops each encircling the thinner axis" argument, that is W4 plus W2.
    The W2 half now checks `chart_boundary` on the thin wall, where the
    stopgap assumes the meridian-edge premise.
  - The P0 fixture stays refused, now as R-loop (§2.9).
  - Take the stopgap's rows over unchanged, apart from the payload
    (§8 Q1).
- **The no-crossings path: one per-pair pass over every curved kind.**
  - **What it replaces:** `torus_extent_gate`'s and
    `cylinder_extent_gate`'s blanket reach refusals. It also adds the
    **cone arm** that path lacks: the measurement lane's cone ×
    partial-cylinder preview was wrong under every op with no crossings,
    because the fallback's roster was only those two gates and the
    sphere scan.
  - **Why it is decisive:** with no events anywhere, W4 never fires,
    step 4 decides every null component, and W1/W2 clear the essential
    and unbounded ones. L1 says such a component meeting `F ∩ G` would
    carry an event, and there are none.
  - **What it cannot classify:** an intractable pose (R-reach) refuses,
    exactly as the old gates did. A cone face in any pose outside §2.5,
    §2.6 and the tractable rows of §2.7 refuses on reach, and so does the
    preview fixture.
  - **The payload:** the refusal variant there stays
    `FallbackExtentUnsupported`.
  - **Ordering:** `sphere_extent_scan` runs first and keeps its re-cut
    machinery, a different answer that is not this unit's to narrow. Its
    sphere faces are left to it. The pass then runs over torus, cylinder
    and cone faces.
  - **Its rows:** the cone arm runs only once the operand gate admits
    cones. Until then its rows are verdict-level (§7).
- **The bound.** `chart_boundary` needs `T: PcurveFittedLane`, which
  `boolean_op_recut` already carries. Widen the verdict's bound to match.
- **The code:** a new `crates/topo/src/boolean/section_cert.rs`, holding
  the classifier per kind pair (pure: surfaces and band in, components
  out) and the per-pair rule. `interior_loop_verdict` shrinks to the
  loop over pairs. `carriers_apart` and `circle_misses_a_face` fold into
  the classifier as W0 and the plane arm's W3.

## 6. Scope: what it gives back, and what it still refuses

**Given back (refused by the stopgap, answered by the certificate):**

1. **The pin-only bracket against the half donut**
   (`the_pin_alone_is_the_torus_guards_conservative_refusal`).
   - The pin's `x = 1.95` and `x = 2.05` faces cut scrape ovals, which
     have no event. The witness point `(1.95, ±0.4975, 0)` is `Out` of the
     pin face (`|y| > 0.3`), so W3 clears them.
   - Its `z` faces and the bracket's other planes clear by W2 (`(0,1)`
     pairs) or W0.
   - Its `y = ±0.3` faces clear by W2 (`(1,0)` pairs).
   - The union's closed form is `π²/2 + 0.594 − 0.006`.
2. **The donut-hole cube** (the no-crossings fallback;
   `torus_extent_gate`'s docs name it as its known conservative refusal).
   Its side planes cut `(0,1)` pairs and its caps `(1,0)` pairs, or stand
   apart, so all clear by W0 or W2.
3. **Any torus × plane or torus × sphere pair whose section is
   essential,** or whose scrape oval has an event or a point outside a
   face. That covers the bars, slabs, pins and balls through a donut
   that do not truly leave a loop inside both faces.
4. **Coaxial torus pairs** (§2.2) and **parallel-axis cylinders through
   a tube** (§2.3: a pin through the tube clears by W2 on the cylinder).
5. **The sphere half:**
   - sphere × sphere with no event, cleared by W3 (the stopgap had W3
     for the plane only);
   - sphere × cylinder with no event: W3 for the one-loop case, W2 for
     the two-loop case;
   - sphere × torus: any pose is classified (§2.1), where the stopgap
     refused unless the two stood apart.
6. **Cylinder pairs, against the concurrent cylinder stopgap, if that
   stopgap refuses on reach:**
   - wall × wall pairs whose section is two thin-axis-encircling loops
     (a rod through a fatter wall, W2), or apart;
   - evented saddle loops (W4);
   - saddle loops with a witness outside a face (W3);
   - every wall × plane pair (W1 or W2).

   Against a stopgap that clears on events, the certificate gives back
   only the no-event W3 cases.
7. **The no-crossings path:**
   - the cylinder wall pairs `cylinder_extent_gate` refuses on reach
     that stand apart, or whose section is essential (W0 or W2);
   - the no-event saddle loops whose witness is `Out` of a face (W3).
8. **The ∖/∩ torus roster admission** (`torus-onto-the-subtract-and-intersect-roster`),
   which is parked on this certificate.

**Still refused:**

- **Definite interior loops** (R-loop), which refuse until a cut-in
  exists (§8 Q2):
  - the half-donut bracket;
  - the dome bracket;
  - a pin straddling the outer equator inside both faces;
  - **the cylinder P0 fixture**, and its tilted variant;
  - its pin-less control, on the no-crossings path.
- **Tangent and near-tangent poses** (R-tan), including the Villarceau
  pose and a plane on the outer equator.
- **Intractable pairs** (R-reach):
  - torus × {oblique cylinder, non-coaxial cone, non-coaxial torus,
    NURBS};
  - sphere × NURBS;
  - cone × {cylinder, cone} in general pose (§2.7);
  - cone × torus, not coaxial.

  Two measured fixtures fall here. **The backstop's tilted rod** is
  oblique cylinder × torus, so the guard is its barrier (the
  measurement lane's finding). **The cone preview fixture** is cone ×
  a partial cylinder about a perpendicular axis.
- **Pole or apex faces where only W2 could clear** (§3 step 5). No known
  tractable instance exists.
- **Cone pairs,** at the operand gate, until `VERBS-CONE`. The §2.5–2.7
  arms are derived here, and whether they land now is §8 Q3.

**Changes that are not give-backs:**

- **The corner bar** (`the_corner_bar_never_comes_back_a_body`) is
  CLEARED by the certificate:
  - its end squares' peanut is one component, and the corners' contacts
    make W4 hold;
  - its `x` faces cut `(0,1)` pairs and its `y` faces cut `(1,0)` pairs,
    so W2 clears both.

  Its lens arcs are corner-to-corner arcs bounded by events: evidenced,
  and the chord rule's business. The row keeps asserting "never a body",
  and what enforces it moves downstream, to the chord rule and the
  sagitta charge. The row's doc must say so.
- **The half-donut bracket and the dome bracket** change payload (R-loop)
  if Q1 adds the variant. Their rows' helper
  (`refuses_as_the_interior_loop_guard`) moves with it.

## 7. Rows the implementer writes: red first, against a named mutant

There are two harnesses:

- **verdict-level rows** in `crates/topo/src/boolean/section_cert_rows.rs`
  (the `torus_predicate_rows.rs` shape), which call the classifier and
  the per-pair rule on constructed bodies and a real reduction;
- **op-level rows** in `crates/sweep/tests/germ_interior_oval.rs`, where
  an op reaches a body.

Each row is seen RED against its mutant before the code it guards lands,
and the PR body names the run.

**One mutant per certificate:**

| certificate | mutant | row that goes red |
|---|---|---|
| W2 | W2 disabled | the plane cutting a tube in two ovals (a bar threading the hole, crossings at its ends): its `x` faces' `(0,1)` pairs refuse. Op-level if ∪ answers, verdict-level otherwise |
| W2 | W2 without the `chart_boundary` check | a seamless cylinder band built through the Euler test support (`cylinder_extent_gate`: nothing in the types forbids one), cut by a plane in an encircling ellipse inside both faces, must refuse. If no seamless band can be built, say so in the PR and pin the check with a verdict-level row on a face `chart_boundary` refuses |
| W3 | accepts `In` | the half-donut bracket answers: red on every op |
| W3 | consults `F` only | the pin-only bracket: the witness sits on `F`'s cap boundary and is `Out` only of `G`, so it refuses: red |
| W4 | reads the op's events, not the pair's | the dome bracket: the side-cap pair has none while the op has some, so it answers: red |
| W4 | drops the count-1 check | a verdict-level row feeding the rule a 2-component classification with an event and no other witness must refuse. No tractable op reaches it (§3 step 5), so the row is on the rule, stated as such |
| §2.1 sign | swaps `σ` | the pin-only bracket: the witness lands on `T₋`, which the plane `x = 1.95` misses, so it refuses: red. Plus the half-donut foot at verdict level: a `u' = π` scrape |
| tangency | `Zero ⇒ Positive` | verdict level: a plane `x = R + r` on the outer equator, and a plane `y = r` on the top circle, both refuse R-tan |
| per pair | reach per op kept | the pin-only bracket refuses: red |
| no-crossings path | `torus_extent_gate` kept | the donut-hole cube's union refuses: red. Assert `vol(donut) + vol(cube)` |
| no-event decision | `In`-both answered as clear | the half-donut bracket answers, and so does the cylinder P0 fixture |
| §2.9 | the middle row read as the first (null loop taken as thin-essential) | the cylinder P0 fixture answers: its four wrong bodies come back, red on every op |
| §2.9 | the `|d₁ × d₂|` decision dropped | verdict level: two walls `1e-9` rad off parallel, offset so that a long saddle loop exists, refuse R-tan |
| no-crossings cylinder | `cylinder_extent_gate` kept | a rod wholly through a fatter wall, no other crossing (two thin-encircling loops): its union refuses: red |
| no-crossings cone arm | the arm omitted, so the fallback reaches the vertex probe | the cone preview fixture (verdict level until admitted; op level on the admitting PR) comes back `Assembly`: red |

**One adversarial fixture per component class** (verdict level unless
marked):

- **torus × plane:**
  - a scrape with no event, inside both (half-donut foot → R-loop, op
    level);
  - a scrape with no event, outside `G` (pin-only → W3, op level);
  - a scrape with an event (corner-bar end square → W4);
  - two `(1,0)` (pin `y` faces → W2);
  - two `(0,1)` (a plane cutting a tube in two ovals → W2);
  - the tangent pair → R-tan;
  - a near-axial scrape (`s ≈ 1e-9`) → R-tan or a decided verdict, never
    a wrong one.
- **torus × sphere:**
  - a ball scraping the outer wall inside both → R-loop;
  - a ball on the axis cutting the tube's top → W2;
  - a ball swallowing an arc of the donut → W2 on two `(0,1)`;
  - a ball tangent to the tube → R-tan.
- **torus × cylinder, parallel axis:**
  - a pin through the tube → W2 on the cylinder, op level if ∪ answers;
  - a pin straddling the outer equator inside both → R-loop;
  - a fat offset cylinder crossing the tube twice → W2 on `(0,1)`.
- **torus coaxial:** a cylinder, a cone and a second torus → W2 on
  parallels.
- **torus oblique:** a tilted rod through the half donut → R-reach.
  - This is the fixture of
    `union-backstop-catches-a-suspect-body-from-a-tilted-rod-in-a-half-donut`;
    cite that item.
- **sphere:**
  - sphere × sphere with no event, circle outside a face → W3;
  - sphere × cylinder with no event, one loop inside both → R-loop;
  - the same, one loop outside a face → W3;
  - two encircling loops → W2;
  - the dome bracket → R-loop, op level;
  - the crown alone keeps its closed form (existing row), now via W2 or
    W4.
- **cone** (verdict level: classifier and rule, since no op reaches a
  cone face):
  - a plane cutting a seamed frustum in an ellipse inside the plane face
    → W2, the moved "interior ellipse";
  - a hyperbola → W1;
  - a pointed cone's ellipse with an event → W4;
  - a plane through the apex → W1 (rays), with no refusal (§2.5);
  - a sphere with the apex inside → W2 per nappe;
  - a lone null loop on a nappe → W3 or R-loop;
  - a parallel-axis cylinder → W2 on the cylinder.
- **cylinder** (op level where noted):
  - the P0 fixture: R-loop on all four ops, op level. Replay the item's
    fixture verbatim, and assert that the payload names A's wall and B's
    arc face.
  - its tilted `0.15` variant → R-loop, op level;
  - its pin-less control → the no-crossings pass's R-loop
    (`FallbackExtentUnsupported`), op level;
  - a thin rod wholly through a fatter wall, with a pin elsewhere
    (two loops, each encircling the rod) → W2 on the rod's wall. Op
    level if ∪ answers; it pins P-trace (Q7).
  - the saddle loop with an event (the arc prism's line edge moved into
    A's wall) → W4;
  - the saddle loop with no event, its witness outside the partial face
    → W3;
  - parallel walls (lines) → W1;
  - a wall × plane ellipse inside the plane face → W2;
  - the tangent pose `|δ₀| = r₁ + r₂` → R-tan.
- **no-crossings, cone** (verdict level until `VERBS-CONE` admits the
  kind; then op level on the admitting PR):
  - the measurement lane's cone × partial-cylinder preview, with and
    without its pin → R-reach (`FallbackExtentUnsupported` without the
    pin, `CurvedPairUnsupported` with it);
  - a cone and a ball clear of it → W0.
- **The known names:**
  - the half-donut bracket → R-loop;
  - the corner bar → cleared at the guard, still never a body;
  - the pin-only bracket → answers, with a closed-form volume;
  - the dome bracket → R-loop;
  - a plane cutting a tube in two ovals → W2;
  - a plane cutting a cone in an "interior" ellipse → W2 or W4, never a
    refusal on reach.

**Sweep owed (discipline §5):** list every reader of the stopgap's
refusal, of the concurrent cylinder stopgap's, and of
`torus_extent_gate`'s and `cylinder_extent_gate`'s, with a disposition
for each. The reason is the class DR-14 logged: every reader of a roster
refusal is a premise. At minimum:

- `CurvedPairUnsupported`'s variant doc, which names the stale
  `ops::interior_loop_reach_gate`;
- `cylinder_extent_gate`'s torus bullet;
- the `germ_interior_oval.rs` module doc;
- the roster row
  `subtract_and_intersect_refuse_an_oval_their_crossings_cannot_see`;
- `VERBS-CONE.md`'s "what a cone admission must carry";
- the P0 cylinder item's rows.

## 8. Open questions (⚑ = design fork, weighed before it is decided)

- **Q1 ⚑ The R-loop payload.** Should R-loop get a new `BooleanError`
  variant, naming both faces and the witness point? The alternative
  keeps `CurvedPairUnsupported`, which today conflates "no certificate"
  with "a certified interior loop" (the "one `Out` for two causes" class
  of the 2026-09-26 log).
  - A variant is public API: `BooleanErrorKind` has a mirror arm, and
    `pncad-py` exposes the kind.
  - It is not ratified text. It is a posture choice for the
    orchestrator, or for Ev if it reads as design.
- **Q2 ⚑ Cut in, or refuse.** R-loop could instead insert the loop as a
  ring edge in both faces (`euler_ring`), and the join would then see
  it. That is option (b)'s second half, it is H, and it is a separate
  unit. Until it exists, R-loop is the honest answer.
- **Q3 The cone arms, on both paths.** They could land now, as pure
  classifier arms with verdict-level rows that no op can reach until
  `VERBS-CONE`, or they could land with `VERBS-CONE`.
  - The measurement lane's preview shows that the admission is unsafe
    without the crossings-path half AND the no-crossings arm.
  - Recommendation: land both here. The admission is then a roster flip
    whose guard is already pinned, and `VERBS-CONE`'s item gains a line
    saying so.
- **Q4 ⚑ The seam premise as a validated invariant.** "Every periodic
  face lifts to one chart sheet" is a constructor convention
  (`pcurves.rs` `LoopWraps`: "no constructor in the tree is known to
  build one"). W2 checks it per face, so this certificate does not need
  it. But the stopgap's sphere × cylinder clearing and
  `cylinder_extent_gate`'s plane exemption assume it unconditionally.
  - Making it a tier-1 check would put it in the validation contract,
    which is ratified ground (DESIGN's topology invariants). So it is
    Ev's call.
  - The alternative is to re-point those two readers at the same
    per-face check.
- **Q5 Cone × cylinder and cone × cone, general pose.** They are
  R-reach here. The ruling reduction (§2.7) makes them tractable:
  - the discriminant is a degree-2 trigonometric polynomial with ≤ 4
    roots;
  - the in-tree Ferrari quartic (`line_torus_roots`) can isolate them.

  That is a follow-up M, and it is what would answer the cone preview
  fixture's evented cousins. Cylinder × cylinder needed no quartic: the
  common-perpendicular reduction (§2.9) is linear in `cos θ`, and it
  lands here.
- **Q6 Retiring `torus_extent_gate` and `cylinder_extent_gate`** in
  favour of the per-pair pass (§5). This is recommended; it is
  code-comment ground, not ratified text. The two gates' docs carry the
  arguments this spec formalises (the plane exemption, the seamless-band
  premise, the donut-hole cube), so fold them into `section_cert.rs`'s
  docs rather than dropping them.
- **Q7 Premise P-trace.** "One event on a closed component lets the
  join trace the whole component" is measured only for the sphere
  crown, a seam crossing (`the_crown_alone_still_answers_its_closed_form`).
  The torus's essential components cross a seam or a band edge the same
  way. The W2 op-level rows are the measurement, and if one of them
  comes back wrong, that is a crossing-layer finding to file, not a
  certificate fault.
- **Q8 A radial hole through a tube** (§2.4, perpendicular axes). It is
  tractable through degree-8 root isolation, and it is a common part
  feature. Is it worth a follow-up M?
- **Q9 Dimensionless margins.** Two margins are angles, not lengths:
  §2.5's `|n·a| − sin α` and §2.9's `|d₁ × d₂|`. `decide` meters in
  metres, so each needs a lever. The natural one is the pair's extent,
  the diagonal of the box overlap: an angle `ε` there moves the section
  by `ε·extent`.
  - `curvature_lever_arm`'s history (the 2026-09-26 "one function for
    two quantities" class) says to give each its own named predicate and
    lever, not a shared helper.
  - The implementer picks the lever and states it in the predicate's
    doc. It is not a design fork.
