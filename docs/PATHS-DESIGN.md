# PATHS-DESIGN: the PartialPath authoring algebra (S5)

Status: **RATIFIED** (design-conversation PR #124, signed off and
merged 2026-07-29; the ratified doc was the deliverable of that
conversation). **Implementation is UNSCHEDULED as a sequencing
choice, not a dependency** — the algebra as specified in §1 is a
generator-layer surface (D8) that lowers to the existing v1
document form (segments + declared flags) with no kernel or schema
change, so it is implementable whenever it is scheduled; nothing
technical gates it. Its natural slot is an early unit of the
usable-as-library program (DESIGN.md, Beyond the kernel), where
path authoring is the user-facing surface. The separate, later
decision is the v2 profiles-as-programs REPRESENTATION switch
recorded in #104 (the program becomes the profile's definition and
derived segments become caches) — that switch wants the algebra
implemented first, not the other way around. *(Status wording
clarified 2026-08-06 with Evan; the earlier phrasing read as if
implementation waited on the v2 work.)* Designed across twelve
review rounds with Evan (2026-07-27/29, #104 + the #124 threads);
the round-by-round trail lives in #124 and the M5 log — this
document states only the resulting design.

Harmonization constraint (ratified context): #101's
declared-tangency discipline (flags verified-never-trusted,
`UndeclaredTangency`/`TangencyContradicted`, fillet fit gating,
same-carrier-is-identity) landed at #109/#112 and is the layer this
algebra LOWERS to today. **End state (the #104 recorded v2
commitment, affirmed here per Evan's round-13 note): the algebra
IS the intended core representation of paths** — the program is
the profile's definition and derived segments are caches/
provenance, exactly as Q8 definitional surfaces work (the
constructing function is the surface). Until v2, the algebra is a
generator surface lowering to the v1 form; v1's declared flags
are what make the eventual mechanical LIFT of a v1 document into
an algebra program well-defined (declared junctions become
`.tangent()` calls, fillet-authored arcs become `.fillet(r)` —
each flag pins which constructor the lift chooses).

## 1. What this is

A typed authoring algebra for profile loops in which **accidental
tangency is unrepresentable, intended tangency is exact by
construction, and every authored point lies on the final path,
authored once**. It is a generator-layer surface (D8); it lowers to
what exists — explicit segments + declared tangency flags, verified
at build by the same junction predicates. No kernel or document
semantics change.

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
- **Arc legs** — one underlying ArcLeg carrier with three binding
  modes, surfaced as distinct verbs (none is sugar for another;
  each binds a different set): {endpoints + bulge} via the
  `arc_to(p, bulge)` sugar; {tangent-to-prev + endpoint} via
  `tangent_arc_to(p)`; {tangent-both + r} = the fillet (§ below),
  which alone carries the neighbor-trimming insertion. The
  lowering may share one ArcLeg representation with a mode tag.
- **NURBS legs** — rigid authored data (clamped, w > 0, the PR 3
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
- **Fillet carriers are line/arc in v1**: a corner fillet tangent
  to a NURBS carrier has no closed form (an iterative solve — and
  this algebra is solver-free); `fillet` adjacent to a NURBS leg
  refuses typed `FilletCarrierUnsupported`.
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

### Fillet

**`.fillet(r)`** consumes the incoming `Directed` (the departure
ray) and opens the arrival side `Open`, bound in either order
(`.at(dd).angle(θ2)` / `.angle(θ2).at(dd)` / `.to(dp)`). Once the
arrival is Directed, the r-arc tangent to both carriers is
inserted at their implicit virtual corner, trimming both; the tip
continues Directed on the open arrival side, and subsequent
direction-consuming forms terminate or continue THAT SAME LEG
(`.line(len)` ends it past the anchor; another `.fillet` runs it
into the next trim) — one leg in the lowering, so no
collinear-split/same-carrier hazard.

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

The primitive offers no control over the split, deliberately. A demo
that needs a particular split (the tour's boss wants three 120° arcs so
a boolean can cross a three-face rim seam) is asking for a specific
lowering, which is a raw-chain question, not an authoring one.

**Refusals**: `r` not definitely positive (`NonpositiveCircleRadius`),
through the same funnel as the other sign gates.

**Composition**: a circle is one loop among others — profiles mix
circle loops and chain loops freely. §6's mixed-authoring rule is read
at LOOP granularity, as it always was: no loop is half raw.

### 2. `arc_via(via, end)` — the arc through a point

**Consumes** a positioned tip, a through-point, and an endpoint.
**Determines** the arc through those three points. A free arc: the
junction semantics are `arc_to`'s exactly — on a directed point the §4
item 1 check runs against the arc's start tangent; `arc_via(v, Start)`
is a sharp arc seam; on a fillet arrival it refuses
`ArcCarrierSpelling`, naming `.at_on`/`.to_on` — an arc arrival binds
its CARRIER, not an arc leg from an already-bound arrival point (§2b).

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

### 3. `arc_center(center, end, winding)` — the arc about a centre

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

## 2b. G2: arc-carrier fillets, 2026-08-08 — the constructor register

W4, the wall §2a explicitly deferred: the rocker's arc-carrier
fillets, and with them §7's "banked" arc-arrival fillets. Nothing in
§2 or §2a is revised. The lattice, the entry rule, the seam rule, the
fillet's DOF count and PQ4 all stand; this is one more addition to
the surface, and it obeys §2a's two exactness contracts unchanged.

### The gap, precisely

§2's fillet says the corner is never authored — it is the two
carriers' intersection. Where both carriers are straight there is
exactly one intersection and the corner is a division. Where one is a
CIRCLE there are 0, 1 or 2, and the algebra had no way to bind a
circular carrier at all: an arrival was a point plus a direction, and
a direction names a line. So arc sides were refused
(`ArcArrivalFillet`, `SeamFilletOntoArc`), and the rocker's five
arc-carrier fillets stayed hand-authored.

### 6. `at_on(p, centre, winding)` — the carrier-bound anchor

**Consumes** an `Open` (the entry token, or a fillet's freshly opened
arrival) plus a point, a centre and a structural winding.
**Determines** BOTH lattice bits at once — position `p`, and the
direction as that carrier's tangent at `p` in the travel sense
`winding` names — and records the carrier on the tip.

The two bits are one authored act because on a circle they are not
independent: the anchor and the centre already fix the tangent up to
travel sense, and `winding` is that sense, structural exactly as in
`arc_center` (§2a item 3) and never a value compared against. An
author made to type the tangent angle separately would be typing a
number nobody measured — §2a item 4's W5 argument, one level over —
and it would be ulp-dirty besides (contract 2).

On a fillet arrival this resolves the fillet EAGERLY, as every
arrival binder does; `p` is on the final path, authored once. The
side then runs ALONG the circle from the fillet's tangent point, past
`p`, into whatever trims it next — and that carrier run is emitted by
the trim (the next `.fillet(r)`) or by the close, exactly as a
straight arrival's run is emitted by the leg that ends it.

**Refusals**: `DegenerateArcCenter` (the anchor at the centre names no
tangent); the derived-corner refusals below. Verbs that would LEAVE
the carrier — `.line(len)`, `tangent_arc_to` — refuse
`ArcCarrierSpelling`, naming `.fillet(r)` / `.to_on` instead: they
would silently drop the carrier run.

### 7. `to_on(Start, centre, winding)` — closing on a carrier

**Consumes** a fillet's open arrival plus a centre and winding.
**Determines** the arrival side's carrier and its END: the side runs
on that circle and stops at the entry vertex. Closing, structurally.

This is the `Start`-targeting far-end form §2a item 4 left "Open,
deliberately", now with its case, and the distinction from `.to(Start)`
is structural rather than stylistic:

- **`.to(Start)`** is the SEAM FILLET. Side 1 and the arrival share
  one carrier through the entry, so the fillet arc becomes the closing
  segment and the entry vertex is RETRIMMED to the arc's end — it was
  never a junction, only where the author began writing. This is why
  it still requires a straight first side: retrimming the start of an
  arc slides that arc off its own carrier.
- **`.to_on(Start, centre, winding)`** closes on a DIFFERENT carrier.
  The entry vertex is then a genuine two-carrier junction — the rocker
  eye's sharp bottom tip is exactly this — so it is KEPT, the arrival's
  carrier run becomes the closing segment, and the seam's junction
  check runs there with both directions known (§4 item 1, sharp).

An exact trim fit (the fillet arc reaching the entry with no carrier
run left) emits no degenerate segment: the fillet arc simply IS the
arrival side and closes the loop, absorbing the anchor into the
tangent point the fit gate just classified as coincident with it —
the far-end anchor's rule (§2a item 4), unchanged.

### The derived corner, and the squared-radius rule

The algebra forbids authoring the corner, so it derives it: ray ×
circle and circle × circle, each admitting 0, 1 or 2. Both closed
forms are written on **squared** radii `R² = |anchor − centre|²` and
never round-trip `√(R²)²`.

That is a design rule, not an optimization. On the eye's circle ×
circle corner the radius form lands `0.8660254037844385` where the
author wrote `√¾ = …86` — one ulp low, and a byte-identity failure
downstream. The squared form lands it bitwise. The `sqrt`s that remain
feed classification MARGINS only (the carrier-meet gates), never an
emitted coordinate. LB4 rules this in as the design, and rules
anchor-fitting out: a site migrates only where its NATURAL,
design-stated anchors land bitwise.

Corners are then gated: **advance** on the incoming side (the corner
strictly ahead of its anchor) and **reach** on the arrival side (the
corner strictly behind its anchor). On a straight carrier these are
the shipped linear `path_corner_advance`; on a circular one they are
the same statement in the carrier's own currency — the SIGNED swept
angle levered to metres by the carrier radius, through the new funnel
predicates `path_corner_advance_arc` and `path_corner_reach_arc`.
Signed, not forward-only, so past-the-anchor classifies Negative
rather than wrapping to nearly 2π.

**Refusals**: `NoCornerForFillet` gains `CarriersDoNotMeet` (a ray
missing its circle; circles disjoint, concentric, or nested) and
`NoTangentCircle(reason)`, which carries the constructor door's own
vocabulary through instead of flattening it.
`AnchorOutsideTrimmedExtent` gains the side's CARRIER KIND, so an arc
side reports the angular margin `(extent − setback)/R` — a bare linear
setback means nothing on a circle.

### The selection ladder, lifted

Each surviving corner is fed to the ratified M5 S2 construction with
exactly the arguments a hand author would have passed had they written
that corner — which is the whole bit-identity contract — and each
yields its own surviving candidates. So the choice is over (corner,
candidate) PAIRS, and it is the S8 ladder applied to the flattened
pair list: smallest total setback, ties to the incoming setback, then
enumeration order.

**Why the dominance argument lifts**: the two levels factor. WITHIN
one corner it is S8's argument verbatim — the survivors are
mirror-symmetric about a line through the offset centres, so the near
candidate is nearer on BOTH carriers at once. ACROSS corners it is not
a dominance claim at all, and does not need to be: the advance/reach
gates discard every corner the author's two anchors do not bracket, so
each surviving pair is a valid fillet tangent to both authored
carriers, and ranking valid fillets by total setback asserts nothing
about geometric truth. That is precisely why S8 is a selection rule
and not a Q1 predicate, and the lift inherits that status unchanged —
no funnel entry, no escalation arm, no error.

### Three stated walls (LB4, LB5, and one mechanism wall)

Named here because they are the unit's evidence, not oversights. The
first two are ratified rulings; the third is an implementation
consequence reported for ratification, not an implementer's taste:

- **Line × circle derived corners are anchor-rounding-dependent**
  (0–4 ulps, measured). Where a site's natural anchors do not land
  bitwise it stays hand-authored; picking an anchor because its
  arithmetic rounds well would be fitting the authoring to the
  fixture.
- **The rocker OUTLINE stays raw.** Its mid-arc seam vertex is
  authored topology — one vertex, one lateral face after extrusion —
  that a `.to(Start)` seam retrim would eat and that §4 item 4/PQ4
  correctly refuse to reproduce as a mid-carrier junction. The EYE
  migrates: its sharp tip is the two-carrier junction `.to_on` keeps.
- **A STRAIGHT arrival off an ARC departure is refused** (typed,
  naming the carrier doors). This is a mechanism wall, not a geometric
  one: the lifted ladder reads the S8 diagnostic channel, so its
  `Bounds` bound propagates to every caller of any door that can
  resolve an arc-carrier fillet — and the ratified discipline confines
  that bound to the one boundary file, which the generic
  `.at`/`.angle`/`.to` doors are not. Consequences: a loop may have at
  most one straight side (the entry's), and the rocker's arc→line
  corners could not migrate on this route even had LB4 allowed them.
  It is unreachable from any chain authored before §2b existed. Two
  ways out exist — admit the path state machine to the allowlist as a
  second entry, or erase the capability into a function pointer fixed
  at `.fillet(r)` (which puts `.fillet` itself behind `Bounds`) — and
  both are ratification calls.

  **LB10 revisit (2026-08-11, issue #377 — RATIFIED, Evan 👍 on
  #386: ROUTE 3).** The "concrete use case" the deferral asked for has
  arrived twice: audit gap G12 (rocker unauthorable from Python)
  and Evan's ruling that LoopBuilder retires as an authoring
  surface. The 2026-08-11 investigation established the geometry
  already exists (`arc_fillet::resolve` handles a
  `SideCarrier::Ray` arrival; the replay driver already inherits
  `ArcCarrierScalar`) — only the typestate door is missing. Routes
  on the table:
  1. *Allowlist the state machine* (the menu's first): no new
     vocabulary, but `Decide + Bounds` lands on the generic
     `.at`/`.angle`/`.to`/`.line` doors — a small diff whose bound
     propagates to every PATHS caller, which is exactly what the
     LB3 confinement discipline exists to prevent. NOT
     recommended.
  2. *Function-pointer erasure* (the menu's second): `.fillet`
     itself moves behind `Bounds`; contained, moderate refactor,
     no new verb.
  3. *A distinctly-named straight-arrival binder* (NEW — not in
     the ratified menu; proposed by the investigation and
     RECOMMENDED): a sibling of `at_on`/`to_on` living in
     `arc_fillet.rs`, mirroring their shape (~60–100 lines),
     carrying the compound bound exactly where the §2b register
     already confines it. One new verb, zero propagation —
     the same pattern that closed G2's carrier anchors.
  Route 3 is recommended for consistency with the §2b register's
  own precedent; it requires ratification precisely because it
  extends the recorded menu.

## 2c. The fillet-family redesign (2026-08-12 conversation with
## Evan — PROPOSED, awaiting sign-off; supersedes the §2b
## compound-verb register when ratified and implemented)

Driven by Evan's observation that the compound doors (`at_on`,
`to_on`, `at_toward`) exist to compensate for the types not
knowing the carrier, his follow-ups refining the arrival
specification, and his closing argument that settled the shape:
§2's own Legs table DEFINES the fillet as the binding mode
{tangent-both + r} — the fillet is the operation that owns both
carriers, so the resolution belongs entirely inside the fillet
clause. Four pieces:

1. **The dependency invariant, stated on the directed point
   (Evan's rounds 3-4): a directed point is a 2-JET — position,
   tangent, signed curvature κ — carried as read-only intrinsic
   data; EVERY operation depends only on the directed point
   before it plus its own authored arguments.** Uniform, fillet
   included: the carrier is DERIVED from the jet (κ = 0 is a
   line; κ ≠ 0 gives the osculating circle, centre = p + n̂/κ —
   for the v1 line/arc carriers the osculating data IS the
   carrier), so the fillet's tangency-with-trim consumes only
   its input value. Legs need the 1-jet; fillet needs the
   2-jet. The formulation also makes the NURBS refusal
   principled: on a NURBS leg the osculating circle is NOT the
   carrier, so the 2-jet genuinely underdetermines the trim —
   `FilletCarrierUnsupported` marks exactly the boundary where
   jet-derived and actual carrier part ways. §2's directed-point
   definition amends at the re-spell to carry κ. Mechanically,
   `.fillet(r)` constructs its resolver from the input jet; the
   capability obligation (`ArcCarrierScalar: Decide + Bounds`)
   sits on `.fillet` ITSELF and nowhere else. No carrier
   parameter enters any lattice type. MEASURED (2026-08-12): the
   bound is free in practice — every scalar that drives an
   authoring chain (f64, Interval, Probe) implements Bounds;
   Dual reaches profiles only by lifting lowered ProfileLoop
   data, never through the algebra. Generic-over-T callers of
   `.fillet` inherit the bound; that set is measured empty
   beyond the boundary file itself.
2. **The fillet arrival is the fillet's OWN builder type** —
   §2 already treats it as distinct ("every fillet's freshly
   opened arrival side"), and the Python lattice split it for
   its own reasons. Its binders accumulate bits and fire the
   captured resolver when complete: no bounds on them, no
   resolution outside the fillet clause.
3. **Uniform arrival binders.** `.at(p)` / `.angle(θ)` /
   `.toward(dx, dy)` in either order on the arrival builder.
   The §2b compound verbs DISSOLVE — `at_on`, `to_on`,
   `at_toward` retire at the re-spell (the resolve machinery
   is unchanged underneath).
4. **Arrival carrier = the minimal residual specification
   (RULED: radius-only for now).** Nothing further = line
   arrival (the bound directed point IS the carrier). An arc
   arrival adds exactly what the bound directed point does not
   determine: `.arc(R, side)` — signed radius or radius + side
   bit; the CENTRE IS DERIVED, never authored (Evan's DOF
   observation: a tangent circle at a directed point has one
   free length and one bit). `at_on`'s authored-centre spelling
   is retired by this; a centre-held-datum mode returns only if
   a use case earns it, via item 5.
5. **`ArcSpec` (Evan's extension, PROPOSED with staging):** a
   single value family for "ways to author an arc" —
   `radius(R, side)` now; potentially `bulge(b)` / `via(q)` /
   `center(c, w)` later — so future arrival modes are VARIANTS,
   not verbs. Stage 2 (measured, separate): §2a's three arc LEG
   verbs could collapse to `arc_to(target, spec)` over the same
   family — the variant preserves the authored-set distinctness
   that record-as-you-lower and the VQ contracts rely on. Stage
   2 touches ratified §2a text and the corpus; it proceeds only
   on its own corpus-measured spec, not as a rider.

(An earlier draft of this section proposed carrier-typed
directed tips; DROPPED per Evan's round-2 challenge — the
capture-at-fillet mechanism achieves the same confinement with
zero type surgery, and the measurement above shows the one cost
that typing would have avoided is empty.)

Sequencing: #413 (route 3 as landed) MERGES FIRST — its door is
review-verified and closes G12; this redesign re-spells the
surface on top of the same resolution machinery in a follow-up
unit, which also re-spells the program Step vocabulary
(pre-release clean break; the v8 step set is not a compatibility
surface). The §2b register text and §3 table rewrite at that
unit; until then the register remains the live surface.

## 3. Surface vocabulary

| Form | Lattice transition | Notes |
|---|---|---|
| **TIER 0 — CORE** | | |
| `Open` | → Open | the entry; every fillet arrival |
| `.at(p)` | Open → Point; Angle → Directed | position binder |
| `.angle(θ)` | Point → Directed; Open → Angle | angle binder (+ junction check on directed points) |
| `.tangent()` | directed point → Directed | inherit + declared; ill-typed on plain points |
| `.to(dp)` | Open → Directed | combined binder; `Start`, `c.start()`, `c.end()` |
| `line(len)` / arc legs / `nurbs_in_place(len1, …)` / `nurbs(curve)` | Directed → Point | legs |
| `.fillet(r)` | Directed → Open | the only corner primitive |
| `Start` | directed-point VALUE | targeting it closes, structurally |
| `.toward(dx, dy)` | Point → Directed; Open → Angle | **G1** — the exact director: same slot as `.angle`, ray stored verbatim |
| `.to(p)` on a bound arrival direction | Angle → Point | **G1** — the far-end anchor: the arrival side ENDS at its authored anchor |
| `circle(c, r)` | — → complete loop | **G1** — closed-carrier program form; a whole loop, not a chain step; authors no seam, so PQ4 is untouched |
| `.at_on(p, c, w)` | Open → Directed | **G2** — the carrier-bound anchor: binds position AND the derived carrier tangent in one act; `w` structural |
| `.to_on(Start, c, w)` | Open → complete loop | **G2** — closes on a DIFFERENT carrier through `Start`; keeps the entry vertex (contrast `.to(Start)`, which retrims it) |
| **TIER 1 — SUGAR** (one call each; expands to core; adds no semantics) | | |
| `line_to(p)` | Point → Point (also from arrivals) | `.angle(toward p).line(dist)` |
| `arc_to(p, bulge)` | Point → Point | direction from chord + bulge (M2 convention) |
| `tangent_arc_to(p)` | Directed → Point | the unique tangent arc |
| `nurbs_reversed(curve)` / `nurbs_mirrored(curve)` | Directed → Point | structural variants of rigid placement |
| `.turn(δ)` | directed point → Directed | `.angle(incoming + δ)`; `turn(0)` refuses → `.tangent()`; `turn(±π)` hits the reverse class |
| `arc_via(via, p)` | Point → Point | **G1** — the arc through three authored points; bulge derived at lowering |
| `arc_center(c, p, winding)` | Point → Point | **G1** — centre-intent arc; winding structural; equidistance checked, never repaired |

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
    .arc_to(Start, bulge)
```

**Refusals.** Compile-time, from the lattice: double director;
`fillet`/legs from non-`Directed` tips; `.tangent()` on a plain
point; leading `.fillet`/`.tangent` (§2 entry rule); a NURBS leg
targeting `Start`. Typed runtime errors, from geometry — the
lattice guarantees the authoring, never the geometry: the junction
check (§4 item 1); `NoCornerForFillet` (r too large, carriers
parallel/non-intersecting, corner behind the ray);
`AnchorOutsideTrimmedExtent` (a trim would eat an anchor — the
#101 `TangentJointOutOfRange` fit-gating generalized; also checked
for the entry point under a seam fillet);
`FilletCarrierUnsupported` (NURBS-adjacent fillets); the
overdetermined tangent-line close; `TangencyContradicted` from the
verify layer as today. From §2a: `NonpositiveCircleRadius`;
`ZeroDirection`; `ArcViaCollinear`; `DegenerateArcChord`;
`ArcCenterNotEquidistant`; `DegenerateArcCenter`;
`FarEndAnchorWithoutFillet`. From §2b: `NoCornerForFillet`'s
`CarriersDoNotMeet` and `NoTangentCircle(reason)`;
`AnchorOutsideTrimmedExtent` now carrying the side's carrier kind (the
angular margin on an arc side); `ArcCarrierSpelling`, which RETIRES
`ArcArrivalFillet` and `SeamFilletOntoArc` — those situations are no
longer out of scope, only spelled by the carrier binders, and the
refusal always names the door that does the job. Compile-time, from
§2a: `circle`'s result
is a loop, so no chain verb follows it; `.toward` is a second director
exactly as `.angle` is; the new arc modes are legs from a Point, so
they are ill-typed on a Directed tip; the far-end `.to(p)` needs the
position slot empty and the angle slot bound.

## 4. Safety invariants

1. **No junction is silently within the input tolerance of
   tangent.** On a directed point, `.angle(θ)`/`.turn(δ)`
   classifies θ against the incoming tangent and its reverse.
   Outcomes: definitely-sharp proceeds; within ε_input of the
   TANGENT direction refuses — ONE refusal, one recourse, for any
   sub-ε_input margin (D4's two-tolerance principle): "this
   junction is tangent at any precision you could care about — if
   intended, use `.tangent()`, which makes it exact by
   construction; otherwise move the geometry (or lower the
   tolerance)". The margin rides the payload as data; the message
   never forks on exactly-on vs in-band. Within ε_input of the
   REVERSE direction refuses as a cusp (reverse-tangent class):
   no declaration door exists — the kernel's material-wedge
   invariant refuses cusp wedges in any solid built from such a
   profile; the refusal names #131 (the tabled higher-level
   question) as the front door that does not exist yet.
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
4. **Same-carrier junctions refuse** exactly as #101's
   `same_carrier: true` (identity, not tangency); the post-fillet
   continuation is exempt by construction — it extends the same
   leg rather than minting a collinear neighbor.

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
unwritable. To re-verify at implementation, not an axiom. D9:
elaboration is pure f64 structure selection (C6 boundary — it
decides leg parameters, never topology); the lowered profile runs
the ordinary generic pipeline. `UnderdeterminedLeg`/
`OverdeterminedJunction` remain as elaborator backstops, expected
unreachable from the typed surface; a reachable case is a design
finding to bring back here, not a silent fix.

Representation: ONE struct — `pos: Option<PosData>`,
`ang: Option<f64>` — under type-level lattice markers
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
TABLED to #131 with cusps refused here; there is no
path-concatenation operator (builder functions instead).

**PQ4 — mid-carrier seams: DECIDED (Evan, in-session,
2026-08-01), as recommended.** The v1 rule stands: a closed
loop's seam sits at a junction or fillet only; closing mid-side
is refused. The M2 closed-carrier conventional-split precedent
was considered and declined — the relaxation touches the
same-carrier discipline (one authored side = one carrier), which
germ matching and the merge ladders lean on. Revisit only with a
concrete authoring need, as a revision to this section.

## 7. Explicitly out of scope

Implementation (banked for v2 profiles-as-programs, #104);
persistence changes (the lowering targets the existing form:
segments + tangent_joints flags); constraint-solver interactions
(fillets/directors are closed forms, never iterative); 3-D paths;
spline legs as junction-vocabulary extensions (they join with
their own continuity story when profiles grow them); arc-arrival
fillets (additive, with a use case).
