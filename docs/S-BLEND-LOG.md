# S-BLEND log — fillet/chamfer completion

Narrative record; the plan is `docs/S-BLEND-PLAN.md`. Convention as
in the other programs: seam entries at pipeline seams, unit entries
at merges, the tail is the live state.

## Opening state (2026-08-29)

Opened by graduation of the ratified work-stream survey (#1200,
merged after Evan's read with VERBS' cession recorded on its
thread), by a fresh orchestrator on a remote container.

**Operational facts, recorded once (the M10 opening's shape, same
day, same container class):**

- **Branch prefix `blend/`**, orchestrator branch
  `blend/orchestrator`, away-channel tag `(S-BLEND orchestrator)`.
  The harness-designated session branch carries only this opening
  PR.
- **A/B band BLEND = 600–699**, claimed in `docs/MODEL-AB-LOG.md`'s
  banding entry in this same commit. Blocks `BLEND-B1, …`; draws
  recorded branch-side on `blend/orchestrator` per the ratified LIB
  shape.
- **Remote container**: GitHub through MCP rather than `gh`; no
  script monitors (PR watching via MCP subscriptions + scheduled
  self check-ins; away-channel etiquette followed by hand). Disk
  ~29 G free is the binding constraint: lanes are worktrees with
  their own `CARGO_TARGET_DIR` each, ≤ ~2 concurrent lane targets,
  review targets reclaimed at report time; sequential reviews with
  a pre-recorded symmetric method note (the G18a precedent) if disk
  cannot hold two. Build-slot mutex, per-lane target rule,
  CONFLICTING-means-silent-CI, push-early: unchanged. Clone
  unshallowed with a blob filter at opening.

**Unilateral decisions at opening (per the orchestration model,
recorded for Evan's retroactive read):**

1. **The survey's 918/708 listing is corrected to LIB-G16's claim.**
   RECIPE-DOORS (ratified same day) assigns the chamfer recipe door
   and the emit_fillet re-shape to LIB-G16, which dispatched before
   the survey merged; the plan records the seam instead of the
   listing. Kernel chamfer parity (919, 917) stays here.
2. **Serialized unit order 1022 → 935 → 919 → 644**, with 961/917
   gated on G16 and track T gated on 2b — every unit edits
   `crates/sweep/src/fillet/`, so parallel implementation lanes
   would merge-conflict by construction; conversations run in
   parallel instead.
3. **BLEND-2's presumptive shape is the narrow seam-key refresh**
   (the issue's own alternative that keeps decide-before-mutate);
   found insufficient ⇒ STOP, design fork to Evan.
4. **Issue 987 recorded as double-gated** (OQ6 taxonomy is Evan's;
   consumer-gated) rather than scheduled.

**Live gates being watched:** #1180 (SHELLFIX 2b — its merge lifts
the shell/offset keep-out and triggers the track T claim);
LIB-G16's PR when it opens (lifts the emitter seam for BLEND-5/6).

## BLEND-1 MERGED (2026-08-29)

PR 1222 merged at sample 46 (ordinal 600; full record in
MODEL-AB-LOG's row). The multi-link closed-rim door is live: a
seam-split rim's band is one annulus, routing by host side, the
lantern's rims fillet whole, and the SeamVertex recourse is TRUE at
every site the tag fires — conditioned on the side the door serves,
pinned composed on both material sides. The A3-2 promise is served.
Handoffs into the backlog: issue 1244 (concave closed-rim band —
the lily's fourth rim), 1245 (boolean-repaired pole-touching rim),
1246 (public rim-arc selector; consumer evidence from both e2e
reports). Next per the plan: BLEND-2 (issue 935), with BLEND-7
(profile crate, ruled 827) able to interleave.

## BLEND-7 MERGED (2026-08-30)

PR 1267 merged (ordinal 601; full record in MODEL-AB-LOG's row —
arm cell redacted to the branch-side record until block close,
since naming slot 3 determines the open sibling's arm by
arithmetic). The ruled enclosing class now refuses typed with an
ENDORSABLE recourse: the payload carries the corner's largest
tangent radius, every enclosing pin builds at it, and both
review_s2 pins are permanent properties citing the ruling. Issue
827 CLOSED — the 2026-08-29 conversation is fully executed.
Handoffs: 1280 (NCSC plausibly dead), 1281 (refusal attribution),
1282 (Display float class). Block BLEND-B1's record merges to main
when BLEND-2's dual concludes.
