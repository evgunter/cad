# VERBS — the modeling-verb breadth program (plan)

Executes `docs/KERNEL-VERBS.md` (the register): the missing modeling
verbs whose prerequisites are already ratified, in dependency order,
plus the register's verb-gating defect rows. Kicked off by Evan
in-chat 2026-08-21. Branch prefix **`verbs/`** (orchestrator worktree
branch `mngr/kernel-verbs` predates the prefix and is armed
alongside). Narrative record and live state: `work/verbs/log.md`'s
tail, never this file. The register stays the reference view; rows
scheduled here are marked there only by this plan's existence — the
register itself never schedules.

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

**Every row here has MERGED**: RIM #910, CHAMFER #920, ARMS-1 #932,
ARMS-2 #962, ARMS-3 #1028, TUBEWALL #960, RING #933.

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
already removed the storage half. Each lane its own unit. **Rows 6
and 7 have MERGED** (GATE #1001; CYLCYL #1021 + #1044); 8-10 are
uncut:

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
   *(Wall 7 — the lily's tepal seam, `demos/tour/src/lily.rs:2206` —
   with this row's honest boxes the operand gate ADMITS the cut. The
   face-maximality precondition answered next until #1031's pole half
   landed (#1131); the scene now REPAIRS the lantern's caps first, so
   wall 7 reaches the reduction's curved PIERCE arm and pins
   `CurvedPierceUnsupported`. No germ pair has yet been exercised by
   it. Measured detail: `git show
   4eda8abec43166ec4c027bb401a8f2cf9f3f7a9f:docs/M9-5-SPEC.md`, seam
   K2.)*
7. **VERBS-CYLCYL (two PRs: A = shared germ substrate L, B = the
   arms M)** — cylinder×cylinder germ lane. Spec:
   docs/VERBS-CYLCYL-SPEC.md (2026-08-26 survey; the SEQUENCE
   REORDERS 7 → 9 → 8 — SPHSPH promoted, CYLSPH last and alone on
   the fitted rung). Klein wall 3 is gated by row 10's cone lane,
   NOT rows 7-9 (measured at #1001). Promoted into
   the wave by the scan: #347 is the strongest live demand signal,
   and the half that stays open is the union — two `circle`-derived
   cylinders refuse to union AT ALL. Its carrier-vs-trimmed-arc
   conservatism was the other half and is fixed, so
   `crates/pncad-py/examples/bracket.py` asks for the 6 mm corner it
   actually wants.
8. **VERBS-CYLSPH (L)** — cylinder×sphere germ lane (the #250
   re-banked row; the SSI lift removed the storage half).
9. **VERBS-SPHSPH (M/L)** — sphere×sphere germ lane. Sphere polar
   rims carry two accepted-direction predicate defects (#723 −47%
   certified volume through a pole-crossing meridian arc; #893
   near-polar lever collapse) — a lane minting sphere faces with
   polar rims must not treat `props_rim_level` as a closed premise.
   *(Wall 7 is a demand signal for this lane, but not a sufficient
   one: `lily.rs:2192-2201` names items 6 and 9 as its dependency
   slate, and with #1131 landed the wall now stops at the curved
   PIERCE arm before any germ pair is reached. A sphere×sphere arm
   alone does not flip it.)*
10. **VERBS-CONE (L)** — cone (and torus) operand lanes, sequenced
    on what 6–9 learn. #226 residual 1 (conic-trimmed cylinder walls
    slip both sense gates) is the known trap; #685 (cone-wedge grid
    sizing drops `nv` at `nu == 1`) sits on the mesh side.

Wave-2 substrate riders: **#762 is DONE** — the guard refuses
non-finite at `crates/geom-brep/src/ssi.rs:991`, landed outside VERBS
at `91164e3b` (the issue wants closing). Still riding, folded into
whichever lane first opens the file: #726/#727 (the iso-rectangle
refusal's owner — the
curved-pierce door these lanes build inherits the pre-#648
arrangement; read both before wiring protection transitively).

## Wave 3 — Q8: offset → shell → the teapot

**MERGED, whole**, in the cut ratification produced:
**VERBS-OFF-A** (#994 — the analytic kinds, closed under offset, the
D3 payoff); **VERBS-OFF-B/C** (#1003, #1012 — the approximating-
surface machinery: intensional `Offset(S,d)`, fit, certified residual
≤ ε, at rest as `Surface::Approx`); **VERBS-OFF-D** (#1043 the
face-replacement door, #1048 the shell verb — open-shell /
face-removal vocabulary per D1 + the verb); and the **Utah teapot
demo** (#1078, the verb's designated demo, Evan 2026-08-09).

Wave-3 substrate the scan pins (the conversation must account for
them): #453/#390 — rational-patch quadrature hull FLOORS the
enclosure, so a fitted (likely rational) offset surface today could
not certify volume: shell rides that lane or schedules it; #528
(lower stretch bounds restricted to constant-arm charts); #427 (the pcurve
unification is RATIFIED — U2, #514 — and executes as the PCURVE
program); #870 (the area enclosure is unmetered, and shell is an
area-and-thickness verb). The teapot demo will meet #757/#758/#759/
#796 (API gaps demos already route around) and #743 (StlOptions
header panic on plausible part names) — demo findings, recorded not
dodged, per the demo-purpose rule.

**OFF-D** carries the **apex-window predicate** (ordinal-73 review's
named residue): before treating a cone mint as a face's offset,
decide `inf(v-window) + d·cot α > 0` (slant meters) — a shifted
window crossing the apex silently reads mirror-nappe geometry.

It is NOT OFF-C's. `SurfaceDescription::Offset` takes a NURBS base
by type — analytic kinds are closed under offset and mint exactly
through `offset_surface`, so no cone-based description is
representable and there is no apex band for an approximating
surface to window around. The predicate arises where mixed
analytic surgery does: the face-replacement unit.

## Wave 4 — what the consumers measured (opened 2026-08-27, unplanned)

Not cut at kickoff. Waves 1-3 shipped the verbs; the demos then asked
them for real parts and the answers cut this wave. Every unit here was
opened by a MEASUREMENT on a shipped verb, not by a scan row.

11. **VERBS-TESSFOLD (#1045)** — the five uncovered tess-lint scenes
    audited and folded, 146 rows, verified-not-blessed. #1038 (the gate
    class) stays open.
12. **VERBS-DEMO2 (#1054)** — the `hollowtorus` scene; #986's content.
13. **VERBS-PIERCE (#1068)** — the curved pierce/split substrate.
14. **VERBS-TEAPOT (#1078)** — shell's designated demo, which measured
    both of the shell defects below.
15. **VERBS-SHELLFIX** — #1082 (`shell_open`'s rim, PR-1 #1099) and
    #1081 (the oblique junction): **PR-2a #1126** landed the planar half
    (`topo::offset_planes_together`), **PR-2b is IN FLIGHT** for the
    curved corners via the C5 table (#1057). Spec:
    `docs/VERBS-SHELLFIX-SPEC.md` — the one VERBS spec sweep 3 kept,
    because it still binds.
16. **VERBS-LILYWELD** — PR-1 #1109 re-authored the flower/arch junction
    circle-coincident on Evan's content call; PR-2 #1127 closed as a
    MEASUREMENT that dissolved its own premise (the plan note below).
17. **VERBS-F7POLE (#1131)** — #1031's POLE half, as a REPAIR in
    `merge_coplanar_faces`, not a gate narrowing. #1031 stays open for
    the ordinary coplanar pair at a full-valence edge.

Open shell residue this wave has not scheduled: #1055 (the curved
wall-clearance window), #1056 (shell of an already-hollow body), #1058
(curved-rim narrowing / winding predicate / per-call pcurve mint),
#1018/#1019/#1020 (the `Approx` face's mesh, perf and transform lanes).

## Design conversations (Evan-paced; open as info firms up)

- **Q8 offset/shell elaboration** — RATIFIED (#907,
  `docs/OFFSET-DESIGN.md`); Wave 3 executed it.
- **Draft** — RATIFIED (#908, `docs/DRAFT-DESIGN.md`): plane-wall v1,
  the cylinder arm its own later plane×cone fitted-SSI lane. The VERB
  is unscheduled — no unit cut.
- **Patterns/mirror (D8)** — the patterns half is SHIPPED; MIRROR is
  RATIFIED (#909, `docs/MIRROR-DESIGN.md`: its own door, u↦−u, the
  audit-checklist scope) and unscheduled. Blocks hole features.
- **Sheet bodies (D1 extension)** — the named non-manifold trigger;
  a real D1 conversation, not a feature.
- **Point-section loft tier 1** — cheap when a consumer appears; by
  ruling ("mark it down for the future") not opened until one does.

## Out of this program

C7/REST joins (M9); variable-radius fillet (frontier (f),
consumer-gated); hole features / rib / text (behind patterns or far
tail); **datums SHIPPED** outside this program (register row 49);
spheroid primitive (unclaimed, no consumer pressure
beyond lily wall 4); #555 and the loft U-turn gate (not verb-gating,
per kickoff ruling).

## Standing constraints (from the 2026-08-21 issue scan)

- **#883 is PARKED** (folded into H-f/H5): W1 units assume
  `T: Decide + Bounds` on the fillet signatures and do NOT re-attempt
  the `CertifiedBounds` tightening as a side effect (its lone blocker
  is `wire_blend`, the one generic blend lowering `wire_fillet` and
  `wire_chamfer` collapsed onto in SEAT-4; #687 and #279 sit behind any
  signature touch).
- **Tessellation-gate claims stay suspect**, on two live defects:
  **#746** (tess-lint joins on face ORDINAL — a reorder compares the
  wrong faces or drops them silently; `diefillet` already has 16
  permuted ordinals) and **#1038** (the gate stops comparing when the
  corpus outgrows its reference and reports green by not looking).
  VERBS-TESSFOLD (#1045) closed the coverage half — the five uncovered
  scenes' 146 rows are folded, verified rather than blessed — and left
  #1038's class fix open. A VERBS spec claims a tessellation
  acceptance row only with the join fixed or the row hand-verified.
  #782's substance is discharged (finding-13 re-pinned at
  `demos/tour/src/lily.rs:2984`; the whole `demos/tour` suite runs in
  k-lint's release-default row) and the issue wants closing.
- **#555 stays out of VERBS** (kickoff ruling) but is a live wall in
  front of meshing full-revolve products — W1 acceptance rows that
  tessellate revolves must not wire that refusal into a gate.
- **#795 is open** (should a demo surface a typed refusal as a clean
  nonzero exit?) — VERBS demos follow whatever it ratifies; until
  then, match the Klein wall-probes precedent.

## Protocol

Implementer dispatches ride the A/B ledger (`docs/MODEL-AB-LOG.md` on
main at dispatch — block draws, ordinals, dual-review sampling, v6
review lanes). Briefs point at `docs/prompts/implementer-discipline.md`
and reviews at `docs/prompts/reviewer-style-lane.md`, by path. Unit
branches `verbs/<unit>`; lanes via `local-scripts/new-lane.sh`;
state-sync docs PRs at every pipeline seam.

## Plan note (#1059): lily wall 2's disposition

Wall 2's stated blocker (transverse curved×curved SSI — the
banked germ-chord lane) is NOT the binding refusal, and neither is
the operand KIND gate that refuses first (`op: None`,
reduce.rs:341). Evan ruled the content question: the flower/arch
junction is authored circle-coincident (VERBS-LILYWELD, #1109).
Measured there, the weld's declared contact is plane×plane and no
declaration covers the cone×torus pair, so the #968-shaped gate
admission has nothing to consult — it is DEFERRED, and the
`carrier_eq` rung has no consumer. Wall 2's measured chain is
gate → F7 → the curved-pierce door. #1031's POLE half landed at
#1131 — necessary, not sufficient; the wall-2 probe deliberately
still passes the UNREPAIRED lantern (`lily.rs:2019-2021`), so the
widened-gate sequence has not been re-measured on the repaired
one. The germ-chord lane has no
other near-term consumer, so its banking deepens. Probe texts are
not evidence of cause — payload + raising site are
(`memories/refusal-text-is-not-cause.md`).

## Plan note (#1031): the cap unit's opening measurement

The measurement the unit opened with — does one maximal cap fall
out of revolve/full.rs's own wire-case construction (half = θ/2,
rot_pi), or does it require the role-ambiguous merge that
merge_coplanar_faces refuses? — is TAKEN, and it falsifies the
premise Evan's two recorded options share (his lean was (b),
producers mint maximal caps; (a), the repair op, first if that is
merge-shaped). A full revolve sweeps in TWO π-bands precisely so
each pole ends valence-2: merging one meridian away leaves
valence 1, which tier 2 bans as strut scaffolding, and merging
both leaves the pole interior to the face, which is exactly the
`MergedFaceRoleAmbiguous` the merge door refuses. There is no
one-face cap to mint or to repair, so what is wrong is the RULE —
F7's planar same-key refusal over-reaching onto revolve poles,
where the identical CURVED shape is already canonical. #1031
therefore split: the POLE half and the ordinary coplanar pair. The
POLE half LANDED at #1131 — and **not** as the gate exemption this
note anticipated. Two structural exemptions were tried and each
admitted a shape it claimed to exclude, both falsified by fixtures
already in the repo and both withdrawn; what shipped is a REPAIR in
`merge_coplanar_faces` (`crates/topo/src/merge_faces.rs:794`),
removing a redundant subdivision vertex on a shared collinear seam,
which changes no locus. Evan's original repair-op steer was right.
#1031 stays open for the ordinary coplanar pair at a full-valence
edge (the teapot cup's meridian plane, endpoints of valence 4).
The deviation from the recorded steer is flagged for Evan's
retroactive review; the plane-face interior-seam question is ruled
permissible (a preference to avoid, not a wall).

## Plan note (2026-08-29, #1200's work-stream survey): VERBS' claims vs the streams

Per Evan's ruling ("take everything that's naturally within your
work"): VERBS CLAIMS #347's remaining half (the germ-arms unit),
#1031 half B, #1076, and #1077 — the S-BOOL anchors that were
already Wave-4 queue items with drafted specs (spec-drafts/ in
cad-work). #1059 is resolved and drops from any cut. CEDED with
handoff records on #1200: S-BLEND's fillet residue (#1022 builds
to the A3-2 measured record, not the issue's framing; #827
starts from LILYWELD's JunctionTangent payload), S-CERT's
#723/#893 (SPHSPH inherits neither, plants the near-polar red),
S-MATE's #968 (the #966 record + LILYWELD's killed-rung context).
S-BOOL's honest remainder (#1011/#750/#542/#368/#433/#1152/#134)
was never VERBS'.
