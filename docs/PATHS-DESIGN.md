# PATHS-DESIGN: the PartialPath authoring algebra (S5, design doc)

Status: **DRAFT round 6c, for Evan's sign-off** (design-conversation
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
  a fully-filleted loop `close_fillet(r)` pairs with the opening
  exactly as interior fillets pair with their preceding sides:
  the loop is a cyclic sequence of [corner, side-bindings]
  units. A leading `.fillet` on the entry Open stays ill-typed —
  fillet consumes a `Directed` departure, and the entry has
  none.

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
anchor — a real on-path point on the side).

**Legs (core).** Direction-consuming only, from `Directed`:
`line(len)`; `arc(…)` forms (the unique tangent arc to a target
point; explicit-sweep arcs at PR-spec time). A leg terminates at
a bound position → `Point`. No leg ever departs a half-bound
tip.

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
`close_fillet` likewise) — one leg in the lowering, so no
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

**Composition.** Paths are values; `p1.then(p2)` concatenates
with the seam typed by the meeting end states: `Point`↔`Point` =
Sharp (checked); a Directed end into a direction-consuming first
leg = tangent handoff; a Directed end may fillet onto p2 where
p2's start supplies the arrival. Associativity holds because the
seam consumes only the two ends' typed states — flat chains,
variables, and nesting produce the identical value (the round-2/3
requirement, preserved through the retype).

**Closure.** `close()` (Sharp seam, checked), `close_tangent()`
(declared handoff last→first), `close_fillet(r)` (fillet between
the final carrier and the START's directed departure — why
`Open.at(p).angle(θ)`, anchoring side 1 at a real path point, is
the natural opening of an all-filleted loop). v1 rule: the seam
sits at a junction or fillet, never mid-carrier (a mid-carrier
seam = two collinear legs across the seam — the same-carrier/
collinear rules refuse it; PQ4 records the possible relaxation).

## 3. Surface vocabulary and worked examples

| Surface form | Lattice transition | Core or sugar |
|---|---|---|
| `Open` | → Open | core (the entry — and every fillet arrival) |
| `.at(p)` | Open → Point; Angle → Directed | core (position binder; replaces start/through) |
| `.angle(θ)` | Point → Directed; Open → Angle | core (angle binder) |
| `.tangent()` | directed point → Directed (inherit + declared) | core; ill-typed on plain points (nothing to inherit) |
| `line(len)` / `arc(…)` | Directed → Point | core legs |
| `.fillet(r)` | Directed → Open | core (the only corner primitive) |
| `fillet(r, dd)` | Directed → Point | sugar: `.fillet(r).at(dd)` |
| `line_to(p)` | Point → Point (also from a fillet-arrival Point) | sugar: `.angle(toward p).line(dist)` |
| `arc_to(p, bulge)` | Point → Point | sugar (direction from chord + bulge) |
| `arc_to(p)` | Directed → Point | sugar for the unique tangent arc |
| `p1.then(p2)` | seam from the two tips' states | core, associative |
| `close()` / `close_tangent()` / `close_fillet(r)` | seam | same typing at last→first |

All-rounded square, fully determined (4 anchors + 4 directions,
sides read as anchor+direction pairs; every mᵢ a real on-path
point, e.g. side midpoints; sugar form shown, core form spelled
once):

```text
Open.at(m1).angle(east)
    .fillet(r, m2).angle(north)        // ≡ .fillet(r).at(m2).angle(north)
    .fillet(r, m3).angle(west)
    .fillet(r, m4).angle(south)
    .close_fillet(r)
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

Evan's original tangent shape:

```text
Open.at(a).angle(d)
    .line(len)
    .tangent().arc_to(b)
    .angle(θ).fillet(r, dd).angle(θ2)
    .line(len2)
    .arc_to(a, bulge)
    .close()
```

**The anchor fit check** (invariant 3 made operational): an
arrival anchor must land on the TRIMMED extent of its side — an
anchor the fillet trim would consume refuses typed
(`AnchorOutsideTrimmedExtent`; #101's `TangentJointOutOfRange`
fit-gating generalized). Same check for `start`'s point under
`close_fillet`.

Refusals/ill-typedness (compile-time where the lattice decides,
typed errors where geometry does): double director; `fillet` on
any non-`Directed` tip (the type-level face of "you cannot
fillet an authored corner away"); `line(len)` from a non-
`Directed` tip; a leg or `close()` from a half-bound tip
(point-sugar excepted — it supplies the missing bit);
`.tangent()` on a plain point (nothing to inherit — the former
circularity rule, now structural); leading `.fillet` (a corner
needs a preceding carrier); `NoCornerForFillet`;
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
   verify-layer door for RAW-authored declared flags (#101),
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
with both carriers already fixed when reached; `close_*` resolves
the seam against the start's recorded state. Round 3's
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

**PQ1 — Direction/tangency vocabulary extent for v1** (updated;
the earlier TangentArc question dissolved into the fillet call):
- `TangentAt(p)`: tangency with a user-pinned contact point —
  consumes leg DOFs to force contact at `p` (arc tangent to a
  line AT a marked point ⇒ center on the normal at p). How an
  author pins a tangency to a datum; cost: a second closed form
  per leg-pair family + a new overdetermination refusal class.
- `Smooth` (G2): direction AND signed curvature agree. Nearly
  vacuous for line/arc legs (line–line G2 = collinear ⇒
  same-carrier; arc–arc G2 = same carrier; line–arc G2 = infinite
  radius); real only once spline legs exist, and D2 keeps G2
  joins in the conventional-`MappedCurve` regime deliberately.
Recommendation: ship §3's vocabulary; `TangentAt` is the
plausible v1.1; `Smooth` waits for spline legs on D2's grounds.

**PQ2 — Mixed authoring** (shape from round 3, restated): raw
vertex+bulge chains embed as sub-paths via an explicit
`lift(chain)` — interior junctions keep today's #101 semantics;
boundary seams are end-state-typed like any `then`. Ad-hoc
per-segment interleaving inside one sub-path stays refused.
Recommendation: as stated.

**PQ3 — (dissolved in round 4)** — the elaboration-order cut
went with the anchoring discipline (§5); recorded so the round-3
trail stays legible.

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
