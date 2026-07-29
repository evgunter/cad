# PATHS-DESIGN: the PartialPath authoring algebra (S5, design doc)

Status: **DRAFT round 11, for Evan's sign-off** (design-conversation
PR; implementation is NOT scheduled — banked for the v2
profiles-as-programs work per #104. The ratified doc is the
deliverable). Rounds: 1 = forward-consuming vs junction-resolver
fork; 2 = Evan's inline review (resolver set collapsed to binary;
in-order authoring); 3 = coincident-corner fillet + pending
resolver; 4–5 (2026-07-28 in-session) = typed path ends, the
anchored fillet form (Evan: `.angle(θ).fillet(r, dd).angle(θ2)`),
directors as the only direction source with every point-targeting
constructor as sugar (Evan: "line_to is just sugar for
.angle(theta).line(length)"); 6 (same session, Evan's
"typestates are maybe Point, Angle, Directed — and Point always
needs its angle", confirmed reading) = **the binding lattice**:
the tip's state is WHICH OF {position, angle} it has bound —
`Point` = {position}, `Angle` = {angle}, `Directed` = {both} —
plus `Open` = {} — the entry point AND every fillet arrival; the
fillet becomes argument-minimal (`.fillet(r)`) with its arrival
side bound in either order. Supersessions from rounds 2–5 are
recorded inline.

Lineage: Evan's concept (#104) + three in-session design rounds
(2026-07-27/28). Harmonization constraint (ratified context):
#101's declared-tangency discipline (flags verified-never-trusted,
`UndeclaredTangency`/`TangencyContradicted`, fillet fit gating,
same-carrier-is-identity) landed at #109/#112 and is the layer
this algebra LOWERS to; schema v1 persists explicit geometry +
flags, and the lift to the algebra is determined by construction.

## 1. What this is

A typed authoring algebra for profile loops in which **accidental
tangency is unrepresentable, intended tangency is exact by
construction, and every authored point lies on the final path**.
The algebra is a generator-layer surface (D8); it lowers to what
exists: explicit segments + declared tangency flags, verified at
build by the same junction predicates. No kernel or document
semantics change.

## 2. The core: the binding lattice, directors, minimal fillets

**End states (round 6, Evan's lattice — the confirmed reading).**
The tip's typestate is exactly WHICH OF {position, angle} it has
bound:

- **`Point`** = {position}. Two FLAVORS, distinguished by the
  types (Evan, round 6b: ".tangent() actually consumes directed
  points, not regular ones"): a **plain point** (`Open.at(p)`,
  `.at(p)` after a fillet — position only, no incoming carrier) and a
  **directed point** (a LEG END — position + the leg's incoming
  end tangent, carried latently). Invariant, verbatim from the
  exchange: **Point always needs its angle** — the only legal
  continuation is angle-binding (a director, or sugar that
  computes one).
- **`Angle`** = {angle}. Direction bound, position pending —
  arises only on a fillet's arrival side when the angle is bound
  first. Symmetric invariant: Angle always needs its point.
- **`Directed`** = {both}. The only state legs and `.fillet(r)`
  consume.
- **`Open`** = {} — and it is BOTH the fillet's freshly opened
  arrival side AND the path's entry point (Evan, round 6c: "i
  like Open as a concept — `Open.at(p).angle(θ).line(len)
  .fillet(r).at(p2).angle(θ2)…`"). `Open` is where every side
  begins; the two binders fill it in either order. `start(p)` /
  `start_angle(θ)` / `through(p)` all dissolve into the one
  position-binder **`.at(p)`** and the angle-binder `.angle(θ)`
  applied to Open. Consequence, recorded: no side is privileged
  — the opening reads exactly like every fillet arrival, and for
  a fully-filleted loop the seam fillet pairs with the opening
  exactly as interior fillets pair with their preceding sides:
  the loop is a cyclic sequence of [corner, side-bindings]
  units. **Entry rule, generalized (round 7b, Evan's review
  question — replacing the earlier shallow "the entry has none"
  justification):** the entry authors the FIRST SIDE; the SEAM is
  authored ONCE, at the back of the chain (round 8: by the verb
  that targets `Start`). In the cyclic view the corner "before"
  the first side IS the seam corner, so a leading `.fillet(r)` is
  the seam fillet's content authored from the front, and a
  leading `.tangent()` is the tangent seam's —
  allowing either would mint a second spelling of the same value
  (the round-3 associativity principle), and close-side is the
  only site that can even elaborate it (both adjacent carriers
  bound there; neither at entry). Both leading forms are refused
  — and the lattice typing already enforces this uniformly
  (each needs bits the entry Open lacks); the principle is why
  that shape is right, not a third rule. Leading `.at`/`.angle`
  are fine (they bind the first side itself, either order); the
  only invariant at the entry is the ordinary one applied to
  side 1 — both bits bound (sugar may compute them) before the
  side extends.

**Directors are the only way angles enter**, and the two-flavor
split makes their typing exact (round 6b — the former
"refused-as-circular" special rule is now plain ill-typedness):
- `.tangent()` consumes a **directed point** ONLY — it re-uses
  the incoming end tangent as the departure and emits the
  DECLARED flag on lowering. On a plain point there is no
  direction to inherit: a fillet-arrival `.tangent()` is not
  refused by a rule, it does not typecheck ("fillets sit between
  defined geometry", now structural).
- `.angle(θ)` adds the angle bit wherever it is missing
  (`Point → Directed`, `Open → Angle`); on a **directed point**
  it additionally runs the junction check of θ against the
  incoming tangent — the Sharp check IS the incoming-vs-outgoing
  comparison (definite-tangent ⇒ `UndeclaredTangency`, "declaring
  is saying `.tangent()`, never guessing the angle", #101
  verbatim; in-band ⇒ `AmbiguousAtEps` per §4). On a plain point
  there is no incoming carrier and hence no check — an arrival
  side meets its fillet arc tangentially by construction, and a
  start point's junction check happens at the seam, at close.
**`.at(p)`** is the position-binder dual to `.angle` (round 6c
naming — it replaces both `start(p)` and the interim
`through(p)`): `Open → Point` (plain), `Angle → Directed` (the
anchor — a real on-path point on the side). **`.to(dp)`** (round
10) is the combined binder consuming a directed-point VALUE:
`Open → Directed` in one step — `Start` (always a full directed
point, never position-only) is its canonical argument.

**Write-once, and incoming vs outgoing are different KINDS
(round 10, Evan's invariant made explicit)**: the OUTGOING angle
is a binding slot — set at most once per side (every director
requires the slot empty; a second `.angle`/`.tangent`/`.turn` on
a Directed tip is ill-typed, the "double director" refusal). The
INCOMING direction is never a slot at all: it is read-only
intrinsic data carried by a leg end (the directed-point flavor),
consultable by `.tangent()`/`.turn(δ)`/the junction check, and
not settable by any author verb. Nothing can set "tangent and/or
angle twice on the same thing" — the first sets the slot, the
slot then does not exist to set.

**Legs (core).** Direction-consuming only, from `Directed`:
`line(len)`; `arc(…)` forms (the unique tangent arc to a target
point; explicit-sweep arcs at PR-spec time); `nurbs(curve)` —
see below. A leg terminates at a bound position → `Point`. No
leg ever departs a half-bound tip.

**NURBS legs (round 9, Evan's pre-commit check — thought through,
and the DOF story extends exactly).** A NURBS leg is RIGID
AUTHORED DATA: a `NurbsCurve2` authored in its own frame
(clamped, w > 0 — the PR 3 invariants), whose end positions and
end tangents are intrinsic (first/last control points; the
end-leg control differences). Rigid placement in 2-D has exactly
3 DOFs (translation 2 + rotation 1) — precisely what a `Directed`
tip supplies (position 2 + direction 1). So `nurbs(curve)` is an
ORDINARY direction-consuming core leg: placement translates the
curve's start to the tip position and rotates its start tangent
onto the departure direction ("rotate/translate this whole curve
so the start is tangent" is the canonical placement semantics,
not a sugar). Consequences, each falling out rather than ruled:
- After a `.tangent()` director the placement gives G1 continuity
  by construction (declared flag emitted); after `.angle(θ)` the
  Sharp check runs against the placed start tangent as at any
  junction. The leg's END is a directed point (intrinsic end
  tangent), so `.tangent()` chains onward.
- **Inline authoring (round 10, Evan: control points can BE the
  tangency)**: alongside rigid placement of pre-authored curves,
  a NURBS leg may be authored IN PLACE with its junction
  constraints absorbed into the control polygon: `P0` is implied
  (the tip position, never written), `P1` is given as a DISTANCE
  along the departure direction (its direction is the consumed
  angle bit — for clamped w > 0 curves the start tangent is the
  P0→P1 direction, so tangency/sharpness is by construction),
  and `P2…Pn` are authored in the DEPARTURE FRAME — origin at
  the tip position, +x along the departure direction (round 11,
  Evan's ask made precise and GENERAL: leg-INTERNAL shape data —
  control points, and any directional non-point leg parameters —
  is always relative to the current directed point's frame, so a
  leg's shape is placement-invariant authored data; TARGET and
  ANCHOR points — `line_to`, `.at`, `tangent_arc_to`, `Start` —
  stay ABSOLUTE, because they are the profile's on-path datums
  and pinning them to the world is their job). The end
  directed point falls out of the last two control points. Same
  DOF accounting, zero placement step — the two forms
  (rigid-place a curve value / author the polygon in place) are
  both core-legal and lower identically.
- NO scale, no deformation: the algebra places authored curves,
  never edits them (metric shape is data; scaled/stretched
  placement is authoring-time transformation of the curve, out of
  scope). `nurbs_reversed(curve)` (parameterization flip) and
  `nurbs_mirrored(curve)` (reflection across the departure line;
  curvature signs flip) are the free structural variants.
- **Fillet carriers stay line/arc in v1**: a corner fillet
  tangent to a NURBS carrier has no closed form (it is a small
  iterative solve — and this algebra is deliberately solver-free,
  §7). `fillet` adjacent to a NURBS leg refuses typed naming the
  door (`FilletCarrierUnsupported`); tangent/sharp junctions
  cover NURBS joins in v1.
- **A NURBS leg cannot target `Start`**: its placement is fully
  consumed at its departure junction, so its end lands where the
  data says — a rigid leg targeting a fixed point is
  overdetermined. Loops whose last authored leg is NURBS close
  through a connecting line/arc leg (or a seam fillet between
  line/arc carriers). Same reasoning as the tangent-line-close
  refusal; the type story is uniform.

**Point-targeting constructors are SUGAR** (round 5: "line_to is
just sugar for .angle(theta).line(length)"): `line_to(p)` =
`.angle(θ toward p).line(|p − cur|)`; `arc_to(p, bulge)`
desugars the same way (start direction from chord + bulge — the
M2 convention). The Sharp junction check rides the desugared
`.angle(…)` uniformly, sharp corners included. Sugar composes
with the lattice: `.fillet(r).at(dd).line_to(p)` = arrival
direction from dd toward p, side terminating at p. Round-1/2's
`arc_tangent_to` and `start_dir` dissolve into
`.tangent().arc_to(p)` and `Open.at(p).angle(θ)`; round-5's
`fillet(r, dd)` survives as sugar for `.fillet(r).at(dd)`.

**Fillet (round-6 form: argument-minimal).** `.fillet(r)`
consumes the incoming `Directed` (the departure ray) and opens
the arrival side `Open`; the arrival binds in EITHER order:

```text
.angle(θ).fillet(r).at(dd).angle(θ2)   // anchor-then-angle
.angle(θ).fillet(r).angle(θ2).at(dd)   // angle-then-anchor
```

Once the arrival side is `Directed` (line through `dd` along
θ2), the r-arc tangent to both carriers is inserted at their
implicit virtual corner, trimming both; the tip is then Directed
on the open arrival side — subsequent direction-consuming forms
TERMINATE OR CONTINUE that same leg (`.line(len)` ends it past
the anchor; another `.fillet(…)` runs it into the next trim;
the seam fillet likewise) — one leg in the lowering, so no
collinear-split/same-carrier hazard arises from the
continuation. Grounds, recorded:

- **The corner is never authored.** The trimmed corner exists
  only as the carrier intersection; fillet takes no corner point,
  and a bare `Point` end cannot fillet (no departure). Round 3's
  blemish (`.line_to(p2).fillet(r).arc_to(p3)` — p2 authored,
  then silently trimmed away) is unrepresentable at the type
  level.
- **Every side is anchored.** Each carrier is fixed by a real
  authored on-path point plus a direction: the incoming ray by
  the current end, the outgoing line by the arrival anchor. The
  anchor-free side between two fillets (a `line_open()` sketch
  from the round-3 chat) is UNDERDETERMINED — direction fixed,
  both extents trimmed, offset free — and cannot be written in
  this form. (The round-3 all-rounded-square sketch had exactly
  that bug: one through-point, three floating sides. Corrected
  example in §3.)
- **DOF check**: arc 5 DOFs; r binds 1; tangency to each fixed
  carrier binds 2 — exactly determined; #101's
  `LoopBuilder::fillet` closed form generalized to a virtual
  corner. Parallel/non-intersecting carriers, or an intersection
  behind the ray start, refuse typed `NoCornerForFillet`.
- Arrival vocabulary v1 = line arrivals (the anchor + a director
  fix a LINE carrier). An arc-arrival variant (anchor + director
  + radius fixing an arc carrier) is additive, added with a use
  case.
- **Arc vs fillet, at the data level (round 11, answering "is arc
  technically a sugar for fillet?")**: no — but they are siblings.
  One underlying ArcLeg carrier kind admits three BINDING MODES:
  {endpoints + bulge} (the sharp-jointed arc), {tangent-to-prev +
  endpoint} (the unique tangent arc), {tangent-both + r} (the
  fillet). Neither is sugar for the other — different binding
  sets — and fillet carries one thing plain arcs never do: the
  neighbor-trimming insertion. The lowering may share one ArcLeg
  representation with a mode tag; the surface verbs stay
  distinct.

**Composition: `then()` DROPPED (round 11, answering Evan's
"what does p1.then(p2) do?").** It concatenated two path values
with a seam typed by the meeting tips — and inspecting it under
round 8's lens shows it reintroduces exactly the defect `Start`
killed: p1's tip position and p2's start position are two
independently-authored points that must value-match at the seam.
Code reuse (repeated motifs, generator loops — the D8 layer's
real need) is covered without it by BUILDER FUNCTIONS:
`fn motif(p: PartialPath<Directed>) -> PartialPath<Directed>`
appends to the one chain, composes associatively at the language
level, and never mints a second path value to glue. One chain,
one spelling, no seams except the loop's own.

**Closure: the `Start` token (round 8, Evan's thought experiment
adopted — the `close_*` family DISSOLVES).** The entry, once
bound, is addressable: **`Start`** is a first-class **directed
point** value (side 1's position + departure direction — always
fully bound by the time the loop returns), legal wherever a
position/anchor argument goes. **Using it IS closing**, and it is
closing STRUCTURALLY:
- Sharp seam: `line_to(Start)` / `arc_to(Start, bulge)` — an
  ordinary leg targeting Start; the seam's sharp check runs with
  both directions known (last leg's arrival vs side 1's
  departure — the check §2 already deferred to the seam).
- Tangent seam: `.tangent().arc_to(Start)` — the unique tangent
  arc landing at Start, arrival direction then checked/declared
  against side 1's departure. (A tangent LINE close,
  `.tangent().line_to(Start)`, is overdetermined — direction
  inherited AND through Start — and refuses unless genuinely
  collinear; the same overdetermination any
  tangent-line-to-a-fixed-point has, but the seam is where
  authors will hit it, so the refusal text names the fix: use an
  arc, or drop the tangent.)
- Seam fillet: `.angle(θ).fillet(r, Start)` — the most
  determined fillet in the language: both carriers already bound
  (the incoming ray and side 1), nothing pending, loop closed.
- `close()` survives only as SUGAR for `line_to(Start)` if the
  word is wanted; `close_tangent`/`close_fillet` are gone.

**The latent defect this fixes, recorded**: the previous shape
(`.arc_to(a, bulge).close()` — a re-authored as coordinates)
relied on the last endpoint VALUE-matching the entry point — a
coincidence between independently-authored numbers, exactly what
the ratified coincidence ladder refuses to infer from. `Start`
makes closure declared and structural: the endpoint IS the start
point by reference, authored once (the authored-points-once
discipline completing itself).

v1 seam-placement rule unchanged: the seam sits at a junction or
fillet, never mid-carrier (a mid-carrier seam = two collinear
legs across it — same-carrier rules refuse; PQ4 records the
relaxation).

## 3. Surface vocabulary and worked examples

| Surface form | Lattice transition | Core or sugar |
|---|---|---|
| **TIER 0 — CORE** (round 9: the two tiers are explicit; every sugar names its core expansion and nothing in tier 1 adds semantics) | | |
| `Open` | → Open | the entry — and every fillet arrival |
| `.at(p)` | Open → Point; Angle → Directed | position binder |
| `.angle(θ)` | Point → Directed; Open → Angle | angle binder |
| `.tangent()` | directed point → Directed | inherit + declared; ill-typed on plain points |
| `line(len)` / `arc(…)` / `nurbs(curve)` | Directed → Point | legs (nurbs = rigid placement, direction-matching whichever director bound the tip) |
| `.fillet(r)` | Directed → Open | the only corner primitive |
| `Start` | directed-point VALUE (the bound entry — ALWAYS both bits, never position-only) | targeting it closes, structurally |
| `.to(dp)` | Open → Directed | combined binder consuming a directed-point VALUE (round 10: `fillet(r).to(Start)` is the seam fillet's honest unsugaring — not `.at(Start).angle(Start)`) |
| **TIER 1 — SUGAR** (each row = one call, expands to core, may only append/insert one leg and/or set bindings) | | |
| `line_to(p)` | Point → Point (also from a fillet-arrival Point) | `.angle(toward p).line(dist)` |
| `arc_to(p, bulge)` | Point → Point | direction from chord + bulge |
| `tangent_arc_to(p)` | Directed → Point | the unique tangent arc (round 10 rename: `arc_to` no longer means two things with different args) |
| `nurbs_reversed(curve)` | Directed → Point | `nurbs(reverse(curve))` — parameterization flip, structural |
| `nurbs_mirrored(curve)` | Directed → Point | reflection across the departure line (structural; curvature signs flip — a left-turning curve places right-turning) |
| `.turn(δ)` | directed point → Directed | `.angle(incoming + δ)` (round 10, Evan): relative director; `turn(0)` refuses → use `.tangent()`; `turn(π)` hits the reverse class (PQ5) |
| *(no `close()` — dropped, round 10: a close alias would suggest it is THE way to close; Start-targeting through ordinary verbs is the mechanism)* | | |

All-rounded square, fully determined (4 anchors + 4 directions,
sides read as anchor+direction pairs; every mᵢ a real on-path
point, e.g. side midpoints; sugar form shown, core form spelled
once):

```text
Open.at(m1).angle(east)
    .fillet(r).at(m2).angle(north)
    .fillet(r).at(m3).angle(west)
    .fillet(r).at(m4).angle(south)
    .fillet(r).to(Start)               // the seam fillet; closed
```

with the opening reading exactly like every fillet arrival
(either binder order); the seam fillet is the fourth corner,
indistinguishable from the interior three.

Mixed sharp + fillet (incl. the sugar-on-AwaitingDirection form):

```text
Open.at(p0).line_to(p1)                 // sharp corner at p1 (sugar over .angle().line())
    .angle(θ).fillet(r, m).angle(θ')  // rounded virtual corner; side anchored at m
    .line(len)                        // arrival side ends len past m
    .line_to(p2)                      // sharp corner
    .close()
```

Evan's original tangent shape (the seam leg now targets Start
structurally — `a` is authored exactly once):

```text
Open.at(a).angle(d)
    .line(len)
    .tangent().arc_to(b)
    .angle(θ).fillet(r, dd).angle(θ2)
    .line(len2)
    .arc_to(Start, bulge)
```

**The anchor fit check** (invariant 3 made operational): an
arrival anchor must land on the TRIMMED extent of its side — an
anchor the fillet trim would consume refuses typed
(`AnchorOutsideTrimmedExtent`; #101's `TangentJointOutOfRange`
fit-gating generalized). Same check for the entry point under a
seam fillet (`fillet(r, Start)`).

**The reverse class (round 10, Evan's π-off question — PQ5).**
The junction classifier gains a third direction class: an
outgoing direction within the band of incoming + π is
**reverse-tangent** (a cusp: `1 − sqrt(|x|)`'s peak; two kissing
circles with their top halves cut). Like tangency it is refused
when it arrives by VALUE (`.angle(θ)`/`.turn(δ)` landing at ≈ π
off) — but unlike tangency, v1 offers NO declaration door that
lowers: the kernel's ratified material-wedge invariant (tier 3:
wedge ∈ (0, 2π), bounded away from the ends) refuses cusp wedges
in any solid built from such a profile, so a declared-cusp
authoring form would mint profiles whose every downstream use
fails loudly. The refusal names both facts ("reverse-tangent
junction: if intended, this needs the declared-cusp door, which
does not exist yet — and solids from cusped profiles are refused
by the wedge invariant regardless"). Whether declared cusps
should become legal (profile-layer form + a kernel wedge-
invariant conversation) is PQ5 — a genuine fork, Evan's call,
out of this doc's power to decide.

**Runtime checks are not only sharpness (round 10, explicit)**:
the compile-time lattice cannot see geometry — sharpness/tangency
classification, fillet FIT (`NoCornerForFillet` when r is too
big for the corner or the carriers do not intersect;
`AnchorOutsideTrimmedExtent` when a trim eats an anchor), and
the anchor checks are all TYPED RUNTIME errors at elaboration,
exactly like #101's fit gating today. The lattice guarantees
well-formedness of the AUTHORING, never of the geometry.

Refusals/ill-typedness (compile-time where the lattice decides,
typed errors where geometry does): double director; `fillet` on
any non-`Directed` tip (the type-level face of "you cannot
fillet an authored corner away"); `line(len)` from a non-
`Directed` tip; a leg or `close()` from a half-bound tip
(point-sugar excepted — it supplies the missing bit);
`.tangent()` on a plain point (nothing to inherit — the former
circularity rule, now structural); `fillet` adjacent to a NURBS
carrier (`FilletCarrierUnsupported` — no closed form, the
solver-free line, §2 round 9); a NURBS leg targeting `Start`
(rigid placement already consumed — §2 round 9); leading `.fillet`/`.tangent`
(the seam belongs to the Start-targeting verbs at the BACK of the
chain — §2's generalized entry rule, restated for round 8: one
authoring site per seam, and only the back side can elaborate
it); `.tangent().line_to(Start)` (the overdetermined tangent-line
close — refusal text names the fix: use an arc, or drop the
tangent); `NoCornerForFillet`;
`AnchorOutsideTrimmedExtent`; `UndeclaredTangency` on an
exactly-tangent `.angle(θ)` at a directed point;
`TangencyContradicted` from the verify layer as today.

## 4. The safety invariants, restated

1. **No junction is silently near-tangent**: Sharp junctions are
   checked; the refusal is ONE method with ONE user story
   (round 7, Evan's ε-framing — superseding the round-2/3
   two-payload split): ε is "the precision we represent", Kε is
   "the least precision the user might care about" — so for ANY
   margin below the user-meaning threshold the recourse is
   uniform: **"this junction is tangent at any precision you
   could care about — if intended, use `.tangent()` (which makes
   it exact BY CONSTRUCTION); otherwise move the geometry (or
   lower ε)."** The exact margin rides the payload as data
   (diagnostics may care whether it was 0 or 3ε); the message
   and the recourse do not fork on it. Correction to the
   round-3 text, recorded honestly: the old `AmbiguousAtEps` arm
   claimed declaring tangent from the in-band case "gets
   verified and refused if false" — wrong IN THE ALGEBRA, where
   `.tangent()` is not a claim about the numbers but a
   construction (the direction is inherited exactly; the
   geometry moves by ≤ the sub-threshold margin, an intended,
   reported change — the ratified repair-shaped resolution, not
   a verification gamble). `TangencyContradicted` remains the
   verify-layer door for declared flags on RAW-AUTHORED segment
   chains — today's explicit vertex+bulge profiles and PQ2's
   `lift(chain)` sub-paths, where the flag is a claim about
   independently-typed numbers and can therefore be false —
   untouched. Declaring SHARP is still never an override —
   kernel-side F6 semantics (exactly-on vs in-band) are
   unchanged by this; the unification is user-message policy at
   the authoring layer. (The deeper reconception this surfaced —
   an `eps_input` decoupled from K·ε_precision, the D7 ε_in
   split extended to native input — is Evan's #124 inline
   comment, tracked as its own design conversation, not decided
   here.)
2. **No tangency without declaration**: tangency enters only via
   `.tangent()` or fillet construction (which lowers to declared
   trimline tangency exactly like `LoopBuilder::fillet` today);
   the lowering emits the flags — declaration by construction,
   never inference.
3. **Every authored point lies on the final path** (round 4):
   points enter only as path points (`start`, `line_to`,
   `arc_to`, arrival anchors); fillets consume only directions
   and anchors; the anchor fit check enforces the invariant
   where trims could threaten it.
4. Same-carrier junctions refuse at elaboration exactly as
   #101's `same_carrier: true` (identity, not tangency); the
   post-fillet continuation is exempt BY CONSTRUCTION because it
   extends the same leg rather than minting a collinear neighbor
   (§2).

The #101 verify layer runs UNCHANGED on the lowered output — the
algebra is upstream insurance; the flags remain the contract of
record.

## 5. Elaboration semantics

Strictly forward, single pass, seam last; every step local and
closed-form: directors bind departures; direction-consuming legs
bind from them; each fillet is the ray×line corner construction
with both carriers already fixed when reached; the seam resolves
when a verb targets `Start` (leg arrival check, tangent-arc
check-and-declare, or the fully-bound seam fillet — §2). Round 3's
`ElaborationOrderUnsupported` class is ELIMINATED by the round-4
anchoring discipline — no chain expressible in this surface needs
right-to-left propagation (recorded as a consequence to re-verify
at implementation, not an axiom). D9: elaboration is pure f64
structure selection (C6 boundary — it decides leg parameters,
never topology); the lowered profile then runs the ordinary
generic pipeline. Failure vocabulary: §3's refusals plus
`UnderdeterminedLeg`/`OverdeterminedJunction` kept as elaborator
backstops (expected unreachable from the typed surface; a
reachable case found at implementation is a design finding to
bring back here, not a silent fix).

## 5b. Implementation note: one representation, four states
(round 6d, Evan's suggestion — recorded so the lattice is known
to be cheap to implement)

Under the hood the tip is ONE struct holding literally the pair
of options — `pos: Option<PosData>`, `ang: Option<f64>` — with
the lattice enforced by type-level markers over it
(`Tip<P, A>`; Open/Point/Angle/Directed are the four
instantiations, the position marker carrying the plain-vs-
directed flavor). Binders are written ONCE, generic over the
slot they do not touch: `.angle(θ): Tip<P, NoAng> → Tip<P,
HasAng>` for any P (Evan: "functions which just set the angle
from second-place-is-None and don't care what the first place
is"), `.at(p)` dually; `.tangent()` exists only at
`Tip<HasPos<WithIncoming>, NoAng>`. Inside `.angle`, the
junction check consults the position flavor's OPTIONAL incoming
tangent at runtime (directed points compare, plain points have
nothing to compare) — one generic function, not a per-state
fork. Invariant for the implementation: fields private, binders
the only constructors — the Option-pair makes off-lattice states
representable at runtime but unreachable through the surface.

## 6. Open questions for Evan

**PQ2 — Mixed authoring: DECIDED AGAINST (Evan, round 10, on
associativity/representation-uniqueness grounds)** — no
`lift(chain)`, no mixing: a profile loop is authored EITHER in
the algebra OR as a raw vertex+bulge chain (today's format),
never both in one loop. A lifted chain would be a second spelling
of content the algebra can express, exactly the redundancy the
associativity principle exists to kill. Both authoring routes
lower to the same persisted form; the verify layer treats them
identically.

**PQ3 — (dissolved in round 4)** — the elaboration-order cut
went with the anchoring discipline (§5); recorded so the round-3
trail stays legible.

**PQ5 — Declared cusps: TABLED (Evan, round 11)** — cusps are
refused here (the reverse-tangent class, §3), and the
higher-level question (a declared-cusp door vs the kernel's
ratified material-wedge invariant) is opened as its own design
issue rather than decided in this doc.

**PQ4 — Mid-carrier seams.** The v1 rule (seam at a junction or
fillet only) forbids closing a loop mid-side. The M2
closed-carrier split precedent (full circles split ≥ 2 at the
input layer) suggests a conventional-split relaxation;
recommendation: keep the v1 rule, bank the relaxation — pure
convenience, and it touches the same-carrier discipline, which
deserves its own care.

## 7. Explicitly out of scope

Implementation (banked for v2 profiles-as-programs, #104);
persistence changes (the lowering targets the EXISTING form:
segments + tangent_joints flags); constraint-solver interactions
(M6 — fillets/directors are closed forms, never iterative); 3-D
paths; spline legs (the end-state typing is carrier-generic; they
join with their own G2 story); arc-arrival fillets (additive,
with a use case); `TangentAt`/`Smooth` (PQ1).
