# S-BLEND — fillet/chamfer completion (plan)

**STATUS: OPEN.** Graduated from the ratified 2026-08-29 work-stream
survey (`docs/WORK-STREAMS-2026-08.md`, merged #1200 after Evan's
read, carrying VERBS' cession and both handoff records from that
PR's thread). Every design decision this plan leans on is ratified
elsewhere and cited, not re-litigated. Live state is
`docs/S-BLEND-LOG.md`'s tail, never this file.

Branch prefix (the #396 convention): **`blend/`** — unit branches
`blend/<slug>`, orchestrator branch `blend/orchestrator`.
Away-channel tag `(S-BLEND orchestrator)`. A/B ordinal band
**BLEND = 600–699**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in this same commit, per that entry's rule. Implementer
blocks are named `BLEND-B1, BLEND-B2, …`; `BLEND-<n>` are unit
names. Block draws are recorded branch-side on `blend/orchestrator`
at draw time and reach main only when the block's last slot's
reviews conclude (the ratified LIB shape, 2026-08-29 redaction
entry).

## Charter

Blend-verb breadth: the fillet band/surgery gaps the ARMS program
filed on its way out, and chamfer parity with the fillet. The verbs
exist and refuse honestly; this stream builds the missing doors.
Kernel-side only — the recipe layer is LIB's (see the seam below).

## Territory

**Owns:** `crates/sweep/src/fillet/` (admit, battery, blend, build,
naming, surgery) and the `sweep/tests/` files this stream's rows
name. **Claims SMELL track T whole** (`crates/sweep/`, fence as the
partition states it) **at the moment VERBS-SHELLFIX PR-2b (#1180)
merges** — not before; until then 2b's shell/offset files in
`crates/sweep` and `topo/replace_face.rs` are keep-out.

**Seams, each with the unit it gates:**

- **LIB-G16 (in flight, dispatched 2026-08-29 under ratified
  `docs/RECIPE-DOORS-DESIGN.md` D2+D3, spec `docs/LIB-G16-SPEC.md`)
  owns the chamfer recipe door and the emitter debt.** The survey's
  cut listed 918 and 708 here, but RECIPE-DOORS — ratified the same
  day, and G16 dispatched before the survey merged — puts
  `Node::Chamfer` (issue 918), `names::emit_chamfer`, and the
  re-shape of `emit_fillet` onto the `TieRows` deferral (the 708
  debt) in that LIB unit. S-BLEND builds neither and stays out of
  `editor-core/src/names/emit_fillet.rs` until G16 merges. What
  S-BLEND keeps is the KERNEL half of chamfer parity: 919 and the
  917 vocabulary question, both sequenced behind G16 (below).
- **VERBS** keeps its boolean-breadth queue; S-BLEND is ceded clear
  (VERBS orchestrator on #1200, at Evan's request) with two handoff
  records carried under "Handoffs" below.
- **M10, PCURVE, LIB façade/bindings**: no contact; the stream
  touches neither editor-core eval/schema nor geom-brep.
- **Naming/schema:** BLEND-5 (issue 961) widens a persisted name
  vocabulary; it claims its schema seam per the standing
  dispatch-time discipline at its own dispatch, coordinating past
  G16's v16 claim.

## Handoffs carried (from #1200's thread)

- **Issue 1022 builds to the corrected A3-2 record in
  `docs/ARMS3-DESIGN.md`, not the issue's original framing**: the
  `SeamVertex` tag's promised recourse ("request the rim whole") is
  measured impossible at every site the tag fires without the
  multi-link door — the door is the ratified promise's only honest
  path. Live pin:
  `sweep/tests/verbs_arms3.rs::requesting_the_rim_whole_gets_past_the_seam`.
- **Issue 827 starts from the `JunctionTangent` pin in `lily.rs`'s
  `review_probes` (margin 1.6e-17)** — that payload is the measured
  boundary the design conversation argues from.

## Unit slate

Every implementation unit here edits `crates/sweep/src/fillet/`, so
implementation is SERIALIZED — one lane at a time, each unit's
branch cut after the previous merges. Design conversations run in
parallel with anything.

1. **BLEND-1 — the multi-link closed-rim door (issue 1022).** The
   annulus surgery accepts a closed chain whose links are the arcs
   of ONE rim across chart seams: the walk carries through seam
   vertices instead of terminating, and per-kind supports may be
   several FACES of one SURFACE. Closes the A3-2 recourse gap on
   all three lantern rims (mouth, neck, lip — including issue 319's
   plane×sphere neck). Consumers: `lily::wall_probes` wall 6, every
   solid of revolution authored closed on the axis.
2. **BLEND-2 — two rims sharing a wall (issue 935).** Presumptive
   shape: the issue's own narrow alternative — refresh only the
   annulus rims' seam keys between carves, keeping every decision
   in the plan phase — so the decide-before-mutate discipline
   stands. If the unit measures that shape insufficient, changing
   the discipline is a design fork: STOP and open it with Evan.
3. **BLEND-3 — concave plane-plane chamfers (issue 919).** The
   geometry already handles both sides; the unit widens the two
   admission doors (`corner_config`'s all-convex requirement,
   `ConvexOpen`), authors the concave-corner fixture through the
   public API, and checks the carve walk is orientation-agnostic.
4. **BLEND-4 — convexity-parametric fillet corners (issue 644).**
   The harder twin, deliberately after BLEND-3: ball admissibility,
   feet signs, octant chart orientation, arc traversals and the
   sense bit move as ONE change, with a concave fixture, then the
   three doors relax together. Verify `corner_ball`'s unexercised
   concave arm before building on it.
5. **BLEND-5 — RimSupport vocabulary (issue 961).** Gated on
   LIB-G16's merge (same emitter seam). Names a curved-on-curved
   rim's support by its role in the carve or by its kind read at
   emit time; persisted-vocabulary change with its N-doc migration
   story and its own schema-seam claim.
6. **BLEND-6 — the shared refusal vocabulary speaks for both verbs
   (issue 917).** Gated on LIB-G16's merge and on its own design
   conversation: HOW a shared refusal names the verb that raised it
   (field on the error / threaded kind / per-verb rendering /
   neutral wording) is user-facing refusal prose with several
   viable answers — Evan's call before the ~255-reference rename
   executes. Must not be closed by minting a parallel enum.
7. **BLEND-7 — the enclosing-tangency refusal (issue 827,
   ruled).** Executes the ratified
   `docs/ENCLOSING-TANGENCY-DESIGN.md`: opening measurement (what
   the lattice serves past the ordinary branch), then the typed
   refusal, the pins' hedge-drop, and the `sugar.rs` purpose
   statement. Reaches `crates/profile` — outside this program's
   sweep fence, taken by exception as the ruling's closing unit
   (no live program owns profile); the fence note rides the
   dispatch, and the unit stays off `profile::structure` (M10-P's
   ground).
- **BLEND-T — SMELL track T (at 2b's merge).** The track taken
  whole per the partition rule; rows worked as style lanes under
  the SMELL conventions (execution record in `docs/SMELL-T-LOG.md`
  when it starts). Note D320 is filed-not-takeable (follows
  track N's D240) and C-e/H13 needs its 779 contradiction verified
  before staffing.

**Backlog, honestly gated:** issue 987 (ruled-spine carve) is
double-gated — its chain terminations are the OQ6 run-out taxonomy
reserved for Evan (per `docs/ARMS3-DESIGN.md` A3-3's parked pair),
and it is consumer-gated with no corpus shape asking. It schedules
only after a design conversation AND a named consumer; neither
exists today.

## Design conversations to open with Evan

- **Issue 827** (enclosing ρ < 0 tangency): **RESOLVED — RATIFIED
  2026-08-29** (`docs/ENCLOSING-TANGENCY-DESIGN.md`): the class is
  ruled out permanently; an enclosing-demanding request refuses.
  BLEND-7 is the closing unit.
- **Issue 917's naming question** — at BLEND-6's turn (above).
- Any move on the surgery module's decide-before-mutate discipline
  (BLEND-2's escape hatch, above).

## Protocol

Full model A/B per `docs/MODEL-AB-LOG.md` (the single normative
source, read on main at every dispatch): v3 triple blocks for
implementer arms, v6 cross-model duals every row, pre-draw
difficulty logging, record-at-merge, per-phase tokens/wall-clock,
blinding fences (no co-author trailers in lane commits, lane-private
paths, no model talk anywhere a reviewer reads). Reviews get
explicit claims to falsify and `docs/prompts/reviewer-style-lane.md`
by path; implementer briefs point at
`docs/prompts/implementer-discipline.md` by path.
