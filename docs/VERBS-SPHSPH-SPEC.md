# VERBS-SPHSPH — the sphere×sphere germ lane (ONE PR; the arms are a separate, blocked unit)

Wave 2 row 9 of `docs/VERBS-PLAN.md:147-154`, promoted ahead of
CYLSPH by the 7 → 9 → 8 reorder (`docs/VERBS-CYLCYL-SPEC.md:95-104`).
Branch `verbs/sphsph`. Difficulty logged pre-dispatch: **L**.
Substrate survey 2026-08-28, anchors read on `mngr/kernel-verbs`
@ `3820532f`; **no code was run** — every claim about what a door
*does at runtime* is marked UNVERIFIED and is the implementer's
opening measurement.

**Two premise corrections bind before anything is built.** Both
correct the promotion sentence at `docs/VERBS-CYLCYL-SPEC.md:99-101`
("after PR-A its only new door is the partial-sphere-face containment
arm"):

- **(P1) The arms are NOT available in this unit.** A sphere×sphere
  crossing is found only through a seam meridian of one ball piercing
  the *other ball's sphere face* — a pierce into a CURVED face. That
  is the exact machinery `docs/VERBS-PIERCE-SPEC.md:68-110` STOPPED
  on and which is still unadjudicated: `vtxfac`'s ring lane reads the
  pierced face's PLANE (`reduce.rs:433-442`'s `face_plane` returns
  `None` for any non-plane surface; `vtxfac.rs:94`, `:192`, `:435`'s
  straight `line_between` chord). SPHSPH's arms wait on the same ring
  lane CYLCYL's do. This unit therefore ships the substrate and moves
  the doors one layer down, honestly — it does not flip a union.
- **(P2) There is a THIRD new door, not one.** Besides the
  partial-sphere-face containment arm, the **arc-side rule refuses
  every non-polar sphere section** (`chord_join.rs:674-690`,
  `split_sphere_section_polar`), and `props`' sphere flux arm refuses
  a non-polar rim by the same condition (`props/curved.rs:1175-1203`).
  A sphere×sphere section circle's axis is the CENTRE LINE, so the
  lane is gated on both charts' polar axes being parallel to `c₂−c₁`.
  Ruling that gate — declared, re-charted, or refused — is this
  unit's central design decision, and it is stated in §2.

## 1. The exact section (the sibling `plane_sphere_section` names)

`plane_sphere_section` lives at `crates/geom-brep/src/intersect.rs:689`
with its classified enum at `:659-673` (`Circle` / `TangentPoint` /
`Empty`), one trilean `ps_center_gap` at `:746`, and the `u_ref`
placement tie-break `ps_frame_seam` at `:734-745` (D9: a deterministic
placement, never a verdict). Its route row is
`intersect.rs:239-245`; the sibling's is `intersect.rs:246-251` —
`Rung::Closed, implemented: false`, pinned as such in
`crates/geom-brep/tests/intersect_table.rs:82` and gated by
`refusal_is_grounded` (`intersect_table.rs:37-46`).

1. **`sphere_sphere_section<T: Decide>(a, b, band)`**, same shape:
   `d = ‖c₂−c₁‖`, `n̂ = (c₂−c₁)/d`, `a = (d² + r₁² − r₂²)/(2d)`,
   centre `c₁ + n̂·a`, axis `n̂`, radius `√((r₁−a)(r₁+a))`. Closed
   form, zero residual against both surfaces by construction, no
   `atan2` and no branch cut — the interval lane takes it unchanged
   (contrast `ssi.rs:735`'s f64-only `cylinder_sphere_ssi`).
2. **The trilean skeleton already exists in 2-D** and must be
   mirrored, not reinvented: `crates/profile/src/seg.rs:562-621`'s
   `arc_arc` carries the identical algebra
   (`(d² + r₁² − r₂²)/2d`, then `√(r₁² − a²)`) under three named
   trileans — `carrier_circles_identity`, `carrier_circles_external`,
   `carrier_circles_internal` (`seg.rs:540-560`). The 3-D lane needs
   the same four verdicts: **Circle**, **TangentPoint** (external or
   internal tangency — C7: classification data, never a carrier),
   **Empty** (separated or strictly nested), and **Coincident**
   (`d` and `|r₁−r₂|` both zero — a whole-sphere REST, never a
   section; refuse typed, CONTACT-DESIGN C2/C4 forbid inferring the
   gluing at any ε).
3. **`u_ref` is a placement, and the sphere sibling has the SAME
   degeneracy** `ps_frame_seam` guards: the section axis is `n̂`, and
   a candidate `n̂ × û` collapses when the centre line is along the
   operand's own seam direction. Reuse `intersect.rs:715-745`'s
   two-candidate tie-break verbatim, derived from operand A's chart
   (`n̂ × û₁`, else `n̂ × â₁`), with the same "no downstream VERDICT
   moves with it" note.
4. **Nothing in the repo computes this today.** The fillet family's
   sphere×sphere arm is a different layer and shares no code: it is
   `BlendArm::SphereSphereTorus` (`sweep/src/fillet/blend.rs:156`,
   dispatched at `fillet/battery.rs:873`), and its closed form
   (`blend.rs:585-609`) is the radical-plane station on the OFFSET
   radii `Rᵢ ∓ r` — it returns the fillet ball's centre in a meridian
   sheet, is deliberately total ("nothing is gated here",
   `blend.rs:524-536`), and lives in `sweep`, which depends on
   `geom-brep` and not the reverse. What IS shared is the fixture
   vocabulary (the lentil, `sweep/tests/verbs_arms3.rs:80`) and
   nothing else. Say so in the PR body; the two must not be conflated
   in the register.
5. Flip `intersect.rs:246-251` to `implemented: true` with a
   `retired this arm` comment on `intersect_table.rs:82` (the pattern
   `(Plane, Sphere)` sets at `:65-66`), a revised note, and the
   file's own two-test pattern (`intersect_table.rs:846-932`):
   zero-residual against both surfaces, and the trilean trio.

## 2. The polar gate — this unit's design decision (ADJUDICATE FIRST)

Three independent sites demand that a sphere section circle be a
LATITUDE rim of the chart it rides:

- `chord_join.rs:674-690` — `split_sphere_section_polar` refuses a
  tilted section because "the azimuth-anchored arc-side rule premises
  azimuth MONOTONE along the carrier and that holds on a sphere chart
  only for polar sections".
- `props/curved.rs:1175-1203` — `props_circle_axis_class` sorts a
  boundary circle into rim (axis ∥ polar axis, then
  `require_rim_incidence`) or meridian great circle (axis ⊥ polar
  axis, centred at the sphere centre). A tilted rim is
  `PropsError::NotIsoRectangle` — so a tilted-rim result body has no
  certified volume at all.
- The chart trim of §3 (latitude window) has the same premise.

The section's axis is `c₂−c₁`. **Three candidate rulings; the spec
picks one before the branch opens:**

- **(a) Declared/structural alignment** (the `RadiusEvidence`
  precedent, `intersect.rs:770-777`): the lane acts only when both
  operands' polar axes are structurally parallel to the centre line,
  and refuses typed otherwise. Cheapest; makes the ordinary
  two-ball union un-servable, so it buys the section and nothing else.
- **(b) Rigid re-chart, the M5 S13 move.** `apply_recuts`
  (`ops.rs:1881-1960+`) already rotates a closed sphere group about
  its own centre — the same point set, seams re-aimed — by an
  ALGEBRAIC Rodrigues form with no trig (`ops.rs:1935-1951`, and the
  reason is the interval lane). A sphere×sphere pair's re-chart
  target is `n̂`. **Recommended.** But note the honest cost: today
  that machinery runs ONLY from the no-crossings fallback
  (`ops.rs:482-491`), and re-aiming both poles at the centre line
  guarantees each seam crosses the section circle, i.e. it *creates*
  the curved pierce (P1) rather than avoiding it.
- **(c) Do neither and keep the refusal sharp.** The section lands,
  the dispatch arm lands, and the polar condition is a named typed
  refusal at the germ frame. This is what §5's acceptance is written
  against, because (b) buys nothing until the ring lane exists.

**Recommendation: land (a)+(c) now — the section plus a named polar
gate — and record (b) as the arms unit's opening move.** State the
ruling explicitly in the PR body; do not leave it implicit in code.

## 3. The partial-sphere-face containment arm

**What `curved_face_containment` covers today** (`contain.rs:582-658`):
the boundary walk first (`curved_boundary_containment`, `:317-328`,
which is `Circle`/`Line` rows plus `UnrowedCarriers::Undecided`), then
`Ok(None)` for any face with rings (`:592-594`), then **a hard
`Surface::Cylinder` match — every other kind is `Ok(None)`**
(`:595-603`). Behind it: the carrier row `bool_curved_contain_carrier`
(`:612-620`), the ISO-BOUNDED class test `iso_bounded_wall`
(`:689-745`: every boundary edge a rim or a meridian), the full-period
guard `bool_curved_contain_period` (`:641-649`), and the trim
`cylinder_chart_trim` / `point_on_wall_in_face`
(`solid_contain.rs:435`, `:512`).

**What the solid door does with a sphere face today**: a CLOSED
sphere group (all faces on one sphere key, closed against each other)
gets an exact arm (`solid_contain.rs:392-402`, `FaceGeo::Sphere`, the
ray/sphere quadratic at `:901-957`); a **trimmed** one refuses
`PartialSphereFace` (`:148-161`, message at `:210-224`). The extent
scan refuses the same class typed (`ops.rs:1478-1486`).

1. **The deliverable is the sphere analogue of the iso-bounded
   class**: no rings; every boundary edge either a **latitude rim**
   (circle, axis ∥ polar axis, `require_rim_incidence`'s placement)
   or a **meridian arc** (great circle, axis ⊥ polar axis, centred at
   the sphere centre) — the same two classes
   `props/curved.rs:1150-1230` already parses. Then the chart
   rectangle is `[az] × [lat]` and `point_on_sphere_in_face` is the
   azimuth window (reusable verbatim: `point_on_wall_in_face`'s
   cosine construction, `solid_contain.rs:524-560`) crossed with a
   latitude window.
2. **`PartialSphereFace` retires for exactly that class**, and keeps
   its refusal for everything else. `solid_contain.rs:148-156`'s
   stated blocker is **already stale and must be corrected in the
   same PR**: it says "`topo::pcurves::chart_mints` is `false` there",
   but `pcurves.rs:264` mints for `Sphere` since M6-3 (the
   analytic-chart completion, "polar/meridian" classes named in the
   comment). The recourse sentence at `:219-222` goes with it.
3. **Where the #723/#893 caution BITES — exactly two places, and
   neither is the section.** Both issues are `props`-layer defects on
   sphere polar rims, in the ACCEPTING direction:
   - **#893 (`props/curved.rs:1193`, `RimLevel::Unit(sin_v, 0)`)** —
     the level margin is `R·|Δ sin v|`, the AXIAL separation, which
     collapses as `cos v̄ → 0`. Two distinct near-polar rims decide
     `Zero` and `require_rims_at_extremes` (`:579-600`,
     `props_rim_level`) passes a non-rectangular domain. **A
     sphere×sphere result hits this as its ordinary case**: with the
     charts polar-aligned, a shallow overlap puts the section rim
     near a pole. **The latitude window of item 1 must NOT reuse
     `sin v` as its margin** — use a lever that does not collapse
     (the polar-angle/chord form) or refuse explicitly in the
     near-polar regime, and say which. The near-polar interior-rim
     row #893 asks for (its item 1) is a natural rider here.
   - **#723 (endpoint-derived extents)** — `min_max`
     (`props/curved.rs:882-895`) folds only edge ENDPOINT levels, so
     a meridian arc whose interior crosses a pole understates the
     `v`-extent (−47% certified volume, tier-3 green). The same
     derivation is what `cylinder_chart_trim` does for `h`
     (`solid_contain.rs:461-479`: the loop's VERTEX points). On a
     cylinder it is sound because the iso-bounded class makes both
     coordinates monotone along every edge; **on a sphere it is not
     — latitude is not monotone along a meridian arc through a
     pole.** The sphere trim must either derive its latitude window
     from the arc SPAN (the torus's move, `props/curved.rs:1244-1253`
     — the one kind #723 does not reach) or carry an explicit
     invariant that no boundary edge contains a pole in its interior,
     planted red.
   - **Disposition, explicit:** this unit does NOT fix #723 or #893;
     it (i) writes the trim so that neither mechanism is inherited,
     (ii) plants the near-polar red #893 asks for, and (iii) states
     in the PR body that any closed-form volume pin on a near-polar
     sphere face is measured through a props lane with a known
     accepting defect. If acceptance §5 cannot be made meaningful
     without the props fix, **STOP** and split it out.

## 4. The dispatch arms

1. **`pair_section_frame`** (`join.rs:690-789`) — the pair-general
   germ frame CYLCYL PR-A generalized. Add the `(Sphere, Sphere)`
   arm returning `(centre, axis)` of §1's circle, mirroring the
   `(Plane, Sphere)` arm at `:701-730` including its `Desync` arms
   for `TangentPoint`/`Empty`. The trap CYLCYL PR-A closed stays
   closed: `_ => Err(FrameError::NoArm)` at `:768` — never `None`,
   because the caller reads `None` as "straight chord" (`:681-688`).
2. **`section_case`** (`chord_join.rs:602-699`) explicitly refuses
   every curved×curved pair at `:623-629` ("a chord's section pair
   has no plane"). That refusal is CORRECT for this unit and stays;
   sharpen its text to name the sphere pair and the polar gate rather
   than widening it. The arc-side rule needs a chart the germ pair
   does not have; that is arms territory.
3. **The extent scan's sphere×sphere arm** (`ops.rs:1644-1687`)
   currently reads "the sphere×sphere germ arm (a closed-form Circle)
   has no join lane in this build". After §1 that sentence is half
   false: the Circle exists, the JOIN does not. Re-word to name the
   join's absent arm — the doors move one layer down, which is this
   unit's whole shape. The pinned assertion is only
   `what.contains("sphere")` (`m5_s13_pips.rs:342`), so the re-word
   is free; but `m5_pr9c_sphere_doors.rs:285` asserts the retired
   string `"no seam lane"` is ABSENT — do not reintroduce it.

## 5. The opening measurement (the GATE precedent; do this FIRST)

Two crossing unit balls give **two different doors depending on the
offset direction relative to the operand's seam**, and the survey
could not run either. `ball_at` (`m5_s13_pips.rs:74-90`) revolves an
XY-plane profile about world Y, so the polar axis is **Y** and the
seam great circle lies in the **XY plane** (azimuth 0 = +X and its π
copy).

| row | configuration | predicted door | status |
|---|---|---|---|
| 1 | offset along **Z** (⊥ the seam plane), overlap shallow enough that neither cap reaches the seam | `FallbackExtentUnsupported` at `ops.rs:1675-1683` | **MEASURED, green today** — `m5_s13_pips.rs:334-343`, whose own doc says the pair is "offset VERTICALLY so neither ball's seam edges enter the other's certified box" |
| 2 | offset along **X** (in the seam plane) or along **Y** (the polar axis) | `CurvedPierceUnsupported { operand, face, edge, band }` from `reduce.rs:952`, reached through `curved_face_arm`'s Circle branch at `reduce.rs:1044-1121` (`bool_circle_curved_clearance` at `:1079` deciding Zero/Negative at `:1118`) | **UNVERIFIED — measure it** |
| 3 | deep Z overlap (cap reaches past the seam) | row 2's door | **UNVERIFIED** |
| 4 | nested balls | already answers: `probe_nested_spheres_union_to_the_outer_ball`, `m5_s13_review_probes.rs:281-311`, volume `4π/3` | **MEASURED, green — must stay bit-identical** |

The measurement drives everything: if row 2 is confirmed, P1 is
confirmed with it and the unit's shape is settled. Land the table in
the PR body. Supporting reads (no runtime claim): `Sphere` has a
revert arm (`reduce.rs:192-197`) so the non-union front gate at
`ops.rs:418-431` admits both ops; `circle_residual_extremes` has a
tight sphere arm (`implicit.rs:356-365`, test at `:672`); the Circle
edge SPLIT already exists (PIERCE door 1 shipped —
`reduce.rs:1456-1521`, `circle_split_param` at `:1532`), so the
missing half really is the ring insert alone.

## 6. Fences

- **No torus and no cone work**, in any file this unit opens
  (`route`'s cone/torus rows, `props`' torus arm, the operand gate's
  kind rows). Klein walls 3–4 and teapot walls 2–3 are row 10's.
- **No join arms, no ring lane, no `vtxfac` work.** After this unit
  a sphere×sphere crossing must still refuse typed — but at the
  layer that actually lacks the machinery, naming it.
- No CYLSPH work: `m5_s13_pips.rs:364-400`'s `cyl×sphere` arm and its
  message stay verbatim (row 8's).
- No `SectionConic` widening, no `select_arc` change, no azimuth-rule
  change (`chord_join.rs:782`, `:1587`).
- The D10 extent-certificate posture and `cylinder_extent_gate` are
  untouched.
- **The fillet family is out of scope in both directions**: no change
  to `BlendArm::SphereSphereTorus` or its roster strings
  (`review_arms2_r1_probes.rs:337-340`,
  `review_arms3_r1_probes.rs:493`), and the register must not let the
  boolean row absorb the fillet row (`docs/KERNEL-VERBS.md:39` —
  "Sphere×sphere SHIPPED (VERBS-ARMS-3)" — does **not** move).
- #1076's general arc-aware ray parity and #1031's
  `merge_coplanar_faces` repair are both out.

## 7. Acceptance

- **Closed forms, both ops, pinned at several overlap depths.** Two
  balls radius `r₁, r₂`, centre distance `d`, crossing:
  the lens volume is `π(r₁+r₂−d)²·(d² + 2d(r₁+r₂) − 3(r₁−r₂)²)/(12d)`,
  the two cap heights are `h₁ = r₁ − a`, `h₂ = r₂ − (d−a)` for
  `a = (d²+r₁²−r₂²)/2d`, each cap `πh²(3r−h)/3` (the helper already
  exists: `m5_s13_pips.rs:93-95`). Union `= V₁+V₂−V_lens`,
  subtract `= V₁ − V_lens`, intersect `= V_lens`. **Where the arms do
  not land, these are written as the SUCCESSOR unit's acceptance and
  the refusal is pinned instead** — say which, per row of §5's table.
- **The section's own pins** (`intersect_table.rs` pattern): zero
  residual against both surfaces at a generic pair; the trilean trio
  (separated → `Empty`, tangent → `TangentPoint`, nested → `Empty`,
  concentric-equal → typed refusal); wrong-lane kinds refuse; the
  in-band twin of each definite verdict escalates through the named
  predicate (F6, two-tolerance).
- **Containment pinned both directions on a sphere chart**: inside /
  outside / boundary-adjacent at the rim and at a meridian, on a
  face of the iso-bounded class; a ringed sphere face and a
  non-iso-bounded one still answer `None`; a full-period azimuth
  window still answers `None`.
- **The near-polar red (#893's item 1)**: a face whose two rims are
  both near a pole and genuinely distinct must not decide `Zero`.
  Planted red at the current lever, green at the new one — or, if
  the disposition is refusal, a typed refusal both directions.
- **The pole-in-edge-interior invariant (#723)**: a planted sphere
  face whose meridian boundary edge contains a pole strictly inside
  refuses typed rather than answering from endpoint levels.
- **Differentials**: every new predicate gets its in-band twin
  (`ps_*`/`ss_*` naming, the `plane_sphere_section` precedent); the
  f64 and Interval lanes both run the section (no lane fork — the
  form is `atan2`-free by construction, the PIERCE door-1 lesson).
- **Bit-identity**: `m5_s13_review_probes.rs:281-311` (nested union),
  `:148-181`, `:376-398` (the plane×sphere escape family),
  `m5_pr9c_sphere_doors.rs` in full, `m5_s13_pips.rs:300-326` and
  `:364-400`, the die-pip corpus (`m5_pr12_die.rs`,
  `editor-core/tests/corpus/die_pips.rs`), and every existing boolean
  suite. The tour walls that must NOT move are listed in §8.

## 8. Consumers — what moves and what conspicuously does not

**Moves:** `docs/KERNEL-VERBS.md:13-15` (the "plane×cyl /
plane×sphere" parenthesis — a third wired *section* class) and
**`:51`, the owning row** ("curved boolean breadth"; its
"**No germ class was added**" sentence, and its stale `(Cone, Sphere)`
sentence, which #1001/M9-5 already measured false — fix both while
the file is open). `docs/M9-5-SPEC.md:126-127` cites the row as
"KERNEL-VERBS.md:43"; it is now `:51`. `m5_s13_pips.rs:334-343`'s
message re-word (§4.3).

**Does NOT move — state this in the PR body so the demo expectations
stay honest:**

- **lily wall 7** (`demos/tour/src/lily.rs:2189-2204`, the
  sphere×sphere tepal-seam SUBTRACT) — it refuses `NonMaximalFaces`
  from the operand's axis-touching planar caps (the F7 defect, #1031)
  **before** any germ arm is reached. The wall's own note says so
  (`lily.rs:2185-2188`), and so does its probe
  (`lily.rs:3520-3671`, `:3656-3662`). SPHSPH alone does not flip it.
- **lily walls 1, 2, 8, 12; klein walls 3–4; teapot walls 2–3;
  bud; projectbox** — all other kinds or other doors.
- **`docs/KERNEL-VERBS.md:39`** (the fillet row) and every ARMS-2/
  ARMS-3 test.
- The teapot's waiting list (`docs/KERNEL-VERBS.md:64-77`) is
  torus×cylinder and cone×plane.

**Adjacent, and the spec must RULE on them** (they are the
partial-sphere-face door's real consumers, not sphere×sphere's):
`m5_s13_pips.rs:344-362` (trimmed-group operand),
`editor-core/tests/m6_5_downstream.rs:263-311` and
`review_m6_5_pr2_sweep_probes.rs:49-72` (both pin
`FallbackExtentUnsupported` for a body carrying sphere OCTANTS
against a DISJOINT operand, and both carry a "replace this pin when
the extent frontier moves" instruction), `verbs_gate_r1_probes.rs:151-200`
(`PartialSphereFace` in containment), and the die demo's one-shell
workaround (`demos/tour/src/diefillet.rs:242-252`, `:508-522` — 21
pips cut as ONE tool precisely because a trimmed sphere face cannot
be an operand). **If §3 lands, several of these flip.** Decide in the
spec whether the extent scan's trimmed-group arm
(`ops.rs:1478-1486`) is inside this unit's scope; the survey's
recommendation is **yes** — it is the same certificate — and if so
those four pins are re-baselined with their instructions followed.

## 9. PR shape and difficulty

**ONE PR, difficulty L.** Not two. §1's section and §3's chart trim
share the deliverable that makes both possible — the sphere chart's
(azimuth, latitude) window with a non-collapsing latitude lever — and
splitting them means writing it in a PR that cannot exercise it.
§4's dispatch arms are a handful of lines on top of §1. The CYLCYL
two-PR shape was justified by PR-A being *shared* substrate several
later lanes consume; here the shared half (§3) and the lane-specific
half (§1) are the same size and the same file-set.

**The arms are a SEPARATE, currently BLOCKED unit** — call it
SPHSPH-ARMS — gated on the curved pierce ring lane
(`docs/VERBS-PIERCE-SPEC.md`'s door-2 STOP, unadjudicated). Do not
spec it until that adjudication lands; CYLCYL PR-B's scope correction
(`docs/VERBS-CYLCYL-SPEC.md:77-93`) is the precedent for what
speccing arms on an unmeasured premise costs.

**Why L and not M**: three files with dense invariant prose
(`intersect.rs`, `contain.rs`, `solid_contain.rs`), a props-adjacent
polar disposition that must be argued rather than coded around, four
existing pins to re-baseline with their own instructions, and a
register row that is stale in two independent ways.

## 10. Fences on the process — STOP conditions

- **STOP for adjudication if §2's polar gate cannot be ruled from
  the survey's three options** — in particular if the implementer
  finds that (b), the rigid re-chart, is required for the section to
  have any consumer at all.
- **STOP if §3's chart trim needs machinery the survey did not map**
  — the CYLCYL PR-A STOP, verbatim in shape (e.g. a sphere face form
  the rectangle cannot express that is nonetheless the ordinary
  output of a sphere×sphere cut).
- **STOP if §7's acceptance cannot be made meaningful without fixing
  #723 or #893** — that fix is a props unit, not this one, and the
  split must be adjudicated rather than absorbed.
- **STOP if the §5 opening measurement contradicts P1** — i.e. if a
  sphere×sphere crossing reaches the JOIN layer without a curved
  pierce. The whole unit's shape is downstream of that row.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Lane-private PR draft. Targeted local runs;
verify hosted coverage at the STEP level (the klint_row lesson — a
green job name is not evidence). Merge origin/main before opening;
confirm CI jobs actually RUNNING (a CONFLICTING PR gets no run,
silently — verify one fires); note the drawn point and coverage;
watch to completion; cancel detached timers before the final report;
kill detached jobs whose evidence is superseded (the #1085 rule).
Do not merge.
