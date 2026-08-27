# Contact census & declared contact (pre-implementation design doc)

**Status: RATIFIED (M6 unit 4, DESIGN-ONLY; Evan's sign-off on
PR #178, 2026-08-04 — ratification recorded in M6-LOG).** This is the
design doc OQ5 deferred to ("the curved coincidence census waits for
its own design doc" — CURVED-DESIGN OQ5, decided Evan #85). It
resolves OQ5's *design* half; it schedules **no implementation** and
changes **no behavior** — every refusal named below keeps refusing
until its unit ships. The C-numbered proposals are ratified as
written. C-numbers are
LOCAL to this document; CURVED-DESIGN's C-numbers are always cited
qualified ("CURVED-DESIGN C7"), never bare.

## Grounding (ratified text this doc builds on, not re-litigates)

D2 / CURVED-DESIGN C7 — `TangentIntersection`'s
predicate-plus-margin pattern (order-k contact = k-jet equations,
margin at order k+1), the jet certificate schedule, the second-order
sector trilean, and OQ5's ratified deferral, discharged here at the
design level; this doc extends the pattern from one body's edges to
*pairs of bodies*. Tier 3′ + the #42 invariant (touching always
backed by explicit intent; discovery is never declaration;
certification runs both directions) and the round-8 coincidence
ladder (structural / declared / typed sliver — value equality never
glues). The declared-intent precedents: `plane_eq`'s declared rung
(M4 F5 — *intent plus non-contradiction, never value-inferred
coincidence*), #101 profile tangency (verified never trusted;
constructors declare the tangency they author — the PATHS
`.fillet(r)` door), and M5 S1/#140's planar declared-REST zip (conformal
patches removed as interior, seam minted once, exact volume
additivity — the shipped precedent generalized here). The M6
evidence: #175 findings 1–2 ("a branching G1 tube system is nothing
but tangent curved contacts"), the two-peg demo
considered-not-built pending cylindrical declared contact, and #161
§2a–c — this unit's origin. ERROR-DESIGN E3/E7 (M10),
whose `min_clearance` Measure and clearance trichotomy consume C5's
contract below.

## C1 — The contact census: classification of the pair germ

Setting: two tier-3-valid bodies A, B at rest, p ∈ ∂A ∩ ∂B with p in
the interior of a face of each. (Contact at edges/vertices reduces to
the existing machinery — vertex-granularity records and sector/wedge
classification; the curve- and patch-classes below are *bounded* by
those records, exactly as the planar census's segment-reconstruction
rule bounds overlaps by vertex events.) Outward normals n_A, n_B are
the S10 sense-signed normals.

**Conventions, fixed once.** For a body X with outward unit normal
n_X at p, write ∂X locally as the graph of h_X over the tangent plane
along the axis n_X (h_X(0) = 0, ∇h_X(0) = 0 when tangent); material
lies below the graph ({z ≤ h_X}) by outwardness. Define
**II_X := Hess h_X(0)** — the second fundamental form measured
against the body's own outward normal in the graph convention. A
solid ball of radius r has II = −(1/r)·I at every boundary point; a
spherical socket of radius R (material outside the hollow) has
II = +(1/R)·I.

**The classification.**

1. **Transverse crossing** — n_A, n_B linearly independent. ∂A ∩ ∂B
   is locally a 1-manifold (IFT; D2's `Intersection` regime one level
   up) and the interiors overlap in an open lens near p. Between
   boolean *operands* this is the generic working case. Between
   bodies *at rest* it is interference, never contact.
2. **Aligned tangency** — n_A = n_B. Lemma: both materials lie on the
   −n side, so {z < min(h_A, h_B)} ⊆ int A ∩ int B — aligned tangency
   ALWAYS carries local material overlap (the flush-containment /
   nested configuration). It classifies under interference or
   containment, never rest contact. Rest contact requires opposed
   normals; this is a theorem of the convention, not a policy.
3. **Opposed tangency** — n_A = −n_B. Fix the frame n := n_A. The
   local **separation** is s := h_B − h_A; interiors locally disjoint
   ⟺ s ≥ 0. s(0) = 0, ∇s(0) = 0, and
   **Hess s = −(II_A + II_B) =: II_rel** — the relative form. (Checks:
   ball on plane, II_rel = (1/r)I ≻ 0; ball r in socket R,
   II_rel = (1/r − 1/R)I, the #161 §2c sign analysis verbatim.)
   Sub-classes by II_rel and the contact set:
   - **Point touch** — II_rel ≻ 0 (positive definite): separation
     grows quadratically in every direction; contact set locally {p}.
     (Sphere on plane; external sphere–sphere tangency; saddle
     contact with definite relative form.)
   - **Curve touch** — II_rel ⪰ 0 with 1-dimensional kernel AND
     s ≡ 0 along a witnessed curve tangent to the kernel, quadratic
     separation transverse to it (κ_rel = the positive eigenvalue,
     bounded away from zero). This is D2/CURVED-DESIGN C7's
     `TangentIntersection` regime stated for a body pair: the jet
     certificate is per-locus, not per-germ — the rank-1 germ alone
     leaves the kernel direction quartic-or-flatter, so the
     along-locus equations (s = 0, ∇s = 0 at samples along the
     witnessed curve) are part of the certificate, never inferred.
     (Cylinder on plane; pin against a flat; ball in a groove of
     strictly larger radius — internal tangency, κ_rel = 1/r − 1/R.)
   - **Conformal (surface-measure) contact** — s ≡ 0 on a
     2-dimensional patch. (Ball seated in a congruent socket; pin
     filling a same-radius bore; the planar crosslap REST face.)
     See the identity lemma in C2: for this kernel's surface classes,
     conformal contact forces *identical carriers*.
   - **Crossing touch** — II_rel indefinite: the surfaces cross
     through the tangent plane (saddle against plane, crossing); the
     interiors overlap on the negative cone. At rest: interference.
   - **Degenerate residue** — II_rel ⪰ 0 with κ_min in the escalation
     band but carriers not identical (near-conformal: r → R in the
     socket; osculating pairs; order-k > 1 non-conformal contact).
     Not a representable class: escalate (F6), recourse in C4.
4. **Interference** — a *regional*, not germ, class: int A ∩ int B
   has positive volume. Locally it is what transverse crossing,
   aligned tangency, and crossing touch produce; a declared
   interference FIT (C6) is the one intentional form.

**Invariant (C1).** The classification is exhaustive at the germ
level, and every class boundary is a Q1 trilean at a stated
differential order: order 1 (normal independence — the existing
transversality margin), order 2 (sign/rank of II_rel — curvature
margins at D4 ¶1 lever arms, threshold Δκ against ε/ℓ² at the named
feature extent ℓ, consistent with CURVED-DESIGN C7's second-order
sector trilean), order ∞ (conformality — decided structurally, C2,
never numerically). In-band at any order escalates; no class is ever
assigned by proximity.

II_A, II_B are closed-form for the D3 analytic carriers and
hull-bounded for NURBS (CURVED-DESIGN C9 ring; the same
second-derivative hull bounds tessellation's chordal certificate
uses) — so every order-1 and order-2 predicate above is decidable in
the C9 ring, with the *decision* trilean and the *blessing* separate
(C2).

## C2 — Representation boundary and the decision procedure

**The identity lemma (load-bearing), stated at its true strength.**
The kernel's surfaces are real-analytic (plane/cylinder/cone/sphere/
torus) or piecewise-rational (NURBS). For the ANALYTIC kinds: two
surfaces agreeing on an open 2-D patch agree as loci (identity
theorem), and agreement pins the defining data (two spheres sharing
a patch share center and radius; a sphere and a cylinder share no
patch; etc.) — so for analytic pairs, **every true conformal contact
is same-carrier contact**, with no "partially conformal" class. For
PIECEWISE-rational carriers the guarantee is per-span: agreement on
an open subset of a knot-span product is polynomial identity on THAT
span, and a piecewise carrier can conform on some spans and depart
at a knot line — whole-carrier identity is NOT implied, and a
span-partial conformal patch is a real configuration. Consequence,
correctly scoped: conformality is decidable structurally with
nothing lost for analytic pairs; for NURBS pairs the structural and
declared rungs decide the conformal patch span-wise, and a
span-partial coincidence WITHOUT structural or declared backing
escalates (F6) rather than classifying — it is exactly the
near-coincidence shape the ladder refuses to glue numerically.
Deciding conformality structurally (shared key / same `GeomSource` /
declared + verified) therefore still loses nothing that is honestly
decidable today. Caveat stated exactly: carrier
identity is at the *locus* level; two descriptions of one locus may
differ as charts (u_ref, seam) and as sources — the structural rungs
are sufficient, not necessary, which is precisely what the declared
rung exists for. NURBS↔analytic and NURBS↔NURBS same-locus
*recognition* is D7 adoption work (M7), not census work.

**The decision procedure**, at the predicate level. For a face pair
(one face of A, one of B) the census asks, in order:

1. **Exclusion** (C9 ring): a certified distance enclosure definitely
   positive ⇒ no contact, no record. This is the "subdivision
   certifies exhaustiveness" posture (the banked SSI-completeness
   principle): interval exclusion proves regions contact-free;
   whatever it cannot exclude becomes a *candidate*, never a verdict.
2. **Structural rung**: shared surface key or same oriented source
   (N6) with opposed senses ⇒ conformal candidate, intent by
   construction; the record needs only the overlap witness (C3).
3. **Declared rung**: a declaration naming this pair (C4) ⇒ verify
   per its class table (C4): definite counter-evidence contradicts
   (typed), definite confirmation certifies, in-band residue is
   bridged by the intent.
4. **Definite-separation / definite-crossing by geometry**: order-1
   and order-2 trileans (C1). Definitely-transverse crossing at rest,
   crossing touch, or aligned tangency ⇒ typed interference error
   (unless the pair carries a Fit declaration, C6). A definite
   tangential *touch* with no declaration ⇒ `UndeclaredContact` —
   the finding is decidable, the blessing is not (#42: discovery is
   never declaration).
5. **In-band anything** ⇒ `Escalated` — a genuine sliver at this ε.

**What the kernel REPRESENTS**: transverse crossings (as boolean
working state — never as an at-rest contact); point touch (vertex
records — exists); curve touch (curve records, C3 — new); conformal
contact (patch records, C3 — new); declared fits (C6). **What it
REFUSES, typed with recourse**: undeclared touching (declare / move /
lower ε — the two-tolerance ONE story), contradicted declarations,
degenerate-residue contact (order-k > 1 non-conformal: no
representation exists; recourse is making it conformal — share the
carrier — or moving off the osculation), and interference without a
Fit declaration.

**Invariant (C2).** Conformality is never decided numerically:
`conformal ⇒ same carrier` (the lemma) and `same carrier` is
structural-or-declared data. Equal-but-independent carriers do not
glue (ladder rung (b) verbatim); a discovered touch never
self-blesses; escalation is terminal, never a guess. No flag, mode,
or tolerance loosens any of these — there is no escape hatch.

*Alternative honestly argued — numeric conformality* (decide "same
sphere" by |Δc| + |Δr| < Kε): rejected. It recreates the
unmargined-cliff defect the round-8 ladder killed, and the identity
lemma shows it buys nothing — every genuine conformal contact
already has a structural or declarable carrier identity; numeric
gluing would admit exactly the near-coincidences F6 escalates.

## C3 — Contact records: two new granularities

Tier 3′'s records are vertex-granularity (`VvContact`, `VfContact`),
with segments certified by reconstruction from bounding vertex
records. Curved contact needs two more, mirroring the census
posture's honesty clause:

- **`CurveContact { face_a, face_b, witness }`** — a certified curve
  touch. Certification = CURVED-DESIGN C7's jet schedule applied to
  the body-pair faces: per sample along the witnessed locus, surface
  coincidence within ε, normal opposition within the derived angle
  ε·κ_rel (lever arm 1/κ_rel), κ_rel definitely positive; hull bounds
  between samples; witness pinned at carrier(mid) (the S2 argument,
  unchanged). Endpoints are bounded by vertex records or by the
  locus's own closure (a full circle needs no bounds) — the
  segment-reconstruction rule one dimension up: a curve record whose
  bound lacks a backing vertex record is `UndeclaredContact`, never
  inferred.
- **`PatchContact { face_a, face_b }`** — a certified conformal
  patch. Certification is structural + 2-D: carrier identity by rung
  2 or 3 (never rung "value-equal"); senses opposed (the C1 lemma:
  aligned coincidence is containment, and a `PatchContact` claiming
  it is contradicted); region overlap **in the shared chart** with
  definitely-positive area (a pcurve-level planar problem — the
  face regions' trim loops intersected in (u,v); the planar census's
  containment machinery, run in chart space). Overlap empty ⇒ the
  record is stale; overlap indeterminate ⇒ escalate.

**Invariant (C3).** Certification strength equals its skeleton,
stated as honestly as the planar census states it: a `CurveContact`
is certified at its samples plus hull bounds — its blessing is
per-locus, and a configuration needing more than bounding records
plus the jet schedule (order-k > 1 contact) has no record type and
refuses. A `PatchContact`'s area test is exact in chart space on the
planar trim inventory and refuses typed on trim curves outside it —
the same envelope discipline as the F5 census, moved to (u,v); its
"shared chart" is the structural rungs' by construction (shared key /
same source ⇒ bit-identical descriptions), and a rung-3 pair
escalates typed — C2's own caveat that two descriptions of one locus
may differ as charts makes chart-space exactness unachievable there.

**REVISION (#1063, 2026-08-27; U-R2 as corrected).** The rung-3
sentence above gains ONE arm, and only one: a declared **PLANAR**
pair may be answered on the pair's **shared world carrier** — one of
the two plane descriptions, taken as the pair's REPRESENTATIVE FRAME.
Everything else about rung 3 is unchanged, curved pairs included.

The revision does NOT claim the exactness C2 refused, and it does not
claim the world embedding is parameter-free — it is not: a
`Surface::Plane` carries `u_ref`, and the arm reads both trims in one
of the two frames. Three things make it honest instead, and all three
are load-bearing:

1. **Frame invariance of the ANSWER.** Both chart maps are isometries
   of the Euclidean plane onto their carriers, so the map between the
   two frames is a rigid motion (possibly with a reflection), and
   every quantity the area machinery consumes — shoelace area,
   perimeter, incidence, containment, the `2A/P` mean width — is a
   Euclidean invariant. Orientation, the one thing a reflection does
   not fix, is absorbed structurally by the loop walk's CCW
   normalization before the machinery runs; the metering is unmoved
   because a plane chart's lever arms are `(1, 1)` in either frame.
   The certified verdicts are therefore invariant under the choice of
   representative. The lemma is WRITTEN at `chart_region.rs`'s
   `world_carrier`, and argument-order symmetry is pinned as a row.
   What the lemma deliberately does not cover: the ray schedule is
   fixed in CHART coordinates, so *which* configurations refuse
   rather than decide is conditioning-dependent and rotates with the
   frame.
2. **Certified everywhere within ε, not exact.** A verified
   declaration does not prove exact locus identity: `decide`'s
   `Ok(Zero)` means `|m| ≤ zero`, never bit-zero. The claim the arm
   earns is that the two carriers agree to within ε everywhere the
   pair's trims reach.
3. **Metered at the PAIR'S OWN EXTENT.** `chart_region_carrier_tilt`
   measures the two carriers' largest separation over the union of
   the pair's own boundary vertices, and refuses typed when that is
   definitely positive. Door 1's carrier ladder meters the same
   disagreement as an angle at a PINNED 1 m arm, which prices a peg
   and a table alike; this row does not. One tilt, two extents, two
   honest answers.

Cross-instance CURVED declared pairs keep the escalation. There the
divergence C2 names is real — two independently authored curved
descriptions differ in `u_ref` and seam, no world embedding
arbitrates that, and there is no isometry lemma to be had (a
cylinder's chart map is not an isometry in azimuth unless the radii
agree exactly, and the seam makes containment branch-dependent). The
closure that fits them is a certified everywhere-within-ε overlap
enclosure on the shared curved carrier, a different shape from
either.

*Alternative — area-sampled patch certification*: rejected; sampling
can miss a trim hole and certify a contact that is not there — the
missed-small-loop disaster the SSI-completeness principle kills.

## C4 — Declared contact as data

**What a declaration is.** Recipe data, exactly the #101/F5 shape: a
typed relation on the *consuming node*, naming two faces by stable
name (never arena keys — the G1 boundary rule), asserting a contact
class:

```text
ContactClass =
  | Rest              -- conformal: same carrier, opposed senses, gap ≡ 0
  | Tangent           -- curve/point touch: opposed tangency, non-crossing
  | Fit { gap: g₀ }   -- carrier-parallel pair at signed nominal gap g₀ ≠ 0
```

`Rest` generalizes S1's planar declared-REST vocabulary to every
carrier kind; `Tangent` generalizes #101's profile-junction tangency
to body pairs; `Fit` is C6. A declaration asserts *intent about the
nominal geometry*; parameter-band assertions ("gap ∈ [lo, hi] over
the tolerance box") are M10 assertions over the C5 measure, not
kernel declarations — the kernel's nominal geometry is never "maybe
touching".

**Verified never trusted — the per-class tables.** Extending the
declared rung's ratified semantics (*intent plus non-contradiction,
never value-inferred coincidence*), each class states exactly three
lists; the declaration bridges ONLY the third:

- must-verify-DEFINITE: conditions the geometry must definitely
  satisfy, or the declaration is `ContactContradicted`;
- contradiction triggers: definite counter-evidence, typed;
- bridged residue: the indeterminate/in-band margins intent covers.

`Rest`: definite = carrier non-contradiction (the kind-generalized
`oriented_plane_eq` ladder: same kind, defining data not definitely
distinct — for spheres center & radius, for cylinders axis & radius,
each margin at its named lever arm), opposed senses (exact bit, S10),
overlap definitely positive **in the pair's chart** (else stale, C3)
— that chart being the structural rungs' by construction, or, for a
declared PLANAR pair, the shared world carrier of C3's revision, whose
representative frame is certified by `chart_region_carrier_tilt` at
the pair's own extent.
Contradicted by: definitely-distinct carriers, aligned senses,
definite separation anywhere on the claimed patch. Bridged: in-band
carrier-data margins between independently-authored descriptions —
the declaration is what makes them one carrier (ladder rung (b)'s
"explicit recipe-level relation", now for every kind).

**The two doors are not independent (#1063).** The world-carrier arm
exists only because the declaration verified, so the area
certification is handed Door 1's `ContactVerdict` rather than
re-deriving or assuming it. It reads it in exactly one place: the
interior-witness rung — the rung that answers a FLUSH seat, where the
trims share a boundary and the region walk can build no intersection
piece — runs only on `Definite`. That rung proves its point lies ON
both carriers, and a `Bridged` verdict is precisely the statement that
the carriers' coincidence rests on the declaration rather than on the
geometry; a precondition may not be discharged by the claim under
test. On `Bridged` the rung declines and the region walk's typed
refusal stands.

`Tangent`: definite = first-order tangency along the witnessed locus
(normal opposition within the derived angle — for constructor-
authored joints, e.g. equal-minor-radius tube chains, this is exact:
both walls' normals at the rim are the shared cross-section circle's
radial directions), and the locus on both surfaces within ε.
Contradicted by: definite normal independence at a claimed sample,
definite crossing (II_rel definitely indefinite), definite
separation. Bridged: in-band κ_rel — including *exact* zeros at
isolated points (the planar-stem tube chain's neutral meridian
points, where both spines' curvature projections vanish; the jet
certificate is honestly indeterminate there and the declaration
carries the non-crossing intent across exactly those samples). This
is the one place declared `Tangent` is deliberately WEAKER than a
jet certificate — and it must be: demanding jet-determinacy
everywhere would make every G1 tube chain undeclarable at its two
neutral points, reproducing #175 finding 1 with extra steps.

`Fit`: C6's table.

**Where it lives.** Declarations are recipe data, period: on the
consuming boolean node today (`DeclaredPairs` grows a class payload),
on assembly-era relation/mate nodes when assemblies exist (GQ4:
assemblies are recipes of the same formalism — the vocabulary binds
now, the second home lands then). Bodies at rest never carry
declarations; they carry the *verified records* (C3) in the result
wrapper, the `BooleanBody` pattern — validity class rides the
wrapper, never a mutable body field. Persistence: declarations
persist as node payload (the `tangent_joints` schema precedent) in
the schema version that ships the implementation, forward-only per
D6.3; records are never persisted (bodies re-derive, D9).

**Replay.** Verification is scalar-generic like every predicate:
f64 replay re-verifies; Interval replay's indeterminate verification
aborts to the Q1 driver, never silently passes. Bit-identical replay
reproduces records bit-identically (D9); records carry across
result-side mints by descendant map, never re-derivation (the tier-3′
carriage rule, unchanged).

**Failure modes, all four typed, one story each (two-tolerance):**

- `UndeclaredContact { finding }` — geometry touches, no backing
  declaration. Recourse: declare the named class / move the geometry.
- `ContactContradicted { declaration, witness, margin }` — definite
  counter-evidence where the lie meets geometry. Fired at use (F5
  verified-at-use: a declaration that never meets geometry is a
  silent no-op at the op) AND at the at-rest gate (S1's deviation-2
  strictness: false RESTs are never silent once they meet the zip).
- `StaleContactDeclaration { declaration }` — at-rest record with no
  geometric witness (dead names, empty overlap): the census's
  two-directional certification, extended verbatim.
- `Escalated { diag }` — in-band geometry; terminal, priced, honest.

**Invariant (C4).** A declaration is trusted exactly on its bridged
residue and nowhere else; every definite verdict wins over every
declaration; and no path exists from "the numbers look equal" to a
glued contact without a structural or declared rung — at any ε, at
any scalar backend.

## C5 — The signed gap: one clearance object, co-designed for M10

**Definition (per declared pair).** For a declared contact/fit pair
on same-kind carriers with a shared mating frame, the **gap** g is
the carrier-relative signed offset, positive toward separation:

- parallel planes: g = signed inter-plane offset (linear in the
  authored offsets — smooth everywhere);
- concentric spheres (ball r in socket R): g = R − r − ‖Δc‖;
- coaxial cylinders (pin r_p in bore r_b): g = r_b − r_p − d, d the
  axis offset (parallel axes; skew axes refuse typed — a skewed
  "fit" is not a fit).

**Sign convention (binding for M10): g > 0 clearance, g = 0 contact,
g < 0 interference.** The census classes are the strata of g's zero
set, stated exactly: g = 0 with structural frame sharing (Δc ≡ 0)
and equal radii is the conformal class `Rest`; g = 0 with an offset
frame (‖Δc‖ = R − r > 0) is C1's internal point/curve touch — the
ball touching its socket off-center. That is why fits and rest
contact are one vocabulary, not two, and why the conformal limit is
reached only through structure, never through g drifting to zero.

**Smoothness, stated honestly (Evan will probe this).** g is NOT the
min-over-points distance; that is the point of defining it
carrier-relative. Its regularity in model parameters:

- with **structural** frame sharing (Δc ≡ 0, d ≡ 0 by construction),
  g is linear in the radii — smooth, the ideal M10 citizen; the
  derivative of `R − r` is ±1 and a tolerance stackup on a fit is
  exact interval arithmetic;
- with **independent** frames, g carries the norm kink ‖Δc‖ at
  Δc = 0 — which is the *nominal operating point* of every real fit.
  There g is semismooth with Clarke subdifferential the closed unit
  ball image (the Q1 `Dual<Interval>` straddle-hull treatment,
  ratified at M0, applies verbatim — no new machinery);
- the undeclared **global** `min_clearance` (E7) keeps its
  min-structure: lower-semismooth, active-pair switching kinks,
  handled by interval subdivision without derivatives, exactly as E7
  already specifies.

Design pressure made explicit: structural mates don't just verify
more cheaply (C2 rung 2), they make the M10 stackup differentiable at
the operating point. The kernel should say so in the recourse text.

**What M10 consumes (the contract, nothing more designed here):**
(i) the sign convention above; (ii) g as an ordinary scalar-generic
E3 Measure (a signed Length — `min_clearance(sel)` generalizes to
`gap(declaration)`); (iii) the zero-set-is-contact identification,
so E7's trichotomy gains a two-sided form: certify g ∈ [band] over
the leaf box; (iv) the smoothness statement, so the E4 dual lane
knows where derivatives exist and where the Clarke enclosure is the
honest object. M10's propagation itself (leaves, mass accounting,
budgets) is untouched by this doc.

**Invariant (C5).** One gap definition serves census verification
(its sign trilean is C4's separation/contact/interference evidence),
the fit band (C6), and M10's measure — defined once, evaluated at any
`T`, never re-derived per consumer with drifting sign conventions.

## C6 — Interference fits: declared negative clearance

A fit declaration `Fit { gap: g₀ }` asserts the *nominal* signed gap
of a carrier-parallel pair: g₀ > 0 a clearance fit, g₀ < 0 an
interference (press) fit. g₀ = 0 is REJECTED at declaration: zero
nominal gap is conformal contact and must be authored as `Rest`
(same carrier, structural) — otherwise the vocabulary would reopen
the value-equality door the ladder closed. Transition fits are a
band question, hence M10's: model the nominal at its definite g₀ (or
as `Rest`), assert the band over the tolerance box.

Verification table (C4 pattern): definite = same kind,
frame relation as declared (coaxial/concentric/parallel — structural
or declared axis relation, itself verified), and the evaluated gap g
definitely on g₀'s side of zero with |g − g₀| within the derived
threshold at the fit's lever arm. Contradicted by: definite opposite
sign (a declared press fit that measures clearance), definitely
non-parallel frames. Bridged: in-band |g − g₀|.

Consequences of a verified interference declaration, each typed and
recorded, none silent:

- the disjointness/containment/extent gates (e.g. S13's extent scan)
  **skip the declared pair as a recorded verdict** — the gate's
  output names the declaration it consumed, never a bare pass;
- assembly-level mass properties **refuse by default**
  (`OverlapUncorrected`, naming the pair); an explicit opt-in
  subtracts the certified overlap volume where closed forms exist
  (coaxial cylinder fit: π(r_p² − r_b²)·L over the engaged length;
  concentric spheres: the lens integral) and refuses typed
  otherwise. The kernel reports *geometric* overlap; elastic
  redistribution is physics and permanently out of scope — stated
  in the op's docs, not silently approximated;
- booleans are unchanged: a fit pair unioned is an ordinary
  overlapping union (generic transverse crossings at the engagement
  rims); fits alter assembly gates and measures, never boolean
  semantics;
- STEP has no native concept: export drops the declaration (stated
  at the writer, the honest #161 §2b answer); the recipe is the save
  format, so nothing is lost on OUR side.

**Invariant (C6).** An undeclared interference is always a typed
error; a declared fit is verified at every evaluation (replay
included) and its skip-verdicts are data in the result, so no gate
weakening survives outside the declared pair. No blanket
"disable interference checking" exists at any layer.

## C7 — The join lane: how declared contact unblocks the banked unions

Design sketch only (implementation is banked; this section exists so
the refusal migration in C8 has a stated target):

1. **The boolean's curved coplanar-lump arm** (#175 finding 1's exact
   wall, `boolean/vtxfac.rs`): where a curved sector's normal is
   plane-parallel at a pierce site today's code refuses
   `CurvedBooleanUnsupported` (the CURVED-DESIGN C7/OQ5 frontier).
   With a verified
   `Tangent`/`Rest` declaration on the face pair, classification
   descends to second order (CURVED-DESIGN C7's sector trilean) with
   the declaration bridging in-band κ_rel per C4 — the lump verdict
   the planar `eq15_3_lump` computes from `PlaneRelation` comes
   instead from the (declared-backed) relative-curvature sign.
2. **The zip generalizes by carrier kind** (S1 → cosurface): patch
   removal + seam mint on a shared carrier of any kind — the
   CURVED-DESIGN C12.5 cosurface-merge ladder, same
   structural/declared rungs, never numeric. Conformal patches are
   removed as interior; rim seams mint once; volume additivity is
   exact when the patch is the full engagement (the S1 dyadic
   precedent).
3. **Post-union edge descriptions**: a G1 rim whose jet is determinate
   must carry `TangentIntersection` (OQ7's two-level rule, unchanged);
   a rim with isolated κ_rel zeros is a mixed per-sample
   classification — conservatively unenforced, exactly today's
   ratified posture — and the conventional `MappedCurve` (the shared
   profile circle both sweeps carry) remains its honest description.
   The wedge predicate already admits wedge = π as the legal smooth
   seam; no tier change is needed for tube chains. A declared-`Tangent`
   join whose result carries material on one side of the locus emits a
   wedge-0/2π edge instead — governed by D1 tier 3's declared
   second-order wedge arm (#131 ruling: legal iff declared and
   jet-determinate, osculation escalates; implementation #941) — and
   the doubled form (material both sides) is F2's
   coincident-distinct-edges class; the join-lane spec grows that arm
   before `Tangent` joins ship.

What stays refused even after this lane ships: undeclared touching
(by law, forever), osculating/in-band pairs (escalate), and any
carrier pair the cosurface ladder has no arm for (typed, per class —
never wholesale).

**Sibling deliverable (binding on the C7 implementation spec;
ASSEMBLY-DESIGN A5, 2026-08-10):** the same census + per-class
verification substrate must also open as an **at-rest door** — an
assembly at rest needs verification with no boolean, i.e. no zip.
The join lane alone leaves touching assemblies unvalidatable (the
#328 scoping trap); the M9 spec adopts both doors deliberately.

## Worked examples (each through C1→C6)

**Ball-and-socket** (#161 §2a). Socket authored by subtracting the
ball's own sphere (shared `GeomSource`) or declared `Rest`. Census:
rung 2/3 conformal candidate; verification: same center/radius
(structural or non-contradicted), senses opposed (socket face sense
opposes the ball's — outward normals negate), chart overlap = the
seated cap, area definite ⇒ `PatchContact`. Gap g = R − r ≡ 0
structurally. M10: nothing to propagate on the contact itself;
perturb r and the pair becomes a `Fit` with g = R − r linear — the
r → R conformal limit is entered by *declaration change*, never by
numeric drift (the C2 invariant closing #161 §2c's failing regime).

**Press-fit pin** (#161 §2b). Pin r_p, bore r_b, r_p > r_b, coaxial
by shared axis datum; declared `Fit { gap: r_b − r_p < 0 }`.
Verified: coaxial structural, g = r_b − r_p definite-negative,
|g − g₀| zero. Gates skip the pair (recorded); mass props refuse or
subtract π(r_p² − r_b²)·L on opt-in; M10 asserts
g ∈ [−δ_max, −δ_min] over the tolerance box with dg/dr_b = +1,
dg/dr_p = −1 exactly.

**Two-peg plate** (the considered-not-built demo). Plate P with two
pegs, plate Q with two bores sharing the pegs' cylinder carriers
(structural) plus the mating plane face. Declarations: one planar
`Rest` (S1's shipped class) + two cylindrical `Rest`s (this doc's).
Census: three `PatchContact`s — cylindrical band overlaps certified
in each cylinder chart ((u,v) rectangle intersection, planar
machinery), plane patch as S1. Union: the C7-lane zip removes all
three patches as interior, bore walls vanish (full engagement),
volume exactly additive. This vocabulary is precisely what the demo
waits on; the demo un-blocks with the implementation milestone, not
with this doc.

**The lily's G1 tube chain** (#175 findings 1–2). Two torus-segment
tubes, equal minor radius ρ, spines meeting G1 at a shared point;
end discs exactly coincident; walls tangent along the shared
cross-section rim (normals are the rim's radial directions — exact).
Declarations, authored by the turtle constructor itself (the
PATHS `.fillet(r)` precedent — the constructor knows the tangency
it built): planar `Rest` on the disc pair + `Tangent` on the wall
pair along the rim. Verification: disc conformality structural;
wall tangency definite at first order everywhere; κ_rel =
|κ₁(φ) − κ₂(φ)| (spine-curvature projections at meridian angle φ)
definite at generic φ and exactly zero at the two neutral meridian
points of a planar stem — bridged residue, C4's `Tangent` table.
Union: disc patch zipped (planar arm), rim minted as the wedge = π
smooth seam, `MappedCurve`-described (mixed jet). The flower∪stem
torus×sphere case is the same story with a `Tangent` declaration at
the pedicel–lantern contact circle — finding 2's wall is the same
arm as finding 1's. What this doc does NOT give the lily: the
transverse curved×curved SSI its finding 2 also wants for *piercing*
unions — that is the banked cyl×sphere/germ-chord lane, not contact.

## C8 — OQ5 disposition and refusal migration

**Ratified (this doc's C1–C7; #178, 2026-08-04):** the contact census
classification and its invariants (C1); the representation boundary,
identity lemma, and decision procedure (C2); the record granularities
(C3); the declaration vocabulary, per-class verification tables,
storage/replay/persistence semantics, and failure modes (C4); the
signed-gap object and the M10 contract (C5); interference-fit
semantics (C6); the join-lane shape as the stated implementation
target (C7). Their ratification CLOSED OQ5: the deferral's condition
("waits for its own design doc") is discharged by this document
(CURVED-DESIGN's OQ5 entry records the closure).

**Explicitly still open, with owners:** NURBS↔analytic same-locus
recognition (D7 adoption work); kinematics (contact records are
geometric — DOF/mate solving is Band 3's SE(3) story, deliberately
absent here). Discharged since ratification: implementation
sequencing — the C7 join lane plus the A5 at-rest census door is
**M9**; and the assembly/mate layer's node vocabulary, which landed
as ASSEMBLY-DESIGN A3 (C4 binds the declaration *shape*, A3 is its
second home).

**Refusal migration (text-level; behavior unchanged until the lane
ships) — TRACKED AS #459:** the `boolean/vtxfac.rs` C7/OQ5 comment
and the census `CensusUnsupported` boundary text update to cite this
document's classes and name the recourse ("a declared Tangent/Rest
contact — vocabulary CONTACT-DESIGN C4, implementation M9") instead
of citing a deferral that no longer defers;
`CurvedBooleanUnsupported` at tangent-contact sites keeps its type
and gains the same pointer. *(Originally scoped to ride any touching
PR. No PR touched those sites in the eight days after ratification,
so it is issue-tracked instead — the rider mechanism did not work
for a change nothing else needs.)* When the C7 lane ships, each arm retires
per class through the CURVED-DESIGN C5 dispatch-table discipline —
incrementally, never wholesale, exactly as
`CurvedBooleanUnsupported` retired by table arm through M5.

**Invariant (C8).** Ratification of this document changes no verdict
on any body: every currently-refusing configuration keeps its typed
refusal, with better prose; every currently-certifying configuration
certifies identically. The design/behavior boundary is auditable by
that statement alone.
