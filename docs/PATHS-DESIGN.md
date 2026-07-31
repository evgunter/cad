# PATHS-DESIGN: the PartialPath authoring algebra (S5)

Status: **DRAFT round 13, for Evan's sign-off** (design-conversation
PR #124; implementation is NOT scheduled — banked for the v2
profiles-as-programs work per #104; the ratified doc is the
deliverable). Designed across twelve review rounds with Evan
(2026-07-27/29, #104 + the #124 threads); the round-by-round trail
lives in #124 and the M5 log — this document states only the
resulting design.

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
  binds 2 + 2 — exactly determined **up to the branch choice**
  (#101's `LoopBuilder::fillet` closed form at a virtual corner;
  see the amendment below). Parallel/non-intersecting carriers,
  or an intersection behind the ray start, refuse typed
  `NoCornerForFillet`.
- **Branch selection (amended 2026-07-30; Evan's in-chat S8
  ruling, twice refined the same day — the ruling is the
  sign-off)**: "exactly determined" holds only up to WHICH of the
  two carrier-intersection roots is meant. With straight legs the
  ray start disambiguates, so #101 never met the second root; arc
  carriers can put both roots on the legs — the recorded
  divergence 2, formerly in `AmbiguousFilletBranch`'s rustdoc,
  now RESOLVED here and the variant retired. `.fillet(r)`
  **selects the tangent circle nearest the authored corner**
  among candidates surviving the corner-side extent
  classification: strict `<` on total tangent setback (the sum of
  the two legs' arc-length setbacks), an exact tie falling to
  strict `<` on the incoming leg's setback alone, and identical
  per-leg setback pairs falling to enumeration order (the v1
  rule, rationale and reachability analysis live on
  `sugar.rs::nearest_candidate`). No escalation and no error:
  both survivors are valid tangent fillets of the authored legs,
  so an ε-scale pick asserts nothing about geometric truth, and
  below ε_input the author cannot have meant a distinguishable
  preference (D4 ¶1); the far circle stays deliberately
  authorable as the NEAR fillet of the other carrier
  intersection. Equivariance
  (`memories/equivariance-principle.md`): the first two rungs
  compare arc lengths — isometry-invariant, so the selection
  commutes with rigid motions and reflections in ℝ; the
  enumeration-order rung is the kernel's first knowingly-designed
  non-equivariant residual, reachable only where a
  candidate-swapping symmetry makes an equivariant pick
  impossible. (The cusp variant split — divergence 3, on
  `FilletCornerAlreadyTangent` — stays open: not ruled.)

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
| **TIER 1 — SUGAR** (one call each; expands to core; adds no semantics) | | |
| `line_to(p)` | Point → Point (also from arrivals) | `.angle(toward p).line(dist)` |
| `arc_to(p, bulge)` | Point → Point | direction from chord + bulge (M2 convention) |
| `tangent_arc_to(p)` | Directed → Point | the unique tangent arc |
| `nurbs_reversed(curve)` / `nurbs_mirrored(curve)` | Directed → Point | structural variants of rigid placement |
| `.turn(δ)` | directed point → Directed | `.angle(incoming + δ)`; `turn(0)` refuses → `.tangent()`; `turn(±π)` hits the reverse class |

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
verify layer as today.

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

Open: **PQ4 — mid-carrier seams.** The v1 rule (seam at a junction
or fillet only) forbids closing a loop mid-side; the M2
closed-carrier split precedent suggests a conventional-split
relaxation. Recommendation: keep the v1 rule; revisit only with a
concrete need, since the relaxation touches the same-carrier
discipline.

## 7. Explicitly out of scope

Implementation (banked for v2 profiles-as-programs, #104);
persistence changes (the lowering targets the existing form:
segments + tangent_joints flags); constraint-solver interactions
(fillets/directors are closed forms, never iterative); 3-D paths;
spline legs as junction-vocabulary extensions (they join with
their own continuity story when profiles grow them); arc-arrival
fillets (additive, with a use case).
