# PATHS-DESIGN: the PartialPath authoring algebra (S5, design doc)

Status: **DRAFT for Evan's sign-off** (design-conversation PR — the
standing exception to self-merge; implementation is NOT scheduled:
it stays banked for the v2 profiles-as-programs work per #104's
sequencing record. This doc is the deliverable).

Lineage: Evan's concept (#104, 2026-07-25) + the 2026-07-27
in-session exchange (junction-join primacy; forward sugar as a
requirement). Harmonization constraint (ratified context): #101's
declared-tangency discipline (flags verified-never-trusted,
`UndeclaredTangency`/`TangencyContradicted`, fillet fit gating,
same-carrier-is-identity) landed at #109/#112 and is the semantic
layer this algebra LOWERS to — nothing here replaces it; schema v1
persists explicit geometry + flags, and the lift to the algebra is
determined by construction (the load-bearing reason the flags
exist).

## 1. What this is

A typed authoring algebra for profile loops in which **accidental
tangency is unrepresentable and intended tangency is exact by
construction** — each constructor consumes exactly the degrees of
freedom the neighboring geometry has not already determined. The
algebra is a *generator-layer* surface (D8: user-facing Rust
generates recipes); it lowers to what exists today: explicit
segments (vertices + bulges) plus declared tangency flags, verified
at build by the same junction predicates. No kernel or document
semantics change.

## 2. The core fork: forward-consuming vs junction-resolver

**F-core (the #104 original)**: `PartialPath` grows end-to-end;
each constructor eats the freedoms the previous END leaves.
Tangent-line takes only a length; tangent-arc takes only an
endpoint; non-tangent constructors CHECK definite non-tangency.

**J-core (this doc's recommendation, from the 2026-07-27
exchange)**: a path is a sequence of **legs** (carrier-typed, some
DOFs possibly unbound) plus one **resolver** per junction:

```text
Junction resolver (v1, closed set):
  | Sharp          -- checked definitively non-tangent (the
                      definite-Zero/band machinery concentrates
                      here: definite-tangent => typed refusal
                      pointing at a tangent resolver; in-band =>
                      escalation. #101's classifier, verbatim)
  | TangentDirect  -- direction handoff across the junction
                      (G1: the outgoing leg's start direction is
                      determined by the incoming leg's end)
  | TangentArc { r } -- a two-sided fillet join: an arc of radius
                      r tangent to BOTH neighbors, trimming each
                      to its tangent point (the #101
                      LoopBuilder::fillet closed form, generalized
                      to any leg pair with defined end directions)
```

Elaboration is a constraint pass over the spine: resolvers bind
the unbound DOFs of adjacent legs; a leg left underdetermined
after all its junctions resolve is a typed error naming the
missing freedom; an overdetermined combination (both a fixed
direction AND a tangent handoff) refuses at the junction that
overdetermines it. Elaboration order and its determinism are §5.

**Why J-core wins (recommendation, with the honest counterweight):**

1. **The two-sided join is the paradigm case, not an add-on.**
   Evan's litmus — "join these two lines by a circle tangent to
   both" — is unwritable in F-core (the successor leg does not
   exist when the arc is authored) and is one constructor call in
   J-core. F-core would need a lookahead/patch-up mechanism that
   breaks its own determined-prefix invariant.
2. **Closure stops being special.** F-core's hardest residue
   (#104 flagged it) is the last-to-first junction: the closing
   constructor is overdetermined for a single arc. J-core closes
   by applying an ordinary resolver to the seam junction — the
   same code path as every interior junction; `close()` merely
   marks the cycle and runs the same pass.
3. **It matches #101's granularity exactly.** The classifier, the
   refusal vocabulary, and fit gating are all per-junction
   already; lowering is junction-by-junction with no impedance
   mismatch.

Counterweight, on record: F-core's "determined prefix + free end"
is a simpler under-construction object with a simpler totality
story; J-core's spine-with-holes needs the §4 invariant restated
carefully and an elaboration pass F-core doesn't need. The
recommendation stands because the complexity J-core adds is
load-bearing (two-sided joins, uniform closure) while the
complexity F-core adds under pressure (lookahead patches, special
closure) is incidental.

## 3. Forward sugar (REQUIREMENT, per Evan 2026-07-27)

Turtle-style forward authoring remains first-class; every forward
constructor is a single call desugaring mechanically to spine +
resolvers. The v1 sugar table (each row: constructor → lowering):

| Forward constructor | Lowering |
|---|---|
| `line_to(p)` | line leg to `p`, trailing junction `Sharp` |
| `line_tangent(len)` | line leg with only `len` bound; trailing junction of the PREVIOUS leg becomes `TangentDirect` |
| `arc_to(p, bulge)` | arc leg (endpoint + bulge), trailing `Sharp` |
| `arc_tangent_to(p)` | arc leg with endpoint only; previous junction `TangentDirect` (center on the start normal, equidistant — the #104 unique-arc construction) |
| `fillet(r)` (post-hoc, between the two most recent legs) | rewrite the junction's resolver to `TangentArc { r }` |
| `close()` / `close_tangent()` | mark cycle; seam junction `Sharp` resp. `TangentDirect` |

Constraint on the table (binding on any future sugar): a sugar
constructor may only (a) append one leg and/or (b) set one
junction resolver. Anything needing more is not sugar and must be
argued as a core change.

## 4. The safety invariant, restated for J-core

**Every junction carries exactly one resolver, and every resolver
is total-or-typed.** Consequences: no junction can be silently
sharp-but-nearly-tangent (Sharp CHECKS, per #101's classifier —
definite-tangent refuses with the repair menu naming the tangent
resolvers; in-band escalates per F6); no tangency exists without a
resolver that declares it (the lowering emits the declared flag
from the resolver — declaration by construction, never inferred);
same-carrier junctions refuse at elaboration exactly as #101's
`same_carrier: true` (identity, not tangency). The #101 verify
layer runs UNCHANGED on the lowered output — the algebra is
upstream insurance, the flags remain the contract of record.

## 5. Elaboration semantics (determinism + failure vocabulary)

- Single pass, spine order, then the seam junction for cycles;
  each resolver is local (consumes only its two adjacent legs'
  bound state). No fixpoint iteration in v1: a resolver whose
  inputs are unbound because they await a LATER junction's
  resolution is a typed `ElaborationOrderUnsupported` refusal
  (v1 scope cut, honest: chains like line—TangentDirect—line—
  TangentDirect—line with only outer endpoints bound elaborate
  left-to-right fine; a chain requiring right-to-left propagation
  refuses with the junction named). D9: elaboration is pure f64
  structure selection (C6 boundary — it decides leg parameters,
  never topology); the lowered profile then goes through the
  ordinary generic pipeline.
- Failure vocabulary (all typed, all naming their junction/leg):
  `UnderdeterminedLeg { leg, missing }`,
  `OverdeterminedJunction { junction, conflict }`,
  `TangentArcUnfit { junction, .. }` (the #101
  `TangentJointOutOfRange` generalized: tangent point off-leg),
  `ElaborationOrderUnsupported { junction }`, plus the #101
  vocabulary passing through from verification.

## 6. Open questions for Evan (genuine forks in this doc)

**PQ1 — Resolver vocabulary extent for v1.** Is the closed set
{Sharp, TangentDirect, TangentArc(r)} enough, or should v1 include
`TangentAt(p)` (tangency with a pinned contact point) and/or
`Smooth` (G2) as vocabulary? Recommendation: the closed three —
each extra resolver is a new closed-form family to verify, and G2
at a junction is exactly the conventional-`MappedCurve` regime D2
keeps OUT of intrinsic tangency; add by demand, the enum is
additive.

**PQ2 — Mixed authoring.** May a single profile mix raw segment
authoring (today's vertex+bulge chains) with PartialPath algebra,
or is a profile all-one-or-the-other? Recommendation: all-one-or-
the-other per profile LOOP (a loop is either authored as a path or
as raw segments; both lower to the same persisted form) — mixing
within a loop reopens exactly the silent-adjacent-tangency seam
the algebra exists to close, for no authoring win.

**PQ3 — Elaboration order cut.** Accept the v1 left-to-right-only
elaboration (§5) with `ElaborationOrderUnsupported`, or require
full local-propagation (both directions) before anything ships?
Recommendation: accept the cut; the refusal names the fix, the
common authoring patterns are forward-shaped, and bidirectional
propagation is additive later.

## 7. Explicitly out of scope

Implementation (banked for v2 profiles-as-programs, #104);
persistence changes of any kind (the lowering targets the EXISTING
v1 form: segments + tangent_joints flags); constraint-solver
interactions (M6 — the algebra is deliberately solver-free:
resolvers are closed forms, never iterative); 3-D paths; splines
as legs (NURBS legs join the vocabulary when profiles grow them,
the junction shape is carrier-generic by construction).
