# Missing kernel operations — the modeling-verb register

**Status: reference register (Evan's ask, 2026-08-09).** The
modeling operations the kernel does NOT yet have, each with its
prerequisites as the ratified records state them. These are
feature-breadth work, and none of it is scheduled by this register;
this register is their home so they stop living in scattered walk
rows and banked notes. DESIGN.md's Band 3 "feature breadth"
paragraph names the GUI-era shape of several; this register is the
kernel-side view with dependencies. Rows move to a milestone plan
when scheduled; the register never schedules anything itself.

**Present today, for contrast**: extrude, revolve (partial/full),
loft, sweep (straight + curved single-arc path), booleans (planar
complete; curved per wired germ classes plane×cyl / plane×sphere),
split, constant-radius edge fillets (**nine analytic support-pair
arms**: plane–plane, plane–sphere and the coaxial-revolution seven —
sphere–cone, cone–plane(⊥), cone–cone, cylinder–cone,
cylinder–sphere, cylinder–plane(⊥), sphere–sphere; the two ruled arms
classify but no door carves their band; closed rims covered since
VERBS-ARMS-1 — the N-link ladder rim and the ANNULUS rim of a full
solid of revolution, the latter for annular AND pole-touching profiles
alike since the seam-split rim's multi-link band, see the row below)
+ in-place composition surgery,
**symmetric-setback edge chamfers** (`chamfer_edges`, **plane–plane
supports only, convex chains, open chains between fully-requested
trivalent corners** — see the row below),
merge_coplanar_faces, rigid transform, **shell / hollow** (sealed and
opened, `topo::shell` / `topo::shell_open` — with the reach bound its
own demo measured, in "Scope limits" below: the sealed arm's PER-CHART
door survives exactly the plane-normal-to-cylinder junction, and since
#1081 PR-2a an ALL-PLANAR body takes the simultaneous door instead and
its oblique corners hollow; curved corners still take the per-chart
door (PR-2b)), **offset of a surface** (analytic exactly, NURBS
through the certified intensional route — no body-level offset verb
for a body with any curved face, per row 36),
**patterns** (linear / circular / explicit, part-level), **datum
planes / axes / points**, **declared REST joins**,
tessellation/STL/STEP export, STEP import (adoption incl. recognition
+ tier gate).

| verb | what | prerequisites / blockers (as ratified) | notes |
|---|---|---|---|
| **shell / hollow** | offset the boundary inward, remove faces, thicken | Q8: offset surfaces — analytic kinds are CLOSED under offset (a D3 payoff); a NURBS offset is NOT a NURBS → needs the approximating-surface machinery (intensional spec `Offset(S,d)` + fit + certified residual ≤ ε, "exactly mirroring fitted intersection curves"). Also wants open-shell/face-removal vocabulary (D1's manifold-first boundary) | **SHIPPED at #1048** (`topo::shell` / `topo::shell_open`), and its designated demo has now MEASURED its reach — the two teapot rows in "Scope limits" below are that measurement: the sealed arm survived exactly the plane-normal-to-cylinder junction and refused every oblique one until #1081 made the offsets simultaneous (PR-2a for planar corners, PR-2b for a body of revolution's, in its meridian half-plane), and the OPENED arm was wrong on every solid of revolution until #1082 was repaired. **The Utah teapot was this verb's designated demo** (Evan, 2026-08-09) — a vessel is a shelled revolve; the demo has landed (`demos/tour/src/teapot.rs`) and ships an OPENED pot whose belly is the ARC a potter would draw — both halves of #1081 have landed, and the pot's shoulders are turned. **Second consumer, and the first to PAY for the absence in full: the Klein bottle** (`demos/tour/src/klein.rs`, 2026-08-16). A thin 3-manifold whose midsurface is the immersed Klein bottle is nothing but shell, so every wall is authored as its own two offsets by hand — each radius spelled twice as `r ± t/2`, each blend radius twice as `R ∓ t/2` with the sign depending on which side the centre of curvature is on, and the offsets swapping sides wherever the surface turns back on itself. It builds and it is exact; it is the whole row, paid once per wall |
| **offset (surface/solid)** | the standalone Q8 operation | same as shell's core; Q8 says "needed before shelling/offset work (M5+), stated now" | **The SURFACE half is SHIPPED** (VERBS-OFF-A/B/C): the analytic kinds are closed under offset and mint exactly through `geom_brep::offset_surface`; a NURBS base takes the intensional route instead — `SurfaceDescription::Offset` + fit + certified residual, at rest as `Surface::Approx`, whose certificate the validator re-derives per face rather than trusting. The face-level door over it is `topo::replace_face_offset` (VERBS-OFF-D PR-1), and `shell` is its only consumer. What is NOT built is the body-level offset verb for a body with any CURVED face; the ALL-PLANAR case has been exposed since #1081 PR-2a by `topo::offset_planes_together`, which offsets every chart of a body at once and refuses `TogetherNonPlanar` on anything else |
| **chamfer** | the fillet's ruled-surface sibling | **Shipped for plane–plane supports** (VERBS-CHAMFER): `sweep::chamfer::chamfer_edges` at equal setback, over the fillet's own battery, admission doors and composition surgery — a flat strip per edge and a flat patch per trivalent corner, every face an exact `Surface::Plane`. What is NOT shipped and what blocks it: **curved supports** (the strip over a curved support is VERBS-ARMS' machinery, refused `ChamferArmUnsupported`); **concave chains** (refused at the same two admission doors the fillet's concave case is, and the corner-configuration classifier reaches first); **asymmetric parameters** (distance–distance, distance–angle — a widening of the same door, nothing forecloses it); **closed chains**; **a recipe-layer `Node::chamfer`**, without which the verb is unreachable from a document and mints no names | The register's own "cheapest entry" call held: the verb is a front door, one blend arm, one corner patch and three parameterized decisions in the shared surgery |
| **constant-radius fillet on CURVED support pairs** | the arms of C8's analytic table that M5 PR 12 did not implement — sphere×cone, cone×plane, cone×cone, sphere×sphere, and the cylinder pairs | **The COAXIAL half is SHIPPED (VERBS-ARMS-2).** Eight arms from ONE derivation: when a support pair carries a symmetry the rolling ball inherits — a common axis of revolution, or a common ruling — the ball's centre is confined to a SHEET (the meridian half-plane through the rim; the cross-section normal to the ruling), where each support cuts a LINE or a CIRCLE and the centre is the crossing of the two OFFSET traces, on the branch that returns the rim as `r → 0`. Coaxial six → TORUS (sphere×cone, cone×plane(⊥), cone×cone, cylinder×cone, cylinder×sphere, cylinder×plane(⊥)); ruled two → CYLINDER (cylinder×cylinder(∥), cylinder×plane(∥)). **No constant-radius arm mints a cone** — that is the variable-radius family, and C8's prose now says so. The closed-rim surgery's gates were re-cut by SHAPE rather than by KIND so the annulus band carries any pair of revolution walls, and `geom-brep`'s tangent-certificate circle arm grew a CONE row so such a band can be described at rest. **Sphere×sphere SHIPPED (VERBS-ARMS-3)** as the ninth coaxial arm and the only one whose shared-axis hypothesis is FREE: two spheres on distinct centres always meet in a circle, and the line through the centres is that circle's own axis, so the `fillet3_support_coaxiality` margin is zero by construction rather than by luck. It is a pure reduction — the circle×circle sheet crossing ARMS-2 already derived, with both spheres' stored sense bits folded in. Consumer, MET: a lentil (the solid between two unit spheres, bored) whose convex equator fillets end to end through the annulus door, tier-3 valid, the band's spine at `√((R − r)² − c²)` (`crates/sweep/tests/verbs_arms3.rs`). **NOT shipped**: the two ruled arms classify but refuse at the open-chain door, which admits plane–plane terminations only (#987); a CONCAVE curved rim's band adds material, which the composition surgery does not build (what the two-sphere snowman waist meets now that its arm exists); and the genuine mid-curve run-out pair below | Consumer, MET: the calochortus bud's sphere–cone seam — its MOUTH RIM alone fillets end to end, tier-3 valid, with closed-form trim circles (`crates/sweep/tests/verbs_arms2_bud.rs`); so do the same bud's cone×plane lip and cylinder×plane bore. `lily::wall_probes` wall 6 asks for EVERY lantern edge and still refuses, at a co-surface seam meridian's tangency (margin exactly zero) — it cannot distinguish this door from that one. **The coaxial arms may need no consumer at all** (Evan, 2026-08-16, on the Klein bottle's neck→flare blend): a blend between two coaxial surfaces of revolution is itself one, so it is authorable as an ARC IN THE MERIDIAN before revolving — exact, free, and *better* than a post-hoc roll. That escape closes as soon as the supports are NOT coaxial, which is the canal case below. **`SpineUnsupported` now discriminates**: a pair outside the arm roster names the roster; a pair inside it whose supports miss the shared axis refuses on the `fillet3_support_coaxiality` margin, and THAT is the canal case (DESIGN frontier (f)) — no approximating surface is involved in anything this row ships |
| **fillet run-out (terminating a blend before the chain ends)** | stopping a band part-way instead of carrying it to a corner or all the way round | **The taxonomy is now honest at both ends, and NEITHER end is machinery** (VERBS-ARMS-3, `docs/ARMS3-DESIGN.md`, #319's second finding). (a) **The valence-4 "seam corner" was never a run-out question**: at the point where a chart seam crosses a latitude rim, the surface is SMOOTH — the seam is where a chart was cut, the two extra incident edges are co-surface seam meridians whose dihedral is zero by construction, and there is no wedge, no ball-rest configuration distinct from the neighbouring rim points. It refuses `FilletCornerUnsupported { corner: SeamVertex, policy: None }` — a zero-constructor tag naming NO run-out policy, because none would help — with a recourse that names the request instead: ask for the rim WHOLE. (b) **The genuine mid-curve run-out is real, PARKED, consumer-gated**, in two named shapes: the **ball-cap stop** (the ball at rest at the final station caps the band with a sphere patch — well-defined at any smooth interior point, the `corner_ball` machinery's smooth sibling; new surgery, no new surface kinds) and the **feather-out** (the radius tapers to zero approaching the station — variable-radius-shaped, frontier (f) adjacent, strictly more machinery). Ball-cap is the presumptive first pick when a consumer arrives | No consumer has ever wanted either: every consumer the whole ARMS program met wanted the full rim (the bud, the snowman, the lentil, every solid of revolution). **The seam tag's recourse is now SERVED**: the closed-rim annulus band takes a MULTI-LINK closed chain whose links are one rim's arcs across chart seams — the walk carries through the seam vertices instead of terminating at them, and each side's support may be several FACES of one SURFACE (the half-band walls a pole-touching revolve mints). So the whole-rim request the tag names is carved, on all three of a lantern's rims and on an unbored hemisphere's equator. Pinned live at `crates/sweep/tests/verbs_arms3.rs` (the witness reproduced, the refusal and the whole-rim carve) and `crates/sweep/tests/blend_seam_split_rim.rs` (the band's own closed forms, and the differential against the one-edge twin) |
| **variable-radius fillet** | radius varies along the spine | the canal-surface blend (banked, consumer-gated — DESIGN frontier (f)): a variable-radius spine is generically neither line nor circle | Band-3; re-opens the canal unit with a consumer. The Klein bottle supplies a CONSTANT-radius one for frontier (f) as well: blending the top loop's torus against the body's cone, taken literally, has supports that share no axis, so the rolling ball's spine is neither line nor circle. The bottle sidesteps it with a tangent neck cylinder (2026-08-16), which is the modeller's answer, not the kernel's |
| **draft** | tapered replacement of walls for molding | a certified re-geom pass (attach layer + a pass-owned vertex step — NOT the M6-1 graft shape, which adds/kills entities; DRAFT-DESIGN DR2) + the pull-direction selection predicate (DR3) | design record: `docs/DRAFT-DESIGN.md` — plane-wall v1; the cylinder arm mints cones and is its own later unit, a plane×cone fitted-SSI lane (DR1 as corrected: R1's conic-inventory refusal bars only exact special cases and stands untouched) |
| **hole features** | counterbore / countersink / tapped | sugar over booleans + patterns per D8 (structural parameters); the recipe-layer node vocabulary | substrate shipped (`PlacedUnion` × `Subtract` spells a counterbore today); remaining: the sugar vocabulary (MIRROR-DESIGN P4/P6), face-tied placements (GROUP-BOOLEAN's staged item), and overlapping cutters behind G8's multi-solid-operand residual |
| **patterns (linear/circular) + mirror** | D8 structural-parameter instancing | the patterns half is SHIPPED (`Node::Pattern` linear/circular/explicit + `PlacedUnion`, `Instance(i)` naming per the ratified obligation, part-level); MIRROR is the open half — reflection instancing, lily wall 5, with the equivariance premise a named prerequisite (A6) | design record: `docs/MIRROR-DESIGN.md` (P1 chart handedness, P2 its own door beside `transform_rigid`, P3 audit boundary); assembly instancing (import side) is the same family's foreign face — fixtures banked in STEP-BANK |
| **helix / thread** | helical sweep | the ≥0.5-turn frontier is EXECUTED and filed (#222: nurbs_span_meter ParamSpan under near-antipode frame roll); joined-path sweeps banked | #222 is the named blocker, not a guess |
| **taper / variable-section sweep** | section scales/varies along path | lily wall 9 — an ABSENCE, not a refusal: the shapes are authorable through `loft_body` by hand-placing every station, and what is missing is the one-op taper along a path-following frame; canal-adjacent (variable-radius tube = circular case) | Band-3 breadth |
| **lofted membrane / sheet bodies** | zero-thickness faces (petals) | D1's manifold-first boundary: sheet/wire bodies are the named non-manifold extension trigger ("add a non-manifold representation later only if sheet/wire bodies demand it") | a real D1 design conversation, not a feature; the lily's petals are authored as solids and the scene carries no probe for this (an absence has no refusal to pin) |
| **spheroid / ellipsoid primitive** | non-spherical quadric | D3 closed-enum extension (new analytic kind: every dispatch site enumerated by the compiler) or NURBS route | lily wall 4 |
| **rib / text** | Band-3 conveniences | text = profile vocabulary + patterns; rib = draft-adjacent | far tail |
| **datum planes / axes** | reference geometry | SHIPPED — `Node::Datum{Plane,Axis,Point}` evaluates to `DatumValue` and serves as revolve axis (`wire.rs:549`), circular-pattern axis (`wire.rs:1274-1282`) and split tool plane (`wire.rs:885`) — **not** as a sketch plane, which the opaque `Profile` payload still carries; it is exported from `pncad::prelude`, frames `GeomPred::DatumDistance`, and carries one generic label in the viewer tree | — |
| **curved boolean breadth** | cyl×sphere, sphere×sphere, cone/torus operands | the banked germ-chord lanes (DESIGN frontier (d)); the SSI lift removed the storage half | each lane is its own unit; the teapot's spout∪body would ride revolve-surface × NURBS classes — far. **The operand gate is PAIR-scoped** (VERBS-GATE): a kind with no wired arm disqualifies an operation only where its BOX may meet a face of the other operand, so a cone or torus face that clears the other body no longer gates anything, and the refusal names the germ PAIR and both faces (`CurvedPairUnsupported`) with the box conservatism stated — overlap is a *may*, not a *does*. **No germ class was added**, so what moved is honesty and reach, not breadth. The widening has one named cost: the gate now admits pairs `point_in_solid` has no ray-crossing arm for (#1011), and the remaining schedule is the banked germ-chord lanes plus #1057's two C5 section arms (plane×torus, cone×cylinder). Measured on the register's own consumers: the Klein bottle's pieces still cannot be joined, but the refusal is now the flare's CONE against a plane of the loop — NOT the coincident annular mate the model cares about — and its self-intersection still cannot be trimmed — a DIFFERENT pair, `(Torus, Torus)` under `Subtract` (`klein::wall_probes` walls 3 and 4). The lily's tepal seam, asked as sphere×sphere, is ADMITTED by the pair-scoped gate. It used to refuse one door later at the maximal-faces precondition — the lantern's two axis-touching planar caps — and no longer does: the scene REPAIRS those caps first (#1031's pole half, below), so wall 7 now reaches the curved PIERCE arm and still no germ pair is exercised. **The lily's FLOWER WELD is no longer an SSI ask** (VERBS-LILYWELD PR-1, #1059): the flower/arch junction is authored circle-coincident — the lantern's neck cone is cut at the arch tube's own radius, so its rim IS that tube's terminal meridian circle, and `lily::weld_circle` computes both off the two STORED carriers and asserts them equal on every run (the lantern axis is the stem's own tangent, so the meridian circle at that station is the one circle a coaxial cone can be cut to). The union still refuses `CurvedPairUnsupported { op: None, (Cone, Torus) }` at the operand gate, because that gate reads KINDS and never loci — the coincidence is invisible to it. **That reading was itself wrong, and measuring it is what refuted it** (VERBS-LILYWELD PR-2, 2026-08-27): widen the gate in a scratch tree and the next refusal is `NonMaximalFaces` on the lantern itself — its two AXIS-TOUCHING planar caps, the F7 defect `merge_coplanar_faces` will not repair, at the time of that measurement; `merge_coplanar_faces` repairs it now (below) — and one door after that the curved PIERCE arm (wall 12's door). The `carrier_eq` rung is never reached and has no consumer: PR-1's own abutment made the weld's declared contact plane×plane, and a cone×torus Rest declaration would be `Contradicted`, correctly. So wall 2's binding blocker is the chain gate → F7 → the curved PIERCE arm. #1031's POLE half has landed (#1131), which was necessary and not sufficient; the gate-admission question stays DEFERRED pending a re-measurement of the widened-gate sequence on the REPAIRED lantern. Pinned live by `lily::review_probes::the_declared_weld_refuses_exactly_as_the_undeclared_one_does` and `the_lanterns_two_pole_split_caps`. This is another instance of *stated blocker ≠ binding constraint* — the first caught in a SPEC's own text rather than a probe comment (`memories/refusal-text-is-not-cause.md` keeps the roster); the measure-before-building rule is what caught it **#1031's POLE HALF has landed** — as a REPAIR, not a gate narrowing. Two structural gate exemptions were tried and both admitted shapes they were claimed to exclude (subdivided chords and inset rings; then `merge_skip`'s L-corner flush caps), each falsified by a fixture already in the repo, and both were withdrawn. What ships instead is in `merge_coplanar_faces`: a full revolve's axis-touching cap is two half-discs joined at the two halves of the disc's DIAMETER, so the pole is a vertex interior to ONE straight carrier; `merge_faces::redundant_subdivision_vertex` decides exactly that (parallel + opposed departures on Line carriers) and the op then removes the seam with `kef` then `kev`, leaving one maximal face and changing no locus. Measured on a plain revolved cone: faces 4→3, vertices 4→3, planar same-key pairs 2→0, tier 2 and tier 3 green; on the lily's lantern (two caps): faces 10→8, vertices 10→8, edges 18→14. **Probe 13 RETIRED** (the merge door it pinned shut is open) and **wall 7 MOVED** to `CurvedPierceUnsupported` on the repaired body. The licence is collinearity, not poleness, so the repair is general — *remove a redundant subdivision vertex on a shared collinear seam* — and #1031 stays open for its OTHER defect: an ordinary coplanar pair at a full-valence edge, measured on the teapot cup's meridian plane, which still refuses — whether that pair is even the same shape is not settled, because the cup seam's straightness was never measured (unverified — measurement pending). |
| **point-section loft ("generalized cone")** | loft whose end section degenerates to a point (apex) | three tiers (Evan's mark-down, 2026-08-10, PR #300-era chat): (1) circle profile + straight axis = the ANALYTIC CONE — mint exact CONICAL_SURFACE (the tube_along_arc exact-intent pattern; #256 always-promote applies), mostly sugar + recognition; (2) polygon profile = pyramid, plain planar walls, possibly already expressible; (3) general curved profile = a NURBS wall with a collapsed boundary row — needs apex-degenerate certification (span meter is honestly zero at the apex, normals undefined, quadrature hulls loose) with the apex as a vertex-loop (the sphere-pole precedent) | no design conversation yet by ruling ("mark it down for the future"); tier 1 is a cheap early pick when a consumer appears |
| **declared conformal / REST joins (curved)** | ball-and-socket, interference fits | **SHIPPED** — C7 (CONTACT-DESIGN, ratified #178) as M9's join lane | listed for completeness; the register's one M9 row. One limitation ships with it, stated: a purely cylindrical declared `Rest` with no planar `Rest` beside it does not reach the rest lane (#1032) |

Consumers waiting on rows above: helical parts (#222), the
petal'd lily (sheet bodies), and **the Klein
bottle** (the shell row's UNMEASURED residue — `topo::shell` shipped
at #1048 and no Klein wall has been re-asked against it; curved
booleans both ways; the canal blend —
`demos/tour/src/klein.rs`, whose module docs carry the findings list
and whose `wall_probes` runs every refusal live). The bottle is
deliberately NOT a consumer of the coaxial curved-fillet arms: the
meridian arc is the better answer there, per the row's own note.

**The Utah teapot has been MET** (`demos/tour/src/teapot.rs`,
2026-08-27) and is no longer waiting on shell; it is now a consumer of
three other rows and the source of two "scope limits" entries below.
What it still waits on: **curved boolean breadth** — handle ∪ pot is
torus × cylinder and spout ∪ pot is cone × plane, both
`CurvedPairUnsupported`, so the teapot is FOUR solids (walls 2 and 3);
**taper / variable-section sweep and the canal family** — a spout the
shape of a spout is a swept curved section along a bent spine, which
`sweep_body` will not round (the U-turn class klein wall 5 pins as
`ReversedStacking`; the teapot's own spout is not attempted) and no
variable-section door exists for, so the scene's spout is a straight
cone frustum tilted into place, a spout the way a LATHE would make
one; and **geometric edge selection**, which is document-layer only,
so the lid's knob rim is scanned for by hand exactly as the bud's and
the bottle's are.

## Scope limits and defects met by consumers — NOT missing verbs

Added 2026-08-16 with the Klein bottle, which met five of these in one
model. They are a different KIND of entry from the table above: the
verb exists and is reachable, but it refuses (or mis-answers) inside
what a modeller would call its own territory. They live here because
this register is where "what the kernel will not let me do" stops
being a scattered note — the section schedules nothing, exactly like
the table.

- **`fillet_edges` on a full-revolve rim — FIXED for ANNULAR
  profiles.** The mis-metering half of this entry was FIXED by
  VERBS-RIM (#554): the battery's lever arm is the maximum pairwise
  chord over its own per-link sample schedule
  (`crates/sweep/src/fillet/battery.rs`, `extent_of`), so a closed
  latitude rim meters ~its diameter, its dihedral decides honestly,
  and the false `TangentialEdge` ("the supports share a tangent
  plane", on supports meeting at 30°) is gone — a co-surface seam's
  refusal is now distinguishable from a closed transverse rim's.
  What REMAINED at the time, and what VERBS-ARMS-1 then closed: a
  closed rim is a one-link closed chain, and the surgery's rim door
  refused it typed because the one-edge torus band was not built.
  **It is built now** — the ANNULUS band, `rim_phase_annulus` — so a
  full solid of revolution's plane–sphere latitude rim fillets end to
  end, tier-3 valid, with mass properties against the closed form
  (`verbs_rim_r1_probes::a_passing_closed_rim_reaches_the_surgery_and_builds_its_annulus_band`,
  `crates/sweep/tests/verbs_arms1_annulus.rs`).

  **The bound on that unlock, stated:** it covers profiles that are
  ANNULAR (off the axis). A profile that touches the axis mints
  HALF-walls — two seam azimuths — so its equator is not a closed edge
  at all but two open arcs over two half-disc supports, and it still
  refuses typed
  (`verbs_arms1_r1_probes::the_unbored_hemisphere_equator_refuses_typed`).
  Two closed rims sharing one wall in ONE call also refuse typed, at an
  upfront gate naming the sequential-call recourse, which composes
  exactly (#935 is the one-call widening). Curved support pairs
  whose supports miss the shared axis still refuse `SpineUnsupported`
  at `fillet3_support_coaxiality` (`battery.rs:781`); cone×cylinder
  itself is an implemented arm now, and `klein::wall_probes` walls 1
  and 2 have moved to `RadiusHeadroom` — the ball is bigger than the
  neck wall's curvature allows, not a missing arm.
- **`mesh::planar`'s banked sub-floor case is no longer synthetic.**
  That module's docs bank exactly one uncovered class — a planar
  face whose boundary points carry off-plane noise, where the chart
  frame's far point has an *engineered* exact-zero v-coordinate whose
  float residue lands below spade's `MIN_ALLOWED_VALUE` (2⁻¹⁴²) — and
  say of it "synthetic today (no corpus body hits it; wild translator
  noise observed so far is axis noise)". The Klein bottle's bulb hits
  it with an ordinary annular cap at plain coordinates: `insert`
  refuses at `(0.4978884624952483, 3.94e-47)` and the face returns
  `Triangulation`, at every δ. Whether it fires is a roundoff lottery
  over parameters that do not touch the cap — sweeping flare angle
  against rim radius, it fires at (30°, 0.85 m) and (34°, 1.00 m) and
  not at their neighbours. `klein::wall_probes` wall 7 pins it by
  building the shipped bottle with a 5 cm wider rim. Same shape as
  #284 — a valid body that refuses tessellation — and filed as
  **#555**, since #284 itself is closed and its fix deliberately did
  not claim this case.
- **`sweep_body` cannot round a U-turn.** The loft's canonical
  stacking trilean compares the LAST placement's mean displacement
  against the FIRST section's plane normal, so any path that ends
  behind where it started refuses `ReversedStacking` wholesale, no
  matter how well every consecutive pair stacks. The Klein bottle's
  top loop is one path and would be one body; this gate is why it is
  two. (`crates/sweep/src/loft.rs`, the `loft_stacking` decide;
  wall 5.)
- **`tube_along_arc` is no longer solid-only.** FIXED by
  VERBS-TUBEWALL: the torus door has a hollow sibling,
  `tube_along_arc_hollow`, taking the outer `minor_radius` plus a
  `wall` thickness. The annular section is built internally the way
  the solid door builds its circle — a second directly constructed
  traversal at the inner radius, handed to the same revolve machinery
  as a hole loop, no second construction and no fork — so a thin tube
  no longer has to be re-said as a `revolve` of an annulus and the
  door's whole point survives the hollow form: the outer wall's radii
  and the frame are still the caller's numbers bit for bit, and the
  inner wall's minor radius is `minor_radius - wall`, ONE IEEE
  subtraction of the caller's own two numbers (which a caller recovers
  by repeating it) rather than a profile→bulge→radius reconstruction.
  A window is an ordinary open elbow of annular section; a full
  period is a torus SHELL whose cavity is born through the shared
  void-insertion door by the revolve's own holed path — the
  VERBS-RING precedent, with the annulus's concentric-circle
  containment carried as the evidence. The three `wall` refusals are
  decided FIRST, before anything is minted, through the door's own
  funnel (`tube_wall`, `tube_wall_bore`, `tube_wall_gap` — plain
  linear margins in meters, unlike this door's levered angular ones),
  which is also where that containment evidence comes from. The third
  is the one worth naming: it meters the REALIZED gap between the two
  radii the walls will store, because a thickness far above ε can
  still fall under a large outer radius's own ulp and round the inner
  radius back onto the outer — a class the first two decides cannot
  see. Everything the solid door refuses the hollow door refuses
  identically, through the same shared decides.
  `crates/sweep/tests/verbs_tubewall.rs`.

  **The bound on that, stated:** a hollow tube's cross-section is an
  ANNULUS about the spine and nothing else — one wall thickness,
  concentric, constant along the arc. An eccentric bore, a varying
  wall, or a non-circular section is still a profile-side job, and
  the `revolve`/`sweep_body` doors remain where those are said. And
  the full-period form is a multi-shell curved solid, so it JOINS the
  STEP row below rather than escaping it — now a receipt, not an
  expectation: the `hollowtorus` tour scene (`demos/tour/src/tubewall.rs`)
  declares the writer's frontier at the body and probes it on every
  pass, and the export refuses `CurvedShellClassification` (the scene
  pins the variant, not the `kind` payload) exactly as the hollow
  ring's does. Self-retiring in klein's wall-6 shape — a different
  refusal, or a success, fails the tour. Issue #986 — the scene is
  built and pinning; the issue is still open and wants closing.
- **`shell`'s PER-CHART arm survives exactly ONE junction shape, and it
  is not about curvature — and since #1081's PR-2a that arm is not the
  only one.** An all-planar body is offset SIMULTANEOUSLY and its
  oblique corners hollow; what is described below is the per-chart door,
  which is what every body with a CURVED face still takes. Added
  2026-08-27 by the verb's own designated demo
  (`demos/tour/src/teapot.rs`, wall 1; the table and its sweep are
  `demos/tour/tests/verbs_teapot.rs`). The sealed arm replaces one
  CHART at a time and re-anchors every edge that ends at a moved
  vertex on a carrier that has NOT moved yet, so a junction survives
  exactly when the neighbouring surface is invariant under the moved
  face's own offset motion. On the analytic vocabulary that is one
  pair: **a plane normal to a cylinder's axis, in both directions** —
  the plane's offset is a translation along the cylinder's ruling, the
  cylinder's is a radial shrink the plane is invariant under. A box is
  in the class because every face is normal to every neighbour.
  Everything else refuses `ShellError::Face { ReplaceFaceError::
  ReanchorOffCarrier { gap } }`, where `gap` is the distance in meters
  the neighbour's edge was pushed off its own carrier: a cone frustum
  between two caps, a sphere zone between two caps, **and a right
  prism on a TRIANGLE** — which is what rules curvature out as the
  variable.

<<<<<<< HEAD
  **REPAIRED IN FULL (#1081 PR-2a and PR-2b).** The measurement that
  forced the split: the refusing edge's two faces are BOTH outside the
  moving group, and re-anchoring alone would have shipped a WRONG BODY
  — `shell` visited each of a corner's charts in turn and transported
  it rigidly each time, accumulating `Σ dᵢ·nᵢ` where an offset body
  needs the point satisfying every `nᵢ·x = nᵢ·oᵢ + dᵢ` at once. On the
  hexagon at `t = 0.02` that lands 11.5 mm from the true corner and
  leaves 30 mm of wall where 20 mm was asked for, and no tier catches
  it. `ReanchorOffCarrier` was the gate PREVENTING that body, which is
  why it was not simply relaxed. The offsets are now SIMULTANEOUS, in
  two doors:

  - **`topo::offset_planes_together` (PR-2a)** solves an all-planar
    body's corner ONCE against every moved plane meeting it, by Cramer
    on the first well-conditioned triple, and re-derives each edge as
    the intersection of its two moved planes. The hexagon, the bevelled
    box, the kite and the triangular prism HOLLOW, the hexagon pinned to
    its closed form. Its scope is named by `TogetherNonPlanar`,
    `TogetherPartialSet` and `TogetherCorner`.
  - **`topo::offset_charts_together` (PR-2b)** takes a body of
    REVOLUTION — every surface a plane, cylinder, cone or sphere about
    one axis — and solves in the MERIDIAN half-plane, where a plane
    normal to the axis is a line, a cylinder is a line, a cone is a
    line and a sphere is a circle. A corner is one line/line or
    line/circle meeting, closed form, with every further surface
    verified against it; the azimuth is CARRIED from the seam (a
    conventional datum, D2) or solved as circle-meets-plane where one
    plane contains the axis. The sphere-zone vase, the cone frustum and
    the partial-revolve wedge HOLLOW, each pinned to its own closed
    form, and so does **the teapot's belly**, which is a sphere zone:
    **the pot is no longer squared**. The corner census that shaped
    this door is the measurement worth keeping — a full revolve's rim
    vertex is incident to exactly TWO distinct surfaces, not three, so
    the planar door's 3×3 frame does not describe it at all. Its scope
    is named by `TogetherAxialUnsupported`, `TogetherNotAxial`,
    `TogetherAxialCorner` and `TogetherAxialEdge`.

  `shell` picks the branch structurally, and everything outside both
  keeps the per-chart posture and the refusal it had. **The honest
  boundary is a TORUS**: it is outside the axial kinds, so a pot whose
  belly bulges about a centre off the axis never reaches the door and
  keeps the C5 table's own `NeighborPairUnroutable` naming the pair —
  which is what the teapot's wall 1 pins now. Nothing in either PR
  widened `intersect::route`. A TANGENT junction refuses too, and now
  at the corner's own transversality meter rather than at a carrier
  lane: a wall meeting a sphere with no angle between them has no
  transversal corner to solve. That is a differential the SAME surface
  pair passes when it is transversal — the pot's own foot-to-belly
  junction is `cylinder ∩ sphere` and hollows.

  Two CONSUMER obligations were being left undischarged, and both are
  discharged here. **The cone's mirror nappe**: `ConeOffset`'s ratified
  header (ordinal 79) says `n₊` does NOT flip across the apex — that is
  the contract, because following the per-point chart normal would
  split the double cone instead of shifting a parameter — and states
  the consequence it puts on consumers, that a mirror-nappe face's
  material moves `−d` along its own chart normal. A `ChartMove`'s
  distance is along the FACE's outward direction, so a face below its
  apex needs the caller's number turned over before the mint sees it;
  unturned, the frustum's cavity came back LARGER than its operand
  (0.001058 against 0.000895). The axial door turns it (`nappe_signed`,
  from the face's own corners, decided). **The rim LIFT** in
  `shell_open` transported a rim rather than solving it — invisible on
  a cylinder, 6.2 mm wrong on a sphere — and now takes the same
  simultaneous door with every other chart at distance zero.

  **The sibling consumer has NOT been swept and is filed rather than
  fixed** (#1181): `replace_face::mint_offset` still hands the caller's
  raw `d` to a cone. No wrong body ships from it today — both review
  arms proved the caps refuse first on every reachable fixture — so it
  is a latent hazard behind a gate, not a live defect, and the issue
  carries both arms' evidence.

  The #1048 corpus is byte-identical across both changes, measured with
  the dump harness rather than asserted — including the two CURVED
  fixtures that moved branches.

  The third door (`CarrierLaneUnsupported`) is out of both PRs' scope:
  its fix is the mapped-description transport family. That row was
  first recorded as "not attributable to tangency alone", and the
  ordinal-100 review SEPARATED the two variables — a dome whose centre
  is lifted clear of the wall's top is definitely NOT tangent and
  reached the identical site. That door is about the neighbour's offset
  not being a rigid translation; tangency is not its variable.

=======
  **THE PLANAR HALF IS REPAIRED (#1081 PR-2a); the curved half
  stands.** The measurement that forced the split: the refusing edge's
  two faces are BOTH outside the moving group, and re-anchoring alone
  would have shipped a WRONG BODY — `shell` visited each of a corner's
  charts in turn and transported it rigidly each time, accumulating
  `Σ dᵢ·nᵢ` where an offset body needs the point satisfying every
  `nᵢ·x = nᵢ·oᵢ + dᵢ` at once. On the hexagon at `t = 0.02` that lands
  11.5 mm from the true corner and leaves 30 mm of wall where 20 mm was
  asked for, and no tier catches it. `ReanchorOffCarrier` was the gate
  PREVENTING that body, which is why it was not simply relaxed.
  `topo::offset_planes_together` now solves each corner ONCE against
  every moved plane meeting it and re-derives each edge as the
  intersection of its two moved planes; `shell` takes that branch when
  every face of the body is a plane. So the hexagon, the bevelled box,
  the kite and the triangular prism HOLLOW, with the hexagon pinned to
  its closed form (`crates/sweep/tests/verbs_shell.rs`, the
  oblique-prism row; `demos/tour/tests/verbs_teapot_r1_probes.rs`'s p2
  for the bevel and the kite) — and the cone frustum, the sphere zone
  and the quarter-revolve wedge still refuse at that door, while the
  tangent bullet and the lifted dome refuse one door further on
  (`CarrierLaneUnsupported`), because a curved face brings no plane
  equation to its corners. Those are the C5-table work of PR-2b, and
  **the teapot's belly is one of them** (a sphere zone), so the pot
  stays squared until 2b lands. Six typed refusals name the new door's
  own scope: `TogetherNonPlanar`, `TogetherPartialSet` and
  `TogetherCorner` for the geometry, `TogetherChartMixed`,
  `TogetherFaceRepeated` and `TogetherEdgeDisagreement` for the call's
  own structure.
  The #1048 corpus was measured byte-identical across the change at the
  unit (an out-of-tree dump; nothing in-tree pins it).
  `crates/topo/src/offset_together.rs` derives the box's bit-identity
  from its mutually perpendicular normals. A TANGENT junction refuses too, at a further door
  (`CarrierLaneUnsupported`). That row was first recorded as "not
  attributable to tangency alone", because the lattice's only route to
  a tangent junction is `.tangent().tangent_arc_to(..)`, whose
  description is a mapped arc — but the ordinal-100 review SEPARATED
  the two variables and the caveat no longer stands: a dome whose
  centre is lifted clear of the wall's top is definitely NOT tangent
  and refuses at the identical site with the identical `what`. That
  door is about the neighbour's offset not being a rigid translation;
  tangency is not its variable. It is out of PR-2a's and PR-2b's scope
  alike — its fix is the mapped-description transport family.
>>>>>>> origin/main
  **Why nothing caught this before:** `shell`'s acceptance corpus
  (`crates/sweep/tests/verbs_shell.rs`) is a box, a cylinder between
  two caps and a tube between two caps — every fixture inside the
  surviving class, and the class was never named. The teapot paid for
  it in shape for two waves, and no longer does.
- **`shell_open`'s rim on a solid of revolution — FIXED.** Added
  2026-08-27 with the junction row above (the teapot's second finding —
  formerly wall 2, now retired into the scene's inline assertions at
  `teapot.rs:920-960` — plus the same test file) and repaired the same
  week. **What was wrong**: the verb
  RETURNED a body that passed tiers 1, 2 and 3 while each designated
  face carried its own cavity counterpart's boundary re-labelled as an
  interior ring — genus 1 where `topo::shell`'s docs say a cup is genus
  0, and `mesh::tessellate` refusing `Triangulation` at every δ. Not
  the `mesh::planar` sub-floor lottery of #555: swept over five wall
  thicknesses, mouth radii at two scales (41.25 mm and 46.875 mm — 14%
  apart, one cluster — and 1 m) and three chord budgets, every case
  gave the same two wrong numbers, the simplest fixture (a cylindrical
  drum) included. A BOX opened at its top was always correct.

  **The class, as the review re-scoped it**: *a designated face whose
  cavity counterpart's boundary cannot become an interior-disjoint RING
  of it*. "An extrusion's cap is ONE face, a full revolve's is TWO
  half-discs sharing a chart" was FALSE as the mechanism — a revolved
  TUBE's mouth chart is ONE face and was wrong too (rings 1, genus 2),
  and a PARTIAL revolve's cap is one face and does touch the axis.

  **What the fix turned out to be, and it is not a ring placement.**
  Both failure shapes are the REVOLVE's seam arriving inside the
  designated chart: an axis-touching cap is two half-discs meeting at
  an axis apex, so the counterpart's boundary is a D-loop reaching that
  same apex and running back along the outer loop's own seam legs; an
  annular cap is ONE face SLIT along a radial edge its loop walks
  twice, so the counterpart's boundary runs along that slit. Neither is
  a fact about the mouth. `shell_open` now reduces both charts to one
  face carrying disjoint cycles before the glue — `kef`, `kev`, `kemr`,
  no new machinery — after which the counterpart's boundary IS strictly
  inside. The axis-touching mouth comes back as ONE annular rim (one
  ring, genus 0, meshing, closed-form volume); the ANNULAR mouth comes
  back as TWO DISJOINT ANNULI, the face SPLIT the first reading said
  `kfmrh` could not express — built with `mfkrh` promoting the
  counterpart's hole to its own rim face before the glue and
  `ring_move` handing it the designated face's matching hole after.
  What is still refused, typed and naming the shape, is
  `ShellError::OpenFaceRimNotExpressible`: a chart whose faces are not
  one region, a counterpart boundary that still meets the designated
  face's, or more than one hole to pair (the pairing this door derives
  is single-hole; a two-holed designation refuses rather than guesses).

  **The invariant is now stated at rest**, which is what turns the
  class loud wherever it is minted: tier 3's check 9
  (`ValidationError::RingMeetsOuter`) refuses a ring that meets its
  face's own outer loop, in three arms — vertex-on-vertex,
  vertex-on-edge-interior, and edge-along-edge. Compared by POSITION,
  not by key: the shapes it catches are minted by surgeries that copy a
  boundary. Key-shared pairs are decided too and carry no exemption —
  tier 1 has no pass that refuses a face's outer loop and its own ring
  sharing a vertex or edge key, so an exemption there would leave the
  umbrella pinch unnetted. Every margin **escalates typed**
  (`ValidationError::RingContactEscalated`) rather than reading as
  "disjoint"; the shell verb's own precondition escalates the same way
  and never proceeds to build.

  **What check 9 does NOT match, enumerated** (an unstated blind spot
  is an unverified claim): one-point TANGENCY between two edges at a
  point that is a vertex of neither (circle-circle internal or
  external, line-circle) and a transversal CROSSING at a non-vertex
  point — three-sample locus agreement cannot see a single shared
  point, and the closed forms that could need an arc-containment test
  this predicate has not got; and `Ellipse`/NURBS carriers in the two
  locus arms, whose endpoints the vertex arm still covers.

  **Why nothing caught it, and the transferable lesson.** Not "the rim
  lift never had a consumer": `offd2_r1_probes::probe_opened_vessel_cup`
  already opened a revolved vessel through this very path and blessed
  it, checking only the things that were right — tier 3, the shell
  count, the volume — and never the rings, the genus or the mesh. A
  probe that checks only what is right is not evidence about what is
  wrong. That row now checks all three. Fixtures:
  `crates/sweep/tests/verbs_shell.rs` (both rim shapes and the
  validator's planted red, built through the public doors the verb used
  to compose), `demos/tour/tests/verbs_teapot.rs` (the sweep, flipped)
  and the two review probe suites beside it. Consequence for the demo:
  **the teapot ships OPENED**, and — being one shell — it also leaves
  as STEP, which the sealed two-shell body could not.
- **A hollow ring cannot leave as STEP.** The one-call hollow ring
  itself SHIPPED (VERBS-RING retired `FullRevolveHoles`: a full
  revolve of a holed profile builds the multi-shell solid through the
  shared void-insertion door — the register's old defect row here),
  but the STEP writer's outward/void shell classifier has closed
  forms for planar faces only, so a multi-shell CURVED solid refuses
  `CurvedShellClassification` — the known standing gate of
  OFFSET-DESIGN O6's demo-gates list, which every hollow curved part
  (this ring, the full-period `tube_along_arc_hollow` shell
  since VERBS-TUBEWALL, and the shelled teapot, which reached it as
  predicted) hits at export. (Wall 6, re-baselined: it now pins THIS
  refusal on the ring it builds.) The gate now carries THREE probes,
  which retire together: klein's wall 6, the `ring` scene's
  `step_at_frontier` on the profile door's hollow ring, and the
  `hollowtorus` scene's on the parameter door's. The `teapot` scene
  was a fourth until #1082's repair: its vessel ships OPENED now, and
  a cup is ONE shell, which this gate never reaches. Three separate
  bodies through three doors, and a widened classifier releases all of
  them;
  what the teapot's adds is that the gate is now on record for a body
  built by asking for a hollow PART rather than for a shape chosen to
  reach it.
- **The PATHS lattice has no tangent straight leg to an anchor.**
  After a declared-tangent joint off an arc, the only straight
  continuation is `.line(len)` — `.to(anchor)` belongs to a fillet's
  arrival side — so a G1 line following an arc is placed by LENGTH and
  its far end is derived. The Klein bottle's inner tube wall is placed
  that way, and the revolve then reconstructs its cylinder radius from
  the swept endpoints: two walls that are geometrically the same
  cylinder come out with radii differing by tens of ulps. Same drift
  class the `tube_along_arc` door was built to retire, met from the
  profile side.
- **Geometric edge selection is document-layer only.** `select_where`
  + `GeomPred::AdjacentKinds` is the ratified way to say "the
  cone×cylinder corners", and it takes an `Evaluation` — so a body
  built by calling `revolve` directly has no selector at all and must
  scan `body.edges()` through two back-pointers by hand
  (`klein::corner_edges`). A gap in reach rather than a refusal.
