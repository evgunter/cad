# VERBS-GERMARMS — the curved pierce ring lane + the cyl×cyl germ arm (two PRs)

RATIFIED 2026-08-29 (orchestrator; from the substrate survey 2026-08-28, anchors verified by reading on
`mngr/kernel-verbs` @ 3820532f; no code run — the corpus-first law
binds and the implementer re-measures every anchor at dispatch).

Branches `verbs/germarms-1`, `verbs/germarms-2`. Difficulty
pre-logged: PR-1 **M** (the ring lane), PR-2 **M contingent**
(the arms; adjudication if a second chord lane is needed).

## Binding inheritance facts

These are the premises the unit is built on. Each is cited; the
implementer's opening measurement re-establishes them or the unit
STOPS.

1. **Only two of #1044's four rows are pierces.**
   `docs/VERBS-PIERCE-SPEC.md:68-110` (the door-2 STOP addendum, already
   adjudicated). `coaxial-equal-r` and `coaxial-stacked` are undeclared
   VALUE-COINCIDENT contacts whose honest destination is the
   declaration ladder — CONTACT-DESIGN C2/C4 forbid inferring either
   gluing at any ε. **They are OUT of this unit** and no acceptance row
   here may claim them.
2. **Line × cylinder-wall roots exist and are `T: Decide`-generic**:
   `crates/topo/src/boolean/solid_contain.rs:818-899` — the ray×wall
   quadratic (`disc = b2.powi(2) - a2*c2` at :848, the
   `bool_ray_cylinder_disc` trilean at :855-866, the two roots at :868),
   trimmed by `point_on_wall_in_face` (`solid_contain.rs:512`,
   `pub(super)`, branch-cut-free cosine window).
3. **Circle × cylinder-wall roots do NOT exist, anywhere.** A full
   repo sweep found **no quartic solver, no cubic solver, no resolvent,
   no companion-matrix/eigenvalue root finder, no Sturm/Descartes/
   Bézier-clipping isolation, no `real_roots`/`solve_poly`, and zero
   occurrences of Ferrari/Cardano/Aberth/Durand-Kerner/Laguerre/
   Halley/Jenkins-Traub/Brent/Ridders/regula-falsi/secant**. Every
   closed-form root lane in the tree is a *specific* quadratic or a
   *single-harmonic* sinusoid; every iterative lane
   (`geom-brep/src/ssi/march.rs:608` `newton_refine`, `:763`
   `push_boundary`'s 32 bisections, `geom/src/curves/projection.rs:196`
   `project_from_seed`, `geom-brep/src/ssi/exhaust.rs`) runs on **f64
   structure** and is architecturally barred from the `Decide` lane
   (`geom/src/projection.rs:44-52` states the boundary).
4. **The residual the circle case needs solved is degree-2
   trigonometric**, and the repo says so at the site it declines to
   solve it: `crates/topo/src/boolean/reduce.rs:1066-1074` — "a
   degree-≤2 TRIGONOMETRIC polynomial with up to four critical
   parameters, so an endpoint gap says nothing about where its minimum
   sits. The unclamped chord-dip charge is what is available without
   solving for them." The coefficient supply already exists generically:
   `geom-brep/src/implicit.rs:397` `circle_residual_harmonics` →
   `(c₀, A₁, A₂)`.
5. **The ring insert is planar by construction, and the blocker is a
   single `None`.** `vtxfac.rs:94-98` takes `(plane, n_pierced)` from
   `face_plane` ∧ `face_outward_normal`; `face_normal.rs:62-73` returns
   `None` for every non-planar face. That `None` — not the chord — is
   what makes a cylinder-wall pierce unreachable.
6. **The join layer has no cyl×cyl arm and refuses typed there**:
   `join.rs:418-433` (`CurvedBooleanUnsupported`, the "no wired join
   arm for this germ pair (cyl×cyl's equal-radius ellipse pair …)"
   catch-all) and `join.rs:746-760`+ (`pair_section_frame`'s cyl×cyl
   arm returns a frame only for the PARALLEL-axis half; a non-parallel
   pair takes `FrameError::NoArm` → `GermFrameUnsupported`,
   `join.rs:654`). The only wired curved chord lane is
   `JoinLane::BoolPlanar` (`chord_join.rs:516-523`) — the **plane-side**
   chord of a curved germ pair.
7. **Door 1 shipped and is reusable verbatim**: the pierced-side split
   at an event point takes its `Circle` arm at `reduce.rs:1459-1521`
   with the mid-anchored azimuth `circle_split_param` (`reduce.rs:1530+`)
   — `T: Decide`-generic, no lane fork, `bool_split_span_period` guard.
   `Ellipse`/`Nurbs` keep `PointSplitCarrierUnsupported`.

## The honest boundary, named up front

**This unit does not make `parallel-equal-r` union.** That row is
circle × wall; its event parameters need the degree-2 trig roots of
fact 3/4, which no machinery in the tree supplies and which cannot be
Newton'd on the `Decide` lane. It is a THIRD unit, behind a design
conversation about what a certified `Decide`-generic trig-quartic lane
even looks like. Any acceptance sentence that promises it is false.

**PR-1 alone does not make `steinmetz` union either.** Two transversal
cylinder walls meet in a curve that lies in face INTERIORS; the only
edge events are where the walls' seam rulings cross the partner wall.
The crossing layer mints those vertices; walking the section between
them is the JOIN's chord lane, which has no cyl×cyl arm (fact 6). So
PR-1's honest destination is exactly the fence
`docs/VERBS-PIERCE-SPEC.md:36-39` already wrote: *the doors move one
layer down, typed, naming the absent arm.*

## PR-1 — the curved pierce ring lane

Branch `verbs/germarms-1`. The crossing layer's last plane-only
assumption.

1. **The point-dependent outward normal.** `vtxfac.rs:94-98` needs a
   per-kind door returning `(tangent datum at p, OutwardNormal at p)`
   where today it gets `(plane, face normal)`. The curved arm is
   `OutwardNormal::from_chart(implicit_gradient(surface, p), f.sense)`
   (`geom-brep/src/implicit.rs:150`; the gradient is unit-magnitude on
   the surface and honestly poison at a cone apex / cylinder axis —
   poison is not a refusal, so the arm gates on the kind and on
   `curvature_lever_arm` (`implicit.rs:208`) before it reads).
   **It belongs in `crates/topo/src/face_normal.rs`, not in `vtxfac`**:
   that module is the ratified one-door home for the sense flip and
   carries a source-walk guard test (`face_normal.rs:102-127`) whose
   whole point is that a second flip must not appear elsewhere. Widen
   the door; do not fork it.
2. **Delta 1 generalizes for free — say so and pin it.** `side_code`
   (`sectors.rs:272-284`) takes only `(dir, OutwardNormal, arm, band)`
   and never touches the plane's origin: it is already a purely
   first-order primitive. So is `pierce_germ_dir` (`vtxfac.rs:516-523`,
   `s.normal.vec().cross(plane_normal)`) — the intersection direction of
   the sector's face plane with the pierced surface's TANGENT plane at
   `p` is the same cross product, because the germ direction is the
   tangent of the section conic at `p`. **State the derivation at the
   site**; a reviewer must be able to see why the substitution is exact
   rather than plausible.
3. **The first-order soundness charge — the item that can STOP this
   PR.** `side_code`'s verdict on a curved wall is a TANGENT-PLANE
   verdict, and a cylinder curves away from its tangent plane. A sector
   bound classified `Out` at lever arm `s.arm` can be inside the
   material when the wall is a hole. The charge is the standard one:
   the sagitta `s.arm² / (2·curvature_lever_arm)` against the
   first-order displacement, decided as a named trilean (proposed
   `bool_pierce_sector_side_curved`), definite ⇒ the verdict stands,
   in-band or wrong-side ⇒ **typed refusal, never a first-order guess**.
   The second-order machinery to do better already exists
   (`geom-brep::enters_material_order2`, consumed by
   `sectors::tangent_lump`, `vtxfac.rs:150`) and is explicitly OUT of
   this PR — name it as the recourse.
4. **Delta 2 stays shut on the curved lane.** The coplanar-sector lump
   (`vtxfac.rs:118-232`) descends to `carrier_eq` against a
   `CarrierDesc::Plane` built from the pierced plane
   (`vtxfac.rs:192-195`). A sector face TANGENT to a curved pierced
   face at `p` is a cosurface/tangency question, and CONTACT-DESIGN
   C2/C4 forbid inferring it. **Refuse typed** (the existing
   `CurvedBooleanUnsupported` at `vtxfac.rs:186-190` is the shape); the
   recourse is a verified declaration, exactly as the undeclared
   crossing-layer rung already says (`reduce.rs:910-924`).
5. **Delta 3's transient chord.** `vtxfac.rs:428-439` mints
   `EdgeCurveSpec::line_between(p_u, p)` and `kemr`s it away two
   statements later. `Body::mev` (`euler.rs:1131`) certifies the edge
   against its OWN carrier and endpoints, not against the face's
   surface, and the module's claim is tier-1-after-every-op
   (`vtxfac.rs:25-26`) — so the straight chord is *probably* still
   legal on a wall. **UNVERIFIED: no code was run.** Measure it first.
   If it does not hold, note that `Curve3` has no helix
   (`geom/src/curves.rs:72-165` — Line, Circle, Ellipse, Nurbs only),
   so the only exact on-wall chords are the chart's iso-curves: a
   ruling (Line, θ = const) or a latitude arc (Circle, h = const).
   Choosing the anchor `u` (`vtxfac.rs:397-413` takes the outer loop's
   `first` half-edge blindly) so that it shares a chart coordinate with
   `p` is the cheap fix; if neither is available, STOP.
6. **The event points.** `curved_face_arm`'s definite-crossing branch
   (`reduce.rs:1177`) currently refuses; it must instead produce the
   crossing parameter. The quadratic and its trim are fact 2's — but
   they are **private to `solid_contain::cast_ray`** and written for a
   RAY (advance-positive folding, closest-hit). Factor the root pair
   out as a `pub(super)` line×wall root function over the edge's
   `[t₀, t₁]` span, keeping the trileans' NAMES (`bool_ray_cylinder_disc`
   is pinned by #1021's acceptance margins — re-metering it moves them,
   and `solid_contain.rs:908-915` already flags the cylinder arm's
   dimensionless `disc/(2r)²` as a deliberate non-normalization). Then
   route each in-span root through the existing `contfp`-shaped
   containment (here `curved_face_containment`, `contain.rs`) and the
   existing `split_at` / `split_other_at_point` / `contacts.vf` triple
   at `reduce.rs:770-790`.
7. **Fences.** No cosurface inference at any ε (item 4). No
   `SectionConic` widening and no join arms (PR-2's). No cone or torus:
   `face_geo` refuses both `KindUnsupported`
   (`solid_contain.rs:404-407`) and the line-clearance `f2` fold has
   arms only for Cylinder/Sphere (`reduce.rs:1228-1237`) — a cone pierce
   has no roots in this tree and this PR must not appear to give it
   one. No `Ellipse`/`Nurbs` carriers. The #1068 arc-loop gate
   (`ArcLoopContainmentUnsupported`, <3-vertex arc-bearing loops) is
   untouched; #1076 owns its remainder.
8. **STOP for adjudication** if item 3's curvature charge cannot be
   made definite on the acceptance fixture without the second-order
   trilean, or if item 5's transient chord turns out to need a curve
   kind that does not exist.
9. **Acceptance.**
   - The `steinmetz` row of
     `crates/sweep/tests/verbs_cylcyl_probe.rs:87-90` **moves**, from
     `CurvedPierceUnsupported` to the join layer's own typed refusal —
     assert the NEW payload and the SITE (`join.rs:428` or
     `join.rs:654`), not merely "not the old one". Update
     `cylinder_unions_refuse_at_the_curved_pierce_door`
     (`verbs_cylcyl_probe.rs:57`) to a two-class table: the two
     cosurface rows keep the pierce door verbatim (fact 1), the two
     pierce rows carry their measured new doors.
   - **Closed form + differential**: a box driven transversally through
     a cylinder WALL (not a cap — `verbs_pierce.rs:65` already owns the
     cap) unions to a metered closed form, tier-3 valid, census pinned;
     the same box moved definitely clear of the wall still answers, and
     the same box moved to graze escalates. This is the row that proves
     the ring lane built a body rather than opening a door.
   - Planted reds: a sector bound whose curvature charge is in-band
     refuses typed (item 3); a tangent sector on a curved pierced face
     refuses typed (item 4); a cone-walled pierce still refuses at its
     own door (the fence, differential).
   - Interval lane pinned both directions (the two-arm pattern).
   - Bit-identity block below holds.

## PR-2 — the cyl×cyl germ arm

Branch `verbs/germarms-2`. Dispatch only after PR-1's measurement names
the join payload; the fence in PR-1 item 7 is deliberately what this PR
retires.

1. **The frame.** `pair_section_frame` (`join.rs:690`) gains the
   non-parallel equal-radius cyl×cyl arm from
   `geom_brep::cylinder_cylinder_section` (`intersect.rs:835`,
   `T: Decide`-generic, already returns the two bisector-plane
   ellipses). Radius equality stays **STRUCTURAL/declared**
   (`RadiusEvidence` — never inferred; VERBS-CYLCYL PR-B item 1's
   sentence is verbatim binding). Skew and unequal-radius keep
   `RoutesToGeneralRung`/`NoArm` verbatim — the general locus is a space
   quartic and is canal territory (`join.rs:739-741`,
   `intersect.rs:264-275`).
2. **The chord lane.** `JoinLane::BoolPlanar` (`chord_join.rs:516`) is
   the plane-side chord of a *plane × wall* pair. A cyl×cyl germ pair
   needs a wall-side chord on BOTH sides. **This is the SectionConic
   widening PR-1 fenced**, and it is the whole content of this PR:
   whether the existing arc-side rule (which already carries ellipses
   from `TiltedEllipse`) serves both sides, or whether a second lane
   variant is required, is the opening MEASUREMENT — it drives the
   difficulty and belongs in the PR body before any code.
3. **The D5 trap stays closed.** `germ_section_frame`'s `None` is a
   CLAIM ("this locus is straight"), not a default
   (`join.rs:621-635`). Every arm added here either proves straightness
   from the axes or names a frame; nothing new may reach the
   straight-chord facing test.
4. **Fences.** No sphere/cone/torus germ arms. No fitted rung, no
   NURBS carrier, no marcher — CYLSPH runs last and alone
   (`docs/VERBS-CYLCYL-SPEC.md:95-104`) and its machinery must not be
   dragged in here. `parallel-equal-r` stays refused (the circle × wall
   roots are still absent — this PR moves nothing for it).
5. **Acceptance.** A Steinmetz pair (equal radius, perpendicular axes,
   declared radius equality) **unions**, validates tier-3, and meters
   against the closed form `16r³/3` for the intersection / the
   corresponding union volume; census pinned; the
   `cylinder_unions_refuse_at_the_curved_pierce_door` steinmetz row
   flips green with the two cosurface rows still refusing per fact 1;
   #347's union demand narrows by comment to the circle × wall residue.
   Differential: the unequal-radius and skew poses still refuse typed at
   their own doors.

## The bit-identity block (#1021 / #1044 conservatism — must not move)

Both PRs. These are the rows the two shipped conservatism units pinned;
a change that moves any of them is a change to their arguments and needs
its own adjudication.

- `crates/sweep/tests/verbs_cylcyl_probe.rs`:
  `the_coaxial_boss_unions_and_meters_at_the_closed_form` (:111,
  `π·2 + π·0.25`), `the_bracket_rounds_at_every_radius_and_meters_exactly`
  (:144, r ∈ {3,4,5,6}), `the_bracket_rounds_at_six_millimetres` (:169),
  `a_fully_crossing_cylinder_pair_with_no_edge_event_refuses_typed`
  (:203, D10's typed refusal — silence never re-opens),
  `cylinders_standing_clear_of_each_other_still_answer` (:223),
  `the_containment_door_answers_both_directions_on_a_cylinder_chart`
  (:333), `the_containment_door_reports_the_trim_boundary_rather_than
  _guessing` (:393), `a_wall_the_trim_cannot_express_gets_no_verdict`
  (:445), `the_line_clearance_clamp_is_what_lets_a_radial_edge_clear`
  (:495).
- `crates/sweep/tests/verbs_pierce.rs`: all four rows (:65, :96, :117,
  :146) — in particular the 6.643185307179586 union volume that #1068
  fixed on main.
- The three #1044 fixes' arguments, unchanged: `arc_extent`'s
  per-coordinate sagitta charge, the boundary-clipped cylinder face box
  (`boxes.rs`), and the segment-clamped span dip
  (`reduce.rs:1244`, `dip = max(0, q/2 − m)/4` — division-free; the
  comment at :1188-1220 is the argument and moves only with it).
- The D10 extent-certificate posture (CYLCYL PR-A) is not touched.
- `bool_ray_cylinder_disc`'s dimensionless `disc/(2r)²` metering stays
  as flagged at `solid_contain.rs:908-915` — re-normalizing it to match
  the sphere arm moves every #1021 acceptance margin and is a separate
  unit's job.

## Wall 2 does NOT flip — it narrows again

Measured on the LILYWELD door table (`docs/VERBS-LILYWELD-SPEC.md:57-63`)
and the wall's own comment (`demos/tour/src/lily.rs:2001-2019`):

| # | door | why this unit does not move it |
|---|---|---|
| 1 | `gate_operand_pairs` — `CurvedPairUnsupported { Cone, Torus }` | today's refusal; deferred by ruling 2 (`LILYWELD:87-93`) |
| 2 | `gate_maximal_faces` — `NonMaximalFaces` | **#1031**, the binding blocker; triple-demanded |
| 3 | curved-face arm — `CurvedPierceUnsupported { face 3v1 neck CONE, edge 2v1 seam strut }` | **a different pair: line × CONE.** `face_geo` refuses Cone `KindUnsupported` (`solid_contain.rs:404-407`); the line-clearance `f2` fold has no cone arm (`reduce.rs:1228-1237`). No ray×cone roots exist anywhere. |

So this unit's ring lane is the same LANE as wall 2's door 3 but not the
same PAIR, and door 3 is not wall 2's binding door in any case. **Any
claim that this unit flips wall 2 is false**; the spec says so, and
wall 2's retire note stays pointed at #1031.

## PR shape — recommendation: TWO PRs, in this order

Ring lane first, arms second.

- They fail differently. PR-1's risk is a *soundness* question (the
  first-order side verdict on a curving wall, item 3) that can force a
  STOP; PR-2's risk is a *dispatch* question (does the arc-side rule
  serve a two-wall chord). Bundling them means a STOP on the first
  strands the second's measurement inside a dead branch.
- PR-1 has its own consumer and its own closed-form acceptance (a box
  through a wall) that does not depend on any join arm existing. It is
  shippable, testable, and useful on its own — which is the M9-2 shape
  the CYLCYL unit already used successfully.
- PR-1 is shared substrate: SPHSPH's arms consume the same ring lane
  (`docs/VERBS-CYLCYL-SPEC.md:88-93` — "which every germ lane's arms
  consume, so it precedes SPHSPH's arms too"). Landing it alone unblocks
  a second lane before PR-2 is even measured.
- PR-2's own scope is genuinely unmeasured (item 2). A spec that binds
  it to a branch shared with PR-1 would be pricing work nobody has
  looked at.

A one-PR shape would only be right if PR-2 turned out to be a
five-line dispatch addition — and fact 6 says it is not: the *only*
wired curved chord lane is plane-side.

## Difficulty

- **PR-1: M.** Four of the six mechanical pieces are substitutions into
  primitives that already take the right arguments (facts 2, 5, 7; items
  1, 2, 6). The M rather than L is item 3: the curvature charge is a new
  certified argument on a material-side verdict, and this lane's history
  (#1068's two silent wrong answers behind an "honest remainder") says
  first-order verdicts on curved carriers are exactly where wrong bodies
  hide. Add the `cylinder_chart_trim` iso-bounded premise
  (`solid_contain.rs:421-428`) — after a wall pierce the face's boundary
  includes a tilted section curve, so the rectangle trim under-covers;
  whether that bites at the crossing layer or only at finish is
  UNVERIFIED and is a real M-sized unknown.
- **PR-2: M, contingent.** M if the ellipse arc-side rule serves both
  sides (the CYLCYL PR-B item 2 premise); **L→adjudication** if a second
  chord-lane variant is needed, because that is a design widening rather
  than an arm. The opening measurement decides which, and the spec
  should be re-cut if it comes back the second way.
- **The circle × wall unit (NOT specced here): L, and design-gated.**
  It needs a `Decide`-generic degree-2 trig-polynomial root lane that
  does not exist in any form; the honest first step is a design
  conversation, not an implementation.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR drafts. Targeted local runs;
verify hosted coverage at the STEP level (the klint_row lesson — a
green job name is not evidence). Merge origin/main before opening;
confirm CI jobs actually RUNNING; note the drawn point; watch to
completion; cancel detached timers before the final report; kill
detached jobs whose evidence is superseded (the #1085 rule). Do not
merge.
