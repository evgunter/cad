# M2 Work Order — Analytic Geometry, Extrude/Revolve, Tessellation, STL

**Status: DRAFT — awaiting Evan's ratification.** Milestone plans are
fundamental forks under the PR #20 process rule: no implementation
until sign-off.

Read `DESIGN.md` first (D1 incl. the M1 conventions/tiers, D2, D3, D4,
D8, D9, Q1). M2's goal (Roadmap): **analytic curves/surfaces;
extrude/revolve from polyline+arc profiles; tessellation; STL export** —
the first "it's a CAD kernel" milestone, verified via exported meshes
in external viewers (demo viewer stays deferred).

Primary sources, read 2026-07-16 into
`<main-checkout>/references/notes/`: Mäntylä ch. 12
(`mantyla-ch12-modeling-algorithms.md` — sweep op-sequences, gluing,
closed-profile revolve) and ch. 13
(`mantyla-ch13-geometric-algorithms.md` — Newell face equations,
classification predicate inventory, divergence-theorem integrals).
Implementer/reviewer prompts cite the notes, never the scan.

## Source grounding (the facts the plan stands on)

- **Translational sweep** (ch. 12.3): per profile loop, n struts +
  n side-quad `mef`s + one closing `mef`; the swept face survives as
  the cap. **Every `lmev` in the book's sweeps is the `he1==he2` strut
  case that the Program 11.6 erratum breaks** — port only against our
  corrected ops, and re-derive the six `[mirror-check]`-flagged
  navigation sites under CCW (the notes mark each).
- **Rotational sweep** (ch. 12.5): open wires sweep to solids; closed
  profiles open the lamina into a pseudo-wire, sweep, then close the
  seam with a **same-shell** `kfmrh` (the genus supplier) + a
  `loopglue` zip built from mekr/kev/mef/kef. Cross-shell `kfmrh`
  stays M3 (only `glue` — joining two solids — needs it). Known
  degeneracy: profiles touching the axis produce broken pole fans
  (the book's ball is knowingly wrong; the fix is Problem 12.2's
  collapse-face-to-vertex).
- **Face equations** (ch. 13.1): Newell's method; the
  translate-to-origin accuracy fix (Problem 13.1) is our default, not
  an exercise. Plane equations are *derived caches* — Euler ops never
  fill them — which is exactly D4 ¶2's certified-residual shape.
- **Classification primitives** (ch. 13.2): the book's tolerance model
  (per-object EPS, three-way `comp`) is what D4 rejects; the notes
  tabulate every raw comparison as a predicate site, including the
  dimensional abuses to avoid (length-ε on dimensionless parameters;
  a length² determinant vs. a length ε — wants a derived angular
  band, θ = ε/r).
- **Integral properties** (ch. 13.3): V = (1/6)·Σ det[p₁, pᵢ, pᵢ₊₁]
  fanned over every loop; rings subtract via orientation. Verified in
  the notes: the printed formula yields **+V under our CCW convention
  uncompensated** — negative volume is a free global orientation
  invariant.

## Working agreements (inherited, with the PR #20 revision)

- One implementer + one adversarial e2e reviewer + one fix pass per
  PR; reviewers write and run real consumer programs; reviewer suites
  promoted into CI per the standing convention.
- High-confidence design PRs self-merge with full writeups (Evan
  reviews retroactively); **fundamental forks wait** — in this plan,
  PR 2 (profile format) and the pole-policy decision in PR 5 are
  flagged as forks; the rest are expected to be self-merge grade.
- Branches `ev/m2-<n>-<slug>`, stacked serially, merge commits only;
  orchestrator log `docs/M2-LOG.md` (L-numbering continues from M1).
- D9 charter throughout. Geometry code is generic over `Real`; every
  topology-determining comparison goes through the Q1 trilean
  predicates — **M2 is where predicates first fire in anger, so the
  K-value experiments (M0 carry) run here**.

## PR sequence

1. **`geom-curves` + `geom-surfaces` crates: analytic evaluators**
   *(self-merge grade; conventions documented once)*. Closed enums per
   D3: `Curve3 { Line, Circle, Nurbs(placeholder) }`,
   `Surface { Plane, Cylinder, Cone, Sphere, Torus,
   Nurbs(placeholder) }`. Evaluators + first/second derivatives
   generic over `Real` (Dual-compatible by construction). Decisions
   documented once, M1-conventions style: parameterization of each
   kind (ranges, periodic domains, radians; seam placement is
   conventional data per D2), **curve entities are complete loci**
   (full circle, infinite line) — an edge's bounds derive from its
   vertices via the ratified `he_plus` forward contract (increasing
   parameter runs start→end of `he_plus`); range reduction for
   periodic evaluation needs `Real::{floor, rem}` (+ `copysign`) —
   the M0-watchlist Real-surface additions land here, and the **L7
   allowlist moment** (first legitimate `Real +` bound) likely
   arrives; refine the CI discipline grep to an allowlist as ratified.
   Watchlist items landing with their first consumer:
   `project`/`reject` with documented association order;
   **orthonormal-basis-from-normal is a value-branch** → per the M0
   review it enters as a predicate-guarded or branchless construction,
   a design point in this PR's writeup.
2. **Profile format + arc generator** *(DESIGN FORK — wait for
   Evan)*. The DESIGN.md deferred item "profile/sketch input format
   (M2)". Proposal to present: a 2-D profile is data (D8-compatible —
   recipes will carry it): a sketch plane (`Affine3` placement) plus
   loops of segments `{ LineTo, ArcTo { center/radius form TBD } }`;
   winding = topology conventions (outer CCW, holes CW in sketch
   plane); closed loops only at M2 (open wires deferred with
   rotational-wire sweeps); simplicity (non-self-intersection)
   required and **checked with trilean predicates, fail-loud** — the
   first real K-experiment data source. Arc discretization does NOT
   happen here (profiles stay exact; arcs become circle-carrier edges
   that sweep to cylinder/cone patches) — the ch. 12 arc *generator*
   is superseded by exact analytic carriers; note the circle-closure
   caveat (a full-circle loop needs ≥ 2 vertices under our edge
   model).
3. **Geometry attachment + face equations + tier-3 start**
   *(self-merge grade)*. The M1 ops' documented signature slots get
   real geometry: sweep-level construction supplies curve/surface
   keys (ops accept geometry parameters; `Placeholder` variants
   retire). Newell plane equations (translate-to-origin default) as
   derived caches with **D4 ¶2 residual certification** — the first
   certified caches (residual: max vertex distance from the fitted
   plane ≤ ε); the validator's geometric pass begins (tier 3:
   per-face residual; planar-face pcurves are trivial/rectangular at
   M2 — general pcurve machinery deferred to M3 with real SSI).
4. **Extrude** *(self-merge grade)*. Translational sweep re-derived
   under CCW from the ch. 12 notes (all mirror-check sites
   hand-verified); plane caps; plane side-faces for line segments,
   **cylinder patches for arc segments**; certified caches
   throughout. Acceptance: extruded L-profile and extruded
   profile-with-hole (genus 0 with rings), tier-1 after every op,
   tier-2 + tier-3 + component-E–P at rest.
5. **Revolve** *(mostly self-merge grade; ONE FORK — pole policy)*.
   Rotational sweep: partial revolutions (planar seam faces) and full
   revolutions (seam via same-shell `kfmrh` + `loopglue`); sphere/
   torus/cone patches from line/arc segments. **Fork for Evan:
   profiles touching the axis** — recommendation: typed error at M2
   (pole-touching profiles rejected; spheres arrive via M3 primitives
   or the collapse-face fix when demanded), with the honest
   counterargument that revolved spheres are natural test shapes.
   Acceptance: washer (annulus revolve, torus-free genus 1), partial
   revolve, full revolve of an offset square (torus-like, genus 1).
6. **Tessellation** *(self-merge grade)*. Per-face triangulation:
   planar faces (with rings) via CDT in the face plane — `spade` per
   the ratified crate table; curved analytic faces via UV-grid
   sampling + CDT in parameter space. **Chordal tolerance is a new,
   per-call display parameter — deliberately NOT the kernel ε**
   (documented distinction; D4 ¶1 unaffected). Watertightness across
   shared edges: shared-edge chord points computed once from the
   edge's curve (deterministic, both faces consume the same points).
   Orientation: outward normals fall out of the loop conventions.
7. **STL export + mass properties + M2 exit** *(self-merge grade)*.
   Binary + ASCII STL from the tessellation (D9: byte-identical
   output for identical inputs). Volume/surface area by the ch. 13
   divergence formulas over the *exact* B-rep (analytic per-face
   contributions where closed-form, else certified quadrature —
   scope-boxed to what the acceptance shapes need); the +V global
   orientation invariant enters the geometric validator. Acceptance:
   volumes/areas of the acceptance shapes vs. closed forms within
   certified bounds; exported STLs verified watertight/manifold by an
   external checker in CI if a suitable tool is available, else by
   our own mesh validator. **K-experiment report**: multi-ε predicate
   telemetry gathered across M2's suites → a written recommendation
   (keep K = 10 or revise) closing the M0 carry.

## Deliberately not in M2

- Booleans, intersections, `glue`/cross-shell `kfmrh`, splitting —
  M3 (ch. 14–15 read then).
- General pcurves and SSI — M3+; NURBS depth — M5.
- Feature DAG / recipe layer — M4 (profiles are built as data now so
  M4 can adopt them wholesale).
- Sketch constraint solver — M6 (profiles are programmatic data).
- Error-propagation demos beyond what the generic evaluators give for
  free — M6.
- Viewer — deferred (STL + external viewers per roadmap).

## Exit criteria

Extruded and revolved parts (incl. a ringed profile and a genus-1
revolve) built end-to-end from profile data through public ops only;
tier-1 validated after every op, tier-2 + tier-3 (residual
certification + orientation/volume invariant) at rest; watertight STL
exports verified externally; mass properties match closed forms within
certified bounds; CI green at ε ∈ {1e-6, 1e-9, 1e-12} + interval lane
(geometry evaluators instantiate at `Interval`); the K report
delivered and its outcome ratified into DESIGN.md's Q1 residue; all
new conventions (parameterizations, profile format, chordal-tolerance
distinction) ratified into DESIGN.md at exit.
