# Q8 offset/shell substrate survey (opus lane, 2026-08-21)

> **Snapshot caveat (added at commit, 2026-08-25):** anchors below were
> verified on main as of 2026-08-21, BEFORE Waves 1-2 landed (the
> void-insertion door, classify_shells, the coaxial fillet arms, the
> hollow tube door). Re-verify any anchor before citing it in a spec;
> the structural conclusions (no offset code; curve-only fitting; the
> approximating-surface absence; open shells unrepresentable) were
> re-confirmed at OFFSET-DESIGN ratification and stand.

Verbatim evidence base for the Q8 design conversation. Anchors
verified by the survey lane on main at 2026-08-21.

## 1. Analytic kinds / closure under offset

- `crates/geom/src/surfaces.rs:87` `Surface<T>` closed enum, 6
  variants: Plane :98, Cylinder :118, Cone :163 (v = slant arc
  length), Sphere :194, Torus :222 (ring, R>r>0), Nurbs :245
  (Arc<NurbsSurface>). Kind tag mirror `geom-brep/src/intersect.rs:89`.
- No constructors — struct-literal on public fields (D2, surfaces.rs:78).
- Analytic offset minting = struct-update: plane origin+normal*d;
  cyl/sphere radius±d; torus minor±d; cone apex shifts d/sin(α)
  along axis. Degeneracy gates live at call sites today (ring-torus
  refusal battery.rs:263) — an offset door owns its own refusals
  (r−d ≤ 0, torus self-intersection, cone through apex).
- Mint precedents: `fillet/blend.rs:174` plane_sphere_blend → Torus
  :208; :117 plane_plane_blend → Cylinder :133; :255 corner_ball →
  Sphere :274. Central mint switch to copy:
  `sweep/src/revolve/surfaces.rs:24` wall_surface.
- tube_along_arc: `sweep/src/revolve/tube.rs:132`.
- ZERO existing offset-surface code (no Offset/OffsetSurface symbol).

## 2. Approximating-curve precedent (Q8's template)

- Intensional spec: `geom-brep/src/edge_geometry.rs:280` EdgeGeometry
  (Intersection :285, TangentIntersection :302, MappedCurve :329,
  Seam :346, IsoCurve :374; no Explicit by design).
- Spec+fit pair: `geom-brep/src/certify.rs:368` EdgeCurveSpec;
  certified product `:480` EdgeCurve (private fields, uncertified
  unrepresentable; certificate field :485).
- Fit: `geom-brep/src/ssi.rs:617` fit_branch (cubic interpolation of
  marched polyline + both pcurves, shared parameter, OQ4). Doors:
  :681 cylinder_sphere_ssi, :883 plane_nurbs_ssi, :1101 trace_plane_nurbs_uncertified.
- Certificates: sampled `certify.rs:462` Certificate (CERT_SAMPLES=9
  :66, run_checks :956, certify :545, recertify :575,
  recertify_nurbs_lane :659); sup-norm `geom-brep/src/ssi/certify.rs:179`
  SsiCertificate {samples, on_locus_max, hull_sup, tube_*}
  (SSI_CERT_SPANS=32 :167; door ssi.rs:1187 certify_rung3; the
  Band's zero() IS the run ε — no eps parameter).
- Storage: branch ssi.rs:517; pcurve cache `pcurve_cache.rs:907`
  (beside max_residual :886, envelope :898; built by fitted_lane
  :978); body side table `topo/src/body.rs:156`
  pcurves: SecondaryMap<HalfEdgeKey, PcurveCache>.
- Validator NEVER trusts stored certificates: `topo/src/validate.rs:1810`
  re-certifies every edge each call; pcurves re-certified
  `topo/src/pcurves.rs:1387`; absence of caches never a defect.
  Exact/approx divergence validate.rs:1976 — NURBS-adjacent edges
  exempt BY KIND from classify_dihedral (TransverseNotIntrinsic
  :2000, jet must-carry :2013 skipped, contact mark Unmarked).
- Approximating-SURFACE scaffolding: NONE. NurbsSurface
  (`geom/src/surfaces/nurbs.rs:177`) has no description/certificate/
  residual field. Deliberate: `editor-core/src/node.rs:446-457`
  (Loft: "NO residual obligation… NO approximating-surface machinery
  downstream"), `sweep/src/skin.rs:9-11`. Nearest analogs are
  mesh-vs-surface chord error (`mesh/src/nurbs_cert.rs`,
  `mesh/src/cert.rs:107-141`) — a DIFFERENT claim.

## 3. NURBS machinery an Offset(S,d) fit composes from

- Eval/jets: nurbs.rs eval :813, ders :736 (SurfaceJet :41),
  ders3 :776 (SurfaceJet3 :74). Normals only via Surface::normal
  surfaces.rs:434.
- Refinement complete for surfaces: insert/refine/elevate/remove
  knot u/v :610-:676 (remove returns certified deviation).
- FITTING IS CURVE-ONLY: `geom/src/curves/fit.rs` interpolate :499
  (A9.1), approximate :575, approximate_budgeted :604;
  interpolate_columns :322 (loft); skinning `sweep/src/skin.rs:632`.
  NO A9.4 grid interpolation, NO A9.10 approximate-to-tolerance.
  No surfaces/fit.rs.
- Hull/quadrature: `geom-core/src/spline/hull.rs` span_hull :136,
  domain_hull :161, rational twins :206/:221, derivative_coeffs :286,
  sup_norm_bound(_rational) :366/:373. Surface AABB surfaces/boxes.rs:25.
- Patch Hessian/interval bounds: mesh/src/nurbs_cert.rs
  NurbsFaceBound :193, nurbs_cell_bounds :307, band_schedule :654,
  nurbs_face_bound :887, rational_split_points :971. Ring residual
  over patch: `geom-core/src/spline/compose/tensor.rs:219`
  SurfaceResidual (sup_bound :256), surface_curve_residual :527;
  ring_coords nurbs.rs:844.
- Two-limb certificate pattern to copy: ssi/certify.rs:27-59 doc,
  hull_sup :193, sup decision :400-406.
- MISSING: surface regularity meter — speed_lower_bound is
  curve-only (curves/nurbs.rs:516, rational :750); no certified
  lower bound on ‖S_u×S_v‖ (offset undefined where it degenerates).
- CURVO: vendoring REJECTED (CURVO-AUDIT.md:118-132); curvo has no
  fitting stack and no SSI (:76-80, :124-126) — nothing to borrow.
  Oracle pinned 47d19d5.

## 4. Open-shell / face-removal — the hardest collision

- Spine: `topo/src/entity.rs:147-168`; Solid{shells} :193;
  Shell :210 doc says "one connected, CLOSED boundary surface".
- Multi-shell solids (voids) ARE supported: entity.rs:170-181;
  outer-vs-cavity derived from signed volume sign; movefac splits
  components into shells (`topo/src/movefac.rs:64`).
- Open shell STRUCTURALLY unrepresentable: Edge born with two
  half-edges (entity.rs:394-406); bijection validate.rs:2728,
  antiparallelism :2760, vertex-orbit closure :2830.
  "Watertightness is structural, tier 1" validate.rs:13-16.
  D1: DESIGN.md:74, :90-92, :1834-1847.
- Closure-assuming checks: shell-partition coherence :2995
  (EdgeAcrossShells), per-shell Euler–Poincaré :3029
  (satisfies_euler_poincare :1468 — χ even, χ ≤ 2; open shell fails
  outright), validate_closed :1524 (c=1 per shell :1561), +V
  invariant :2299, tier-3' pseudomanifold :2413 opens with
  validate_closed :2416. Census assumes closure transitively
  (census.rs:1457,1485,1534); ray-parity meaningless on open shell.
- Surgery precedent (M6-1): `sweep/src/fillet/surgery.rs` module doc
  :1-104, entry fillet_surgery :228 (one solid/one shell
  precondition :232-235); pattern = decide numerics first :329,
  CLONE :333, Euler-operator-only mutation (blank_phase :959,
  rim_phase :1308), attach surfaces/descriptions/pcurves :354-386,
  postcondition once at end :388-393. Mid-transplant refusal leaves
  body "spent, never resumable" :92-98 — shell inherits this.
  Graft sibling topo/src/instance.rs:82.
- Face-count-changing ops (ALL closure-preserving; nothing can leave
  a boundary): mef euler.rs:1253, kef euler_kill.rs:764, mfkrh
  :1031, kfmrh euler_ring.rs:738, merge_coplanar_faces
  merge_faces.rs:345, movefac, split_edge split.rs:145.
- RATIFIED RULE shell must argue against: DESIGN.md:348 "Sweeps emit
  single-shell bodies; voids are born only from booleans" + :452-467
  — unless shell is DEFINED AS body − offset_inward(body) (i.e. IS a
  boolean-family void source).
- Downstream: multi-shell curved body refuses STEP export
  (`step-export/src/lib.rs:310` CurvedShellClassification, rationale
  :160-178) — the teapot hits this when closed; mass properties
  body-level only (topo/src/props.rs:152).

## 5. Klein bottle hand-offsets (what shell replaces)

- klein.rs doc :69-79 finding 1 ("no shell… no refusal to pin");
  const WALL :213. 7 literal r±t/2 pair sites (:295-296, :298-299,
  :303-304, :329/:349, :443/:446, :805/:806, :836/:839); derived
  :518, :663-664. No helper. Blend sign transcribed per site (:329
  rf+half, :349 rf−half; rule as prose :318-323). Side swap at rim
  :252-261 / :300-304. wall_probes :697 pins 7 refusals; NO shell
  probe by design. shell(t) replaces 14 literal expressions / 4
  whole constructions (meridian_at :284-308, band :324-358, elbow
  annulus :442-449, probe annuli :804-807/:833-844).

## 6. Prior art in docs

- DESIGN.md:2054-2067 (Q8), :745-761 (D3), :763+ (D4 two-tolerance —
  residual is ε_precision), :1487 (difficulty ranking: shelling
  hardest, late, scope-boxed), :1823 ("Same principle applies to
  shell/offset" — validity = named margined Q1 predicates over
  inputs, pre-construction; ratified stance, adopt not re-derive),
  :596-604 frontier (f) canal blend PARKED for want of consumer —
  Q8's offset is the first caller (strongest sequencing argument).
- CURVED-DESIGN.md:485-500 (C8 approximating surface = spec + fitted
  cache + residual ≤ ε; certificate = C2 lifted one dimension);
  :610-620 (fit engine named: NURBS Book §9.4.1-9.4.4 / A9.8-A9.10
  pp. 428-432, surface Eqs. 9.86-9.89); :1050-1066 (VMV taxonomy:
  "parametric with procedural definition retained as certification
  target"). C12 items 6-7 landed (nurbs_cert.rs, props/quad.rs) and
  reusable.
- No LONGTERM-IDEAS/PERF entry on offset/shell.

## Missing pieces, ranked

1. Surface-side fitting to tolerance (A9.4 + A9.10) — from-the-book
   build, nothing to borrow. **L**
2. The approximating-surface object (spec+fit+residual triple lifted
   from the EdgeCurve triple; certificate home private-field vs body
   side-table; validator re-derives per face at (u,v)-grid cost). **L**
3. Open-shell vocabulary IF shell produces one — structurally
   impossible today; cheapest ratifiable answer: never materialize —
   shell = body − offset_inward(body), cavity shell of same solid
   (legal per entity.rs:170-181), face removal as one composed Euler
   sequence on a clone (surgery pattern). Collides with "voids born
   only from booleans" unless shell IS boolean-family. **L open / M
   defined-away**
4. Surface regularity meter (certified lower bound ‖S_u×S_v‖) from
   hull.rs. **M**
5. Offset-collapse predicate d vs 1/κ_max over a patch (ingredients:
   SurfaceJet3, Hessian hull bounds). **M**
6. Analytic offset minting (struct-update + degeneracy refusals +
   home). **S**
7. Two-shell curved body downstream: STEP export refusal
   (CurvedShellClassification), no per-shell mass-props door. **S
   each; the demo's gate.**
