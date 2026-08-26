# VERBS-CYLCYL — the cylinder×cylinder germ lane (two PRs)

Wave 2 row 7 of `docs/VERBS-PLAN.md`. A TWO-PR unit (the M9-2
shape): **PR-A is shared substrate every later germ lane consumes;
PR-B is the arms.** Branches `verbs/cylcyl-a`, `verbs/cylcyl-b`.
Difficulty logged pre-dispatch: PR-A **L**, PR-B **M**. Substrate
survey (2026-08-26, anchors verified on main @ 5e38d56e) — its two
premise corrections bind: M9-3's PR-B zip substrate is NOT on main
(#971 open — lean on nothing from it), and M9-3 is the
declared-contact lane, not the germ join analog.

## PR-A — the containment door and the pair-general dispatch

1. **D3, the spine: curved point-in-face containment on cylinder
   charts** (`contain.rs:169`'s `curved_boundary_containment` is
   boundary-only and Line-carriers-only, and says so). This
   retires `reduce.rs:996-998`'s full-circle conservatism — #347's
   defect (a) IS this door, not a rider. The cylinder chart's
   exact trim machinery exists at the solid_contain arm; the
   face-level analog is the deliverable. Circle carriers join Line
   carriers in the boundary walk.
2. **D5/D6: pair-general dispatch, with the trap closed.**
   `germ_section_frame`'s `_ => Ok(None)` means "straight/planar
   germ" — a silent wrong-chord mint the moment D4 widens. Close
   it EXPLICITLY (a non-plane pair without a frame arm refuses
   typed, never defaults) BEFORE any arm lands. `section_case`'s
   plane×wall signature generalizes to the pair form;
   `SectionCase::Straight` already models the parallel-axis
   answer.
3. **D10: the no-crossings posture for cylinder pairs.**
   `sphere_extent_scan` skips cylinder faces, so a fully-crossing
   cyl pair with no edge event reaches the vertex-probe fallback
   with NO extent certificate — the S12-silence shape, and the one
   path that could yield a WRONG answer rather than a refusal.
   Give cylinder groups an extent certificate or a typed refusal;
   silence never re-opens.
4. **The opening probe** (the GATE precedent): build #347's
   coaxial union case FIRST and name the door that actually
   refuses today — the survey could not run code and left it
   unmeasured. The measurement drives which of D3/D4/D10 the
   bracket's path exercises, and lands in the PR body.
5. Fences: no new join arms (PR-B's), no sphere/cone work, the
   split gate's body-scoped Plane|Cylinder stays (noted, not
   changed), #971's territory untouched.
6. Acceptance: the D5 trap red-able (a planted non-plane pair
   without an arm refuses, never mints straight); D3's containment
   pinned both directions on cylinder charts (inside/outside/
   boundary-adjacent at band edges); D10's posture pinned; the
   full-circle conservatism row (`bool_circle_curved_clearance`
   consumers) re-pinned to the honest arc-scoped verdict; existing
   boolean suites bit-identical.

## PR-B — the arms

1. **Parallel-axis first** (#347's whole need): route
   `cylinder_cylinder_section`'s `ParallelLines`/`TangentLine`/
   `Empty` through the widened dispatch (`SectionCase::Straight`;
   `tangent_locus`'s parallel-cylinder arm at rest.rs:766/783
   already exists). Radius equality stays STRUCTURAL/declared
   (`RadiusEvidence` — never inferred; the existing refusals stand
   verbatim).
2. **Equal-radius Steinmetz second**: `TwoEllipses` through the
   existing `SectionConic` arc-side rule (which already carries
   ellipses from `TiltedEllipse`).
3. Skew and unequal-radius stay `RoutesToGeneralRung` verbatim —
   the honest refusal is already written; the general quartic is
   canal territory and OUT.
4. **Acceptance: `bracket.py` rounds at 6 mm** (#347's own bound —
   the corner rounds union at the requested radius); the
   two-circle-derived-cylinders union completes (both parallel and
   coaxial); a Steinmetz pair joins and the result validates
   tier-3 with census + mass-properties pins; #347 CLOSES;
   existing suites bit-identical. Note: klein wall 3 does NOT move
   (its pair is (Cone, Plane) — row 10's), and the spec says so to
   keep the demo expectations honest.

## Plan consequences (recorded at this spec's sync)

Rows reorder **7 → 9 → 8**: SPHSPH promoted ahead of CYLSPH —
(Sphere, Sphere) is rung CLOSED with an unwritten exact circle
(structurally `plane_sphere_section`'s sibling), and after PR-A its
only new door is the partial-sphere-face containment arm (where
the #723/#893 polar-rim caution binds). CYLSPH runs LAST and alone:
it is the only fitted-rung lane (Nurbs carrier, the fitted-azimuth
rule, the Nurbs edge-gate consequence, the f64-only marcher) and
must not drag that machinery into the exact lanes' dispatches.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR drafts. Merge origin/main
before opening each PR; confirm CI runs STARTED; note the drawn
point + coverage; watch to completion. Do not merge. PR-A STOPS
for adjudication if D3's containment turns out to need machinery
the survey did not map (e.g. a chart form the trim walk cannot
express).
