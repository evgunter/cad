# M5 PR 9 — curved booleans end-to-end + the tangency regime (binding spec)

Branch `ev/m5-pr9-curved-booleans` from current main. Plan line 9
(docs/M5-PLAN.md:262-274). Consumes: PR 5's C5 dispatch table, PR 6's
certified pcurve doors, PR 7's SSI (march + three-limb certificate +
in-op exhaustiveness), PR 8's BVH, S1's zip shape, S9's window
selection. Ratified contract: CURVED-DESIGN C7 (:398-461),
C12.1-C12.5 (:640-665), OQ5 (:829-840), OQ7 (:870-901), the M5
envelope (:691-708); DESIGN.md D2 as sharpened (:448-506). Binding
throughout; deviations are numbered and reported, never improvised.

## 1. The zip: SSI wired into splitting/boolean (C12.1, C12.3)

- `CurvedBooleanUnsupported` (topo/src/boolean/mod.rs:374-390)
  retires **per C5 table arm, never wholesale**. An arm wires here
  iff its intersection construction is live at this PR's merge base
  (exact conic arms from PR 5; retired SSI arms from PR 7:
  cylinder×sphere). The plane×NURBS arm wires **structurally** —
  routed through the same code path, still surfacing PR 7b's typed
  limb-2 refusal until 7b flips its flag; wiring must be such that
  7b's flip alone (no PR 9 edits) makes the boolean arm live.
- Pipeline shape: curved arms flow through the SAME stages as
  planar booleans — classify/reduce consume
  `geom_brep::intersect::route` results; output stages reused
  verbatim (declared merge, D6 edge descriptions, contact remap,
  tier gates, volume backstop) per the S1 precedent
  (topo/src/boolean/rest.rs:1-70's staged shape and its
  reuse-don't-reinvent rule). No new region algebra; anything the
  stages cannot realize refuses typed, never silently falls back.
- Sections on curved faces: S9's azimuth-window machinery is the
  selection precedent; section curves on curved walls use exact
  carriers where the table gives them (conics) and SSI fitted
  carriers where it doesn't (rung 3). Both lanes (f64 + Interval).
- `split_edge`/`EdgeCurveSpec::split_specs` (C12.3): NURBS carrier
  split = knot insertion (C11); conic split = parameter-interval
  split. `EdgeCurve::certify`'s Nurbs-carrier refusal FLIPS — this
  PR mints the kernel's first rung-3 edges at rest. The end-to-end
  Nurbs split row lands next to the m5_pr7_split_meter rows
  (topo/tests/m5_pr7_split_meter.rs:17-19 names this PR as the
  trigger; PR 7 fix-pass item m8 completes here). The `MappedCurve`
  arc lane's unreachable-at-rest coverage note is revisited
  (CURVED C12.3).
- **Pcurve storage variant** (PR 7 deviation 3;
  geom-brep/src/pcurve_cache.rs:154-196 is the binding in-code
  statement): a second `Pcurve` variant stores the fitted 2-D NURBS
  pcurve from the ℝ⁴ trace. `Copy` drops from `Pcurve`/`PcurveCache`
  (Arc payload — the `Surface`-at-PR-3 precedent), rippling through
  topo storage. Certification stays in metres: C2.2 control-hull
  bounds via the SSI limbs (limb 1 certified foot points; limb 2
  per-operand sup bound), NOT the Harmonic sampled schedule.
  `PcurveCertifyError::UnsupportedCarrier` retires for exactly this
  class; every curved edge at rest carries per-half-edge certified
  pcurves, seam edges with distinct pcurves (exit criterion).

## 2. `TangentIntersection` construction + jet certification (C7)

- The `EdgeGeometry` variant lands: `TangentIntersection { s1, s2,
  witness }` (geom-brep/src/edge_geometry.rs:280-311 slot), witness
  pinned at carrier(mid) by the same S2 argument as `Intersection`.
  No contact-order field (order-k > 1 out of scope, D2 note).
- Constructed **by classification, never by marching**: the C5
  table's tangent arms (PR 5's `TangentLine` classification at
  splitting/join.rs:210-216, splitting/mod.rs:205-214 doors) flip
  from `TangencyUnsupported` refusals to construction. SSI's
  σ₂-sliver band keeps refusing toward C7 exactly as today
  (ssi/march.rs:25) — marching never produces tangency.
- Certification is the jet schedule (C7 bullet 2, verbatim): per
  sample — both implicit residuals within ε; normal parallelism
  within ε·κ_rel (lever arm 1/κ_rel, D4 ¶1); relative transverse
  normal curvature bounded away from zero (the second-order margin,
  the IFT denominator). Plus C2.2 hull bounds between samples and a
  C2.3-style uniqueness tube built on the JET system. Each decide
  goes through the K funnel with a static name (`tangent_*` family).

## 3. Second-order sector classification (C12.2)

- Where the first-order sector trilean returns exactly-on
  (`EntersMaterial::Tangent`, enters.rs:98; the sector-search Zero
  graze, boolean/sectors.rs:27-36), classification descends ONE
  order: compare normal curvatures of the tied sectors along the
  probe direction as a **new named trilean**, margin = curvature
  difference against the derived threshold at lever arm 1/κ (D4 ¶1
  discipline). In-band second-order ties ESCALATE (F6 — an
  osculating pair is a sliver at this ε). Follow the arm-then-margin
  idiom of dihedral.rs:94-152; funnel through the crate's single
  `decide` wrapper; k-lint clean.
- `ConsecutiveOnSectors` (splitting/mod.rs:240-252) flips from typed
  refusal to second-order classification where the new trilean
  resolves it; stays typed where it doesn't (never guess).
- The ON-set machinery consumes curved carrier tangents
  (neighborhood.rs:145-152 already derives them) instead of assuming
  straight edges.
- This family is the K funnel's second genuinely ill-conditioned
  crop (after `solver_branch_margin`): telemetry from birth — a
  verdict-log row asserting every new predicate name appears
  (m5_pr7_ssi.rs:922-957 is the shape to copy), plus the PR 14
  K-snapshot dependency noted in rustdoc.

## 4. Tier-3 mark + must-carry (OQ7's two-level shape, ratified)

- (i) **The mark**: every definitely-tangent edge carries the
  tangency verdict as a named recorded classification — tier 3
  already samples per-edge dihedrals (validate.rs:1574-1605); the
  same data is KEPT as a verdict instead of discarded.
- (ii) **The must-carry**: `TangentNotIntrinsic` (sibling of
  `TransverseNotIntrinsic`, validate.rs:389/:918) fires only on
  **jet-determinate** tangencies — definitely-tangent AND
  second-order separation definite. G2 conventional `MappedCurve`
  joins are exempt BY THE PREDICATE (zero-side second-order margin —
  the surfaces under-determine the locus), never by an exemption
  list; pinned in BOTH directions (a G2 join that must not carry, a
  fillet-grade tangency that must). In-band second order escalates
  (F6). Escalated and `Seam` edges stay exempt exactly as today; the
  ε-tightening-never-flips-valid-to-invalid property is preserved
  and pinned.

## 5. Cosurface merge (C12.5)

- `merge_coplanar_faces` generalizes to same-surface (cosurface)
  merging for curved seams the zip manufactures (the named case: a
  cylinder split by a through-cut re-merging its wall pieces). Same
  ladder, same never-numeric rule, N3 naming semantics unchanged;
  the declared rung is a provenance lookup (M4's GeomSource
  retirement consumed — not bit_identity). The merge_faces.rs:26-28
  "curved same-key neighbors stay unmerged" note flips with the
  machinery that makes it safe.

## 6. Census boundary + the 3′ frontier (C12.4, OQ5, envelope)

- The census stays planar-exact. `CensusUnsupported` boundary text
  updates to name the C7/OQ5 deferral explicitly (C12.4).
- Curved boolean results that TOUCH refuse typed at the 3′ gate —
  the M5 envelope's frontier, pinned by fixture. M5 curved booleans
  produce tier-3 transverse, non-touching results; say so in the
  refusal text.
- **Every new error arm follows the two-tolerance message shape
  INCLUDING definite arms** (the S9 lesson, standing process): the
  width-τ-exact vs τ−δ message pair is pinned for each new arm.

## 7. Acceptance

- **Shape (ii)**: cylinder boss ∪ plate — the first transverse
  curved boolean end-to-end at tier 3, both lanes, bit-replay at
  ε ∈ {1e-6, 1e-12} + Interval; volume backstop; joins the Band 4
  corpus with standard persistence/latency rows.
- **Rung-3 at rest**: a cylinder×sphere boolean (PR 7's retired SSI
  arm) mints fitted-carrier edges at rest — every fitted cache
  carries the full C2 certificate (no schedule-max-only cache at
  rest); certified pcurves both halves; the end-to-end Nurbs
  split_edge row; tier gates pass.
- **Shape (iii) readiness**: the (Plane, Nurbs) boolean arm is
  demonstrated to sit one 7b-flag-flip from live (a test pins the
  typed limb-2 refusal SURFACING THROUGH the boolean pipeline). If
  PR 7b has merged by this PR's fix pass, the directly-authored
  NURBS-wall boolean row lands here; otherwise it lands in 7b's
  wake and this spec says so honestly.
- **Tangency**: an authored tangent pair classifies to
  `TangentIntersection` with the full jet certificate; the
  second-order sector row resolves a first-order tie and escalates
  in-band osculation typed (exit criteria wording); the mark + the
  must-carry + the G2 exemption pinned both directions.
- **Cosurface merge**: the through-cut cylinder re-merge fixture.
- **3′ frontier**: a touching curved boolean refuses typed.
- Multi-ε honesty (the #146 lesson, standing): all probe placements
  and corruptions SCALE FROM THE RESOLVED BAND; sample counts
  derived from the governing law; explicit skip-with-reason only
  where scaling is dishonest; verify locally at 1e-6/1e-12/Interval
  before push.

## 8. Out of scope

Curved census (OQ5 — own design doc); repair/adoption for
near-tangent operands (M5+, F6); fillet trimlines (PR 12; they
STORE TangentIntersection — leave the constructor general enough);
sweeps/lofts (PR 10); tensor Bernstein composition and the
plane×NURBS limb-2 bound (PR 7b); offsets/shelling; touching-result
certification. Frontier errors name the front door that does not
exist yet (FullRevolveHoles precedent).

## 9. Process

One implementer + one blinded adversarial reviewer + one fix pass.
Review must run real e2e programs (consumer demos, independent
re-derivations, merge-base differentials), not diff-reading; review
charter must include: attack the zip's stage reuse (can a curved
seam reach an output stage planar booleans never exercised?),
independent re-derivation of the second-order margin's lever-arm
scaling, adversarial G2-vs-intrinsic boundary probes, and the
CODE QUALITY REPORT with the fixed rubric. Touched-crate local
battery only (default ε + Interval); hosted CI is the gate.
Push per unit. Deviations numbered. K-funnel: k-lint clean; no
ladder-pollution (certify.rs:531-539 rule).
