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
split, constant-radius edge fillets (**plane–plane and plane–sphere
supports only, and only on OPEN rims** — see the row below, and the
lever-arm entry in the defects section) + in-place composition surgery,
merge_coplanar_faces, rigid transform, tessellation/STL/STEP
export, STEP import (adoption incl. recognition + tier gate).

| verb | what | prerequisites / blockers (as ratified) | notes |
|---|---|---|---|
| **shell / hollow** | offset the boundary inward, remove faces, thicken | Q8: offset surfaces — analytic kinds are CLOSED under offset (a D3 payoff); a NURBS offset is NOT a NURBS → needs the approximating-surface machinery (intensional spec `Offset(S,d)` + fit + certified residual ≤ ε, "exactly mirroring fitted intersection curves"). Also wants open-shell/face-removal vocabulary (D1's manifold-first boundary) | **The Utah teapot is this verb's designated demo** (Evan, 2026-08-09) — a vessel is a shelled revolve; the demo queues behind the verb. **Second consumer, and the first to PAY for the absence in full: the Klein bottle** (`demos/tour/src/klein.rs`, 2026-08-16). A thin 3-manifold whose midsurface is the immersed Klein bottle is nothing but shell, so every wall is authored as its own two offsets by hand — each radius spelled twice as `r ± t/2`, each blend radius twice as `R ∓ t/2` with the sign depending on which side the centre of curvature is on, and the offsets swapping sides wherever the surface turns back on itself. It builds and it is exact; it is the whole row, paid once per wall |
| **offset (surface/solid)** | the standalone Q8 operation | same as shell's core; Q8 says "needed before shelling/offset work (M5+), stated now" | shell's substrate; may land as one unit |
| **chamfer** | the fillet's ruled-surface sibling | the fillet machinery's trimline/support-split infrastructure exists (M5 PR 12 + M6-1 surgery); a chamfer swaps the rolling-ball band for a ruled strip | likely the cheapest entry in this register |
| **constant-radius fillet on CURVED support pairs** | the arms of C8's analytic table that M5 PR 12 did not implement — sphere×cone, cone×plane, cone×cone, sphere×sphere, and the cylinder pairs | **Not frontier (f), despite sharing its error variant.** `classify_arm` (`sweep/src/fillet/battery.rs`) implements exactly two arms — plane×plane → cylinder, plane×sphere → torus — and everything else falls through to `FilletError::SpineUnsupported`, whose payload reads `non-(plane–plane / plane–sphere)`. Its own doc comment says so: "C8's list, **restricted to the arms this unit implements**". CURVED-DESIGN C8 already ratified the missing arms as ANALYTIC: "circular-arc spine with fixed profile orientation → torus patch; … cone cases → cone/torus". On a solid of revolution the sphere×cone rim is the easy case — offset sphere and offset cone are coaxial, so the spine is a circle and the blend is a TORUS, the surface `PlaneSphereTorus` already mints (derivation not yet tested in-repo) | Consumer: the calochortus bud's sphere–cone seam (2026-08-09). **The COAXIAL arms of this row may need no consumer at all** (Evan, 2026-08-16, on the Klein bottle's neck→flare blend — a cone×cylinder pair): a blend between two coaxial surfaces of revolution is itself a surface of revolution, so it is authorable as an ARC IN THE MERIDIAN (`.fillet(r)` on the profile before revolving) — exact, free, and *better* than asking `fillet_edges` for it, because the blend is then a constructed part of the shape rather than a post-hoc roll. `klein::wall_probes` wall 2 records what the verb says today (`SpineUnsupported`) without claiming the bottle needs it. The escape closes as soon as the supports are NOT coaxial — that is the canal case below. **Distinguish from DESIGN frontier (f)** — that is the canal-surface general blend, for spines that are neither line nor circle, and is parked for want of a consumer. These arms need no approximating surface at all, so they are plumbing, not research. Until this row lands, `SpineUnsupported` does not by itself mean "the canal case" |
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
| **curved boolean breadth** | cyl×sphere, sphere×sphere, cone/torus operands | the banked germ-chord lanes (DESIGN frontier (d)); the SSI lift removed the storage half | each lane is its own unit; the teapot's spout∪body would ride revolve-surface × NURBS classes — far. **The operand gate is per-face-KIND and rejects the whole body**, so a single cone or torus face makes every boolean unavailable to it: the Klein bottle's three pieces cannot be joined (`union` → `CurvedBooleanUnsupported { kind: Torus }`) and its self-intersection — the neck through the body wall, the one crossing an immersed Klein bottle MUST have — cannot be trimmed (`subtract` → `CurvedOpUnsupported`). Walls 3 and 4 of `klein::wall_probes` |
| **point-section loft ("generalized cone")** | loft whose end section degenerates to a point (apex) | three tiers (Evan's mark-down, 2026-08-10, PR #300-era chat): (1) circle profile + straight axis = the ANALYTIC CONE — mint exact CONICAL_SURFACE (the tube_along_arc exact-intent pattern; #256 always-promote applies), mostly sugar + recognition; (2) polygon profile = pyramid, plain planar walls, possibly already expressible; (3) general curved profile = a NURBS wall with a collapsed boundary row — needs apex-degenerate certification (span meter is honestly zero at the apex, normals undefined, quadrature hulls loose) with the apex as a vertex-loop (the sphere-pole precedent) | no design conversation yet by ruling ("mark it down for the future"); tier 1 is a cheap early pick when a consumer appears |
| **declared conformal / REST joins (curved)** | ball-and-socket, interference fits | **this one IS M9** — C7 (CONTACT-DESIGN, ratified #178) | listed for completeness; the register's one M9 row |

Consumers waiting on rows above: the Utah teapot (shell), the full
calochortus rebuild (C7 — in M9), helical parts (#222), the
petal'd lily (sheet bodies), the calochortus bud's sphere–cone seam
(constant-radius fillet on curved supports, #319), and **the Klein
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
  What REMAINS: no full solid of revolution can be filleted at the
  verb level. A closed rim is a one-link closed chain, and the
  surgery's rim door refuses it typed (`UnsupportedChain`, "a closed
  chain of fewer than two links") because the one-edge torus band is
  not built; curved support pairs (cone×cylinder etc.) refuse
  `SpineUnsupported` earlier, at the analytic-arm table. Both
  remainders are VERBS-ARMS's territory. `klein::wall_probes` walls
  1 and 2 now agree (`SpineUnsupported` on the full and the partial
  revolve alike), and
  `verbs_rim_r1_probes::a_passing_closed_rim_reaches_the_surgery_and_refuses_unsupported_chain`
  pins the rim door live.
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
- **`tube_along_arc` is solid-only.** The torus door takes a
  `minor_radius` and no wall thickness, so nothing HOLLOW can use it
  — a thin tube must be re-said as a partial `revolve` of an annulus,
  giving up the door's whole point (the caller's intent parameters
  stored bit-exactly). A `wall`/inner-radius parameter is the obvious
  shape and has no design record yet.
- **A full revolve of a holed profile refuses `FullRevolveHoles`**, so
  the one-call hollow RING is unavailable while partial elbows are
  fine. The revolve's own docs name this as M2 scope ("the per-hole
  seam surgery is mechanical but unexercised by the plan's acceptance
  set") — recorded here because it is now a consumer's refusal and
  not only a deviation note. (Wall 6.)
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
