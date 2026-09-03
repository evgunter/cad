# PATHS-DESIGN: the PartialPath authoring algebra (S5)

Status: **RATIFIED** (design-conversation PR #124, signed off and
merged 2026-07-29; the ratified doc was the deliverable of that
conversation). **IMPLEMENTED** as LIB unit U2 (`crates/profile/`);
`docs/LIB-U2-SPEC.md` and `work/lib/log.md` are the implementation
record. Designed across twelve review rounds with Ev
(2026-07-27/29, #104 + the #124 threads); the round-by-round trail
lives in #124 and the M5 log — this document states only the
resulting design.

Harmonization constraint (ratified context): #101's
declared-tangency discipline (flags verified-never-trusted,
`UndeclaredTangency`/`TangencyContradicted`, fillet fit gating,
same-carrier-is-identity) landed at #109/#112 and is the layer this
algebra lowers to. **End state (the #104 recorded v2 commitment,
affirmed here per Ev's round-13 note): the algebra IS the core
representation of paths** — the program is the profile's definition
and derived segments are caches/provenance, exactly as Q8
definitional surfaces work (the constructing function is the
surface). That representation switch is designed in
`docs/PROFILES-V2-DESIGN.md` and shipped in the LIB SWITCH units;
this document specifies the algebra itself, which is what both the
pre-switch generator surface and the stored profile-program speak.

## 1. What this is

A typed authoring algebra for profile loops in which **accidental
tangency is unrepresentable, intended tangency is exact by
construction, and every authored point lies on the final path,
authored once**. It is a generator-layer surface (D8); it lowers to
explicit segments + declared tangency flags, verified at build by
the same junction predicates. No kernel semantics change; the
document layer's own change — the stored program — is
PROFILES-V2's.

## 2. The core

### The binding lattice

The tip's typestate is exactly which of {position, angle} it has
bound:

- **`Open`** = {} — the entry point, and every fillet's freshly
  opened arrival side.
- **`Point`** = {position}. Two flavors, distinguished by type: a
  **plain point** (`Open.at(p)`, `.at(p)` on an arrival — position
  only, no incoming carrier) and a **directed point** (a leg end —
  position plus the leg's incoming end tangent, carried as
  read-only intrinsic data). A Point's only legal continuation is
  binding its outgoing angle (a director, or sugar that computes
  one).
- **`Angle`** = {angle} — direction bound, position pending (a
  fillet arrival bound angle-first). Its only continuation is
  binding the position.
- **`Directed`** = {both} — the only state legs and `.fillet(r)`
  consume.

**Incoming and outgoing directions are different kinds.** The
OUTGOING angle is a binding slot, set at most once per side: every
director requires the slot empty, so a second
`.angle`/`.tangent`/`.turn` on a Directed tip is ill-typed. The
INCOMING direction is never a slot: it is intrinsic data on a leg
end, consultable by `.tangent()`/`.turn(δ)` and the junction check,
settable by nothing.

**The directed point is its binding bits and NOTHING else (the §2c
axiom):** a directed point is (position, tangent) — what leg
produced it is unrepresentable knowledge, so no verb can branch on
it, refuse on it, or silently depend on it. The verbs are pure
functions over these bare state values in a sealed kernel module;
the chain threads values and applies their emissions. (The
junction check's lever arm and the §4 item 4 identity data are the
CHAIN's own emission-layer bookkeeping, not anything a verb can
consult.)

### Binders and directors

- **`.angle(θ)`** — adds the angle bit wherever it is missing
  (`Point → Directed`, `Open → Angle`). On a directed point it
  runs the junction check of θ against the incoming tangent (§4
  item 1); on a plain point there is nothing to check (an arrival
  side meets its fillet arc tangentially by construction; the
  entry point's junction check happens at the seam).
- **`.tangent()`** — consumes a **directed point only**: re-uses
  the incoming end tangent as the departure and emits the DECLARED
  flag on lowering. Ill-typed on plain points (no direction to
  inherit) — which is what makes "fillets sit between defined
  geometry" structural rather than a rule.
- **`.at(p)`** — adds the position bit (`Open → Point`,
  `Angle → Directed`); on an arrival side, `p` is the side's
  anchor, a real on-path point.
- **`.to(dp)`** — the combined binder consuming a directed-point
  VALUE: `Open → Directed` in one step. `Start` is its canonical
  argument; a curve value's `c.start()`/`c.end()` are the others.

### Legs

Direction-consuming only, from `Directed`; a leg terminates at a
bound position → `Point` (directed flavor). No leg departs a
half-bound tip.

- **`line(len)`.**
- **Arc legs** — the SHARP arc leg is `arc_to(spec)`, one verb over
  the `ArcData` spec family (§2c rounds 5–9), with admissibility
  the state-keyed trait matrix: from a Point tip the endpoint-full
  modes (`Bulge{p, b}`, `Via{q, p}`, `Center{c, winding, p}` — the
  retired three-verb register's bindings, one spelling); from a
  Directed tip the endpoint-free pair (`Sweep{r, side, angle}`,
  `ArcLen{r, side, len}` — the arc analogs of `line(len)`:
  tangent-departing, endpoint DERIVED). `tangent_arc_to(p)` stays
  the tangent-departing endpoint-full form; {tangent-both + r} =
  the fillet family (§ below), which alone carries the
  neighbor-trimming insertion.
- **NURBS legs** — specified here, not built: `ProfileLoop` has no
  NURBS segment to lower to, so they wait on the segment vocabulary
  (PROFILES-V2 VQ7). Rigid authored data (clamped, w > 0, the PR 3
  invariants); end positions and end tangents are intrinsic, so a
  NURBS leg's end is a directed point and `.tangent()` chains
  onward. Two doors:
  - **`nurbs_in_place(len1, ctrl, weights, knots)`** on a
    `Directed` tip — inline authoring: `P0` is the tip position
    and `P1` = tip + len1·departure, both IMPLIED (never
    authored); the remaining control points are authored
    **absolute**. Only P0/P1 carry junction constraints, so
    implying them makes the value-matching violations
    unrepresentable while every other control point is free
    world-frame shape data.
  - **`nurbs(curve)`** — rigid placement of a pre-authored curve
    VALUE: translation takes the curve's start to the tip,
    rotation takes its start tangent onto the bound departure
    (2 + 1 placement DOFs = exactly what `Directed` supplies,
    whichever director bound it). No scale, no deformation — the
    algebra places authored curves, never edits them.
    `nurbs_reversed(curve)` / `nurbs_mirrored(curve)` are the
    structural variants (parameterization flip; reflection across
    the departure line — curvature signs flip).
  - Pose-preserving use of a curve value joins through its own
    directed-point values: `.to(c.start())` binds the tip to the
    curve's OWN start pose (position + start tangent, read from
    the curve data by reference); the subsequent `nurbs(curve)`
    placement transform is then the IDENTITY BY CONSTRUCTION —
    the tip's bits ARE the curve's start bits, so the curve lands
    exactly at its authored absolute pose, with nothing re-typed
    and nothing to value-match.
- **Authoring frames, uniformly**: every authored point — anchors,
  targets, control points — is ABSOLUTE (profile frame); the only
  implied points are those the junction already owns (P0/P1 above,
  fillet corners, trim points). Lengths and `turn` angles are the
  relative quantities.
- **Fillet carriers are line/arc**: a corner fillet tangent to a
  NURBS carrier has no closed form (an iterative solve — and this
  algebra is solver-free), so `nurbs_fillet` is an ABSENT VERB —
  unrepresentable, not refused (§2c round 10). Bare `fillet(r)`
  after a NURBS leg is the uniform ray extension: the surviving
  ray piece is a genuine line leg off the curve's end.
- **NURBS legs CAN close** (round 13, Ev's observation that
  trailing DOFs can be left off exactly as leading ones are): the
  FULLY-authored form cannot target `Start` (placement consumed
  at departure, the end lands where the data says), but the
  closing variants imply the trailing control points the seam
  owns, mirroring `nurbs_in_place`'s start side: the sharp-seam
  form implies `Pn := Start.pos` (interior points authored,
  seam junction checked as usual); the tangent-seam form
  additionally implies `Pn−1 := Start.pos − len_end·Start.dir`
  (with `len_end` authored, the mirror of `len1`). Both-ends
  forms compose (in-place start + Start-targeting end). Same
  principle throughout: junction-owned control points are
  implied, never authored.

### Fillet (the §2c fused family)

Four verbs, each naming its incoming and arrival carrier kinds
with LINE the unmarked default (§2c): **`fillet(r)`** (line
incoming, line arrival), **`fillet_arc(r, spec)`** (line incoming,
arc arrival), **`arc_fillet(spec, r)`** (fused arc incoming, line
arrival), **`arc_fillet_arc(spec, r, spec₂)`** (both). A fillet
that needs an arc carrier cannot LEARN it under the axiom, so it
AUTHORS it — the arc and the fillet that trims it are one
authoring act.

- **Incoming**: `fillet`/`fillet_arc` consume a directed point —
  a bound Directed tip, or a bare leg end, where the incoming
  side is the TANGENT RAY extended as a REAL line leg (ray
  extension, uniform across line/arc/NURBS incomings; after a
  straight leg the extension EXTENDS that leg's own segment, the
  §4 item 4 exemption applied at emission). The fused verbs'
  incoming spec is state-keyed exactly as `arc_to(spec)`
  (endpoint-full from a Point, endpoint-free from a Directed tip)
  plus the two carrier-continuation rows: `Center{c, winding, p}`
  from the ENTRY (the entry bound ON a carrier — the retired
  `at_on` entry, fused) and `Radius{r, side}` from any DIRECTED
  POINT — **arc extension**, the arc analog of ray extension
  (§2c dissolution amendment below): the centre is DERIVED from
  the tip's binding bits, so tangency holds by construction, and
  the incoming run extends FORWARD from the tip along the derived
  carrier. When that carrier continues the incoming segment's own
  carrier, the extension MOVES that segment's end vertex to the
  trim point (the §4 item 4 exemption, exactly as a straight
  leg's ray extension); otherwise the joint at the tip is a
  constructed tangency onto a new carrier — sound for every
  authored `r`. A trim that would eat the tip's authored anchor
  refuses (`AnchorOutsideTrimmedExtent`). `Center` from a
  directed tip stays EXCLUDED — the tip's direction is bound, so
  an authored centre's derived tangent would have to value-match
  it, and no direction remains for the centre to supply
  retroactively (authored-once decides).
- **Line arrivals** keep the uniform builder: `.at(p)` /
  `.angle(θ)` / `.toward(dx, dy)` in either order, the far-end
  `.to(p)`, and the seam `.to(Start)` (straight first side only —
  the seam retrims the entry vertex).
- **Arc arrivals** are the spec's own completion story:
  `Center{c, winding, p}` is complete at the verb (interior `p`:
  the run to `p` is EMITTED at the verb and the tip is an
  ORDINARY DIRECTED POINT at `p` — a hard anchor, uniform with
  line arrivals; the §2c dissolution amendment below retired the
  OnArc state; `p: Start`: the close that KEEPS the entry vertex
  — the retired `to_on`); `Radius{r, side}` derives its centre
  from the arrival's directed anchor, so both binders stay free;
  `Via{q, p}` carries its anchor and awaits one director.
  `Bulge` is never an arrival (no chord exists there).
- Once both carriers are fixed, the r-arc tangent to both is
  inserted at their implicit virtual corner, trimming both — the
  resolution machinery of the retired register, unchanged bit for
  bit.

- **The corner is never authored** — it exists only as the carrier
  intersection; a corner takes `r` (tangent-to-two-carriers is a
  one-parameter family). Authoring a point and filleting it away
  is unrepresentable: fillet takes no corner point, and a bare
  Point cannot fillet.
- **Every side is anchored** by a real on-path point plus a
  direction (the incoming ray by the current end; the outgoing
  side by its anchor/arrival data). The anchor-free
  both-ends-trimmed side — underdetermined (offset free) — cannot
  be written.
- DOF check: arc 5; r binds 1; tangency to each fixed carrier
  binds 2 + 2 — exactly determined (#101's `LoopBuilder::fillet`
  closed form at a virtual corner). Parallel/non-intersecting
  carriers, or an intersection behind the ray start, refuse typed
  `NoCornerForFillet`.

### Closure: the `Start` token

`Start` is a first-class **directed point** value — the bound
entry (always both bits by the time the loop returns). It is legal
wherever a directed-point/position argument goes, and **using it
is closing — structurally**: the endpoint IS the start point by
reference, authored once; closure never depends on re-typed
coordinates value-matching.

- Sharp seam: `line_to(Start)` / `arc_to(Bulge { p: Start, b })` —
  an ordinary leg targeting Start; the seam's junction check runs
  with both directions known, and an UNDECLARED tangent seam
  refuses (`SeamTangent`) from every closing verb.
- Declared seam (§6's revised PQ4): the seam's own junction is the
  one declaration that cannot ride a departing leg, because the
  arriving leg is authored last. It rides the TARGET —
  `Start.arrives_tangent()`, the ONE arrival declaration — and
  **every closing verb takes it**: `line_to`, `continue_to`,
  `tangent_arc_to`, `arc_to(Bulge { … })`. Every zero-turn joint is
  a declared tangent joint (Ev, in-chat, 2026-09-02), so there is
  nothing else to declare and no sibling token. The kernel CHECKS
  the arriving direction against `Start`'s own direction and
  NOTHING else — never whether the two carriers are the same.
- Tangent seam: **declared**, per the bullet above. The 2026-07-28
  text here read "`.tangent().tangent_arc_to(Start)`", and that
  spelling refuses `SeamTangent` — its DEPARTURE tangency is
  declared and its ARRIVAL was not, which is the half no departing
  leg could ever carry. The stadium is exactly that shape, and it
  closes as `.tangent().tangent_arc_to(Start.arrives_tangent())`.
  A straight closing leg says the same thing the same way:
  `line_to(Start.arrives_tangent())` — what the token classifies is
  the JOINT, not the shape of the leg reaching it.
  A tangent LINE close remains impossible, and that part of the
  2026-07-28 text stands: a line's direction cannot be both
  INHERITED from the tip and aimed at an independently authored
  point, so there is no such verb to reach — `.tangent()` leaves the
  tip Directed, where the only straight leg is `line(len)`, which
  has no target. Accepting it would mean reading "the author meant
  this ray to hit Start" off a ray that happens to hit it, which is
  the value inference the ladder refuses.
- Straight-run seam: what an author reaching for a tangent line
  close actually wants — a straight leg CONTINUING its run onto the
  entry — is `continue_to(Start)` (§4, BOOL-11), and
  it departs the very state `.tangent()` does: the two are the
  alternatives at one tip. The verb DECLARES the continuation, and
  the joint it mints is a declared TANGENT joint like any other
  zero-turn joint; nothing is inferred, and the kernel then CHECKS
  that the target lies on the departing ray to within ε_input —
  authored-data consistency, the arc verbs' class, refusing
  `ContinuationTargetOffRay` past the band. Where the seam's own
  joint is zero-turn too, the arrival token says so on the same
  verb:

  ```text
  Open.at((0, 0)).angle(north)
      .line(2.0)
      .arc_to(Bulge { p: (0, -2), b: 1.0 })
      .line_to((0, -1))
      .continue_to(Start.arrives_tangent())
  ```

  — the D-shape, closing on its own straight side, pinned by
  `the_d_shape_closes_with_the_declared_straight_arrival` in
  `profile`'s `bool12_probes`. Its `tangent_joints` is `[0, 3]`: the
  seam's joint and the interior continuation's, both declared.
- Seam fillet: `.angle(θ).fillet(r).to(Start)` — both carriers
  bound, nothing pending, loop closed.

There is deliberately no `close()` alias — Start-targeting through
ordinary verbs is the one mechanism.

**Entry rule**: the entry authors the FIRST SIDE (either binder
order — `Open.at(p).angle(θ)` or `Open.angle(θ).at(p)`); the SEAM
is authored once, at the back, by the verb that targets `Start`. A
leading `.fillet`/`.tangent` would be the seam's content authored
from the front — a second spelling of the same value, and the
front cannot elaborate it (neither adjacent carrier bound there);
both are ill-typed, uniformly, because they need bits the entry
Open lacks. In a fully-filleted loop no side is privileged: the
loop is a cyclic sequence of [corner, side-bindings] units and the
seam fillet reads exactly like the interior ones.

**Composition**: there is no path-concatenation operator. Repeated
motifs are BUILDER FUNCTIONS over the one chain
(`fn motif(p: PartialPath<Directed>) -> PartialPath<Directed>`) —
associative at the language level, no second path value, no glue
seam to value-match.

## 2a. G1 vocabulary growth, 2026-08-08 — ratified via PROFILES-V2-DESIGN VQ1(b)

PROFILES-V2-DESIGN §V7's VQ1 ruling is **(b)-direct**: the algebra
grows until the persisted corpus authors fully, BEFORE the schema
switch. This section is that growth's cheap set — five constructors,
all closed-form, all lowering to the same v1 form §2 lowers to. The
evidence is LIB-U2 PR-2's corpus-scale wall list (LIB-LOG accumulator):
W1 directors-as-angles are ulp-dirty, W2 missing arc binding modes,
W5 no far-end-anchor spelling. W4 (arc-carrier fillets — the rocker's
five) is explicitly NOT here; it is G2.

Nothing in §2 is revised. The lattice, the entry rule, the seam rule,
the fillet's DOF count and PQ4 all stand exactly as ratified; these are
additions to the surface, and each one states below what it consumes,
what it determines, and what it refuses.

### The two exactness contracts

Every constructor here obeys both, and they are what make the growth
safe to migrate a byte-identical corpus onto:

1. **Authored points are stored verbatim.** A point the author types is
   emitted as itself. Every derived quantity — bulges, rays, corners,
   trim points — is computed at LOWERING, from those authored points.
   Nothing computed is ever re-typed by the author, which is why the
   algebra and a hand-built chain given the same authored points agree
   bit for bit, not merely to tolerance.
2. **Direction-exact rays.** A director may fix an ANGLE (and derive
   its ray) or fix a RAY directly. The second spelling exists because
   the first cannot be exact: `sin_cos` quantizes, so `.angle(PI)`
   yields `(-1, 1.2246e-16)` and carries that ulp into every corner
   downstream. Where a ray is what the author means, the ray is what
   gets stored.

### 1. `circle(center, r)` — a complete-loop program form

**Consumes** a centre and a radius. **Determines** the whole loop: it
IS the profile loop, not a chain step, so there is no tip to continue
from and no verb that follows it.

It authors **no seam**. That is the load-bearing property. PQ4 (§6: a
closed loop's seam sits at a junction or fillet; closing mid-side is
refused) is a rule about CHAINS, and it is untouched — a chain still
cannot close mid-carrier. The conventional split into two semicircles
at the ±x poles is the primitive's PRIVATE lowering, exactly the M2
closed-carrier precedent: a detail of how a closed carrier reaches a
vertex+bulge document, not a junction anyone said. The two joints are
same-carrier identities, so nothing is declared tangent — there is no
tangency to declare, it is one circle.

§6's PQ4 entry records that same M2 precedent as "considered and
declined", so the citation deserves one clause: what was declined is
the CHAIN relaxation — letting an authored loop close mid-carrier,
which would have touched the one-authored-side-one-carrier discipline
germ matching and the merge ladders lean on. Nothing here reopens it.
This primitive is not a chain and authors no seam, so the chain rule
stands untouched and a chain closing on its own carrier still refuses
(pinned by test).

`circle` itself offers no control over the split. A loop whose
downstream naming depends on the seam structure (the tour's boss wants
three 120° arcs so a boolean can cross a three-face rim seam) authors
it with `circle_split(centre, r, n, phase)` instead — the
declared-subdivision closed carrier: `n` arcs of equal sweep, first
vertex at `phase` from +x, `n ≥ 2` or `CircleSplitCount`. Its vertices
are STRUCTURAL subdivisions of one carrier — same-carrier identities,
nothing declared tangent — so it too authors no seam and PQ4 stays
untouched; the count and phase are simply authored data rather than a
private lowering detail.

**Refusals**: `r` not definitely positive (`NonpositiveCircleRadius`),
through the same funnel as the other sign gates.

**Composition**: a circle is one loop among others — profiles mix
circle loops and chain loops freely. §6's mixed-authoring rule is read
at LOOP granularity, as it always was: no loop is half raw.

### 2. The arc through a point — the `Via { q, p }` mode of
### `arc_to(spec)` (§2c's unified family; the standalone `arc_via`
### name is gone)

**Consumes** a positioned tip, a through-point, and an endpoint.
**Determines** the arc through those three points. A free arc: the
junction semantics are `arc_to`'s exactly — on a directed point the §4
item 1 check runs against the arc's start tangent;
`arc_to(Via { q, p: Start })` is a sharp arc seam. It is a LEG, not an arrival: an arc arrival binds
its carrier through the fused verbs' arrival specs (§2c), never as an
arc leg from an already-bound arrival point.

The through-point is authored but is NOT a chain vertex — it is the
bulge's input, and the bulge is derived at lowering by the existing
inscribed-angle closed form. This is the first constructor where an
authored point is not itself on the emitted chain, and the §4 item 3
invariant survives intact: the through-point lies on the final PATH
(it is on the arc), just not at a vertex.

**Refusals**: a through-point within ε_input of the chord LINE
(`ArcViaCollinear`) — one refusal for the whole collinear class,
on-chord, beyond-the-end and on-endpoint alike, because all three make
the same statement (three collinear points name no arc) and the
recourse is the same (move it off the chord, or author a line);
coincident endpoints (`DegenerateArcChord`).

### 3. The arc about a centre — the `Center { c, winding, p }` mode
### of `arc_to(spec)` (§2c's unified family; the standalone
### `arc_center` name is gone)

**Consumes** a positioned tip, a centre, an endpoint, and a winding.
**Determines** the arc from tip to end about that centre, with the
winding selecting which of the two.

The winding is **structural** (`Ccw` | `Cw`), not a number whose sign
the author has to get right. The choice is discrete, so it is spelled
discretely — the same reasoning that makes `.tangent()` a verb rather
than an angle that happens to match.

This is the centre-INTENT spelling: a lantern's belly is *the sphere's
own arc about the globe centre*, and authoring it this way says so,
rather than fitting an arc and hoping its carrier lands on the sphere.

**Equidistance is CHECKED, never repaired.** |tip − centre| and
|end − centre| go through the funnel; a definite mismatch refuses
`ArcCenterNotEquidistant`, naming both radii. Silently re-projecting
the centre onto the endpoints' bisector — or an endpoint onto the
circle — would MOVE AN AUTHORED POINT, which §4 item 3 forbids
outright. Three points that contradict each other are a bug in the
authoring, and the algebra's job is to say which two disagree, not to
pick a winner. An undecidable margin escalates; it is not rounded into
agreement.

**Refusals**: the equidistance mismatch above; a centre within ε_input
of an endpoint (`DegenerateArcCenter` — no radius, so the winding
selects nothing); coincident endpoints (`DegenerateArcChord` — a full
turn is a closed carrier, which is `circle`'s business, not a leg's).

### 4. `to(anchor)` on a bound arrival direction — the far-end anchor

**Consumes** a fillet arrival whose direction is bound, plus a point.
**Determines** the arrival side's position bit AND the side's end: the
side runs from its trim point and STOPS at `anchor`. Lattice-wise
`Angle → Point` (directed flavor), which is why it belongs to the `to`
family: like `.to(dp)`, it is the one-step form of a binding that would
otherwise take two.

W5 was a gap in EXPRESSION, not in geometry. §2 already says every side
is anchored by a real on-path point plus a direction, and
`.angle(θ).at(p)` binds exactly that pair — the arrival carrier is the
line through `p` in direction θ, the corner is still the carrier
intersection, never authored. What was missing was only the ability for
the side to STOP at its anchor: `.at(p)` leaves the tip Directed at
`p`, and every continuation runs PAST it, so a side whose natural end
is its far vertex had to be authored as a synthetic mid-side anchor
plus a length — a point that is not a vertex, and a number nobody
measured. `.angle(θ).to(p)` determines exactly what `.angle(θ).at(p)`
determines; it adds no geometry, no new DOF, and no new corner rule.

Consequently the fillet resolution, its corner gates, and the anchor
fit checks are `.at(p)`'s, unchanged, and `p` is on the final path
either way, authored once. The result is a directed point (incoming
tangent θ), so the next verb's junction check runs as after any leg.
An exact trim fit — the arc reaching `anchor` with no straight run left
— emits no degenerate segment: the side simply IS the arc, mirroring
how the incoming side's exact fit is already handled.

The direction must be bound FIRST: with the anchor as the terminus, the
side's carrier is what the director supplies.

**Refusals**: reached at the ENTRY, where the direction is bound but no
side is waiting to be ended, it refuses `FarEndAnchorWithoutFillet` —
the entry authors its first side with `.at(p)`, and the seam is
authored at the back (§2's entry rule).

**Open, deliberately**: a `Start`-targeting far-end form
(`.fillet(r).angle(θ).to(Start)` — an arrival side ending at the entry
vertex) is well-defined by the same reading and is NOT in this surface.
It is a second closing spelling, and the corpus has no case for it; if
one appears, it is an addition here, not an improvisation at the call
site.

### 5. `toward(dx, dy)` — the exact director

**Consumes** the same angular DOF as `.angle(θ)` — the SAME lattice
slot, set at most once per side, with the same §4 item 1 junction check
on a directed point and the same fillet resolution on a bound arrival.
**Determines** the departure ray directly.

`(dx, dy)` is normalized and the unit ray stored VERBATIM, with no trig
round-trip. Axis-aligned and Pythagorean directions are therefore
exact: `.toward(-1, 0)` gives `(-1, 0)`, where `.angle(PI)` gives
`(-1, 1.2246e-16)`. Only the components' RATIO is read — the magnitude
is not a length and binds nothing — so the author never has to
normalize by hand.

This kills the W1 drift class at its source. The corpus's one line×line
fillet (the bracket, the #101 showcase) could not move to the algebra
under LIB-U2 PR-2 because its corner is reached through a director, and
an angle-spelled axis ray put both lowered trim vertices 1 ulp off the
hand chain. With `.toward` the corner comes out exactly, and the
bracket lowers bit-identically.

**Refusals**: `(0, 0)`, and any norm within ε_input of zero, refuse
`ZeroDirection`. The sub-ε case is refused rather than normalized
because normalizing such a vector amplifies its own noise into the ray
— and since only the ratio matters, the recourse costs nothing: scale
the components up.

**Representation note**: the angle slot's PAYLOAD widens to
angle-or-direction (both are carried: the ray for every ray
construction, the angle for the arithmetic `.turn(δ)` and arc end
tangents genuinely need). §5's shape is unchanged — one struct, two
optional bits, fields private, binders the only constructors.

## 2b. G2: arc-carrier fillets (HISTORICAL — the compound register,
## superseded by §2c)

This section held the compound-verb register that first opened
arc-carrier fillets: `at_on(p, centre, winding)` (the carrier-bound
anchor, entry and arrival), `to_on(Start, centre, winding)` (the
carrier close that keeps the entry vertex), and `at_toward(p, dx,
dy)` (LB10 route 3, the straight arrival off an arc departure) —
carrier knowledge riding the TIP, consumed by a carrier-aware
`.fillet(r)`, with a typed spelling refusal (`ArcCarrierSpelling`)
walling the doors off from the generic binders.

What SURVIVES of it, verbatim: the derived-corner resolution
machinery (the squared-radius rule, the advance/reach gates, the
lifted S8 ladder — `path/arc_fillet.rs`, bit for bit), the
squared-radius design rule (LB4), and the mid-arc-seam rule (LB5:
`.to(Start)` needs a straight first side because the seam retrims
the entry vertex). What DISSOLVED: the register's spelling — under
§2c's axiom carrier knowledge cannot ride a tip, so the doors
re-spelled into the fused family (`at_on`+`fillet` → `arc_fillet` /
the `Center` arrival; `to_on` → `fillet_arc(r, Center { …, p:
Start })`; `at_toward` → the ordinary `.at(p).toward(dx, dy)`
arrival of a fused verb) and `ArcCarrierSpelling` retired with them
— a carrier-keyed refusal is unwritable against the kernel's state
types. The register's full text is the git history of this section
(and #386/#413's threads); the ratified surface is §2c.

## 2c. The fillet-family redesign (RATIFIED — fifteen rounds with
## Ev, merged #419; implemented by LIB-RESPELL, which re-spelled
## §2/§2a/§3 to this surface and compressed §2b to its historical
## note)

**THE AXIOM (leads by design — Ev, round 11): every verb can
depend ONLY on its incoming lattice state — Open / Point /
Angle / Directed, carrying nothing but its binding bits
(position and/or tangent) — plus the verb's own authored
arguments. Nothing else about the chain is knowable.** "What
leg produced this point" is not a fact any verb can consult:
being NURBS-adjacent, arc-adjacent, or line-adjacent is
UNREPRESENTABLE knowledge, so no verb can branch on it, refuse
on it, or silently depend on it. (The retired
`FilletCarrierUnsupported` wall was possible exactly because
this axiom was not load-bearing in the ratified text — a
carrier-aware refusal is unwritable under it.)

Everything below DERIVES from the axiom:

- A fillet that needs an arc carrier cannot LEARN it — so it
  must AUTHOR it: the fused verbs (round 7's reframe — an arc
  and the fillet that trims it are ONE authoring act).
- A bare `fillet(r)` knows only the tangent ray its directed
  point defines — so its incoming side IS that ray, uniformly
  (round 10's ray-extension semantics), whatever leg came
  before.
- No jet, no carrier data riding the tip, no reach into
  neighbors, no carrier-keyed refusals; the one surviving
  refusal (`NoCornerForFillet`) consumes only the verb's own
  inputs.

**The axiom is ENFORCED BY CONSTRUCTION (Ev, round 12 — not
discipline, structure):** every verb is a PURE FUNCTION over
bare state VALUES — e.g. `fillet(dp: DirectedPoint<T>, r: T) ->
FilletArrival<T>`, where the state types hold NOTHING but their
binding bits — living in a SEALED VERBS MODULE with no
visibility into the chain's accumulator. The chain type merely
threads state values through the verb functions and applies
their EMISSIONS (append-leg / insert-arc / extend-ray) to the
accumulating loop on the far side of the module boundary. A
verb consulting the previous leg is thereby UNWRITABLE (its
one parameter has two fields; the module cannot name the
accumulator) — re-introducing carrier-awareness would require
changing a signature, the loud reviewable act such a change
should be. CONSEQUENCE (the drift-proofing dividend, completed by
Ev's round-13 push toward full unification): the surface and
the replay driver become TWO MECHANICAL PROJECTIONS OF ONE
DECLARATION — a single TRANSITION TABLE, one row per
(state, verb, kernel fn, next state), macro-expanded (the
point_state precedent) into all four artifacts: the typed
method, the driver match arm, the Step variant, and the tag
entry. None of those four is written twice, so no two of them
can drift: a missing row is missing from all four, consistently
and loudly; an inconsistent pair is unwritable because there is
no second place to write it. All four are inside `profile`;
what the table does not reach is at the head of
`transition_table!`. **The round-9 exhaustiveness pressure does
NOT ride the same table, and this sentence used to say it did**
(smell-scan S195, corrected by #836): that pressure is over the
ARC-MODE enum `ArcData`, the table is over the VERB vocabulary,
and the three sites round 9 names as matching `ArcData`
exhaustively — the replay driver's arc dispatchers, the persist
wire, the tag map — are hand-written, none of them expanded
from a row. The pressure is real at each of those matches; it
is bought by hand at every site rather than projected from one
declaration. The V2 drift-proofing differential
census RETIRES to one smoke row (it becomes a tautology). The
entry signatures genuinely differ (typed method vs step data),
which is why the unification lives at the DECLARATION level —
the delegation alternative (typed methods calling through the
driver) was considered and rejected: it needs an unreachable!()
where the statically-known state meets the enum return, a
runtime assertion standing where the types should speak.
Spelling freedom (round 14, Ev's trait suggestion): the
REQUIREMENT is the invariant — every transition declared exactly
once, all projections (typed method, driver arm, Step variant,
tag) mechanically derived, drift unwritable. TWO spellings
satisfy it: (a) the table-macro generating all four artifacts;
(b) rows as ordinary trait impls (`impl Apply<Verb> for State`,
one per row, calling the kernel fn — rustdoc-visible, consistent
with the ArcSpecFor admissibility impls) plus a SLIM macro for
only the enum-side projections, which Rust cannot derive from
impls (no reflection — without that step the enum match is
hand-written and the drift point quietly returns). RULED (Ev, round 15): **lean (a), the
table-macro** — a macro exists in both spellings, so the trait
layer buys little, and (b)'s generic impls (flavored states ×
verb types × associated Out types) add trait-resolution surface
that taxes compile time where (a) expands to flat concrete
methods. The re-spell unit may still adopt (b) only if it
measures no compile-time cost and reads cleaner in situ.
Mechanism details (row/table syntax, emission vocabulary, module
seam) to the re-spell unit's spec.

**Shipped form: the invariant NOW HOLDS (LIB-RTABLE).** The one
declaration is `transition_table!` in
`crates/profile/src/path/program.rs`: one row per (state, verb,
kernel fn, next state), expanded into all four projections — the
typed method (rustdoc and signature carried by the row, geometry
by the kernel fn it names), the driver arm, the `Step` variant
and the `Verb` tag — so deleting a row breaks all four at
compile, and there is no second place to write a transition.

**The family (line is the unmarked middle-position default):**

- `fillet(r)` — line incoming, line arrival. The plain directed
  point fully determines a line carrier, so no fusion is needed
  on either side.
- `fillet_arc(r, spec)` — line incoming, arc arrival.
- `arc_fillet(spec, r)` — the fused verb: authors the incoming
  arc (mode per `spec`) and fillets off it, line arrival.
- `arc_fillet_arc(spec, r, spec₂)` — fused arc incoming, arc
  arrival.
- Plain `arc_to(spec)` remains for SHARP-cornered arcs;
  converting sharp→filleted is an edit at the same call site.
- **The endpoint lives INSIDE the endpoint-full variants**
  (Ev's round-8 observation, vindicating his wrap-the-args
  instinct): once the family admits endpoint-FREE modes, `p`
  stops being a uniform argument — so `Bulge{p, b}`,
  `Via{q, p}`, `Center{c, w, p}` carry their target, and the
  endpoint-free variants derive theirs.
- Fillet-after-fillet: the leg between two fillets is a line
  (verb₁'s arrival side), so bare `fillet(r)` chains it — the
  §2 both-ends-trimmed semantics carry over.

**Arrival halves** (unchanged from rounds 4–6, now living
inside the verbs): uniform binders `.at(p)` / `.angle(θ)` /
`.toward(dx, dy)` in either order on the fillet's own arrival
builder; the arrival carrier's residual spec per the ArcData
matrix below; The arrival's `Radius{r, side}` derives its
centre — never authored (a tangent circle at a directed point
has one free length + a side bit; `side` = which half of the
normal the centre sits on, the same single bit every mode
carries in its own dress — winding, bulge sign). The round-4
"arrivals ship Radius-only" staging is SUPERSEDED by round 9,
below.

**`ArcData` (rounds 5–6, extended round 8): spec modes as
standalone value types** — `Radius{r, side}`, `Bulge{p, b}`,
`Via{q, p}`, `Center{c, winding, p}`, plus the ENDPOINT-FREE
pair `Sweep{r, side, angle}` / `ArcLen{r, side, len}` (the arc
analogs of `line(len)`: tangent-departing, endpoint DERIVED —
DOF-complete from a directed start) — with admissibility a
TRAIT MATRIX
(`ArcSpecFor<State>`, the ArcTarget dispatch precedent): an
inadmissible (state, mode) pair is a missing impl —
unrepresentable, not refused; new modes additive. The matrix is
DOF-derived and STATE-KEYED: `Center@Point` supplies the
direction (retroactively, exactly what `at_on` was);
`Center@Directed` is EXCLUDED because the bound direction would
have to value-match the derived tangent (authored-once decides,
not taste); `Via` completes a directed anchor, underdetermines
a bare one; `Bulge` is chord-relative — admissible where the
chord exists: leg targets AND the fused verbs' incoming specs
(the target `p` is in the args), never arrivals. Full matrix at
the re-spell unit's spec. The wire/program layer records ONE
unified `ArcData` enum (record-as-you-lower keeps the authored
mode; the VQ contracts rely on that distinctness).
**RULED (Ev, round 9): the ENTIRE family ships in stage 1 —
every admissible (site, mode) pair, tested.** The forcing
argument is exhaustiveness one layer down: the wire enum is
matched exhaustively by the replay driver, persist wire, and
tag map, so a declared-but-unshipped variant would need a
refuse-stub arm (forbidden — fail-loud has no stubs) and an
undeclared one makes every later mode enum-growth churn
(tags, replay arms, schema conversation). The family is closed
and DOF-complete; Bulge/Via/Center lowering already exists;
Sweep/ArcLen are closed-form; Radius is the fillet-arrival
resolution needed regardless. Use-case-gating applies to no
variant; inadmissible pairs remain unrepresentable (free).

**Bounds:** the capability obligation (`ArcCarrierScalar:
Decide + Bounds`) sits per-method on the arc-involving verbs
(`fillet_arc`, `arc_fillet`, `arc_fillet_arc`) — the generic
lattice doors and plain `fillet(r)` never carry it. MEASURED
(2026-08-12): every scalar that drives an authoring chain (f64,
Interval, Probe) implements Bounds; Dual reaches profiles only
by lifting lowered ProfileLoop data. The bound is free in
practice either way. That last sentence is **guarded by the
compiler** (§Q6) and needs no register: the obligation is a trait
bound, so a scalar that stopped satisfying it fails to build at
every arc-involving call site rather than falsifying this
paragraph quietly.

**Honest residuals:** (a) a generic motif that fillets off a
RECEIVED tip of unknown leg kind cannot be written — the caller
names the verb (explicitness, arguably a feature; the jet
design of rounds 3–4 would have allowed it and was DROPPED with
the fusion reframe); (b) the verb inventory is four fillet
forms + two plain legs, kept tidy by ArcData; (c) SUPERSEDED by
round 10, below.

**Superseded en route (the record of the conversation):**
carrier-typed tips (round 2 — capture-at-fillet made them
unnecessary); capture-at-fillet itself and the 2-jet directed
point (rounds 3–4 — the fusion reframe made BOTH unnecessary:
with the arc authored inside the verb there is nothing to
capture and nothing second-order to carry). `at_on`, `to_on`,
`at_toward` all dissolve at the re-spell; the §2b register and
the §2/§3 fillet text rewrite at that unit.

**Round 10 (Ev): `FilletCarrierUnsupported` RETIRES.** The
incoming contact of bare `fillet(r)` lies on the tangent ray
AHEAD of the directed point, as new path (latent in §2's own
anchoring: the corner is the carrier INTERSECTION; behind-the-
ray-start refuses) — so after a NURBS leg the surviving ray
piece is a GENUINE LINE LEG extending from the curve's end,
lowered and recorded as such; no refusal, no special case. The
same semantics after a sharp `arc_to`: ray extension off the
arc's end. The system's whole shape: **contact ON the carrier ⇔
the fused verb** (`arc_fillet`; a `nurbs_fillet` has no closed
form and is an ABSENT VERB — unrepresentable, strictly better
doctrine than the old typed wall); **bare `fillet` ⇔ tangent-ray
extension**, uniform across line/arc/NURBS incomings. The one
surviving refusal is `NoCornerForFillet` (parallel carriers /
intersection behind the ray start) — geometry, not mechanism.

The program Step vocabulary carries the same spelling as the
surface: pre-release, the step set is not a compatibility
surface (LQ7a's clean break).

### §2c dissolution amendment — OnArc RETIRES (RATIFIED
### 2026-08-16; Ev's in-chat ruling, ratification delegated
### on a clean blast-radius census)

The re-spell unit shipped a fifth tip state, `OnArc` (an
interior arc arrival's tip, its carrier run to the anchor left
un-emitted for the NEXT fused verb to trim). Ev's ruling: the
axiom's own state vocabulary is the four binding states and the
directed point suffices — carrier continuation folds into the
fused verbs the way `arc_fillet` already folds carrier
authorship. OnArc is an emission-deferral trick wearing a
typestate, and it retires:

- **Arc arrivals emit their run at the verb** — the arrival
  carrier is the verb's own authored spec, so the emission is
  axiom-clean — and the tip lands as an ordinary directed
  point at the authored anchor (a HARD anchor, uniform with
  line arrivals).
- **Arc extension** replaces the `Radius@OnArc` row: from any
  directed point, a fused verb's `Radius{r, side}` incoming
  derives its carrier from the tip's binding bits and extends
  it FORWARD. Same-carrier continuation moves the incoming
  segment's end vertex (the §4 item 4 exemption, exactly as
  ray extension); a different `r` is a legal new tangent
  carrier with a constructed tangency at the tip — sound for
  every authored `r`, where the retired row was UNGUARDED for
  mismatched `r` (`bulge_from_center` computes from angles
  alone; the emitted run's bulge, claimed centre, and declared
  tangency went mutually inconsistent — a latent defect this
  amendment deletes structurally; the implementing unit pins
  it with an executed probe first).
- **Sharp-after-arc-arrival is restored**: the directed point
  takes an ordinary director. This closes the vocabulary gap
  the #576 §3 continuation-verb proposal named — that proposal
  is RETIRED (the state deletes instead) — and with it the
  LoopBuilder shim's last caller class, so the shim DELETES
  and #377 closes.
- **Deletions**: `OnArc`, `OnArcIncoming`, `TipState::OnArc`,
  the `DynTip::OnArc` replay arm, Python's `PathOnArc` + its
  arrival-builder returns (the builders re-target the directed
  point), and every doc surface that teaches the state.
- **What is unchanged**: shipped geometry — the census
  (2026-08-16, in LIB-LOG) found the fit gate already refuses
  a trim that would eat the authored anchor
  (`AnchorOutsideTrimmedExtent`), so every constructing chain
  already has its trim at/after the anchor and re-emits the
  IDENTICAL final vertex chain; `p: Start` closes; the entry
  fused rows. The all-blended-loop entry gap is NOT addressed
  here — it lives in the entry/seam machinery and stays a
  named gap.

## 3. Surface vocabulary

| Form | Lattice transition | Notes |
|---|---|---|
| **TIER 0 — CORE** | | |
| `Open` | → Open | the entry; every fillet's line arrival |
| `.at(p)` | Open → Point; Angle → Directed | position binder |
| `.angle(θ)` | Point → Directed; Open → Angle | angle binder (+ junction check on directed points) |
| `.tangent()` | directed point → Directed | inherit + declared; ill-typed on plain points |
| `.toward(dx, dy)` | Point → Directed; Open → Angle | **G1** — the exact director: same slot as `.angle`, ray stored verbatim |
| `line(len)` | Directed → Point; directed point → directed point | off a directed point, the straight continuation: the leg departs along the point's own intrinsic tangent. Binding bits only; there is no junction (no authored direction exists to classify) and nothing is declared. The minted vertex is a structural subdivision of the carrier — the loft vertex-budget shape. |
| `continue_to(target)` | directed point → directed point; `Start` → complete loop | the DECLARED point-target continuation: the same leg `line(len)` emits, its extent said as an authored POINT. The declaration is the verb, so nothing is inferred from the target's position — the kernel CHECKS the target lies on the departing point's ray, within ε_input, metered as the target's own lateral displacement (no lever: the datum is a point), and refuses `ContinuationTargetOffRay` past the band. The emitted vertex IS the authored target (§4 item 3), never its projection. `Start` is the structural CLOSER: it mints no vertex — the entry is already one — and runs the SEAM check unchanged, refusing `SeamTangent` when the seam is undeclared and zero-turn; `Start.arrives_tangent()` is the target that DECLARES it (§6's revised PQ4) and inverts that verdict. The zero-turn joint the leg itself mints is DECLARED by the verb (2026-09-02), so it enters `tangent_joints`. The closer classifies no departure junction at all (there is no authored direction), and where a closing leg DOES have one it refuses `JunctionTangent` like any other verb. |
| `nurbs_in_place(len1, …)` / `nurbs(curve)` | Directed → Point | legs; the NURBS pair awaits the segment vocabulary (VQ7) |
| `arc_to(spec)` | Point → Point (Bulge/Via/Center); Directed → Point (Sweep/ArcLen) | **§2c** — the sharp arc leg over the `ArcData` family; admissibility = the state-keyed trait matrix; `p: Start` closes |
| `fillet(r)` | Directed \| leg end → Open | line incoming (ray extension off a leg end), line arrival |
| `fillet_arc(r, spec)` | Directed \| leg end → per spec | line incoming, ARC arrival (see arrival rows below) |
| `arc_fillet(spec, r)` | Entry \| Point \| Directed → Open | fused arc incoming (arc extension from a directed point), line arrival |
| `arc_fillet_arc(spec, r, spec₂)` | as `arc_fillet` → per spec₂ | fused arc incoming, arc arrival |
| arrival `Center{c, w, p}` | (open fillet) → directed Point; `p: Start` → complete loop | complete at the verb; interior `p` is a HARD anchor (run emitted, ordinary directed point); `Start` keeps the entry vertex |
| arrival `Radius{r, side}` | (open fillet) → builder → directed Point | centre DERIVED from the directed anchor the binders supply |
| arrival `Via{q, p}` | (open fillet) → builder → directed Point; `p: Start` closes | anchor in the spec; one director pending |
| `Start` | directed-point VALUE | targeting it closes, structurally; the seam's junction is classified and, UNDECLARED, a tangent one refuses `SeamTangent` from every closing verb |
| `Start.arrives_tangent()` | target of EVERY closing verb → complete loop | the seam's joint is a declared **TANGENT joint** — the ONE arrival declaration, because every zero-turn joint is a declared tangent joint (Ev, in-chat, 2026-09-02). `line_to`, `continue_to`, `tangent_arc_to` and `arc_to(Bulge { … })` all take it: what it classifies is the JOINT, not the shape of the leg reaching it. The kernel CHECKS the arriving direction against `Start`'s own, banded through the funnel, the turn LEVERED by the arriving leg's arm (the datum is an angle; §4 item 1's precedent), refusing `SeamArrivalOffDirection` past ε_input, `JunctionCusp` for a reversed arrival and `SeamArrivalLeverTooShort` when the leg is too short to carry the question. It reads NOTHING about the carriers — identity is a fact about carriers, tangency a fact about directions. Joint 0 carries the flag, which the verify layer re-checks. |
| `.to(p)` on a bound arrival direction | Angle → Point | **G1** — the far-end anchor: the arrival side ENDS at its authored anchor |
| `circle(c, r)` | — → complete loop | **G1** — closed-carrier program form; a whole loop, not a chain step; authors no seam, so PQ4 is untouched |
| `circle_split(c, r, n, phase)` | — → complete loop | the declared-subdivision closed carrier: `n` equal arcs from `phase`, structural subdivisions of one carrier — the same no-seam story as `circle`, with the count and phase authored |
| `arc_continue(p)` | directed point → directed point | continues the incoming ARC carrier to `p`, minting a structural subdivision vertex; a same-carrier identity, so no junction check runs and nothing is declared |
| **TIER 1 — SUGAR** (one call each; expands to core; adds no semantics) | | |
| `line_to(p)` | Point → Point (also from line arrivals) | `.angle(toward p).line(dist)` |
| `tangent_arc_to(p)` | Directed → Point | the unique tangent arc |
| `nurbs_reversed(curve)` / `nurbs_mirrored(curve)` | Directed → Point | structural variants of rigid placement; VQ7-banked with the legs |
| `.turn(δ)` | directed point → Directed | `.angle(incoming + δ)`; `turn(0)` refuses → `.tangent()`; `turn(±π)` hits the reverse class |

The retired-name doors (`arc_to(p, bulge)` / `arc_via` / `arc_center`
as standalone verbs, and the §2b compat trio `at_on` / `to_on` /
`at_toward`) are DELETED: the consumer re-spell moved every call site
onto the rows above, so the surface has one spelling per act.

All-rounded square (4 anchors + 4 directions; every mᵢ a real
on-path point, e.g. a side midpoint):

```text
Open.at(m1).angle(east)
    .fillet(r).at(m2).angle(north)
    .fillet(r).at(m3).angle(west)
    .fillet(r).at(m4).angle(south)
    .fillet(r).to(Start)
```

Mixed sharp + fillet + tangent:

```text
Open.at(a).angle(d)
    .line(len)
    .tangent().tangent_arc_to(b)
    .angle(θ).fillet(r).at(m).angle(θ2)
    .line(len2)
    .arc_to(Bulge { p: Start, b })
```

**Refusals.** Compile-time, from the lattice and the §2c trait
matrix: double director; `fillet`/legs from non-Directed tips;
`.tangent()` on a plain point; leading `.fillet`/`.tangent` (§2
entry rule); the overdetermined tangent-LINE close (§2's closure
bullets — `.tangent()` leaves the tip Directed, where the only
straight leg is `line(len)` and no targeting verb is in reach, so
there is no verb to refuse at runtime); a NURBS leg targeting
`Start`; every INADMISSIBLE
(state, mode) pair of the `ArcData` matrix is a missing impl —
unrepresentable, not refused (at the wire the same pair is the
replay driver's Transition class). Typed runtime errors, from
geometry — the lattice guarantees the authoring, never the
geometry: the junction check (§4 item 1); `NoCornerForFillet`
(r too large, carriers parallel/non-intersecting/never meeting,
corner behind the ray, no tangent circle); the M8 conditioning
gate `FilletOffsetLeverTooShort`; `AnchorOutsideTrimmedExtent`
(a trim would eat an anchor — the #101 `TangentJointOutOfRange`
fit-gating generalized, carrying the side's carrier kind; also
checked for the entry point under a seam fillet);
`SeamRetrimsArcFirstSide` (a `.to(Start)` seam needs a straight
side 1 — closing onto a carrier while keeping the entry vertex is
`fillet_arc(r, Center { c, winding, p: Start })`);
`TangencyContradicted` from the verify layer as today. From §2a and the spec family:
`NonpositiveCircleRadius`; `ZeroDirection`; `ArcViaCollinear`;
`DegenerateArcChord`; `DegenerateArcSpec` (a zero bulge, a
non-positive sweep/arc-length); `ArcCenterNotEquidistant`;
`DegenerateArcCenter`; `FarEndAnchorWithoutFillet`;
`CircleSplitCount`; `ArcContinueNeedsArcCarrier` and
`ArcContinueOffCarrier` (no incoming arc carrier to continue; an
authored target off it — authored points never re-project);
`SeamArrivalOffDirection` (a DECLARED seam arrival that definitely
does not continue the entry's outgoing direction — the levered
miss, its lever and the member that declared, in the payload; a
REVERSED one is `JunctionCusp`);
`SeamArrivalLeverTooShort` (a declared arrival on a closing leg
whose arm cannot carry the question).
RETIRED with the §2b register: `ArcCarrierSpelling` and the doctrine-level
`FilletCarrierUnsupported` — under the §2c axiom a carrier-keyed
refusal is unwritable (contact ON a carrier is the fused verb;
bare `fillet` is ray extension; `nurbs_fillet` is an absent verb).
What survives of the first is not carrier-keyed and so keeps its
own name: `ArcLegOnOpenFillet`, a sharp arc LEG reached while a
fillet is still open — the arrival's carrier is authored INSIDE
`fillet_arc`/`arc_fillet_arc`, so a leg departing an
already-positioned arrival point would claim that direction twice.
Compile-time, from §2a: `circle`'s result is a loop, so no chain
verb follows it; `.toward` is a second director exactly as
`.angle` is; the far-end `.to(p)` needs the position slot empty
and the angle slot bound.

## 4. Safety invariants

1. **No junction is silently within the input tolerance of
   tangent.** On a directed point, `.angle(θ)`/`.turn(δ)`
   classifies θ against the incoming tangent and its reverse.
   Outcomes: definitely-sharp proceeds; within ε_input of the
   TANGENT direction refuses — ONE refusal, one recourse, for any
   sub-ε_input margin (D4's two-tolerance principle): "this
   junction is tangent at any precision you could care about — if
   intended as tangency onto a new carrier, use `.tangent()`, which
   makes it exact by construction; if intended as a straight
   continuation of the same line, spell it `line(len)` off the
   directed point — no junction exists there; otherwise move the
   geometry (or lower the tolerance)". The margin rides the payload
   as data; the message never forks on exactly-on vs in-band.
   Within ε_input of the REVERSE direction refuses as a cusp (the
   reverse-tangent class).
   Declared cusps are legal kernel geometry (D1 tier 3's declared
   second-order wedge arm; #131 ruled 2026-08-23), but the
   authoring door — a cusp analogue of `.tangent()` that authors
   the reverse-tangent junction exactly and emits the declaration
   — is unbuilt (#941); until it ships the junction refuses, and
   the refusal names the absent verb.
2. **No tangency without declaration**: tangency enters only via
   `.tangent()` or fillet construction; the lowering emits the
   declared flags — declaration by construction, never inference.
   (`.tangent()` is a construction, not a claim: the direction is
   inherited exactly, so there is nothing for verification to
   contradict. `TangencyContradicted` remains the verify-layer
   door for raw-authored segment chains, where a flag is a claim
   about independently-typed numbers.)
3. **Every authored point lies on the final path, authored
   once**: points enter only as path points (entry, targets,
   anchors); junction-owned points (fillet corners, trim points,
   NURBS P0/P1) are implied, never authored; the anchor fit check
   enforces the invariant where trims could threaten it.
4. **Every zero-turn joint is a declared tangent joint** (RULED —
   Ev, in-chat, 2026-09-02). The lattice checks DIRECTIONS and
   never asks whether the two carriers are the same: identity is a
   fact about carriers, tangency is a fact about directions, and
   where the directions agree the joint is tangent whatever the
   carriers do. So `.tangent()` onto the incoming carrier is a
   declared tangent joint and is legal, and the continuation verbs —
   `line(len)` off a directed point, `continue_to`, the post-fillet
   extension — DECLARE the zero-turn joint they mint: declaration by
   construction, exactly as `.tangent()` is, re-checked by the verify
   layer.

   *History (this item's earlier reading, kept because the refusals
   it names appear in older logs).* It read "same-carrier junctions
   refuse when DECLARED — `.tangent()` onto the incoming carrier is
   identity, not tangency (#101's rule)", and the continuations were
   said to mint no junction and declare nothing. `SameCarrierJunction`
   was that reading's refusal, along with `refuse_identical_carriers`
   and `validate`'s `TangencyContradicted { same_carrier: true }`;
   all three are retired. What is NOT retired is the undeclared case:
   an undeclared zero-turn junction still refuses `JunctionTangent`
   (or `SeamTangent` at the seam). The rule is "declared", not
   "anything goes".

**RULED (#433 — Ev, in-chat, 2026-09-01, with a second-round
extension): the lattice and `validate` AGREE.** A straight run
subdivided at an interior vertex is well formed as DATA
(`validate`, unchanged: it is what STEP import and raw authored
loops routinely produce, and an UNDECLARED one claims nothing) and
it is expressible STRUCTURALLY in the algebra — `line(len)` off a
directed point, chained, mints subdivision vertices on the one
carrier the binding bits already determine (item 4 above; the §3
row). Since 2026-09-02 the algebra also DECLARES those joints, so a
lattice-authored subdivided run reaches the gate with its zero-turn
joints named; the raw-authored one still reaches it undeclared, and
both are accepted — that is what "the two doors agree" means here. The two doors were never measuring different things about
this shape; the authoring door was simply missing its spelling. An
AUTHORED direction landing in the tangent band still refuses,
recourse as in item 1: a target that happens to be collinear is a
value coincidence, and the ladder never reads intent off a margin.

Per the ruling's second-round extension, `arc_continue` is NOT kept
as the §2c axiom's exception: it is scheduled for REMOVAL, its
subdivision need re-spelling as declared subdivision on the arc leg
itself — the open-carrier analog of `circle_split`, with vertices
minted at the chain's emission layer where the axiom's bookkeeping
legitimately lives. Companion: `RawLoop` is not an authoring door —
the vertex table is the materialized form intensional recipes
evaluate into. The units: this half is **BOOL-8**, the
`arc_continue` retirement **BOOL-10**, the declared point-target
continuation and its closer **BOOL-11**, the raw-door demotion
**BOOL-9** (resequenced behind BOOL-11), the seam's declared arrival
**BOOL-12**. **Both lattice halves and the SEAM have landed** — the
interior continuation (BOOL-8), the declared point-target form with its
structural closer (BOOL-11), and the declared arrival that admits a
subdivision or G1 seam (BOOL-12, §6's revised PQ4); the RAW DOOR remains
(BOOL-9), and #433 closes when it lands.

**The seam, measured here and RULED (third round, Ev, in-chat,
2026-09-01) — and LANDED (BOOL-11).** The interior continuation as
BOOL-8 shipped it spells INTERIOR subdivisions only, and a straight run
crossing the SEAM was unauthorable in either rotation: with the seam at
a corner the closer departs the run's subdivision vertex (a tangent
DEPARTURE), and with the seam at that subdivision vertex the
seam's own junction is the straight one, which PQ4 (§6, no mid-carrier
seam) refuses by construction. What forced the choice is the strict
corner/subdivision ALTERNATION that one subdivision per side produces:
the seam junction and the junction the closer departs are then always
adjacent and always of different kinds, so no rotation puts a corner at
both. The lift layer has carried a name for this wall since it was written:
`LiftRefusal::DeclaredJointBeforeClosingLine` (`crates/profile/src/lift.rs`),
whose message says a run's "leaving segment closes the loop straight;
`.tangent().line(len)` cannot close" — the straight wall exactly. (An
earlier draft cited `SameCarrierClose` here. That variant is about ARC
runs — `arc_continue` has no closing form — so it names a different
wall; the identical mis-citation was dropped from the lily demo's
comment in the same unit, and this is its retained sibling.)

The ruling: the straight continuation gains a DECLARED POINT-TARGET
form — the leg declared to land on a NAMED point, with the kernel
CHECKING that the target lies on the departing directed point's ray and
refusing when it does not (a declared structural fact, verified, never
inferred from a value coincidence). The target is ANY authored point,
and the structural CLOSER — `Start` as the target — is the special case
that ends the seam wall. Axiom-clean: it consults the directed point's
binding bits, the authored target, and, for the closer, `Start`, which
is the chain's own emission-layer bookkeeping. PQ4 stands unchanged;
the closer makes a seam at a CORNER sufficient for an
all-sides-subdivided outline.

**Landed as `continue_to(target)` (§3's row).** The f64 question the
third round left open — exact-or-refuse versus a banded check — was
ruled in the fourth round: BANDED, as ever, because the DECLARATION is
what legalizes the band. With the intent authored, comparing the target
against the ray is authored-data CONSISTENCY (the arc verbs' class),
not the value inference the ladder refuses, which reads intent OFF a
coincidence nobody declared. The unit's three decisions, recorded here
for the record they belong to:

- **Which ε: the run's own linear band, whose refusing edge is
  ε_input.** The two candidates were not really alternatives. ε_input
  IS K·ε (D4's two-tolerance principle — a role name, not a third
  dial), and K·ε is the escalation band's upper edge, so "use ε_input"
  and "use the run band" name the same threshold; what differs is
  whether the comparison goes through the predicate funnel. It does.
  Below ε_precision the target and the ray are the same place at the
  precision anything here represents, and the declaration is
  consistent; above ε_input they are definitely different places, and
  the authored data contradicts itself; between them nothing is
  decidable and the band ESCALATES. A bare comparison against K·ε would
  have swallowed that middle and decided where the numbers cannot,
  which is the one thing escalate-never-guess forbids. ε_input is the
  right edge to refuse at for the reason the role exists: the question
  is about authored INPUT — does the point the author wrote agree with
  the intent the author declared — not about what the kernel can build.
- **The lever: none, and that is the dimension-honest answer.** The
  miss is `(target − at) · n̂` with `n̂ ⟂ û` a unit direction — already
  the target's own displacement from the ray, in metres, and already
  the point deviation the tolerance is defined about. §4 item 1 levers
  ITS margin (sin φ · arm) because its datum is an ANGLE, which means
  nothing until an arm says what it displaces; here the datum is a
  POINT. Levering anyway would mean dividing the length by the leg to
  make an angle and multiplying it back — bits lost, and a threshold
  that would drift with how far along the ray the author put the point.
- **The D2 row: 1 (reachable by input, invalid) — typed error.** Row 0
  was asked first and answered NO out loud: whether a runtime `Point2`
  lies on a runtime ray is a value fact, and making it unrepresentable
  needs the type system to carry the geometry. Row 3 (poison) would be
  wrong — an off-ray target is not a domain degeneracy, it is
  well-formed input that disagrees with itself.

Two smaller decisions came with it, both recorded because they are
contracts rather than implementation. The emitted vertex is the
AUTHORED TARGET, never its projection onto the ray: item 3 says every
authored point lies on the final path, and projecting would also leave
the closer's endpoint a hair off the entry vertex, which is the one
place a hair is not allowed. And the CLOSER mints no vertex at all —
`Start` is the entry, which the loop already carries.

**What the closer did NOT move, measured.** It ends the DEPARTURE half
of the wall and leaves PQ4's half exactly where it was, and the two are
now separable at the refusal rather than only through the fixture that
provoked them — and separable by TYPE, not by a payload tag. A tangent
DEPARTURE on a closing leg refuses `JunctionTangent`, exactly as any
other departure does; a tangent SEAM refuses `SeamTangent`, a refusal
only a seam can produce.

That is a correction to how this first landed (ruled 2026-09-01). The
first version gave one refusal a `site: Departure | Seam` payload, which
kept a close-only second name for a departure — and a tangent departure
on a closing leg is geometrically identical to one mid-chain, with an
identical recourse now that the declared closer exists (spell it
structurally). A second name for the same fact is uniformity debt
against this document's own rule that `Start` goes through ORDINARY
verbs, and it predated the program: it is the original lattice's
`line_close: bool`, from when the recourses really did differ. Two types
also beat a tag on the merits — a tag must be read and a `{ .. }`
pattern can ignore it, whereas types cannot be confused by a caller, and
each refusal now carries only the payload its own recourse needs. Re-running BOOL-8's exhaustive hunt with the declared closer in
the alphabet (64 rings — both lily section widths, both ends of the
shoulder parameter, every starting vertex, both directions) closes 32
of them where the undeclared closer still closes zero. Every closure is
a spelling whose SEAM is a corner, which is the ruling's "sufficient"
made a measurement.

The same run also measures what remains, and it is a fact about lily
rather than about the closer. The spellings that close sit at OPPOSITE
PARITY in the two sections a leaf plan carries: in the kite
(`shoulder = 0`) the corners are the TIPS — starts 0, 2, 4, 6 — and in
the rectangle (`shoulder = 1`) they are the SHOULDERS — starts 1, 3, 5,
7. The two sets are disjoint, and not by accident: the kite's corner
set IS its tips and the rectangle's IS its shoulders, which are
disjoint points of the outline whatever vertex budget is spent. A loft
matches segment j of every section to segment j of every other, so all
of a plan's sections must be authored at ONE rotation — and a plan
carrying both a `shoulder = 1` base and a `shoulder = 0` belly has no
rotation that gives every section a corner at its seam. So the lily
demo does NOT migrate here: its remaining wall is PQ4's, reached by the
one section whose seam is forced onto a subdivision vertex.

**That question was asked and RULED, and the other half of the wall is
gone too (fifth round, Ev, in-chat, 2026-09-01; landed by BOOL-12).**
A DECLARED subdivision vertex IS an admissible seam — the loop cut where
the author said the carrier continues — and so is a DECLARED G1 joint.
PQ4's revised entry in §6 carries the rule, the loop-start reading of
its two named consumers, and the uniformity argument; what belongs here
is the mechanism and its three decisions.

**The declaration rides the TARGET**, not the verb, because the seam is
the one junction whose ARRIVING leg is the later-authored one. Every
other declaration in this document rides the departing leg, and at the
seam there is no departing leg to ride: the entry's first side is
authored at the front, where §2's entry rule makes the seam's content
ill-typed. So `Start` gains ONE declaring sibling —
`Start.arrives_tangent()`, because every zero-turn joint is a declared
tangent joint and there is nothing else to say there — and the closing
verbs are the ordinary ones, unchanged, each still one
`transition_table!` row. Admissibility is still the §2c matrix
discipline, and the matrix over the CLOSERS is FULL: `line_to`,
`continue_to`, `tangent_arc_to` and `arc_to(Bulge { … })` all take it,
because the token classifies the JOINT and not the shape of the leg
reaching it. What stays unrepresentable is the arc DATA that has no
arm: `arc_to`'s `Via` and `Center` modes carry no declaring target, so
those pairs are missing impls rather than refusals, and at the wire they
are the replay driver's `Transition` class (issue 1579).

**The two declarations on a closing leg are INDEPENDENT.** `continue_to`
declares the DEPARTURE (this leg continues its run, and the joint it
mints is a declared tangent joint); the target declares the seam's own
joint. A closing leg departing a corner and arriving zero-turn declares
only the second; one that does both declares both; one that turns at
each end declares neither and is the plain `line_to(Start)` it always
was. Folding the arrival into `continue_to` would have made the middle
case unspellable.

- **Which ε: the run's own linear band, whose refusing edge is
  ε_input.** The two candidates were not really alternatives. ε_input
  IS K·ε (D4's two-tolerance principle — a role name, not a third
  dial), and K·ε is the escalation band's upper edge, so "use ε_input"
  and "use the run band" name the same threshold; what differs is
  whether the comparison goes through the predicate funnel. It does.
  Below ε_precision the target and the ray are the same place at the
  precision anything here represents, and the declaration is
  consistent; above ε_input they are definitely different places, and
  the authored data contradicts itself; between them nothing is
  decidable and the band ESCALATES. A bare comparison against K·ε would
  have swallowed that middle and decided where the numbers cannot,
  which is the one thing escalate-never-guess forbids. ε_input is the
  right edge to refuse at for the reason the role exists: the question
  is about authored INPUT — does the point the author wrote agree with
  the intent the author declared — not about what the kernel can build.
- **The lever: none, and that is the dimension-honest answer.** The
  miss is `(target − at) · n̂` with `n̂ ⟂ û` a unit direction — already
  the target's own displacement from the ray, in metres, and already
  the point deviation the tolerance is defined about. §4 item 1 levers
  ITS margin (sin φ · arm) because its datum is an ANGLE, which means
  nothing until an arm says what it displaces; here the datum is a
  POINT. Levering anyway would mean dividing the length by the leg to
  make an angle and multiplying it back — bits lost, and a threshold
  that would drift with how far along the ray the author put the point.
- **The D2 row: 1 (reachable by input, invalid) — typed error.** Row 0
  was asked first and answered NO out loud: whether a runtime `Point2`
  lies on a runtime ray is a value fact, and making it unrepresentable
  needs the type system to carry the geometry. Row 3 (poison) would be
  wrong — an off-ray target is not a domain degeneracy, it is
  well-formed input that disagrees with itself.

Two smaller decisions came with it, both recorded because they are
contracts rather than implementation. The emitted vertex is the
AUTHORED TARGET, never its projection onto the ray: item 3 says every
authored point lies on the final path, and projecting would also leave
the closer's endpoint a hair off the entry vertex, which is the one
place a hair is not allowed. And the CLOSER mints no vertex at all —
`Start` is the entry, which the loop already carries.

**What the closer did NOT move, measured.** It ends the DEPARTURE half
of the wall and leaves PQ4's half exactly where it was, and the two are
now separable at the refusal rather than only through the fixture that
provoked them — and separable by TYPE, not by a payload tag. A tangent
DEPARTURE on a closing leg refuses `JunctionTangent`, exactly as any
other departure does; a tangent SEAM refuses `SeamTangent`, a refusal
only a seam can produce.

That is a correction to how this first landed (ruled 2026-09-01). The
first version gave one refusal a `site: Departure | Seam` payload, which
kept a close-only second name for a departure — and a tangent departure
on a closing leg is geometrically identical to one mid-chain, with an
identical recourse now that the declared closer exists (spell it
structurally). A second name for the same fact is uniformity debt
against this document's own rule that `Start` goes through ORDINARY
verbs, and it predated the program: it is the original lattice's
`line_close: bool`, from when the recourses really did differ. Two types
also beat a tag on the merits — a tag must be read and a `{ .. }`
pattern can ignore it, whereas types cannot be confused by a caller, and
each refusal now carries only the payload its own recourse needs. Re-running BOOL-8's exhaustive hunt with the declared closer in
the alphabet (64 rings — both lily section widths, both ends of the
shoulder parameter, every starting vertex, both directions) closes 32
of them where the undeclared closer still closes zero. Every closure is
a spelling whose SEAM is a corner, which is the ruling's "sufficient"
made a measurement.

The same run also measures what remains, and it is a fact about lily
rather than about the closer. The spellings that close sit at OPPOSITE
PARITY in the two sections a leaf plan carries: in the kite
(`shoulder = 0`) the corners are the TIPS — starts 0, 2, 4, 6 — and in
the rectangle (`shoulder = 1`) they are the SHOULDERS — starts 1, 3, 5,
7. The two sets are disjoint, and not by accident: the kite's corner
set IS its tips and the rectangle's IS its shoulders, which are
disjoint points of the outline whatever vertex budget is spent. A loft
matches segment j of every section to segment j of every other, so all
of a plan's sections must be authored at ONE rotation — and a plan
carrying both a `shoulder = 1` base and a `shoulder = 0` belly has no
rotation that gives every section a corner at its seam. So the lily
demo does NOT migrate here: its remaining wall is PQ4's, reached by the
one section whose seam is forced onto a subdivision vertex.

**That question was asked and RULED, and the other half of the wall is
gone too (fifth round, Ev, in-chat, 2026-09-01; landed by BOOL-12).**
A DECLARED subdivision vertex IS an admissible seam — the loop cut where
the author said the carrier continues — and so is a DECLARED G1 joint.
PQ4's revised entry in §6 carries the rule, the loop-start reading of
its two named consumers, and the uniformity argument; what belongs here
is the mechanism and its three decisions.

**The declaration rides the TARGET**, not the verb, because the seam is
the one junction whose ARRIVING leg is the later-authored one. Every
other declaration in this document rides the departing leg, and at the
seam there is no departing leg to ride: the entry's first side is
authored at the front, where §2's entry rule makes the seam's content
ill-typed. So `Start` gains ONE declaring sibling —
`Start.arrives_tangent()`, because every zero-turn joint is a declared
tangent joint and there is nothing else to say there — and the closing
verbs are the ordinary ones, unchanged, each still one
`transition_table!` row. Admissibility is still the §2c matrix
discipline, and the matrix over the CLOSERS is FULL: `line_to`,
`continue_to`, `tangent_arc_to` and `arc_to(Bulge { … })` all take it,
because the token classifies the JOINT and not the shape of the leg
reaching it. What stays unrepresentable is the arc DATA that has no
arm: `arc_to`'s `Via` and `Center` modes carry no declaring target, so
those pairs are missing impls rather than refusals, and at the wire they
are the replay driver's `Transition` class (issue 1579).

**The two declarations on a closing leg are INDEPENDENT**, and lily's
own section needs three of the four combinations. `continue_to`
declares the DEPARTURE (this leg continues its run); the target declares
the ARRIVAL (it continues the entry's first side). A closing leg
departing a corner and arriving straight declares only the second; one
that does both declares both; one that turns at each end declares
neither and is the plain `line_to(Start)` it always was. Folding the
arrival into `continue_to` would have made the middle case unspellable.

- **Which ε: the run's own linear band, refusing edge ε_input**, for the
  reason `continue_to`'s target check uses it — the question is about
  authored INPUT, whether the direction the author's points produce
  agrees with the intent the author declared. Zero accepts, the band
  ESCALATES, past ε_input refuses typed.
- **The LEVER: the arriving leg's own arm, and here there IS one.** The
  datum is `sin` of the turn between the arriving direction and the
  entry's outgoing one — dimensionless, and comparing it against a
  length tolerance is a category error. §4 item 1 levers its turn margin
  for exactly this reason, and this is the same junction at the same
  vertex, so it uses the same lever: the emitted leg's length for a
  straight closer, `radius.min(chord)` for an arc one. The product is,
  TO FIRST ORDER, the lateral displacement the misalignment opens at the
  seam — for an arc leg the exact figure is `s·sin φ + s²/2R`, and the
  lever takes the leading term exactly as §4 item 1 does — which is the
  point deviation the tolerance is defined about, so the threshold is on
  the DISPLACEMENT and does not drift with leg length. This is the
  mirror of `continue_to`'s decision to lever NOTHING: there the datum
  was already a length, and levering it would have invented an angle.
- **The D2 row: 1 (reachable by input, invalid) — typed error.** Row 0
  again answers NO out loud: whether two runtime directions are parallel
  is a value fact. Row 3 (poison) would be wrong — a misaligned arrival
  is well-formed input disagreeing with itself, not a domain degeneracy.

**The token classifies the JOINT, the seam check consults NOTHING about
the carriers, and there is ONE token** (RULED — Ev, in-chat,
2026-09-02, superseding the two-token reading of the day before). The
only entry-side datum the check reads is `Start`'s own direction —
exactly what the interior junction check reads of a directed point. It
never asks whether the two sides ride the same carrier: identity is a
fact about CARRIERS, tangency a fact about DIRECTIONS, and where the
directions agree the joint is tangent whatever the carriers do. So
`Start.arrives_tangent()` is the whole arrival vocabulary, every closing
verb takes it, and a declared zero-turn seam onto ONE carrier is a
tangent joint rather than a contradiction.

Two refusals retired with that reading and are recorded here because
older logs name them: `SameCarrierJunction` (the lattice's
declared-tangency-onto-identity refusal, and BOOL-11's addendum arm at
the collinear tangent-arc close) and `validate`'s
`TangencyContradicted { same_carrier: true }`. What stays is the
UNDECLARED case — `JunctionTangent` mid-chain, `SeamTangent` at the seam,
`UndeclaredTangency` for a materialized loop — because the rule is
"declared", not "anything goes".

One gate was REPLACED rather than removed. `tangent_arc_geom` used to
refuse a collinear target under a declared departure as carrier
identity; the collinear FORWARD case is now legal (the arc degenerates
to the straight segment the declaration asks for), and what survives is
a geometry question about the chain's own leg: behind the tip the
tangent-chord angle is π and the bulge unbounded, so that refuses
`DegenerateArcChord`.

Three more contracts came with the arrival. A declared arrival that
REVERSES the entry's outgoing direction has a near-zero turn too, and it
is a CUSP: it refuses `JunctionCusp`, the name it carries at every other
junction — one fact, one refusal. A closing leg whose ARM is not
definitely positive carries no question at all — the levered turn and
the levered alignment both read Zero, so any arriving direction would
satisfy the declaration — and refuses `SeamArrivalLeverTooShort`, which
is the degeneracy `junction_check` already refuses at its own site. And
a declared arrival puts joint 0 in `tangent_joints`, where the verify
layer re-checks it.

**Every seam ARRIVAL is classified as a seam.** `line_to(Start)` already
passed `junction_check`'s seam flag; the two ARC closers did not, so a
stadium closed with `.tangent().tangent_arc_to(Start)` refused
`JunctionTangent` — a departure's name for the loop's own junction,
measured before this unit built anything. The flag is the only thing
that says "this junction is the seam", and a seam arrival has a recourse
no departure has (the entry cannot carry `.tangent()`), so naming it as
a departure sent the reader to a spelling the seam does not have. Both
arc closers pass `true` now.

**The band's guarantee is PER LEG, and the run-level certifier is the
data gate** (recorded after review; a limit, not a hole). Each
`continue_to` checks THIS leg's target against THIS leg's declared ray.
That is the honest scope of the on-ray band, and it composes as any
per-step tolerance composes: forty legs each accepting a same-side miss
of 0.5·ε put the run's end 20·ε — two full ε_input — off the ray it
started on, with every per-leg check green and correctly so. The drift
does not escape quietly: the data gate sees the accumulated bow that no
per-leg check can, and ESCALATES on `chord_side` rather than accepting
it. Loud, not silent, and not a guess. Tightening the per-leg band
would not change the shape of this, so the answer is the gate — which
is already this design's answer for run-level facts.
`the_per_leg_band_composes_and_the_data_gate_catches_the_sum` pins the
gate's verdict.

**The ε_input story of the new arms** (D4 consequence (iv): for every
arm added to a decision, name which ε_input story it belongs to, or say
why it belongs to none). The definite arm,
`ContinuationTargetOffRay`, belongs to the AUTHORED-DATA story: the
target and the ray are definitely different places, and the recourse is
to move the target onto the ray — or, if the miss is genuinely
acceptable, to WIDEN the input tolerance. That is the opposite
direction from the tangency refusals, where closeness is what refuses
and lowering the tolerance is the recourse, and the message says so
rather than inheriting the wrong half of the template.

The in-band arm belongs to NO existing story, and this is the "or say
why" branch. The shared sub-ε_input recourse is "declare the
coincidence, move the geometry, or lower the tolerance" — and at this
site the first lever is meaningless, because the DECLARATION IS THE
VERB: there is nothing left to declare that `continue_to` has not
already said. So the escalation at `path_continuation_target_offset`
composes its own message from the margin payload, with its own two
levers (move the target, or widen ε_input), instead of the shared tail.
The same correction was owed to `path_leg_length`, which had been
reporting an authored extent under the prefix "path junction
classification" — it is not a junction, and neither is this.

The #101 verify layer runs UNCHANGED on the lowered output — the
algebra is upstream insurance; the flags remain the contract of
record.

## 5. Elaboration and implementation

Strictly forward, single pass, seam last; every step local and
closed-form (directors bind departures; legs bind from them; each
fillet is the ray×carrier corner construction with both carriers
fixed when reached; the seam resolves when a verb targets
`Start`). No chain expressible in this surface needs
right-to-left propagation, by induction over the chain: every
binding verb consumes only its own arguments plus already-bound
state (the entry binds side 1 from its args; a leg consumes the
bound slot + its args; a fillet arrival binds from its own
anchor/director args; the seam consumes `Start` — bound at entry
— plus the final carrier), and the one construct that needed a
LATER binding (the anchor-free both-ends-trimmed side) is
unwritable. D9:
elaboration is pure f64 structure selection (C6 boundary — it
decides leg parameters, never topology); the lowered profile runs
the ordinary generic pipeline. `UnderdeterminedLeg`/
`OverdeterminedJunction` remain as elaborator backstops, expected
unreachable from the typed surface; a reachable case is a design
finding to bring back here, not a silent fix.

Representation: ONE struct — `pos: Option<PosData>`,
`ang: Option<Dir>` (the §2a widening: the ray stored verbatim, the
angle carried beside it) — under type-level lattice markers
(`Tip<P, A>`; the four states are the instantiations, the
position marker carrying the plain-vs-directed flavor). Binders
are written once, generic over the slot they do not touch;
`.tangent()` exists only at `Tip<HasPos<WithIncoming>, NoAng>`;
the junction check reads the flavor's optional incoming tangent
at runtime (one generic function). Fields private, binders the
only constructors — off-lattice states are representable at
runtime but unreachable through the surface.

## 6. Decided and open questions

Decided during review (details in #124): mixed authoring is OUT —
a loop is authored either in the algebra or as a raw vertex+bulge
chain, never both (representation uniqueness); declared cusps are
legal at the kernel (#131 ruled into D1 tier 3's declared
second-order wedge arm) with the authoring verb banked at #941 —
cusps refuse here until it ships; there is no
path-concatenation operator (builder functions instead).

**PQ4 — mid-carrier seams: REVISED (Ev, in-chat, 2026-09-01,
the Q1 fifth round; implemented by BOOL-12).** The v1 rule was
that a closed loop's seam sits at a junction or fillet only and
closing mid-side is refused; the M2 closed-carrier
conventional-split precedent was considered and declined, because
the relaxation was thought to touch the same-carrier discipline
(one authored side = one carrier) that germ matching and the merge
ladders lean on. The rule now reads:

> A closed loop's seam may sit at a DECLARED TANGENT JOINT, mid-side
> or not — the seam is a joint like any other, and every zero-turn
> joint is a declared tangent joint. The declaration rides the
> closing verb's TARGET, `Start.arrives_tangent()`, which every
> closing verb takes; the kernel CHECKS the arriving direction
> against `Start`'s own within ε_input through the funnel, refusing
> `SeamArrivalOffDirection` when it definitely does not continue,
> and asks NOTHING about the carriers. An UNDECLARED zero-turn seam
> keeps refusing `SeamTangent`, and nothing is inferred from a
> value: `Start` alone reads exactly as it always did.

**Why the seam has a spelling the interior does not, and why that
is not a leak of the `Start`-goes-through-ordinary-verbs rule.**
Every declaration that elsewhere rides the DEPARTING leg —
`.tangent()` for a G1 junction, `line(len)`/`continue_to` for a
straight continuation — has no departing leg to ride at the seam:
the entry's first side is authored FIRST, at the front, where the
seam's content is ill-typed by §2's entry rule (neither adjacent
carrier is bound there). The seam is the one junction whose
arriving leg is the later-authored one, so an arrival-side
declaration has no interior counterpart to be uniform WITH. The
verbs are the ordinary ones; what is new is a target value, which
is the same mechanism `Start` itself has always been.

**The loop-start reading the ruling required before the build**
(BOOL-12, reported before implementation). PQ4's recorded
rationale named two consumers. Neither distinguishes a seam vertex
from an interior subdivision vertex, and neither carries the
one-authored-side-one-carrier premise:

- **Germ matching** (`crates/topo/src/boolean/mod.rs`'s `HalfGerm`,
  matched in `boolean/join.rs`) keys a null-edge half by its
  `(A-face, B-face)` pair and its 3-D direction — "never by slot
  position or dynamic face lookups", as its own doc says. It never
  sees a profile loop, a side, a carrier or a vertex index, so it
  cannot tell the two vertices apart. Its real premise is one
  A-face and one B-face per germ line, i.e. MAXIMAL-FACED operands,
  which a different gate enforces.
- **That gate**, `gate_maximal_faces` in `boolean/reduce.rs`, walks
  EDGES: two distinct parent faces with the same surface key and a
  planar surface refuse `NonMaximalFaces`, and otherwise the planes
  are compared with `declared: false`. Edge-keyed; no loop, no
  vertex index, no seam.
- **The merge ladder** (`crates/topo/src/merge_faces.rs`) merges
  adjacent faces on the same surface KEY, the same `GeomSource`, or
  a declared pair verified by `oriented_plane_eq`. The numeric rung
  is retired: coincidence is never inferred from values. Face-
  adjacency keyed, seam-blind.
- **The loop start is an ORIGIN OF INDEXING downstream and nothing
  else.** `sweep/src/loft.rs` raises a strut for every vertex
  `j in 0..n` and re-describes every one of them identically in its
  phase 6; `profile/src/validate.rs`'s `judge_joints` walks joints
  with `prev = (joint + n − 1) % n`. Both `loft.rs` and
  `extrude.rs` DO carry an `if j == 0` arm, and it is worth being
  exact about it: each remembers the first wall's top half-edge
  (`first_top`) so the ring can be closed back onto it at the end.
  That is ring-wrap bookkeeping — the wall built at `j = 0` is
  identical in kind to every other, its strut is described by the
  same code, and nothing branches on the profile's seam. The
  adjacency a seam subdivision creates is walls `n−1 / 0` — the
  same modular adjacency an interior subdivision creates at
  `j / j+1`.
- **BOOL-8 already crossed whatever there was to cross**, in the
  interior: since the `line(len)` ruling the algebra has emitted
  loops whose consecutive segments share a carrier, so every
  downstream consequence of a subdivided side already exists. The
  seam adds no new KIND of adjacency, only the same one at the index
  pair the ring wraps at.

A forward observation from that reading, filed as issue 1568 and
not acted on here: a subdivided side lowers to two coplanar walls,
which the merge ladder will merge only on a structural or declared
rung, so whether the sweep lowering gives them one surface key or
one `GeomSource` is a live question BOOL-8's ruling opened. It is
seam-independent and belongs to sweep/topo.

**The token classifies the JOINT, and there is ONE of them** (RULED —
Ev, in-chat, 2026-09-02, closing the question this entry put and
superseding the two-token reading of the day before). Every zero-turn
joint is a declared tangent joint, so `Start.arrives_tangent()` is the
whole arrival vocabulary and every closing verb takes it: what is
declared is the JOINT, not the shape of the leg reaching it, so a
STRAIGHT leg declares a tangent seam exactly as a closing ARC does. The
seam is a joint like any other — mid-side or not is not a property the
kernel asks about.

**And nothing at the seam consults the carriers.** The only entry-side
datum the check reads is `Start`'s own direction — exactly what the
interior junction check reads of a directed point. It does not ask
whether the two sides ride one carrier, because identity is a fact about
CARRIERS and tangency a fact about DIRECTIONS. Two readings of this
entry are withdrawn by that: the "subdivision vs G1" split, and the
sentence "a straight leg's arrival direction IS its own direction",
which conflated the leg's shape with the joint's class. So is the pair
of authoring-layer refusals the first of those produced — a straight
arrival onto an arc first side, and a cocircular declared tangent
arrival — both of which are now simply declared tangent joints and
validate.

The continuation verbs declare the zero-turn joints they mint for the
same reason (`line(len)` off a directed point, `continue_to`, the
post-fillet extension): the departure IS the incoming ray, so the joint
is tangent by construction, and declaration by construction is what
`.tangent()` already was. §4 item 4 carries the rule and the history of
the reading it replaced.

**What is still refused at the LATTICE.** An UNDECLARED zero-turn
seam (`SeamTangent`), from every closing verb — the arc closers
classify the seam under the seam's own name now, not the
departure's; a DECLARED arrival the direction check refuses
(`SeamArrivalOffDirection`); a declared arrival that REVERSES the
entry's outgoing direction, which is a cusp and says so
(`JunctionCusp`); and a declared arrival whose closing leg has no
LEVER — an arm below ε, where any arriving direction would satisfy
the declaration inside the band — which refuses
`SeamArrivalLeverTooShort` rather than being left to the data
gate's `DegenerateSegment`. Every one of these is a fact about the
ARRIVING leg or about `Start`'s own bits; none reads a carrier. A circular arc cannot generically carry a
tangency at both ends, so a closing arc asked for both gets the
check's refusal with the seam FILLET named as the spelling that
constructs them.

**Which closing verbs take the declaration**: `line_to`,
`continue_to`, `tangent_arc_to` and `arc_to(Bulge { … })` — all of
them, because the token names the joint rather than the leg. `arc_to`'s
`Via` and `Center` modes are the one gap: they fix an end tangent
like `Bulge` does and the same tokens would serve them, and their
arms stay lattice violations until a unit takes them (issue 1579).
The construct-from-arrival closer — the arc through the departure
point and `Start` whose END tangent is `Start.dir`, which would
spare an author solving for a departure angle — is also still
unbuilt (issue 1578).

## 7. Explicitly out of scope

Constraint-solver interactions (fillets/directors are closed
forms, never iterative); 3-D paths — LIBRARY-DESIGN LQ3(a)
ratifies the landing site as this layer, an open-chain vocabulary,
and nothing is built; spline legs as junction-vocabulary
extensions (they join with their own continuity story when
profiles grow them — PROFILES-V2 VQ7).
