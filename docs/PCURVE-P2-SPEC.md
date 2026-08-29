# PCURVE P-2 — #498's home for interior-column `Intersection` carriers (spec)

Orchestrator work order for PCURVE-PLAN item 2 **as narrowed by its own
substrate** (2026-08-29, PR #1168). Ratified ground is
`docs/PCURVE-UNIFY-DESIGN.md` (U2) and `docs/DUAL-DESIGN.md` (DL6) —
not re-litigated here.

**Scope, stated once:** interior-column `Intersection` carriers only.
The diagonal half is SPLIT OUT and is not this unit's — its binding
blocker is `PXN_IMAGE_DEGREE = 1` in `geom-brep/src/edge_nurbs.rs`,
edge-certification work banked to #264. P-2 cannot flip it however it
lands. Do not attempt it; do not widen scope to reach it.

## What is already true (measured by the substrate — verify cheaply, do not re-derive)

- **`General` costs nothing on the certification side.** Both #498
  sub-classes certify through `certify_general` TODAY, unmodified, at
  the `(surface, mate)` pair `mate_surface` (`pcurves.rs:348`) already
  computes. P-1 wired `General` through `chart_box`, `shift_branch`
  and `recertify`, so the loop walk, the face window and the tier-3
  at-rest replay need nothing.
- **The refusal is a search fall-through, not a classification.**
  `crates/topo/src/pcurves.rs:690-739`, `nurbs_iso_derive`'s
  `Intersection` arm; the refusal at `:725` fires when the fixed
  4-candidate schedule (chart columns × two `v` directions) finds no
  `Ok(Zero)`. On the widened fixture the correct answer is `u = 2`, an
  interior knot the schedule never tries.
- **Nobody reads these carriers today, because they cannot exist.**
  `mint_pcurves` propagates `IsoUnsupported`, so such an edge kills the
  whole construction (for STEP: the file does not import).

## Items

1. **PREREQUISITE — fix #1157 first, and prove the f64 path did not
   move.** `Vec3::orthonormal_basis` (`geom-core/src/linalg/vec.rs:344`)
   manufactures a poisoned `Trv` (and for `n.x != 0`, UNBOUNDED) chart
   frame for every vertical plane under `Interval`: Duff's trick relies
   on `s` and `n.z` being correlated, and interval arithmetic evaluates
   each occurrence independently, so `copysign(1,[0,0]) = [-1,1]` makes
   the denominator contain zero. Under **DL6 this is a ratified-contract
   violation** — the inputs pose a real question, an absorbing path was
   taken where a widening one exists, and the refusal surfaces four
   layers from its minting site. Restore the correlation (compute the
   magnitude `1 + |n.z|` and apply the sign) rather than special-casing.
   **The f64 path is EXPECTED bit-identical by derivation** — for
   `n.z >= 0`, `s*(1+|n.z|) = 1+n.z`; for `n.z < 0`, `-(1+|n.z|) =
   -1+n.z`, the same f64 add since negation is exact — **but that is a
   derivation, not a measurement: verify it against the golden fixtures
   and say so.** Evan's ruling (2026-08-29) folds this into P-2 rather
   than leaving it filed, so it gets this unit's review. **If the
   substrate work shows P-2 never reaches a vertical plane, say so and
   drop the item back out** — do not carry it for tidiness.
2. **Build the interior-column deriver.** The 4-candidate schedule
   cannot find an interior knot; it needs a column SEARCH. **Two image
   producers already exist and should be considered before writing a
   third**: `geom-brep/src/edge_nurbs.rs:330-336` already derives
   exactly this image (33 certified foot points, interpolated on the
   carrier's own parameter, `on_carrier_domain`-lifted) at EDGE
   certification time and then THROWS IT AWAY, returning only
   `PlaneNurbsLimbs` scalars; and `plane_nurbs_ssi` returns a
   ready-made `General` image as `SsiBranch::pcurve_b` (f64-only, so
   any route through it inherits the f64-structure + T-lift pattern).
   Reusing an existing producer is preferred to a new one; if you write
   a new one, say why the existing ones did not fit.
3. **Raise `mint_pcurves` to `T: PcurveFittedLane`.** Evan ruled
   (2026-08-29) that P-2 pays this bound. Measured ripple: 4 `E0277`s
   in `topo` (`boolean/ops.rs:568`, `merge_faces.rs:571`,
   `splitting/mod.rs:650`, `transform.rs:509`) plus 4 static sites in
   `sweep` (`loft::assemble`, `fillet/surgery.rs:514`,
   `revolve/tube.rs` `build`, `revolve::revolve`). **It is signature
   churn, NOT a capability loss** — `Dual<T>` implements the trait with
   a statically-refusing impl (`pcurve_cache.rs:1407`), so no scalar is
   excluded. The transitive closure past `topo` is UNMEASURED; measure
   it rather than assuming it stops there.
4. **Retire DESIGN.md frontier (c)'s bound clause with it.** Frontier
   (c) names "the mint pass needs the `PcurveFittedLane` bound on every
   constructor" as an open blocker, and `certify_fitted`'s docs echo
   it. Once item 3 lands that is paid; correct BOTH texts. Frontier
   (c)'s OTHER residue (the cone/torus oblique classes, which have no
   ring-computable meters composite) is untouched and stays open.
5. **Re-express the pin, and fix its vacuity (#1167).**
   `crates/sweep/tests/m8_4_intersection_iso.rs::an_interior_column_intersection_refuses_typed:405`
   is the ONLY row in the tree pinning this refusal, and it has two
   defects: `posture()` accepts `Refused` OR `Escalated` (teeth = "does
   not mint"), and at ε=1e-12 the fixture's seam does not attach at all
   (`PlaneNurbsCertificate`, margin 6.217e-12) so the row RETURNS
   EARLY having asserted nothing. Flipping it is a genuine
   red-then-green, but the re-expression **must assert a DEFINITE
   outcome** — a `General` cache whose image is the interior column and
   whose certificate envelope is `<= eps` — and **must be
   non-ε-conditional, or carry an explicit three-cell ε table**. A row
   that goes quiet is worse than one that reds.
6. **Do NOT disturb these rows** (checked; they keep their teeth):
   `geom-brep/tests/imported_chart_arc_rim.rs::an_interior_column_still_refuses`
   and `::a_seam_column_certifies_on_a_non_unit_chart`, and
   `step-import/tests/nurbs_import.rs::an_adopted_iso_column_is_a_knot_domain_end`.
   These are the CERTIFICATION lane refusing an interior column offered
   as an exact `IsoLine`/`IsoArc` — a different variant from `General`,
   and that must keep refusing.
7. **The third excluded case — ruling: widen the schedule, do not
   downgrade.** The refusal payload names "a partial or reparameterized
   restatement of a column", which is a GENUINE boundary column getting
   the same fall-through. Where the exact `IsoLine`/`IsoArc` class
   applies, the schedule should find it — an exact description is
   strictly better than a fitted one (cheaper, exact, and existing rows
   pin it). Fall back to `General` only where the exact class genuinely
   does not apply. Do not hand a locus to `General` that has an exact
   description available.

## Acceptance

1. An interior-column `Intersection` edge MINTS a `General` cache,
   certified, and the body validates at rest.
2. The #1167 pin flips red-then-green, asserting a definite outcome at
   every ε the matrix draws.
3. `mint_pcurves` carries the `PcurveFittedLane` bound; the workspace
   compiles; frontier (c)'s bound clause and `certify_fitted`'s docs
   are corrected.
4. #1157 is fixed with the f64 path measured (not derived) unchanged —
   or explicitly dropped with a reason.
5. Hosted CI green **at a NAMED configuration** (`CI-Config:` trailer
   per #1136), covering BOTH compile modes. Do not rely on the sampler;
   verify `CONFIG_SOURCE` reports the trailer was honoured.

## What P-2 does NOT claim, and must say so in the PR body

The body builds and validates at rest; **volume, area, tessellation and
offset of the affected face then refuse TYPED** at six sites —
`mesh/src/trimmed.rs:982`, `mesh/src/chords.rs:564`,
`topo/src/props.rs:1147` and `:1160`, `topo/src/chart_region.rs:1224`,
`topo/src/replace_face.rs:1675` — every one of which cites "the cut-loft
unit". That is a real improvement over "cannot be built" and it is
narrower than #498's acceptance text, which asks for "the extractor with
its own certification class". **No new certification class is needed**
(`certify_general` at the Fitted grade is it, measured working). File
the six refusals as a named follow-up rather than leaving them implicit.

**Structural constraint:** `validate_pcurves` requires a face's cache
set to be COMPLETE — a face carrying one `General` must carry a cache
on every half-edge of every loop.

## Process

Standard: one implementer + one blinded cross-model reviewer pair at a
frozen head + fix pass; A/B row at merge; ordinal claimed on main at
review dispatch WITHOUT naming arms; hosted CI the only gate. Standing
brief lines apply (OUTPUT DISCIPLINE, the `setsid` exception,
lane-private publish paths, NO `Co-Authored-By` in lane commits, k-lint
discipline, merge-main + build the union, `--no-fail-fast` locally per
#1128).
