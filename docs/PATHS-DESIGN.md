# PATHS-DESIGN: the PartialPath authoring algebra (S5, design doc)

Status: **DRAFT round 4, for Evan's sign-off** (design-conversation
PR; implementation is NOT scheduled — banked for the v2
profiles-as-programs work per #104. The ratified doc is the
deliverable). Rounds: 1 = forward-consuming vs junction-resolver
fork; 2 = Evan's inline review (resolver set collapsed to binary;
in-order authoring); 3 = coincident-corner fillet + pending
resolver; 4–5 (this text, 2026-07-28 in-session) = **typed path
ends**, the **fillet(r, anchor) form with a directed-awaiting
state** (Evan: `.angle(θ).fillet(r, dd).angle(θ2)`), and the
**directors-are-the-only-direction-source core** with every
point-targeting constructor as sugar (Evan: "line_to is just
sugar for .angle(theta).line(length)"), superseding parts of
rounds 2–3 as recorded inline.

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

## 2. The core: typed ends, directors, anchored fillets

**End states.** A path end (start end and growing tip) is one of
THREE types (the third is Evan's round-5 addition):

- **`Point`** — position bound, no direction commitment.
  Produced by `start(p)` and by legs that terminate at a bound
  position.
- **`Directed`** — position + departure direction. Produced by a
  *director* on a `Point` end.
- **`AwaitingDirection`** — a fillet has bound its arrival-side
  ANCHOR but the side's direction is pending; the next director
  completes it. Produced only by `.fillet(r, anchor)`.

**Directors are the only way directions enter** (the round-5
core-minimality rule): `.tangent()` (inherit the incoming leg's
end tangent — emits the DECLARED flag on lowering) and
`.angle(θ)` (explicit; a θ that happens to hit the incoming
tangent direction is `UndeclaredTangency` — declaring is saying
`.tangent()`, never guessing the angle; #101 verbatim).
`.angle(θ)` serves two states: `Point → Directed` and
`AwaitingDirection → Directed` (on the arrival side, direction
θ through the bound anchor). `.tangent()` serves only `Point`
ends — on `AwaitingDirection` it is refused as circular (tangent
to the fillet arc, which is not determined until the direction
is: the "fillets sit between defined geometry" rule, enforced by
the state machine).

**Legs (core).** Direction-consuming only, from a `Directed`
end: `line(len)`; `arc(…)` forms (the unique tangent arc to a
target point; explicit-sweep arcs at PR-spec time). A leg
terminates at a bound position → `Point`.

**Point-targeting constructors are SUGAR** (Evan, round 5:
"line_to is just sugar for .angle(theta).line(length)"):
`line_to(p)`
= `.angle(θ toward p).line(|p − cur|)`; `arc_to(p, bulge)`
desugars the same way (start direction computable from chord +
bulge — the M2 bulge convention). The Sharp junction check rides
the desugared `.angle(…)` uniformly. Sugar composes with the
fillet state: `.fillet(r, dd).line_to(p)` = arrival direction
from dd toward p, side terminating at p — well-defined because
the anchor is bound. Round-1/2's `arc_tangent_to` and
`start_dir` dissolve into `.tangent().arc_to(p)` and
`start(p).angle(θ)`.

**Fillet (round-5 form, Evan's).** One call on a `Directed` end
carrying only the arrival ANCHOR; the arrival direction follows
by director:

```text
.angle(θ).fillet(r, dd).angle(θ2)
```

Incoming carrier = the ray from the current position along θ;
the call binds anchor `dd` and enters `AwaitingDirection`; the
following director fixes the arrival carrier (line through `dd`
along θ2); the r-arc tangent to both carriers is inserted at
their implicit virtual corner, trimming both; the tip is then
Directed on the open arrival side — subsequent
direction-consuming forms TERMINATE OR CONTINUE that same leg
(`.line(len)` ends it past the anchor; another `.fillet(…)` runs
it into the next trim; `close_fillet` likewise) — one leg in the
lowering, so no collinear-split/same-carrier hazard arises from
the continuation. Grounds, recorded:

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
`start(p).angle(θ)`, anchoring side 1 at a real path point, is
the natural opening of an all-filleted loop). v1 rule: the seam
sits at a junction or fillet, never mid-carrier (a mid-carrier
seam = two collinear legs across the seam — the same-carrier/
collinear rules refuse it; PQ4 records the possible relaxation).

## 3. Surface vocabulary and worked examples

| Surface form | End-state transition | Core or sugar |
|---|---|---|
| `start(p)` | → Point | core |
| `.angle(θ)` | Point → Directed; AwaitingDirection → Directed | core (the two-state director) |
| `.tangent()` | Point → Directed (inherit + declared) | core; refused on AwaitingDirection (circular) |
| `line(len)` / `arc(…)` | Directed → Point | core legs |
| `.fillet(r, anchor)` | Directed → AwaitingDirection | core (the only corner primitive) |
| `line_to(p)` | Point or AwaitingDirection → Point | sugar: `.angle(toward p).line(dist)` |
| `arc_to(p, bulge)` | Point → Point | sugar (direction from chord + bulge) |
| `arc_to(p)` | Directed → Point | sugar for the unique tangent arc |
| `p1.then(p2)` | seam from the two end states | core, associative |
| `close()` / `close_tangent()` / `close_fillet(r)` | seam | same typing at last→first |

All-rounded square, fully determined (4 anchors + 4 directions,
sides read as anchor+direction pairs; every mᵢ a real on-path
point, e.g. side midpoints):

```text
start(m1).angle(east)
    .fillet(r, m2).angle(north)
    .fillet(r, m3).angle(west)
    .fillet(r, m4).angle(south)
    .close_fillet(r)
```

Mixed sharp + fillet (incl. the sugar-on-AwaitingDirection form):

```text
start(p0).line_to(p1)                 // sharp corner at p1 (sugar over .angle().line())
    .angle(θ).fillet(r, m).angle(θ')  // rounded virtual corner; side anchored at m
    .line(len)                        // arrival side ends len past m
    .line_to(p2)                      // sharp corner
    .close()
```

Evan's original tangent shape:

```text
start(a).angle(d)
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

Refusals from the typing (all typed): double director; `fillet`
on a `Point` end (no departure — the type-level face of "you
cannot fillet an authored corner away"); `line(len)` from a
`Point` end; a leg or `close()` from `AwaitingDirection` (the
arrival side has no direction yet — point-sugar excepted, since
it supplies one); `.tangent()` on `AwaitingDirection` (circular);
`NoCornerForFillet`; `AnchorOutsideTrimmedExtent`;
`UndeclaredTangency` on an exactly-tangent `.angle(θ)`;
`TangencyContradicted` from the verify layer as today.

## 4. The safety invariants, restated

1. **No junction is silently near-tangent**: Sharp junctions are
   checked; the refusal is ONE method with two payloads (Evan's
   round-2 framing): `ExactlyTangent` ("this junction is tangent
   — use `.tangent()`, or change the geometry") and
   `AmbiguousAtEps` ("tangent or sharp is ambiguous at this ε —
   move the geometry, or declare tangent, which is then VERIFIED
   and refused as `TangencyContradicted` if false"). Declaring
   SHARP is never an override — a declaration cannot make
   ill-conditioned geometry well-conditioned (F6). At the
   authoring layer there is nothing downstream; "escalation" is
   just this refusal.
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
