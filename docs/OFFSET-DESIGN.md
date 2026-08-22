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

**The fork**: is a sealed shell (a) a third void source, revising
that sentence, or (b) definitionally boolean-family —
`shell(B, t) := B − offset_inward(B, t)`, so the ratified sentence
stands because shell IS a boolean? **Recommendation: (b)** — it
keeps the invariant's justification intact (void bookkeeping stays
in one place), matches the verb's semantics exactly, and the open-
faces variant factors through the same subtraction followed by the
rim surgery (a composed Euler sequence on a clone, decided-then-
mutated per the M6-1 surgery pattern, postcondition validated once
at the end). Honest counterargument to (b): it makes shell's
availability contingent on boolean operand support — a NURBS-walled
body cannot be shelled until the curved-boolean lanes (Wave 2 /
frontier (d)) cover its kinds, whereas a native shell construction
could in principle run ahead of the booleans. That coupling is
real; it is also honest about what the kernel can certify, and the
teapot's revolve body is analytic-kind, inside what Wave 2 covers.

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
