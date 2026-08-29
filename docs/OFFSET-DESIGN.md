# Offset & shell — the Q8 design conversation

**Status: RATIFIED (Evan's sign-off on PR #907). IMPLEMENTED across
VERBS Wave 3: OFF-A the mint (#994), OFF-B the meters + fit +
certificate (#1003), OFF-C `Surface::Approx` (#1012), OFF-D the
face-replacement door (#1043) and the shell verb (#1048), with the
Utah teapot as the designated demo (#1078).**
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
public fields, the mint is a struct-update table — a sibling of
`revolve/surfaces.rs::wall_surface`, the existing central mint
switch; it lives at `geom-brep/src/offset.rs`.

What is genuinely new is the refusal set, owned by the offset door
itself (today's degeneracy gates live at call sites, e.g. the
ring-torus refusal in the fillet battery): `r − d ≤ 0` for
cylinder/sphere/torus-minor inward offsets, the torus crossing
`minor ≥ major` (spindle), the cone offset crossing its apex
*(the cone item resolved at OFF-A per this spec's own
derive-and-drop rule: nothing STORED degenerates — the apex
translates finitely — so no door predicate exists; the real
question is the consumer-side v-window crossing the shifted apex,
which is `offset_apex_window` at the face-replacement door,
`topo/src/replace_face.rs`)*. Each is a named margined Q1
predicate over the INPUTS, evaluated before construction —
DESIGN.md:2088 already ratifies this stance for
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
(`EdgeCurve`, uncertified unrepresentable). `NurbsSurface` carries
knots/control/weights only and claims nothing about a locus beyond
itself — which is why the loft sites can say "NO
approximating-surface machinery anywhere downstream" and mean it; a
surface that DOES claim one needs the triple of its own.

**Recommendation** — mirror the triple, one dimension up (built at
OFF-B/C, `geom/src/surfaces/approx.rs`):

- `SurfaceDescription<T>`: the intensional layer. First inhabitant
  `Offset { base: Arc<NurbsSurface<T>>, d: T }`; the canal blend
  (DESIGN frontier (f)) becomes its second inhabitant when a consumer
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

Two meters are prerequisites, not details (both built at OFF-B,
`geom-brep/src/offset_meters.rs`):

- **A certified lower bound on ‖S_u × S_v‖** (surface regularity).
  `speed_lower_bound` is curve-only. The offset is undefined exactly
  where the normal degenerates, so the fit door must refuse — not
  degrade — on a patch whose regularity cannot be bounded away from
  zero. Built from the spline hull machinery: `patch_regularity`,
  metered by `offset_normal_floor`.
- **The offset-collapse predicate**: d vs the principal-curvature
  bound (the inward offset folds where d reaches 1/κ_max — the same
  fact as the fillet battery's spine-regularity predicate, one
  dimension up). Ingredients exist (third-order jets, Hessian hull
  bounds per patch); the named margined predicate is
  `offset_curvature_headroom` over `patch_collapse`.

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
  sign). The M2 convention this has to argue against reads "sweeps
  emit single-shell bodies; **voids are born only from booleans**"
  — the bullet DESIGN.md:387 now carries in its revised form.

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
  as the composition `revolve(outer) − revolve(hole-as-outer)` and
  executed through the same degenerate no-crossing arm — the hole's
  swept boundary provably touches nothing, which is the very fact
  the `FullRevolveHoles` refusal cited before VERBS-RING (#933)
  retired it. RING is that unit: it factored the void-insertion
  door callable-without-SSI, the door's first consumer, ahead of
  shell. DESIGN.md:387 carries the revised bullet.
- The open-faces variant runs the same offset construction and then
  rim surgery (a composed Euler sequence on a clone, decided-then-
  mutated per the M6-1 pattern, validated once) — no boolean
  machinery there either.

Residual honest coupling: a NURBS-walled body still cannot be
shelled — not for want of the approximating-surface machinery
(O2/O3 ship), but because `Approx × anything` has no C5 route arm,
so the face-replacement door refuses typed on a fitted face's
intrinsically-described boundary. That is the offset side's own
gate, not the boolean operand gate; the degenerate-arm execution
carries no Wave-2 dependency. The teapot's path is analytic-only
either way.

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

The unit cut implied by O1–O5, as executed: O1 mint+refusals (S,
OFF-A) → the two meters + fit + certificate (L, the program's
hardest unit, OFF-B) → `Surface::Approx` integration (L, wide
mechanical enumeration, OFF-C) → the face-replacement door, then
shell-as-boolean + rim surgery (L, OFF-D) → the Utah teapot demo
(the verb's designated demo). The Klein bottle's hand-offsets are
the second consumer and have NOT retired: every one of its wall
pairs is revolved, so each waits on the plane×torus and
cone×cylinder section arms (#1057).

Downstream gates, live: a closed two-shell CURVED body refuses
STEP export (`CurvedShellClassification` — the outward/void
classifier is a planarity identity with no curved counterpart),
pinned at `crates/sweep/tests/verbs_shell.rs`; rational-patch
quadrature flooring (#453/#390) means a fitted (likely rational)
offset wall may not certify volume until that lane lands; the area
enclosure is unmetered (#870) and shell is an
area-and-thickness verb. None blocks the verb; each is named in the
demo's findings list per the demo-purpose rule.
