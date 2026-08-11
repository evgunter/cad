# Missing kernel operations — the modeling-verb register

**Status: reference register (Evan's ask, 2026-08-09).** The
modeling operations the kernel does NOT yet have, each with its
prerequisites as the ratified records state them. These are
feature-breadth work, mostly NOT part of M8 (error propagation);
this register is their home so they stop living in scattered walk
rows and banked notes. DESIGN.md's Band 3 "feature breadth"
paragraph names the GUI-era shape of several; this register is the
kernel-side view with dependencies. Rows move to a milestone plan
when scheduled; the register never schedules anything itself.

**Present today, for contrast**: extrude, revolve (partial/full),
loft, sweep (straight + curved single-arc path), booleans (planar
complete; curved per wired germ classes plane×cyl / plane×sphere),
split, constant-radius edge fillets (**plane–plane and plane–sphere
supports only** — see the row below) + in-place composition surgery,
merge_coplanar_faces, rigid transform, tessellation/STL/STEP
export, STEP import (adoption incl. recognition + tier gate).

| verb | what | prerequisites / blockers (as ratified) | notes |
|---|---|---|---|
| **shell / hollow** | offset the boundary inward, remove faces, thicken | Q8: offset surfaces — analytic kinds are CLOSED under offset (a D3 payoff); a NURBS offset is NOT a NURBS → needs the approximating-surface machinery (intensional spec `Offset(S,d)` + fit + certified residual ≤ ε, "exactly mirroring fitted intersection curves"). Also wants open-shell/face-removal vocabulary (D1's manifold-first boundary) | **The Utah teapot is this verb's designated demo** (Evan, 2026-08-09) — a vessel is a shelled revolve; the demo queues behind the verb |
| **offset (surface/solid)** | the standalone Q8 operation | same as shell's core; Q8 says "needed before shelling/offset work (M5+), stated now" | shell's substrate; may land as one unit |
| **chamfer** | the fillet's ruled-surface sibling | the fillet machinery's trimline/support-split infrastructure exists (M5 PR 12 + M6-1 surgery); a chamfer swaps the rolling-ball band for a ruled strip | likely the cheapest entry in this register |
| **constant-radius fillet on CURVED support pairs** | the arms of C8's analytic table that M5 PR 12 did not implement — sphere×cone, cone×plane, cone×cone, sphere×sphere, and the cylinder pairs | **Not frontier (f), despite sharing its error variant.** `classify_arm` (`sweep/src/fillet/battery.rs`) implements exactly two arms — plane×plane → cylinder, plane×sphere → torus — and everything else falls through to `FilletError::SpineUnsupported`, whose payload reads `non-(plane–plane / plane–sphere)`. Its own doc comment says so: "C8's list, **restricted to the arms this unit implements**". CURVED-DESIGN C8 already ratified the missing arms as ANALYTIC: "circular-arc spine with fixed profile orientation → torus patch; … cone cases → cone/torus". On a solid of revolution the sphere×cone rim is the easy case — offset sphere and offset cone are coaxial, so the spine is a circle and the blend is a TORUS, the surface `PlaneSphereTorus` already mints (derivation not yet tested in-repo) | Consumer: the calochortus bud's sphere–cone seam (2026-08-09). **Distinguish from DESIGN frontier (f)** — that is the canal-surface general blend, for spines that are neither line nor circle, and is parked for want of a consumer. These arms need no approximating surface at all, so they are plumbing, not research. Until this row lands, `SpineUnsupported` does not by itself mean "the canal case" |
| **variable-radius fillet** | radius varies along the spine | the canal-surface blend (banked, consumer-gated — DESIGN frontier (f)): a variable-radius spine is generically neither line nor circle | Band-3; re-opens the canal unit with a consumer |
| **draft** | tapered replacement of walls for molding | face-replacement surgery (the M6-1 split/graft pattern generalized) + tapered-surface mint (cone/ruled) | no design record yet — needs its own conversation |
| **hole features** | counterbore / countersink / tapped | sugar over booleans + patterns per D8 (structural parameters); the recipe-layer node vocabulary | blocked mainly on patterns |
| **patterns (linear/circular) + mirror** | D8 structural-parameter instancing | recipe-level: pattern indices are a ratified naming-doc requirement ("never degrade to positional guessing"); mirror additionally needs reflection instancing — lily wall 5; equivariance convention (D9 conv. 4) is the design frame | assembly instancing (import side) is the same family's foreign face — fixtures banked in STEP-BANK |
| **helix / thread** | helical sweep | the ≥0.5-turn frontier is EXECUTED and filed (#222: nurbs_span_meter ParamSpan under near-antipode frame roll); joined-path sweeps banked | #222 is the named blocker, not a guess |
| **taper / variable-section sweep** | section scales/varies along path | lily wall 9; canal-adjacent (variable-radius tube = circular case) | Band-3 breadth |
| **lofted membrane / sheet bodies** | zero-thickness faces (petals) | D1's manifold-first boundary: sheet/wire bodies are the named non-manifold extension trigger ("add a non-manifold representation later only if sheet/wire bodies demand it") | lily wall 10; a real D1 design conversation, not a feature |
| **spheroid / ellipsoid primitive** | non-spherical quadric | D3 closed-enum extension (new analytic kind: every dispatch site enumerated by the compiler) or NURBS route | lily wall 4 |
| **rib / text** | Band-3 conveniences | text = profile vocabulary + patterns; rib = draft-adjacent | far tail |
| **datum planes / axes** | reference geometry | recipe-layer entities with stable names (N-doc machinery exists) | GUI-era consumer |
| **curved boolean breadth** | cyl×sphere, sphere×sphere, cone/torus operands | the banked germ-chord lanes (DESIGN frontier (d)); the SSI lift removed the storage half | each lane is its own unit; the teapot's spout∪body would ride revolve-surface × NURBS classes — far |
| **point-section loft ("generalized cone")** | loft whose end section degenerates to a point (apex) | three tiers (Evan's mark-down, 2026-08-10, PR #300-era chat): (1) circle profile + straight axis = the ANALYTIC CONE — mint exact CONICAL_SURFACE (the tube_along_arc exact-intent pattern; #256 always-promote applies), mostly sugar + recognition; (2) polygon profile = pyramid, plain planar walls, possibly already expressible; (3) general curved profile = a NURBS wall with a collapsed boundary row — needs apex-degenerate certification (span meter is honestly zero at the apex, normals undefined, quadrature hulls loose) with the apex as a vertex-loop (the sphere-pole precedent) | no design conversation yet by ruling ("mark it down for the future"); tier 1 is a cheap early pick when a consumer appears |
| **declared conformal / REST joins (curved)** | ball-and-socket, interference fits | **this one IS M9** — C7 (CONTACT-DESIGN, ratified #178; renumbered from M8 at PR #300's closure ruling) | listed for completeness; the register's one M9 row |

Consumers waiting on rows above: the Utah teapot (shell), the full
calochortus rebuild (C7 — in M9), helical parts (#222), the
petal'd lily (sheet bodies), the calochortus bud's sphere–cone seam
(constant-radius fillet on curved supports, #319).
