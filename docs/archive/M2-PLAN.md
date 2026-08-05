# M2 Work Order — Analytic Geometry, Extrude/Revolve, Tessellation, STL

**Status: RATIFIED** (the #24 conversation, 2026-07-16/17 — all forks
pre-resolved there); **amended** 2026-07-19/20 via #32 (PR 6:
entity back-references incl. Vertex keys, per-face patch
separability, Appearance fully deferred) and the corrections noted
inline below.

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
  reviews retroactively); **fundamental forks wait**. All forks
  identified in this plan were resolved with Evan in the #24
  conversation (2026-07-16): profile format (bulge chain, ≥2-vertex
  closed carriers, winding invisible), revolve pole policy (axis
  contact fully supported, partial/full split, sliver-band
  rejection), D2 intensional EdgeGeometry landing at M2 (option (a)),
  tessellation's certified-conservative export promise, and
  no-automatic-face-merging — every PR is now self-merge grade,
  subject to the standing rule that anything fork-shaped discovered
  during implementation still waits.
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
2. **Profile format + arc generator** *(fork RESOLVED with Evan in
   the #24 conversation, 2026-07-16 — now self-merge grade with the
   full writeup)*. The DESIGN.md deferred item "profile/sketch input
   format (M2)". **Ratified**: (a) **vertex chain with bulge** — a
   loop is a list of `{ pos: Point2<T>, bulge: T }` (bulge =
   tan(θ/4) of the arc to the next vertex; 0 = line; DXF-compatible);
   chosen for ZERO representation-consistency conditions (the D2
   peer-representation lesson applied at the input boundary — command
   lists carry radius/endpoint redundancy, carrier-interval segments
   are the extensional bug farm as input); constructor sugar computes
   bulges from human-friendly forms; closure by construction.
   (b) **Closed carriers split into ≥ 2 vertices** (no single-edge
   full circles): periodic carriers with identical endpoints
   under-determine the parameter interval, so full-period edges would
   need stored intervals, weakening the vertices-derive-bounds rule;
   bulge cannot express θ = 2π anyway — representation and topology
   agree. Cost: one seam vertex per hole; revisit on annoyance.
   (c) **Winding is invisible to users**: nesting derives from
   containment (trilean point-in-loop), winding is canonicalized
   internally to the topology conventions — no winding concept in the
   API surface, no winding errors. Fail-loud, trilean-checked:
   non-simple loops (closed-form line/arc pairwise tests) and
   nesting deeper than outer + holes (typed error at M2). Closed
   loops only at M2; sketch plane = `Affine3` placement; `Profile<T>`
   scalar-generic (D8: coordinates are parameter expressions) — the
   first real K-experiment data source and the first end-to-end Q1
   exercise. The ch. 12 arc *generator* (polyline discretization) is
   superseded by exact circle carriers; discretization exists only at
   tessellation time.
3. **D2's intensional `EdgeGeometry` + face equations + tier-3 start**
   *(scope ratified with Evan in the #24 conversation, 2026-07-16 —
   option (a): the intensional layer lands NOW, not as an M3
   retrofit)*. The M1 ops' documented signature slots get real
   geometry, and edge geometry is stored per D2's ratified rule:
   the `EdgeGeometry` sum type (`Intersection { s1, s2, witness }` /
   `MappedCurve { source, map }` / `Seam`) with carriers (Line/
   Circle) demoted to **certified caches** against the description.
   The **dihedral classification predicate** (the material
   wedge-angle predicate, tier 3, arriving on schedule) classifies
   each constructed edge: transverse ⇒ `Intersection` (intrinsic —
   prefer-intrinsic rule active from the first native construction;
   the sweep supplies only the witness); smooth join ⇒ conventional
   (`MappedCurve`); sliver dihedral ⇒ typed error at construction
   (D2's ratified text). Certification is cheap by D3: all M2 pairs
   (plane×plane, plane×cylinder, revolve meridian pairs) are
   closed-form, so cache-vs-both-surfaces residuals ≤ ε are directly
   evaluable. Newell plane equations (translate-to-origin default)
   as certified caches likewise; the validator's geometric pass
   begins (tier 3: per-face and per-edge residuals; planar-face
   pcurves trivial/rectangular at M2 — general pcurve machinery
   deferred to M3 with real SSI).
4. **Extrude** *(self-merge grade)*. Translational sweep re-derived
   under CCW from the ch. 12 notes (all mirror-check sites
   hand-verified); plane caps; plane side-faces for line segments,
   **cylinder patches for arc segments**; certified caches
   throughout. Acceptance: extruded L-profile and extruded
   profile-with-hole (*correction 2026-07-19: the original text said
   "genus 0 with rings," which is impossible — a through-holed
   extrusion has genus h; square + 2-vertex hole: v−e+f−r =
   12−18+8−2 = 0 ⇒ g = 1; independently confirmed in the PR 4
   review for h ∈ {1,2,3} — see M2-LOG*), tier-1 after every op,
   tier-2 + tier-3 + component-E–P at rest.
5. **Revolve** *(fork RESOLVED with Evan in the #24 conversation,
   2026-07-16 — axis contact fully supported; the original
   reject-axis-contact recommendation was withdrawn as it would have
   limited revolve to ring solids)*. Rotational sweep with the
   partial/full case split: **partial revolutions (θ < 2π)** — axis
   contact is unproblematic; axis-lying edges/vertices become
   ordinary boundary entities shared by the start/end wedge faces;
   **full revolutions (θ = 2π)** — seam via same-shell `kfmrh` +
   `loopglue`, with two axis-contact classes handled as first-class
   case analysis (Mäntylä Problem 12.2 done properly): edges ON the
   axis are omitted (they sweep to nothing; endpoints land in cap
   interiors), vertices ON the axis collapse to poles/apexes (cone
   apex, sphere poles — the analytic surfaces are regular there;
   only the revolution parameterization is singular, handled by
   tessellation pole fans). **Axis-contact classification is a
   trilean predicate**: exactly on-axis ⇒ the special class; within
   the sliver band but nonzero ⇒ typed error (micro-radius revolve is
   a genuine sliver); beyond ⇒ generic. Plus the half-plane check
   (profile crossing to r < 0 ⇒ typed error). Sphere/torus/cone
   patches from line/arc segments. Acceptance: **the ball**
   (half-disc profile — axis edge omitted, two poles), **the cone**
   (apex), the washer (genus 1), a partial revolve wedge (axis edge
   as ordinary shared edge).
6. **Tessellation** *(self-merge grade)*. Per-face triangulation:
   planar faces (with rings) via CDT in the face plane — `spade` per
   the ratified crate table; curved analytic faces via UV-grid
   sampling + CDT in parameter space. **Chordal tolerance is a new,
   per-call display parameter — deliberately NOT the kernel ε**
   (documented distinction; D4 ¶1 unaffected), and the bound is
   **certified-conservative** (ratified 2026-07-16): analytic
   closed-form sagitta bounds guarantee the mesh lies within the
   requested chordal tolerance of the true surface — but this is an
   export promise, explicitly not a kernel validity invariant.
   **Coplanar side faces at collinear/smooth profile joins are NOT
   merged** (ratified 2026-07-16): faces stay per-segment, the join
   edge is a conventional split per D2's G2-join story, and the
   surface KEY is shared when identical-by-construction.
   Watertightness across
   shared edges: shared-edge chord points computed once from the
   edge's curve (deterministic, both faces consume the same points).
   Orientation: outward normals fall out of the loop conventions.
   **Entity back-references (added 2026-07-19, per DESIGN.md's
   "Beyond the kernel" §Band 1; Vertex keys added 2026-07-20 per the
   PR #32 orchestrator review, Evan-agreed)**: the mesh type carries
   per-triangle source-`Face` keys, per-segment source-`Edge` keys on
   the boundary polylines, and per-polyline-endpoint source-`Vertex`
   keys (chord points are already computed per edge and endpoints
   already are the vertices, so all three associations are free at
   construction — completing the picking chain triangle→face,
   segment→edge, endpoint→vertex; vertex picking is what
   sketch-on-face wants most). Rationale: viewport picking is cheap
   to design in now and painful to retrofit; STL export simply drops
   them.
   **Appearance: deliberately NO M2 artifact (final, Evan
   2026-07-20, superseding both the 2026-07-19 keyed-container text
   and the orchestrator review's option (B))**: M2 ships nothing —
   no container (arena keys are per-lineage and die on rebuild:
   fake durability + consumer-migration debt) and no placeholder
   type either, because the type's correct home is the document
   layer (`editor-core`), which does not exist until M4-era work —
   parking it in `topo`/`mesh` would model the exact layering
   mistake it was meant to prevent. The ratified contract lives in
   DESIGN.md Band 1: display attributes attach via *stable names*,
   in the document layer, from M4; nothing attaches anywhere before
   that (STL/tessellation ignore appearance; demos color
   client-side).
   **Per-face patch separability (added 2026-07-19, per the banked
   content-keyed-cache-transfer principle)**: the mesh type keeps
   per-face triangle patches individually addressable (the
   back-references already give the association) so a future
   content-keyed reuse layer can transfer unchanged faces' patches
   across rebuilds without re-tessellation; no keying machinery in
   M2 — just don't flatten the per-face structure away.
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

Extruded and revolved parts (incl. a ringed profile, a genus-1
revolve, **the ball, and the cone**) built end-to-end from profile
data through public ops only;
tier-1 validated after every op, tier-2 + tier-3 (residual
certification + orientation/volume invariant) at rest; watertight STL
exports verified externally; mass properties match closed forms within
certified bounds; CI green at ε ∈ {1e-6, 1e-9, 1e-12} + interval lane
(geometry evaluators instantiate at `Interval`); the K report
delivered and its outcome ratified into DESIGN.md's Q1 residue; all
new conventions (parameterizations, profile format, chordal-tolerance
distinction) ratified into DESIGN.md at exit.
