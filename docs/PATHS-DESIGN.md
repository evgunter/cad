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
exchange; resolver set REDUCED per Evan's review — see below)**:
a path is a sequence of **legs** (carrier-typed, some DOFs
possibly unbound) plus one **resolver** per junction:

```text
Junction resolver (v1, closed and BINARY):
  | Sharp          -- checked definitively non-tangent (the
                      definite-Zero/band machinery concentrates
                      here; #101's classifier, verbatim; refusal
                      payloads in §4)
  | TangentDirect  -- direction handoff across the junction
                      (G1: the two legs' directions agree at the
                      shared point)
```

*(Revision note, Evan's review 2026-07-28: the draft had a third
resolver `TangentArc { r }` — a junction that synthesizes a
fillet arc. Evan asked why it isn't just `TangentDirect` twice on
an arc, and he is right: a fillet is an arc LEG with only its
radius bound and BOTH its junctions `TangentDirect`; elaboration
binds its center/endpoints and trims the neighbors. The resolver
vocabulary collapses to the binary set above, the arc stays a
first-class leg like any other, and "fillet" becomes pure sugar
(§3). DOF check, recorded: an arc leg carries 5 DOFs
(center 2, r 1, sweep endpoints 2); r bound leaves 4; each
TangentDirect junction against a neighbor with a free trim
endpoint contributes contact + direction agreement = 2 bindings;
two tangent junctions = 4 — exactly determined, the classic
corner-fillet closed form, #101's `LoopBuilder::fillet`
generalized.)*

Elaboration is a constraint pass over the spine: resolvers bind
the unbound DOFs of adjacent legs; a leg left underdetermined
after all its junctions resolve is a typed error naming the
missing freedom; an overdetermined combination (both a fixed
direction AND a tangent handoff) refuses at the junction that
overdetermines it. A leg whose both junctions participate in its
determination (the fillet arc above) resolves as a local
leg-with-both-junctions unit — still closed-form, still local to
one leg and its two neighbors. Elaboration order and determinism
are §5.

**Composition and the trailing pending resolver (settled with
Evan, 2026-07-28 — round 2 of this section).** The round-1 text
here proposed distinct join operators, and a path-valued
`fillet(p2)` join whose land-on-the-next-carrier semantics Evan
then rejected against his own example ("it shouldn't land on
c→d"); the fillet semantics was re-forked and DECIDED as the
coincident-corner form (option 3 of the recorded fork: an r-arc
tangent to both carriers of an EXISTING nominal corner, trimming
back into each — the classic corner fillet, uniform across
in-chain corners, composition seams, and close; a corner always
requires r since tangent-to-two-carriers is a 1-parameter
family). The overdetermined single-arc, land-on-carrier, and
extend-to-intersect/biarc gap forms are all OUT of v1; the biarc
door may be named by a future refusal payload where a gap can
even be expressed (see below — in chain style it cannot).

The mechanism that makes composition and flat chains ONE algebra
(Evan's associativity point — nested-call authoring must not be a
distinct representation): a `PartialPath` value is legs + resolved
junctions + an optional **trailing pending resolver**.

- Every leg constructor CONSUMES the pending resolver (default
  `Sharp`) as its incoming junction.
- Junction markers (`.tangent()`, `.fillet(r)`) SET the pending
  resolver; two markers with no leg between refuse typed (one
  resolver per junction).
- Concatenation `p1.then(p2)` uses p1's pending resolver as the
  seam junction with p2's first leg. Hence
  `(p1.then(p2)).then(p3) == p1.then(p2.then(p3))` — flat chains,
  variables, and inline nesting all produce the identical value;
  there is no separate "join" vocabulary.
- `close()` / `close_tangent()` / `close_fillet(r)` resolve the
  last-to-first seam junction through the same code path.

Junction resolver spellings, complete: `Sharp` (default, checked),
`Tangent` (the `TangentDirect` handoff), `Fillet(r)` (inserts the
r-bound arc leg with both junctions tangent, trimming both
neighbors — a resolver spelling in the surface syntax, an
inserted leg in the core, per the DOF note above).

A structural consequence worth recording: in chain style the next
leg always starts where the previous ended, so the coincident-
corner requirement is satisfied BY CONSTRUCTION — gap
configurations (the biarc/extend-to-intersect family) are not
refused in the chain surface, they are unrepresentable in it.

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

| Surface form | Meaning under the pending-resolver mechanism (§2) |
|---|---|
| `start(p)` | begin at `p`, no pending resolver |
| `start_dir(p, d)` | begin at `p` with a start direction (required if the FIRST leg is direction-consuming — see the leading-tangent note) |
| `line_to(p)` | consume pending (default `Sharp`) as incoming junction; line leg to `p` |
| `arc_to(p, bulge)` | same; arc leg (endpoint + bulge) |
| `.tangent()` | set pending = `Tangent`; next leg's direction-consuming DOF binds through the handoff |
| `line_tangent(len)` | sugar for `.tangent().line(len)` — line leg with only `len` bound |
| `arc_tangent_to(p)` | sugar for `.tangent().arc_endpoint(p)` — the #104 unique-arc construction |
| `.fillet(r)` | set pending = `Fillet(r)` — the corner the previous leg's end and next leg's start form (coincident by chain construction) is rounded: r-arc inserted, both junctions tangent, both neighbors trimmed |
| `p1.then(p2)` | concatenate; p1's pending resolver is the seam junction (associative — §2) |
| `close()` / `close_tangent()` / `close_fillet(r)` | resolve the last-to-first seam with `Sharp` / `Tangent` / `Fillet(r)`, same code path |

Worked flat chains (recorded from the 2026-07-28 exchange):

```text
rounded square:
  start(p0).line_to(p1).fillet(r).line_to(p2).fillet(r)
           .line_to(p3).fillet(r).line_to(p0).close_fillet(r)

Evan's tangent shape, flat:
  start_dir(a, d).line_tangent(len).arc_tangent_to(b)
                 .fillet(r).line_to(dd).arc_to(a, bulge).close()
```

Refusals that fall out of the mechanism (all typed): two markers
with no leg between (one resolver per junction);
`.fillet(r).line_tangent(len)` — circular: the fillet needs the
next carrier defined, the tangent line wants its direction FROM
the fillet arc (fillets sit between defined geometry, the refusal
says so); a leading tangent leg without `start_dir` — its
direction would only resolve through the seam at close(), a
cyclic elaboration step v1 refuses (`ElaborationOrderUnsupported`
naming the seam; the one-pending-leg cyclic pass is a possible
v1.1 relaxation, recorded not committed).

Constraint on the table (binding on any future sugar): a sugar
constructor may only (a) append/insert one leg and/or (b) set the
pending resolver (an inserted fillet leg sets its own two
junctions). Anything needing more is not sugar and must be argued
as a core change.

## 4. The safety invariant, restated for J-core

**Every junction carries exactly one resolver, and every resolver
is total-or-typed.** Consequences: no junction can be silently
sharp-but-nearly-tangent; no tangency exists without a resolver
that declares it (the lowering emits the declared flag from the
resolver — declaration by construction, never inferred);
same-carrier junctions refuse at elaboration exactly as #101's
`same_carrier: true` (identity, not tangency).

**Refusal shape at a Sharp junction (clarified per Evan's review,
2026-07-28 — one method, two payloads, his framing adopted):**
both outcomes are the SAME typed refusal
(`SharpJunctionRefused { junction, kind }`), differing only in
payload:
- `kind: ExactlyTangent` — the junction is definitively tangent;
  message: "this junction is tangent — use a tangent resolver
  (or change the geometry)". The author's one-step fix is to say
  what is true.
- `kind: AmbiguousAtEps` — the classification is in-band;
  message: "tangent or sharp is ambiguous at this ε — move the
  geometry, or declare tangent (which will then be VERIFIED and
  refused as `TangencyContradicted` if false)". Note the
  asymmetry, stated honestly: declaring tangent is a real escape
  from the ambiguous band only when the geometry verifies exactly
  downstream; declaring SHARP is not offered as an override at
  all — an in-band junction under Sharp stays refused, because a
  declaration cannot make ill-conditioned geometry
  well-conditioned (F6's stance; the earlier draft called this
  "escalation", which wrongly suggested some downstream handler —
  at the authoring layer there is nothing downstream, it is just
  this refusal).

The #101 verify layer runs UNCHANGED on the lowered output — the
algebra is upstream insurance, the flags remain the contract of
record.

## 5. Elaboration semantics (determinism + failure vocabulary)

- Single pass, spine order, then the seam junction for cycles;
  each resolution step is local — a junction consumes its two
  adjacent legs' bound state, and an inserted/underdetermined leg
  whose determination spans both its junctions (the fillet arc)
  resolves as one leg-plus-both-junctions closed-form unit. No
  fixpoint iteration in v1: a step whose inputs are unbound
  because they await a LATER step is a typed
  `ElaborationOrderUnsupported` refusal (v1 scope cut, honest:
  chains like line—TangentDirect—line—TangentDirect—line with
  only outer endpoints bound elaborate left-to-right fine; a
  chain requiring right-to-left propagation refuses with the
  junction named — but see PQ3: composition makes these rare,
  because each composed sub-path elaborates forward independently
  and the join steps consume only both sides' already-determined
  local end state). D9: elaboration is pure f64
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

**PQ1 — Resolver vocabulary extent for v1** (revised: the set is
now the binary {Sharp, TangentDirect} after your TangentArc
collapse). The candidates the draft deferred, now spelled out per
your ask:
- `TangentAt(p)`: a tangent junction whose CONTACT POINT is
  user-pinned at `p` rather than derived by elaboration. Where it
  matters: today a tangent junction's contact point falls out of
  the neighbors' bound data (e.g. the fillet arc's tangent points
  land wherever the closed form puts them); `TangentAt(p)` says
  "tangent, AND the contact is exactly here", consuming leg DOFs
  to make it so — e.g. an arc leg tangent to a line AT a marked
  point has its center forced onto the normal at `p`. It is how
  an author pins a tangency to a datum. Cost: every leg-pair
  family needs a second closed form (tangent-with-pinned-contact),
  and overdetermination refusals get a new class.
- `Smooth` (G2): curvature-continuous junction — directions AND
  signed curvatures agree at the contact. For the v1 leg
  vocabulary it is nearly vacuous: line–line G2 is collinear
  (same-carrier ⇒ identity refusal), line–arc G2 forces infinite
  radius, arc–arc G2 forces equal radius + same center side ⇒
  same carrier ⇒ identity refusal. G2 only becomes a real
  junction kind once spline legs exist — and D2 deliberately
  keeps G2 joins in the conventional-`MappedCurve` regime (the
  surfaces/carriers under-determine the locus at G2; that is why
  the kernel's tangency enforcement is jet-determinate-only).
Recommendation (unchanged in substance): ship the binary set;
`TangentAt(p)` is the plausible v1.1 addition when a use appears
(the enum is additive); `Smooth` waits for spline legs on D2's
own grounds.

**PQ2 — Mixed authoring** (revised — your composition example
changes the answer). The draft's all-one-or-the-other rule was
written against ad-hoc interleaving. With composition in the core
(§2), the principled middle exists: a raw vertex+bulge chain
embeds as a sub-path via an explicit `lift(chain)` — its INTERIOR
junctions keep exactly today's #101 semantics (flags as authored,
verified at build), and the BOUNDARY junctions where it composes
with algebra-authored sub-paths are resolver-typed like any seam.
The dangerous seam (a silent near-tangency where two independently
authored pieces meet) is precisely the composition junction, and
that is now always guarded; interior raw junctions were already
#101-verified. Recommendation (revised): allow mixing at
composition granularity via explicit `lift`; still refuse ad-hoc
per-segment interleaving inside one sub-path (no win, and it
blurs which layer owns each junction).

**PQ3 — Elaboration order cut** (revised — yes, your example
addresses it). Accept the v1 left-to-right-only elaboration (§5)
with `ElaborationOrderUnsupported`, or require bidirectional
propagation first? Your multiple-forward-chains-joined shape is
exactly the escape hatch: each composed sub-path elaborates
forward independently, and join steps (including fillet_join)
consume only both sides' already-determined LOCAL end state — so
a construction that would need right-to-left propagation as one
long chain is instead authored as two forward chains and a join.
What remains refused in v1 is only the genuinely
both-ends-underdetermined single chain (e.g. a middle leg whose
every DOF waits on both neighbors, inside one sub-path, where no
composition boundary can be drawn). Recommendation (unchanged,
now with the stronger ground): accept the cut; the refusal names
the junction and the fix ("split and join here"); bidirectional
propagation stays additive later.

## 7. Explicitly out of scope

Implementation (banked for v2 profiles-as-programs, #104);
persistence changes of any kind (the lowering targets the EXISTING
v1 form: segments + tangent_joints flags); constraint-solver
interactions (M6 — the algebra is deliberately solver-free:
resolvers are closed forms, never iterative); 3-D paths; splines
as legs (NURBS legs join the vocabulary when profiles grow them,
the junction shape is carrier-generic by construction).
