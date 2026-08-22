# Offset & shell — the Q8 design conversation

**Status: DRAFT — design conversation, awaiting Evan's sign-off**
(VERBS program, Wave 3 gate; DESIGN.md Q8 is the ratified seed).
Proposals O1–O6, each with a firm recommendation. Substrate anchors
were verified on main 2026-08-21; the survey behind them is at the
VERBS orchestrator lane and its load-bearing lines are inlined here.

Vocabulary, defined once (Evan-profile convention): the **offset** of
a surface S at distance d is the normal pushforward
`S_d(u,v) = S(u,v) + d·n(u,v)` — each point moved along the unit
normal, i.e. the boundary of the d-tube on one side. **Shelling**
(hollowing) turns a solid into a thin-walled solid: offset the
boundary inward by the wall thickness, and either keep the hollow
closed (a cavity) or open it by designating faces whose material is
removed, leaving annular rims where the wall's thickness shows.

## O1 — Analytic offsets: mint by struct-update, refuse degeneracies at the door

Analytic kinds are closed under offset (Q8, a D3 payoff): plane →
plane (`origin + d·normal`), cylinder/sphere → radius ± d, torus →
minor ± d, cone → cone (apex slides `d/sin α` along the axis, half-
angle unchanged). Since D2 makes construction struct-literal on
public fields, the mint is a struct-update table — the natural home
is a sibling of `revolve/surfaces.rs::wall_surface`, the existing
central mint switch.

What is genuinely new is the refusal set, owned by the offset door
itself (today's degeneracy gates live at call sites, e.g. the
ring-torus refusal in the fillet battery): `r − d ≤ 0` for
cylinder/sphere/torus-minor inward offsets, the torus crossing
`minor ≥ major` (spindle), the cone offset crossing its apex. Each
is a named margined Q1 predicate over the INPUTS, evaluated before
construction — DESIGN.md:1823 already ratifies this stance for
shell/offset ("same principle applies"); O1 just instantiates it.

**Recommendation**: build exactly this, as the first Wave-3 unit; it
is small (S) and everything later consumes it.

**Self-intersection, and the two doors this design leaves cheap
(added at Evan's #907 note).** The offset definition carries no
self-intersection special case; where d reaches the collapse
threshold, the door REFUSES via the collapse predicate (O3's d vs
1/κ_max, and for solid offsets the clearance margin) — loud, never a
silently looped surface. Two future verbs are then cheap by
construction, and naming them now is what keeps them cheap:

- **Trimmed offset ("remove the loop the offset created")**: because
  the spec is intensional, the untrimmed offset remains the
  certification target and loop removal is a TOPOLOGY operation — a
  self-intersection trim (self-SSI + region classification)
  consuming the same `Offset(S,d)` spec — a separate verb, not a
  mode of this one. The collapse predicate's failing samples
  localize where loops form, which is exactly the trim's seed. No O1
  decision forecloses it.
- **Solved-distance offset ("offset by the distance that causes a
  tangency here")**: d is a plain stored parameter, so a derived or
  solved d composes through the recipe layer (D8 expressions)
  without new offset machinery. The near-term spelling is the
  user-supplied d DECLARED as intentional tangency through the
  C7 declared-contact vocabulary — the same door M9 builds, which is
  what makes the resulting kiss certifiable rather than an
  undeclared-tangency refusal. The certified root-solve for d*
  itself ("zero this clearance margin") is precisely the
  margin-over-a-parameter-box machinery M10 owns; building it
  earlier would duplicate that, so it banks until a consumer
  demands it. (If one arrives sooner: the solve is 1-D and the
  margin is already a certified monotone-in-d quantity near the
  kiss, so the unit is small.)

## O2 — The approximating surface: lift the EdgeCurve triple one dimension

The offset of a NURBS is not a NURBS (normalizing n(u,v) introduces
the square root that breaks rationality — Q8's canonical
obstruction), so the kernel fits one. Q8's own template is "exactly
mirroring fitted intersection curves", and that machinery is a
triple: intensional description (`EdgeGeometry`), spec+fit pair
(`EdgeCurveSpec`), certified product with private fields
(`EdgeCurve`, uncertified unrepresentable). Nothing surface-shaped
exists — `NurbsSurface` carries knots/control/weights only, and the
absence is deliberately marked at both loft sites ("NO
approximating-surface machinery anywhere downstream").

**Recommendation** — mirror the triple, one dimension up:

- `SurfaceDescription<T>`: the intensional layer. First inhabitant
  `Offset { base: SurfaceRef, d: T }`; the canal blend (DESIGN
  frontier (f)) becomes its second inhabitant when a consumer
  arrives — this conversation is what gives that parked machinery
  its first caller, which is the sequencing argument for doing O2
  properly rather than special-casing offsets.
- `SurfaceSpec<T>` (description + fitted `NurbsSurface` + domain
  window) → certify → `ApproxSurface<T>` with a private certificate,
  so an uncertified approximating surface is unrepresentable — the
  EdgeCurve invariant, lifted.
- **Storage: a seventh `Surface` variant**,
  `Surface::Approx(Arc<ApproxSurface<T>>)`. The D3 closed-enum
  argument runs the same direction it did for the first six: adding
  the variant makes the compiler enumerate every dispatch site, and
  each site must then SAY what it does with an approximating surface
  (most delegate to the fitted NURBS; some — dihedral
  classification, census — must consult the description). The
  alternative (a body-level side table keyed by face, mirroring the
  pcurve cache) keeps the enum closed but lets every consumer
  silently treat the surface as a plain NURBS, which is precisely
  the failure D3 exists to prevent. Honest counterargument: the
  variant touches every match in the workspace (a wide, mechanical
  diff), and `Surface` is `Clone`-heavy — the `Arc` keeps that
  cheap.

## O3 — The certificate: C2 lifted, with two new meters the fit needs

CURVED-DESIGN C8 already states the certificate shape: "C2's lifted
one dimension — schedule over (u,v), hull bounds per patch,
envelope-system residuals as the on-locus test". The SSI
certificate's two-limb pattern (on-locus max + hull sup over a span
schedule) is the code shape to copy. The residual claim is
`sup ‖ S_fit(u,v) − (S(u,v) + d·n(u,v)) ‖ ≤ ε_precision` (D4's
two-tolerance split: this is ε_precision, not ε_input), with the
ring/interval composite machinery (`SurfaceResidual`,
`surface_curve_residual`, patch Hessian hull bounds) as ingredients.

Two meters do not exist and are prerequisites, not details:

- **A certified lower bound on ‖S_u × S_v‖** (surface regularity).
  `speed_lower_bound` is curve-only. The offset is undefined exactly
  where the normal degenerates, so the fit door must refuse — not
  degrade — on a patch whose regularity cannot be bounded away from
  zero. Built from the spline hull machinery.
- **The offset-collapse predicate**: d vs the principal-curvature
  bound (the inward offset folds where d reaches 1/κ_max — the same
  fact as the fillet battery's spine-regularity predicate, one
  dimension up). Ingredients exist (third-order jets, Hessian hull
  bounds per patch); the named margined predicate does not.

**Recommendation**: both meters land in the same unit as the fit;
the certificate is two-limb per C8. The fit engine itself is the
NURBS Book's §9.4 stack (A9.4 grid interpolation + A9.10
approximate-to-tolerance) built in-house — the audit already
established curvo has no fitting stack to borrow.

## O4 — What shell IS: two definitions, one fork for Evan

The survey's structural finding: **an open shell is unrepresentable
in this kernel, by construction, not by check** — `Edge` is born
with exactly two half-edges, and tier 1's watertightness, the
per-shell Euler–Poincaré gate, c = 1, and +V all assume closure.
D1 is manifold-first and names sheet/wire bodies as the only
sanctioned trigger for revisiting that. Shell does not need to
trigger it:

- **Shell with designated open faces** (the teapot body, every Klein
  wall): the result is an ordinary CLOSED thin solid — the removed
  faces' material is replaced by annular rim faces where the wall
  thickness shows. The Klein bottle already builds exactly these
  bodies by hand (7 `r ± t/2` pair sites, sign rules living in
  prose); shell mechanizes that bookkeeping. Genus rises, nothing
  opens. Single shell, all invariants hold.
- **Shell with no opening** (a sealed hollow): the result is a
  TWO-SHELL solid — outer boundary plus cavity. Multi-shell solids
  are already legal (voids, outer-vs-cavity derived from volume
  sign). But DESIGN.md:348 ratifies "sweeps emit single-shell
  bodies; **voids are born only from booleans**".

**The fork, refined after Evan's #907 note** (the "voids are born
only from booleans" sentence is revisable — it predates booleans
existing, and the original reasons are not strongly held). The
definition and the execution separate, and separating them answers
the performance question:

- **Definition**: `shell(B, t) := B − offset_inward(B, t)` —
  semantically boolean-family, matching the verb's meaning exactly.
- **Execution: the sealed case NEVER runs the general boolean
  pipeline.** When the clearance/collapse predicate certifies the
  inner offset strictly inside B (which shell's validity already
  requires — d below the reach), the two boundaries provably do not
  cross, so there is nothing for SSI, the crossing census, or the
  classification walk to do; running them would be pure waste, and
  worse, the general path's containment examination is extent-box
  coarse (#750 — a non-convex container cannot certify), so routing
  through it would REFUSE bodies the construction itself has
  already proven nested. The sealed shell executes as the
  DEGENERATE no-crossing arm: direct cavity insertion through the
  boolean's own void-insertion door, factored so it is callable
  without the SSI pipeline, with strict containment certified from
  the offset construction (d vs reach — a by-construction margin,
  not a post-hoc box test). Cost = offset mint + certification +
  one structural insertion.
- **The invariant, restated at its real value**: what the ratified
  sentence protects is that cavity bookkeeping has ONE home. The
  ratified form (this round, with Evan's sweep note) — **every
  cavity is born through the shared void-insertion door** — which
  THREE producers satisfy: boolean subtraction, shell, and **the
  full revolve of a holed profile** (Evan's #907 addition), defined
  as the composition `revolve(outer) − revolve(hole-as-outer)` the
  current `FullRevolveHoles` error text already points at, and
  executed through the same degenerate no-crossing arm — the hole's
  swept boundary provably touches nothing, which is the very fact
  the old refusal cited. That unit is VERBS-PLAN's existing RING
  row, now redefined: it FACTORS the void-insertion door
  callable-without-SSI (becoming the door's first consumer, ahead
  of shell) and retires `FullRevolveHoles`. The DESIGN.md:348
  bullet is revised in this PR accordingly.
- The open-faces variant runs the same offset construction and then
  rim surgery (a composed Euler sequence on a clone, decided-then-
  mutated per the M6-1 pattern, validated once) — no boolean
  machinery there either.

Residual honest coupling: a NURBS-walled body still cannot be
shelled until the approximating-surface machinery covers its
offset (O2/O3) — but that is the offset's own gate, not the boolean
operand gate; the degenerate-arm execution removes the Wave-2
dependency the earlier draft carried. The teapot needs only
analytic offsets either way.

## O5 — Validator posture: re-derive per face, same as edges

The validator never trusts a stored certificate — tier 3
re-certifies every edge on every call. An `ApproxSurface` face gets
the same posture: re-derive the two-limb certificate over the (u,v)
schedule per validation call. That is a real cost (a grid per face
where edges pay a line schedule); the alternative — trust-on-read
for surfaces only — would make the surface certificate the one
uncached... the one UNCHECKED claim in tier 3, which is not a
posture this kernel has anywhere. **Recommendation**: re-derive,
measure, and if the grid cost bites, that is a perf-lane finding
with its own box, not a design change. Note also the existing
exact/approximate divergence: NURBS-adjacent edges are exempt BY
KIND from dihedral classification (contact mark stays Unmarked) —
Approx faces inherit that exemption, and narrowing it is its own
future conversation, not smuggled in here.

## O6 — Sequencing and the demo gates

Unit cut implied by O1–O5 (re-cut at ratification is expected):
O1 mint+refusals (S) → the two meters + fit + certificate (L, the
program's hardest unit) → `Surface::Approx` integration (L, wide
mechanical enumeration) → shell-as-boolean + rim surgery (L) → the
Utah teapot demo (the verb's designated demo) with the Klein
bottle's hand-offsets retired to `shell` calls as the second
consumer.

Known downstream gates the demo will hit, recorded now so they are
findings and not surprises: a closed two-shell CURVED body refuses
STEP export (`CurvedShellClassification` — the outward/void
classifier is a planarity identity with no curved counterpart);
rational-patch quadrature flooring (#453/#390) means a fitted
(likely rational) offset wall may not certify volume until that
lane lands; the area enclosure is unmetered (#870) and shell is an
area-and-thickness verb. None blocks the verb; each is named in the
demo's findings list per the demo-purpose rule.
