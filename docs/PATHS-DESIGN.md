# PATHS-DESIGN: the PartialPath authoring algebra (S5)

Status: **RATIFIED** (design-conversation PR #124, signed off and
merged 2026-07-29; the ratified doc was the deliverable of that
conversation). **IMPLEMENTED** as LIB unit U2 (`crates/profile/`);
`docs/LIB-U2-SPEC.md` and `docs/LIB-LOG.md` are the implementation
record. Designed across twelve review rounds with Evan
(2026-07-27/29, #104 + the #124 threads); the round-by-round trail
lives in #124 and the M5 log — this document states only the
resulting design.

Harmonization constraint (ratified context): #101's
declared-tangency discipline (flags verified-never-trusted,
`UndeclaredTangency`/`TangencyContradicted`, fillet fit gating,
same-carrier-is-identity) landed at #109/#112 and is the layer this
algebra lowers to. **End state (the #104 recorded v2 commitment,
affirmed here per Evan's round-13 note): the algebra IS the core
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
- **NURBS legs CAN close** (round 13, Evan's observation that
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

- Sharp seam: `line_to(Start)` / `arc_to(Start, bulge)` — an
  ordinary leg targeting Start; the seam's junction check runs
  with both directions known.
- Tangent seam: `.tangent().tangent_arc_to(Start)`. (A tangent
  LINE close is overdetermined — direction inherited AND through
  Start — and refuses ALWAYS, exact collinearity included: a ray
  hitting an independently-authored point is a VALUE coincidence,
  and the ratified ladder never infers from values — geometry
  that works by luck is refused, not blessed. The refusal names
  the two structural spellings: close with the tangent ARC
  instead, or rotate the loop's authoring origin — the loop is
  cyclic, so the straight run can always be authored forward as
  side 1 and the arc becomes the closer.)
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
## Evan, merged #419; implemented by LIB-RESPELL, which re-spelled
## §2/§2a/§3 to this surface and compressed §2b to its historical
## note)

**THE AXIOM (leads by design — Evan, round 11): every verb can
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

**The axiom is ENFORCED BY CONSTRUCTION (Evan, round 12 — not
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
Evan's round-13 push toward full unification): the surface and
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
Spelling freedom (round 14, Evan's trait suggestion): the
REQUIREMENT is the invariant — every transition declared exactly
once, all projections (typed method, driver arm, Step variant,
tag) mechanically derived, drift unwritable. TWO spellings
satisfy it: (a) the table-macro generating all four artifacts;
(b) rows as ordinary trait impls (`impl Apply<Verb> for State`,
one per row, calling the kernel fn — rustdoc-visible, consistent
with the ArcSpecFor admissibility impls) plus a SLIM macro for
only the enum-side projections, which Rust cannot derive from
impls (no reflection — without that step the enum match is
hand-written and the drift point quietly returns). RULED (Evan, round 15): **lean (a), the
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
  (Evan's round-8 observation, vindicating his wrap-the-args
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
**RULED (Evan, round 9): the ENTIRE family ships in stage 1 —
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

**Round 10 (Evan): `FilletCarrierUnsupported` RETIRES.** The
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
### 2026-08-16; Evan's in-chat ruling, ratification delegated
### on a clean blast-radius census)

The re-spell unit shipped a fifth tip state, `OnArc` (an
interior arc arrival's tip, its carrier run to the anchor left
un-emitted for the NEXT fused verb to trim). Evan's ruling: the
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
| `continue_to(target)` | directed point → directed point; `Start` → complete loop | the DECLARED point-target continuation: the same leg `line(len)` emits, its extent said as an authored POINT. The declaration is the verb, so nothing is inferred from the target's position — the kernel CHECKS the target lies on the departing point's ray, within ε_input, metered as the target's own lateral displacement (no lever: the datum is a point), and refuses `ContinuationTargetOffRay` past the band. The emitted vertex IS the authored target (§4 item 3), never its projection. `Start` is the structural CLOSER: it mints no vertex — the entry is already one — and runs the SEAM check unchanged, so PQ4 still wants a corner there, refusing `SeamTangent` when it is not one. The closer classifies no departure junction at all (there is no authored direction), and where a closing leg DOES have one it refuses `JunctionTangent` like any other verb. |
| `nurbs_in_place(len1, …)` / `nurbs(curve)` | Directed → Point | legs; the NURBS pair awaits the segment vocabulary (VQ7) |
| `arc_to(spec)` | Point → Point (Bulge/Via/Center); Directed → Point (Sweep/ArcLen) | **§2c** — the sharp arc leg over the `ArcData` family; admissibility = the state-keyed trait matrix; `p: Start` closes |
| `fillet(r)` | Directed \| leg end → Open | line incoming (ray extension off a leg end), line arrival |
| `fillet_arc(r, spec)` | Directed \| leg end → per spec | line incoming, ARC arrival (see arrival rows below) |
| `arc_fillet(spec, r)` | Entry \| Point \| Directed → Open | fused arc incoming (arc extension from a directed point), line arrival |
| `arc_fillet_arc(spec, r, spec₂)` | as `arc_fillet` → per spec₂ | fused arc incoming, arc arrival |
| arrival `Center{c, w, p}` | (open fillet) → directed Point; `p: Start` → complete loop | complete at the verb; interior `p` is a HARD anchor (run emitted, ordinary directed point); `Start` keeps the entry vertex |
| arrival `Radius{r, side}` | (open fillet) → builder → directed Point | centre DERIVED from the directed anchor the binders supply |
| arrival `Via{q, p}` | (open fillet) → builder → directed Point; `p: Start` closes | anchor in the spec; one director pending |
| `Start` | directed-point VALUE | targeting it closes, structurally |
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
entry rule); a NURBS leg targeting `Start`; every INADMISSIBLE
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
`fillet_arc(r, Center { c, winding, p: Start })`); the
overdetermined tangent-line close; `TangencyContradicted` from the
verify layer as today. From §2a and the spec family:
`NonpositiveCircleRadius`; `ZeroDirection`; `ArcViaCollinear`;
`DegenerateArcChord`; `DegenerateArcSpec` (a zero bulge, a
non-positive sweep/arc-length); `ArcCenterNotEquidistant`;
`DegenerateArcCenter`; `FarEndAnchorWithoutFillet`;
`CircleSplitCount`; `ArcContinueNeedsArcCarrier` and
`ArcContinueOffCarrier` (no incoming arc carrier to continue; an
authored target off it — authored points never re-project).
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
4. **Same-carrier junctions refuse when DECLARED** — `.tangent()`
   onto the incoming carrier is identity, not tangency (#101's
   rule). The structural continuations — `line(len)` off a directed
   point, the post-fillet extension — are not junctions at all: the
   departure is the point's own tangent by construction, so nothing
   is checked and nothing is declared.

**RULED (#433 — Evan, in-chat, 2026-09-01, with a second-round
extension): the lattice and `validate` AGREE.** A straight run
subdivided at an interior vertex is well formed as DATA
(`validate`, unchanged: it is what STEP import and raw authored
loops routinely produce, and nothing there claims tangency) and it
is expressible STRUCTURALLY in the algebra — `line(len)` off a
directed point, chained, mints subdivision vertices on the one
carrier the binding bits already determine (item 4 above; the §3
row). The two doors were never measuring different things about
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
**BOOL-9** (resequenced behind BOOL-11). **The LATTICE half has now
landed in full** — the interior continuation (BOOL-8) and the declared
point-target form with its structural closer (BOOL-11); the RAW DOOR
remains (BOOL-9), and #433 closes when it lands.

**The seam, measured here and RULED (third round, Evan, in-chat,
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

**That leaves a question this document does not answer, and it is
Evan's.** BOOL-8 already made a side legal with two authored vertices
on one carrier, so "one authored side, one carrier" is not what PQ4 is
protecting any more. Whether a DECLARED subdivision vertex is an
admissible seam — the loop cut where the author said the carrier
continues, rather than mid-segment where nobody said anything — is the
question the lily family raises and this unit did not take: the ruling
said PQ4 stands unchanged, and it stands. Until it is asked and
answered, an outline whose corner set moves between its own sections is
authored as loop DATA, and the lily demo says so at its own site.

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

**PQ4 — mid-carrier seams: DECIDED (Evan, in-session,
2026-08-01), as recommended.** The v1 rule stands: a closed
loop's seam sits at a junction or fillet only; closing mid-side
is refused. The M2 closed-carrier conventional-split precedent
was considered and declined — the relaxation touches the
same-carrier discipline (one authored side = one carrier), which
germ matching and the merge ladders lean on. Revisit only with a
concrete authoring need, as a revision to this section.

> **A concrete authoring need is now on the table (§4, BOOL-11, open —
> Evan-gated).** The declared closer made a seam at a CORNER sufficient,
> which is what closed the departure half of the seam wall; it did not
> reach the lily leaf family, because that family's two sections put
> their corners at disjoint stations (tips vs shoulders) while a loft
> pins one rotation for all sections, so one section always seams at a
> subdivision vertex. §4 asks whether a DECLARED subdivision vertex — the
> loop cut where the author said the carrier continues — is an admissible
> seam, given that BOOL-8 already made a side legal with two authored
> vertices on one carrier, which is the "one authored side = one carrier"
> premise this entry rests on. The question is filed there, not answered;
> this pointer exists so a reader of the register finds it, per this
> entry's own "as a revision to this section".

## 7. Explicitly out of scope

Constraint-solver interactions (fillets/directors are closed
forms, never iterative); 3-D paths — LIBRARY-DESIGN LQ3(a)
ratifies the landing site as this layer, an open-chain vocabulary,
and nothing is built; spline legs as junction-vocabulary
extensions (they join with their own continuity story when
profiles grow them — PROFILES-V2 VQ7).
