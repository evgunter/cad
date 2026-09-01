# VERBS-C5ARMS — the two C5 section arms (#1057)

**HOLD (2026-09-01).** PR-1 is held behind **`docs/VERBS-TORAX-SPEC.md`**
(the offset-axial torus arm). PR-1's own item-1 STOP fired at
dispatch: with the flag scratch-flipped, ALL FOUR named consumers
(klein elbow ×2, torus_barrel, teapot wall 1) stop at the
per-chart corner-accumulation refusal (`ReanchorOffCarrier`,
`replace_face.rs:1922-1930`) — real 0.85–6.1 mm corner errors
from rigid per-chart transport, correctly refused, and not a door
a geom-brep section arm can satisfy. Adjudicated option (c)
(VERBS-LOG 2026-09-01): TORAX lands first; rows 3/4/8/12/13
below are unreachable until it does (rows 12/13 unblock on
TORAX's barrel/teapot half if its elbow-split STOP fires).
Branch `verbs/c5arms-1` stays pushed at main with zero commits;
the PR opens when the hold lifts.

Issue #1057, filed from the teapot unit (#1078) and OFF-D PR-2
(#1048). Two arms of the C5 table: `plane × torus` and
`cone × cylinder`. Branches `verbs/c5arms-1`, `verbs/c5arms-2`.
Difficulty pre-logged: PR-1 **M**, PR-2 **M** (reasoning in "PR
shape" below). **Survey run 2026-08-28 against the checkout at
`mngr/kernel-verbs`; three of the issue's own premises were
measured FALSE and the corrections bind — read "What the survey
refuted" FIRST.**

**Ratification note (2026-09-01).** Ratified against main
`e40a85ec` after a full premise re-verification following the
merges of #1290 (SPHSPH), #1353 (GERMARMS PR-2), #1417 (MATE-2),
#1425 (BOOL-2), and #1180 (SHELLFIX PR-2b — the bellied pot). The
three original premise corrections stand, except correction 3's
cylinder claim: the spout's real pair is now `cone × sphere`
(the belly shipped as a SPHERE zone). Nothing this unit plans to
build has been built or foreclosed; every drifted line cite below
is refreshed to that head and tagged where a factual premise
changed.

---

## What the survey refuted (binding premise corrections)

**(1) The arms move the OFFSET/SHELL layer, not the boolean
walls.** #1057's title says "blocking the Klein wall-pair debt"
and its body cites `klein::wall_probes` walls 3 and 4. Those two
walls are the boolean operand gate, and neither one is a
plane×torus or cone×cylinder question:

- wall 3 (call at `demos/tour/src/klein.rs:~823`; pinned by
  `verbs_gate_r1_probes::the_bottles_two_boolean_walls_name_the_pairs_the_scene_claims`,
  `klein.rs:1088-1126`) pins `CurvedPairUnsupported` on
  `{Cone, Plane}`. (Re-cut 2026-09-01: the payload has GROWN
  fields since the survey — `op: None, operand: A`, both faces —
  because the gate is now PAIR-scoped and box-conservative,
  VERBS-GATE. The pair itself is unchanged.) `(Plane, Cone)` is
  **already `implemented: true`**
  (`crates/geom-brep/src/intersect.rs:228-239`, flag at `:230`).
  The refusal is the KIND roster `boolean_arm_exists`
  (`crates/topo/src/boolean/reduce.rs:172` — `Plane | Cylinder |
  Sphere | Nurbs`), which never reads `route`; the roster now
  feeds the pair-scoped box scan
  (`UnsupportedPair`/`first_unsupported_pair`,
  `reduce.rs:~200-240`).
- wall 4 (call at `klein.rs:~846`; test `:1112-1125`, `op:
  Some(Subtract)`) pins `(Torus, Torus)` — a rung-3 arm blocked
  on the torus's exact meters conversion
  (`intersect.rs:340-347`), not in this unit at all.

The arms' real gate is `crates/topo/src/replace_face.rs:1615`,
the ONLY site in the workspace that reads
`route(..).implemented` as a gate (`splitting/mod.rs:315` reads
`route` for a refusal STRING only, as does
`replace_face.rs:512`). That is `replace_face_offset` /
`replace_faces_offset` / `topo::shell`.

**(2) `cone × cylinder` has no Klein consumer.** #1057 says "the
flare adds cone × cylinder". Klein's bulb meridian
(`klein.rs:363-399`) is neck-line → `.fillet` arc → flare-line →
`.tangent_arc` → tube-line, i.e. **cylinder–torus–cone–torus–
cylinder**: the flare is bracketed by two arcs precisely so that
"every G1 joint in the bulb is a construction and not a
coincidence" (its own doc). There is no cone-abutting-cylinder
adjacency anywhere in the bulb. Only `sharp_band`
(`klein.rs:402`) has one, and that is the `fillet_edges` probe
fixture, not a shell ask. (The cone×cylinder FILLET arm has
existed since ARMS-2/#962 — the blend layer, unrelated to this
section arm.)

**Corollary, and it is the honest scope of the whole unit**: the
Klein BULB's `r ± WALL/2` debt does NOT retire with these two
arms. Shelling the bulb needs `cylinder × torus` AND
`cone × torus` (`intersect.rs:304-311`, `:326-333`) — both
rung-3, both blocked on the torus's meters conversion. What
retires is the **two elbows** (`klein.rs:21-28`), whose only rim
pair is `plane × torus`. Two of the bottle's three bodies, not
the bottle.

**(3) The teapot's spout union is not served by the coaxial
arm — and (re-cut 2026-09-01) its real pair is now
`cone × sphere`, not `cone × cylinder`.** The task's caveat is
right about the gate — wall 3 NAMES `(Cone, Plane)`
(`teapot.rs:~1063-1082`, whose own comment says it is "not the
belly wall the spout actually pierces"). But SHELLFIX PR-2b
(#1180) shipped the BELLIED pot: `R_BELLY` is now "the belly's
sphere radius" (`teapot.rs:176`; `vessel_meridian` arc at
`:313-345`), so the pair the spout actually needs is
`cone × sphere` (`intersect.rs:319-323`, unimplemented) — the
draft's own parenthetical became reality. The spout's axis is
still `SPOUT_DIR`, the 3-4-5 direction (`teapot.rs:251-255`),
against the pot's z-axis: **skew, not coaxial**. Nothing this
unit ships serves it, the gate would refuse first anyway, and
teapot wall 3 moves on neither arm. The PR-2 demand argument
loses nothing (`coned_tube` stands on its own); the old
"genuinely needs cone × cylinder" sentence is dead.

This is (re-cut 2026-09-01) approximately the SEVENTH recorded
instance of *stated blocker ≠ binding constraint* — the roster
now lives in `memories/refusal-text-is-not-cause.md` (six
instances by 2026-08-28; `docs/KERNEL-VERBS.md:67` carries the
pointer) — and the second caught in a spec's own opening rather
than in a probe. Record it there when this unit lands.

---

## The closed forms (demand-driven)

### `plane × torus`

Torus `{center c, axis a, R major, r minor, u_ref}`, plane
`{q, n}`. Trileans BEFORE any rung (C5), in this order:

1. `pt_axis_in_plane` — TWO margins: `a·n` metered at `extent`,
   and `(c − q)·n` in meters. Both `Zero` ⇒ the plane CONTAINS
   the axis ⇒ **two meridian circles**, radius `r`, axis `n`,
   centres `c ± m·R` where `m = (n × a).normalize()` is the
   in-plane radial. `u_ref` = `a`. Zero-residual-by-construction
   against both implicit forms in ℝ. *(The LILYWELD-recorded
   schedule; `verbs_shell.rs:598-605`'s "what would retire
   it".)* Guard: `R − r` must be definitely Positive (a spindle
   or horn torus's two circles meet or cross on the axis) —
   refuse typed if not. (Re-cut 2026-09-01: the invariant is
   MEASURED to exist — `geom/src/surfaces.rs:218-228` states the
   convention `R > r > 0`, `sweep::revolve` refuses spindle/horn
   tori at construction, and tier-3 check 1 reports
   `DegenerateTorus` at rest, `validate.rs:406`. The guard is
   cheap insurance against pre-validate STEP-minted tori, not a
   missing invariant; the old STOP condition on this point is
   RETIRED.)
2. `pt_axis_normal` — margin `‖a×n‖` metered at the would-be
   circle radius. `Zero` ⇒ the plane is perpendicular to the
   axis. Let `h = (q − c)·a`; sub-trilean `r − |h|`:
   Positive ⇒ **two concentric circles**, radii `R ± √(r² − h²)`,
   axis `a`, centre `c + a·h`; `Zero` ⇒ ONE circle of radius `R`
   — the tangency, **classification data, not a constructible
   edge** (the `TangentLine`/`ApexTangentLine` precedent,
   `intersect.rs:69-73`, and now the stronger sibling:
   `SphereSphereSection::TangentPoint` refused as a carrier,
   SPHSPH); Negative ⇒ `Empty`.
3. Everything else ⇒ `SectionError::RoutesToGeneralRung`, with
   the bitangent (Villarceau) two-circle case NAMED as
   deliberately unclassified, exactly as the cylinder×cylinder
   arm names skew and the plane×cone arm names the conic trio.

**The cut**: ship 1 and 2, refuse 3. (Re-cut 2026-09-01: BOTH
configurations now have live consumers. Case 1 is the Klein
elbow. Case 2 — axis-normal caps × torus wall — is DEMANDED, no
longer a negligible-cost inclusion:
`demos/tour/tests/…/verbs_teapot.rs:576`
`the_hollow_now_survives_every_axial_junction` pins
`"NeighborPairUnroutable(Plane x Torus)"` (`:613`) on the
`torus_barrel` fixture, and teapot wall 1 pins the same refusal
on the torus-bellied pot.) Villarceau is refused because no
consumer configuration reaches it and its trilean (bitangency at
`R > r` with a specific tilt) is a third classification with no
demand behind it.

### `cone × cylinder`

Cone `{apex, axis a, half_angle α}`, cylinder `{o, axis b, R}`.

1. `cc_cone_axes_parallel` — margin `‖a×b‖` metered at `extent`.
   Definite ⇒ `RoutesToGeneralRung` (a tilted cylinder cuts a
   cone in a quartic).
2. `cc_cone_coaxial` — margin the axis-to-axis distance
   `‖(o − apex).reject_from(a)‖`. Definite ⇒
   `RoutesToGeneralRung` (parallel-but-offset is still quartic).
   `Zero` ⇒ coaxial.
3. Coaxial ⇒ **two circles, one per nappe**: radius `R`, axis
   `a`, centres `apex ± a·(R·cot α)`. Exactly on both surfaces
   in ℝ. No tangency sub-case exists in this lane — a coaxial
   cylinder always cuts a cone transversally — which makes this
   arm strictly simpler than `plane_cone_section`.

**The cut**: coaxial only. The demand is
`verbs_offd::an_undescribable_neighbor_pair_refuses_typed`'s
`coned_tube` (`verbs_offd.rs:76-78`: an r=0.8 cylinder under an
0.8→0.4 cone, coaxial by construction) and the sharp-cornered
revolve family it stands for. Non-coaxial has no consumer this
unit can name, and (per premise correction 3 as re-cut) the one
candidate — the teapot spout — is skew, gated, and now against a
sphere anyway.

---

## The admission machinery (what `false → true` actually costs)

Measured, and it is smaller than it looks — **nothing in the
offset/shell path ever CALLS a section function** (re-measured
2026-09-01: zero `*_section(` calls in `replace_face.rs`,
`shell.rs`, `offset_together.rs`, `offset_axial.rs`):

- `replace_face.rs:1615` consults `route(kind,
  other_kind).implemented` as a boolean and nothing else. The
  moved boundary's geometry comes from `transport_curve`
  (`replace_face.rs:654`, the OLD surface's own offset action
  applied to the stored carrier — the torus arm at `:751` already
  exists) and `plan_reanchors` (`:1843`).
- Correctness after admission is enforced downstream by
  `attach.rs`'s certify (`:258`, `set_edge_curve_via`) and
  `validate.rs`'s re-certification and `DescriptionNotAdjacent`
  (doc `:1997`, raise `:2531`) — which re-derive and never trust
  the stored certificate.

So the flag flip **moves the refusal one door**, and where it
lands must be MEASURED, not assumed. The precedent is in the
suite already: `offd_r1_probes::the_routed_opening_cone_reaches_
past_c5_and_refuses_at_the_caps` (`:184`) is a routed pair
(cone×plane) that gets past C5 and then refuses because "the caps
cannot follow the cone's moved rims" (now pinned specifically as
`ReanchorOffCarrier`).

`SectionCase` / `SectionConic`
(`crates/topo/src/chord_join.rs` — `SectionConic` `:578`,
`SectionCase` `:602`, `section_case` `:633`) and the arc-side
rules from CYLCYL PR-B are the **splitting/boolean** chord layer,
plane-first by construction (`:656-670` refuses every
curved×curved pair typed; the refusal text now carries a
sphere-pair carve-out sentence from SPHSPH). This unit does NOT
touch them; see Fences.

## Consumers table

| # | Site | What it pins today | After PR-1 | After PR-2 |
|---|---|---|---|---|
| 1 | `crates/geom-brep/tests/intersect_table.rs:67` | `(Plane, Torus, General, false)` | row flips (and its rung — see below) | — |
| 2 | `intersect_table.rs:72` | `(Cylinder, Cone, General, false)` | — | row flips |
| 3 | `crates/sweep/tests/verbs_shell.rs:614` `the_klein_wall_pair_waits_on_a_plane_torus_route` (assert `:645-658`) | `ShellError::Face{NeighborPairUnroutable{Plane,Torus}}` on the elbow; its docs carry the comparison it will make | **self-retiring** — re-author to the shell_open form; topology equal, radii within 1 ulp, volume within 1e-12, naturalness not byte-identity | — |
| 4 | `crates/sweep/tests/offd2_r1_probes.rs:549` `probe_late_err_leaves_body_untouched` (expect_err `:583`) | uses plane×torus as its LATE-Err instance | needs a replacement late-Err instance (do not delete the property) | — |
| 5 | `crates/sweep/tests/verbs_offd.rs:317` `an_undescribable_neighbor_pair_refuses_typed` (pin `:325-330`) | `NeighborPairUnroutable{Cone,Cylinder}` on `coned_tube` | — | **self-retiring** |
| 6 | `crates/sweep/tests/offd_r1_probes.rs:109` `opening_nappe_small_d_passes_the_apex_predicate` | uses the C5 refusal as ORDERING evidence (the apex predicate passed) | — | re-express the ordering pin; the property survives, the expected error does not |
| 7 | `offd_r1_probes.rs:156` `a_large_d_away_from_the_apex…` | `expect_err("cone x cylinder still has no route arm")` | — | same |
| 7b | (re-cut 2026-09-01) `offd_r1_probes.rs:113-125` `opening_nappe_small_d…`'s C5 assert | asserts the `{Cone,Cylinder}` refusal itself, not just ordering | — | re-express with 6/7 — the re-express list is THREE tests, not two |
| 8 | `demos/tour/src/klein.rs:70-79` finding 1 + the two elbows | "paid once per wall", both elbows hand-spelled `R ± WALL/2` | the elbows re-author to disc-revolve + `shell_open`; **finding 1 narrows, it does not retire** (the bulb still pays) | — |
| 9 | `klein.rs` walls 3/4 (calls `:~823`/`:~846`; tests `verbs_gate_r1_probes` + `klein.rs:1088-1126`) | `{Cone,Plane}` / `{Torus,Torus}` at the pair-scoped operand gate | **do not move** (premise correction 1) | do not move |
| 10 | `demos/tour/src/teapot.rs` walls 2/3 | (re-cut 2026-09-01) `(Torus,Sphere)` / `(Cone,Plane)` at the operand gate — the belly is a sphere since #1180 | do not move | **do not move** (premise correction 3) |
| 11 | `docs/KERNEL-VERBS.md` shell row (`:52`) | Klein "paid once per wall" | narrowed to the bulb, with the two blocking pairs named | — |
| 12 | (re-cut 2026-09-01) `demos/tour/…/verbs_teapot.rs:576` `the_hollow_now_survives_every_axial_junction` (pin `:613`) | `"NeighborPairUnroutable(Plane x Torus)"` on `torus_barrel` (axis-normal caps × torus wall — case 2) | **self-retiring** — the hollow proceeds to the next honest door; re-pin what it reaches, measured | — |
| 13 | (re-cut 2026-09-01) teapot wall 1 (torus-bellied pot, finding 1) | the same `NeighborPairUnroutable(Plane x Torus)` refusal | moves with row 12 — re-pin measured | — |

## The gate-admission question — ANSWERED, and it is a fence

**Routing an arm does not admit the pair at the operand gate,
and this unit must not widen it.** For a Klein-wall union to
proceed, ALL of these must also open, in order:

1. `boolean_arm_exists` (`reduce.rs:172`) must carry `Cone` and
   `Torus`; `revert_arm_exists` (`:192`) likewise for `∖`/`∩`.
2. The germ-chord section lane — (re-cut 2026-09-01)
   `chord_join::section_case` (`:633`) still refuses every
   curved×curved pair typed, but `boolean::join::
   pair_section_frame` now carries FOUR arms: plane×cylinder,
   plane×sphere, sphere×sphere (SPHSPH), and cylinder×cylinder
   (GERMARMS PR-2 — parallel ⇒ None, coplanar-intersecting ⇒
   `GermFrameCylinderPinch` via `bool_germ_frame_axes_coplanar`,
   `join.rs:856`, skew ⇒ NoArm). Neither lane has any
   torus-bearing or cone-bearing arm; that is the fence this
   unit inherits.
3. The **curved pierce/split door** —
   `CurvedPierceUnsupported` + `PointSplitCarrierUnsupported` —
   the shared substrate every germ lane's arms consume (re-cut
   2026-09-01: established by CYLCYL PR-B's adjudicated scope
   correction, whose spec file has since been deleted in the
   docs sweep; the surviving statements are
   `docs/KERNEL-VERBS.md:67` and the VERBS-LOG record. SPHSPH
   measured its own union stopping at `CurvedPierceUnsupported`
   ABOVE any join arm — the door is load-bearing).
4. LILYWELD PR-2 already MEASURED what a widened gate meets on
   the adjacent pair: `NonMaximalFaces` on axis-touching planar
   caps (the F7 defect, #1031), and one door after that the
   curved PIERCE arm. It ruled the gate-admission question
   **DEFERRED**. (Re-cut 2026-09-01: #1031's POLE half landed
   at #1131 — necessary, not sufficient; the deferral now awaits
   the re-measurement on the repaired lantern, and #1031 stays
   open for the full-valence coplanar pair,
   `KERNEL-VERBS.md:67`.)

That ruling stands and this unit inherits it. The two arms are
the offset/shell layer's admission; the boolean layer is a
separate queue.

---

## PR-1 — `plane × torus`, and the Klein elbow retires

1. **The opening measurement, BEFORE any construction** (the
   LILYWELD PR-2 method, and the reason that spec's own item 1
   was refuted): in a scratch tree flip
   `intersect.rs:271`'s `implemented` to `true` with NO
   constructor, run `verbs_shell.rs:614`'s `shell_open` on the
   klein elbow, AND (re-cut 2026-09-01) the `torus_barrel`
   hollow (`verbs_teapot.rs:576`) — case 2 now has a live
   consumer and its post-flip door must be measured too. Record
   the doors actually reached in the PR body. **STOP for
   adjudication if the next door is not a
   certification/adjacency row this unit can satisfy** (e.g. the
   `shell_open` rim-construction class — #1082's territory, not
   this unit's).
2. `geom_brep::plane_torus_section` + `PlaneTorusSection<T>` in
   `crates/geom-brep/src/intersect.rs`, modelled arm-for-arm on
   `plane_cone_section` (`:1229`) — and (re-cut 2026-09-01) on
   the closer in-file precedent `sphere_sphere_section`
   (`:~808-890`, SPHSPH: named `ss_carrier_*` trileans,
   tangency-as-classification-data): named K-funnel-registered
   trileans with named lever arms, exact-degenerates only,
   generic tilt `RoutesToGeneralRung`. Variants:
   `MeridianCircles { c1, c2 }`, `ConcentricCircles { c1, c2 }`,
   `TangentCircle(Curve3<T>)` (classification data), `Empty`.
   Export through `crates/geom-brep/src/lib.rs` (intersect
   export block `:96-101`).
3. **The table arm's rung is a decision, not a copy.** Today
   `(Plane, Torus)` is `Rung::General`
   (`intersect.rs:269-280`) with a note citing the ℝ³
   implicit-pair marcher, blocked on the torus quartic's meters
   conversion. What ships is a rung-1 `Circle` in two closed
   configurations, so the arm's `rung` and `note` must both be
   re-authored — the note keeping the general-rung sentence for
   the tilted residue exactly as `(Plane, Cone)`'s does.
   `intersect_table.rs`'s `route_inventory` row changes rung AND
   flag; write the reason at the arm.
4. **The demo re-authoring, per the #1048 contract**: both Klein
   elbows become a disc revolve + `shell_open` with both meridian
   caps designated. Contract is **naturalness, not
   byte-identity**: one radius instead of two.
   `verbs_shell.rs:614` becomes the comparison its own docs
   specify — topology exactly equal, stored radii within one ulp
   (`R − WALL/2` against `(R + WALL/2) − WALL`), volume within
   `1e-12`. `klein.rs` finding 1 is NARROWED (the bulb still
   pays) and says which two pairs the bulb waits on.
5. **The sealed arm is a separate answer and must be stated.**
   `topo::shell` (sealed) translates a meridian cap plane along
   its own normal, so the offset cap no longer contains the
   torus axis and its rim is the spiric quartic — outside every
   configuration this arm ships. `verbs_shell.rs:645-658`
   currently asserts only `ShellError::Face { .. }` for that
   path; re-pin it to whatever it becomes and say plainly in
   the docs that the OPENED arm is what the elbow's shape
   admits. **UNVERIFIED — this is a derivation, not a
   measurement; confirm it in the opening probe (item 1) and let
   the measurement rule.**
6. Acceptance: rows 1, 3, 4, 8, 11, 12, 13 of the consumers
   table move as stated; the two closed forms pinned at zero
   residual against both implicit forms on a fixture per
   configuration plus the tangency and the empty case; the
   ill-conditioned band escalates typed (F6) on each trilean;
   klein walls 3/4 and the teapot's walls **bit-identical** —
   (re-cut 2026-09-01) the teapot's walls being wall 2
   `(Torus, Sphere)` and wall 3 `(Cone, Plane)`, the
   bellied-pot pairs; every other existing suite bit-identical.

## PR-2 — `cone × cylinder`, coaxial

1. Same opening measurement on `coned_tube`
   (`verbs_offd.rs:76-78`), same STOP condition.
2. `geom_brep::cone_cylinder_section` +
   `ConeCylinderSection<T>` with one live variant
   `CoaxialCircles { c1, c2 }` and the two
   `RoutesToGeneralRung` refusals (non-parallel, parallel-offset)
   written as the arm's documented decision.
3. Table arm `(Cylinder, Cone)` (`intersect.rs:281-295`, flag
   `:283`) flips with its rung and note re-authored the same way
   as PR-1's.
4. Acceptance: rows 2, 5, 6, 7, 7b move as stated — and rows
   6/7/7b are ORDERING or refusal pins, so they must be
   re-expressed rather than deleted, each keeping the property
   it was written for (the apex predicate runs BEFORE the C5
   gate; a large `d` reads as `ApexWindow` and not as the route
   refusal; the small-d C5 assert names whatever honest door
   replaces it). `coned_tube`'s offset produces a body that
   validates tier-3 with a closed-form volume pin. Teapot walls
   bit-identical. Existing suites bit-identical elsewhere.

---

## Fences

- **The germ/pierce territory is OUT.** No `boolean_arm_exists`
  / `revert_arm_exists` widening, no `chord_join::section_case`
  arm, no `pair_section_frame` arm, no curved pierce/split work.
  The gate-admission question is answered above and stays
  DEFERRED (per LILYWELD PR-2's ruling as amended by #1131's
  landing — the repaired-lantern re-measurement is the trigger,
  not this unit).
- **No klein/teapot boolean wall moves.** If one does, the unit
  has left its layer — STOP.
- **The shell wall-clearance layer is a DIFFERENT layer and must
  not be entangled.** (Re-cut 2026-09-01: SHELLFIX PR-2b is
  LANDED, #1180; what remains of that program is #1055's curved
  residue.) `crates/topo/src/shell.rs`'s `wall_clearance`
  (`:1421-…`, predicate `shell_wall_clearance` `:1445`; the file
  was also touched by SEAT-1) answers per-corner **POINT**
  solves (a plane-to-cylinder/sphere/cone distance question,
  "is there `2t` of material") in `crates/topo`. This unit
  builds section **CURVES** in
  `crates/geom-brep/src/intersect.rs` and flips a routing flag.
  Different crate, different file, different question — they do
  not collide, and neither may be used as evidence for the
  other. State the distinction in the PR body.
- Villarceau/bitangent plane×torus, tilted plane×torus, skew and
  parallel-offset cone×cylinder: all stay
  `RoutesToGeneralRung`, named in the arm.
- (Re-cut 2026-09-01) `Cylinder × Torus`, `Cone × Torus`,
  `Torus × Torus` untouched — the bulb's two blockers are
  NAMED, not built (C12.1: arms retire one at a time).
  `Sphere × Sphere` is dropped from this fence list: it is now
  IMPLEMENTED (`sphere_sphere_section`, `intersect.rs:246-253`,
  SPHSPH #1290) — another program's landing, not this unit's
  concern in either direction.
- No SSI, no crossing-pipeline entry, no marching. Both arms are
  rung-1 closed forms or a typed refusal.

## STOP conditions

- The opening measurement lands on a door outside this unit's
  layer (a rim-construction class, an F7 defect, a surgery the
  existing doors cannot express).
- The elbow's `shell_open` result is validated-but-wrong rather
  than refused (the #1048 MAJ-1 lesson: a validated wrong body
  is never acceptable; a typed refusal is).
- (Re-cut 2026-09-01) ~~`Surface::Torus` turns out to admit
  `r ≥ R`~~ — RETIRED: the invariant is measured to exist
  (`surfaces.rs:218-228`, revolve-time refusal, tier-3
  `DegenerateTorus` at `validate.rs:406`). The meridian-circle
  guard ships as cheap insurance against pre-validate
  STEP-minted tori, and is not a STOP.

## PR shape and difficulty

**Two PRs, sequenced, not one.** The arms share a file and a
mechanism but nothing else: disjoint consumers, disjoint
fixtures, disjoint acceptance corpora, and PR-1 carries a demo
re-authoring PR-2 does not. Landing them together would put the
Klein re-authoring's naturalness judgement in the same review as
a two-line cone arm.

- **PR-1 = M.** The closed forms are easy; the work is the demo
  re-authoring under the naturalness contract, the rung/note
  re-decision, the sealed-vs-opened answer (item 5, a derivation
  this survey could not run code to confirm), and (re-cut
  2026-09-01) the two case-2 consumers' measured re-pins.
- **PR-2 = M, at the low end** — arguably L. One configuration,
  one variant, no tangency sub-case, no demo. It is M only
  because rows 6/7/7b are ordering and refusal pins that must be
  re-expressed thoughtfully rather than re-asserted, and getting
  that wrong silently removes a real property.

## Lane obligations (both PRs)

`docs/prompts/implementer-discipline.md` binds. No
Co-Authored-By trailer (blinding). Lane-private PR drafts.
Targeted local runs only — hosted CI verified at the STEP level
is the suite evidence (a green job name is not evidence; the
regular suite is not re-run locally, per Evan's 2026-08-31
method ruling). Merge origin/main before opening; confirm CI
jobs actually RUNNING; note the drawn point; watch to
completion; cancel detached timers before the final report; kill
detached jobs whose evidence is superseded (the #1085 rule). Do
not merge.
