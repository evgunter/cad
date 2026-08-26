# VERBS — the modeling-verb breadth program (plan)

Executes `docs/KERNEL-VERBS.md` (the register): the missing modeling
verbs whose prerequisites are already ratified, in dependency order,
plus the register's verb-gating defect rows. Kicked off by Evan
in-chat 2026-08-21. Branch prefix **`verbs/`** (orchestrator worktree
branch `mngr/kernel-verbs` predates the prefix and is armed
alongside). Narrative record: `docs/VERBS-LOG.md`. The register stays
the reference view; rows scheduled here are marked there only by this
plan's existence — the register itself never schedules.

**Kickoff rulings (Evan, in-chat 2026-08-21):**

- Wave order as below; an issue-scan subagent surveyed all open
  GitHub issues (2026-08-21) and its findings are folded in below —
  per-unit constraints live in the unit specs, cross-cutting ones in
  "Standing constraints" at the end.
- VERBS owns the register's **verb-gating** defect rows: #554 (closed-
  rim lever arm), `FullRevolveHoles`, and `tube_along_arc`'s missing
  wall parameter. The other defect rows (#555 mesh sub-floor, the
  loft U-turn gate, edge-selection reach) stay with their home
  programs / the #614 routing.
- Design conversations open **as soon as the relevant info is
  available** — likely most now — rather than queuing behind their
  implementation waves. Each is a design-conversation PR per the
  standing rule (Evan sign-off before merge).
- The C7 / declared-conformal row stays M9's. Helix: **stays
  blocked** — #222's closure records an executed measurement, not a
  fix (its own text: fix shape "segmented/rolled chord meter…,
  unscheduled"), corroborated by #368; the register is accurate as
  written.

## Wave 1 — cheap, already-ratified plumbing

1. **VERBS-RIM (#554, M)** — the fillet battery's lever arm is the
   endpoint chord, ~0 on every closed rim, so full revolves get a
   FALSE `TangentialEdge` on 30° dihedrals. Fix the metering
   (`crates/sweep/src/fillet/battery.rs`, `extent_of`), with the
   scan's two widenings: the replacement functional is a DESIGN CALL
   made in the spec (not a closed-case bolt-on), and `arm_len` feeds
   the chain/clearance predicates too, so every consumer is in
   scope. Spec: `docs/VERBS-RIM-SPEC.md`. First because it gates
   every fillet consumer on a solid of revolution.
2. **VERBS-CHAMFER (M)** — the fillet's ruled-surface sibling: swap
   the rolling-ball band for a ruled strip over the existing
   trimline/support-split infrastructure (M5 PR 12 + M6-1 surgery).
   The register's cheapest verb row. Rides after VERBS-RIM (same
   files). Scan constraints: the corner code is convexity-parametric
   in name only (#644) — do not derive one of its four convex-only
   arguments alone; no new spelling for enclosing tangency (#827);
   a chamfer emitter copies `emit_topo`'s TieRows deferral (#708).
3. **VERBS-ARMS (L)** — the C8-ratified analytic constant-radius
   fillet arms `classify_arm` never implemented: sphere×cone,
   cone×plane, cone×cone, sphere×sphere, and the cylinder pairs
   (CURVED-DESIGN C8: circular-arc spine → torus patch, cone cases →
   cone/torus; `PlaneSphereTorus` already mints the torus). Consumer:
   the calochortus bud's sphere–cone seam (#319). The coaxial arms
   ride along per the register's own note (meridian-arc authoring
   stays the better answer where it applies — no consumer claimed).
   Scan widenings from #319's own body: the corner run-out door at a
   valence-4 seam vertex (`FilletCornerUnsupported { NEdgeVertex }`)
   is a CO-REQUISITE — the supported plane×sphere arm already dies
   there; the coaxial circle-spine derivation is untested in-repo
   (verify, don't assume); prior art re-mintable from
   `git show 60941420:crates/sweep/src/fillet/blend.rs`
   (`corner_contact_circle`, `BlendArm::name`) — re-mint, never
   restore; minting tori from curved supports makes #889's missing
   `R > r` validate check load-bearing — it lands here. VERBS-RIM's
   review added a fourth co-requisite: **the one-edge CLOSED chain**
   — post-RIM, a supported closed rim passes the battery and
   `fillet_edges` refuses `UnsupportedChain` ("fewer than two
   links"), and a closed single-link chain never receives a
   wrap-around G1 check — so the verb-level unlock for full solids
   of revolution (the #554 consequence that remains) is THIS unit's
   acceptance, not RIM's. **Cut into three sub-units at the
   2026-08-22 survey (docs/VERBS-ARMS-SPEC.md): ARMS-1 closed-rim
   surgery + #889's torus net (the #554 unlock, no new arms);
   ARMS-2 the coaxial arms (one shared torus derivation — and the
   C8 "cone" prose corrected: constant-radius rolls mint only
   torus/cylinder); ARMS-3 general sphere×sphere + the OQ6
   valence-4 run-out door (a design conversation, Evan-gated).**
4. **VERBS-TUBEWALL (S)** — `tube_along_arc` grows a wall/inner-
   radius parameter so hollow tubes keep the door's exact-intent
   storage. No design record yet: the unit PR carries the (small)
   design elaboration; self-merges only if it stays a faithful
   elaboration of the door's existing contract.
5. **VERBS-RING (M)** — retire `FullRevolveHoles` under the #907
   refined invariant (every cavity born through the shared
   void-insertion door): the holed full revolve is DEFINED as
   `revolve(outer) − revolve(hole-as-outer)` and executed as the
   degenerate no-crossing arm. The unit FACTORS the boolean's
   void-insertion door callable-without-SSI (its first consumer,
   ahead of Wave 3's shell, which inherits it). One-call hollow
   ring becomes available (register wall 6). **Gated on #907's
   ratification** (which revises DESIGN.md's M2 bullet in place).

## Wave 2 — curved boolean breadth

The banked germ-chord lanes (DESIGN frontier (d)); the SSI lift
already removed the storage half. Each lane its own unit:

6. **VERBS-GATE (M)** — the operand gate is per-face-KIND over the
   whole body, so one cone/torus face makes every boolean unavailable
   to the body (Klein bottle walls 3–4). Re-scope the refusal to the
   face pairs that actually meet, refusing typed only where an
   unsupported KIND pair genuinely intersects. Spec must rule what
   "genuinely intersects" costs (box-level conservatism is the
   likely shape) — and NOTE the box machinery it would lean on has a
   live wrong answer: #862's axial-slab over-widening (37% measured)
   feeding false `CensusUndecidable`, with #700's lapsed-premise
   duplicate. Dispatch after Wave 1 lands evidence.
   *(Steered demand signal — Evan's ruling on #966, 2026-08-23,
   recorded by the M9 orchestrator: lily wall 7, the tepal seam —
   `lily.rs:1563`, a sphere×sphere SUBTRACT refused at the
   non-union kind gate ops.rs:415-427 before the cut is attempted —
   waits on THIS item plus item 9; its probe's retirement text
   executes when both land. docs/M9-5-SPEC.md seam K2 carries the
   measured detail. CORRECTED TWICE, then RESOLVED at #1001's fix
   pass (2026-08-26): the interim (Cone, Sphere) attribution was a
   BOX ARTIFACT (two layers of inflation — the full-cone slab, then
   tilted-axis corner projection; the exact frustum clears the ball
   by 0.2909). With honest boxes the gate ADMITS the cut, and the
   true refusal is `NonMaximalFaces`: the lantern's zone is two
   half-bands on one surface key — a full-revolve PRECONDITION
   (`merge_coplanar_faces` / the F7 door), not curved-boolean
   breadth. Wall 7 is therefore NOT a demand signal for rows 6/9/10
   — its retirement text re-points at the face-maximality
   precondition; the sphere×sphere germ arm was never reached.)*
7. **VERBS-CYLCYL (L)** — cylinder×cylinder germ lane. Promoted into
   the wave by the scan: #347 is the strongest live demand signal
   (two `circle`-derived cylinders refuse to union AT ALL;
   `examples/bracket.py` rounds at 3 mm instead of 6 mm). #347's
   carrier-vs-trimmed-arc conservatism is a SEPARATE predicate bug
   that rides whichever unit opens that code.
8. **VERBS-CYLSPH (L)** — cylinder×sphere germ lane (the #250
   re-banked row; the SSI lift removed the storage half).
9. **VERBS-SPHSPH (M/L)** — sphere×sphere germ lane. Sphere polar
   rims carry two accepted-direction predicate defects (#723 −47%
   certified volume through a pole-crossing meridian arc; #893
   near-polar lever collapse) — a lane minting sphere faces with
   polar rims must not treat `props_rim_level` as a closed premise.
   *(The wall-7 steering formerly recorded here RESOLVED elsewhere:
   see item 6's note — with honest boxes the blocker is a
   full-revolve face-maximality precondition, not this lane.)*
10. **VERBS-CONE (L)** — cone (and torus) operand lanes, sequenced
    on what 6–9 learn. #226 residual 1 (conic-trimmed cylinder walls
    slip both sense gates) is the known trap; #685 (cone-wedge grid
    sizing drops `nv` at `nu == 1`) sits on the mesh side.

Wave-2 substrate riders, folded into whichever lane first opens the
file: #762 (`plane_nurbs_ssi` chart-speed guard admits `+∞` — three-
line fix), #726/#727 (the iso-rectangle refusal's owner — the
curved-pierce door these lanes build inherits the pre-#648
arrangement; read both before wiring protection transitively).

## Wave 3 — Q8: offset → shell → the teapot

Gated on the Q8 design conversation (below). Anticipated cut, to be
re-cut at ratification: **VERBS-OFF-A** (analytic kinds, closed under
offset — D3 payoff); **VERBS-OFF-N** (the approximating-surface
machinery: intensional `Offset(S,d)`, fit, certified residual ≤ ε,
mirroring fitted intersection curves); **VERBS-SHELL** (open-shell /
face-removal vocabulary per D1 + the verb); the **Utah teapot demo**
(the verb's designated demo, Evan 2026-08-09) with the Klein bottle's
hand-authored double-offset walls as the second consumer.

Wave-3 substrate the scan pins (the conversation must account for
them): #453/#390 — rational-patch quadrature hull FLOORS the
enclosure, so a fitted (likely rational) offset surface today could
not certify volume: shell rides that lane or schedules it; #528
(lower stretch bounds restricted to constant-arm charts); #427 (the
pcurve-unification design conversation is OPEN — a W3 spec must not
pre-empt it); #870 (the area enclosure is unmetered, and shell is an
area-and-thickness verb). The teapot demo will meet #757/#758/#759/
#796 (API gaps demos already route around) and #743 (StlOptions
header panic on plausible part names) — demo findings, recorded not
dodged, per the demo-purpose rule.

OFF-C/D carry the **apex-window predicate** (ordinal-73 review's
named residue): before treating a cone mint as a face's offset,
decide `inf(v-window) + d·cot α > 0` (slant meters) — a shifted
window crossing the apex silently reads mirror-nappe geometry.

## Design conversations (Evan-paced; open as info firms up)

- **Q8 offset/shell elaboration** — first; gates Wave 3.
- **Draft** — "no design record yet — needs its own conversation"
  (register). Face-replacement surgery generalized + tapered mint.
- **Patterns/mirror (D8)** — recipe-level instancing; pattern indices
  are a ratified naming-doc requirement; mirror needs reflection
  instancing + the D9 conv. 4 equivariance frame. Blocks hole
  features.
- **Sheet bodies (D1 extension)** — the named non-manifold trigger;
  a real D1 conversation, not a feature.
- **Point-section loft tier 1** — cheap when a consumer appears; by
  ruling ("mark it down for the future") not opened until one does.

## Out of this program

C7/REST joins (M9); variable-radius fillet (frontier (f),
consumer-gated); hole features / rib / text / datums (behind patterns
or far tail); spheroid primitive (unclaimed, no consumer pressure
beyond lily wall 4); #555 and the loft U-turn gate (not verb-gating,
per kickoff ruling).

## Standing constraints (from the 2026-08-21 issue scan)

- **#883 is PARKED** (folded into H-f/H5): W1 units assume
  `T: Decide + Bounds` on the fillet signatures and do NOT re-attempt
  the `CertifiedBounds` tightening as a side effect (its lone blocker
  is `wire_fillet`; #687 and #279 sit behind any signature touch).
- **Tessellation-gate claims are suspect until #746/#782 resolve**:
  tess-lint joins on face ORDINAL (a reorder compares the wrong faces
  or drops them silently — `diefillet` already has 16 permuted
  ordinals) and the tour's finding-13 pin is RED on main with no lane
  running it. A VERBS spec claims a tessellation acceptance row only
  with the join fixed or the row hand-verified.
- **#555 stays out of VERBS** (kickoff ruling) but is a live wall in
  front of meshing full-revolve products — W1 acceptance rows that
  tessellate revolves must not wire that refusal into a gate.
- **#795 is open** (should a demo surface a typed refusal as a clean
  nonzero exit?) — VERBS demos follow whatever it ratifies; until
  then, match the Klein wall-probes precedent.

## Protocol

Implementer dispatches ride the A/B ledger (`docs/MODEL-AB-LOG.md` on
main at dispatch — block draws, ordinals, dual-review sampling, v5
review lanes). Briefs point at `docs/prompts/implementer-discipline.md`
and reviews at `docs/prompts/reviewer-style-lane.md`, by path. Unit
branches `verbs/<unit>`; lanes via `local-scripts/new-lane.sh`;
state-sync docs PRs at every pipeline seam.
