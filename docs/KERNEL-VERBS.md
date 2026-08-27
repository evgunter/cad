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
VERBS-ARMS-1 — the N-link ladder rim and the one-edge ANNULUS rim of a
full solid of revolution, the latter for ANNULAR profiles only, see
the row below) + in-place composition surgery,
**symmetric-setback edge chamfers** (`chamfer_edges`, **plane–plane
supports only, convex chains, open chains between fully-requested
trivalent corners** — see the row below),
merge_coplanar_faces, rigid transform, tessellation/STL/STEP
export, STEP import (adoption incl. recognition + tier gate).

| verb | what | prerequisites / blockers (as ratified) | notes |
|---|---|---|---|
| **shell / hollow** | offset the boundary inward, remove faces, thicken | Q8: offset surfaces — analytic kinds are CLOSED under offset (a D3 payoff); a NURBS offset is NOT a NURBS → needs the approximating-surface machinery (intensional spec `Offset(S,d)` + fit + certified residual ≤ ε, "exactly mirroring fitted intersection curves"). Also wants open-shell/face-removal vocabulary (D1's manifold-first boundary) | **The Utah teapot is this verb's designated demo** (Evan, 2026-08-09) — a vessel is a shelled revolve; the demo queues behind the verb. **Second consumer, and the first to PAY for the absence in full: the Klein bottle** (`demos/tour/src/klein.rs`, 2026-08-16). A thin 3-manifold whose midsurface is the immersed Klein bottle is nothing but shell, so every wall is authored as its own two offsets by hand — each radius spelled twice as `r ± t/2`, each blend radius twice as `R ∓ t/2` with the sign depending on which side the centre of curvature is on, and the offsets swapping sides wherever the surface turns back on itself. It builds and it is exact; it is the whole row, paid once per wall |
| **offset (surface/solid)** | the standalone Q8 operation | same as shell's core; Q8 says "needed before shelling/offset work (M5+), stated now" | shell's substrate; may land as one unit |
| **chamfer** | the fillet's ruled-surface sibling | **Shipped for plane–plane supports** (VERBS-CHAMFER): `sweep::chamfer_edges` at equal setback, over the fillet's own battery, admission doors and composition surgery — a flat strip per edge and a flat patch per trivalent corner, every face an exact `Surface::Plane`. What is NOT shipped and what blocks it: **curved supports** (the strip over a curved support is VERBS-ARMS' machinery, refused `ChamferArmUnsupported`); **concave chains** (refused at the same two admission doors the fillet's concave case is, and the corner-configuration classifier reaches first); **asymmetric parameters** (distance–distance, distance–angle — a widening of the same door, nothing forecloses it); **closed chains**; **a recipe-layer `Node::chamfer`**, without which the verb is unreachable from a document and mints no names | The register's own "cheapest entry" call held: the verb is a front door, one blend arm, one corner patch and three parameterized decisions in the shared surgery |
| **constant-radius fillet on CURVED support pairs** | the arms of C8's analytic table that M5 PR 12 did not implement — sphere×cone, cone×plane, cone×cone, sphere×sphere, and the cylinder pairs | **The COAXIAL half is SHIPPED (VERBS-ARMS-2).** Eight arms from ONE derivation: when a support pair carries a symmetry the rolling ball inherits — a common axis of revolution, or a common ruling — the ball's centre is confined to a SHEET (the meridian half-plane through the rim; the cross-section normal to the ruling), where each support cuts a LINE or a CIRCLE and the centre is the crossing of the two OFFSET traces, on the branch that returns the rim as `r → 0`. Coaxial six → TORUS (sphere×cone, cone×plane(⊥), cone×cone, cylinder×cone, cylinder×sphere, cylinder×plane(⊥)); ruled two → CYLINDER (cylinder×cylinder(∥), cylinder×plane(∥)). **No constant-radius arm mints a cone** — that is the variable-radius family, and C8's prose now says so. The closed-rim surgery's gates were re-cut by SHAPE rather than by KIND so the annulus band carries any pair of revolution walls, and `geom-brep`'s tangent-certificate circle arm grew a CONE row so such a band can be described at rest. **Sphere×sphere SHIPPED (VERBS-ARMS-3)** as the ninth coaxial arm and the only one whose shared-axis hypothesis is FREE: two spheres on distinct centres always meet in a circle, and the line through the centres is that circle's own axis, so the `fillet3_support_coaxiality` margin is zero by construction rather than by luck. It is a pure reduction — the circle×circle sheet crossing ARMS-2 already derived, with both spheres' stored sense bits folded in. Consumer, MET: a lentil (the solid between two unit spheres, bored) whose convex equator fillets end to end through the annulus door, tier-3 valid, the band's spine at `√((R − r)² − c²)` (`crates/sweep/tests/verbs_arms3.rs`). **NOT shipped**: the two ruled arms classify but refuse at the open-chain door, which admits plane–plane terminations only (#987); a CONCAVE curved rim's band adds material, which the composition surgery does not build (what the two-sphere snowman waist meets now that its arm exists); and the genuine mid-curve run-out pair below | Consumer, MET: the calochortus bud's sphere–cone seam — its MOUTH RIM alone fillets end to end, tier-3 valid, with closed-form trim circles (`crates/sweep/tests/verbs_arms2_bud.rs`); so do the same bud's cone×plane lip and cylinder×plane bore. `lily::wall_probes` wall 6 asks for EVERY lantern edge and still refuses, at a co-surface seam meridian's tangency (margin exactly zero) — it cannot distinguish this door from that one. **The coaxial arms may need no consumer at all** (Evan, 2026-08-16, on the Klein bottle's neck→flare blend): a blend between two coaxial surfaces of revolution is itself one, so it is authorable as an ARC IN THE MERIDIAN before revolving — exact, free, and *better* than a post-hoc roll. That escape closes as soon as the supports are NOT coaxial, which is the canal case below. **`SpineUnsupported` now discriminates**: a pair outside the arm roster names the roster; a pair inside it whose supports miss the shared axis refuses on the `fillet3_support_coaxiality` margin, and THAT is the canal case (DESIGN frontier (f)) — no approximating surface is involved in anything this row ships |
| **fillet run-out (terminating a blend before the chain ends)** | stopping a band part-way instead of carrying it to a corner or all the way round | **The taxonomy is now honest at both ends, and NEITHER end is machinery** (VERBS-ARMS-3, `docs/ARMS3-DESIGN.md`, #319's second finding). (a) **The valence-4 "seam corner" was never a run-out question**: at the point where a chart seam crosses a latitude rim, the surface is SMOOTH — the seam is where a chart was cut, the two extra incident edges are co-surface seam meridians whose dihedral is zero by construction, and there is no wedge, no ball-rest configuration distinct from the neighbouring rim points. It refuses `FilletCornerUnsupported { corner: SeamVertex, policy: None }` — a zero-constructor tag naming NO run-out policy, because none would help — with a recourse that names the request instead: ask for the rim WHOLE. (b) **The genuine mid-curve run-out is real, PARKED, consumer-gated**, in two named shapes: the **ball-cap stop** (the ball at rest at the final station caps the band with a sphere patch — well-defined at any smooth interior point, the `corner_ball` machinery's smooth sibling; new surgery, no new surface kinds) and the **feather-out** (the radius tapers to zero approaching the station — variable-radius-shaped, frontier (f) adjacent, strictly more machinery). Ball-cap is the presumptive first pick when a consumer arrives | No consumer has ever wanted either: every consumer the whole ARMS program met wanted the full rim (the bud, the snowman, the lentil, every solid of revolution). **The seam tag's recourse is honest but not yet fully served**, and the gap is the closed-rim door's, not the taxonomy's: a rim a chart seam has split is TWO arcs, so the whole-rim request is a multi-link closed chain, and the ring-free annulus band is a ONE-EDGE rim's — a seam-split rim's band is still uncarved (#1022). Pinned live at `crates/sweep/tests/verbs_arms3.rs` (the witness reproduced, both the before/after refusal and the whole-rim frontier) |
| **variable-radius fillet** | radius varies along the spine | the canal-surface blend (banked, consumer-gated — DESIGN frontier (f)): a variable-radius spine is generically neither line nor circle | Band-3; re-opens the canal unit with a consumer. The Klein bottle supplies a CONSTANT-radius one for frontier (f) as well: blending the top loop's torus against the body's cone, taken literally, has supports that share no axis, so the rolling ball's spine is neither line nor circle. The bottle sidesteps it with a tangent neck cylinder (2026-08-16), which is the modeller's answer, not the kernel's |
| **draft** | tapered replacement of walls for molding | a certified re-geom pass (attach layer + a pass-owned vertex step — NOT the M6-1 graft shape, which adds/kills entities; DRAFT-DESIGN DR2) + the pull-direction selection predicate (DR3) | design record: `docs/DRAFT-DESIGN.md` — plane-wall v1; the cylinder arm mints cones and is its own later unit, a plane×cone fitted-SSI lane (DR1 as corrected: R1's conic-inventory refusal bars only exact special cases and stands untouched) |
| **hole features** | counterbore / countersink / tapped | sugar over booleans + patterns per D8 (structural parameters); the recipe-layer node vocabulary | substrate shipped (`PlacedUnion` × `Subtract` spells a counterbore today); remaining: the sugar vocabulary (MIRROR-DESIGN P4/P6), face-tied placements (GROUP-BOOLEAN's staged item), and overlapping cutters behind G8's multi-solid-operand residual |
| **patterns (linear/circular) + mirror** | D8 structural-parameter instancing | the patterns half is SHIPPED (`Node::Pattern` linear/circular/explicit + `PlacedUnion`, `Instance(i)` naming per the ratified obligation, part-level); MIRROR is the open half — reflection instancing, lily wall 5, with the equivariance premise a named prerequisite (A6) | design record: `docs/MIRROR-DESIGN.md` (P1 chart handedness, P2 its own door beside `transform_rigid`, P3 audit boundary); assembly instancing (import side) is the same family's foreign face — fixtures banked in STEP-BANK |
| **helix / thread** | helical sweep | the ≥0.5-turn frontier is EXECUTED and filed (#222: nurbs_span_meter ParamSpan under near-antipode frame roll); joined-path sweeps banked | #222 is the named blocker, not a guess |
| **taper / variable-section sweep** | section scales/varies along path | lily wall 9; canal-adjacent (variable-radius tube = circular case) | Band-3 breadth |
| **lofted membrane / sheet bodies** | zero-thickness faces (petals) | D1's manifold-first boundary: sheet/wire bodies are the named non-manifold extension trigger ("add a non-manifold representation later only if sheet/wire bodies demand it") | lily wall 10; a real D1 design conversation, not a feature |
| **spheroid / ellipsoid primitive** | non-spherical quadric | D3 closed-enum extension (new analytic kind: every dispatch site enumerated by the compiler) or NURBS route | lily wall 4 |
| **rib / text** | Band-3 conveniences | text = profile vocabulary + patterns; rib = draft-adjacent | far tail |
| **datum planes / axes** | reference geometry | recipe-layer entities with stable names (N-doc machinery exists) | GUI-era consumer |
| **curved boolean breadth** | cyl×sphere, sphere×sphere, cone/torus operands | the banked germ-chord lanes (DESIGN frontier (d)); the SSI lift removed the storage half | each lane is its own unit; the teapot's spout∪body would ride revolve-surface × NURBS classes — far. **The operand gate is PAIR-scoped** (VERBS-GATE): a kind with no wired arm disqualifies an operation only where its BOX may meet a face of the other operand, so a cone or torus face that clears the other body no longer gates anything, and the refusal names the germ PAIR and both faces (`CurvedPairUnsupported`) with the box conservatism stated — overlap is a *may*, not a *does*. **No germ class was added**, so what moved is honesty and reach, not breadth. Measured on the register's own consumers: the Klein bottle's pieces still cannot be joined, but the refusal is now the flare's CONE against a plane of the loop — NOT the coincident annular mate the model cares about — and its self-intersection still cannot be trimmed, the same pair under `Subtract` (`klein::wall_probes` walls 3, 4). The lily's tepal seam, asked as sphere×sphere, answers `(Cone, Sphere)`: the lantern's conical pucker reaches the carving ball first, so that wall waits on the cone lane as well as on sphere×sphere (`lily::wall_probes` wall 7) |
| **point-section loft ("generalized cone")** | loft whose end section degenerates to a point (apex) | three tiers (Evan's mark-down, 2026-08-10, PR #300-era chat): (1) circle profile + straight axis = the ANALYTIC CONE — mint exact CONICAL_SURFACE (the tube_along_arc exact-intent pattern; #256 always-promote applies), mostly sugar + recognition; (2) polygon profile = pyramid, plain planar walls, possibly already expressible; (3) general curved profile = a NURBS wall with a collapsed boundary row — needs apex-degenerate certification (span meter is honestly zero at the apex, normals undefined, quadrature hulls loose) with the apex as a vertex-loop (the sphere-pole precedent) | no design conversation yet by ruling ("mark it down for the future"); tier 1 is a cheap early pick when a consumer appears |
| **declared conformal / REST joins (curved)** | ball-and-socket, interference fits | **this one IS M9** — C7 (CONTACT-DESIGN, ratified #178) | listed for completeness; the register's one M9 row |

Consumers waiting on rows above: the Utah teapot (shell), the full
calochortus rebuild (C7 — in M9), helical parts (#222), the
petal'd lily (sheet bodies), and **the Klein
bottle** (shell; curved booleans both ways; the canal blend —
`demos/tour/src/klein.rs`, whose module docs carry the findings list
and whose `wall_probes` runs every refusal live). The bottle is
deliberately NOT a consumer of the coaxial curved-fillet arms: the
meridian arc is the better answer there, per the row's own note.

## Scope limits and defects met by consumers — NOT missing verbs

Added 2026-08-16 with the Klein bottle, which met five of these in one
model. They are a different KIND of entry from the table above: the
verb exists and is reachable, but it refuses (or mis-answers) inside
what a modeller would call its own territory. They live here because
this register is where "what the kernel will not let me do" stops
being a scattered note — the section schedules nothing, exactly like
the table.

- **`fillet_edges` still refuses every full-revolve rim — now
  honestly.** The mis-metering half of this entry was FIXED by
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
  (cone×cylinder etc.) still refuse `SpineUnsupported` earlier, at the
  analytic-arm table — VERBS-ARMS-2's territory — and
  `klein::wall_probes` walls 1 and 2 still agree there.
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
  the full-period form is a multi-shell curved solid, so it is
  EXPECTED to join the STEP row below rather than escape it — stated
  as an expectation, not a receipt: unlike the hollow ring, which
  `klein::wall_probes` pins in place, nothing in the tree runs the
  hollow tube through the STEP writer today. The tour scene that
  would pin it is issue #986.
- **A hollow ring cannot leave as STEP.** The one-call hollow ring
  itself SHIPPED (VERBS-RING retired `FullRevolveHoles`: a full
  revolve of a holed profile builds the multi-shell solid through the
  shared void-insertion door — the register's old defect row here),
  but the STEP writer's outward/void shell classifier has closed
  forms for planar faces only, so a multi-shell CURVED solid refuses
  `CurvedShellClassification` — the known standing gate of
  OFFSET-DESIGN O6's demo-gates list, which every hollow curved part
  (this ring today, the full-period `tube_along_arc_hollow` shell
  since VERBS-TUBEWALL, the shelled teapot when Wave 3 lands) hits at
  export. (Wall 6, re-baselined: it now pins THIS refusal on the
  ring it builds.)
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
