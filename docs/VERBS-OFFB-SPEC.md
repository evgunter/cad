# VERBS-OFF-B — the offset fit and its certificate (the two meters, A9.4/A9.10, the two-limb bound)

Wave 3 unit 2 of `docs/VERBS-PLAN.md`, per the ratified
`docs/OFFSET-DESIGN.md` O2/O3. Branch `verbs/offb`, PR to main.
Difficulty logged pre-dispatch: **L** — the program's hardest unit:
the kernel's first surface-fitting machinery, built from the Book.
Substrate: `docs/Q8-SUBSTRATE-2026-08-21.md` §2–3 (snapshot-caveated
— re-verify anchors; OFF-A's module now exists as the natural home).

## Scope: machinery, not integration

Given a NURBS base surface and d, produce a fitted NURBS offset and
a certificate — or refuse typed. **No `Surface` variant, no
storage, no validator wiring, no topology** (OFF-C's). The deliver-
able is three composable pieces in `geom-brep` (module layout the
implementer's call; `offset` and a sibling `fit` module are the
likely shape):

1. **The regularity meter** — a certified LOWER bound on
   ‖S_u × S_v‖ over a (u,v) patch. The offset is undefined where
   the normal degenerates; the fit door refuses (never degrades) on
   a patch whose regularity cannot be bounded away from zero. Build
   from the spline hull machinery (`sup_norm_bound` family gives
   upper bounds; the lower bound needs the cross-product's
   coefficient hulls minus the enclosure width — derive it, state
   the conservatism direction, and note it is the tree's FIRST
   inf-side surface bound: #528 records the same need for
   chart-region stretch, so name the shared shape without building
   #528's consumer). Named predicate (e.g.
   `offset_normal_floor`), margin in the natural units with the
   lever stated.
2. **The collapse meter** — d vs principal-curvature headroom over
   the patch: the inward offset folds where |d| reaches 1/κ_max.
   Ingredients exist (`SurfaceJet3`, the patch Hessian hull bounds
   in `mesh::nurbs_cert` — reuse or lift, do not restate; if
   lifting from `mesh` into `geom-brep` is the honest home, say so
   and keep ONE spelling). Named predicate (e.g.
   `offset_curvature_headroom`), the fillet battery's
   radius-headroom shape one dimension up.
3. **The fit + certificate** —
   `fit_offset(base: &NurbsSurface, d, tol/band) ->
   Result<(NurbsSurface, OffsetCertificate), _>`:
   - Fit engine per C8's naming: the Book's §9.4 stack — A9.4 grid
     interpolation seeded from the sampled offset, A9.10-style
     refine-until-tolerance (knot insertion on the worst spans)
     with a budget refusing typed on exhaustion (`QuadratureBudget`'s
     shape — never an uncertified return). The Book scans are in
     `references/` (read pages visually; §9.4.1–9.4.4, A9.8–A9.10,
     pp. 428–432, Eqs. 9.86–9.89).
   - Certificate: the C8 two-limb shape lifted from the SSI
     precedent — a (u,v) span schedule; per-cell on-locus residual
     samples of ‖S_fit − (S + d·n)‖; a hull-side sup bound via the
     ring/interval composite (`SurfaceResidual` / the tensor
     composite — the pieces exist; what is new is composing the
     NORMALIZED normal, whose square root is exactly why the offset
     is not NURBS: bound it with the regularity floor from meter 1
     — the floor is what makes 1/‖S_u×S_v‖ boundable). Claim:
     `sup ‖S_fit − (S + d·n)‖ ≤ ε_precision` (D4's two-tolerance
     split — this is ε_precision). The certificate is a plain
     struct here; private-field/unrepresentable-invalid discipline
     rides OFF-C's `ApproxSurface`.

## Fences

- No `Surface::Approx`, no `SurfaceDescription`, no persistence, no
  validator, no shell. OFF-A's analytic mints untouched.
- The intensional-description vocabulary (O2's `Offset { base, d }`)
  is OFF-C's; here the base+d travel as arguments.
- No new dependencies (the fit is in-house by ratified decision;
  curvo has nothing to borrow — the audit stands).

## Acceptance

- **The analytic oracle**: a cylinder (and a sphere) re-expressed
  as an exact rational NURBS, offset via `fit_offset`, checked
  against OFF-A's exact analytic offset — the fitted surface's
  residual against the CLOSED FORM certified and independently
  sampled. This is the one place the answer is known exactly; it is
  the unit's spine.
- A genuinely non-analytic base (a lofted/skinned surface from the
  existing machinery) fits with a certified residual ≤ ε at
  default ε; the certificate's sup bound CONTAINS a dense random
  sample's max (the bound never under-reports — the red direction).
- Planted reds: a degraded fit (coarsened knots) fails the
  certificate; a near-degenerate patch (collapsed control row —
  the sphere-pole shape) refuses at the regularity floor; |d| past
  curvature headroom refuses at the collapse meter; budget
  exhaustion refuses typed.
- Interval rows for the certificate arithmetic; both d signs;
  existing suites untouched.
- Note the drawn CI point; both compile modes covered between
  hosted draws and local runs, stated per the convention.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Merge origin/main before
opening the PR titled "VERBS-OFF-B: the offset fit and its
certificate — the two meters, the Book's fit stack, the two-limb
bound"; confirm CI runs STARTED; watch to completion. Do not merge.
If a genuine design fork surfaces (e.g. the fit's parameterization
choice materially changes the certificate's claim), STOP and report
for adjudication rather than choosing silently.
